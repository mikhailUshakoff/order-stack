use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
}
