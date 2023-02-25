use ffxiv_marketboard_agregator::config::parse_env_variables;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let config = parse_env_variables();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config["DATABASE_URL"][..])
        .await
        .unwrap();

    // import_worlds_data(&pool).await.unwrap();
    // import_items_data(&pool).await.unwrap();
    // import_worlds_data(&pool).await.unwrap();
}
