//! CTAS-7 Real Port Manager Handlers
//!
//! API handlers for the standalone port manager service.

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::port_manager::PortManager;
use crate::types::*;

/// Health check endpoint
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "ctas7-real-port-manager",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get port manager status
pub async fn get_status(State(port_manager): State<Arc<RwLock<PortManager>>>) -> Json<Value> {
    let manager = port_manager.read().await;
    Json(json!({
        "status": "active",
        "port_range": "18100-18199",
        "allocated_ports": manager.allocations.len(),
        "reserved_ports": manager.reserved_ports.len(),
        "mirror_blocks": manager.mirror_blocks.len(),
        "cyber_ops_enabled": manager.cyber_ops_enabled,
        "deception_active": manager.deception_settings.stealth_mode,
        "timestamp": chrono::Utc::now()
    }))
}

/// Get all port allocations
pub async fn get_ports(State(port_manager): State<Arc<RwLock<PortManager>>>) -> Json<Value> {
    let manager = port_manager.read().await;
    let allocations: Vec<&PortAllocation> = manager.get_all_allocations();

    Json(json!({
        "allocations": allocations,
        "total_allocated": allocations.len(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Get specific port allocation
pub async fn get_port(
    Path(port): Path<u16>,
    State(port_manager): State<Arc<RwLock<PortManager>>>,
) -> Json<Value> {
    let manager = port_manager.read().await;

    match manager.get_port_allocation(port) {
        Some(allocation) => Json(json!({
            "allocation": allocation,
            "found": true
        })),
        None => Json(json!({
            "allocation": null,
            "found": false,
            "message": format!("Port {} not allocated", port)
        })),
    }
}

/// Allocate a port
pub async fn allocate_port(
    State(port_manager): State<Arc<RwLock<PortManager>>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let mut manager = port_manager.write().await;

    let port = payload["port"].as_u64().unwrap_or(0) as u16;
    let service_name = payload["service_name"].as_str().unwrap_or("unknown");
    let service_type = match payload["service_type"].as_str().unwrap_or("custom") {
        "foundation" => ServiceType::Foundation,
        "cdn" => ServiceType::CDN,
        "xsd" => ServiceType::XSD,
        "port_manager" => ServiceType::PortManager,
        "cyber_ops" => ServiceType::CyberOps,
        "shipyard_manager" => ServiceType::ShipyardManager,
        "analytics" => ServiceType::Analytics,
        "monitoring" => ServiceType::Monitoring,
        custom => ServiceType::Custom(custom.to_string()),
    };

    match manager
        .allocate_port(port, service_name, service_type)
        .await
    {
        Ok(allocation) => Json(json!({
            "success": true,
            "allocation": allocation,
            "message": format!("Port {} allocated successfully", port)
        })),
        Err(e) => Json(json!({
            "success": false,
            "error": e.to_string(),
            "message": format!("Failed to allocate port {}", port)
        })),
    }
}

/// Release a port
pub async fn release_port(
    Path(port): Path<u16>,
    State(port_manager): State<Arc<RwLock<PortManager>>>,
) -> Json<Value> {
    let mut manager = port_manager.write().await;

    match manager.release_port(port).await {
        Ok(_) => Json(json!({
            "success": true,
            "message": format!("Port {} released successfully", port)
        })),
        Err(e) => Json(json!({
            "success": false,
            "error": e.to_string(),
            "message": format!("Failed to release port {}", port)
        })),
    }
}

/// Get mirror blocks
pub async fn get_mirror_blocks(
    State(port_manager): State<Arc<RwLock<PortManager>>>,
) -> Json<Value> {
    let manager = port_manager.read().await;
    Json(json!({
        "mirror_blocks": manager.get_mirror_blocks(),
        "total_blocks": manager.mirror_blocks.len(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Get deception settings
pub async fn get_deception_settings(
    State(port_manager): State<Arc<RwLock<PortManager>>>,
) -> Json<Value> {
    let manager = port_manager.read().await;
    Json(json!({
        "deception_settings": manager.get_deception_settings(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Get cyber operations status
pub async fn get_cyber_ops(State(port_manager): State<Arc<RwLock<PortManager>>>) -> Json<Value> {
    let manager = port_manager.read().await;
    Json(json!({
        "cyber_ops": {
            "enabled": manager.cyber_ops_enabled,
            "stealth_mode": manager.deception_settings.stealth_mode,
            "traffic_obfuscation": manager.deception_settings.traffic_obfuscation,
            "port_randomization": manager.deception_settings.port_randomization,
            "fake_ports": manager.deception_settings.fake_ports,
            "decoy_services": manager.deception_settings.decoy_services
        },
        "timestamp": chrono::Utc::now()
    }))
}
