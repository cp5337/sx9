//! Ring Bus Integration for Plasma Defender
//!
//! RFC-9301: Thyristor, Crystal, and Ring Bus Architecture
//!
//! Provides:
//! - JetStream persistence for threat events and tool outputs
//! - Ring Bus inter-node communication
//! - Defender-specific NATS subjects and streams

use anyhow::Result;
use async_nats::jetstream::{self, consumer::PullConsumer, stream::Stream};
use async_nats::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// Ring Bus Node ID for Plasma Defender
/// Per RFC-9301, each node has a unique ID in the ring topology
pub const DEFENDER_NODE_ID: u8 = 9; // Node 9 = Forge/Defender

// =============================================================================
// JETSTREAM STREAMS
// =============================================================================

/// JetStream streams for Plasma Defender
pub mod streams {
    /// Threat events (persistent for correlation)
    pub const THREAT_EVENTS: &str = "SX9_DEFENDER_THREATS";
    /// Tool outputs (persistent for re-processing)
    pub const TOOL_OUTPUTS: &str = "SX9_DEFENDER_TOOL_OUTPUTS";
    /// OSSEC alerts
    pub const OSSEC_ALERTS: &str = "SX9_DEFENDER_OSSEC";
    /// EEI queries and responses
    pub const EEI_QUERIES: &str = "SX9_DEFENDER_EEI";
}

// =============================================================================
// NATS SUBJECTS
// =============================================================================

/// NATS subjects for Plasma Defender
pub mod subjects {
    /// Threat detected event
    pub const THREAT_DETECTED: &str = "sx9.defender.threat.detected";
    /// Threat blocked event
    pub const THREAT_BLOCKED: &str = "sx9.defender.threat.blocked";
    /// Tool result received
    pub const TOOL_RESULT: &str = "sx9.defender.tool.result";
    /// OSSEC alert received
    pub const OSSEC_ALERT: &str = "sx9.defender.ossec.alert";
    /// EEI query
    pub const EEI_QUERY: &str = "sx9.defender.eei.query";
    /// EEI response
    pub const EEI_RESPONSE: &str = "sx9.defender.eei.response";
    /// Ring Bus inter-node communication
    pub const RING_BUS: &str = "sx9.ring";
}

// =============================================================================
// RING BUS NODE
// =============================================================================

/// Ring Bus Node for Plasma Defender
pub struct RingBusNode {
    node_id: u8,
    client: Client,
    jetstream: jetstream::Context,
}

impl RingBusNode {
    /// Create and connect a new Ring Bus node
    pub async fn new(nats_url: &str) -> Result<Self> {
        let client = async_nats::connect(nats_url).await?;
        let jetstream = jetstream::new(client.clone());

        let node = Self {
            node_id: DEFENDER_NODE_ID,
            client,
            jetstream,
        };

        node.init_streams().await?;

        tracing::info!(
            "🔗 Ring Bus node {} connected to {}",
            DEFENDER_NODE_ID,
            nats_url
        );

        Ok(node)
    }

