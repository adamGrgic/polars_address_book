use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;



mod commands;

use commands::{
    execute_generate,
    GenerateArgs,
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
    /// Start the admin daemon server
    Generate(GenerateArgs),

}



fn main() -> Result<()>{
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_address_book=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    // let args = Args::parse();
    let cli = Cli::parse();

    match cli.command {
         Commands::Generate(args) => {
            info!("generate was chosen");
            let result = execute_generate(args);

            info!("result: {:?}", result);

        }
    }
    Ok(())
}
