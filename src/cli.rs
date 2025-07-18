use crate::models::Side;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "order-stack")]
#[command(about = "Manage tokens portfolio using sled")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    AddToken {
        #[arg(long)]
        name: String,
        #[arg(long)]
        symbol: String,
    },
    RemoveToken {
        #[arg(long)]
        symbol: String,
    },
    ListTokens,
    AddOrder {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        side: Side,
        #[arg(long)]
        date: String,
        #[arg(long)]
        volume: f64,
        #[arg(long)]
        spent_usdt: f64,
        #[arg(long)]
        note: Option<String>,
    },
    RemoveOrder {
        #[arg(long)]
        id: u64,
    },
    ListOrders {
        #[arg(long)]
        symbol: String,
    },
    Summary,
    Import,
    Export,
}
