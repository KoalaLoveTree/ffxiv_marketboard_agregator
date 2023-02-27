use crate::import::errors::ImportError;
use crate::import::errors::ImportError::{
    HashMapAccessError, JsonMissedDataError, JsonParseError, RequestError, ResponseParseError,
    SqlExecutionError, UrlParseError, WrongUrlBaseError,
};
use crate::item::Item;
use crate::server::{DataCenter, World};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use url::Url;

pub async fn import_items_trade_volume(
    pool: &Pool<Postgres>,
    data_center_name: String,
    home_world_name: String,
) -> Result<(), ImportError> {
    let data_center: DataCenter = sqlx::query_as!(
        DataCenter,
        r"select * from data_centers where name = $1",
        data_center_name
    )
    .fetch_one(pool)
    .await
    .map_err(|err| SqlExecutionError {
        message: err.to_string(),
    })?;

    let home_world: World = sqlx::query_as!(
        World,
        r"select * from worlds where name = $1 and data_center_id = $2",
        home_world_name,
        data_center.id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| SqlExecutionError {
        message: err.to_string(),
    })?;

    let worlds: Vec<World> = sqlx::query_as!(
        World,
        r"select * from worlds where data_center_id = $1",
        data_center.id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| SqlExecutionError {
        message: err.to_string(),
    })?;

    let items: Vec<Item> = sqlx::query_as!(Item, r"select * from items limit 50")
        .fetch_all(pool)
        .await
        .map_err(|err| SqlExecutionError {
            message: err.to_string(),
        })?;

    for item in items {
        let mut velocity: f64 = 0.0;
        let mut avg_price_per_world: HashMap<i64, f64> = HashMap::new();

        for world in &worlds {
            let mut url = Url::parse("https://universalis.app/api/v2/history").map_err(|err| {
                UrlParseError {
                    message: err.to_string(),
                }
            })?;
            let world_name_param = format!("/{}", world.name);
            let item_id_param = format!("/{}", item.item_id);
            url.path_segments_mut()
                .map_err(|_err| WrongUrlBaseError)?
                .push(&world_name_param);
            url.path_segments_mut()
                .map_err(|_err| WrongUrlBaseError)?
                .push(&item_id_param);

            let body_sale_history = reqwest::get(url.as_str())
                .await
                .map_err(|err| RequestError {
                    message: err.to_string(),
                })?
                .text()
                .await
                .map_err(|err| ResponseParseError {
                    message: err.to_string(),
                })?;

            let sale_history: Value =
                serde_json::from_str(&body_sale_history[..]).map_err(|err| JsonParseError {
                    message: err.to_string(),
                })?;

            let sale_history_entries = match sale_history["entries"].as_array() {
                None => {
                    return Err(JsonMissedDataError {
                        field_name: "sale_history.entries".to_string(),
                    });
                }
                Some(entries) => entries,
            };

            let mut total_gil_spent: f64 = 0.0;
            let mut quantity = 0.0;

            for sale_history_unit in sale_history_entries {
                let sale_history_unit_quantity = match sale_history_unit["quantity"].as_f64() {
                    None => {
                        return Err(JsonMissedDataError {
                            field_name: "sale_history_unit.quantity".to_string(),
                        });
                    }
                    Some(quantity) => quantity,
                };

                let sale_history_unit_price_per_unit =
                    match sale_history_unit["pricePerUnit"].as_f64() {
                        None => {
                            return Err(JsonMissedDataError {
                                field_name: "sale_history_unit.pricePerUnit".to_string(),
                            });
                        }
                        Some(price_per_unit) => price_per_unit,
                    };
                total_gil_spent = sale_history_unit_price_per_unit * sale_history_unit_quantity;
                quantity = sale_history_unit_quantity;
            }

            avg_price_per_world.insert(world.world_id, total_gil_spent / quantity);

            if world.world_id == home_world.world_id {
                let mut url =
                    Url::parse("https://universalis.app/api/v2").map_err(|err| UrlParseError {
                        message: err.to_string(),
                    })?;
                let world_name_param = format!("/{}", world.name);
                let item_id_param = format!("/{}", item.item_id);
                url.path_segments_mut()
                    .map_err(|_err| WrongUrlBaseError)?
                    .push(&world_name_param);
                url.path_segments_mut()
                    .map_err(|_err| WrongUrlBaseError)?
                    .push(&item_id_param);
                url.query_pairs_mut()
                    .append_pair("fields", "nqSaleVelocity,hqSaleVelocity");

                let body_velocity = reqwest::get(url.as_str())
                    .await
                    .map_err(|err| RequestError {
                        message: err.to_string(),
                    })?
                    .text()
                    .await
                    .map_err(|err| ResponseParseError {
                        message: err.to_string(),
                    })?;

                let velocity_value: Value =
                    serde_json::from_str(&body_velocity[..]).map_err(|err| JsonParseError {
                        message: err.to_string(),
                    })?;

                let hq_sale_velocity = match velocity_value["hqSaleVelocity"].as_f64() {
                    None => {
                        return Err(JsonMissedDataError {
                            field_name: "velocity_value.hqSaleVelocity".to_string(),
                        });
                    }
                    Some(velocity) => velocity,
                };

                let nq_sale_velocity = match velocity_value["hqSaleVelocity"].as_f64() {
                    None => {
                        return Err(JsonMissedDataError {
                            field_name: "velocity_value.hqSaleVelocity".to_string(),
                        });
                    }
                    Some(velocity) => velocity,
                };

                if hq_sale_velocity != 0.0 {
                    velocity = hq_sale_velocity;
                } else {
                    velocity = nq_sale_velocity;
                }
            }
        }

        // let home_world_avg_price: f64 =
        //     *(avg_price_per_world).get(&home_world.world_id).ok_or(|| -> ImportError {
        //         return HashMapAccessErrorError;
        //     })?;
        let home_world_avg_price: f64 = match avg_price_per_world.get(&home_world.world_id) {
            None => {
                return Err(HashMapAccessError);
            }
            Some(price) => *price,
        };
        let mut cheapest_world_avg_price: f64 = home_world_avg_price;
        let mut cheapest_world_id: i64 = home_world.world_id;

        for (world_id, avg_price) in avg_price_per_world {
            if cheapest_world_avg_price > avg_price {
                cheapest_world_avg_price = avg_price;
                cheapest_world_id = world_id;
            }
        }

        let _row: (i64, ) = sqlx::query_as("insert into items_trade_volume (item_id, world_id, sale_score, price_diff_score, cheapest_world_id) values ($1, $2, $3, $4, $5) returning item_id")
            .bind(item.item_id)
            .bind(home_world.world_id)
            .bind(velocity)
            .bind(home_world_avg_price/cheapest_world_avg_price)
            .bind(cheapest_world_id)
            .fetch_one(pool)
            .await
            .map_err(|err| {
                SqlExecutionError {
                    message: err.to_string(),
                }
            })?;
    }

    Ok(())
}
