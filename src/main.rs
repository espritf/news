use clap::{Parser, Subcommand};
use anyhow::Result;
use rss::Channel;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch data from given url
    Fetch { url: String },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Fetch { url } => {
            println!("get feed from: {url:?}");
            let _ = fetch(url);
        }
    }
}

fn fetch(url: &String) -> Result<()> {
    let res = reqwest::blocking::get(url)?.text()?;
    let channel = Channel::read_from(res.as_bytes())?;

    for item in channel.items().iter() {
        println!("{:#?}", item);
    }

    Ok(())
}
