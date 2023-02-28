use db_sdk::item::get_items;
use db_sdk::market::save_item_trade_volumes;
use db_sdk::server::{get_server, get_world};
use error::Error;
use error::Error::HashMapAccessError;
use sdk_dto::market::ItemTradeVolume;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use universalis_sdk::market::{get_item_sale_history_by_world, get_item_velocity_by_world};

pub async fn import_market_trade_volumes(
    data_center_name: String,
    home_world_name: String,
    pool: &Pool<MySql>,
) -> Result<(), Error> {
    let server = get_server(data_center_name, pool).await?;
    let home_world = get_world(home_world_name, server.data_center.id, pool).await?;
    let items = get_items(pool).await?;

    let mut items_trade_volumes: Vec<ItemTradeVolume> = Vec::new();

    for item in items {
        let mut avg_price_per_world: HashMap<u64, f64> = HashMap::new();

        for world in &server.worlds {
            let sale_history = get_item_sale_history_by_world(item.item_id, &world.name).await?;

            let mut total_gil_spent = 0.0;
            let mut quantity = 0.0;

            for sale_history_unit in sale_history {
                total_gil_spent += sale_history_unit.price_per_unit * sale_history_unit.quantity;
                quantity += sale_history_unit.quantity;
            }

            avg_price_per_world.insert(world.world_id, total_gil_spent / quantity);
        }

        let home_world_avg_price = *(avg_price_per_world
            .get(&home_world.world_id)
            .ok_or(HashMapAccessError)?);

        let mut cheapest_world_avg_price: f64 = home_world_avg_price;
        let mut cheapest_world_id: u64 = home_world.world_id;

        for (world_id, avg_price) in avg_price_per_world {
            if cheapest_world_avg_price > avg_price {
                cheapest_world_avg_price = avg_price;
                cheapest_world_id = world_id;
            }
        }

        let item_velocity = get_item_velocity_by_world(item.item_id, &home_world.name).await?;

        items_trade_volumes.push(ItemTradeVolume {
            item_id: item.item_id,
            world_id: home_world.world_id,
            cheapest_world_id,
            sale_score: item_velocity.get_better_velocity(),
            price_diff_score: home_world_avg_price / cheapest_world_avg_price,
        });
    }

    save_item_trade_volumes(items_trade_volumes, pool).await?;

    Ok(())
}
