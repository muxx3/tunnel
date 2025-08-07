use anyhow::Result;
use std::net::UdpSocket;
use std::thread;
use tracing::info;

use crate::network::tcp;

pub async fn handle() -> Result<()> {
    // TODO: Ideally dynamically set this or receive as param
    let port = 8080;

    info!(
        "Handling recv: Listening for incoming file on port {}",
        port
    );

    // start UDP discovery responder thread
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

    tcp::receive_file("", port).await?;
    let path = tcp::receive_file("", port).await?;

    println!("[ OK ] File received and saved as '{:?}'", path);

    Ok(())
}
