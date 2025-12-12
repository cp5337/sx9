//! NATS Bridge for distributed ATLAS communication
//!
//! Enables cross-process and cross-machine IPC via NATS JetStream.
//!
//! ## Subjects
//! ```text
//! sx9.atlas.cmd.{priority}    - Commands (critical/urgent/normal)
//! sx9.atlas.result            - Results back to apecs
//! sx9.atlas.plasma            - Plasma state updates
//! sx9.atlas.tick              - Tick synchronization
//! sx9.sdt.{gate_id}.trigger   - SDT gate triggers
//! sx9.sdt.{gate_id}.state     - SDT state changes
//! ```
//!
//! Note: This module requires the `nats` feature and std.

use std::time::Duration;

use crate::plasma::PlasmaSnapshot;
use async_nats::jetstream;
use async_nats::Client;
use tokio::sync::mpsc;

/// NATS subject prefixes
pub mod subjects {
    pub const CMD_CRITICAL: &str = "sx9.atlas.cmd.critical";
    pub const CMD_URGENT: &str = "sx9.atlas.cmd.urgent";
    pub const CMD_NORMAL: &str = "sx9.atlas.cmd.normal";
    pub const RESULT: &str = "sx9.atlas.result";
    pub const PLASMA: &str = "sx9.atlas.plasma";
    pub const TICK: &str = "sx9.atlas.tick";
    pub const SDT_TRIGGER: &str = "sx9.sdt"; // + .{gate_id}.trigger
    pub const SDT_STATE: &str = "sx9.sdt"; // + .{gate_id}.state
}

/// Stream names for JetStream
pub mod streams {
    pub const COMMANDS: &str = "SX9_ATLAS_COMMANDS";
    pub const RESULTS: &str = "SX9_ATLAS_RESULTS";
    pub const PLASMA: &str = "SX9_PLASMA_STATE";
    pub const SDT: &str = "SX9_SDT_EVENTS";
}

/// NATS bridge configuration
#[derive(Debug, Clone)]
pub struct NatsBridgeConfig {
    /// NATS server URL (e.g., "nats://localhost:4222")
    pub url: String,
    /// Instance ID for this ATLAS daemon
    pub instance_id: String,
    /// Enable JetStream for persistence
    pub jetstream: bool,
    /// Consumer batch size
    pub batch_size: usize,
    /// Consumer timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for NatsBridgeConfig {
    fn default() -> Self {
        Self {
            url: "nats://localhost:4222".to_string(),
            instance_id: uuid::Uuid::new_v4().to_string(),
            jetstream: true,
            batch_size: 100,
            timeout_ms: 100,
        }
    }
}

/// Wire format for commands over NATS
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WireCommand {
    pub kind: WireCommandKind,
    pub sch_hash: u64,
    pub tick_id: u64,
    pub priority: u8,
    pub request_id: u32,
    pub source_instance: String,
}

/// Serializable command kinds
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WireCommandKind {
    Dijkstra {
        src: u32,
        dst: u32,
        max_hops: u8,
    },
    Bfs {
        src: u32,
        max_depth: u8,
    },
    MatroidRank {
        nodes: Vec<u32>,
    },
    ConvergenceCheck {
        entity_id: u32,
        epsilon: f32,
        window: u16,
    },
    BatchHash {
        count: u32,
        item_size: u16,
    },
    TrivariateHash {
        domain: u8,
        execution: u8,
        delta_class: u8,
    },
    TickSync {
        tick_id: u64,
        timestamp_ns: u64,
    },
    SdtTrigger {
        gate_id: u32,
        reason: u16,
    },
    SdtReset {
        gate_id: u32,
    },
    PlasmaUpdate {
        field_id: u32,
        delta_angle: u16,
        entropy: u32,
        excited: bool,
    },
    Ping {
        seq: u32,
        timestamp_ns: u64,
    },
    Shutdown,
    Stats,
}

/// Wire format for results over NATS
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WireResult {
    pub kind: WireResultKind,
    pub request_id: u32,
    pub tick_id: u64,
    pub latency_ns: u64,
    pub success: bool,
    pub error_code: u8,
    pub source_instance: String,
}

/// Serializable result kinds
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WireResultKind {
    Path {
        nodes: Vec<u32>,
        cost: f32,
    },
    MatroidRank {
        rank: u32,
    },
    Convergence {
        converged: bool,
        current_delta: f32,
        ticks_stable: u16,
    },
    TrivariateHash {
        sch: u64,
        cuid_hi: u64,
        cuid_lo: u64,
        uuid_hi: u64,
        uuid_lo: u64,
    },
    TickAck {
        tick_id: u64,
        drift_ns: i64,
    },
    SdtState {
        gate_id: u32,
        state: u8,
        last_trigger_tick: u64,
    },
    PlasmaState {
        field_id: u32,
        delta_angle: u16,
        entropy: u32,
        excited: bool,
    },
    Pong {
        seq: u32,
        request_timestamp_ns: u64,
        response_timestamp_ns: u64,
    },
    Stats {
        commands_processed: u64,
        avg_latency_ns: u64,
        queue_depth: u32,
        uptime_secs: u64,
    },
    Ack,
    Error {
        code: u16,
        message: String,
    },
}

