//! CTAS-7 Geospatial CDN Server
//!
//! HFT-optimized edge server for geospatial data with NATS backbone.
//! Serves tiles, orbital data, and Cesium assets.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use futures::StreamExt;
use tracing::{error, info, warn};

use sx9_cdn_geospatial::{
    GeoContentType, GeoHealthStatus, GeoNodeMetrics, GeospatialCdnNode, LayerConfig, MapboxConfig,
    OrbitalObject, TileCoord,
};

// ═══════════════════════════════════════════════════════════════════════════
// NATS + HFT Integration
// ═══════════════════════════════════════════════════════════════════════════

struct NatsGeoCdnNode {
    cdn: Arc<GeospatialCdnNode>,
    nats: Option<async_nats::Client>,
}

impl NatsGeoCdnNode {
    async fn new(node_id: String, nats_url: Option<&str>) -> anyhow::Result<Self> {
        let cdn = Arc::new(GeospatialCdnNode::new(node_id.clone()));

        let nats = if let Some(url) = nats_url {
            match async_nats::connect(url).await {
                Ok(client) => {
                    info!("Connected to NATS at {}", url);
                    Some(client)
                }
                Err(e) => {
                    warn!("NATS connection failed: {} - running standalone", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self { cdn, nats })
    }

    /// Publish orbital update to NATS (HFT path)
    async fn publish_orbital_update(&self, obj: &OrbitalObject) -> anyhow::Result<()> {
        if let Some(nats) = &self.nats {
            let subject = format!("ctas.geo.orbital.{}", obj.norad_id);
            let payload = serde_json::to_vec(obj)?;
            nats.publish(subject, payload.into()).await?;
        }
        Ok(())
    }

    /// Subscribe to orbital updates
    async fn subscribe_orbital(&self) -> anyhow::Result<()> {
        if let Some(nats) = &self.nats {
            let cdn = self.cdn.clone();
            let mut sub = nats.subscribe("ctas.geo.orbital.*").await?;

            tokio::spawn(async move {
                while let Some(msg) = sub.next().await {
                    if let Ok(obj) = serde_json::from_slice::<OrbitalObject>(&msg.payload) {
                        cdn.store_orbital(obj).await;
                    }
                }
            });

            info!("Subscribed to NATS orbital updates");
        }
        Ok(())
    }
}

type AppState = Arc<NatsGeoCdnNode>;

// ═══════════════════════════════════════════════════════════════════════════
// HTTP API
// ═══════════════════════════════════════════════════════════════════════════

async fn health_check(State(state): State<AppState>) -> Json<GeoHealthStatus> {
    Json(state.cdn.health_check().await)
}

async fn get_metrics(State(state): State<AppState>) -> Json<GeoNodeMetrics> {
    Json(state.cdn.get_metrics().await)
}

/// Get tile: /tiles/{layer}/{z}/{x}/{y}
async fn get_tile(
    State(state): State<AppState>,
    Path((layer, z, x, y)): Path<(String, u8, u32, u32)>,
) -> Response {
    let coord = TileCoord::new(z, x, y);

    match state.cdn.get_tile(&layer, &coord).await {
        Some(tile) => (
            StatusCode::OK,
            [
                ("content-type", tile.mime_type.as_str()),
                ("x-tile-hash", tile.hash.as_str()),
            ],
            tile.data,
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Get orbital object by NORAD ID
async fn get_orbital(
    State(state): State<AppState>,
    Path(norad_id): Path<u32>,
) -> Result<Json<OrbitalObject>, StatusCode> {
    state
        .cdn
        .get_orbital(norad_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Get all orbital objects
async fn get_all_orbital(State(state): State<AppState>) -> Json<Vec<OrbitalObject>> {
    Json(state.cdn.get_all_orbital())
}

/// Store orbital object (HFT endpoint)
async fn store_orbital(
    State(state): State<AppState>,
    Json(obj): Json<OrbitalObject>,
) -> Json<serde_json::Value> {
    // Publish to NATS for cluster sync
    if let Err(e) = state.publish_orbital_update(&obj).await {
        warn!("Failed to publish orbital to NATS: {}", e);
    }

    state.cdn.store_orbital(obj.clone()).await;
    Json(serde_json::json!({ "norad_id": obj.norad_id, "status": "stored" }))
}

/// Register layer
async fn register_layer(
    State(state): State<AppState>,
    Json(config): Json<LayerConfig>,
) -> Json<serde_json::Value> {
    state.cdn.register_layer(config.clone()).await;
    Json(serde_json::json!({ "layer": config.name, "status": "registered" }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Mapbox Integration Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/// Configure Mapbox access
async fn configure_mapbox(
    State(state): State<AppState>,
    Json(config): Json<MapboxConfig>,
) -> Json<serde_json::Value> {
    state.cdn.configure_mapbox(config.clone()).await;
    Json(serde_json::json!({ "status": "configured", "username": config.username }))
}

/// Get Mapbox vector tile (MVT)
async fn get_mapbox_tile(
    State(state): State<AppState>,
    Path((z, x, y)): Path<(u8, u32, u32)>,
) -> Response {
    let coord = TileCoord::new(z, x, y);

    match state.cdn.get_mapbox_tile(&coord).await {
        Some(tile) => (
            StatusCode::OK,
            [
                ("content-type", "application/x-protobuf"),
                ("x-tile-hash", tile.hash.as_str()),
            ],
            tile.data,
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Get Mapbox terrain-RGB elevation tile
async fn get_mapbox_terrain(
    State(state): State<AppState>,
    Path((z, x, y)): Path<(u8, u32, u32)>,
) -> Response {
    let coord = TileCoord::new(z, x, y);

    match state.cdn.get_mapbox_terrain(&coord).await {
        Some(tile) => (
            StatusCode::OK,
            [
                ("content-type", "image/png"),
                ("x-tile-hash", tile.hash.as_str()),
            ],
            tile.data,
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Get Mapbox style JSON
async fn get_mapbox_style(
    State(state): State<AppState>,
    Path(style_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.cdn.get_mapbox_style(&style_id).await {
        Some(style_json) => serde_json::from_str(&style_json)
            .map(Json)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get priority tiles for prefetching
async fn get_priority_tiles(State(state): State<AppState>) -> Json<Vec<(String, f64)>> {
    Json(state.cdn.get_priority_tiles(100))
}

/// Register hotspot for priority caching
async fn register_hotspot(
    State(state): State<AppState>,
    Json(hotspot): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let lat = hotspot["lat"].as_f64().unwrap_or(0.0);
    let lon = hotspot["lon"].as_f64().unwrap_or(0.0);
    let weight = hotspot["weight"].as_f64().unwrap_or(1.0);

    state.cdn.register_hotspot(lat, lon, weight).await;
    Json(serde_json::json!({ "status": "registered", "lat": lat, "lon": lon }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("ctas7_cdn_geospatial=info".parse()?),
        )
        .init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "18111".to_string())
        .parse()
        .unwrap_or(18111);

    let node_id =
        std::env::var("NODE_ID").unwrap_or_else(|_| format!("geo-{}", uuid::Uuid::new_v4()));

    let nats_url = std::env::var("NATS_URL").ok();

    info!("Starting CTAS-7 Geospatial CDN (HFT Mode)");
    info!("  Node ID: {}", node_id);
    info!("  Port: {}", port);
    info!("  NATS: {}", nats_url.as_deref().unwrap_or("disabled"));

    let node = Arc::new(NatsGeoCdnNode::new(node_id, nats_url.as_deref()).await?);
    node.subscribe_orbital().await?;

    let app = Router::new()
        // Health and metrics
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        // Standard tile endpoints
        .route("/tiles/:layer/:z/:x/:y", get(get_tile))
        // Mapbox integration (MVT, terrain-RGB, styles)
        .route("/mapbox/config", post(configure_mapbox))
        .route("/mapbox/tiles/:z/:x/:y", get(get_mapbox_tile))
        .route("/mapbox/terrain/:z/:x/:y", get(get_mapbox_terrain))
        .route("/mapbox/style/:style_id", get(get_mapbox_style))
        // Orbital data
        .route("/orbital", get(get_all_orbital))
        .route("/orbital/:norad_id", get(get_orbital))
        .route("/orbital", post(store_orbital))
        // Layer management
        .route("/layers", post(register_layer))
        // Smart caching (priority/hotspot)
        .route("/cache/priority", get(get_priority_tiles))
        .route("/cache/hotspot", post(register_hotspot))
        .with_state(node);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
