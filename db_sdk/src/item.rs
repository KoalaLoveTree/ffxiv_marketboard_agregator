use error::Error;
use sdk_dto::item::Item;
use sqlx::{MySql, Pool};

#[derive(Debug, Clone)]
pub struct DBItem {
    pub item_id: u64,
    pub name: String,
}

pub async fn save_items(items: Vec<Item>, pool: &Pool<MySql>) -> Result<(), Error> {
    for item in items {
        sqlx::query!(
            "insert ignore into items (item_id, name) values (?, ?)",
            item.id,
            item.name
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_items(pool: &Pool<MySql>) -> Result<Vec<DBItem>, Error> {
    let items: Vec<DBItem> = sqlx::query_as!(DBItem, r"select * from items")
        .fetch_all(pool)
        .await?;

    Ok(items)
}
