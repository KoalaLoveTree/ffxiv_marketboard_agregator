use crate::db::ItemData;
use crate::import::errors::Error;
use universalis_sdk::get_marketable_items_ids;
use universalis_sdk::xivapi::get_all_items;

pub struct ItemImport {
    item_data: ItemData,
}

impl ItemImport {
    pub fn new(item_data: ItemData) -> Self {
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

        let ids_for_delete = self
            .item_data
            .get_items_ids_for_delete(marketable_items_ids)
            .await?;

        self.item_data.delete_items(ids_for_delete);

        Ok(())
    }

    pub async fn sync_items(&self) -> Result<(), Error> {
        let marketable_items_ids = get_marketable_items_ids().await?;

        let ids_for_delete = self
            .item_data
            .get_items_ids_for_delete(marketable_items_ids)
            .await?;

        self.item_data.delete_items(ids_for_delete);

        Ok(())
    }
}
