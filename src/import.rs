use crate::import::errors::ImportError;
use crate::import::item::import_items_data;
use crate::import::server::import_servers_data;
use sqlx::{Pool, Postgres};

pub mod errors;
mod item;
mod server;
pub mod store;

pub async fn sync_base_data(pool: &Pool<Postgres>) -> Result<(), ImportError> {
    import_items_data(pool).await?;
    import_servers_data(pool).await?;
    Ok(())
}
