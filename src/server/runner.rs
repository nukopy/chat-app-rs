//! Server execution logic.

use std::{collections::HashMap, sync::Arc};

use axum::{Router, routing::get};
use tokio::sync::Mutex;

use super::{handler::websocket_handler, signal::shutdown_signal, state::AppState};

/// Run the WebSocket chat server
///
/// # Arguments
///
/// * `host` - The host address to bind to (e.g., "127.0.0.1")
/// * `port` - The port number to bind to (e.g., 8080)
pub async fn run_server(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Create shared state for client management
    let connected_clients = Mutex::new(HashMap::new());
    let app_state = Arc::new(AppState { connected_clients });

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let bind_addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    tracing::info!(
        "WebSocket chat server listening on {}",
        listener.local_addr()?
    );
    tracing::info!("Connect to: ws://{}/ws", bind_addr);
    tracing::info!("Press Ctrl+C to shutdown gracefully");

    // Set up graceful shutdown signal handler
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");

    Ok(())
}
