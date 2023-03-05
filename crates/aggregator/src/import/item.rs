use crate::import::errors::Error;
use db_sdk::ItemData;
use universalis_sdk::get_marketable_items_ids;
use xivapi_sdk::get_all_items;

pub struct ItemImport<'a> {
    item_data: &'a ItemData,
}

impl<'a> ItemImport<'a> {
    pub fn new(item_data: &'a ItemData) -> Self {
        Self { item_data }
    }

    pub async fn import_marketable_items(&self) -> Result<(), Error> {
        let items = get_all_items().await?;
        let marketable_items_ids = get_marketable_items_ids().await?;

        let marketable_items = items
            .into_iter()
            .filter(|item| {
                marketable_items_ids
                    .iter()
                    .any(|marketable_item_id| marketable_item_id == &item.id)
            })
            .collect();

        self.item_data.save_items(marketable_items).await?;

        Ok(())
    }
}
