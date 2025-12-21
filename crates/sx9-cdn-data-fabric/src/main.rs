//! GLAF CDN Data Fabric
//!
//! Universal database aggregation layer for visualization.
//! All databases register here and report their schemas.
//! Queries are routed to appropriate databases and results transformed.

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod adapters;
mod registry;
mod router;
mod transform;

use registry::{DatabaseInfo, DatabaseRegistry};

/// Application state
pub struct AppState {
    pub registry: DatabaseRegistry,
    pub geojson_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🌐 Starting GLAF CDN Data Fabric...");

    let state = Arc::new(AppState {
        registry: DatabaseRegistry::new(),
        geojson_path: std::env::var("GEOJSON_PATH")
            .unwrap_or_else(|_| "./data/cdn-geo".to_string()),
    });

    // Build router
    let app = Router::new()
        // Health & Info
        .route("/health", get(health_check))
        .route("/api/v1/databases", get(list_databases))
        // Registry
        .route("/api/v1/registry/register", post(register_database))
        .route("/api/v1/registry/heartbeat/:db_id", post(heartbeat))
        .route(
            "/api/v1/registry/unregister/:db_id",
            post(unregister_database),
        )
        // Queries
        .route("/api/v1/query", post(execute_query))
        .route("/api/v1/graph/:db_id", get(get_graph))
        .route("/api/v1/table/:db_id/:table", get(get_table))
        .route("/api/v1/schema/:db_id", get(get_schema))
        // GeoJSON
        .route("/api/v1/geojson", get(list_geojson_layers))
        .route("/api/v1/geojson/:layer", get(get_geojson_layer))
        // Network Flow Analysis (Forge/SX9 Integration)
        .route("/api/v1/flow/events", get(get_flow_events))
        .route("/api/v1/flow/workflows", get(get_workflows))
        .route("/api/v1/flow/taps", get(get_taps_status))
        .route("/api/v1/flow/neural-mux", get(get_neural_mux_status))
        .route("/api/v1/flow/orchestrator", get(get_orchestrator_status))
        .route("/api/v1/flow/graph", get(get_flow_graph))
        // Live Updates
        .route("/ws/live", get(live_updates))
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let addr = "0.0.0.0:18100";
    info!("🚀 CDN Data Fabric listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ============================================================================
// Health & Info
// ============================================================================

async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_count = state.registry.count();
    Json(serde_json::json!({
        "status": "healthy",
        "service": "cdn-data-fabric",
        "version": env!("CARGO_PKG_VERSION"),
        "registered_databases": db_count
    }))
}

async fn list_databases(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let databases = state.registry.list_all();
    Json(databases)
}

// ============================================================================
// Registry Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    db_id: String,
    db_type: String,
    host: String,
    port: u16,
    namespace: Option<String>,
    database: Option<String>,
    capabilities: Vec<String>,
    schema: Option<serde_json::Value>,
    health_endpoint: Option<String>,
    heartbeat_interval_ms: Option<u64>,
}

async fn register_database(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let info = DatabaseInfo {
        db_id: req.db_id.clone(),
        db_type: req.db_type,
        host: req.host,
        port: req.port,
        namespace: req.namespace,
        database: req.database,
        capabilities: req.capabilities,
        schema: req.schema,
        health_endpoint: req.health_endpoint,
        last_heartbeat: chrono::Utc::now(),
        status: "online".to_string(),
    };

    state.registry.register(info);
    info!("✅ Registered database: {}", req.db_id);

    Ok(Json(serde_json::json!({
        "status": "registered",
        "db_id": req.db_id
    })))
}

