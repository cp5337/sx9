//! CTAS-7 Port Manager Types
//! 
//! Core types for the real CTAS-7 port manager with major port blocks,
//! mirror blocks, and deception settings.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Port Manager Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortManagerConfig {
    pub port_range: (u16, u16), // 18100-18199
    pub mirror_blocks: Vec<MirrorBlock>,
    pub deception_settings: DeceptionSettings,
    pub cyber_ops_enabled: bool,
}

/// Port Allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocation {
    pub port: u16,
    pub service_name: String,
    pub service_type: ServiceType,
    pub allocated_at: DateTime<Utc>,
    pub cyber_ops_enabled: bool,
    pub mirror_ports: Vec<u16>,
    pub deception_active: bool,
    pub allocation_id: String,
}

/// Service Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Foundation,
    CDN,
    XSD,
    Orbital,
    PortManager,
    CyberOps,
    ShipyardManager,
    Analytics,
    Monitoring,
    Custom(String),
}

/// Mirror Block Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorBlock {
    pub primary_port: u16,
    pub mirror_ports: Vec<u16>,
    pub mirror_type: MirrorType,
    pub active: bool,
}

/// Mirror Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirrorType {
    LoadBalancing,
    Failover,
    Deception,
    Stealth,
}

/// Deception Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeceptionSettings {
    pub stealth_mode: bool,
    pub fake_ports: Vec<u16>,
    pub decoy_services: Vec<String>,
    pub traffic_obfuscation: bool,
    pub port_randomization: bool,
}

/// Port Manager Error
#[derive(Debug, thiserror::Error)]
pub enum PortManagerError {
    #[error("Port {0} is out of range")]
    PortOutOfRange(u16),
    #[error("Port {0} is already allocated")]
    PortAlreadyAllocated(u16),
    #[error("Port {0} is reserved")]
    PortReserved(u16),
    #[error("Service {0} not found")]
    ServiceNotFound(String),
    #[error("No ports available: {0}")]
    NoPortsAvailable(String),
    #[error("Mirror block configuration error: {0}")]
    MirrorBlockError(String),
    #[error("Deception settings error: {0}")]
    DeceptionError(String),
}

/// Port Manager State
#[derive(Debug)]
pub struct PortManagerState {
    pub allocations: HashMap<u16, PortAllocation>,
    pub reserved_ports: Vec<u16>,
    pub mirror_blocks: Vec<MirrorBlock>,
    pub deception_settings: DeceptionSettings,
    pub cyber_ops_enabled: bool,
}

