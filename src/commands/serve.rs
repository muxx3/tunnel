use anyhow::Result;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use local_ip_address::local_ip;
use qrcode::render::unicode;
use qrcode::QrCode;
use std::{net::IpAddr, path::PathBuf, sync::Arc};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    file_path: Arc<PathBuf>,
    file_name: Arc<String>,
}

pub async fn handle(file_path: String) -> Result<()> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        println!("File does not exist: {}", file_path);
        return Ok(());
    }

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("donwload")
        .to_string();

    let ip_addr: IpAddr = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let local_ip = ip_addr.to_string();

    let state = AppState {
        file_path: Arc::new(path),
        file_name: Arc::new(file_name),
    };

    let app = Router::new()
        .route("/download", get(download_handler))
        .with_state(state);

    let addr = "0.0.0.0:8080";
    let server_url = format!("http://{}:8080/download", local_ip);

    let code = QrCode::new(&server_url).expect("Failed to generate QR code");
    let qr = code.render::<unicode::Dense1x2>().build();
    println!("\nScan this QR code on your phone to download the file:");
    println!("{}", qr);
    println!("Or open this URL manually: {}", server_url);

    println!("[ OK ] HTTP server started on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn download_handler(State(state): State<AppState>) -> Response {
    let mut file = match File::open(&*state.file_path).await {
        Ok(f) => f,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let mut contents = Vec::new();
    if file.read_to_end(&mut contents).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );

    // This header tells the browser the filename for saving the file
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", state.file_name)
            .parse()
            .unwrap(),
    );

    (headers, contents).into_response()
}
