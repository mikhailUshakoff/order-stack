use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize)]
pub struct MarketData {
    #[serde(deserialize_with = "uppercase")]
    symbol: String,
    current_price: f64,
    market_cap: u64,
    fully_diluted_valuation: u64,
}

fn uppercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.to_uppercase())
}

impl MarketData {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn price(&self) -> f64 {
        self.current_price
    }

    fn format_large_number(value: u64) -> String {
        let value_f64 = value as f64;
        if value >= 1_000_000_000 {
            format!("{:.2}B", value_f64 / 1_000_000_000.0)
        } else if value >= 1_000_000 {
            format!("{:.2}M", value_f64 / 1_000_000.0)
        } else if value >= 1_000 {
            format!("{:.2}K", value_f64 / 1_000.0)
        } else {
            format!("{}", value)
        }
    }

    pub fn market_cap(&self) -> String {
        MarketData::format_large_number(self.market_cap)
    }

    pub fn fdv(&self) -> String {
        MarketData::format_large_number(self.fully_diluted_valuation)
    }
}
