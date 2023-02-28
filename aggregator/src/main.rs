use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;

mod import;

#[derive(Deserialize, Debug)]
struct Config {
    db_url: String,
}

#[tokio::main]
async fn main() {
    let config =  match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{error:#?}")
    };

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .unwrap();
}
