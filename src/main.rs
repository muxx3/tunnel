mod commands;
mod errors;
mod network;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
#[allow(unused_imports)]
use tracing::{debug, error, info, warn};

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
    // send a file to a peer
    Send {
        #[arg(short, long, num_args = 1..)]
        file: Vec<String>,
        #[arg(short, long)]
        target_ip: String,
    },
    // receive an incoming file
    Recv {},
    // search for peers on the network
    Dig {},
    // serve a file over HTTP and show QR
    Serve {
        #[arg(short, long, num_args = 1..)]
        file: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    println!(">>> DEBUG: Entering match on cli.command");
    match cli.command {
        Commands::Send { file, target_ip } => {
            info!("Send called with file(s): {:?} to IP: {}", file, target_ip);
            commands::send::handle(file, target_ip).await?
        }
        Commands::Recv {} => {
            info!("Recv called");
            commands::recv::handle().await?
        }
        Commands::Dig {} => {
            info!("Dig called");
            println!(">>> DEBUG: Inside Dig arm before handle()");
            commands::dig::handle().await?
        }
        Commands::Serve { file } => {
            info!("Serve called with file: {:?}", file);
            commands::serve::handle(file).await?
        }
    }

    Ok(())
}
