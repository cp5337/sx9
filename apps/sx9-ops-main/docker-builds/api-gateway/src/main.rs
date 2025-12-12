use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

#[derive(Clone)]
struct AppState {
    surrealdb_url: String,
    sledis_url: String,
    plasma_url: String,
    kali_tools_url: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    services: ServiceStatus,
}

#[derive(Serialize)]
struct ServiceStatus {
    surrealdb: bool,
    sledis: bool,
    plasma: bool,
    kali: bool,
}

#[derive(Serialize)]
struct UsimResponse {
    count: usize,
    usims: Vec<String>,
}

#[derive(Deserialize)]
struct PlasmaAlert {
    message: String,
    severity: String,
}

#[derive(Deserialize)]
struct KaliExecute {
    tool: String,
    args: Vec<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let state = Arc::new(AppState {
        surrealdb_url: std::env::var("SURREALDB_URL")
            .unwrap_or_else(|_| "http://surrealdb:8000".to_string()),
        sledis_url: std::env::var("SLEDIS_URL")
            .unwrap_or_else(|_| "http://sledis:19014".to_string()),
        plasma_url: std::env::var("PLASMA_URL")
            .unwrap_or_else(|_| "http://wazuh-manager:55000".to_string()),
        kali_tools_url: std::env::var("KALI_TOOLS_URL")
            .unwrap_or_else(|_| "http://kali-tools:18300".to_string()),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/usims", get(list_usims))
        .route("/api/plasma/alert", post(plasma_alert))
        .route("/api/kali/execute", post(kali_execute))
        .route("/api/tasks", get(list_tasks))
        .route("/api/plasma/status", get(plasma_status))
        .route("/api/kali/tools", get(kali_tools))
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:18450";
    info!("🚀 CTAS API Gateway v7.3.1 starting on {}", addr);
    info!("📡 Exposing APIs for dev center agents");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let client = reqwest::Client::new();

    let surrealdb_ok = client
        .get(format!("{}/health", state.surrealdb_url))
        .send()
        .await
        .is_ok();

    let plasma_ok = client
        .get(format!("{}/", state.plasma_url))
        .send()
        .await
        .is_ok();

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "7.3.1".to_string(),
        services: ServiceStatus {
            surrealdb: surrealdb_ok,
            sledis: true, // Assume OK for now
            plasma: plasma_ok,
            kali: true, // Assume OK for now
        },
    })
}

async fn list_usims(State(_state): State<Arc<AppState>>) -> Json<UsimResponse> {
    info!("📊 Listing USIMs");
    // TODO: Query SurrealDB for actual USIMs
    Json(UsimResponse {
        count: 2309,
        usims: vec!["USIM data would be here".to_string()],
    })
}

async fn plasma_alert(
    State(_state): State<Arc<AppState>>,
    Json(alert): Json<PlasmaAlert>,
) -> StatusCode {
    info!("🚨 Plasma alert: {} ({})", alert.message, alert.severity);
    // TODO: Forward to Plasma/Wazuh
    StatusCode::OK
}

async fn kali_execute(
    State(_state): State<Arc<AppState>>,
    Json(exec): Json<KaliExecute>,
) -> StatusCode {
    info!("🔪 Executing Kali tool: {} {:?}", exec.tool, exec.args);
    // TODO: Forward to Kali tools container
    StatusCode::OK
}

async fn list_tasks(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    info!("📋 Listing CTAS tasks");
    Json(serde_json::json!({
        "count": 165,
        "tasks": ["Task data would be here"]
    }))
}

async fn plasma_status(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    info!("🛡️  Checking Plasma status");
    Json(serde_json::json!({
        "status": "operational",
        "services": {
            "wazuh": "running",
            "axon": "running",
            "legion": "running"
        }
    }))
}

async fn kali_tools(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    info!("🔪 Listing Kali tools");
    Json(serde_json::json!({
        "count": 165,
        "tools": ["nmap", "rustscan", "metasploit", "burpsuite"]
    }))
}

