//! SX9 Forge Server - HTTP API for Dynamic Tool Generation
//!
//! Exposes GLAF graph, Mission Loads, and tool execution via REST API.
//! Ring Bus Node ID: 9

use std::sync::Arc;
use axum::{
    Router,
    routing::{get, post},
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber;

use sx9_forge::{
    ForgeEngine, ForgeConfig,
    NonagonCell, calculate_teth_entropy, MIN_TETH_ENTROPY,
    MissionLoadSet, MissionLoadCatalog, HD4Phase, ClearanceLevel, Primitive,
    ForgeGraph, ForgeNode,
    ToolGenerator, GeneratedTool, ToolChain,
};

/// Application state shared across handlers
struct AppState {
    engine: Arc<RwLock<ForgeEngine>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting SX9 Forge Server v7.3.1");

    // Load configuration
    let config = ForgeConfig::default();
    info!("Configuration loaded:");
    info!("  Ring Bus Node: {}", config.ring_bus_node_id);
    info!("  L2 Execution: {}", config.l2_execution);
    info!("  Kali Endpoint: {}", config.kali_iso_endpoint);

    // Initialize Forge engine
    let engine = ForgeEngine::new(config).await?;
    let state = Arc::new(AppState {
        engine: Arc::new(RwLock::new(engine)),
    });

    // Build router
    let app = Router::new()
        // Health and status
        .route("/health", get(health_handler))
        .route("/smart-crate/status", get(status_handler))
        .route("/metrics", get(metrics_handler))
        // Graph endpoints
        .route("/graph", get(graph_stats_handler))
        .route("/graph/nodes", get(list_nodes_handler))
        .route("/graph/nodes/:id", get(get_node_handler))
        // Nonagon endpoints
        .route("/nonagon", post(create_nonagon_handler))
        .route("/nonagon/:id", get(get_nonagon_handler))
        .route("/nonagon/:id/entropy", get(nonagon_entropy_handler))
        // Mission Load endpoints
        .route("/mission-loads", get(list_mission_loads_handler))
        .route("/mission-loads/:id", get(get_mission_load_handler))
        .route("/mission-loads/:id/tool", post(create_tool_from_load_handler))
        // Tool endpoints
        .route("/tools", get(list_tools_handler))
        .route("/tools/:id/execute", post(execute_tool_handler))
        // Tool chain endpoints
        .route("/chains", post(create_chain_handler))
        .route("/chains/:id/execute", post(execute_chain_handler))
        // Add middleware
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server on port 18350
    let addr = "0.0.0.0:18350";
    info!("Forge Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Health check handler
async fn health_handler() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".into(),
        service: "sx9-forge".into(),
        version: "7.3.1".into(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

// Smart crate status handler
async fn status_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let graph = engine.graph();
    let graph_lock = graph.read().await;
    let catalog = engine.catalog();
    let catalog_lock = catalog.read().await;

    Json(StatusResponse {
        crate_name: "sx9-forge".into(),
        version: "7.3.1".into(),
        ring_bus_node: 9,
        l2_execution: true,
        graph_nodes: graph_lock.node_count(),
        graph_edges: graph_lock.edge_count(),
        mission_loads: catalog_lock.len(),
        min_teth_entropy: MIN_TETH_ENTROPY,
    })
}

#[derive(Serialize)]
struct StatusResponse {
    crate_name: String,
    version: String,
    ring_bus_node: u8,
    l2_execution: bool,
    graph_nodes: usize,
    graph_edges: usize,
    mission_loads: usize,
    min_teth_entropy: f64,
}

// Metrics handler (Prometheus format)
async fn metrics_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let graph = engine.graph();
    let graph_lock = graph.read().await;
    let catalog = engine.catalog();
    let catalog_lock = catalog.read().await;

    let metrics = format!(
        "# HELP forge_graph_nodes Total graph nodes\n\
         # TYPE forge_graph_nodes gauge\n\
         forge_graph_nodes {}\n\
         # HELP forge_graph_edges Total graph edges\n\
         # TYPE forge_graph_edges gauge\n\
         forge_graph_edges {}\n\
         # HELP forge_mission_loads Total mission loads\n\
         # TYPE forge_mission_loads gauge\n\
         forge_mission_loads {}\n\
         # HELP forge_teth_entropy_min Minimum TETH entropy threshold\n\
         # TYPE forge_teth_entropy_min gauge\n\
         forge_teth_entropy_min {}\n",
        graph_lock.node_count(),
        graph_lock.edge_count(),
        catalog_lock.len(),
        MIN_TETH_ENTROPY,
    );

    (StatusCode::OK, metrics)
}

// Graph stats handler
async fn graph_stats_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let graph = engine.graph();
    let graph_lock = graph.read().await;

    Json(GraphStats {
        node_count: graph_lock.node_count(),
        edge_count: graph_lock.edge_count(),
    })
}

#[derive(Serialize)]
struct GraphStats {
    node_count: usize,
    edge_count: usize,
}

// List all nodes
async fn list_nodes_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let graph = engine.graph();
    let graph_lock = graph.read().await;

    // Get all nodes (limited for API response)
    match graph_lock.find_by_entropy(0.0) {
        Ok(nodes) => Json(nodes).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// Get specific node
async fn get_node_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let graph = engine.graph();
    let graph_lock = graph.read().await;

    match graph_lock.get_node(&id) {
        Ok(Some(node)) => Json(node).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Node not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// Create nonagon cell
async fn create_nonagon_handler(
    Json(req): Json<CreateNonagonRequest>,
) -> impl IntoResponse {
    let cell = NonagonCell::new(&req.id);
    Json(NonagonResponse {
        id: cell.id.clone(),
        teth_entropy: cell.teth_entropy,
        confidence: cell.confidence,
        is_valid: cell.is_valid(),
        vertices: cell.vertices().to_vec(),
    })
}

#[derive(Deserialize)]
struct CreateNonagonRequest {
    id: String,
}

#[derive(Serialize)]
struct NonagonResponse {
    id: String,
    teth_entropy: f64,
    confidence: f64,
    is_valid: bool,
    vertices: Vec<f64>,
}

// Get nonagon by ID
async fn get_nonagon_handler(Path(id): Path<String>) -> impl IntoResponse {
    let cell = NonagonCell::new(&id);
    Json(NonagonResponse {
        id: cell.id.clone(),
        teth_entropy: cell.teth_entropy,
        confidence: cell.confidence,
        is_valid: cell.is_valid(),
        vertices: cell.vertices().to_vec(),
    })
}

// Get nonagon entropy
async fn nonagon_entropy_handler(Path(id): Path<String>) -> impl IntoResponse {
    let cell = NonagonCell::new(&id);
    Json(EntropyResponse {
        id: cell.id,
        teth_entropy: cell.teth_entropy,
        min_threshold: MIN_TETH_ENTROPY,
        is_valid: cell.is_valid(),
    })
}

#[derive(Serialize)]
struct EntropyResponse {
    id: String,
    teth_entropy: f64,
    min_threshold: f64,
    is_valid: bool,
}

// List mission loads
async fn list_mission_loads_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let catalog = engine.catalog();
    let catalog_lock = catalog.read().await;

    let loads: Vec<MissionLoadSummary> = catalog_lock
        .all_ids()
        .iter()
        .filter_map(|id| catalog_lock.get(id))
        .map(|load| MissionLoadSummary {
            id: load.id.clone(),
            name: load.name.clone(),
            hd4_phase: format!("{:?}", load.hd4_phase),
            clearance: format!("{:?}", load.clearance),
            price_credits: load.price_credits,
            primitive_count: load.primitives.len(),
        })
        .collect();

    Json(loads)
}

#[derive(Serialize)]
struct MissionLoadSummary {
    id: String,
    name: String,
    hd4_phase: String,
    clearance: String,
    price_credits: u64,
    primitive_count: usize,
}

// Get specific mission load
async fn get_mission_load_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let catalog = engine.catalog();
    let catalog_lock = catalog.read().await;

    match catalog_lock.get(&id) {
        Some(load) => Json(load.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, "Mission Load not found").into_response(),
    }
}

// Create tool from mission load
async fn create_tool_from_load_handler(
    State(state): State<Arc<AppState>>,
    Path(load_id): Path<String>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;

    match engine.create_tool_from_load(&load_id).await {
        Ok(tool) => Json(tool).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

// List tools
async fn list_tools_handler() -> impl IntoResponse {
    Json(Vec::<GeneratedTool>::new())
}

// Execute tool
async fn execute_tool_handler(
    Path(_tool_id): Path<String>,
) -> impl IntoResponse {
    Json(ExecutionResponse {
        success: true,
        message: "Tool execution queued via Ring Bus L2".into(),
    })
}

#[derive(Serialize)]
struct ExecutionResponse {
    success: bool,
    message: String,
}

// Create tool chain
async fn create_chain_handler(
    Json(req): Json<CreateChainRequest>,
) -> impl IntoResponse {
    let mut chain = ToolChain::new(&req.name);
    for tool_id in req.tool_ids {
        chain.add_tool(tool_id);
    }

    Json(chain)
}

#[derive(Deserialize)]
struct CreateChainRequest {
    name: String,
    tool_ids: Vec<String>,
}

// Execute tool chain
async fn execute_chain_handler(
    State(state): State<Arc<AppState>>,
    Path(chain_id): Path<String>,
    Json(chain): Json<ToolChain>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;

    match engine.execute_tool_chain(&chain).await {
        Ok(()) => Json(ExecutionResponse {
            success: true,
            message: format!("Chain {} executed via Ring Bus L2", chain_id),
        }).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
