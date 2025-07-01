use anyhow::Result;
use tracing::info;

use crate::network::discovery::discover_peers;

/// Handle "dig" command to discover peers.
pub async fn handle() -> Result<()> {
    info!("Handlig Dig: Starting peer discovery...");

    let peers = discover_peers().await?;

    if peers.is_empty() {
        println!("No peers found on the local network.");
    } else {
        println!("Discovered peers:");
        for ip in peers {
            println!(" - {}", ip);
        }
    }

    Ok(())
}
