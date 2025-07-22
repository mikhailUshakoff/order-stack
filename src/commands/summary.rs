use std::collections::HashMap;

use crate::{
    market::fetch_market_data,
    models::{
        constants::{PREFIX_POSITION, PREFIX_TOKEN}, MarketData, Token, TokenPosition
    },
};

pub async fn summary(db: &sled::Db) -> sled::Result<()> {
    let mut total_spent = 0.0;
    let mut total_value = 0.0;

    println!(
        "{:<8} {:>12} {:>15} {:>15} {:>15} {:>15} {:>10} {:>10} {:>10}",
        "Symbol",
        "Volume",
        "Spent USDT",
        "Avg Price",
        "Cur Price",
        "Cur Value",
        "RATIO",
        "MCap",
        "FDV"
    );
    println!("{}", "-".repeat(120));

    // Collect all symbols
    let mut symbols = Vec::new();
    for entry in db.scan_prefix(format!("{}/", PREFIX_TOKEN)) {
        let (_, val) = entry?;
        let token: Token =
            serde_json::from_slice(&val).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
        symbols.push(token.symbol);
    }
    let market_data_vec = fetch_market_data(&symbols.join(",")).await?;
    let market_data: HashMap<String, MarketData> = market_data_vec
        .into_iter()
        .map(|d| (d.symbol().to_string(), d))
        .collect();

    let start = std::time::Instant::now();

    let position_prefix = format!("{}/", PREFIX_POSITION);
    let mut line_color = true;
    for entry in db.scan_prefix(&position_prefix) {
        let (key_bytes, val) = entry?;
        let key = std::str::from_utf8(&key_bytes).unwrap();
        let symbol = key.strip_prefix(&position_prefix).unwrap_or("UNKNOWN");

        let position: TokenPosition =
            serde_json::from_slice(&val).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;

        let avg_price = position.get_avg_price();
        let Some(mdata) = market_data.get(symbol) else {
            eprintln!("⚠️  Market data for {} not found", symbol);
            continue;
        };

        let value = position.volume * mdata.price();
        let ratio = position.get_ratio(value);

        total_spent += position.spent_usdt;
        total_value += value;

        line_color = !line_color;

        println!(
            "{}{:<8} {:>12.4} {:>15.4} {:>15.4} {:>15.4} {:>15.4} {:>10.2} {:>10} {:>10}\x1b[0m",
            if line_color { "\x1b[0m" } else { "\x1b[38;5;248m" },
            symbol,
            position.volume,
            position.spent_usdt,
            avg_price,
            mdata.price(),
            value,
            ratio,
            mdata.market_cap(),
            mdata.fdv()
        );
    }

    println!("{}", "-".repeat(120));
    println!(
        "{:<8} {:>12} {:>15.4} {:>15} {:>15} {:>15.4}",
        "TOTAL", "", total_spent, "", "", total_value
    );
    println!(
        "\x1b[38;5;50mTOTAL RATIO: {:.2}%\x1b[0m",
        total_value / total_spent * 100.0
    );

    println!("Time: {} nanos", start.elapsed().as_nanos());
    Ok(())
}
