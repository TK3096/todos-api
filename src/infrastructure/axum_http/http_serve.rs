use std::net::SocketAddr;

use anyhow::{Ok, Result};
use axum::{Router, http::Method, routing::get};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use crate::infrastructure::axum_http::{default_routers, routers};

pub async fn start() -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .nest("/todos", routers::todos::routes())
        .route("/health-check", get(default_routers::health_check))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let listener = TcpListener::bind(addr).await?;

    info!("Server is running on port {}", 3001);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = terminate => {
            info!("Termination signal received, shutting down...");
        }
    }
}
