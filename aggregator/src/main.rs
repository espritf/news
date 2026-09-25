pub mod schema;
pub mod collector;
pub mod translator;
pub mod publisher;
pub mod error;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use diesel::prelude::*;
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
    dotenv_flow::dotenv_flow().ok();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();
    let conn = &mut SqliteConnection::establish(&env::var("DATABASE_URL")?)?;

    match &cli.command {
        Commands::Fetch => {
            tracing::info!("Start fetch task");
            let sources_toml = fs::read_to_string("sources.toml")
                .context("Failed to read sources.toml (expected in the current working directory)")?;
            let config: Config = toml::from_str(sources_toml.as_str())
                .context("Failed to parse sources.toml")?;
            collector::collect(conn, config.sources)?;
        },
        Commands::Publish => {
            tracing::info!("Start publish task");
            publisher::publish(conn)?;
        }
    }

    Ok(())
}
