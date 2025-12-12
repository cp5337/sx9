//! SX9 ATLAS Daemon - Main Entry Point
//!
//! Cognitive engine with OODA loop, integrated with sx9-atlas-bus

use axum::{extract::State, response::Json, routing::get, Router};
use clap::Parser;
use std::sync::Arc;
use sx9_atlas_daemon::{AtlasConfig, AtlasDaemon, AtlasStatus};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

#[derive(Parser, Debug)]
#[command(name = "sx9-atlas-daemon")]
#[command(about = "SX9 ATLAS Daemon - Cognitive Engine with OODA Loop")]
#[command(version)]
struct Args {
    /// Port for HTTP API (default: 18500)
    #[arg(short, long, default_value_t = 18500)]
    port: u16,

    /// Cognitive tick rate in milliseconds (default: 1ms)
    #[arg(short, long, default_value_t = 1)]
    tick_rate_ms: u64,

    /// Maximum tick duration before zone violation (default: 1ms)
    #[arg(long, default_value_t = 1)]
    max_tick_duration_ms: u64,

    /// Enable telemetry
    #[arg(long, default_value_t = true)]
    telemetry: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("sx9_atlas_daemon=info".parse()?),
        )
        .init();

    let args = Args::parse();

    println!(
        r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║   ███████╗██╗  ██╗ █████╗        █████╗ ████████╗██╗      █████╗ ║
    ║   ██╔════╝╚██╗██╔╝██╔══██╗      ██╔══██╗╚══██╔══╝██║     ██╔══██╗║
    ║   ███████╗ ╚███╔╝ ╚██████║█████╗███████║   ██║   ██║     ███████║║
    ║   ╚════██║ ██╔██╗  ╚═══██║╚════╝██╔══██║   ██║   ██║     ██╔══██║║
    ║   ███████║██╔╝ ██╗ █████╔╝      ██║  ██║   ██║   ███████╗██║  ██║║
    ║   ╚══════╝╚═╝  ╚═╝ ╚════╝       ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝║
    ║                                                               ║
    ║   ATLAS Daemon - Cognitive Engine with OODA Loop             ║
    ║   Zone B - {}ms tick rate                                     ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
    "#,
        args.tick_rate_ms
    );

    // Create daemon configuration
    let config = AtlasConfig {
        port: args.port,
        tick_rate_ms: args.tick_rate_ms,
        max_tick_duration_ms: args.max_tick_duration_ms,
        telemetry_enabled: args.telemetry,
        ..Default::default()
    };

    // Create daemon
    let daemon = AtlasDaemon::new(config.clone());

    // Create shared state for HTTP server
    let daemon_state = Arc::new(RwLock::new(daemon));

    // Build HTTP router for health/metrics
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(status_handler))
        .route("/metrics", get(metrics_handler))
        .layer(CorsLayer::new().allow_origin(axum::http::HeaderValue::from_static("*")))
        .with_state(daemon_state.clone());

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    tracing::info!(
        "🚀 SX9 ATLAS Daemon HTTP API listening on port {}",
        config.port
    );

    // Start cognitive tick loop in background
    let daemon_for_tick = daemon_state.clone();
    tokio::spawn(async move {
        let mut daemon = daemon_for_tick.write().await;
        daemon.start_cognitive_tick().await;
    });

    // Start HTTP server
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler(State(daemon): State<Arc<RwLock<AtlasDaemon>>>) -> Json<serde_json::Value> {
    let daemon = daemon.read().await;
    let status = AtlasStatus::from_daemon(&daemon);
    Json(serde_json::json!({
        "status": "ok",
        "version": status.version,
        "tick_count": status.tick_count,
        "zone": status.zone,
    }))
}

async fn status_handler(State(daemon): State<Arc<RwLock<AtlasDaemon>>>) -> Json<AtlasStatus> {
    let daemon = daemon.read().await;
    Json(AtlasStatus::from_daemon(&daemon))
}

async fn metrics_handler(
    State(daemon): State<Arc<RwLock<AtlasDaemon>>>,
) -> Json<serde_json::Value> {
    let daemon = daemon.read().await;
    let status = AtlasStatus::from_daemon(&daemon);

    Json(serde_json::json!({
        "tick_count": status.tick_count,
        "tick_rate_ms": status.tick_rate_ms,
        "current_phase": format!("{:?}", status.current_phase),
        "vertical_level": format!("{:?}", status.vertical_level),
        "plasma": {
            "delta_angle": status.plasma.delta_angle,
            "entropy": status.plasma.entropy,
            "excited": status.plasma.excited,
            "sdt_state": status.plasma.sdt_state,
            "ring_strength": status.plasma.last_ring_strength,
        },
    }))
}
