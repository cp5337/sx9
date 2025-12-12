//! CTAS-7 Hashing Engine Microservice v7.3.1
//!
//! Pure microservice for trivariate hash generation
//! - gRPC on port 50051
//! - REST on port 8002
//! - Prometheus metrics on /metrics
//!
//! **Ground Truth**: Murmur3 trivariate hashing ONLY (NO Blake3)

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sx9_foundation_core::{
    ContextFrame,
    ExecEnv,
    ExecState,
    TrivariateHashEngine, // V7.3.1 (Correct spelling)
};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;

/// Service state
#[derive(Clone)]
struct AppState {
    hash_engine: Arc<RwLock<TrivariateHashEngine>>,
    request_count: Arc<RwLock<u64>>,
}

/// Hash generation request
#[derive(Debug, Deserialize)]
struct HashRequest {
    content: String,
    context: String,        // Treat as domain/tag string
    primitive_type: String, // Treat as exec_class
    #[serde(default)]
    compress_unicode: bool,
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
    batch_context: Option<String>,
}

/// Individual item in batch
#[derive(Debug, Deserialize)]
struct BatchItem {
    id: String,
    content: String,
    context: String,
    primitive_type: String,
}

/// Batch hash response
#[derive(Debug, Serialize)]
struct BatchHashResponse {
    batch_hash: String,
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
        hash_engine: Arc::new(RwLock::new(TrivariateHashEngine::new())),
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

// --- Helper: Map String Context to Strict ContextFrame ---
fn map_context_to_frame(context_str: &str) -> ContextFrame {
    let normalized = context_str.to_lowercase();

    // Heuristic ExecEnv detection
    let exec_env = if normalized.contains("wasm") {
        ExecEnv::Wasm
    } else if normalized.contains("container") || normalized.contains("docker") {
        ExecEnv::Container
    } else if normalized.contains("kernel") {
        ExecEnv::Kernel
    } else {
        ExecEnv::Native // Default fallback
    };

    // Heuristic State detection
    let state = if normalized.contains("cold") {
        ExecState::Cold
    } else if normalized.contains("warm") {
        ExecState::Warm
    } else {
        ExecState::Hot // Assume active execution
    };

    // Agent ID from simple hash of context string (modulo 65536)
    let agent_id = normalized
        .chars()
        .fold(0u16, |acc, c| acc.wrapping_add(c as u16));

    ContextFrame::new(exec_env, agent_id, state)
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

    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }

    let engine = state.hash_engine.read().await;

    // Map inputs to v7.3.1 requirements
    let context_frame = map_context_to_frame(&request.context);

    // Generate canonical trivariate
    let trivariate = engine.generate_trivariate(
        &request.content,
        "service",               // node_type
        &request.context,        // domain (heuristic: usage of context string as domain)
        &request.primitive_type, // exec_class
        &context_frame,
    );

    let hash_str = trivariate.to_48char_hash();

    // Optional Unicode compression
    let unicode_compressed = if request.compress_unicode {
        // Re-generate CUID slots for strict unicode alignment if needed,
        // or just use slot method if available.
        // For now, we manually reconstruct slots since engine exposes generate_cuid which returns string.
        // But TrivariateHash struct doesn't have the slots.
        // However, ContextFrame -> CuidSlots -> to_unicode_runes works.
        // We will do a best-effort compression matching the internal logic.
        let slots = sx9_foundation_core::CuidSlots::from(&context_frame);
        Some(slots.to_unicode_runes())
    } else {
        None
    };

    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;

    info!(
        "✅ Generated hash in {:.3}ms: {}... (unicode: {})",
        generation_time_ms,
        &hash_str[0..12],
        unicode_compressed.is_some()
    );

    Ok(Json(HashResponse {
        trivariate_hash: hash_str,
        sch: trivariate.sch,
        cuid: trivariate.cuid,
        uuid: trivariate.uuid,
        unicode_compressed,
        generation_time_ms,
    }))
}

