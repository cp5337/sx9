//! CTAS-7 Threat Intelligence CDN Server
//!
//! Edge server for threat intel distribution with NATS backbone.
//! Serves DSL playbooks, Wazuh rules, MITRE mappings to consumers.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use futures::StreamExt;
use serde::Deserialize;
use tracing::{error, info, warn};

use sx9_cdn_threat_intel::{
    ContentType, HealthStatus, NodeMetrics, ThreatIntelCdnNode, ThreatIntelContent, WazuhRule,
};

// ═══════════════════════════════════════════════════════════════════════════
// NATS Integration
// ═══════════════════════════════════════════════════════════════════════════

/// NATS-connected CDN node
struct NatsCdnNode {
    cdn: Arc<ThreatIntelCdnNode>,
    nats: Option<async_nats::Client>,
}

impl NatsCdnNode {
    async fn new(node_id: String, nats_url: Option<&str>) -> anyhow::Result<Self> {
        let cdn = Arc::new(ThreatIntelCdnNode::new(node_id.clone()));

        let nats = if let Some(url) = nats_url {
            match async_nats::connect(url).await {
                Ok(client) => {
                    info!("Connected to NATS at {}", url);
                    Some(client)
                }
                Err(e) => {
                    warn!(
                        "Failed to connect to NATS: {} - running without backbone",
                        e
                    );
                    None
                }
            }
        } else {
            None
        };

        Ok(Self { cdn, nats })
    }

    /// Publish content update to NATS
    async fn publish_update(&self, content: &ThreatIntelContent) -> anyhow::Result<()> {
        if let Some(nats) = &self.nats {
            let subject = format!("ctas.threat-intel.{:?}.update", content.content_type);
            let payload = serde_json::to_vec(content)?;
            nats.publish(subject, payload.into()).await?;
            info!("Published update for {} to NATS", content.sch);
        }
        Ok(())
    }

    /// Subscribe to content updates
    async fn subscribe_updates(&self) -> anyhow::Result<()> {
        if let Some(nats) = &self.nats {
            let cdn = self.cdn.clone();

            // Subscribe to all threat-intel updates
            let mut sub = nats.subscribe("ctas.threat-intel.*.update").await?;

            tokio::spawn(async move {
                while let Some(msg) = sub.next().await {
                    match serde_json::from_slice::<ThreatIntelContent>(&msg.payload) {
                        Ok(content) => {
                            if let Err(e) = cdn.store(content).await {
                                error!("Failed to store NATS update: {}", e);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse NATS message: {}", e);
                        }
                    }
                }
            });

            info!("Subscribed to NATS threat-intel updates");
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP API
// ═══════════════════════════════════════════════════════════════════════════

type AppState = Arc<NatsCdnNode>;

#[derive(Deserialize)]
struct QueryParams {
    content_type: Option<String>,
    mitre_id: Option<String>,
    source: Option<String>,
    limit: Option<usize>,
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthStatus> {
    Json(state.cdn.health_check().await)
}

/// Get metrics
async fn get_metrics(State(state): State<AppState>) -> Json<NodeMetrics> {
    Json(state.cdn.get_metrics().await)
}

/// Get content by SCH
async fn get_content(
    State(state): State<AppState>,
    Path(sch): Path<String>,
) -> Result<Json<ThreatIntelContent>, StatusCode> {
    state
        .cdn
        .get(&sch)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Store new content
async fn store_content(
    State(state): State<AppState>,
    Json(content): Json<ThreatIntelContent>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.cdn.store(content.clone()).await {
        Ok(sch) => {
            // Publish to NATS backbone
            if let Err(e) = state.publish_update(&content).await {
                warn!("Failed to publish to NATS: {}", e);
            }
            Ok(Json(serde_json::json!({ "sch": sch, "status": "stored" })))
        }
        Err(e) => {
            error!("Failed to store content: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Query content by type
async fn query_by_type(
    State(state): State<AppState>,
    Path(type_name): Path<String>,
) -> Json<Vec<ThreatIntelContent>> {
    let content_type = match type_name.as_str() {
        "dsl_playbook" => ContentType::DslPlaybook,
        "mitre_attack" => ContentType::MitreAttack,
        "mitre_car" => ContentType::MitreCar,
        "wazuh_rule" => ContentType::WazuhRule,
        "sigma_rule" => ContentType::SigmaRule,
        "yara_rule" => ContentType::YaraRule,
        "ioc" => ContentType::Ioc,
        "osint_feed" => ContentType::OsintFeed,
        "threat_actor" => ContentType::ThreatActor,
        "campaign" => ContentType::Campaign,
        _ => return Json(vec![]),
    };

    Json(state.cdn.query_by_type(content_type).await)
}

/// Query by MITRE technique
async fn query_by_mitre(
    State(state): State<AppState>,
    Path(technique_id): Path<String>,
) -> Json<Vec<ThreatIntelContent>> {
    Json(state.cdn.query_by_mitre(&technique_id).await)
}

/// Get all Wazuh rules
async fn get_wazuh_rules(State(state): State<AppState>) -> Json<Vec<WazuhRule>> {
    Json(state.cdn.get_all_wazuh_rules())
}

/// Get Wazuh rule by ID
async fn get_wazuh_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<u32>,
) -> Result<Json<WazuhRule>, StatusCode> {
    state
        .cdn
        .get_wazuh_rule(rule_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Store Wazuh rule
async fn store_wazuh_rule(
    State(state): State<AppState>,
    Json(rule): Json<WazuhRule>,
) -> Json<serde_json::Value> {
    state.cdn.store_wazuh_rule(rule.clone()).await;
    Json(serde_json::json!({ "rule_id": rule.rule_id, "status": "stored" }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("ctas7_cdn_threat_intel=info".parse()?),
        )
        .init();

    // Parse configuration
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "18110".to_string())
        .parse()
        .unwrap_or(18110);

    let node_id = std::env::var("NODE_ID")
        .unwrap_or_else(|_| format!("threat-intel-{}", uuid::Uuid::new_v4()));

    let nats_url = std::env::var("NATS_URL").ok();

    info!("Starting CTAS-7 Threat Intel CDN");
    info!("  Node ID: {}", node_id);
    info!("  Port: {}", port);
    info!("  NATS: {}", nats_url.as_deref().unwrap_or("disabled"));

    // Create NATS-connected node
    let node = Arc::new(NatsCdnNode::new(node_id, nats_url.as_deref()).await?);

    // Subscribe to NATS updates
    node.subscribe_updates().await?;

    // Build router
    let app = Router::new()
        // Health and metrics
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        // Content CRUD
        .route("/content/:sch", get(get_content))
        .route("/content", post(store_content))
        // Query endpoints
        .route("/query/type/:type_name", get(query_by_type))
        .route("/query/mitre/:technique_id", get(query_by_mitre))
        // Wazuh integration
        .route("/wazuh/rules", get(get_wazuh_rules))
        .route("/wazuh/rules/:rule_id", get(get_wazuh_rule))
        .route("/wazuh/rules", post(store_wazuh_rule))
        .with_state(node);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
