use clap::{Parser, Subcommand};

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
        }
    }
}
