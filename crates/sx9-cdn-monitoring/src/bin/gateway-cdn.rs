//! CTAS Gateway CDN - Gateway to the World
//!
//! Main entry point for the CTAS Gateway CDN that serves as both
//! content delivery network and cyber warfare platform.

use axum::{
    routing::{get, post},
    Router,
};
use tracing::{error, info, warn};

use sx9_cdn_monitoring::{
    component_cdn::create_component_cdn_routes, gateway_handlers::*, intelligence_handlers::*,
    ServiceRegistry,
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get port from command line args
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "18106".to_string())
        .parse::<u16>()
        .unwrap_or(18106);

    info!("🌐 Starting CTAS-7 Gateway CDN - Gateway to the World");

    // Initialize service registry
    let mut service_registry = ServiceRegistry::new();
    if let Err(e) = service_registry.register_core_services().await {
        error!("Failed to register core services: {}", e);
    }

    // Create API routes
    let app = Router::new()
        // Gateway handlers
        .route("/health", get(health_check))
        .route("/status", get(get_status))
        .route("/services", get(get_services))
        .route("/services/{service_name}", get(get_service))
        .route("/cyber-ops", get(get_cyber_ops))
        .route("/cyber-ops/start", post(start_cyber_ops))
        .route("/nginx-config/{service_name}", get(get_nginx_config))
        // Intelligence handlers
        .route("/threat-intel", get(get_threat_intel))
        .route("/traffic-analysis", get(get_traffic_analysis))
        .route("/port-allocations", get(get_port_allocations))
        // Component CDN routes
        .nest("/cdn", create_component_cdn_routes());

    // Start server
    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    info!("🚀 CTAS-7 Gateway CDN listening on http://{}", bind_addr);
    info!("⚔️ Cyber Operations: ENABLED");
    info!("🌍 Gateway to the World: ACTIVE");
    info!("📦 Component CDN: ACTIVE");

    axum::serve(listener, app).await.unwrap();
}