/// Wire format for plasma state broadcasts
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WirePlasmaState {
    pub delta_angle: u16,
    pub entropy: u32,
    pub excited: bool,
    pub sdt_state: u8,
    pub last_trigger_tick: u64,
    pub trigger_count: u32,
    pub source_instance: String,
    pub timestamp_ns: u64,
}

impl From<PlasmaSnapshot> for WirePlasmaState {
    fn from(snap: PlasmaSnapshot) -> Self {
        Self {
            delta_angle: snap.delta_angle,
            entropy: snap.entropy,
            excited: snap.excited,
            sdt_state: snap.sdt_state as u8,
            last_trigger_tick: snap.last_trigger_tick,
            trigger_count: snap.trigger_count,
            source_instance: String::new(), // Set by bridge
            timestamp_ns: 0,                // Set by bridge
        }
    }
}

/// NATS Bridge for ATLAS daemon
pub struct NatsBridge {
    config: NatsBridgeConfig,
    client: Client,
    jetstream: Option<jetstream::Context>,

    // Channels for local bus integration
    cmd_tx: mpsc::Sender<WireCommand>,
    cmd_rx: mpsc::Receiver<WireCommand>,
    result_tx: mpsc::Sender<WireResult>,
    result_rx: mpsc::Receiver<WireResult>,
}

impl NatsBridge {
    /// Connect to NATS and initialize bridge
    pub async fn connect(config: NatsBridgeConfig) -> anyhow::Result<Self> {
        let client = async_nats::connect(&config.url).await?;

        let jetstream = if config.jetstream {
            Some(jetstream::new(client.clone()))
        } else {
            None
        };

        let (cmd_tx, cmd_rx) = mpsc::channel(1024);
        let (result_tx, result_rx) = mpsc::channel(1024);

        let mut bridge = Self {
            config,
            client,
            jetstream,
            cmd_tx,
            cmd_rx,
            result_tx,
            result_rx,
        };

        // Initialize streams if JetStream enabled
        if bridge.jetstream.is_some() {
            bridge.init_streams().await?;
        }

        Ok(bridge)
    }

