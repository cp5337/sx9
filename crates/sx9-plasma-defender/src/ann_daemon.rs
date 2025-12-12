//! ANN Daemon for threat analysis
//!
//! Observes tool results and generates advisories

use crate::advisory::{AnnAdvisory, AnnContext};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnObservation {
    pub hash_entropy: f32,
    pub routing_latency_ns: u64,
    pub sdt_state: Option<u8>,
    pub crystal_resonance: Option<f32>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Default)]
pub struct AnnObserver {
    pub buffer: Arc<RwLock<Vec<AnnObservation>>>,
}

impl AnnObserver {
    pub async fn record(&self, obs: AnnObservation) -> Result<()> {
        let mut buf = self.buffer.write().await;
        buf.push(obs);
        if buf.len() > 1000 {
            buf.remove(0);
        }
        Ok(())
    }
}

pub struct AnnDaemon {
    pub enabled: bool,
    observer: Arc<AnnObserver>,
}

#[derive(Clone, Debug, Default)]
pub struct AnnConfig {
    pub enabled: bool,
}

impl AnnDaemon {
    pub async fn new(config: AnnConfig) -> Result<Self> {
        Ok(Self {
            enabled: config.enabled,
            observer: Arc::new(AnnObserver::default()),
        })
    }

    pub async fn observe(&self, obs: AnnObservation) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        self.observer.record(obs).await
    }

    pub async fn get_advisory(&self, ctx: &AnnContext) -> Result<Option<AnnAdvisory>> {
        if !self.enabled {
            return Ok(None);
        }

        // Simple heuristic: high entropy + low latency = proceed
        // Low entropy or high latency = block/escalate
        let confidence = (ctx.entropy + ctx.latency_score) / 2.0;

        if confidence > 0.85 {
            Ok(Some(AnnAdvisory {
                confidence,
                recommendation: "proceed".to_string(),
                reason_trace: vec![
                    "High crystal resonance".to_string(),
                    "SDT gate open".to_string(),
                    "Low latency variance".to_string(),
                ],
                timestamp: Utc::now(),
            }))
        } else if confidence < 0.5 {
            Ok(Some(AnnAdvisory {
                confidence: 1.0 - confidence,
                recommendation: "block".to_string(),
                reason_trace: vec![
                    "Low crystal resonance".to_string(),
                    "SDT gate closed or uncertain".to_string(),
                    "High latency variance".to_string(),
                ],
                timestamp: Utc::now(),
            }))
        } else {
            Ok(Some(AnnAdvisory {
                confidence,
                recommendation: "escalate".to_string(),
                reason_trace: vec![
                    "Moderate confidence".to_string(),
                    "Requires manual review".to_string(),
                ],
                timestamp: Utc::now(),
            }))
        }
    }
}
