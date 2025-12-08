//! CTAS-7 Hashing Engine Microservice v7.3.1
//!
//! Pure microservice for trivariate hash generation
//! - gRPC on port 50051
//! - REST on port 8002
//! - Prometheus metrics on /metrics
//!
//! **Ground Truth**: Murmur3 trivariate hashing ONLY (NO Blake3)

use axum::{
    extract::{State, ws::{WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use ctas7_foundation_core::hashing::TrivariteHashEngine;
use serde::{Deserialize, Serialize};
// SHA256 removed - Ground Truth: Murmur3 ONLY
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;

/// Service state
#[derive(Clone)]
struct AppState {
    hash_engine: Arc<RwLock<TrivariteHashEngine>>,
    request_count: Arc<RwLock<u64>>,
}

/// Hash generation request
#[derive(Debug, Deserialize)]
struct HashRequest {
    content: String,
    context: String,
    primitive_type: String,
    #[serde(default)]
    compress_unicode: bool,
    // TODO: Add environmental masks support when foundation-core exports them with Deserialize
}

/// Hash generation response
#[derive(Debug, Serialize)]
struct HashResponse {
    trivariate_hash: String,
    sch: String,
    cuid: String,
    uuid: String,
    unicode_compressed: Option<String>,
    generation_time_ms: f64,
}

/// Batch hash request
#[derive(Debug, Deserialize)]
struct BatchHashRequest {
    items: Vec<BatchItem>,
    #[serde(default)]
    compress_unicode: bool,
    #[serde(default)]
    preserve_context: bool,
    batch_context: Option<String>, // e.g., "ground_stations_257", "kali_tools_list"
}

/// Individual item in batch
#[derive(Debug, Deserialize)]
struct BatchItem {
    id: String, // e.g., "station_001", "nmap"
    content: String,
    context: String,
    primitive_type: String,
}

/// Batch hash response
#[derive(Debug, Serialize)]
struct BatchHashResponse {
    batch_hash: String, // Hash of the entire batch
    batch_context: Option<String>,
    items: Vec<BatchHashItem>,
    total_generation_time_ms: f64,
    items_per_second: f64,
}

/// Individual item response in batch
#[derive(Debug, Serialize)]
struct BatchHashItem {
    id: String,
    trivariate_hash: String,
    sch: String,
    cuid: String,
    uuid: String,
    unicode_compressed: Option<String>,
    batch_index: usize,
}

/// Health check response
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
    total_requests: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .json()
        .init();

    info!("🔐 CTAS-7 Hashing Engine v7.3.1 starting...");
    info!("📊 Ground Truth: Murmur3 Trivariate Hash (NO Blake3)");

    // Initialize state
    let state = AppState {
        hash_engine: Arc::new(RwLock::new(TrivariteHashEngine::new())),
        request_count: Arc::new(RwLock::new(0)),
    };

    // Build REST API
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/hash", post(generate_hash))
        .route("/hash/batch", post(generate_hash_batch))
        .route("/hash/stream", get(hash_stream_handler))
        .route("/usim", post(generate_usim))
        .route("/usim/header", post(generate_usim_header))
        .route("/metrics", get(metrics))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start REST server
    let rest_addr = "0.0.0.0:8002";
    info!("🌐 REST API listening on {}", rest_addr);
    
    let listener = tokio::net::TcpListener::bind(rest_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let count = *state.request_count.read().await;
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "7.3.1".to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
        total_requests: count,
    })
}

/// Generate trivariate hash
async fn generate_hash(
    State(state): State<AppState>,
    Json(request): Json<HashRequest>,
) -> Result<Json<HashResponse>, StatusCode> {
    let start = std::time::Instant::now();
    
    // Increment request counter
    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }
    
    // Get hash engine
    let engine = state.hash_engine.read().await;
    
    // Generate hash
    let trivariate_hash = engine.generate_trivariate_hash(
        &request.content,
        &request.context,
        &request.primitive_type,
    );
    
    let sch = trivariate_hash[0..16].to_string();
    let cuid = trivariate_hash[16..32].to_string();
    let uuid = trivariate_hash[32..48].to_string();
    
    // Optional Unicode compression
    let unicode_compressed = if request.compress_unicode {
        Some(engine.generate_unicode_compressed(&sch, &cuid, &uuid))
    } else {
        None
    };
    
    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;
    
    info!(
        "✅ Generated hash in {:.3}ms: {}... (unicode: {})",
        generation_time_ms,
        &trivariate_hash[0..12],
        unicode_compressed.is_some()
    );
    
    Ok(Json(HashResponse {
        trivariate_hash,
        sch,
        cuid,
        uuid,
        unicode_compressed,
        generation_time_ms,
    }))
}

