use crate::models::{constants::PREFIX_TOKEN, Token};

pub fn add_token(db: &sled::Db, name: String, symbol: String) -> sled::Result<()> {
    let symbol = symbol.to_uppercase();
    let key = format!("{}/{}", PREFIX_TOKEN, symbol);
    let token = Token {
        name: name.to_lowercase(),
        symbol,
    };
    let token_value =
        serde_json::to_vec(&token).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
    db.insert(key, token_value)?;
    println!("✅ Token added: {} ({})", token.name, token.symbol);
    Ok(())
}
