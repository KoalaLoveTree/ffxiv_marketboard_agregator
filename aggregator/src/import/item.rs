use db_sdk::item::save_items;
use error::Error;
use sdk_dto::item::Item;
use sqlx::{MySql, Pool};
use universalis_sdk::market::get_marketable_items_ids;
use xivapi_sdk::get_all_items;

pub async fn import_marketable_items(pool: &Pool<MySql>) -> Result<(), Error> {
    let items = get_all_items().await?;

    let marketable_items_ids = get_marketable_items_ids().await?;

    let mut marketable_items: Vec<Item> = Vec::new();

    for item in items {
        if marketable_items_ids
            .iter()
            .any(|&marketable_item_id| marketable_item_id == item.id)
        {
            marketable_items.push(item);
        }
    }

    save_items(marketable_items, pool).await?;

    Ok(())
}
