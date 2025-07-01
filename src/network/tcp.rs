use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Sends a file to the given target IP and port using TCP.
///
/// # Arguments
///
/// * `file_path` - File path to send (accepts &str, String, Path, PathBuf).
/// * `target_ip` - Target IP address.
/// * `port` - Target port.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or the connection fails.

pub async fn send_file<P: AsRef<Path>>(file_path: P, target_ip: &str, port: u16) -> Result<()> {
    // Check file
    let mut file = File::open(file_path.as_ref())
        .await
        .with_context(|| "Failed to open file")?;

    // Conect to receiver
    let addr = format!("{}:{}", target_ip, port);
    let mut stream = TcpStream::connect(&addr)
        .await
        .with_context(|| format!("Failed to connect to {}", addr))?;

    // Send file contents in chunks
    let mut buffer = [0u8; 4096];
    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n]).await?;
    }
    Ok(())
}
