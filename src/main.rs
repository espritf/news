pub mod models;
pub mod schema;

use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use rss;
use std::env;

use crate::models::{NewChannel, Channel};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch data from given url
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
            let channel = fetch_channel(url)?;
            add_channel(conn, channel);
        }
    }

    Ok(())
}

fn fetch_channel(url: &String) -> Result<NewChannel> {
    let res = reqwest::blocking::get(url)?.text()?;
    let channel = rss::Channel::read_from(res.as_bytes())?;

    Ok(NewChannel {
        title: channel.title,
        link: url.clone(),
        language: channel.language.unwrap(),
        last_build_date: channel.last_build_date.unwrap(),
    })
}

fn add_channel(conn: &mut SqliteConnection, channel: NewChannel) {

    use crate::schema::channels;

    diesel::insert_or_ignore_into(channels::table)
        .values(&channel)
        .returning(Channel::as_returning())
        .get_result(conn)
        .expect("Error saving new channel");
}
