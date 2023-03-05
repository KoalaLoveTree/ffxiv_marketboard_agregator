use error::Error;
use sdk_dto::market::ItemTradeVolume;
use sqlx::{MySql, Pool};

#[derive(Debug, Copy, Clone)]
pub struct DBItemTradeVolume {
    pub id: u64,
    pub item_id: u64,
    pub world_id: u64,
    pub cheapest_world_id: u64,
    pub sale_score: f64,
    pub price_diff_score: f64,
}

pub async fn save_item_trade_volumes(
    items_trade_volumes: Vec<ItemTradeVolume>,
    pool: &Pool<MySql>,
) -> Result<(), Error> {
    for item_trade_volume in items_trade_volumes {
        sqlx::query!(
            "insert ignore into items_trade_volumes (item_id, world_id, sale_score, price_diff_score, cheapest_world_id) values (?, ?, ?, ?, ?)",
            item_trade_volume.item_id,
            item_trade_volume.world_id,
            item_trade_volume.sale_score,
            item_trade_volume.price_diff_score,
            item_trade_volume.cheapest_world_id
        )
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn get_item_trade_volume_by_world(
    world_id: u64,
    item_id: u64,
    pool: &Pool<MySql>,
) -> Result<DBItemTradeVolume, Error> {
    Ok(sqlx::query_as!(
        DBItemTradeVolume,
        r"select * from items_trade_volumes where item_id = ? and world_id = ?",
        item_id,
        world_id
    )
    .fetch_one(pool)
    .await?)
}
