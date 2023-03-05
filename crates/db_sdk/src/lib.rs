pub mod errors;

use crate::errors::Error;
use sqlx::{MySql, Pool, QueryBuilder};
use universalis_sdk::{ItemTradeVolume, Server};
use xivapi_sdk::Item;

const BIND_LIMIT: usize = 65535;

#[derive(Debug, Clone)]
pub struct DBDataCenter {
    pub id: u64,
    pub name: String,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct DBWorld {
    pub world_id: u64,
    pub name: String,
    pub data_center_id: u64,
}

pub struct DBServer {
    pub data_center: DBDataCenter,
    pub worlds: Vec<DBWorld>,
}
#[derive(Debug, Clone)]
pub struct DBItem {
    pub item_id: u64,
    pub name: String,
}

#[derive(Debug, Copy, Clone)]
pub struct DBItemTradeVolume {
    pub id: u64,
    pub item_id: u64,
    pub world_id: u64,
    pub cheapest_world_id: u64,
    pub sale_score: f64,
    pub price_diff_score: f64,
}

pub struct ItemData {
    pool: Pool<MySql>,
}

impl ItemData {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn save_items(&self, items: Vec<Item>) -> Result<(), Error> {
        let mut query_builder: QueryBuilder<MySql> =
            QueryBuilder::new("INSERT IGNORE INTO items(item_id, name)");

        let items_chunks = items.chunks(BIND_LIMIT / 2);

        for items_chunk in items_chunks {
            query_builder.push_values(items_chunk, |mut b, item| {
                b.push_bind(item.id).push_bind(&item.name);
            });

            query_builder.build().execute(&self.pool).await?;
        }

        Ok(())
    }

    pub async fn get_items(&self) -> Result<Vec<DBItem>, Error> {
        //TODO remove limit, added for tests
        let items = sqlx::query_as!(DBItem, r"SELECT * FROM items LIMIT 100")
            .fetch_all(&self.pool)
            .await?;

        Ok(items)
    }
}

pub struct ItemTrades {
    pool: Pool<MySql>,
}

impl ItemTrades {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn save_item_trade_volumes(
        &self,
        items_trade_volumes: Vec<ItemTradeVolume>,
    ) -> Result<(), Error> {
        let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
            "INSERT IGNORE INTO items_trade_volumes (item_id, world_id, sale_score, price_diff_score, cheapest_world_id, home_world_avg_price)"
        );

        let items_trade_volumes_chunks =
            items_trade_volumes.chunks(BIND_LIMIT / 6);

        for items_trade_volumes_chunk in items_trade_volumes_chunks {
            query_builder.push_values(items_trade_volumes_chunk, |mut b, item_trade_volume| {
                b.push_bind(item_trade_volume.item_id)
                    .push_bind(item_trade_volume.world_id)
                    .push_bind(item_trade_volume.sale_score)
                    .push_bind(item_trade_volume.price_diff_score)
                    .push_bind(item_trade_volume.cheapest_world_id)
                    .push_bind(item_trade_volume.home_world_avg_price);
            });

            query_builder.build().execute(&self.pool).await?;
        }

        Ok(())
    }

    pub async fn get_item_trade_volume_by_world(
        &self,
        world_id: u64,
        item_id: u64,
    ) -> Result<DBItemTradeVolume, Error> {
        let item_trade_volume = sqlx::query_as!(
            DBItemTradeVolume,
            r"SELECT * FROM items_trade_volumes WHERE item_id = ? AND world_id = ?",
            item_id,
            world_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item_trade_volume)
    }
}

pub struct ServerData {
    pool: Pool<MySql>,
}

impl ServerData {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn save_servers(&self, servers: Vec<Server>) -> Result<(), Error> {
        for server in servers {
            let data_center_id: u64 = sqlx::query!(
                "INSERT IGNORE INTO data_centers (name, region) VALUES (?, ?)",
                server.data_center.name,
                server.data_center.region
            )
            .execute(&self.pool)
            .await?
            .last_insert_id();
            for world in server.worlds {
                if server
                    .data_center
                    .worlds
                    .iter()
                    .any(|&dc_world_id| dc_world_id == world.id)
                {
                    sqlx::query!(
                    "INSERT IGNORE INTO worlds (world_id, name, data_center_id) VALUES (?, ?, ?)",
                    world.id,
                    world.name,
                    data_center_id
                )
                    .execute(&self.pool)
                    .await?;
                }
            }
        }

        Ok(())
    }

    pub async fn get_server(&self, data_center_name: String) -> Result<DBServer, Error> {
        let data_center = sqlx::query_as!(
            DBDataCenter,
            "SELECT * FROM data_centers WHERE name = ?",
            data_center_name
        )
        .fetch_one(&self.pool)
        .await?;

        let worlds = sqlx::query_as!(
            DBWorld,
            "SELECT * FROM worlds WHERE data_center_id = ?",
            data_center.id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(DBServer {
            data_center,
            worlds,
        })
    }

    pub async fn get_data_center_world_by_name(
        &self,
        world_name: String,
        data_center_id: u64,
    ) -> Result<DBWorld, Error> {
        let world = sqlx::query_as!(
            DBWorld,
            "SELECT * FROM worlds WHERE name = ? AND data_center_id = ?",
            world_name,
            data_center_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(world)
    }
}
