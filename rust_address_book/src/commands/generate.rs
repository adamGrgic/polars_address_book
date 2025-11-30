use anyhow::Result;
use clap::{Args, Subcommand};
use fake::Fake;
use tracing::info;
use fake::faker::address::en;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub command: GenerateCommands,
}

#[derive(Subcommand, Debug)]
pub enum GenerateCommands {
    Addresses {
        #[arg(short,long, default_value="10")]
        count: u32,


    }
}


pub fn execute_generate(args: GenerateArgs) -> Result<()> {
    match args.command {
        GenerateCommands::Addresses { count } => {
            info!("generating addresses with count {}", count);
            let address: String = en::StreetName().fake();
            info!("new address: {:?}", address);
        }
    }

    Ok(())
}
