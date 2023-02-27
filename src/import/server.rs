use crate::import::errors::ImportError;
use crate::import::errors::ImportError::{
    JsonMissedDataError, JsonParseError, RequestError, ResponseParseError, SqlExecutionError,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

pub async fn import_servers_data(pool: &Pool<Postgres>) -> Result<(), ImportError> {
    let body_data_centers = reqwest::get("https://universalis.app/api/v2/data-centers")
        .await
        .map_err(|err| RequestError {
            message: err.to_string(),
        })?
        .text()
        .await
        .map_err(|err| ResponseParseError {
            message: err.to_string(),
        })?;

    let body_worlds = reqwest::get("https://universalis.app/api/v2/worlds")
        .await
        .map_err(|err| RequestError {
            message: err.to_string(),
        })?
        .text()
        .await
        .map_err(|err| ResponseParseError {
            message: err.to_string(),
        })?;

    let data_centers: Value =
        serde_json::from_str(&body_data_centers[..]).map_err(|err| JsonParseError {
            message: err.to_string(),
        })?;

    let data_centers = match data_centers.as_array() {
        None => {
            return Err(JsonMissedDataError {
                field_name: "Data Centers".to_string(),
            });
        }
        Some(data_centers) => data_centers,
    };

    let worlds: Value = serde_json::from_str(&body_worlds[..]).map_err(|err| JsonParseError {
        message: err.to_string(),
    })?;

    let worlds = match worlds.as_array() {
        None => {
            return Err(JsonMissedDataError {
                field_name: "Worlds".to_string(),
            });
        }
        Some(worlds) => worlds,
    };

    for data_center in data_centers {
        let data_center_row: (i64, ) = sqlx::query_as("insert into data_centers (name, region) values ($1, $2) on conflict (name) do nothing returning item_id")
            .bind(data_center["name"].as_str())
            .bind(data_center["region"].as_str())
            .fetch_one(pool)
            .await
            .map_err(|err| {
                SqlExecutionError {
                    message: err.to_string(),
                }
            })?;

        let data_center_worlds = match data_center["worlds"].as_array() {
            None => {
                return Err(JsonMissedDataError {
                    field_name: "data_center.worlds".to_string(),
                });
            }
            Some(worlds) => worlds,
        };

        for world_id in data_center_worlds {
            for world in worlds {
                if world["item_id"].as_i64() != world_id.as_i64() {
                    continue;
                }
                let _row: (i64, ) = sqlx::query_as("insert into worlds (world_id, name, data_center_id) values ($1, $2, $3) on conflict (world_id) do update set name = $2 returning world_id")
                    .bind(world["item_id"].as_i64())
                    .bind(world["name"].as_str())
                    .bind(data_center_row.0)
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

    Ok(())
}
