use error::Error;
use error::Error::JsonMissedDataError;
use reqwest::Url;
use sdk_dto::item::Item;
use serde_json::Value;

pub async fn get_all_items() -> Result<Vec<Item>, Error> {
    let body_items = reqwest::get("https://xivapi.com/search?indexes=Item")
        .await?
        .text()
        .await?;

    let items_response: Value = serde_json::from_str(&body_items)?;

    let mut page = items_response["Pagination"]["Page"]
        .as_i64()
        .ok_or( JsonMissedDataError {
            field_name: "Pagination.Page".to_string(),
        })?;

    let mut items_result: Vec<Item> = Vec::new();

    loop {
        let mut url = Url::parse("https://xivapi.com/search")?;
        url.query_pairs_mut().append_pair("indexes", "Item");
        url.query_pairs_mut()
            .append_pair("page", &page.to_string()[..]);

        let body_items = reqwest::get(url.as_str()).await?.text().await?;

        let items_response: Value = serde_json::from_str(&body_items[..])?;

        let items = items_response["Results"]
            .as_array()
            .ok_or( JsonMissedDataError {
                field_name: "Results".to_string(),
            })?;

        for item in items {
            let item_id = item["ID"].as_u64().ok_or( JsonMissedDataError {
                field_name: "Items.item.id".to_string(),
            })?;
            let item_name = item["Name"].as_str().ok_or( JsonMissedDataError {
                field_name: "Items.item.name".to_string(),
            })?;

            items_result.push(Item {
                id: item_id,
                name: item_name.to_string(),
            });
        }

        if items_response["Pagination"]["PageNext"].is_null() {
            break;
        } else {
            page = items_response["Pagination"]["PageNext"]
                .as_i64()
                .ok_or( JsonMissedDataError {
                    field_name: "Pagination.PageNext".to_string(),
                })?;
        }
    }

    Ok(items_result)
}
