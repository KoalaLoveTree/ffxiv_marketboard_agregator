use db_sdk::server::save_servers;
use error::Error;
use sqlx::{MySql, Pool};
use universalis_sdk::server::get_servers;

pub async fn import_servers(pool: &Pool<MySql>) -> Result<(), Error> {
    let servers = get_servers().await?;
    save_servers(servers, pool).await?;
    Ok(())
}
