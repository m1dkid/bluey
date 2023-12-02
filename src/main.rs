mod adapters;
mod services;

use std::str::FromStr;

use crate::adapters::list_adapters;
use crate::services::service;

use bluer::{gatt::local::Characteristic, Address};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists adapters
    List(ListArgs),
    /// Scan a device
    Enumerate(EnumArgs),
    /// Write to a characteristic
    Read(ReadArgs),
}

#[derive(Args)]
struct ListArgs {
    /// List all properties
    #[arg(short, long)]
    all_properties: bool,
}

#[derive(Args)]
struct EnumArgs {
    /// Device address
    #[arg(short = 'b', long)]
    address: String,
    /// Adapter to use
    #[arg(short, long)]
    adapter: String,
}

#[derive(Args)]
struct ReadArgs {
    /// Device address
    #[arg(short = 'b', long)]
    address: String,
    /// Adapter to use
    #[arg(short, long)]
    adapter: String,
    /// Characteristic to read
    #[arg(short, long)]
    characteristic: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let cli = Cli::parse();

    let session = bluer::Session::new().await?;

    match &cli.command {
        Commands::List(args) => {
            let adapter_names = session.adapter_names().await?;
            for adapter_name in adapter_names {
                let adapter = session
                    .adapter(&adapter_name)
                    .expect("Cound not find adapter");
                println!("Devices:");
                println!("    {}:", &adapter_name);
                println!(
                    "        Address:                    {:?}",
                    adapter.address().await?
                );
                if args.all_properties {
                    list_adapters::query_adapter(&adapter).await?
                }
                println!();
            }
        }
        Commands::Enumerate(args) => {
            let adapter = session
                .adapter(&args.adapter)
                .expect("Could not find adapter");
            let addr = Address::from_str(args.address.as_str()).expect("Could not parse address");
            let device = adapter.device(addr).expect("Could not parse device");
            service::enumerate(&device).await?;
        }
        Commands::Read(args) => {
            let adapter = session
                .adapter(&args.adapter)
                .expect("Could not find adapter");
            let addr = Address::from_str(args.address.as_str()).expect("Could not parse address");
            let device = adapter.device(addr).expect("Could not parse device");
        }
    }
    Ok(())
}
