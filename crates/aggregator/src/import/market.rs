use crate::import::errors::Error;
use db_sdk::{DBWorld, ItemData, ItemTrades, ServerData};
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use universalis_sdk::{
    get_item_sale_history_by_world, get_item_velocity_by_world, ItemTradeVolume,
};

pub struct MarketImport<'a> {
    item_trades: &'a ItemTrades,
    server_data: &'a ServerData,
    item_data: &'a ItemData,
}

struct LowestAverageItemPrice {
    world_id: u64,
    price: f64,
    home_world_price: f64,
}

impl<'a> MarketImport<'a> {
    pub fn new(
        item_trades: &'a ItemTrades,
        server_data: &'a ServerData,
        item_data: &'a ItemData,
    ) -> Self {
        Self {
            item_trades,
            server_data,
            item_data,
        }
    }

    pub async fn import_market_trade_volumes(
        &self,
        data_center_name: String,
        home_world_name: String,
    ) -> Result<(), Error> {
        let server = self.server_data.get_server(data_center_name).await?;

        let (home_world, items) = tokio::try_join!(
            self.server_data
                .get_data_center_world_by_name(home_world_name, server.data_center.id),
            self.item_data.get_items()
        )?;

        let mut handles = HashMap::new();

        for world in &server.worlds {
            let mut items_handles = Vec::new();
            for items_chunk in items.chunks(100) {
                let chunk_ids: Vec<u64> = items_chunk.iter().map(|item| item.item_id).collect();
                items_handles.push(tokio::spawn(get_item_sale_history_by_world(
                    chunk_ids,
                    world.name.clone(),
                )));
            }
            handles.insert(&world.name, items_handles);
        }

        let mut lowest_avg_items_prices: HashMap<u64, LowestAverageItemPrice> = HashMap::new();

        for world in &server.worlds {
            let sale_history_results = handles.get_mut(&world.name).ok_or(crate::import::errors::Error::HashMapAccess)?;

            for sale_history_unit in sale_history_results {
                let sale_history = sale_history_unit.await??;
                for (item_id, item_sale_history) in sale_history.items {
                    let mut total_gil_spent = 0;
                    let mut quantity = 0;

                    for sale_history_unit in item_sale_history.entries {
                        total_gil_spent +=
                            sale_history_unit.price_per_unit * sale_history_unit.quantity;
                        quantity += sale_history_unit.quantity;
                    }

                    if let Vacant(e) = lowest_avg_items_prices.entry(item_id) {
                        let mut home_world_price = 0.0;
                        if home_world.world_id == world.world_id {
                            home_world_price = total_gil_spent as f64 / quantity as f64;
                        }
                        e.insert(LowestAverageItemPrice {
                            world_id: world.world_id,
                            price: total_gil_spent as f64 / quantity as f64,
                            home_world_price,
                        });
                    } else {
                        let lowest_avg_item_price = lowest_avg_items_prices
                            .get_mut(&item_id)
                            .ok_or(crate::import::errors::Error::HashMapAccess)?;

                        if lowest_avg_item_price.price > total_gil_spent as f64 / quantity as f64 {
                            lowest_avg_item_price.price = total_gil_spent as f64 / quantity as f64;
                            lowest_avg_item_price.world_id = world.world_id;
                            if home_world.world_id == world.world_id {
                                lowest_avg_item_price.home_world_price =
                                    total_gil_spent as f64 / quantity as f64;
                            }
                        }
                    }
                }
            }
        }

        let mut trade_volumes_handlers = Vec::new();

        for item in items {
            let lowest_avg_item_price = lowest_avg_items_prices
                .remove(&item.item_id)
                .ok_or(crate::import::errors::Error::HashMapAccess)?;
            trade_volumes_handlers.push(tokio::spawn(
                MarketImport::avg_item_prices_to_trade_volume(
                    item.item_id,
                    home_world.clone(),
                    lowest_avg_item_price,
                ),
            ));
        }

        let mut items_trade_volumes: Vec<ItemTradeVolume> = Vec::new();

        for trade_volumes_handler in trade_volumes_handlers {
            items_trade_volumes.push(trade_volumes_handler.await??);
        }

        self.item_trades
            .save_item_trade_volumes(items_trade_volumes)
            .await?;

        Ok(())
    }

    async fn avg_item_prices_to_trade_volume(
        item_id: u64,
        home_world: DBWorld,
        lowest_avg_item_price: LowestAverageItemPrice,
    ) -> Result<ItemTradeVolume, Error> {
        let item_velocity = get_item_velocity_by_world(item_id, home_world.name.clone()).await?;

        let item_trade_volume = ItemTradeVolume {
            item_id,
            world_id: home_world.world_id,
            cheapest_world_id: lowest_avg_item_price.world_id,
            sale_score: item_velocity.get_better_velocity(),
            price_diff_score: lowest_avg_item_price.home_world_price / lowest_avg_item_price.price,
            home_world_avg_price: lowest_avg_item_price.home_world_price,
        };

        Ok(item_trade_volume)
    }
}
