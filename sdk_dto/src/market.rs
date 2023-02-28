#[derive(Debug, Clone)]
pub struct ItemVelocity {
    pub nq_velocity: f64,
    pub hq_velocity: f64,
}

impl ItemVelocity {
    pub fn get_better_velocity(&self) -> f64 {
        if self.hq_velocity > 0.0 {
            return self.hq_velocity;
        }

        self.nq_velocity
    }
}

#[derive(Debug, Clone)]
pub struct ItemSaleHistoryUnit {
    pub quantity: f64,
    pub price_per_unit: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ItemTradeVolume {
    pub item_id: u64,
    pub world_id: u64,
    pub cheapest_world_id: u64,
    pub sale_score: f64,
    pub price_diff_score: f64,
}
