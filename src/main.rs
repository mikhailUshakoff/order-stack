mod cli;
mod commands;
mod market;
mod models;

use clap::Parser;
use cli::{Cli, Commands};
use tokio;

use crate::commands::{
    add_order, add_token, export, import, list_orders, list_tokens, remove_order, remove_token,
    summary,
};

#[tokio::main]
async fn main() -> sled::Result<()> {
    let cli = Cli::parse();
    let db = sled::open("order_db")?;

    match cli.command {
        Commands::AddToken { name, symbol } => add_token(&db, name, symbol)?,
        Commands::RemoveToken { symbol } => remove_token(&db, &symbol)?,
        Commands::ListTokens => list_tokens(&db)?,
        Commands::AddOrder {
            symbol,
            side,
            date,
            volume,
            spent_usdt,
            note,
        } => add_order(&db, symbol, side, date, volume, spent_usdt, note)?,
        Commands::ListOrders { symbol } => list_orders(&db, &symbol)?,
        Commands::Summary => summary(&db).await?,
        Commands::RemoveOrder { id } => remove_order(&db, id)?,
        Commands::Import => import(&db, "import")?,
        Commands::Export => export(&db, "export")?,
    }

    Ok(())
}
