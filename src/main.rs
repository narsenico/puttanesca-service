use clap::{Parser, Subcommand};

mod error;
mod models;
mod hunters;
mod processors;

use error::Error;
use hunters::create_hunter;
use processors::create_processor;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Parser)]
#[clap(about = "puttanesca service", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List,
    Hunt {
        #[clap(required = true, index = 1, help = "Hunter name")]
        hunter: String,
        #[clap(short, long, help = "Processor name", default_value = "console")]
        processor: String,
    },
}

fn list() -> Result<()> {
    println!("Hunters: test");
    println!("Processors: console, sqlite");

    Ok(())
}

async fn hunt(hunter_name: String, processor_name: String) -> Result<()> {
    let hunter = create_hunter(&hunter_name)?;
    let processor = create_processor(&processor_name)?;

    processor.process(hunter).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::List => list(),
        Commands::Hunt { hunter, processor } => hunt(hunter, processor).await,
    }
}
