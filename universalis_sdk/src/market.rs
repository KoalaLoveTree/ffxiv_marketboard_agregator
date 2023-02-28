use error::Error;
use error::Error::{JsonMissedDataError, UrlParseError};
use reqwest::Url;
use sdk_dto::market::{ItemSaleHistoryUnit, ItemVelocity};
use serde_json::Value;

pub async fn get_marketable_items_ids() -> Result<Vec<u64>, Error> {
    let body_marketable_items = reqwest::get("https://universalis.app/api/v2/marketable")
        .await?
        .text()
        .await?;

    let marketable_items: Value = serde_json::from_str(&body_marketable_items)?;

    let marketable_items = marketable_items
        .as_array()
        .ok_or( JsonMissedDataError {
            field_name: "Marketable Items".to_string(),
        })?;

    let mut marketable_items_ids: Vec<u64> = Vec::new();

    for marketable_item in marketable_items {
        let marketable_item_id = marketable_item
            .as_u64()
            .ok_or( JsonMissedDataError {
                field_name: "Marketable Items.item_id".to_string(),
            })?;
        marketable_items_ids.push(marketable_item_id);
    }

    Ok(marketable_items_ids)
}

pub async fn get_item_velocity_by_world(
    item_id: u64,
    world_name: &str,
) -> Result<ItemVelocity, Error> {
    let mut url = Url::parse("https://universalis.app/api/v2")?;
    url.path_segments_mut()
        .map_err(|_| UrlParseError)?
        .push(world_name);
    url.path_segments_mut()
        .map_err(|_| UrlParseError)?
        .push(&item_id.to_string());
    url.query_pairs_mut()
        .append_pair("fields", "nqSaleVelocity,hqSaleVelocity");

    let body_velocity = reqwest::get(url.as_str()).await?.text().await?;

    let velocity_value: Value = serde_json::from_str(&body_velocity[..])?;

    let hq_velocity =
        velocity_value["hqSaleVelocity"]
            .as_f64()
            .ok_or( JsonMissedDataError {
                field_name: "velocity.hqSaleVelocity".to_string(),
            })?;

    let nq_velocity =
        velocity_value["nqSaleVelocity"]
            .as_f64()
            .ok_or( JsonMissedDataError {
                field_name: "velocity.nqSaleVelocity".to_string(),
            })?;

    Ok(ItemVelocity {
        nq_velocity,
        hq_velocity,
    })
}

pub async fn get_item_sale_history_by_world(
    item_id: u64,
    world_name: &str,
) -> Result<Vec<ItemSaleHistoryUnit>, Error> {
    let mut url = Url::parse("https://universalis.app/api/v2/history")?;
    url.path_segments_mut()
        .map_err(|_| UrlParseError)?
        .push(world_name);
    url.path_segments_mut()
        .map_err(|_| UrlParseError)?
        .push(&item_id.to_string());

    let body_sale_history = reqwest::get(url.as_str()).await?.text().await?;

    let sale_history: Value = serde_json::from_str(&body_sale_history[..])?;

    let sale_history_entries =
        sale_history["entries"]
            .as_array()
            .ok_or( JsonMissedDataError {
                field_name: "sale_history.entries".to_string(),
            })?;

    let mut item_sale_history: Vec<ItemSaleHistoryUnit> = Vec::new();

    for sale_history_unit in sale_history_entries {
        let quantity =
            sale_history_unit["quantity"]
                .as_f64()
                .ok_or( JsonMissedDataError {
                    field_name: "sale_history_unit.quantity".to_string(),
                })?;

        let price_per_unit =
            sale_history_unit["pricePerUnit"]
                .as_f64()
                .ok_or( JsonMissedDataError {
                    field_name: "sale_history_unit.pricePerUnit".to_string(),
                })?;

        item_sale_history.push(ItemSaleHistoryUnit {
            quantity,
            price_per_unit,
        });
    }

    Ok(item_sale_history)
}
