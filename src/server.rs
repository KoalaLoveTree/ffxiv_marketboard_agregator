#[derive(Debug, Clone)]
pub struct DataCenter {
    pub id: i64,
    pub name: String,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct World {
    pub world_id: i64,
    pub name: String,
    pub data_center_id: i64,
}
