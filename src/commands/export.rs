use std::fs::{create_dir_all, File};
use std::io::Write;
use crate::models::{constants::{PREFIX_TOKEN, PREFIX_ORDER}, Token, Order};

pub fn export(db: &sled::Db, output_dir: &str) -> sled::Result<()> {
    create_dir_all(output_dir).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;

    for entry in db.scan_prefix(format!("{}/", PREFIX_TOKEN)) {
        let (_, val) = entry?;
        let token: Token = serde_json::from_slice(&val)
            .map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
        let symbol = token.symbol.to_uppercase();
        let file_path = format!("{}/{}.txt", output_dir, symbol);
        let mut file = File::create(&file_path)
            .map_err(|e| sled::Error::ReportableBug(e.to_string()))?;

        writeln!(file, "{}", symbol).unwrap();
        writeln!(file, "{}", token.name).unwrap();

        let order_prefix = format!("{}/{}/", PREFIX_ORDER, symbol);
        for entry in db.scan_prefix(order_prefix) {
            let (_, val) = entry?;
            let order: Order = serde_json::from_slice(&val)
                .map_err(|e| sled::Error::ReportableBug(e.to_string()))?;

            writeln!(
                file,
                "{},{},{},",
                order.date, order.volume, order.spent_usdt,
            )
            .unwrap();
        }

        println!("✅ Exported {}", symbol);
    }

    Ok(())
}
