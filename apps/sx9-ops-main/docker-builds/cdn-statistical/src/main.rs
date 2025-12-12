//! Statistical Analysis CDN v7.3.1
//! Port: 18108
//! Integrates with CTAS-7 Port Manager

use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    service: String,
    port: u16,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PortManagerRegistration {
    service_name: String,
    port: u16,
    protocol: String,
    health_endpoint: String,
    metrics_endpoint: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics))
        .layer(CorsLayer::permissive());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 18108));
    info!("🚀 Statistical Analysis CDN starting on {}", addr);
    
    // Register with Port Manager
    register_with_port_manager().await;
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "cdn-statistical".to_string(),
        port: 18108,
        version: "7.3.1".to_string(),
    })
}

async fn metrics() -> String {
    "# HELP cdn-statistical_requests_total Total requests\n".to_string()
}

async fn register_with_port_manager() {
    let client = reqwest::Client::new();
    let registration = PortManagerRegistration {
        service_name: "cdn-statistical".to_string(),
        port: 18108,
        protocol: "http".to_string(),
        health_endpoint: "/health".to_string(),
        metrics_endpoint: "/metrics".to_string(),
    };
    
    match client
        .post("http://localhost:18103/api/ports/register")
        .json(&registration)
        .send()
        .await
    {
        Ok(_) => info!("✅ Registered with Port Manager"),
        Err(e) => info!("⚠️  Port Manager registration failed: {}", e),
    }
}
