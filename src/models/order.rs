use serde::{Deserialize, Serialize};

use crate::models::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub date: String,
    pub volume: f64,
    pub spent_usdt: f64,
    pub note: Option<String>,
}

impl Order {
    pub fn price(&self) -> f64 {
        if self.volume == 0.0 {
            return 0.0;
        }
        self.spent_usdt / self.volume
    }
}
