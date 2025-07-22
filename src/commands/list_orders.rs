use crate::models::{
    constants::{PREFIX_ORDER, PREFIX_POSITION}, Order, Side, TokenPosition
};

pub fn get_color(side: &Side) -> &str {
    match side {
        //Side::Buy => "\x1b[38;5;42m",
        //Side::Sell => "\x1b[38;5;196m",
        Side::Buy => "\x1b[38;5;118m",
        Side::Sell => "\x1b[38;5;204m",
    }
}

pub fn list_orders(db: &sled::Db, symbol: &str) -> sled::Result<()> {
    let symbol_upper = symbol.to_uppercase();
    println!("--- Orders for {} ---", symbol_upper);

    let position_key = format!("{}/{}", PREFIX_POSITION, symbol_upper);
    if let Some(val) = db.get(&position_key)? {
        let position: TokenPosition = serde_json::from_slice(&val)
            .map_err(|e| sled::Error::ReportableBug(format!("position parse error: {}", e)))?;
        let avg_price = if position.volume.abs() > f64::EPSILON {
            if position.spent_usdt < 0.0 {
                0.0
            } else {
                position.spent_usdt / position.volume
            }
        } else {
            0.0
        };

        println!(
            "📊 \x1b[38;5;50mNet Position: {:>35.4} {:>15.4} {:>15.4}\x1b[0m",
            position.volume, position.spent_usdt, avg_price
        );
    } else {
        println!("📊 No position for {}", symbol_upper);
    }

    let prefix = format!("{}/{}/", PREFIX_ORDER, symbol_upper);
    let mut orders: Vec<Order> = vec![];

    for entry in db.scan_prefix(prefix) {
        let (_, val) = entry?;
        let order: Order = serde_json::from_slice(&val)
            .map_err(|e| sled::Error::ReportableBug(format!("order parse error: {}", e)))?;
        orders.push(order);
    }

    orders.sort_by(|a, b| a.date.cmp(&b.date));

    println!("{}", "-".repeat(89));
    println!(
        "{:<18} {:<12} {:<7} {:>12} {:>15} {:>15}  {}",
        "ID", "Date", "Side", "Volume", "USDT", "Price", "Note"
    );
    println!("{}", "-".repeat(89));

    for order in orders {
        println!(
            "{:<18} {:<12} {}{:<7} {:>15.4} {:>15.4} {:>15.4} {}\x1b[0m",
            format!("{:016}", order.id),
            order.date,
            get_color(&order.side),
            order.side,
            order.volume,
            order.spent_usdt,
            order.price(),
            order.note.as_deref().unwrap_or(""),
        );
    }

    Ok(())
}
