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
        "service": "sx9-port-manager",
        "version": "0.2.0",
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

/// Smart service registration with auto-bump on conflicts
pub async fn register_service(
    State(port_manager): State<Arc<RwLock<PortManager>>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let mut manager = port_manager.write().await;

    let service_name = payload["service_name"].as_str().unwrap_or("unknown");
    let service_type_str = payload["service_type"].as_str().unwrap_or("custom");
    let preferred_port = payload["preferred_port"].as_u64().unwrap_or(0) as u16;
    let request_mirror = payload["request_mirror"].as_bool().unwrap_or(false);

    // Parse service type
    let service_type = match service_type_str {
        "foundation" => ServiceType::Foundation,
        "gateway" => ServiceType::Gateway,
        "cdn" => ServiceType::CDN,
        "xsd" => ServiceType::XSD,
        "port_manager" => ServiceType::PortManager,
        "cyber_ops" => ServiceType::CyberOps,
        "shipyard_manager" => ServiceType::ShipyardManager,
        "analytics" => ServiceType::Analytics,
        "monitoring" => ServiceType::Monitoring,
        "orbital" => ServiceType::Orbital,
        custom => ServiceType::Custom(custom.to_string()),
    };

    // Smart allocation: try preferred port, auto-bump on conflict
    let mut port_to_try = preferred_port;
    let max_attempts = 10;
    let mut attempts = 0;
    let mut allocated_port = None;

    while attempts < max_attempts {
        // Check if port is in valid range
        if port_to_try < 18100 || port_to_try > 18199 {
            port_to_try = 18100; // Reset to start of range
            attempts += 1;
            continue;
        }

        // Check if port is reserved or allocated
        if manager.reserved_ports.contains(&port_to_try)
            || manager.allocations.contains_key(&port_to_try)
        {
            port_to_try += 1; // Auto-bump
            attempts += 1;
            continue;
        }

        // Try to allocate
        match manager
            .allocate_port(port_to_try, service_name, service_type.clone())
            .await
        {
            Ok(allocation) => {
                allocated_port = Some(allocation);
                break;
            }
            Err(_) => {
                port_to_try += 1; // Auto-bump on error
                attempts += 1;
            }
        }
    }

    match allocated_port {
        Some(allocation) => {
            let mut response = json!({
                "success": true,
                "service_name": service_name,
                "allocated_port": allocation.port,
                "preferred_port": preferred_port,
                "auto_bumped": allocation.port != preferred_port,
                "mirror_ports": allocation.mirror_ports,
                "allocation_id": allocation.allocation_id,
                "timestamp": chrono::Utc::now()
            });

            // Add mirror block info if requested
            if request_mirror && !allocation.mirror_ports.is_empty() {
                let mirror_start = allocation.port + 10000; // 18XXX -> 28XXX
                let mirror_end = mirror_start + (allocation.mirror_ports.len() as u16) - 1;
                response["mirror_block"] = json!({
                    "primary": allocation.port,
                    "mirrors": allocation.mirror_ports,
                    "mirror_range": format!("{}-{}", mirror_start, mirror_end)
                });
            }

            Json(response)
        }
        None => Json(json!({
            "success": false,
            "error": "No ports available",
            "message": format!("Failed to allocate port for {} after {} attempts", service_name, max_attempts),
            "timestamp": chrono::Utc::now()
        })),
    }
}

/// Discover all registered services
pub async fn discover_services(
    State(port_manager): State<Arc<RwLock<PortManager>>>,
) -> Json<Value> {
    let manager = port_manager.read().await;
    let allocations: Vec<&PortAllocation> = manager.get_all_allocations();

    let services: Vec<Value> = allocations
        .iter()
        .map(|alloc| {
            json!({
                "service_name": alloc.service_name,
                "port": alloc.port,
                "service_type": format!("{:?}", alloc.service_type),
                "mirror_ports": alloc.mirror_ports,
                "allocated_at": alloc.allocated_at,
                "allocation_id": alloc.allocation_id
            })
        })
        .collect();

    Json(json!({
        "services": services,
        "total_services": services.len(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Get port blocks (alias for mirror-blocks with additional info)
pub async fn get_blocks(State(port_manager): State<Arc<RwLock<PortManager>>>) -> Json<Value> {
    let manager = port_manager.read().await;

    let blocks = vec![
        json!({
            "name": "Core Services",
            "range": "18100-18119",
            "description": "Foundation, Port Manager, Core Infrastructure",
            "reserved_ports": manager.reserved_ports
        }),
        json!({
            "name": "Gateway Block",
            "range": "18120-18139",
            "primary": 18120,
            "mirrors": [18121, 18122],
            "mirror_range": "28120-28139",
            "description": "API Gateway, Load Balancing, Failover"
        }),
        json!({
            "name": "CDN Block",
            "range": "18140-18159",
            "primary": 18140,
            "mirrors": [18141, 18142],
            "mirror_range": "28140-28159",
            "description": "Content Delivery, Static Assets"
        }),
        json!({
            "name": "Deception Block",
            "range": "18150-18159",
            "primary": 18150,
            "mirrors": [18151, 18152],
            "mirror_range": "28150-28159",
            "description": "Honeypots, Decoy Services, Threat Intelligence"
        }),
        json!({
            "name": "Neural/Memory Block",
            "range": "18160-18179",
            "primary": 18160,
            "mirrors": [18161, 18162],
            "mirror_range": "28160-28179",
            "description": "Neural Mesh, Memory Services, ANN Layer"
        }),
        json!({
            "name": "Analytics Block",
            "range": "18180-18189",
            "description": "Metrics, Logging, Observability"
        }),
        json!({
            "name": "Monitoring Block",
            "range": "18190-18199",
            "description": "Health Checks, Status, Alerting"
        }),
    ];

    Json(json!({
        "blocks": blocks,
        "total_blocks": blocks.len(),
        "mirror_blocks": manager.get_mirror_blocks(),
        "timestamp": chrono::Utc::now()
    }))
}
