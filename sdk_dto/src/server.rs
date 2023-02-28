#[derive(Debug, Clone)]
pub struct DataCenter {
    pub name: String,
    pub region: String,
    pub worlds_ids: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct World {
    pub id: u64,
    pub name: String,
}

pub struct Server {
    pub data_center: DataCenter,
    pub worlds: Vec<World>,
}
