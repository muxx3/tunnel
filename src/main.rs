// src/main.rs
mod commands;
mod errors;
mod network;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
#[allow(unused_imports)]
use tracing::{debug, error, info, warn};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "tunnel")]
#[command(about = "CLI P2P file sharing", long_about = None)]
struct Cli {
    #[arg(long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file to a peer
    Send {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        target_ip: String,
    },
    /// Receive an incoming file
    Recv {},
    /// Search for peers on the network
    Dig {},
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber (logs to stdout)
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Send { file, target_ip } => {
            info!("Send called with file: {} to IP: {}", file, target_ip);
            commands::send::handle(file, target_ip).await
        }
        Commands::Recv {} => {
            info!("Recv called");
            commands::recv::handle().await
        }
        Commands::Dig {} => {
            info!("Dig called");
            commands::dig::handle().await
        }
    }

    Ok(())
}
