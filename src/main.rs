use ffxiv_marketboard_agregator::config::parse_env_variables;
use ffxiv_marketboard_agregator::db::init_database;
use ffxiv_marketboard_agregator::import::import_items_data;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let config = parse_env_variables();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &(format!(
                "postgres://{username}:{password}@{host}/{database}",
                username = &config["DATABASE_USERNAME"],
                password = &config["DATABASE_PASSWORD"],
                host = &config["DATABASE_HOST"],
                database = &config["DATABASE_NAME"],
            )),
        )
        .await
        .unwrap();

    init_database(&pool).await.unwrap();
    import_items_data(&pool).await.unwrap();
}
