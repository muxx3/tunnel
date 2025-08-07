use anyhow::Result;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

pub async fn discover_peers() -> Result<Vec<String>> {
    let broadcast_addr = "192.168.1.255:8888";
    let listen_addr = "0.0.0.0:0";

    let socket = UdpSocket::bind(listen_addr)?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;

    // Send DISCOVER message
    socket.send_to(b"DISCOVER", broadcast_addr)?;

    let start = Instant::now();
    let timeout = Duration::from_secs(3);

    let mut peers = Vec::new();

    while start.elapsed() < timeout {
        let mut buf = [0u8; 1024];
        if let Ok((size, src)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..size]);
            if msg == "TUNNEL_HERE" && !peers.contains(&src.ip().to_string()) {
                {
                    peers.push(src.ip().to_string());
                }
            }
        }
    }

    Ok(peers)
}
