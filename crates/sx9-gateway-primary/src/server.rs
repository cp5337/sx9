//! WebSocket server
//!
//! Axum-based WebSocket server that routes messages to handlers.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn};

use crate::handlers::handle_message;
use crate::protocol::{WsMessage, WsResponse};
use crate::state::{GatewayState, SharedState};

// QA system - dual heartbeat
use sx9_harness::gates::heartbeat_gate::HeartbeatGate;

/// Default gateway port
pub const DEFAULT_PORT: u16 = 18600;

/// Run the gateway server
pub async fn run_gateway(port: Option<u16>) -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("sx9_gateway=info".parse()?),
        )
        .init();

    // Create shared state
    let mut state = GatewayState::new();

    // Connect to backends
    info!("Connecting to SX9 backends...");
    if let Err(e) = state.connect_all().await {
        warn!("Some backends failed to connect: {}", e);
    }

    let shared_state: SharedState = Arc::new(state);

    // Build router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/health", get(health_handler))
        .route("/qa/heartbeat", get(qa_heartbeat_handler))
        .with_state(shared_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Bind and serve
    let addr = SocketAddr::from(([0, 0, 0, 0], port.unwrap_or(DEFAULT_PORT)));
    info!("SX9 Gateway listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let statuses = state.get_connection_statuses().await;
    let connected_count = statuses.iter().filter(|s| s.connected).count();

    axum::Json(serde_json::json!({
        "status": if connected_count > 0 { "ok" } else { "degraded" },
        "connected_backends": connected_count,
        "total_backends": statuses.len(),
    }))
}

/// QA Dual Heartbeat endpoint - returns full harness health report
async fn qa_heartbeat_handler() -> impl IntoResponse {
    let gate = HeartbeatGate::new();
    match gate.run().await {
        Ok(report) => axum::Json(serde_json::json!(report)),
        Err(e) => axum::Json(serde_json::json!({
            "error": e,
            "passed": false
        })),
    }
}

/// WebSocket upgrade handler
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle a WebSocket connection
async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();

    info!("New WebSocket connection");

    // Process incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse the message
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(ws_msg) => {
                        // Handle the message
                        let response = match handle_message(ws_msg, state.clone()).await {
                            Ok(resp) => resp,
                            Err(e) => WsResponse::Error {
                                code: "HANDLER_ERROR".to_string(),
                                message: e.to_string(),
                                details: None,
                            },
                        };

                        // Send response
                        if let Ok(json) = serde_json::to_string(&response) {
                            if let Err(e) = sender.send(Message::Text(json.into())).await {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        // Send parse error
                        let error = WsResponse::Error {
                            code: "PARSE_ERROR".to_string(),
                            message: format!("Invalid message format: {}", e),
                            details: Some(serde_json::json!({ "raw": text })),
                        };

                        if let Ok(json) = serde_json::to_string(&error) {
                            let _ = sender.send(Message::Text(json.into())).await;
                        }
                    }
                }
            }
            Ok(Message::Binary(_)) => {
                // Binary messages not supported yet
                warn!("Received binary message, ignoring");
            }
            Ok(Message::Ping(data)) => {
                if let Err(e) = sender.send(Message::Pong(data)).await {
                    error!("Failed to send pong: {}", e);
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                // Ignore pongs
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket connection closed by client");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    info!("WebSocket connection ended");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_endpoint() {
        let state = Arc::new(GatewayState::new());
        let statuses = state.get_connection_statuses().await;
        assert_eq!(statuses.len(), 5);
    }
}
