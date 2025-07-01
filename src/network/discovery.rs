use anyhow::{Context, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::net::TcpStream;

pub async fn discover_peers() -> Result<Vec<String>> {
    let base_ip = "192.168.1.";
    let port = 8080;
    let mut tasks = FuturesUnordered::new();

    for i in 1..=254 {
        let ip = format!("{}{}", base_ip, i);
        let addr = format!("{}:{}", ip, port);

        tasks.push(async move {
            match TcpStream::connect(&addr)
                .await
                .with_context(|| format!("Failed to connect to {}", addr))
            {
                Ok(_) => Some(ip),
                Err(_) => None,
            }
        });
    }

    let mut peers = Vec::new();

    while let Some(result) = tasks.next().await {
        if let Some(ip) = result {
            peers.push(ip);
        }
    }

    Ok(peers)
}
