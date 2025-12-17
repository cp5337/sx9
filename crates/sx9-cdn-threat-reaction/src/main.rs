//! CTAS-7 Threat Reaction CDN - Main Entry Point
//!
//! HTTP server for threat reaction registration, execution, and simulation

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use uuid::Uuid;

use sx9_cdn_threat_reaction::{
    ReactionSession, RecognizedThreat, SimulationResult, ThreatReactionCDN,
};

#[derive(Clone)]
struct AppState {
    cdn: Arc<ThreatReactionCDN>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting CTAS-7 Threat Reaction CDN");

    // Initialize CDN
    let cdn = Arc::new(ThreatReactionCDN::new(
        "http://localhost:18103".to_string(), // Port Manager
        "http://localhost:18650".to_string(), // Foundation Daemon
        "http://localhost:55000".to_string(), // Plasma (Wazuh)
    ));

    let app_state = AppState { cdn };

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/reactions/register", post(register_reaction))
        .route("/api/reactions/:id", get(get_reaction_status))
        .route("/api/reactions/:id/execute", post(execute_reaction))
        .route("/api/simulations/run", post(run_simulation))
        .route("/api/simulations/:id", get(get_simulation_result))
        .with_state(app_state);

    // Start server
    let listener = TcpListener::bind("0.0.0.0:18111").await?;
    info!("✅ Threat Reaction CDN listening on port 18111");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "threat-reaction-cdn",
        "version": "7.3.1"
    }))
}

async fn register_reaction(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ReactionSession>, StatusCode> {
    // TODO: Deserialize FormulatedResponse from payload
    // For now, create a mock response
    let response = sx9_cdn_threat_reaction::FormulatedResponse {
        playbook: serde_json::json!({}),
        escalation_plan: serde_json::json!({}),
        hd4_phase: "Detect".to_string(),
        dual_trivariate_hash: serde_json::json!({}),
        patterns: serde_json::json!({}),
        interdiction_points: vec![],
    };

    match state.cdn.register_reaction(&response).await {
        Ok(session) => Ok(Json(session)),
        Err(e) => {
            error!("Failed to register reaction: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_reaction_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReactionSession>, StatusCode> {
    let session_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    match state.cdn.get_reaction_status(session_id).await {
        Ok(Some(session)) => Ok(Json(session)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get reaction status: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn execute_reaction(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let session_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    match state.cdn.execute_reaction(session_id).await {
        Ok(result) => Ok(Json(serde_json::to_value(result).unwrap())),
        Err(e) => {
            error!("Failed to execute reaction: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn run_simulation(
    State(state): State<AppState>,
    Json(threat): Json<RecognizedThreat>,
) -> Result<Json<SimulationResult>, StatusCode> {
    match state.cdn.simulate_reaction(&threat).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Failed to run simulation: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_simulation_result(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Implement simulation result retrieval
    Err(StatusCode::NOT_IMPLEMENTED)
}
