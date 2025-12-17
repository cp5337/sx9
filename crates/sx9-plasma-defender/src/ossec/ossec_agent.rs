//! OSSEC Agent for Plasma Defender
//!
//! Consumes OSSEC alerts from NATS/JetStream and converts them
//! to threat entities in the ECS world.

use crate::atlas_integration::ThreatObservation;
use crate::ecs::components::OssecAlertComponent;
use crate::ecs::DefenderWorld;
use crate::ossec::alert_parser::{OssecAlertParser, ParsedAlert};
use crate::ring_bus::{OssecAlert, RingBusNode};
use anyhow::Result;
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

// =============================================================================
// OSSEC AGENT
// =============================================================================

/// OSSEC Agent - consumes alerts and creates threat entities
pub struct OssecAgent {
    /// Agent ID
    id: String,
    /// Alert parser
    parser: OssecAlertParser,
    /// ECS world for entity creation
    world: Arc<DefenderWorld>,
    /// Ring Bus for alert publishing
    ring_bus: Option<Arc<RingBusNode>>,
    /// ATLAS observation sender
    observation_tx: Option<mpsc::Sender<ThreatObservation>>,
    /// Stats
    stats: OssecAgentStats,
}

/// OSSEC agent statistics
#[derive(Debug, Clone, Default)]
pub struct OssecAgentStats {
    pub alerts_received: u64,
    pub alerts_parsed: u64,
    pub alerts_failed: u64,
    pub entities_created: u64,
    pub high_priority_alerts: u64,
}

impl OssecAgent {
    /// Create new OSSEC agent
    pub fn new(id: impl Into<String>, world: Arc<DefenderWorld>) -> Self {
        Self {
            id: id.into(),
            parser: OssecAlertParser::new(),
            world,
            ring_bus: None,
            observation_tx: None,
            stats: OssecAgentStats::default(),
        }
    }

    /// Connect to Ring Bus
    pub fn with_ring_bus(mut self, ring_bus: Arc<RingBusNode>) -> Self {
        self.ring_bus = Some(ring_bus);
        self
    }

    /// Connect to ATLAS for observations
    pub fn with_atlas(mut self, tx: mpsc::Sender<ThreatObservation>) -> Self {
        self.observation_tx = Some(tx);
        self
    }

    /// Process raw alert bytes
    pub async fn process_bytes(&mut self, data: &[u8]) -> Result<u64> {
        self.stats.alerts_received += 1;

        match self.parser.parse_bytes(data) {
            Ok(parsed) => self.process_parsed(parsed).await,
            Err(e) => {
                self.stats.alerts_failed += 1;
                tracing::warn!("Failed to parse OSSEC alert: {}", e);
                Err(e)
            }
        }
    }

    /// Process JSON alert string
    pub async fn process_json(&mut self, json: &str) -> Result<u64> {
        self.stats.alerts_received += 1;

        match self.parser.parse_json(json) {
            Ok(parsed) => self.process_parsed(parsed).await,
            Err(e) => {
                self.stats.alerts_failed += 1;
                tracing::warn!("Failed to parse OSSEC alert: {}", e);
                Err(e)
            }
        }
    }

