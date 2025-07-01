use crate::network::tcp;
use anyhow::Result;

use tracing::{debug, info};

pub async fn handle(file: String, target_ip: String) -> Result<()> {
    info!("Handling send: file = {}", file);
    debug!("File: {}, TargetIP: {}", file, target_ip);

    // Use default port for now
    let port = 8080;

    // Call tcp::send_file with file path (String works since it implements AsRef<Path>)
    tcp::send_file(&file, &target_ip, port).await?;

    println!("(OK): {} sent to {} on port: {}", file, target_ip, port);
    Ok(())
}
