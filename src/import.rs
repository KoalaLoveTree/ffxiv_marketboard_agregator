use serde_json::{Result, Value};
use sqlx::{Pool, Postgres};

pub async fn import_items_data(pool: &Pool<Postgres>) -> Result<()> {
    let body = reqwest::get("https://xivapi.com/search?indexes=Item")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let items: Value = serde_json::from_str(&body[..])?;

    let mut page = items["Pagination"]["PageNext"].as_i64().unwrap();

    loop {
        let mut url: String = String::from("https://xivapi.com/search?indexes=Item&page=");
        url.push_str(&page.to_string());
        let body = reqwest::get(&url[..]).await.unwrap().text().await.unwrap();

        let items: Value = serde_json::from_str(&body[..])?;

        for item in items["Results"].as_array().unwrap() {
            let _row: (i64, ) = sqlx::query_as("insert into items (item_id, name) values ($1, $2) on conflict (item_id) do update set name = $2 returning item_id")
                .bind(item["ID"].as_i64())
                .bind(item["Name"].as_str())
                .fetch_one(pool)
                .await
                .unwrap();
        }

        if items["Pagination"]["PageNext"].is_null() {
            break;
        } else {
            page = items["Pagination"]["PageNext"].as_i64().unwrap();
        }
    }

    Ok(())
}
