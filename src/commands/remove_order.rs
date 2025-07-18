use crate::models::{
    constants::{PREFIX_ORDER, PREFIX_POSITION},
    Order, TokenPosition,
};

pub fn remove_order(db: &sled::Db, id: u64) -> sled::Result<()> {
    let prefix = format!("{}/", PREFIX_ORDER);
    let mut found = false;

    for entry in db.scan_prefix(prefix) {
        let (key_bytes, val_bytes) = entry?;
        let key = std::str::from_utf8(&key_bytes).unwrap();

        if key.ends_with(&format!("{:016}", id)) {
            let order: Order = serde_json::from_slice(&val_bytes)
                .map_err(|e| sled::Error::ReportableBug(format!("order parse error: {}", e)))?;

            db.remove(key)?;
            println!("✅ Removed order with id {}", id);
            found = true;

            let position_key = format!("{}/{}", PREFIX_POSITION, order.symbol);
            let mut position: TokenPosition = match db.get(&position_key)? {
                Some(val) => serde_json::from_slice(&val).map_err(|e| {
                    sled::Error::ReportableBug(format!("position parse error: {}", e))
                })?,
                None => TokenPosition {
                    volume: 0.0,
                    spent_usdt: 0.0,
                },
            };

            position.remove(&order.side, order.volume, order.spent_usdt);

            let updated_position = serde_json::to_vec(&position).map_err(|e| {
                sled::Error::ReportableBug(format!("position serialize error: {}", e))
            })?;
            db.insert(position_key, updated_position)?;

            break;
        }
    }

    if !found {
        println!("❌ Order with id {} not found", id);
    }

    Ok(())
}
