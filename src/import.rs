use crate::item::Item;
use crate::server::{DataCenter, World};
use reqwest::Url;
use serde_json::{Result, Value};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

pub async fn import_items_data(pool: &Pool<Postgres>) -> Result<()> {
    let body_items = reqwest::get("https://xivapi.com/search?indexes=Item")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let items: Value = serde_json::from_str(&body_items[..])?;

    let body_marketable_items = reqwest::get("https://universalis.app/api/v2/marketable")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let marketable_items: Value = serde_json::from_str(&body_marketable_items[..])?;

    let mut page = items["Pagination"]["PageNext"].as_i64().unwrap();

    loop {
        let mut url = Url::parse("https://xivapi.com/search").unwrap();
        url.query_pairs_mut().append_pair("indexes", "Item");
        url.query_pairs_mut()
            .append_pair("page", &page.to_string()[..]);

        let body = reqwest::get(url.as_str())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let items: Value = serde_json::from_str(&body[..])?;

        for item in items["Results"].as_array().unwrap() {
            for marketable_item_id in marketable_items.as_array().unwrap() {
                if item["ID"].as_i64() == marketable_item_id.as_i64() {
                    let _row: (i64, ) = sqlx::query_as("insert into items (item_id, name) values ($1, $2) on conflict (item_id) do update set name = $2 returning item_id")
                        .bind(item["ID"].as_i64())
                        .bind(item["Name"].as_str())
                        .fetch_one(pool)
                        .await
                        .unwrap();
                }
            }
        }

        if items["Pagination"]["PageNext"].is_null() {
            break;
        } else {
            page = items["Pagination"]["PageNext"].as_i64().unwrap();
        }
    }

    Ok(())
}

pub async fn import_worlds_data(pool: &Pool<Postgres>) -> Result<()> {
    let body_data_centers = reqwest::get("https://universalis.app/api/v2/data-centers")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let body_worlds = reqwest::get("https://universalis.app/api/v2/worlds")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let data_centers: Value = serde_json::from_str(&body_data_centers[..])?;
    let worlds: Value = serde_json::from_str(&body_worlds[..])?;

    for data_center in data_centers.as_array().unwrap() {
        let data_center_row: (i64, ) = sqlx::query_as("insert into data_centers (name, region) values ($1, $2) on conflict (name) do nothing returning item_id")
            .bind(data_center["name"].as_str())
            .bind(data_center["region"].as_str())
            .fetch_one(pool)
            .await
            .unwrap();

        for world_id in data_center["worlds"].as_array().unwrap() {
            for world in worlds.as_array().unwrap() {
                if world["item_id"].as_i64() != world_id.as_i64() {
                    continue;
                }
                let _row: (i64, ) = sqlx::query_as("insert into worlds (world_id, name, data_center_id) values ($1, $2, $3) on conflict (world_id) do update set name = $2 returning world_id")
                    .bind(world["item_id"].as_i64())
                    .bind(world["name"].as_str())
                    .bind(data_center_row.0)
                    .fetch_one(pool)
                    .await
                    .unwrap();
            }
        }
    }

    Ok(())
}

pub async fn import_data_centers_data(pool: &Pool<Postgres>) -> Result<()> {
    let body = reqwest::get("https://universalis.app/api/v2/data-centers")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let items: Value = serde_json::from_str(&body[..])?;

    for item in items.as_array().unwrap() {
        let _row: (i64, ) = sqlx::query_as("insert into data_centers (data_center_id, name) values ($1, $2) on conflict (data_center_id) do update set name = $2 returning data_center_id")
            .bind(item["item_id"].as_i64())
            .bind(item["name"].as_str())
            .fetch_one(pool)
            .await
            .unwrap();
    }

    Ok(())
}

pub async fn import_items_trade_volume(
    pool: &Pool<Postgres>,
    data_center_name: String,
    home_world_name: String,
) -> Result<()> {
    let data_center: DataCenter = sqlx::query_as!(
        DataCenter,
        r"select * from data_centers where name = $1",
        data_center_name
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let home_world: World = sqlx::query_as!(
        World,
        r"select * from worlds where name = $1 and data_center_id = $2",
        home_world_name,
        data_center.id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let worlds: Vec<World> = sqlx::query_as!(
        World,
        r"select * from worlds where data_center_id = $1",
        data_center.id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let items: Vec<Item> = sqlx::query_as!(Item, r"select * from items")
        .fetch_all(pool)
        .await
        .unwrap();

    for item in items {
        let mut velocity: f64 = 0.0;
        let mut avg_price_per_world: HashMap<i64, f64> = HashMap::new();

        for world in worlds {
            let mut url = Url::parse("https://universalis.app/api/v2/history").unwrap();
            let world_name_param = format!("/{}", world.name);
            let item_id_param = format!("/{}", item.item_id);
            url.path_segments_mut().unwrap().push(&world_name_param);
            url.path_segments_mut().unwrap().push(&item_id_param);

            let body_sale_history = reqwest::get(url.as_str())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let sale_history: Value = serde_json::from_str(&body_sale_history[..])?;

            let mut total_gil_spent: f64 = 0.0;
            let mut quantity = 0.0;

            for sale_history_unit in sale_history["entries"].as_array().unwrap() {
                total_gil_spent = sale_history_unit["pricePerUnit"].as_f64().unwrap()
                    * sale_history_unit["quantity"].as_f64().unwrap();
                quantity = sale_history_unit["quantity"].as_f64().unwrap();
            }

            avg_price_per_world.insert(world.world_id, total_gil_spent / quantity);

            if world.world_id == home_world.world_id {
                let mut url = Url::parse("https://universalis.app/api/v2").unwrap();
                let world_name_param = format!("/{}", world.name);
                let item_id_param = format!("/{}", item.item_id);
                url.path_segments_mut().unwrap().push(&world_name_param);
                url.path_segments_mut().unwrap().push(&item_id_param);
                url.query_pairs_mut()
                    .append_pair("fields", "nqSaleVelocity,hqSaleVelocity");

                let body_velocity = reqwest::get(url.as_str())
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let velocity_value: Value = serde_json::from_str(&body_velocity[..])?;

                if velocity_value["hqSaleVelocity"] != 0 {
                    velocity = velocity_value["hqSaleVelocity"].as_f64().unwrap();
                } else {
                    velocity = velocity_value["nqSaleVelocity"].as_f64().unwrap();
                }
            }
        }

        let home_world_avg_price: f64 = *avg_price_per_world
            .get(&home_world.world_id)
            .unwrap();
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
            .unwrap();
    }

    Ok(())
}
