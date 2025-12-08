//! Lightweight health check network for meta control
//!
//! Minimal overhead health monitoring across CTAS-7 ecosystem

use super::hash_engine::{get_global_ecosystem_verification, get_global_hash_state};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::interval;

/// Lightweight health check network
pub struct HealthNetwork {
    /// Network nodes (orchestrators, services, etc.)
    nodes: HashMap<String, HealthNode>,
    /// Network topology for efficient routing
    topology: NetworkTopology,
    /// Global health state
    global_health: GlobalHealthState,
    /// Network configuration
    config: HealthNetworkConfig,
}

/// Individual health node in the network
#[derive(Debug)]
pub struct HealthNode {
    /// Node identifier
    pub node_id: String,
    /// Node type for routing decisions
    pub node_type: NodeType,
    /// Current health status
    pub health_status: NodeHealthStatus,
    /// Last health check timestamp
    pub last_check: AtomicU64,
    /// Health check counter
    pub check_count: AtomicU64,
    /// Node is active in network
    pub is_active: AtomicBool,
    /// Minimal endpoint for health checks
    pub endpoint: Option<String>,
}

/// Node types in the health network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    /// Foundation orchestrator (critical)
    FoundationOrchestrator,
    /// Service orchestrator
    ServiceOrchestrator,
    /// Smart crate orchestrator
    CrateOrchestrator,
    /// Quality assurance orchestrator
    QAOrchestrator,
    /// Shipyard orchestrator (master)
    ShipyardOrchestrator,
    /// External service
    ExternalService,
}

/// Node health status (minimal data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealthStatus {
    /// Overall health
    pub status: HealthLevel,
    /// Last response time (microseconds)
    pub response_time_us: u64,
    /// Error count since last reset
    pub error_count: u32,
    /// Hash integrity status
    pub hash_integrity: bool,
}

/// Health levels (3-bit encoding for efficiency)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthLevel {
    Healthy = 0,
    Degraded = 1,
    Critical = 2,
    Offline = 3,
}

/// Network topology for efficient health routing
#[derive(Debug, Clone)]
pub struct NetworkTopology {
    /// Direct connections between nodes
    connections: HashMap<String, Vec<String>>,
    /// Master nodes (can query all others)
    master_nodes: Vec<String>,
    /// Critical path nodes (must be healthy)
    critical_path: Vec<String>,
}

/// Global health state (minimal memory footprint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHealthState {
    /// Overall ecosystem health
    pub ecosystem_health: HealthLevel,
    /// Total active nodes
    pub active_nodes: u32,
    /// Healthy nodes count
    pub healthy_nodes: u32,
    /// Critical failures count
    pub critical_failures: u32,
    /// Hash integrity status
    pub hash_integrity: bool,
    /// Last update timestamp
    pub last_updated: u64,
}

/// Configuration for health network
#[derive(Debug, Clone)]
pub struct HealthNetworkConfig {
    /// Health check interval (milliseconds)
    pub check_interval_ms: u64,
    /// Network timeout (milliseconds)
    pub timeout_ms: u64,
    /// Maximum nodes in network
    pub max_nodes: usize,
    /// Enable hash integrity checks
    pub hash_checks_enabled: bool,
    /// UDP multicast configuration
    pub multicast: MulticastConfig,
}

/// UDP multicast configuration
#[derive(Debug, Clone)]
pub struct MulticastConfig {
    /// Multicast group IP (e.g., "239.1.2.3")
    pub group_ip: String,
    /// Multicast port
    pub port: u16,
    /// Time-to-live for multicast packets
    pub ttl: u32,
    /// Enable multicast (fallback to direct UDP if disabled)
    pub enabled: bool,
}

/// Minimal health check message (UDP multicast-optimized)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckMessage {
    /// Message type
    pub msg_type: MessageType,
    /// Source node ID (compressed to 4 bytes)
    pub from_node_hash: u32,
    /// Multicast group (0 for global broadcast)
    pub multicast_group: u8,
    /// Timestamp (4 bytes - relative to epoch)
    pub timestamp: u32,
    /// Status data (8 bytes packed)
    pub status_data: u64,
    /// Sequence number for ordering
    pub sequence: u16,
}

