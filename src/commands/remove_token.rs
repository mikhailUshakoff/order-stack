use crate::models::constants::{PREFIX_ORDER, PREFIX_POSITION, PREFIX_TOKEN};

pub fn remove_token(db: &sled::Db, symbol: &str) -> sled::Result<()> {
    let symbol_upper = symbol.to_uppercase();

    // 1. Remove the token
    let token_key = format!("{}/{}", PREFIX_TOKEN, symbol_upper);
    if db.remove(&token_key)?.is_none() {
        println!("❌ Token {} not found", symbol_upper);
        return Ok(());
    }

    // 2. Remove the net position
    let position_key = format!("{}/{}", PREFIX_POSITION, symbol_upper);
    db.remove(&position_key)?;

    // 3. Remove all related orders
    let order_prefix = format!("{}/{}/", PREFIX_ORDER, symbol_upper);
    let mut removed_count = 0;

    for entry in db.scan_prefix(order_prefix.clone()) {
        let (key_bytes, _) = entry?;
        db.remove(key_bytes)?;
        removed_count += 1;
    }

    println!(
        "✅ Removed token {} with {} order(s) and position",
        symbol_upper, removed_count
    );
    Ok(())
}