/// Generate batch of hashes with context preservation
async fn generate_hash_batch(
    State(state): State<AppState>,
    Json(request): Json<BatchHashRequest>,
) -> Result<Json<BatchHashResponse>, StatusCode> {
    let start = std::time::Instant::now();
    
    // Increment request counter
    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }
    
    // Get hash engine
    let engine = state.hash_engine.read().await;
    
    let mut batch_items = Vec::new();
    let mut all_hashes = String::new();
    
    // Process each item in batch
    for (index, item) in request.items.iter().enumerate() {
        // Generate hash with batch context if requested
        let context = if request.preserve_context {
            format!("{}:{}:{}", 
                request.batch_context.as_deref().unwrap_or("batch"),
                index,
                item.context
            )
        } else {
            item.context.clone()
        };
        
        let trivariate_hash = engine.generate_trivariate_hash(
            &item.content,
            &context,
            &item.primitive_type,
        );
        
        let sch = trivariate_hash[0..16].to_string();
        let cuid = trivariate_hash[16..32].to_string();
        let uuid = trivariate_hash[32..48].to_string();
        
        // Optional Unicode compression
        let unicode_compressed = if request.compress_unicode {
                        Some(engine.generate_unicode_compressed(&sch, &cuid, &uuid))
                    } else {
                        None
                    };
        
        // Accumulate for batch hash
        all_hashes.push_str(&trivariate_hash);
        
        batch_items.push(BatchHashItem {
            id: item.id.clone(),
            trivariate_hash,
            sch,
            cuid,
            uuid,
            unicode_compressed,
            batch_index: index,
        });
    }
    
    // Generate hash of entire batch
    let batch_hash = engine.generate_trivariate_hash(
        &all_hashes,
        request.batch_context.as_deref().unwrap_or("batch"),
        "BATCH_HASH",
    );
    
    let total_time_ms = start.elapsed().as_micros() as f64 / 1000.0;
    let items_per_second = (request.items.len() as f64 / total_time_ms) * 1000.0;
    
    info!(
        "✅ Generated {} hashes in {:.3}ms ({:.0} items/sec) - Batch: {}...",
        request.items.len(),
        total_time_ms,
        items_per_second,
        &batch_hash[0..12]
    );
    
    Ok(Json(BatchHashResponse {
        batch_hash,
        batch_context: request.batch_context,
        items: batch_items,
        total_generation_time_ms: total_time_ms,
        items_per_second,
    }))
}

/// WebSocket streaming hash handler
async fn hash_stream_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| hash_stream(socket, state))
}

/// Handle streaming hash generation over WebSocket
async fn hash_stream(stream: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = stream.split();
    
    info!("🌊 WebSocket stream connected");
    
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if let Ok(text) = msg.to_text() {
                // Parse streaming hash request
                if let Ok(request) = serde_json::from_str::<StreamHashRequest>(text) {
                    // Increment counter
                    {
                        let mut count = state.request_count.write().await;
                        *count += 1;
                    }
                    
                    // Generate hash
                    let engine = state.hash_engine.read().await;
                    let start = std::time::Instant::now();
                    
                    let trivariate_hash = engine.generate_trivariate_hash(
                        &request.content,
                        &request.context,
                        &request.primitive_type,
                    );
                    
                    let sch = trivariate_hash[0..16].to_string();
                    let cuid = trivariate_hash[16..32].to_string();
                    let uuid = trivariate_hash[32..48].to_string();
                    
                    let unicode_compressed = if request.compress_unicode {
                        Some(engine.generate_unicode_compressed(&sch, &cuid, &uuid))
                    } else {
                        None
                    };
                    
                    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;
                    
                    // Send response back
                    let response = StreamHashResponse {
                        stream_id: request.stream_id,
                        trivariate_hash,
                        sch,
                        cuid,
                        uuid,
                        unicode_compressed,
                        generation_time_ms,
                        timestamp: chrono::Utc::now().timestamp_millis(),
                    };
                    
                    if let Ok(json) = serde_json::to_string(&response) {
                        let _ = sender.send(axum::extract::ws::Message::Text(json)).await;
                    }
                }
            }
        }
    }
    
    info!("🌊 WebSocket stream disconnected");
}

