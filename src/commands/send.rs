use crate::network::tcp;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info};

pub async fn handle(files: Vec<String>, target_ip: String) -> Result<()> {
    info!("Handling send: files = {:?}", files);
    debug!("File: {:?}, TargetIP: {}", files, target_ip);

    let port = 8080;
    let paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();

    tcp::send_files(&paths, &target_ip, port).await?;

    println!(
        "[ OK ] Sent {} item(s) to {} on port: {}",
        paths.len(),
        target_ip,
        port
    );
    Ok(())
}
