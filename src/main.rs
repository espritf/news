pub mod models;
pub mod schema;
pub mod collector;
pub mod repository;
pub mod error;

use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add channel
    Add { url: String },
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();
    let conn = &mut establish_connection();

    match &cli.command {
        Commands::Add { url } => {
            println!("add channel from url: {url:?}");
            let channel = collector::fetch_channel(url)?;
            repository::add_channel(conn, channel);
        }
    }

    Ok(())
}
