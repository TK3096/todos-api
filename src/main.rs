use todos_api::infrastructure::axum_http::http_serve::start;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting the server...");

    start().await.expect("Failed to start the server");
}
