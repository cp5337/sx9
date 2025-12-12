//! Configuration for Plasma Defender

use crate::agents::AgentType;
use serde::{Deserialize, Serialize};
use sx9_atlas_bus::CrystalFamily;

// Local ThyristorConfig wrapper for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThyristorConfigWrapper {
    pub gate_thresh: f32,
    pub holding_thresh: f32,
    pub perfect_thresh: f32,
    pub entropy_drought: u32,
}

impl From<sx9_atlas_bus::ThyristorConfig> for ThyristorConfigWrapper {
    fn from(config: sx9_atlas_bus::ThyristorConfig) -> Self {
        Self {
            gate_thresh: config.gate_thresh,
            holding_thresh: config.holding_thresh,
            perfect_thresh: config.perfect_thresh,
            entropy_drought: config.entropy_drought,
        }
    }
}

impl From<ThyristorConfigWrapper> for sx9_atlas_bus::ThyristorConfig {
    fn from(wrapper: ThyristorConfigWrapper) -> Self {
        Self {
            gate_thresh: wrapper.gate_thresh,
            holding_thresh: wrapper.holding_thresh,
            perfect_thresh: wrapper.perfect_thresh,
            entropy_drought: wrapper.entropy_drought,
        }
    }
}

impl From<&ThyristorConfigWrapper> for sx9_atlas_bus::ThyristorConfig {
    fn from(wrapper: &ThyristorConfigWrapper) -> Self {
        Self {
            gate_thresh: wrapper.gate_thresh,
            holding_thresh: wrapper.holding_thresh,
            perfect_thresh: wrapper.perfect_thresh,
            entropy_drought: wrapper.entropy_drought,
        }
    }
}

impl Copy for ThyristorConfigWrapper {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenderConfig {
    pub bind_addr: String,
    pub health_endpoint: String,
    pub metrics_endpoint: String,
    pub nats_url: String,
    pub ann_enabled: bool,
    #[serde(skip)]
    pub crystal_family: Option<CrystalFamily>,
    pub sdt_config: Option<ThyristorConfigWrapper>,
    pub agents: Vec<AgentConfig>,
    pub monitor_interval_ms: u64,
    pub enforce_latency: bool,
    pub max_latency_ms: u64,
    pub request_timeout_secs: u64,
    pub body_size_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub agent_type: AgentType,
    pub enabled: bool,
}

impl Default for DefenderConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:18115".to_string(),
            health_endpoint: "/health".to_string(),
            metrics_endpoint: "/metrics".to_string(),
            nats_url: "nats://localhost:4222".to_string(),
            ann_enabled: false,
            crystal_family: Some(CrystalFamily::GroundStation),
            sdt_config: Some(ThyristorConfigWrapper::from(
                sx9_atlas_bus::ThyristorConfig::default(),
            )),
            agents: vec![
                AgentConfig {
                    id: "network-monitor".to_string(),
                    agent_type: AgentType::NetworkMonitor,
                    enabled: true,
                },
                AgentConfig {
                    id: "threat-hunter".to_string(),
                    agent_type: AgentType::ThreatHunter,
                    enabled: true,
                },
            ],
            monitor_interval_ms: 100,
            enforce_latency: true,
            max_latency_ms: 50,
            request_timeout_secs: 30,
            body_size_limit: 10 * 1024 * 1024, // 10MB
        }
    }
}