    /// Initialize JetStream streams
    async fn init_streams(&mut self) -> anyhow::Result<()> {
        let js = self.jetstream.as_ref().unwrap();

        // Commands stream
        let _ = js
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::COMMANDS.to_string(),
                subjects: vec![
                    format!("{}.*", subjects::CMD_CRITICAL),
                    format!("{}.*", subjects::CMD_URGENT),
                    format!("{}.*", subjects::CMD_NORMAL),
                ],
                max_messages: 100_000,
                max_age: Duration::from_secs(3600), // 1 hour
                ..Default::default()
            })
            .await?;

        // Results stream
        let _ = js
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::RESULTS.to_string(),
                subjects: vec![format!("{}.*", subjects::RESULT)],
                max_messages: 100_000,
                max_age: std::time::Duration::from_secs(3600),
                ..Default::default()
            })
            .await?;

        // Plasma state stream
        let _ = js
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::PLASMA.to_string(),
                subjects: vec![format!("{}.*", subjects::PLASMA)],
                max_messages: 10_000,
                max_age: Duration::from_secs(300), // 5 minutes
                ..Default::default()
            })
            .await?;

        // SDT events stream
        let _ = js
            .get_or_create_stream(jetstream::stream::Config {
                name: streams::SDT.to_string(),
                subjects: vec![
                    format!("{}.*.trigger", subjects::SDT_TRIGGER),
                    format!("{}.*.state", subjects::SDT_STATE),
                ],
                max_messages: 50_000,
                max_age: Duration::from_secs(86400), // 24 hours
                ..Default::default()
            })
            .await?;

        tracing::info!("JetStream streams initialized");
        Ok(())
    }

    /// Publish a command to NATS
    pub async fn publish_command(&self, cmd: WireCommand) -> anyhow::Result<()> {
        let subject = match cmd.priority {
            2 => format!("{}.{}", subjects::CMD_CRITICAL, self.config.instance_id),
            1 => format!("{}.{}", subjects::CMD_URGENT, self.config.instance_id),
            _ => format!("{}.{}", subjects::CMD_NORMAL, self.config.instance_id),
        };

        let payload = serde_json::to_vec(&cmd)?;

        if let Some(js) = &self.jetstream {
            js.publish(subject, payload.into()).await?;
        } else {
            self.client.publish(subject, payload.into()).await?;
        }

        Ok(())
    }

    /// Publish a result to NATS
    pub async fn publish_result(&self, result: WireResult) -> anyhow::Result<()> {
        let subject = format!("{}.{}", subjects::RESULT, self.config.instance_id);
        let payload = serde_json::to_vec(&result)?;

        if let Some(js) = &self.jetstream {
            js.publish(subject, payload.into()).await?;
        } else {
            self.client.publish(subject, payload.into()).await?;
        }

        Ok(())
    }

    /// Broadcast plasma state
    pub async fn broadcast_plasma(&self, state: WirePlasmaState) -> anyhow::Result<()> {
        let subject = format!("{}.{}", subjects::PLASMA, self.config.instance_id);
        let payload = serde_json::to_vec(&state)?;

        self.client.publish(subject, payload.into()).await?;
        Ok(())
    }

    /// Publish SDT trigger event
    pub async fn publish_sdt_trigger(&self, gate_id: u32, reason: u16) -> anyhow::Result<()> {
        let subject = format!("{}.{}.trigger", subjects::SDT_TRIGGER, gate_id);
        let payload = serde_json::to_vec(&serde_json::json!({
            "gate_id": gate_id,
            "reason": reason,
            "source_instance": self.config.instance_id,
            "timestamp_ns": std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }))?;

        if let Some(js) = &self.jetstream {
            js.publish(subject, payload.into()).await?;
        } else {
            self.client.publish(subject, payload.into()).await?;
        }

        Ok(())
    }

    /// Subscribe to commands from other instances
    pub async fn subscribe_commands(&self) -> anyhow::Result<async_nats::Subscriber> {
        // Subscribe to all command subjects except our own
        let subject = format!("{}.>", subjects::CMD_CRITICAL);
        let sub = self.client.subscribe(subject).await?;
        Ok(sub)
    }

    /// Subscribe to plasma state from other instances
    pub async fn subscribe_plasma(&self) -> anyhow::Result<async_nats::Subscriber> {
        let subject = format!("{}.>", subjects::PLASMA);
        let sub = self.client.subscribe(subject).await?;
        Ok(sub)
    }

    /// Subscribe to SDT events
    pub async fn subscribe_sdt(
        &self,
        gate_id: Option<u32>,
    ) -> anyhow::Result<async_nats::Subscriber> {
        let subject = match gate_id {
            Some(id) => format!("{}.{}.>", subjects::SDT_TRIGGER, id),
            None => format!("{}.>", subjects::SDT_TRIGGER),
        };
        let sub = self.client.subscribe(subject).await?;
        Ok(sub)
    }

    /// Get sender for incoming commands (from NATS to local bus)
    pub fn command_sender(&self) -> mpsc::Sender<WireCommand> {
        self.cmd_tx.clone()
    }

    /// Get receiver for incoming commands
    pub fn command_receiver(&mut self) -> &mut mpsc::Receiver<WireCommand> {
        &mut self.cmd_rx
    }

    /// Get sender for outgoing results
    pub fn result_sender(&self) -> mpsc::Sender<WireResult> {
        self.result_tx.clone()
    }

    /// Get receiver for outgoing results (to publish to NATS)
    pub fn result_receiver(&mut self) -> &mut mpsc::Receiver<WireResult> {
        &mut self.result_rx
    }

    /// Instance ID
    pub fn instance_id(&self) -> &str {
        &self.config.instance_id
    }
}

/// Tick synchronization message
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TickSync {
    pub tick_id: u64,
    pub timestamp_ns: u64,
    pub source_instance: String,
}

/// Publish tick sync to all instances
pub async fn broadcast_tick(
    client: &Client,
    tick_id: u64,
    instance_id: &str,
) -> anyhow::Result<()> {
    let msg = TickSync {
        tick_id,
        timestamp_ns: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
        source_instance: instance_id.to_string(),
    };

    let payload = serde_json::to_vec(&msg)?;
    client.publish(subjects::TICK, payload.into()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_command_serialization() {
        let cmd = WireCommand {
            kind: WireCommandKind::Dijkstra {
                src: 1,
                dst: 2,
                max_hops: 5,
            },
            sch_hash: 0xDEADBEEF,
            tick_id: 42,
            priority: 1,
            request_id: 123,
            source_instance: "test".to_string(),
        };

        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: WireCommand = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.sch_hash, 0xDEADBEEF);
        assert_eq!(parsed.tick_id, 42);
        assert_eq!(parsed.request_id, 123);
    }

    #[test]
    fn test_wire_result_serialization() {
        let result = WireResult {
            kind: WireResultKind::MatroidRank { rank: 5 },
            request_id: 456,
            tick_id: 100,
            latency_ns: 1000,
            success: true,
            error_code: 0,
            source_instance: "test".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: WireResult = serde_json::from_str(&json).unwrap();

        assert!(parsed.success);
        assert_eq!(parsed.request_id, 456);
    }
}
