use crate::models::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPosition {
    pub volume: f64,
    pub spent_usdt: f64,
}

impl TokenPosition {
    pub fn get_avg_price(&self) -> f64 {
        if self.volume.abs() > f64::EPSILON {
            if self.spent_usdt < 0.0 {
                0.0
            } else {
                self.spent_usdt / self.volume
            }
        } else {
            0.0
        }
    }

    pub fn get_ratio(&self, current_value: f64) -> f64 {
        if self.spent_usdt > f64::EPSILON {
            current_value / self.spent_usdt * 100.0
        } else {
            0.0
        }
    }

    pub fn add(&mut self, side: &Side, volume: f64, spent_usdt: f64) {
        match side {
            Side::Buy => {
                self.volume += volume;
                self.spent_usdt += spent_usdt;
            }
            Side::Sell => {
                self.volume -= volume;
                self.spent_usdt -= spent_usdt;
            }
        }
        if self.volume < 0.0 {
            self.volume = 0.0;
        }
    }

    pub fn remove(&mut self, side: &Side, volume: f64, spent_usdt: f64) {
        match side {
            Side::Buy => {
                self.volume -= volume;
                self.spent_usdt -= spent_usdt;
            }
            Side::Sell => {
                self.volume += volume;
                self.spent_usdt += spent_usdt;
            }
        }
        if self.volume < 0.0 {
            self.volume = 0.0;
        }
    }
}
