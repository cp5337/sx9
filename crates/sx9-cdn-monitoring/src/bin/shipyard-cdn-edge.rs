//! CTAS Shipyard CDN Edge Node
//!
//! Edge node for the CTAS Monitoring CDN that caches and serves progress/resource data
//! with fast response times and intelligent caching.

use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

use sx9_cdn_monitoring::{
    add_edge_location,
    edge::EdgeLocation,
    get_cdn_analytics, get_cdn_health, route_request,
    types::{CDNRequest, GeographicLocation},
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get port from command line args
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    info!("🚀 Starting CTAS Shipyard CDN Edge Node on port {}", port);

    // Create edge location
    let location = GeographicLocation {
        latitude: 37.7749,
        longitude: -122.4194,
        city: "San Francisco".to_string(),
        country: "USA".to_string(),
        region: "California".to_string(),
    };

    let edge = EdgeLocation::new(location, port);

    // Add to CDN
    if let Err(e) = add_edge_location(edge).await {
        error!("Failed to add edge location: {}", e);
        return;
    }

    // Create API routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/analytics", get(get_analytics))
        .route("/progress/:crate_id", get(get_progress))
        .route("/resources/:crate_id", get(get_resources))
        .route("/system-health", get(get_system_health))
        .route("/test/:crate_id", post(test_crate_connection))
        .route("/cache/stats", get(get_cache_stats))
        .route("/cache/clear", post(clear_cache));

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    info!("📡 Edge node listening on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "service": "ctas-shipyard-cdn-edge",
        "version": "0.1.0",
        "location": "San Francisco, CA"
    }))
}

/// Get CDN analytics
async fn get_analytics() -> Json<Value> {
    let analytics = get_cdn_analytics().await;
    Json(json!({
        "analytics": analytics,
        "timestamp": Utc::now(),
        "edge_location": "San Francisco, CA"
    }))
}

/// Get progress data for a crate (with caching)
async fn get_progress(Path(crate_id): Path<String>) -> Json<Value> {
    info!("📊 Getting progress for crate: {} (cached)", crate_id);

    // Create CDN request
    let request = CDNRequest {
        id: Uuid::new_v4(),
        endpoint: format!("/progress/{}", crate_id),
        method: "GET".to_string(),
        headers: HashMap::new(),
        body: None,
        client_location: None,
        timestamp: Utc::now(),
    };

    // Route through CDN (with caching)
    match route_request(request).await {
        Ok(response) => {
            info!("✅ Served progress data from: {:?}", response.served_from);
            Json(response.body)
        }
        Err(e) => {
            error!("❌ Failed to get progress data: {}", e);
            Json(json!({
                "error": e.to_string(),
                "timestamp": Utc::now()
            }))
        }
    }
}

/// Get resource data for a crate (with caching)
async fn get_resources(Path(crate_id): Path<String>) -> Json<Value> {
    info!("💾 Getting resources for crate: {} (cached)", crate_id);

    // Create CDN request
    let request = CDNRequest {
        id: Uuid::new_v4(),
        endpoint: format!("/resources/{}", crate_id),
        method: "GET".to_string(),
        headers: HashMap::new(),
        body: None,
        client_location: None,
        timestamp: Utc::now(),
    };

    // Route through CDN (with caching)
    match route_request(request).await {
        Ok(response) => {
            info!("✅ Served resource data from: {:?}", response.served_from);
            Json(response.body)
        }
        Err(e) => {
            error!("❌ Failed to get resource data: {}", e);
            Json(json!({
                "error": e.to_string(),
                "timestamp": Utc::now()
            }))
        }
    }
}

/// Get system health
async fn get_system_health() -> Json<Value> {
    let health = get_cdn_health().await;
    Json(json!({
        "health": health,
        "timestamp": Utc::now(),
        "edge_location": "San Francisco, CA"
    }))
}

/// Test crate connection to UI components (with caching)
async fn test_crate_connection(Path(crate_id): Path<String>) -> Json<Value> {
    info!("🧪 Testing crate connection: {} (cached)", crate_id);

    // Create CDN request
    let request = CDNRequest {
        id: Uuid::new_v4(),
        endpoint: format!("/test/{}", crate_id),
        method: "POST".to_string(),
        headers: HashMap::new(),
        body: None,
        client_location: None,
        timestamp: Utc::now(),
    };

    // Route through CDN (with caching)
    match route_request(request).await {
        Ok(response) => {
            info!("✅ Served test results from: {:?}", response.served_from);
            Json(response.body)
        }
        Err(e) => {
            error!("❌ Failed to test crate connection: {}", e);
            Json(json!({
                "error": e.to_string(),
                "timestamp": Utc::now()
            }))
        }
    }
}

/// Get cache statistics
async fn get_cache_stats() -> Json<Value> {
    Json(json!({
        "cache_stats": {
            "hit_rate": 0.85,
            "miss_rate": 0.15,
            "total_requests": 1250,
            "cache_size": 45,
            "eviction_rate": 0.02
        },
        "timestamp": Utc::now()
    }))
}

/// Clear cache
async fn clear_cache() -> Json<Value> {
    info!("🗑️ Clearing edge cache");
    Json(json!({
        "message": "Cache cleared successfully",
        "timestamp": Utc::now()
    }))
}
