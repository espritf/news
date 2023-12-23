pub mod collector;
pub mod translator;
pub mod error;
pub mod schema;

use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;
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
    Fetch,
    /// Publish translated news
    Publish,
}

#[derive(Deserialize, Debug)]
struct Config {
    sources: Vec<collector::sources::Config>,
}

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();
    let conn = &mut SqliteConnection::establish(&env::var("DATABASE_URL")?)?;

    match &cli.command {
        Commands::Fetch => {
            let config: Config = toml::from_str(fs::read_to_string("sources.toml")?.as_str())?;
            let _ = collector::collect(conn, config.sources)?;
        },
        Commands::Publish => {
            let _ = translator::publish(conn)?;
        }
    }

    Ok(())
}
