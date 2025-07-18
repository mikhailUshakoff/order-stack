use crate::models::{
    constants::{PREFIX_ORDER, PREFIX_POSITION, PREFIX_TOKEN},
    Order, Side, TokenPosition,
};
use serde_json::{to_value, Value};

pub fn add_order(
    db: &sled::Db,
    symbol: String,
    side: Side,
    date: String,
    volume: f64,
    spent_usdt: f64,
    note: Option<String>,
) -> sled::Result<()> {
    let symbol_upper = symbol.to_uppercase();
    let token_key = format!("{}/{}", PREFIX_TOKEN, symbol_upper);
    db.get(&token_key)?
        .ok_or(sled::Error::ReportableBug("Token not found".to_string()))?;

    let id = db.generate_id()?;
    let key = format!("{}/{}/{}/{:016}", PREFIX_ORDER, symbol_upper, side, id);
    let order = Order {
        id,
        symbol: symbol_upper.clone(),
        side: side.clone(),
        date,
        volume,
        spent_usdt,
        note,
    };

    let order_value: Value =
        to_value(&order).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
    let value =
        serde_json::to_vec(&order_value).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
    db.insert(key, value)?;
    println!(
        "✅ Order added for {} ({}) date: {} volume: {} spent_usdt: {}",
        symbol_upper, side, order.date, order.volume, order.spent_usdt
    );

    let position_key = format!("{}/{}", PREFIX_POSITION, symbol_upper);
    let mut position_value = match db.get(&position_key)? {
        Some(val) => serde_json::from_slice::<TokenPosition>(&val).map_err(|e| {
            sled::Error::ReportableBug(format!("position deserialization error: {}", e))
        })?,
        None => TokenPosition {
            volume: 0.0,
            spent_usdt: 0.0,
        },
    };

    position_value.add(&order.side, order.volume, order.spent_usdt);

    let serialized = serde_json::to_vec(&position_value)
        .map_err(|e| sled::Error::ReportableBug(format!("position serialization error: {}", e)))?;
    db.insert(position_key, serialized)?;

    Ok(())
}
