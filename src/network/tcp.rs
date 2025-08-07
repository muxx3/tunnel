use crate::network::zip_helper::create_zip_from_paths;
use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn send_file<P: AsRef<Path>>(file_path: P, target_ip: &str, port: u16) -> Result<()> {
    let file_path = file_path.as_ref();
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;

    let display_name = format!("tunnel-{}", file_name);

    let mut file = File::open(file_path)
        .await
        .with_context(|| "Failed to open file")?;

    let addr = format!("{}:{}", target_ip, port);
    let mut stream = TcpStream::connect(&addr)
        .await
        .with_context(|| format!("Failed to connect to {}", addr))?;

    let name_bytes = display_name.as_bytes();
    let name_len = name_bytes.len() as u16;
    stream.write_u16(name_len).await?;
    stream.write_all(name_bytes).await?;

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

pub async fn send_files<P: AsRef<Path>>(paths: &[P], target_ip: &str, port: u16) -> Result<()> {
    let zip_path = std::env::temp_dir().join("tunnel.zip");

    if paths.len() > 1 || paths[0].as_ref().is_dir() {
        create_zip_from_paths(paths, &zip_path).with_context(|| "Failed to create zip archive")?;

        send_file(zip_path, target_ip, port).await?;
    } else {
        send_file(&paths[0], target_ip, port).await?;
    }

    Ok(())
}

pub async fn receive_file<P: AsRef<Path>>(_save_path: P, port: u16) -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .with_context(|| format!("Failed to bind to port {}", port))?;

    tracing::info!("Listening for incoming files on port {}", port);

    loop {
        let (mut socket, addr) = listener
            .accept()
            .await
            .with_context(|| "Failed to accept connection")?;
        tracing::info!("Connection accepted from {}", addr);

        // skip probes
        let mut probe_buf = [0u8; 12];
        let n = socket.peek(&mut probe_buf).await?;
        if n >= 12 && &probe_buf[..12] == b"HELLO_TUNNEL" {
            tracing::info!("Received discovery probe from {}", addr);
            continue;
        }

        // read filename length + filename
        let name_len = socket.read_u16().await?;
        let mut name_buf = vec![0u8; name_len as usize];
        socket.read_exact(&mut name_buf).await?;
        let filename = String::from_utf8(name_buf)?;

        // save as the received filename
        let mut file = File::create(&filename)
            .await
            .with_context(|| format!("Failed to create file '{}'", filename))?;

        let mut buffer = [0u8; 4096];
        loop {
            let n = socket.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n]).await?;
        }

        tracing::info!("File saved as '{}'", filename);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_send_files() {
        let dummy_paths = [PathBuf::from("dummy.txt")];
        let _ = send_files(&dummy_paths, "127.0.0.1", 12345).await;
    }
}