    /// Initialize JetStream streams for Defender
    async fn init_streams(&self) -> Result<()> {
        // Threat events stream (persistent for correlation)
        let _ = self
            .jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::THREAT_EVENTS.to_string(),
                subjects: vec![
                    format!("{}.*", subjects::THREAT_DETECTED),
                    format!("{}.*", subjects::THREAT_BLOCKED),
                ],
                max_messages: 100_000,
                max_age: Duration::from_secs(86400 * 7), // 7 days retention
                storage: jetstream::stream::StorageType::File,
                ..Default::default()
            })
            .await?;

        // Tool outputs stream (persistent for re-processing)
        let _ = self
            .jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::TOOL_OUTPUTS.to_string(),
                subjects: vec![format!("{}.*", subjects::TOOL_RESULT)],
                max_messages: 500_000,
                max_age: Duration::from_secs(86400 * 30), // 30 days retention
                storage: jetstream::stream::StorageType::File,
                ..Default::default()
            })
            .await?;

        // OSSEC alerts stream
        let _ = self
            .jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::OSSEC_ALERTS.to_string(),
                subjects: vec![format!("{}.*", subjects::OSSEC_ALERT)],
                max_messages: 200_000,
                max_age: Duration::from_secs(86400 * 14), // 14 days retention
                storage: jetstream::stream::StorageType::File,
                ..Default::default()
            })
            .await?;

        // EEI queries stream (short retention for request/response)
        let _ = self
            .jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::EEI_QUERIES.to_string(),
                subjects: vec![
                    format!("{}.*", subjects::EEI_QUERY),
                    format!("{}.*", subjects::EEI_RESPONSE),
                ],
                max_messages: 50_000,
                max_age: Duration::from_secs(3600), // 1 hour retention
                storage: jetstream::stream::StorageType::Memory,
                ..Default::default()
            })
            .await?;

        tracing::info!("✅ Defender JetStream streams initialized");
        Ok(())
    }

    // =========================================================================
    // THREAT EVENT PUBLISHING
    // =========================================================================

    /// Publish threat detected event
    pub async fn publish_threat_detected(&self, event: &ThreatEvent) -> Result<()> {
        let subject = format!("{}.{}", subjects::THREAT_DETECTED, event.threat_hash);
        let payload = serde_json::to_vec(event)?;

        self.jetstream
            .publish(subject, payload.into())
            .await?
            .await?;

        tracing::debug!("Published threat detected: {}", event.threat_hash);
        Ok(())
    }

    /// Publish threat blocked event
    pub async fn publish_threat_blocked(&self, event: &ThreatEvent) -> Result<()> {
        let subject = format!("{}.{}", subjects::THREAT_BLOCKED, event.threat_hash);
        let payload = serde_json::to_vec(event)?;

        self.jetstream
            .publish(subject, payload.into())
            .await?
            .await?;

        tracing::info!("🛡️ Published threat blocked: {}", event.threat_hash);
        Ok(())
    }

    // =========================================================================
    // TOOL OUTPUT PUBLISHING
    // =========================================================================

    /// Publish tool output to JetStream for re-processing
    pub async fn publish_tool_output(&self, output: &ToolOutput) -> Result<()> {
        let subject = format!("{}.{}", subjects::TOOL_RESULT, output.tool_hash);
        let payload = serde_json::to_vec(output)?;

        self.jetstream
            .publish(subject, payload.into())
            .await?
            .await?;

        tracing::debug!(
            "Published tool output: {} ({})",
            output.tool_name,
            output.tool_hash
        );
        Ok(())
    }

    // =========================================================================
    // OSSEC ALERT PUBLISHING
    // =========================================================================

    /// Publish OSSEC alert
    pub async fn publish_ossec_alert(&self, alert: &OssecAlert) -> Result<()> {
        let subject = format!("{}.{}", subjects::OSSEC_ALERT, alert.rule_id);
        let payload = serde_json::to_vec(alert)?;

        self.jetstream
            .publish(subject, payload.into())
            .await?
            .await?;

        tracing::debug!("Published OSSEC alert: rule {}", alert.rule_id);
        Ok(())
    }

    // =========================================================================
    // EEI QUERY PUBLISHING
    // =========================================================================

    /// Publish EEI query
    pub async fn publish_eei_query(&self, query: &EeiQuery) -> Result<()> {
        let subject = format!("{}.{}", subjects::EEI_QUERY, query.query_id);
        let payload = serde_json::to_vec(query)?;

        self.jetstream
            .publish(subject, payload.into())
            .await?
            .await?;

        tracing::debug!("Published EEI query: {}", query.query_id);
        Ok(())
    }

    // =========================================================================
    // RING BUS INTER-NODE COMMUNICATION
    // =========================================================================

    /// Send message on Ring Bus to next node
    pub async fn ring_forward(&self, msg: RingMessage) -> Result<()> {
        let next_node = (self.node_id + 1) % 9; // 9-node ring
        let subject = format!("{}.{}.forward", subjects::RING_BUS, next_node);
        let payload = serde_json::to_vec(&msg)?;

        self.client.publish(subject, payload.into()).await?;

        tracing::trace!("Ring forwarded to node {}", next_node);
        Ok(())
    }

    /// Broadcast message to all Ring Bus nodes
    pub async fn ring_broadcast(&self, msg: RingMessage) -> Result<()> {
        let subject = format!("{}.all.broadcast", subjects::RING_BUS);
        let payload = serde_json::to_vec(&msg)?;

        self.client.publish(subject, payload.into()).await?;

        tracing::trace!("Ring broadcast from node {}", self.node_id);
        Ok(())
    }

    /// Subscribe to Ring Bus messages for this node
    pub async fn subscribe_ring(&self) -> Result<async_nats::Subscriber> {
        let subject = format!("{}.{}.>", subjects::RING_BUS, self.node_id);
        let sub = self.client.subscribe(subject).await?;
        Ok(sub)
    }

    /// Subscribe to Ring Bus broadcasts
    pub async fn subscribe_broadcasts(&self) -> Result<async_nats::Subscriber> {
        let subject = format!("{}.all.>", subjects::RING_BUS);
        let sub = self.client.subscribe(subject).await?;
        Ok(sub)
    }

    // =========================================================================
    // CONSUMER CREATION (for pull-based consumption)
    // =========================================================================

    /// Create consumer for threat events
    pub async fn create_threat_consumer(&self, name: &str) -> Result<PullConsumer> {
        let stream = self.jetstream.get_stream(streams::THREAT_EVENTS).await?;
        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                durable_name: Some(name.to_string()),
                ..Default::default()
            })
            .await?;
        Ok(consumer)
    }

    /// Create consumer for tool outputs
    pub async fn create_tool_consumer(&self, name: &str) -> Result<PullConsumer> {
        let stream = self.jetstream.get_stream(streams::TOOL_OUTPUTS).await?;
        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                durable_name: Some(name.to_string()),
                ..Default::default()
            })
            .await?;
        Ok(consumer)
    }

    /// Create consumer for OSSEC alerts
    pub async fn create_ossec_consumer(&self, name: &str) -> Result<PullConsumer> {
        let stream = self.jetstream.get_stream(streams::OSSEC_ALERTS).await?;
        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                durable_name: Some(name.to_string()),
                ..Default::default()
            })
            .await?;
        Ok(consumer)
    }

    // =========================================================================
    // UTILITIES
    // =========================================================================

    /// Get node ID
    pub fn node_id(&self) -> u8 {
        self.node_id
    }

    /// Get NATS client reference
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get JetStream context reference
    pub fn jetstream(&self) -> &jetstream::Context {
        &self.jetstream
    }
}