async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Path(db_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    if state.registry.heartbeat(&db_id) {
        Ok(Json(serde_json::json!({"status": "ok"})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn unregister_database(
    State(state): State<Arc<AppState>>,
    Path(db_id): Path<String>,
) -> impl IntoResponse {
    state.registry.unregister(&db_id);
    info!("🔴 Unregistered database: {}", db_id);
    Json(serde_json::json!({"status": "unregistered"}))
}

// ============================================================================
// Query Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct QueryRequest {
    database: String,
    query: String,
    format: Option<String>, // graph, table, json, geojson
}

#[derive(Debug, Serialize)]
struct QueryResponse {
    database: String,
    format: String,
    data: serde_json::Value,
    stats: QueryStats,
}

#[derive(Debug, Serialize)]
struct QueryStats {
    rows_returned: usize,
    execution_time_ms: f64,
}

async fn execute_query(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, StatusCode> {
    let db_info = state
        .registry
        .get(&req.database)
        .ok_or(StatusCode::NOT_FOUND)?;

    let start = std::time::Instant::now();

    // Route to appropriate adapter based on db_type
    let result = match db_info.db_type.as_str() {
        "slotgraph" => adapters::slotgraph::execute(&db_info, &req.query).await,
        "postgres" => adapters::postgres::execute(&db_info, &req.query).await,
        "neo4j" => adapters::neo4j::execute(&db_info, &req.query).await,
        _ => Err(anyhow::anyhow!("Unsupported database type")),
    }
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let format = req.format.unwrap_or_else(|| "json".to_string());
    let data = transform::to_format(&result, &format);
    let execution_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(QueryResponse {
        database: req.database,
        format,
        data,
        stats: QueryStats {
            rows_returned: result.len(),
            execution_time_ms,
        },
    }))
}

#[derive(Debug, Deserialize)]
struct GraphParams {
    limit: Option<usize>,
    label: Option<String>,
}

async fn get_graph(
    State(state): State<Arc<AppState>>,
    Path(db_id): Path<String>,
    Query(params): Query<GraphParams>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let db_info = state.registry.get(&db_id).ok_or(StatusCode::NOT_FOUND)?;

    let limit = params.limit.unwrap_or(100);
    let query = match &params.label {
        Some(label) => format!("SELECT * FROM {} LIMIT {}", label, limit),
        None => format!("SELECT * FROM any_table LIMIT {}", limit),
    };

    let result = adapters::slotgraph::execute(&db_info, &query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let graph_data = transform::to_graph(&result);
    Ok(Json(graph_data))
}

async fn get_table(
    State(state): State<Arc<AppState>>,
    Path((db_id, table)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let db_info = state.registry.get(&db_id).ok_or(StatusCode::NOT_FOUND)?;

    let query = format!("SELECT * FROM {} LIMIT 1000", table);
    let result = adapters::slotgraph::execute(&db_info, &query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let table_data = transform::to_table(&result);
    Ok(Json(table_data))
}

async fn get_schema(
    State(state): State<Arc<AppState>>,
    Path(db_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let db_info = state.registry.get(&db_id).ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(db_info.schema.unwrap_or(serde_json::json!({}))))
}

// ============================================================================
// GeoJSON Endpoints
// ============================================================================

async fn list_geojson_layers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let layers = vec![
        "ground-stations",
        "submarine-cables",
        "cable-landings",
        "landing-points",
    ];
    Json(serde_json::json!({
        "layers": layers,
        "path": state.geojson_path
    }))
}

async fn get_geojson_layer(
    State(state): State<Arc<AppState>>,
    Path(layer): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let file_path = format!("{}/{}.geojson", state.geojson_path, layer);

    let content = tokio::fs::read_to_string(&file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let geojson: serde_json::Value =
        serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(geojson))
}

// ============================================================================
// Network Flow Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct FlowEventsParams {
    limit: Option<usize>,
    event_type: Option<String>,
}

async fn get_flow_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FlowEventsParams>,
) -> Result<Json<Vec<adapters::network_flow::NetworkFlowEvent>>, StatusCode> {
    let limit = params.limit.unwrap_or(100);

    // Get SX9 orchestrator info from registry
    let sx9_info =
        state
            .registry
            .get("sx9-orchestrator")
            .unwrap_or_else(|| crate::registry::DatabaseInfo {
                db_id: "sx9-orchestrator".to_string(),
                db_type: "network_flow".to_string(),
                host: "localhost".to_string(),
                port: 15174,
                namespace: None,
                database: None,
                capabilities: vec!["network_flow".to_string()],
                schema: None,
                health_endpoint: Some("/health".to_string()),
                last_heartbeat: chrono::Utc::now(),
                status: "unknown".to_string(),
            });

    let events =
        adapters::network_flow::get_flow_events(&sx9_info, limit, params.event_type.as_deref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}

async fn get_workflows(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<adapters::network_flow::WorkflowState>>, StatusCode> {
    let sx9_info =
        state
            .registry
            .get("sx9-orchestrator")
            .unwrap_or_else(|| crate::registry::DatabaseInfo {
                db_id: "sx9-orchestrator".to_string(),
                db_type: "network_flow".to_string(),
                host: "localhost".to_string(),
                port: 15174,
                namespace: None,
                database: None,
                capabilities: vec!["network_flow".to_string()],
                schema: None,
                health_endpoint: Some("/health".to_string()),
                last_heartbeat: chrono::Utc::now(),
                status: "unknown".to_string(),
            });

    let workflows = adapters::network_flow::get_workflow_states(&sx9_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(workflows))
}

async fn get_taps_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<adapters::network_flow::TapsBufferState>, StatusCode> {
    let sx9_info =
        state
            .registry
            .get("sx9-orchestrator")
            .unwrap_or_else(|| crate::registry::DatabaseInfo {
                db_id: "sx9-orchestrator".to_string(),
                db_type: "network_flow".to_string(),
                host: "localhost".to_string(),
                port: 15174,
                namespace: None,
                database: None,
                capabilities: vec!["network_flow".to_string()],
                schema: None,
                health_endpoint: Some("/health".to_string()),
                last_heartbeat: chrono::Utc::now(),
                status: "unknown".to_string(),
            });

    let taps = adapters::network_flow::get_taps_buffer_state(&sx9_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(taps))
}

async fn get_neural_mux_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<adapters::network_flow::NeuralMuxState>, StatusCode> {
    let sx9_info =
        state
            .registry
            .get("sx9-orchestrator")
            .unwrap_or_else(|| crate::registry::DatabaseInfo {
                db_id: "sx9-orchestrator".to_string(),
                db_type: "network_flow".to_string(),
                host: "localhost".to_string(),
                port: 15174,
                namespace: None,
                database: None,
                capabilities: vec!["network_flow".to_string()],
                schema: None,
                health_endpoint: Some("/health".to_string()),
                last_heartbeat: chrono::Utc::now(),
                status: "unknown".to_string(),
            });

    let mux = adapters::network_flow::get_neural_mux_state(&sx9_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(mux))
}

async fn get_orchestrator_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<adapters::network_flow::ServiceOrchestratorState>, StatusCode> {
    let sx9_info =
        state
            .registry
            .get("sx9-orchestrator")
            .unwrap_or_else(|| crate::registry::DatabaseInfo {
                db_id: "sx9-orchestrator".to_string(),
                db_type: "network_flow".to_string(),
                host: "localhost".to_string(),
                port: 15174,
                namespace: None,
                database: None,
                capabilities: vec!["network_flow".to_string()],
                schema: None,
                health_endpoint: Some("/health".to_string()),
                last_heartbeat: chrono::Utc::now(),
                status: "unknown".to_string(),
            });

    let orchestrator = adapters::network_flow::get_orchestrator_state(&sx9_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(orchestrator))
}

async fn get_flow_graph(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let sx9_info =
        state
            .registry
            .get("sx9-orchestrator")
            .unwrap_or_else(|| crate::registry::DatabaseInfo {
                db_id: "sx9-orchestrator".to_string(),
                db_type: "network_flow".to_string(),
                host: "localhost".to_string(),
                port: 15174,
                namespace: None,
                database: None,
                capabilities: vec!["network_flow".to_string()],
                schema: None,
                health_endpoint: Some("/health".to_string()),
                last_heartbeat: chrono::Utc::now(),
                status: "unknown".to_string(),
            });

    let events = adapters::network_flow::get_flow_events(&sx9_info, 100, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let workflows = adapters::network_flow::get_workflow_states(&sx9_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let orchestrator = adapters::network_flow::get_orchestrator_state(&sx9_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let graph = adapters::network_flow::flow_to_graph(&events, &workflows, &orchestrator);

    Ok(Json(graph))
}

// ============================================================================
// Live Updates WebSocket
// ============================================================================

async fn live_updates(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        info!("WebSocket client connected for live updates");
        // TODO: Implement live subscription routing
    })
}
