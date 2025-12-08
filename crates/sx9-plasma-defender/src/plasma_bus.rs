//! Plasma Telemetry & ANN Bridge
//!
//! NATS integration for plasma telemetry and ANN event streaming

use async_nats::Client;
use tokio::sync::broadcast;
use std::sync::Arc;
use std::time::Instant;
use serde_json::json;
use tracing::{info, warn};

/// PlasmaBus - NATS telemetry bridge with ANN event channel
pub struct PlasmaBus {
    pub nats: Arc<Client>,
    pub ann_tx: broadcast::Sender<PlasmaEvent>,
}

#[derive(Clone, Debug)]
pub struct PlasmaEvent {
    pub timestamp: Instant,
    pub metric: String,
    pub value: f64,
}

impl PlasmaBus {
    /// Create new PlasmaBus with NATS connection
    pub async fn new(nats_url: &str) -> anyhow::Result<Self> {
        let nats = Arc::new(async_nats::connect(nats_url).await?);
        let (ann_tx, _) = broadcast::channel(2048);
        
        info!("✅ PlasmaBus connected to NATS: {}", nats_url);
        
        Ok(Self { nats, ann_tx })
    }

    /// Emit telemetry metric to NATS and ANN channel
    pub async fn emit(&self, metric: &str, value: f64) -> anyhow::Result<()> {
        let payload = json!({
            "metric": metric,
            "value": value,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        // Publish to NATS
        self.nats
            .publish("sx9.stream.ops.plasma.telemetry", serde_json::to_vec(&payload)?.into())
            .await?;

        // Broadcast to ANN channel
        let _ = self.ann_tx.send(PlasmaEvent {
            timestamp: Instant::now(),
            metric: metric.to_string(),
            value,
        });

        Ok(())
    }
    
    /// Get ANN event receiver
    pub fn subscribe_ann(&self) -> broadcast::Receiver<PlasmaEvent> {
        self.ann_tx.subscribe()
    }
}



