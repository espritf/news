pub mod schema;
pub mod collector;
pub mod error;

use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch data from all sources
    Fetch
}

#[derive(Deserialize, Debug)]
struct Config {
    sources: Vec<collector::sources::Config>,
}

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Fetch => {
            let conn = &mut SqliteConnection::establish(&env::var("DATABASE_URL")?)?;
            let config: Config = toml::from_str(fs::read_to_string("sources.toml")?.as_str())?;

            let _ = collector::collect(conn, config.sources)?;
        }
    }

    Ok(())
}
