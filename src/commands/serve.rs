use crate::network::zip_helper::create_zip_from_paths;
use axum::{routing::get_service, Router};
use local_ip_address::local_ip;
use qrcode::{render::unicode, QrCode};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use url::Url;

pub async fn handle(inputs: Vec<String>) -> anyhow::Result<()> {
    if inputs.is_empty() {
        anyhow::bail!("Please provide one or more files or directories to serve");
    }

    let paths: Vec<PathBuf> = inputs.iter().map(PathBuf::from).collect();

    let serve_path = if paths.len() == 1 && paths[0].is_file() {
        paths[0].clone()
    } else {
        let tmp_dir = std::env::temp_dir();
        let zip_path = tmp_dir.join("tunnel.zip");
        create_zip_from_paths(&paths, &zip_path)?;
        zip_path
    };

    let dir = serve_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let filename = serve_path.file_name().unwrap().to_string_lossy();

    let app = Router::new().fallback_service(get_service(ServeDir::new(&dir)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    let ip = local_ip()?;
    let url = Url::parse(&format!("http://{}:8080/{}", ip, filename))?;

    let qr = QrCode::new(url.as_str())?;
    let qr_text = qr.render::<unicode::Dense1x2>().build();

    println!("Scan this QR to download:\n{}", url);
    println!("{}", qr_text);

    println!("Serving {} on http://{}", filename, addr);
    axum::serve(listener, app).await?;

    Ok(())
}