    /// Process parsed alert
    async fn process_parsed(&mut self, parsed: ParsedAlert) -> Result<u64> {
        self.stats.alerts_parsed += 1;

        if parsed.high_priority {
            self.stats.high_priority_alerts += 1;
        }

        // Create threat entity in ECS
        let entity_id = self
            .world
            .add_threat_from_ossec(parsed.component.clone())
            .await?;

        self.stats.entities_created += 1;

        tracing::debug!(
            "Created entity {} from OSSEC alert {} (level: {}, technique: {:?})",
            entity_id,
            parsed.component.rule_id,
            parsed.component.level,
            parsed.component.mitre_technique
        );

        // Publish to Ring Bus if connected
        if let Some(ref ring_bus) = self.ring_bus {
            let alert = OssecAlert {
                rule_id: parsed.component.rule_id,
                level: parsed.component.level,
                description: parsed.component.description.clone(),
                mitre_technique: parsed.component.mitre_technique.clone(),
                mitre_tactic: parsed.component.mitre_tactic.clone(),
                src_ip: parsed.component.src_ip.clone(),
                dst_ip: parsed.component.dst_ip.clone(),
                raw_data: parsed.component.raw_data.clone(),
                timestamp_ns: parsed.component.timestamp,
            };

            if let Err(e) = ring_bus.publish_ossec_alert(&alert).await {
                tracing::warn!("Failed to publish OSSEC alert to Ring Bus: {}", e);
            }
        }

        // Send observation to ATLAS if connected
        if let Some(ref tx) = self.observation_tx {
            let observation = ThreatObservation {
                source: format!("ossec:{}", self.id),
                threat_hash: self.calculate_alert_hash(&parsed),
                entity_id,
                confidence: parsed.confidence,
                mitre_technique: parsed.component.mitre_technique.clone(),
                timestamp: Instant::now(),
            };

            if let Err(e) = tx.try_send(observation) {
                tracing::warn!("Failed to send observation to ATLAS: {}", e);
            }
        }

        Ok(entity_id)
    }

    /// Calculate hash for alert (for deduplication)
    fn calculate_alert_hash(&self, parsed: &ParsedAlert) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        parsed.component.rule_id.hash(&mut hasher);
        parsed.component.src_ip.hash(&mut hasher);
        parsed.component.dst_ip.hash(&mut hasher);
        parsed.component.mitre_technique.hash(&mut hasher);
        hasher.finish()
    }

    /// Subscribe to NATS subject and process alerts
    pub async fn subscribe_and_process(&mut self, nats_client: &async_nats::Client) -> Result<()> {
        let subject = "sx9.ossec.alert.>";
        let mut subscriber = nats_client.subscribe(subject.to_string()).await?;

        tracing::info!("OSSEC Agent {} subscribed to {}", self.id, subject);

        while let Some(msg) = subscriber.next().await {
            if let Err(e) = self.process_bytes(&msg.payload).await {
                tracing::error!("Error processing OSSEC alert: {}", e);
            }
        }

        Ok(())
    }

    /// Get agent stats
    pub fn stats(&self) -> &OssecAgentStats {
        &self.stats
    }

    /// Get agent ID
    pub fn id(&self) -> &str {
        &self.id
    }
}

// =============================================================================
// BATCH PROCESSING
// =============================================================================

/// Batch process OSSEC alerts from file
pub async fn process_alert_file(
    path: &std::path::Path,
    world: Arc<DefenderWorld>,
) -> Result<Vec<u64>> {
    let content = tokio::fs::read_to_string(path).await?;
    let mut agent = OssecAgent::new("file-processor", world);
    let mut entity_ids = Vec::new();

    // Handle both single alerts and NDJSON format
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        match agent.process_json(line).await {
            Ok(id) => entity_ids.push(id),
            Err(e) => tracing::warn!("Failed to process line: {}", e),
        }
    }

    tracing::info!(
        "Processed {} alerts from {}, created {} entities",
        agent.stats().alerts_received,
        path.display(),
        entity_ids.len()
    );

    Ok(entity_ids)
}

/// Process multiple alert files in directory
pub async fn process_alert_directory(
    dir: &std::path::Path,
    world: Arc<DefenderWorld>,
) -> Result<Vec<u64>> {
    let mut all_ids = Vec::new();

    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            match process_alert_file(&path, world.clone()).await {
                Ok(ids) => all_ids.extend(ids),
                Err(e) => tracing::warn!("Failed to process {}: {}", path.display(), e),
            }
        }
    }

    Ok(all_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ossec_agent() {
        let world = Arc::new(DefenderWorld::new());
        let mut agent = OssecAgent::new("test", world);

        let json = r#"{
            "rule_id": 5501,
            "level": 5,
            "description": "Login session opened."
        }"#;

        let entity_id = agent.process_json(json).await.unwrap();
        assert!(entity_id > 0);
        assert_eq!(agent.stats().alerts_parsed, 1);
        assert_eq!(agent.stats().entities_created, 1);
    }
}
