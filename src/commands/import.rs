use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::commands::{add_order, add_token};
use crate::models::{Side, Token};

fn normalize_date(input: &str) -> sled::Result<String> {
    let digits: Vec<&str> = input
        .split(|c: char| !c.is_ascii_digit()) // split on any non-digit character
        .filter(|s| !s.is_empty())
        .collect();
    if digits.len() == 3 {
        let (day, month, year) = (digits[0], digits[1], digits[2]);
        if year.len() == 2 {
            return Ok(format!("20{}-{:0>2}-{:0>2}", year, month, day));
        }
        Ok(format!("{:0>4}-{:0>2}-{:0>2}", year, month, day))
    } else {
        Err(sled::Error::ReportableBug(
            "Invalid date format".to_string(),
        ))
    }
}

fn split_csv_line(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ',' if !in_quotes => {
                result.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        result.push(current.trim().to_string());
    }

    result
}

pub fn import(db: &sled::Db, folder_path: &str) -> sled::Result<()> {
    let folder = Path::new(folder_path);

    if !folder.exists() || !folder.is_dir() {
        println!(
            "❌ Folder '{}' does not exist or is not a directory",
            folder_path
        );
        return Ok(());
    }

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();
        println!("path: {:?}", path);
        if path.is_file() {
            let content = fs::read_to_string(&path)?;
            let mut lines = content.lines();

            let symbol = match lines.next() {
                Some(line) => line.trim().to_uppercase(),
                None => {
                    println!("❌ Empty file: {:?}", path);
                    continue;
                }
            };

            let name = match lines.next() {
                Some(line) => line.trim().to_string(),
                None => {
                    println!("❌ Missing token name in file: {:?}", path);
                    continue;
                }
            };

            // Insert token
            let token = Token {
                name,
                symbol: symbol.clone(),
            };
            add_token(db, token.name, token.symbol)?;

            // Add orders
            for line in lines {
                let fields = split_csv_line(line);

                if fields.len() < 3 {
                    println!("❌ Invalid line: {}", line);
                    continue;
                }

                let date_str = &fields[0];
                let volume_str = fields[1].replace(',', ".");
                let spent_str = fields[2].replace(',', ".");

                let date_parsed = normalize_date(date_str)?;
                let mut volume = f64::from_str(&volume_str).unwrap_or(0.0);
                let mut spent = f64::from_str(&spent_str).unwrap_or(0.0);
                let side = if volume < 0.0 {
                    volume *= -1.0;
                    spent *= -1.0;
                    Side::Sell
                } else {
                    Side::Buy
                };

                add_order(db, symbol.clone(), side, date_parsed, volume, spent, None)?;
            }

            println!(
                "✅ Imported token {} from {:?}",
                symbol,
                path.file_name().unwrap()
            );
        }
    }

    Ok(())
}
