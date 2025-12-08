//! GLAF Graph Server
//!
//! Rust backend for GLAF Browser with Legion ECS and GLAF math.
//! Replaces Neo4j with SurrealDB + native Rust graph processing.

use axum::{
    extract::{Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod glaf;
mod graph;
mod surreal;

use glaf::GlafMathEngine;
use graph::GraphState;
use surreal::SurrealClient;

/// Application state shared across handlers
pub struct AppState {
    pub surreal: SurrealClient,
    pub graph: RwLock<GraphState>,
    pub glaf: GlafMathEngine,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🧬 Starting GLAF Graph Server...");

    // Connect to SurrealDB
    let surreal = SurrealClient::connect(
        "ws://localhost:18019",
        "ctas7",
        "glaf",
        "root",
        "root",
    )
    .await?;

    info!("✅ Connected to SurrealDB (GLAF Core)");

    // Initialize graph state
    let graph = RwLock::new(GraphState::new());

    // Initialize GLAF math engine
    let glaf = GlafMathEngine::new();

    let state = Arc::new(AppState {
        surreal,
        graph,
        glaf,
    });

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        // Graph queries
        .route("/api/graph/nodes", get(get_nodes))
        .route("/api/graph/relationships", get(get_relationships))
        .route("/api/query", post(run_query))
        // GLAF math endpoints
        .route("/api/glaf/matroid-rank", post(calculate_matroid_rank))
        .route("/api/glaf/hawkes-intensity", post(calculate_hawkes))
        .route("/api/glaf/convergence", post(calculate_convergence))
        // WebSocket for real-time updates
        .route("/ws/stream", get(graph_stream))
        // CORS for browser
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state);

    let addr = "0.0.0.0:18050";
    info!("🚀 GLAF Graph Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ============================================================================
// Health Check
// ============================================================================

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "glaf-graph-server",
        "version": env!("CARGO_PKG_VERSION"),
        "backend": "rust-legion-ecs"
    }))
}

// ============================================================================
// Graph Query Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct NodeParams {
    limit: Option<usize>,
    hd4_phase: Option<String>,
    label: Option<String>,
}

#[derive(Debug, Serialize)]
struct GlafNode {
    id: String,
    element_id: String,
    labels: Vec<String>,
    properties: serde_json::Value,
    #[serde(rename = "_glaf")]
    glaf_meta: GlafNodeMeta,
}

#[derive(Debug, Serialize)]
struct GlafNodeMeta {
    triv_hash: Option<String>,
    hd4_phase: Option<String>,
    teth_entropy: Option<f64>,
    matroid_rank: Option<f64>,
}

async fn get_nodes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<NodeParams>,
) -> Result<Json<Vec<GlafNode>>, StatusCode> {
    let limit = params.limit.unwrap_or(100);
    
    let surql = match (&params.hd4_phase, &params.label) {
        (Some(phase), _) => format!(
            "SELECT * FROM ptcc_configurations WHERE recommended_hd4_phase = '{}' LIMIT {}",
            phase, limit
        ),
        (_, Some(label)) => format!(
            "SELECT * FROM {} LIMIT {}",
            label, limit
        ),
        _ => format!("SELECT * FROM ptcc_configurations LIMIT {}", limit),
    };

    let result = state.surreal.query(&surql).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let nodes = transform_to_nodes(result);
    Ok(Json(nodes))
}

