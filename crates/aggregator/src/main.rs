extern crate core;

mod import;

use crate::import::{ItemImport, MarketImport, ServerImport};
use db_sdk::{ItemData, ItemTrades, ServerData};
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;
use dotenv::dotenv;

const DATA_CENTER_NAME_ARG: &str = "-dcn";
const HOME_WORLD_NAME_ARG: &str = "-hwn";

struct Config {
    database_url: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config{
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set").to_string(),
    };

    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    handle_command(command, &config).await;
}

async fn handle_command(command: &str, config: &Config) {
    match command {
        "--help" => {
            println!(
                "Hi, its aggregator for marketboard for ffxiv. You can execute following commands:"
            );
            println!("--help    show this help text <3!");
            println!("--sync_base_data    to sync items and servers data, usable at most after patches when new items was added or new server was created!");
            println!("--sync_trades {DATA_CENTER_NAME_ARG}DATA_CENTER_NAME {HOME_WORLD_NAME_ARG}HOME_WORLD_NAME    this one will update database market data for all items. Calculation of sale_score and price_diff_score related to home world, means you plan to sell items on this world!");
        }
        "--sync_base_data" => {
            let pool = create_database_pool(config).await;
            println!("Importing items data ...");
            let item_data = ItemData::new(pool.clone());
            let item_import = ItemImport::new(&item_data);
            item_import.import_marketable_items().await.unwrap();
            println!("Done!");

            println!("Importing servers data ...");
            let server_data = ServerData::new(pool.clone());
            let server_import = ServerImport::new(&server_data);
            server_import.import_servers().await.unwrap();
            println!("Done!");

            println!("Base data successfully synced!");
        }
        "--sync_trades" => {
            let pool = create_database_pool(config).await;
            let item_trades = ItemTrades::new(pool.clone());
            let server_data = ServerData::new(pool.clone());
            let item_data = ItemData::new(pool.clone());
            let market_imports = MarketImport::new(&item_trades, &server_data, &item_data);
            let mut data_center_name: String = String::new();
            let mut home_world_name: String = String::new();
            let args: Vec<String> = env::args().collect();
            if args.len() < 3 {
                panic!("not enough arguments");
            }
            if args[2].starts_with(DATA_CENTER_NAME_ARG) {
                data_center_name = str::replace(&args[2], DATA_CENTER_NAME_ARG, "");
            } else {
                panic!("Data center name missing!")
            }
            if args[3].starts_with(HOME_WORLD_NAME_ARG) {
                home_world_name = str::replace(&args[3], HOME_WORLD_NAME_ARG, "");
            } else {
                panic!("Home world name missing!")
            }

            println!("Importing trades data ...");
            market_imports
                .import_market_trade_volumes(data_center_name, home_world_name)
                .await
                .unwrap();
            println!("Done!");

            println!("Trades data successfully synced!");
        }
        _ => {
            println!("execute `aggregator --help` to see list of available commands")
        }
    };
}

async fn create_database_pool(config: &Config) -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();

    pool
}
