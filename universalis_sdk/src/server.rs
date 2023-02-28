use error::Error;
use error::Error::JsonMissedDataError;
use sdk_dto::server::{DataCenter, Server, World};
use serde_json::Value;

pub async fn get_servers() -> Result<Vec<Server>, Error> {
    let data_centers = get_data_centers().await?;
    let mut servers: Vec<Server> = Vec::new();
    for data_center in data_centers {
        let data_center_worlds = get_data_center_worlds(&data_center).await?;
        servers.push(Server {
            data_center,
            worlds: data_center_worlds,
        })
    }

    Ok(servers)
}

async fn get_data_center_worlds(data_center: &DataCenter) -> Result<Vec<World>, Error> {
    let body_worlds = reqwest::get("https://universalis.app/api/v2/worlds")
        .await?
        .text()
        .await?;

    let worlds_data: Value = serde_json::from_str(&body_worlds)?;

    let worlds = worlds_data.as_array().ok_or( JsonMissedDataError {
        field_name: "Worlds".to_string(),
    })?;

    let mut data_center_worlds: Vec<World> = Vec::new();

    for world in worlds {
        let world_id = world["id"].as_u64().ok_or( JsonMissedDataError {
            field_name: "Worlds.world.id".to_string(),
        })?;
        let world_name = world["name"].as_str().ok_or( JsonMissedDataError {
            field_name: "Worlds.world.name".to_string(),
        })?;
        if data_center
            .worlds_ids
            .iter()
            .any(|&dc_world_id| dc_world_id == world_id)
        {
            data_center_worlds.push(World {
                id: world_id,
                name: world_name.to_string(),
            });
        }
    }

    Ok(data_center_worlds)
}

async fn get_data_centers() -> Result<Vec<DataCenter>, Error> {
    let body_data_centers = reqwest::get("https://universalis.app/api/v2/worlds")
        .await?
        .text()
        .await?;

    let data_centers_data: Value = serde_json::from_str(&body_data_centers)?;

    let data_centers = data_centers_data
        .as_array()
        .ok_or( JsonMissedDataError {
            field_name: "DataCenters".to_string(),
        })?;

    let mut data_centers_map: Vec<DataCenter> = Vec::new();

    for data_center in data_centers {
        let data_center_name = data_center["name"]
            .as_str()
            .ok_or( JsonMissedDataError {
                field_name: "data_center.name".to_string(),
            })?;
        let data_center_region =
            data_center["region"]
                .as_str()
                .ok_or( JsonMissedDataError {
                    field_name: "data_center.region".to_string(),
                })?;
        let data_center_worlds =
            data_center["worlds"]
                .as_array()
                .ok_or( JsonMissedDataError {
                    field_name: "data_center.region".to_string(),
                })?;

        let mut data_center_worlds_ids: Vec<u64> = Vec::new();

        for data_center_world in data_center_worlds {
            let data_center_world_id =
                data_center_world
                    .as_u64()
                    .ok_or( JsonMissedDataError {
                        field_name: "data_center.worlds.id".to_string(),
                    })?;

            data_center_worlds_ids.push(data_center_world_id);
        }

        data_centers_map.push(DataCenter {
            name: data_center_name.to_string(),
            region: data_center_region.to_string(),
            worlds_ids: data_center_worlds_ids,
        });
    }

    Ok(data_centers_map)
}
