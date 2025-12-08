//! CTAS-7 Real Port Manager
//! 
//! Standalone port manager with major port blocks, mirror blocks, and deception settings.
//! Extracted from the CDN to be a dedicated service.

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Query},
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod port_manager;
mod types;
mod handlers;

use port_manager::PortManager;
use types::*;
use handlers::*;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting CTAS-7 Real Port Manager");
    info!("📡 Port Range: 18100-18199 (Major Port Blocks)");
    info!("🪞 Mirror Blocks: ENABLED");
    info!("🥷 Deception Settings: ACTIVE");
    
    // Initialize port manager
    let port_manager = Arc::new(RwLock::new(PortManager::new()));
    
    // Create API routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/status", get(get_status))
        .route("/ports", get(get_ports))
        .route("/ports/allocate", post(allocate_port))
        .route("/ports/:port", get(get_port))
        .route("/ports/:port/release", post(release_port))
        .route("/mirror-blocks", get(get_mirror_blocks))
        .route("/deception-settings", get(get_deception_settings))
        .route("/cyber-ops", get(get_cyber_ops))
        .with_state(port_manager);

    // Start server on real CTAS-7 port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:18103").await.unwrap();
    info!("🎯 Real Port Manager listening on http://0.0.0.0:18103");
    info!("⚔️ Cyber Operations: ENABLED");
    info!("🪞 Mirror Blocks: ACTIVE");
    info!("🥷 Deception Mode: STEALTH");
    
    axum::serve(listener, app).await.unwrap();
}

