//! Tactical Communications Status and Management

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStatus {
    pub overall_health: f64,
    pub radio_systems: Vec<RadioSystem>,
    pub networks: Vec<Network>,
    pub crypto_status: CryptoStatus,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioSystem {
    pub name: String,
    pub frequency: String,
    pub operational: bool,
    pub signal_strength: u8,
    pub battery_level: f64,
    pub encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub network_type: NetworkType,
    pub nodes_connected: u32,
    pub bandwidth_mbps: f64,
    pub latency_ms: u32,
    pub operational: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NetworkType {
    Command,
    Intelligence,
    Logistics,
    Fires,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoStatus {
    pub keys_loaded: bool,
    pub crypto_period_valid: bool,
    pub secure_communications: bool,
    pub key_expiry: DateTime<Utc>,
}

impl CommunicationStatus {
    pub fn new() -> Self {
        Self {
            overall_health: 0.85,
            radio_systems: vec![
                RadioSystem {
                    name: "AN/PRC-152".to_string(),
                    frequency: "54.000".to_string(),
                    operational: true,
                    signal_strength: 85,
                    battery_level: 78.0,
                    encryption: true,
                },
                RadioSystem {
                    name: "AN/PRC-117G".to_string(),
                    frequency: "142.500".to_string(),
                    operational: true,
                    signal_strength: 92,
                    battery_level: 65.0,
                    encryption: true,
                },
            ],
            networks: vec![
                Network {
                    name: "Command Net".to_string(),
                    network_type: NetworkType::Command,
                    nodes_connected: 8,
                    bandwidth_mbps: 2.5,
                    latency_ms: 45,
                    operational: true,
                },
                Network {
                    name: "Intel Net".to_string(),
                    network_type: NetworkType::Intelligence,
                    nodes_connected: 4,
                    bandwidth_mbps: 1.2,
                    latency_ms: 67,
                    operational: true,
                },
            ],
            crypto_status: CryptoStatus {
                keys_loaded: true,
                crypto_period_valid: true,
                secure_communications: true,
                key_expiry: chrono::Utc::now()
                    + chrono::Duration::days(7),
            },
            last_check: chrono::Utc::now(),
        }
    }

    pub fn calculate_health(&mut self) -> f64 {
        let radio_health = self.radio_systems.iter()
            .map(|r| if r.operational { 1.0 } else { 0.0 })
            .sum::<f64>() / self.radio_systems.len() as f64;

        let network_health = self.networks.iter()
            .map(|n| if n.operational { 1.0 } else { 0.0 })
            .sum::<f64>() / self.networks.len() as f64;

        let crypto_health = if self.crypto_status.secure_communications
            && self.crypto_status.keys_loaded
            && self.crypto_status.crypto_period_valid {
            1.0
        } else {
            0.0
        };

        self.overall_health = (radio_health + network_health + crypto_health) / 3.0;
        self.last_check = chrono::Utc::now();

        self.overall_health
    }

    pub fn check_radio_status(&mut self, radio_name: &str) -> Option<&RadioSystem> {
        self.radio_systems.iter().find(|r| r.name == radio_name)
    }

    pub fn get_operational_radios(&self) -> Vec<&RadioSystem> {
        self.radio_systems.iter().filter(|r| r.operational).collect()
    }

    pub fn get_network_status(&self, net_type: NetworkType) -> Option<&Network> {
        self.networks.iter().find(|n|
            matches!((net_type, &n.network_type),
                (NetworkType::Command, NetworkType::Command) |
                (NetworkType::Intelligence, NetworkType::Intelligence) |
                (NetworkType::Logistics, NetworkType::Logistics) |
                (NetworkType::Fires, NetworkType::Fires)
            )
        )
    }
}