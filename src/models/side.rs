use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum Side {
    Buy,
    Sell,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "buy "),
            Side::Sell => write!(f, "sell"),
        }
    }
}