// =============================================================================
// MESSAGE TYPES
// =============================================================================

/// Threat event for JetStream persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    /// Trivariate hash of threat
    pub threat_hash: u64,
    /// Entity ID in ECS
    pub entity_id: u64,
    /// Threat confidence (0.0-1.0)
    pub confidence: f32,
    /// HD4 phase
    pub hd4_phase: u8,
    /// MITRE technique if known
    pub mitre_technique: Option<String>,
    /// Source information
    pub source: String,
    /// Timestamp (nanos since epoch)
    pub timestamp_ns: u64,
    /// Ring Bus source node
    pub source_node: u8,
}

/// Tool output for JetStream persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    /// Tool trivariate hash
    pub tool_hash: u64,
    /// Tool name
    pub tool_name: String,
    /// Raw output (base64 encoded if binary)
    pub output: String,
    /// Output hash for deduplication
    pub output_hash: u64,
    /// Output size in bytes
    pub output_size: u32,
    /// Success flag
    pub success: bool,
    /// MITRE technique if applicable
    pub mitre_technique: Option<String>,
    /// Operator ID
    pub operator_id: String,
    /// Timestamp (nanos since epoch)
    pub timestamp_ns: u64,
}

/// OSSEC alert for JetStream persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssecAlert {
    /// OSSEC rule ID
    pub rule_id: u32,
    /// Alert level (0-15)
    pub level: u8,
    /// Rule description
    pub description: String,
    /// MITRE technique
    pub mitre_technique: Option<String>,
    /// MITRE tactic
    pub mitre_tactic: Option<String>,
    /// Source IP
    pub src_ip: Option<String>,
    /// Destination IP
    pub dst_ip: Option<String>,
    /// Raw alert data (JSON)
    pub raw_data: Option<String>,
    /// Timestamp (nanos since epoch)
    pub timestamp_ns: u64,
}

/// EEI query for intelligence correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiQuery {
    /// Query ID
    pub query_id: u64,
    /// Query keywords
    pub keywords: Vec<String>,
    /// MITRE techniques to match
    pub mitre_techniques: Vec<String>,
    /// Time window start (nanos)
    pub window_start_ns: u64,
    /// Time window end (nanos)
    pub window_end_ns: u64,
    /// Requesting entity
    pub requester: String,
}

/// EEI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiResponse {
    /// Query ID this responds to
    pub query_id: u64,
    /// Matched EEI IDs
    pub matched_eeis: Vec<u64>,
    /// Correlation score
    pub correlation_score: f32,
    /// Time-of-Value remaining (ms)
    pub tov_remaining_ms: u64,
    /// Response timestamp
    pub timestamp_ns: u64,
}

/// Ring Bus message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingMessage {
    /// Source node ID
    pub source_node: u8,
    /// Target node ID (None = broadcast)
    pub target_node: Option<u8>,
    /// Message type
    pub msg_type: RingMessageType,
    /// Payload (serialized inner message)
    pub payload: Vec<u8>,
    /// Hop count (for loop detection)
    pub hop_count: u8,
    /// Timestamp (nanos since epoch)
    pub timestamp_ns: u64,
}

/// Ring Bus message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RingMessageType {
    /// Threat alert
    ThreatAlert,
    /// Tool trigger request
    ToolTrigger,
    /// SDT gate event
    SdtGate,
    /// Crystal resonance update
    CrystalResonance,
    /// Tick synchronization
    TickSync,
    /// EEI query
    EeiQuery,
    /// HD4 phase transition
    Hd4Transition,
}

impl RingMessage {
    /// Create new Ring message from this node
    pub fn new(msg_type: RingMessageType, payload: Vec<u8>) -> Self {
        Self {
            source_node: DEFENDER_NODE_ID,
            target_node: None,
            msg_type,
            payload,
            hop_count: 0,
            timestamp_ns: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    /// Create targeted Ring message
    pub fn new_targeted(msg_type: RingMessageType, target: u8, payload: Vec<u8>) -> Self {
        Self {
            source_node: DEFENDER_NODE_ID,
            target_node: Some(target),
            msg_type,
            payload,
            hop_count: 0,
            timestamp_ns: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }
}
