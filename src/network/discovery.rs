use anyhow::{Context, Result};
use futures::future::join_all;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn discover_peers() -> Result<Vec<String>> {
    let base_ip = "192.168.1.";
    let port = 8080;
    let mut tasks = Vec::new();

    for i in 1..=254 {
        let ip = format!("{}{}", base_ip, i);
        let addr = format!("{}:{}", ip, port);

        let fut = async move {
            let conn = timeout(Duration::from_secs(1), TcpStream::connect(&addr))
                .await
                .with_context(|| format!("Failed to connect to {}", addr));
            if let Ok(Ok(mut stream)) = conn {
                println!(">>> DEBUG: Successfully connected to {}", addr);
                // Timeout the write as well
                let write_res =
                    timeout(Duration::from_secs(1), stream.write_all(b"HELLO_TUNNEL")).await;
                if let Ok(Ok(())) = write_res {
                    Some(ip)
                } else {
                    None
                }
            } else {
                None
            }
        };

        tasks.push(fut);
    }

    let results = join_all(tasks).await;
    let peers: Vec<String> = results.into_iter().filter_map(|x| x).collect();

    println!(">>> DEBUG: discovery done, final peers list: {:?}", peers);

    Ok(peers)
}
