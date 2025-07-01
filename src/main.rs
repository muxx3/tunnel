// src/main.rs
mod commands;
mod errors;
mod network;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tunnel")]
#[command(about = "CLI P2P file sharing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file to a peer
    Send { file: String, target_ip: String },

    /// Receive an incoming file
    Recv {},

    /// Search for peers on the network
    Dig {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send { file, target_ip } => commands::send::handle(file, target_ip).await,
        Commands::Recv {} => commands::recv::handle().await,
        Commands::Dig {} => commands::dig::handle().await,
    }

    Ok(())
}