#[derive(Debug, Deserialize)]
struct RelParams {
    from: Option<String>,
    to: Option<String>,
    rel_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct GlafRelationship {
    id: String,
    element_id: String,
    #[serde(rename = "type")]
    rel_type: String,
    start_node_id: String,
    end_node_id: String,
    properties: serde_json::Value,
    #[serde(rename = "_glaf")]
    glaf_meta: GlafEdgeMeta,
}

#[derive(Debug, Serialize)]
struct GlafEdgeMeta {
    hawkes_intensity: Option<f64>,
    convergence_score: Option<f64>,
}

async fn get_relationships(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RelParams>,
) -> Result<Json<Vec<GlafRelationship>>, StatusCode> {
    // TODO: Implement relationship queries
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
struct QueryRequest {
    surql: String,
}

#[derive(Debug, Serialize)]
struct QueryResult {
    nodes: Vec<GlafNode>,
    relationships: Vec<GlafRelationship>,
    stats: QueryStats,
}

#[derive(Debug, Serialize)]
struct QueryStats {
    nodes_returned: usize,
    relationships_returned: usize,
    execution_time_ms: f64,
}

async fn run_query(
    State(state): State<Arc<AppState>>,
    Json(body): Json<QueryRequest>,
) -> Result<Json<QueryResult>, StatusCode> {
    let start = std::time::Instant::now();
    
    let result = state.surreal.query(&body.surql).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let nodes = transform_to_nodes(result);
    let execution_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(QueryResult {
        nodes: nodes.clone(),
        relationships: vec![],
        stats: QueryStats {
            nodes_returned: nodes.len(),
            relationships_returned: 0,
            execution_time_ms,
        },
    }))
}

// ============================================================================
// GLAF Math Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct MatroidRequest {
    fragment_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct MatroidResult {
    rank: usize,
    max_possible: usize,
    normalized: f64,
}

async fn calculate_matroid_rank(
    State(state): State<Arc<AppState>>,
    Json(body): Json<MatroidRequest>,
) -> Result<Json<MatroidResult>, StatusCode> {
    let result = state.glaf.calculate_matroid_rank(&body.fragment_ids);
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
struct HawkesRequest {
    event_type: String,
    window_hours: Option<f64>,
}

#[derive(Debug, Serialize)]
struct HawkesResult {
    intensity: f64,
    branching_ratio: f64,
    is_stable: bool,
    event_count: usize,
}

async fn calculate_hawkes(
    State(state): State<Arc<AppState>>,
    Json(body): Json<HawkesRequest>,
) -> Result<Json<HawkesResult>, StatusCode> {
    let window = body.window_hours.unwrap_or(24.0);
    let result = state.glaf.calculate_hawkes(&body.event_type, window);
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
struct ConvergenceRequest {
    fragment_indices: Vec<usize>,
    h1_input: f64,
}

#[derive(Debug, Serialize)]
struct ConvergenceResult {
    h1_operational: f64,
    h2_semantic: f64,
    combined: f64,
    exceeds_threshold: bool,
}

async fn calculate_convergence(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ConvergenceRequest>,
) -> Result<Json<ConvergenceResult>, StatusCode> {
    let result = state.glaf.calculate_convergence(&body.fragment_indices, body.h1_input);
    Ok(Json(result))
}

// ============================================================================
// WebSocket Streaming
// ============================================================================

async fn graph_stream(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        // TODO: Implement real-time graph streaming from Legion ECS
        info!("WebSocket client connected");
    })
}

// ============================================================================
// Transform Helpers
// ============================================================================

fn transform_to_nodes(result: Vec<serde_json::Value>) -> Vec<GlafNode> {
    result
        .into_iter()
        .filter_map(|record| {
            let id = record.get("id")?.as_str()?.to_string();
            let labels = extract_labels(&id);
            
            Some(GlafNode {
                id: id.clone(),
                element_id: id.clone(),
                labels,
                properties: record.clone(),
                glaf_meta: GlafNodeMeta {
                    triv_hash: record.get("triv_hash").and_then(|v| v.as_str()).map(String::from),
                    hd4_phase: record.get("recommended_hd4_phase").and_then(|v| v.as_str()).map(String::from),
                    teth_entropy: record.get("entropy_h").and_then(|v| v.as_f64()),
                    matroid_rank: None,
                },
            })
        })
        .collect()
}

fn extract_labels(id: &str) -> Vec<String> {
    // Extract table name from SurrealDB ID (e.g., "ptcc_configurations:abc" -> ["PtccConfiguration"])
    if let Some(table) = id.split(':').next() {
        vec![to_pascal_case(table)]
    } else {
        vec!["Node".to_string()]
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}