/// Streaming hash request
#[derive(Debug, Deserialize)]
struct StreamHashRequest {
    stream_id: String,
    content: String,
    context: String,
    primitive_type: String,
    #[serde(default)]
    compress_unicode: bool,
}

/// Streaming hash response
#[derive(Debug, Serialize)]
struct StreamHashResponse {
    stream_id: String,
    trivariate_hash: String,
    sch: String,
    cuid: String,
    uuid: String,
    unicode_compressed: Option<String>,
    generation_time_ms: f64,
    timestamp: i64,
}

/// Generate USIM (Universal Symbolic Message)
async fn generate_usim(
    State(state): State<AppState>,
    Json(request): Json<UsimRequest>,
) -> Result<Json<UsimResponse>, StatusCode> {
    let start = std::time::Instant::now();
    
    // Increment counter
    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }
    
    // Get hash engine
    let engine = state.hash_engine.read().await;
    
    // Generate Murmur3 trivariate hash for USIM addressing
    let usim_hash = engine.generate_trivariate_hash(
        &request.file_path,
        &request.domain,
        "USIM",
    );
    
    // Generate Murmur3 hash for file integrity (NO SHA256)
    // We use the trivariate engine but with a specific context for integrity
    let integrity_hash = engine.generate_trivariate_hash(
        &request.content,
        "integrity_check",
        "murmur3_integrity"
    );
    // Use the full hash as the integrity check
    let integrity_hex = integrity_hash.clone();
    
    // Unicode compression
    let sch = usim_hash[0..16].to_string();
    let cuid = usim_hash[16..32].to_string();
    let uuid = usim_hash[32..48].to_string();
    let unicode_compressed = engine.generate_unicode_compressed(&sch, &cuid, &uuid);
    
    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;
    
    info!(
        "✅ Generated USIM in {:.3}ms: {}... (unicode: {})",
        generation_time_ms,
        &usim_hash[0..12],
        unicode_compressed
    );
    
    Ok(Json(UsimResponse {
        usim_hash,
        integrity_hash: integrity_hex,
        unicode_compressed,
        sch,
        cuid,
        uuid,
        generation_time_ms,
    }))
}