/// Generate batch of hashes
async fn generate_hash_batch(
    State(state): State<AppState>,
    Json(request): Json<BatchHashRequest>,
) -> Result<Json<BatchHashResponse>, StatusCode> {
    let start = std::time::Instant::now();

    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }

    let engine = state.hash_engine.read().await;

    let mut batch_items = Vec::new();
    let mut all_hashes = String::new();

    for (index, item) in request.items.iter().enumerate() {
        let context_str = if request.preserve_context {
            format!(
                "{}:{}:{}",
                request.batch_context.as_deref().unwrap_or("batch"),
                index,
                item.context
            )
        } else {
            item.context.clone()
        };

        let context_frame = map_context_to_frame(&context_str);

        let trivariate = engine.generate_trivariate(
            &item.content,
            "service",
            &context_str,
            &item.primitive_type,
            &context_frame,
        );

        let hash_str = trivariate.to_48char_hash();

        let unicode_compressed = if request.compress_unicode {
            let slots = sx9_foundation_core::CuidSlots::from(&context_frame);
            Some(slots.to_unicode_runes())
        } else {
            None
        };

        all_hashes.push_str(&hash_str);

        batch_items.push(BatchHashItem {
            id: item.id.clone(),
            trivariate_hash: hash_str.clone(),
            sch: trivariate.sch,
            cuid: trivariate.cuid,
            uuid: trivariate.uuid,
            unicode_compressed,
            batch_index: index,
        });
    }

    // Batch summary hash
    let batch_ctx_str = request.batch_context.as_deref().unwrap_or("batch");
    let batch_frame = map_context_to_frame(batch_ctx_str);
    let batch_hash_obj = engine.generate_trivariate(
        &all_hashes,
        "batch",
        batch_ctx_str,
        "BATCH_HASH",
        &batch_frame,
    );
    let batch_hash = batch_hash_obj.to_48char_hash();

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
async fn hash_stream_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| hash_stream(socket, state))
}

