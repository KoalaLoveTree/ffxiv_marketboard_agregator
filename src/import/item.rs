use crate::import::errors::ImportError;
use crate::import::errors::ImportError::{
    JsonMissedDataError, JsonParseError, RequestError, ResponseParseError, SqlExecutionError,
    UrlParseError,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use url::Url;

pub async fn import_items_data(pool: &Pool<Postgres>) -> Result<(), ImportError> {
    let body_items = reqwest::get("https://xivapi.com/search?indexes=Item")
        .await
        .map_err(|err| RequestError {
            message: err.to_string(),
        })?
        .text()
        .await
        .map_err(|err| ResponseParseError {
            message: err.to_string(),
        })?;

    let items: Value = serde_json::from_str(&body_items[..]).map_err(|err| JsonParseError {
        message: err.to_string(),
    })?;

    let body_marketable_items = reqwest::get("https://universalis.app/api/v2/marketable")
        .await
        .map_err(|err| RequestError {
            message: err.to_string(),
        })?
        .text()
        .await
        .map_err(|err| ResponseParseError {
            message: err.to_string(),
        })?;

    let marketable_items: Value =
        serde_json::from_str(&body_marketable_items[..]).map_err(|err| JsonParseError {
            message: err.to_string(),
        })?;

    let marketable_items = match marketable_items.as_array() {
        None => {
            return Err(JsonMissedDataError {
                field_name: "Marketable Items".to_string(),
            });
        }
        Some(items) => items,
    };

    let mut page = match items["Pagination"]["Page"].as_i64() {
        None => {
            return Err(JsonMissedDataError {
                field_name: "Pagination.Page".to_string(),
            });
        }
        Some(page) => page,
    };

    loop {
        let mut url = Url::parse("https://xivapi.com/search").map_err(|err| UrlParseError {
            message: err.to_string(),
        })?;
        url.query_pairs_mut().append_pair("indexes", "Item");
        url.query_pairs_mut()
            .append_pair("page", &page.to_string()[..]);

        let body = reqwest::get(url.as_str())
            .await
            .map_err(|err| RequestError {
                message: err.to_string(),
            })?
            .text()
            .await
            .map_err(|err| ResponseParseError {
                message: err.to_string(),
            })?;

        let items: Value = serde_json::from_str(&body[..]).map_err(|err| JsonParseError {
            message: err.to_string(),
        })?;

        let items_results = match items["Results"].as_array() {
            None => {
                return Err(JsonMissedDataError {
                    field_name: "Results".to_string(),
                });
            }
            Some(results) => results,
        };

        for item in items_results {
            for marketable_item_id in marketable_items {
                if item["ID"].as_i64() == marketable_item_id.as_i64() {
                    let _row: (i64, ) = sqlx::query_as("insert into items (item_id, name) values ($1, $2) on conflict (item_id) do update set name = $2 returning item_id")
                        .bind(item["ID"].as_i64())
                        .bind(item["Name"].as_str())
                        .fetch_one(pool)
                        .await
                        .map_err(|err| {
                            SqlExecutionError {
                                message: err.to_string(),
                            }
                        })?;
                }
            }
        }

        if items["Pagination"]["PageNext"].is_null() {
            break;
        } else {
            page = match items["Pagination"]["PageNext"].as_i64() {
                None => {
                    return Err(JsonMissedDataError {
                        field_name: "Pagination.PageNext".to_string(),
                    });
                }
                Some(page) => page,
            };
        }
    }

    Ok(())
}