/// Message types (4-bit encoding)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MessageType {
    Ping = 0,
    Pong = 1,
    StatusUpdate = 2,
    HashSync = 3,
    CriticalAlert = 4,
}

impl HealthNetwork {
    /// Create new lightweight health network
    pub fn new(config: HealthNetworkConfig) -> Self {
        Self {
            nodes: HashMap::new(),
            topology: NetworkTopology::new(),
            global_health: GlobalHealthState::new(),
            config,
        }
    }

    /// Register node in health network (O(1) operation)
    pub fn register_node(&mut self, node_id: String, node_type: NodeType, endpoint: Option<String>) {
        let node = HealthNode {
            node_id: node_id.clone(),
            node_type: node_type.clone(),
            health_status: NodeHealthStatus::new(),
            last_check: AtomicU64::new(0),
            check_count: AtomicU64::new(0),
            is_active: AtomicBool::new(true),
            endpoint,
        };

        self.nodes.insert(node_id.clone(), node);

        // Update topology based on node type
        match node_type {
            NodeType::ShipyardOrchestrator => {
                self.topology.master_nodes.push(node_id.clone());
                self.topology.critical_path.push(node_id);
            }
            NodeType::FoundationOrchestrator => {
                self.topology.critical_path.push(node_id);
            }
            _ => {}
        }
    }

    /// Perform lightweight health check (minimal overhead)
    pub async fn perform_health_check(&mut self, node_id: &str) -> Option<NodeHealthStatus> {
        let node = self.nodes.get_mut(node_id)?;

        let start_time = SystemTime::now();
        node.check_count.fetch_add(1, Ordering::Relaxed);

        // Minimal health check - just verify node responsiveness
        let health_status = if node.is_active.load(Ordering::Relaxed) {
            let response_time = start_time.elapsed().unwrap_or(Duration::ZERO).as_micros() as u64;

            // Check hash integrity if enabled
            let hash_integrity = if self.config.hash_checks_enabled {
                get_global_ecosystem_verification()
                    .map(|v| v.ecosystem_integrity)
                    .unwrap_or(false)
            } else {
                true
            };

            NodeHealthStatus {
                status: HealthLevel::Healthy,
                response_time_us: response_time,
                error_count: 0,
                hash_integrity,
            }
        } else {
            NodeHealthStatus {
                status: HealthLevel::Offline,
                response_time_us: 0,
                error_count: 1,
                hash_integrity: false,
            }
        };

        node.health_status = health_status.clone();
        node.last_check.store(
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            Ordering::Relaxed,
        );

        Some(health_status)
    }

    /// Get global health state (O(1) operation)
    pub fn get_global_health(&self) -> GlobalHealthState {
        self.global_health.clone()
    }

