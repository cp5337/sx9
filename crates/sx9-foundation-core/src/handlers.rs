use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::info;

use crate::service_registry::ServiceRegistry;

// Health check endpoint
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "ctas7-smart-cdn-gateway",
        "timestamp": chrono::Utc::now()
    }))
}

// Get overall system status
pub async fn get_status(State(registry): State<ServiceRegistry>) -> Json<Value> {
    let services = registry.get_all_services().await;
    let healthy_services = services.iter().filter(|s| s.health_status == "healthy").count();
    
    Json(json!({
        "status": "operational",
        "total_services": services.len(),
        "healthy_services": healthy_services,
        "services": services
    }))
}

// Get all registered services
pub async fn get_services(State(registry): State<ServiceRegistry>) -> Json<Value> {
    let services = registry.get_all_services().await;
    Json(json!({
        "services": services
    }))
}

// XSD Analysis endpoint
pub async fn analyze_with_xsd(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("🔍 XSD Analysis request received");
    
    // Route to XSD service
    match registry.route_to_service("xsd", "/analyze", payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("XSD analysis failed: {}", e),
                "status": "error"
            }))
        }
    }
}

// Get XSD context
pub async fn get_xsd_context(State(registry): State<ServiceRegistry>) -> Json<Value> {
    match registry.route_to_service("xsd", "/context", json!({})).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Failed to get XSD context: {}", e),
                "status": "error"
            }))
        }
    }
}

// Assess intelligence
pub async fn assess_intelligence(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("🧠 Intelligence assessment request received");
    
    match registry.route_to_service("xsd", "/intelligence", payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Intelligence assessment failed: {}", e),
                "status": "error"
            }))
        }
    }
}

// Cannon plug registration
pub async fn register_plug(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("🔌 Cannon plug registration request received");
    
    Json(json!({
        "status": "registered",
        "plug_id": uuid::Uuid::new_v4().to_string(),
        "message": "Cannon plug registered successfully"
    }))
}

// Get all cannon plugs
pub async fn get_plugs(State(registry): State<ServiceRegistry>) -> Json<Value> {
    Json(json!({
        "plugs": [],
        "message": "No plugs currently registered"
    }))
}

// Connect cannon plug
pub async fn connect_plug(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("🔗 Cannon plug connection request received");
    
    Json(json!({
        "status": "connected",
        "message": "Cannon plug connected successfully"
    }))
}

// Process inference
pub async fn process_inference(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("🤖 Inference processing request received");
    
    match registry.route_to_service("xsd", "/inference", payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Inference processing failed: {}", e),
                "status": "error"
            }))
        }
    }
}

// Get inference patterns
pub async fn get_patterns(State(registry): State<ServiceRegistry>) -> Json<Value> {
    match registry.route_to_service("xsd", "/patterns", json!({})).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Failed to get patterns: {}", e),
                "status": "error"
            }))
        }
    }
}

// Learn new pattern
pub async fn learn_pattern(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("📚 Pattern learning request received");
    
    match registry.route_to_service("xsd", "/learn", payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Pattern learning failed: {}", e),
                "status": "error"
            }))
        }
    }
}

// Route to specific service
pub async fn route_to_service(
    State(registry): State<ServiceRegistry>,
    Path(service): Path<String>,
    Query(params): Query<HashMap<String, String>>
) -> Json<Value> {
    info!("🔄 Routing request to service: {}", service);
    
    let payload = json!(params);
    match registry.route_to_service(&service, "/", payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Failed to route to service {}: {}", service, e),
                "status": "error"
            }))
        }
    }
}

// Route to specific service with path
pub async fn route_to_service_path(
    State(registry): State<ServiceRegistry>,
    Path((service, path)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>
) -> Json<Value> {
    info!("🔄 Routing request to service: {} path: {}", service, path);
    
    let payload = json!(params);
    match registry.route_to_service(&service, &format!("/{}", path), payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Failed to route to service {}: {}", service, e),
                "status": "error"
            }))
        }
    }
}

// Collect telemetry
pub async fn collect_telemetry(
    State(registry): State<ServiceRegistry>,
    Json(payload): Json<Value>
) -> Json<Value> {
    info!("📊 Telemetry collection request received");
    
    match registry.route_to_service("telemetry", "/collect", payload).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Telemetry collection failed: {}", e),
                "status": "error"
            }))
        }
    }
}

// Get telemetry status
pub async fn get_telemetry_status(State(registry): State<ServiceRegistry>) -> Json<Value> {
    match registry.route_to_service("telemetry", "/status", json!({})).await {
        Ok(response) => Json(response),
        Err(e) => {
            Json(json!({
                "error": format!("Failed to get telemetry status: {}", e),
                "status": "error"
            }))
        }
    }
}

