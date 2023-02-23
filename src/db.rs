use sqlx::{Pool, Postgres, Result};

pub async fn init_database(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (
            item_id BigInt PRIMARY KEY,name TEXT NOT NULL)",
    )
    .execute(pool)
    .await?;

    Ok(())
}
