//! Optimized Axum Server with Crystal/SDT Integration
//!
//! Uses compression, timeout, body limit, and trace layers for performance

use crate::config::DefenderConfig;
use crate::plasma_bus::PlasmaBus;
use axum::{extract::State, response::Json, routing::get, Router};
use std::sync::Arc;
use std::time::Duration;
use sx9_atlas_bus::PlasmaState;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub struct PlasmaDefenderServer {
    router: Router,
    health_endpoint: String,
    metrics_endpoint: String,
    plasma: Arc<PlasmaState>,
    plasma_bus: Arc<PlasmaBus>,
}

impl PlasmaDefenderServer {
    pub async fn new(
        config: &DefenderConfig,
        plasma: Arc<PlasmaState>,
        plasma_bus: Arc<PlasmaBus>,
    ) -> anyhow::Result<Self> {
        let router = Router::new()
            .route("/health", get(health_handler))
            .route("/metrics", get(metrics_handler))
            .route("/sdt/state", get(sdt_state_handler))
            .route("/crystal/resonance", get(crystal_resonance_handler))
            .route("/crystal/family", get(crystal_family_handler))
            .layer(CompressionLayer::new())
            .layer(TimeoutLayer::new(Duration::from_secs(
                config.request_timeout_secs,
            )))
            .layer(RequestBodyLimitLayer::new(config.body_size_limit))
            .layer(TraceLayer::new_for_http())
            .with_state((plasma.clone(), plasma_bus.clone()));

        Ok(Self {
            router,
            health_endpoint: config.health_endpoint.clone(),
            metrics_endpoint: config.metrics_endpoint.clone(),
            plasma,
            plasma_bus,
        })
    }

    pub async fn start(&self, addr: &str) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("🚀 Plasma Defender server listening on {}", addr);
        axum::serve(listener, self.router.clone()).await?;
        Ok(())
    }
}

async fn health_handler(
    State((plasma, _)): State<(Arc<PlasmaState>, Arc<PlasmaBus>)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "sdt_state": format!("{:?}", plasma.sdt_state()),
        "excited": plasma.is_excited(),
    }))
}

async fn metrics_handler(
    State((plasma, _)): State<(Arc<PlasmaState>, Arc<PlasmaBus>)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "delta_angle": plasma.delta_angle_raw(),
        "entropy": plasma.entropy(),
        "excited": plasma.is_excited(),
        "ring_strength": plasma.last_ring_strength(),
        "trigger_count": plasma.trigger_count(),
        "supersession_count": plasma.supersession_count(),
    }))
}

async fn sdt_state_handler(
    State((plasma, _)): State<(Arc<PlasmaState>, Arc<PlasmaBus>)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "sdt_state": format!("{:?}", plasma.sdt_state()),
        "delta_angle": plasma.delta_angle_raw(),
        "entropy": plasma.entropy(),
        "excited": plasma.is_excited(),
        "ring_strength": plasma.last_ring_strength(),
        "trigger_count": plasma.trigger_count(),
    }))
}

async fn crystal_resonance_handler(
    State((plasma, _)): State<(Arc<PlasmaState>, Arc<PlasmaBus>)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ring_strength": plasma.last_ring_strength(),
        "excited": plasma.is_excited(),
        "delta_angle": plasma.delta_angle_raw(),
        "entropy": plasma.entropy(),
    }))
}

async fn crystal_family_handler(
    State((_, _)): State<(Arc<PlasmaState>, Arc<PlasmaBus>)>,
) -> Json<serde_json::Value> {
    // Return current crystal family configuration
    Json(serde_json::json!({
        "crystal_family": "GroundStation", // From config
    }))
}