/// Generate printable USIM header for inventory/documentation
async fn generate_usim_header(
    State(state): State<AppState>,
    Json(request): Json<UsimHeaderRequest>,
) -> Result<Json<UsimHeaderResponse>, StatusCode> {
    let start = std::time::Instant::now();
    
    // Increment counter
    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }
    
    // Get hash engine
    let engine = state.hash_engine.read().await;
    
    // Generate hashes
    let usim_hash = engine.generate_trivariate_hash(
        &request.file_path,
        &request.domain,
        "USIM",
    );
    
    // Generate Murmur3 hash for file integrity (NO SHA256)
    let integrity_hash = engine.generate_trivariate_hash(
        &request.content,
        "integrity_check",
        "murmur3_integrity"
    );
    let integrity_hex = integrity_hash.clone();
    
    // Unicode compression
    let sch = usim_hash[0..16].to_string();
    let cuid = usim_hash[16..32].to_string();
    let uuid = usim_hash[32..48].to_string();
    let unicode_compressed = engine.generate_unicode_compressed(&sch, &cuid, &uuid);
    
    // Generate header based on format
    let header = match request.format {
        UsimHeaderFormat::Full => format!(
            r#"/*
// ┌─────────────────────────────────────────────────────────────┐
// │ █████████████████ CTAS USIM HEADER ███████████████████████ │
// ├─────────────────────────────────────────────────────────────┤
// │ 🔖 usim_hash     : {}                │
// │ 🔐 integrity_hash: {}... │
// │ 📦 unicode       : {}                                │
// │ 📁 domain        : {}                                │
// │ 🧠 description   : {}                                │
// │ 🕸️ hash_type     : SCH+CUID+UUID trivariate (Murmur3)      │
// │ 🔄 parent_node   : {}                                │
// │ 🧩 dependencies  : {}                                │
// │ 🔧 language      : {}                                │
// │ 📡 file_type     : {}                                │
// │ 🧪 complexity    : {}                                │
// │ ⌛ TTL Policy    : {}                                │
// └─────────────────────────────────────────────────────────────┘
*/"#,
            usim_hash,
            &integrity_hex[0..32],
            unicode_compressed,
            request.domain,
            request.description,
            request.parent_node.unwrap_or_else(|| "ROOT".to_string()),
            request.dependencies.join(", "),
            request.language.unwrap_or_else(|| "Unknown".to_string()),
            request.file_type.unwrap_or_else(|| "Unknown".to_string()),
            request.complexity.unwrap_or(0.0),
            request.ttl_policy.unwrap_or_else(|| "Persistent".to_string()),
        ),
        
        UsimHeaderFormat::Footer => format!(
            r#"
────────────────────────────────────────────────────────────────────
Document ID: {}
Integrity: {}...
Index: {}
Domain: {} | {}
────────────────────────────────────────────────────────────────────"#,
            usim_hash,
            &integrity_hex[0..16],
            unicode_compressed,
            request.domain,
            request.description,
        ),
        
        UsimHeaderFormat::Index => format!(
            r#"[{}] {} - {}
    📦 {} | 🔐 {}... | 📁 {}"#,
            unicode_compressed,
            request.file_path,
            request.description,
            usim_hash,
            &integrity_hex[0..16],
            request.domain,
        ),
        
        UsimHeaderFormat::Minimal => format!(
            r#"USIM: {} | Unicode: {} | SHA256: {}..."#,
            usim_hash,
            unicode_compressed,
            &integrity_hex[0..16],
        ),
    };
    
    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;
    
    info!(
        "✅ Generated USIM header in {:.3}ms for {}",
        generation_time_ms,
        request.file_path
    );
    
    Ok(Json(UsimHeaderResponse {
        header,
        usim_hash,
        integrity_hash: integrity_hex,
        unicode_compressed,
        generation_time_ms,
    }))
}

/// USIM request
#[derive(Debug, Deserialize)]
struct UsimRequest {
    file_path: String,
    content: String,
    domain: String,
}

/// USIM response
#[derive(Debug, Serialize)]
struct UsimResponse {
    usim_hash: String,
    integrity_hash: String,
    unicode_compressed: String,
    sch: String,
    cuid: String,
    uuid: String,
    generation_time_ms: f64,
}

/// USIM header request
#[derive(Debug, Deserialize)]
struct UsimHeaderRequest {
    file_path: String,
    content: String,
    domain: String,
    description: String,
    parent_node: Option<String>,
    dependencies: Vec<String>,
    language: Option<String>,
    file_type: Option<String>,
    complexity: Option<f64>,
    ttl_policy: Option<String>,
    #[serde(default)]
    format: UsimHeaderFormat,
}

/// USIM header format options
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum UsimHeaderFormat {
    #[default]
    Full,      // Full header with all metadata
    Footer,    // Compact footer for legal docs
    Index,     // Index-style for catalogs
    Minimal,   // Just hash + unicode
}

/// USIM header response
#[derive(Debug, Serialize)]
struct UsimHeaderResponse {
    header: String,
    usim_hash: String,
    integrity_hash: String,
    unicode_compressed: String,
    generation_time_ms: f64,
}

/// Prometheus metrics endpoint
async fn metrics(State(state): State<AppState>) -> String {
    let count = *state.request_count.read().await;
    
    format!(
        "# HELP ctas7_hash_requests_total Total hash generation requests\n\
         # TYPE ctas7_hash_requests_total counter\n\
         ctas7_hash_requests_total {}\n",
        count
    )
}

