mod db;
mod import;

use crate::db::{ItemData, ItemTrades, ServerData};
use crate::import::{ItemImport, MarketImport, ServerImport};
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,
}

#[derive(Parser)]
#[command(name = "FFXVI Market Board Aggregator")]
#[command(author = "Koala")]
#[command(version = "0.1.0")]
#[command(about = "Aggregates data from market board api(Universalis https://docs.universalis.app/)", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SyncBaseData,
    SyncTrades(SyncTradesArgs),
}

#[derive(Args)]
struct SyncTradesArgs {
    data_center_name: String,
    home_world_name: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{error:#?}"),
    };

    let cli = Cli::parse();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();

    match &cli.command {
        Commands::SyncBaseData => {
            sync_base_data(pool).await;
        }
        Commands::SyncTrades(args) => {
            sync_trades(args, pool).await;
        }
    }
}

async fn sync_base_data(pool: Pool<MySql>) {
    println!("Importing items data ...");
    let item_data = ItemData::new(pool.clone());
    let item_import = ItemImport::new(item_data);
    item_import.import_marketable_items().await.unwrap();
    println!("Done!");

    println!("Syncing items data ...");
    item_import.sync_items().await.unwrap();
    println!("Done!");

    println!("Importing servers data ...");
    let server_data = ServerData::new(pool.clone());
    let server_import = ServerImport::new(server_data);
    server_import.import_servers().await.unwrap();
    println!("Done!");

    println!("Base data successfully synced!");
}

async fn sync_trades(args: &SyncTradesArgs, pool: Pool<MySql>) {
    let item_trades = ItemTrades::new(pool.clone());
    let server_data = ServerData::new(pool.clone());
    let item_data = ItemData::new(pool.clone());
    let market_imports = MarketImport::new(item_trades, server_data, item_data);

    println!("Importing trades data ...");
    market_imports
        .import_market_trade_volumes(args.data_center_name.clone(), args.home_world_name.clone())
        .await
        .unwrap();
    println!("Done!");

    println!("Trades data successfully synced!");
}
