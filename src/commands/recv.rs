use anyhow::Result;
use tracing::info;

use crate::network::tcp;

pub async fn handle() -> Result<()> {
    let save_path = "received_file"; // Default file name
    let port = 8080;

    info!(
        "Handling recv: Listening for incoming file on port {}",
        port
    );

    tcp::receive_file(save_path, port).await?;

    println!("[ OK ] File received and saved as '{}'", save_path);

    Ok(())
}
