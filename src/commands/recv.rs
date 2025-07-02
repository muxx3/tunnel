use anyhow::Result;
use std::net::UdpSocket;
use std::thread;
use tracing::info;

use crate::network::tcp;

pub async fn handle() -> Result<()> {
    let save_path = "received_file"; // Default file name
    let port = 8080;

    info!(
        "Handling recv: Listening for incoming file on port {}",
        port
    );

    // Start UDP discovery reply thread
    thread::spawn(|| {
        let udp_socket = UdpSocket::bind("0.0.0.0:8888").expect("Failed to bind UDP socket");
        let mut buf = [0u8; 1024];
        loop {
            if let Ok((size, src)) = udp_socket.recv_from(&mut buf) {
                let msg = String::from_utf8_lossy(&buf[..size]);
                if msg == "DISCOVER" {
                    let _ = udp_socket.send_to(b"TUNNEL_HERE", src);
                }
            }
        }
    });

    tcp::receive_file(save_path, port).await?;

    println!("[ OK ] File received and saved as '{}'", save_path);

    Ok(())
}
