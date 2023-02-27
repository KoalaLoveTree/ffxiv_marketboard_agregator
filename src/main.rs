use ffxiv_marketboard_agregator::config::parse_env_variables;
use ffxiv_marketboard_agregator::import::errors::ImportError;
use ffxiv_marketboard_agregator::import::store::import_items_trade_volumes;
use ffxiv_marketboard_agregator::import::sync_base_data;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), ImportError> {
    let config = parse_env_variables();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config["DATABASE_URL"][..])
        .await
        .unwrap();

    sync_base_data(&pool).await?;

    import_items_trade_volumes(&pool, String::from("Chaos"), String::from("Phantom")).await?;

    Ok(())
}