    /// Update global health state (lightweight aggregation)
    pub fn update_global_health(&mut self) {
        let mut healthy_count = 0;
        let mut critical_count = 0;
        let active_count = self.nodes.len() as u32;

        // Fast iteration over nodes
        for node in self.nodes.values() {
            if node.is_active.load(Ordering::Relaxed) {
                match node.health_status.status {
                    HealthLevel::Healthy => healthy_count += 1,
                    HealthLevel::Critical | HealthLevel::Offline => critical_count += 1,
                    _ => {}
                }
            }
        }

        // Check hash integrity
        let hash_integrity = get_global_ecosystem_verification()
            .map(|v| v.ecosystem_integrity)
            .unwrap_or(true);

        // Determine ecosystem health
        let ecosystem_health = if critical_count > 0 {
            HealthLevel::Critical
        } else if healthy_count < active_count / 2 {
            HealthLevel::Degraded
        } else {
            HealthLevel::Healthy
        };

        self.global_health = GlobalHealthState {
            ecosystem_health,
            active_nodes: active_count,
            healthy_nodes: healthy_count,
            critical_failures: critical_count,
            hash_integrity,
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
    }

    /// Start health network monitoring (lightweight background task)
    pub async fn start_monitoring(&mut self) {
        let mut interval = interval(Duration::from_millis(self.config.check_interval_ms));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // Perform health checks on critical path first
                    for node_id in &self.topology.critical_path.clone() {
                        self.perform_health_check(node_id).await;
                    }

                    // Update global health state
                    self.update_global_health();
                }
            }
        }
    }

    /// Export minimal health data for external monitoring
    pub fn export_health_data(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();

        data.insert("ecosystem_health".to_string(), format!("{:?}", self.global_health.ecosystem_health));
        data.insert("active_nodes".to_string(), self.global_health.active_nodes.to_string());
        data.insert("healthy_nodes".to_string(), self.global_health.healthy_nodes.to_string());
        data.insert("critical_failures".to_string(), self.global_health.critical_failures.to_string());
        data.insert("hash_integrity".to_string(), self.global_health.hash_integrity.to_string());

        // Add hash state if available
        if let Some(hash_state) = get_global_hash_state() {
            for (key, value) in hash_state {
                data.insert(format!("hash_{}", key), value);
            }
        }

        data
    }

    /// Get TOML representation for status reporting
    pub fn to_toml(&self) -> String {
        format!(
            r#"[health_network]
ecosystem_health = "{:?}"
active_nodes = {}
healthy_nodes = {}
critical_failures = {}
hash_integrity = {}
last_updated = "{}"

# Node health summary
[health_network.nodes]
total_registered = {}
critical_path_nodes = {}
master_nodes = {}
"#,
            self.global_health.ecosystem_health,
            self.global_health.active_nodes,
            self.global_health.healthy_nodes,
            self.global_health.critical_failures,
            self.global_health.hash_integrity,
            chrono::DateTime::from_timestamp(self.global_health.last_updated as i64, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            self.nodes.len(),
            self.topology.critical_path.len(),
            self.topology.master_nodes.len()
        )
    }
}

impl NetworkTopology {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
            master_nodes: Vec::new(),
            critical_path: Vec::new(),
        }
    }
}

impl GlobalHealthState {
    fn new() -> Self {
        Self {
            ecosystem_health: HealthLevel::Healthy,
            active_nodes: 0,
            healthy_nodes: 0,
            critical_failures: 0,
            hash_integrity: true,
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

impl NodeHealthStatus {
    fn new() -> Self {
        Self {
            status: HealthLevel::Healthy,
            response_time_us: 0,
            error_count: 0,
            hash_integrity: true,
        }
    }
}

impl Default for HealthNetworkConfig {
    fn default() -> Self {
        Self {
            check_interval_ms: 5000, // 5 second intervals
            timeout_ms: 1000,        // 1 second timeout
            max_nodes: 100,          // Maximum 100 nodes
            hash_checks_enabled: true,
            multicast: MulticastConfig::default(),
        }
    }
}

impl Default for MulticastConfig {
    fn default() -> Self {
        Self {
            group_ip: "239.1.7.53".to_string(), // CTAS-7 multicast group
            port: 17053,                         // CTAS health port
            ttl: 4,                              // Local network only
            enabled: true,
        }
    }
}

/// Global health network instance
pub static mut GLOBAL_HEALTH_NETWORK: Option<HealthNetwork> = None;

/// Initialize global health network
pub fn init_global_health_network(config: HealthNetworkConfig) {
    unsafe {
        GLOBAL_HEALTH_NETWORK = Some(HealthNetwork::new(config));
    }
}

/// Register node in global health network
pub fn register_global_health_node(node_id: String, node_type: NodeType, endpoint: Option<String>) {
    unsafe {
        if let Some(network) = GLOBAL_HEALTH_NETWORK.as_mut() {
            network.register_node(node_id, node_type, endpoint);
        }
    }
}

/// Get global health state
pub fn get_global_health_state() -> Option<GlobalHealthState> {
    unsafe {
        GLOBAL_HEALTH_NETWORK.as_ref().map(|n| n.get_global_health())
    }
}

/// Export global health data
pub fn export_global_health_data() -> Option<HashMap<String, String>> {
    unsafe {
        GLOBAL_HEALTH_NETWORK.as_ref().map(|n| n.export_health_data())
    }
}