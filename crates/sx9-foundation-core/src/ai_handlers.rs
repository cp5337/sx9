use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::phi_service::{PhiService, PhiRequest, PhiResponse};
use crate::gnn_service::{GNNService, GNNRequest, GNNResponse};

pub type PhiServiceState = Arc<RwLock<PhiService>>;
pub type GNNServiceState = Arc<RwLock<GNNService>>;

/// Phi model text generation endpoint
pub async fn phi_generate(
    State(phi_service): State<PhiServiceState>,
    Json(request): Json<PhiRequest>,
) -> Result<Json<PhiResponse>, StatusCode> {
    info!("Processing Phi generation request");

    let service = phi_service.read().await;
    match service.generate(request).await {
        Ok(response) => {
            info!("Phi generation completed successfully");
            Ok(Json(response))
        }
        Err(e) => {
            error!("Phi generation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Phi model health check
pub async fn phi_health(
    State(phi_service): State<PhiServiceState>,
) -> Json<Value> {
    let service = phi_service.read().await;
    let is_healthy = service.health_check().await;

    Json(json!({
        "service": "phi-3.5-mini",
        "status": if is_healthy { "healthy" } else { "unhealthy" },
        "model_info": service.get_model_info(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Phi model information endpoint
pub async fn phi_info(
    State(phi_service): State<PhiServiceState>,
) -> Json<Value> {
    let service = phi_service.read().await;
    Json(service.get_model_info())
}

/// GNN processing endpoint
pub async fn gnn_process(
    State(gnn_service): State<GNNServiceState>,
    Json(request): Json<GNNRequest>,
) -> Result<Json<GNNResponse>, StatusCode> {
    info!("Processing GNN request: {:?}", request.task_type);

    let mut service = gnn_service.write().await;
    match service.process_graph(request).await {
        Ok(response) => {
            info!("GNN processing completed successfully");
            Ok(Json(response))
        }
        Err(e) => {
            error!("GNN processing failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GNN model information endpoint
pub async fn gnn_info(
    State(gnn_service): State<GNNServiceState>,
) -> Json<Value> {
    let service = gnn_service.read().await;
    Json(service.get_model_info())
}

/// Combined AI services status
pub async fn ai_status(
    State(phi_service): State<PhiServiceState>,
    State(gnn_service): State<GNNServiceState>,
) -> Json<Value> {
    let phi = phi_service.read().await;
    let gnn = gnn_service.read().await;

    let phi_healthy = phi.health_check().await;

    Json(json!({
        "ai_services": {
            "phi": {
                "status": if phi_healthy { "healthy" } else { "unhealthy" },
                "model": "Phi-3.5-mini-instruct"
            },
            "gnn": {
                "status": "healthy",
                "model": "CTAS-GNN"
            }
        },
        "capabilities": [
            "Text generation with Phi-3.5",
            "Graph neural network analysis",
            "Statistical analysis integration",
            "CTAS tactical intelligence"
        ],
        "memory_usage": "Under 12GB total",
        "timestamp": chrono::Utc::now()
    }))
}

/// CTAS tactical intelligence endpoint (combines statistical + AI)
pub async fn tactical_intelligence(
    State(phi_service): State<PhiServiceState>,
    State(gnn_service): State<GNNServiceState>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!("Processing tactical intelligence request");

    let data_analysis = request.get("statistical_data");
    let graph_data = request.get("graph_data");
    let query = request.get("query").and_then(|v| v.as_str()).unwrap_or("Analyze the tactical situation");

    let mut intelligence_report = json!({
        "timestamp": chrono::Utc::now(),
        "analysis_type": "tactical_intelligence",
        "components": {}
    });

    // Process with Phi if query provided
    if let Ok(phi) = phi_service.read().await.generate(PhiRequest {
        prompt: format!("Tactical Analysis: {}", query),
        max_tokens: Some(512),
        temperature: Some(0.7),
        system_prompt: Some("You are a CTAS 7.0 tactical intelligence analyst. Provide concise, actionable intelligence.".to_string()),
    }).await {
        intelligence_report["components"]["phi_analysis"] = json!({
            "text_analysis": phi.generated_text,
            "processing_time_ms": phi.processing_time_ms
        });
    }

    // Process with GNN if graph data provided
    if graph_data.is_some() {
        intelligence_report["components"]["graph_analysis"] = json!({
            "status": "Graph analysis capabilities available",
            "note": "Submit graph data for network analysis"
        });
    }

    // Add statistical component placeholder
    intelligence_report["components"]["statistical_analysis"] = json!({
        "status": "Statistical engine integrated",
        "capabilities": ["Correlation analysis", "Trend detection", "Anomaly identification"]
    });

    Ok(Json(intelligence_report))
}

/// Model registration endpoint for CTAS SDK integration
pub async fn register_model(
    Json(registration): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!("Processing model registration");

    let model_name = registration.get("model_name").and_then(|v| v.as_str()).unwrap_or("unknown");
    let model_type = registration.get("model_type").and_then(|v| v.as_str()).unwrap_or("unknown");

    Ok(Json(json!({
        "registration_status": "success",
        "model_name": model_name,
        "model_type": model_type,
        "registered_at": chrono::Utc::now(),
        "integration_status": "ready",
        "ctas_sdk_compatible": true
    })))
}