async fn hash_stream(stream: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = stream.split();
    info!("🌊 WebSocket stream connected");

    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if let Ok(text) = msg.to_text() {
                if let Ok(request) = serde_json::from_str::<StreamHashRequest>(text) {
                    {
                        let mut count = state.request_count.write().await;
                        *count += 1;
                    }

                    let engine = state.hash_engine.read().await;
                    let start = std::time::Instant::now();

                    let context_frame = map_context_to_frame(&request.context);
                    let trivariate = engine.generate_trivariate(
                        &request.content,
                        "stream",
                        &request.context,
                        &request.primitive_type,
                        &context_frame,
                    );
                    let hash_str = trivariate.to_48char_hash();

                    let unicode_compressed = if request.compress_unicode {
                        let slots = sx9_foundation_core::CuidSlots::from(&context_frame);
                        Some(slots.to_unicode_runes())
                    } else {
                        None
                    };

                    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;

                    let response = StreamHashResponse {
                        stream_id: request.stream_id,
                        trivariate_hash: hash_str.clone(),
                        sch: trivariate.sch,
                        cuid: trivariate.cuid,
                        uuid: trivariate.uuid,
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

#[derive(Debug, Deserialize)]
struct StreamHashRequest {
    stream_id: String,
    content: String,
    context: String,
    primitive_type: String,
    #[serde(default)]
    compress_unicode: bool,
}

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

/// Generate USIM
async fn generate_usim(
    State(state): State<AppState>,
    Json(request): Json<UsimRequest>,
) -> Result<Json<UsimResponse>, StatusCode> {
    let start = std::time::Instant::now();

    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }

    let engine = state.hash_engine.read().await;

    // USIM hashing
    let context_frame = map_context_to_frame(&request.domain);
    let usim_obj = engine.generate_trivariate(
        &request.file_path,
        "USIM",
        &request.domain,
        "usim_node",
        &context_frame,
    );
    let usim_hash = usim_obj.to_48char_hash();

    // Integrity Hashing (Murmur3)
    let integrity_obj = engine.generate_trivariate(
        &request.content,
        "integrity",
        "checksum",
        "integrity_check",
        &context_frame,
    );
    let integrity_hash = integrity_obj.to_48char_hash();

    let slots = sx9_foundation_core::CuidSlots::from(&context_frame);
    let unicode_compressed = slots.to_unicode_runes();

    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;

    info!(
        "✅ Generated USIM in {:.3}ms: {}... (unicode: {})",
        generation_time_ms,
        &usim_hash[0..12],
        unicode_compressed
    );

    Ok(Json(UsimResponse {
        usim_hash: usim_hash.clone(),
        integrity_hash,
        unicode_compressed,
        sch: usim_obj.sch,
        cuid: usim_obj.cuid,
        uuid: usim_obj.uuid,
        generation_time_ms,
    }))
}

/// Generate USIM header
async fn generate_usim_header(
    State(state): State<AppState>,
    Json(request): Json<UsimHeaderRequest>,
) -> Result<Json<UsimHeaderResponse>, StatusCode> {
    let start = std::time::Instant::now();

    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }

    let engine = state.hash_engine.read().await;

    let context_frame = map_context_to_frame(&request.domain);

    let usim_obj = engine.generate_trivariate(
        &request.file_path,
        "USIM",
        &request.domain,
        "usim_node",
        &context_frame,
    );
    let usim_hash = usim_obj.to_48char_hash();

    let integrity_obj = engine.generate_trivariate(
        &request.content,
        "integrity",
        "checksum",
        "integrity_check",
        &context_frame,
    );
    let integrity_hash = integrity_obj.to_48char_hash();

    let slots = sx9_foundation_core::CuidSlots::from(&context_frame);
    let unicode_compressed = slots.to_unicode_runes();

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
            &integrity_hash[0..32],
            unicode_compressed,
            request.domain,
            request.description,
            request.parent_node.unwrap_or_else(|| "ROOT".to_string()),
            request.dependencies.join(", "),
            request.language.unwrap_or_else(|| "Unknown".to_string()),
            request.file_type.unwrap_or_else(|| "Unknown".to_string()),
            request.complexity.unwrap_or(0.0),
            request
                .ttl_policy
                .unwrap_or_else(|| "Persistent".to_string()),
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
            &integrity_hash[0..16],
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
            &integrity_hash[0..16],
            request.domain,
        ),

        UsimHeaderFormat::Minimal => format!(
            r#"USIM: {} | Unicode: {} | Integrity: {}..."#,
            usim_hash,
            unicode_compressed,
            &integrity_hash[0..16],
        ),
    };

    let generation_time_ms = start.elapsed().as_micros() as f64 / 1000.0;

    info!(
        "✅ Generated USIM header in {:.3}ms for {}",
        generation_time_ms, request.file_path
    );

    Ok(Json(UsimHeaderResponse {
        header,
        usim_hash,
        integrity_hash,
        unicode_compressed,
        generation_time_ms,
    }))
}

#[derive(Debug, Deserialize)]
struct UsimRequest {
    file_path: String,
    content: String,
    domain: String,
}

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

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum UsimHeaderFormat {
    #[default]
    Full,
    Footer,
    Index,
    Minimal,
}

#[derive(Debug, Serialize)]
struct UsimHeaderResponse {
    header: String,
    usim_hash: String,
    integrity_hash: String,
    unicode_compressed: String,
    generation_time_ms: f64,
}

async fn metrics(State(state): State<AppState>) -> String {
    let count = *state.request_count.read().await;

    format!(
        "# HELP ctas7_hash_requests_total Total hash generation requests\n\
         # TYPE ctas7_hash_requests_total counter\n\
         ctas7_hash_requests_total {}\n",
        count
    )
}
