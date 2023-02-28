use error::Error;
use sdk_dto::server::Server;
use sqlx::{MySql, Pool};

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

pub async fn save_servers(servers: Vec<Server>, pool: &Pool<MySql>) -> Result<(), Error> {
    for server in servers {
        let data_center_id: u64 = sqlx::query!(
            r"insert ignore into data_centers (name, region) values (?, ?)",
            server.data_center.name,
            server.data_center.region
        )
        .execute(pool)
        .await?
        .last_insert_id();
        for world in server.worlds {
            if server
                .data_center
                .worlds_ids
                .iter()
                .any(|&dc_world_id| dc_world_id == world.id)
            {
                sqlx::query!(
                    r"insert ignore into worlds (world_id, name, data_center_id) values (?, ?, ?)",
                    world.id,
                    world.name,
                    data_center_id
                )
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

pub async fn get_server(data_center_name: String, pool: &Pool<MySql>) -> Result<DBServer, Error> {
    let data_center: DBDataCenter = sqlx::query_as!(
        DBDataCenter,
        r"select * from data_centers where name = ?",
        data_center_name
    )
    .fetch_one(pool)
    .await?;

    let worlds: Vec<DBWorld> = sqlx::query_as!(
        DBWorld,
        r"select * from worlds where data_center_id = ?",
        data_center.id
    )
    .fetch_all(pool)
    .await?;

    Ok(DBServer {
        data_center,
        worlds,
    })
}

pub async fn get_world(
    world_name: String,
    data_center_id: u64,
    pool: &Pool<MySql>,
) -> Result<DBWorld, Error> {
    Ok(sqlx::query_as!(
        DBWorld,
        r"select * from worlds where name = ? and data_center_id = ?",
        world_name,
        data_center_id
    )
    .fetch_one(pool)
    .await?)
}
