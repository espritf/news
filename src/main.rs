use clap::{Parser, Subcommand};
use anyhow::Result;

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

    println!("{res}");
    Ok(())
}
