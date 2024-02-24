use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;

use news::collector;
use news::publisher;

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
    dotenv_flow::dotenv_flow().ok();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();
    let conn = &mut SqliteConnection::establish(&env::var("DATABASE_URL")?)?;

    match &cli.command {
        Commands::Fetch => {
            tracing::info!("Start fetch task");
            let config: Config = toml::from_str(fs::read_to_string("sources.toml")?.as_str())?;
            collector::collect(conn, config.sources)?;
        },
        Commands::Publish => {
            tracing::info!("Start publish task");
            publisher::publish(conn)?;
        }
    }

    Ok(())
}
