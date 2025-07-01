use crate::network::discovery;
use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt, BufReader};

/// Handle "dig" command to discover peers.
pub async fn handle() -> Result<()> {
    println!("Starting peer discovery...");

    let peers = discovery::discover_peers().await?;
    println!(">>> DEBUG: peers vector length = {}", peers.len());

    if peers.is_empty() {
        println!("No peers found on local network.");
    } else {
        println!("Discovered peers:");
        for peer in peers {
            println!(" - {}", peer);
        }
    }

    println!("\nPress 'q' then Enter to quit.");

    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().eq_ignore_ascii_case("q") {
            println!("Quitting...");
            break;
        } else {
            println!("Press 'q' then Enter to quit.");
        }
    }

    Ok(())
}
