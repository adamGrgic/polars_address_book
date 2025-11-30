use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;

mod commands;

use commands::{
    generate::{execute_generate, GenerateArgs},
    database::{handle_db_command, DatabaseArgs},
};

#[derive(Parser)]
#[command(name = "address-book")]
#[command(about = "Rust Polars Address Book", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate list of addresses
    Generate(GenerateArgs),
    /// Database management commands
    Database(DatabaseArgs),
}

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_address_book=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => {
            info!("generate was chosen");
            let result = execute_generate(args);
            info!("result: {:?}", result);
        }
        Commands::Database(args) => {
            if let Err(e) = handle_db_command(args).await {
                eprintln!("Error executing database command: {}", e);
            }
        }
    }
    Ok(())
}
