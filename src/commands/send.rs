pub async fn handle(file: String, target_ip: String) {
    tracing::debug!("Preparing to send file: {}", file);
    tracing::info!("Handling send: file = {}, target = {}", file, target_ip);
    println!("(Stub) Sending {} to {}", file, target_ip);
}
