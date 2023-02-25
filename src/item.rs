#[derive(Debug, Copy, Clone)]
pub struct ItemTradeVolume {
    pub id: i64,
    pub item_id: i64,
    pub world_id: i64,
    pub cheapest_world_id: i64,
    pub sale_score: f64,
    pub price_diff_score: f64,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub item_id: i64,
    pub name: String,
}
