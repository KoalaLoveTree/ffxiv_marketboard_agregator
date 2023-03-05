#![allow(non_snake_case)]

pub mod errors;

use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

const UNIVERSALIS_URL: &str = "https://universalis.app/api/v2";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemVelocity {
    #[serde(alias = "nqSaleVelocity")]
    pub nq_velocity: f64,
    #[serde(alias = "hqSaleVelocity")]
    pub hq_velocity: f64,
}

impl ItemVelocity {
    pub fn get_better_velocity(&self) -> f64 {
        if self.hq_velocity > 0.0 {
            return self.hq_velocity;
        }

        self.nq_velocity
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemSaleHistoryUnit {
    pub quantity: u64,
    #[serde(alias = "pricePerUnit")]
    pub price_per_unit: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct ItemTradeVolume {
    pub item_id: u64,
    pub world_id: u64,
    pub cheapest_world_id: u64,
    pub sale_score: f64,
    pub price_diff_score: f64,
    pub home_world_avg_price: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataCenter {
    pub name: String,
    pub region: String,
    pub worlds: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct World {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub data_center: DataCenter,
    pub worlds: Vec<World>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemSaleHistory {
    pub items: HashMap<u64, ItemMapping>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemMapping {
    pub entries: Vec<ItemSaleHistoryUnit>,
}

pub async fn get_servers() -> Result<Vec<Server>, Error> {
    let data_centers = get_data_centers().await?;
    let mut servers: Vec<Server> = Vec::new();
    for data_center in data_centers {
        let data_center_worlds = get_data_center_worlds(&data_center).await?;
        servers.push(Server {
            data_center,
            worlds: data_center_worlds,
        })
    }

    Ok(servers)
}

async fn get_data_center_worlds(data_center: &DataCenter) -> Result<Vec<World>, Error> {
    let mut url = Url::parse(UNIVERSALIS_URL)?;
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push("worlds");

    let body_worlds = reqwest::get(url.as_str()).await?.text().await?;

    let mut worlds: Vec<World> = serde_json::from_str(&body_worlds)?;

    worlds.retain(|world| {
        data_center
            .worlds
            .iter()
            .any(|&dc_world_id| dc_world_id == world.id)
    });

    Ok(worlds)
}

async fn get_data_centers() -> Result<Vec<DataCenter>, Error> {
    let mut url = Url::parse(UNIVERSALIS_URL)?;
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push("data-centers");

    let body_data_centers = reqwest::get(url.as_str()).await?.text().await?;

    let data_centers: Vec<DataCenter> = serde_json::from_str(&body_data_centers)?;

    Ok(data_centers)
}

pub async fn get_marketable_items_ids() -> Result<Vec<u64>, Error> {
    let mut url = Url::parse(UNIVERSALIS_URL)?;
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push("marketable");

    let body_marketable_items = reqwest::get(url.as_str()).await?.text().await?;

    let marketable_items_ids: Vec<u64> = serde_json::from_str(&body_marketable_items)?;

    Ok(marketable_items_ids)
}

pub async fn get_item_velocity_by_world(
    item_id: u64,
    world_name: String,
) -> Result<ItemVelocity, Error> {
    let mut url = Url::parse(UNIVERSALIS_URL)?;
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push(&world_name);
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push(&item_id.to_string());
    url.query_pairs_mut()
        .append_pair("fields", "nqSaleVelocity,hqSaleVelocity");

    let body_velocity = reqwest::get(url.as_str()).await?.text().await?;

    let item_velocity: ItemVelocity = serde_json::from_str(&body_velocity[..])?;

    Ok(item_velocity)
}

pub async fn get_item_sale_history_by_world(
    item_ids: Vec<u64>,
    world_name: String,
) -> Result<ItemSaleHistory, Error> {
    let mut url = Url::parse(UNIVERSALIS_URL)?;
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push("history");
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push(&world_name);

    let mut ids_param = item_ids
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>();
    ids_param = ids_param.trim_end_matches(',').to_string();

    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push(&ids_param);

    let body_sale_history = reqwest::get(url.as_str()).await?.text().await?;

    let sale_history: ItemSaleHistory = serde_json::from_str(&body_sale_history[..])?;

    Ok(sale_history)
}
