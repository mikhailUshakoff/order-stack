use crate::models::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPosition {
    volume: f64,
    buy_usdt: f64,
    sell_usdt: f64,
}

impl TokenPosition {
    pub fn new() -> Self {
        TokenPosition {
            volume: 0.0,
            buy_usdt: 0.0,
            sell_usdt: 0.0,
        }
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn buy_usdt(&self) -> f64 {
        self.buy_usdt
    }

        pub fn sell_usdt(&self) -> f64 {
            self.sell_usdt
        }

    pub fn spent_usdt(&self) -> f64 {
        self.buy_usdt - self.sell_usdt
    }

    pub fn get_avg_price(&self) -> f64 {
        if self.volume.abs() > f64::EPSILON {
            if self.spent_usdt() < 0.0 {
                0.0
            } else {
                self.spent_usdt() / self.volume
            }
        } else {
            0.0
        }
    }

    pub fn position_value(&self, current_price: f64) -> f64 {
        self.volume * current_price
    }

    pub fn get_ratio(&self, current_price: f64) -> f64 {
        let current_value = self.position_value(current_price);
        if self.spent_usdt() > f64::EPSILON {
            current_value / self.spent_usdt() * 100.0
        } else {
            (current_value + self.sell_usdt) / self.buy_usdt * 100.0
        }
    }

    pub fn add(&mut self, side: &Side, volume: f64, spent_usdt: f64) {
        match side {
            Side::Buy => {
                self.volume += volume;
                self.buy_usdt += spent_usdt;
            }
            Side::Sell => {
                self.volume -= volume;
                self.sell_usdt += spent_usdt;
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
                self.buy_usdt -= spent_usdt;
            }
            Side::Sell => {
                self.volume += volume;
                self.sell_usdt -= spent_usdt;
            }
        }
        if self.volume < 0.0 {
            self.volume = 0.0;
        }
    }
}
