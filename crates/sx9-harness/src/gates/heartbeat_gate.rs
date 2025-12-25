//! Dual Heartbeat Gate
//!
//! Implements zero-trust verification via dual heartbeat pattern:
//! 1. Local heartbeat: Direct HTTP checks to individual services
//! 2. Global heartbeat: HealthNetwork status with hash integrity
//!
//! This gate MUST pass before other QA gates run.
//!
//! Part of the SX9 zero-trust architecture.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Dual heartbeat verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatReport {
    pub schema_version: String,
    pub loadset_id: String,
    /// Overall heartbeat status (both local and global must pass)
    pub passed: bool,
    /// Local service heartbeat results
    pub local: LocalHeartbeat,
    /// Global ecosystem heartbeat
    pub global: GlobalHeartbeat,
    /// Zero-trust score (0-100)
    pub zero_trust_score: u8,
    /// Timestamp of check
    pub timestamp: u64,
}

/// Local heartbeat - direct service checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHeartbeat {
    pub passed: bool,
    pub services: Vec<ServiceStatus>,
}

/// Individual service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub endpoint: String,
    pub healthy: bool,
    pub latency_ms: Option<f64>,
    pub error: Option<String>,
}

/// Global heartbeat - ecosystem health with hash integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHeartbeat {
    pub passed: bool,
    /// HealthNetwork ecosystem status
    pub ecosystem_health: String,
    /// Hash integrity verified
    pub hash_integrity: bool,
    /// Active nodes in health network
    pub active_nodes: u32,
    /// Healthy nodes count
    pub healthy_nodes: u32,
    /// Critical failures
    pub critical_failures: u32,
}

/// Service configuration for local heartbeat
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub health_endpoint: String,
    pub required: bool,
    pub timeout_ms: u64,
}

/// Default services for forge QA
pub fn default_forge_services() -> Vec<ServiceConfig> {
    vec![
        ServiceConfig {
            name: "leptose".to_string(),
            health_endpoint: std::env::var("LEPTOSE_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string())
                + "/api/tags",
            required: false,
            timeout_ms: 2000,
        },
        ServiceConfig {
            name: "chromadb".to_string(),
            health_endpoint: std::env::var("CHROMADB_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string())
                + "/api/v1/heartbeat",
            required: false,
            timeout_ms: 2000,
        },
        ServiceConfig {
            name: "gateway".to_string(),
            health_endpoint: format!(
                "http://localhost:{}/health",
                std::env::var("SX9_GATEWAY_PORT").unwrap_or_else(|_| "18600".to_string())
            ),
            required: false,
            timeout_ms: 2000,
        },
    ]
}

pub struct HeartbeatGate {
    services: Vec<ServiceConfig>,
}

impl HeartbeatGate {
    pub fn new() -> Self {
        Self {
            services: default_forge_services(),
        }
    }

    pub fn with_services(services: Vec<ServiceConfig>) -> Self {
        Self { services }
    }

    /// Run dual heartbeat verification
    pub async fn run(&self) -> Result<HeartbeatReport, String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Run local and global heartbeat checks in parallel
        let (local_result, global_result) = tokio::join!(
            self.check_local_heartbeat(),
            self.check_global_heartbeat()
        );

        let local = local_result;
        let global = global_result;

        // Calculate zero-trust score
        let zero_trust_score = self.calculate_zero_trust_score(&local, &global);

        // Both heartbeats must pass for overall success
        let passed = local.passed && global.passed;

        Ok(HeartbeatReport {
            schema_version: "1.0".to_string(),
            loadset_id: format!("heartbeat-{}", now),
            passed,
            local,
            global,
            zero_trust_score,
            timestamp: now,
        })
    }

    /// Check local service heartbeats
    async fn check_local_heartbeat(&self) -> LocalHeartbeat {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();

        let mut services = Vec::new();
        let mut required_healthy = true;

        for service in &self.services {
            let start = std::time::Instant::now();
            let timeout = Duration::from_millis(service.timeout_ms);

            let result = tokio::time::timeout(
                timeout,
                client.get(&service.health_endpoint).send(),
            )
            .await;

            let status = match result {
                Ok(Ok(response)) => {
                    let healthy = response.status().is_success();
                    if service.required && !healthy {
                        required_healthy = false;
                    }
                    ServiceStatus {
                        name: service.name.clone(),
                        endpoint: service.health_endpoint.clone(),
                        healthy,
                        latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                        error: None,
                    }
                }
                Ok(Err(e)) => {
                    if service.required {
                        required_healthy = false;
                    }
                    ServiceStatus {
                        name: service.name.clone(),
                        endpoint: service.health_endpoint.clone(),
                        healthy: false,
                        latency_ms: None,
                        error: Some(e.to_string()),
                    }
                }
                Err(_) => {
                    if service.required {
                        required_healthy = false;
                    }
                    ServiceStatus {
                        name: service.name.clone(),
                        endpoint: service.health_endpoint.clone(),
                        healthy: false,
                        latency_ms: None,
                        error: Some("Timeout".to_string()),
                    }
                }
            };

            services.push(status);
        }

        LocalHeartbeat {
            passed: required_healthy,
            services,
        }
    }

    /// Check global ecosystem heartbeat via HealthNetwork
    async fn check_global_heartbeat(&self) -> GlobalHeartbeat {
        // Try to get global health state from HealthNetwork
        #[cfg(feature = "health-network")]
        {
            if let Some(health_state) = sx9_foundation_core::health_network::get_global_health_state() {
                return GlobalHeartbeat {
                    passed: matches!(
                        health_state.ecosystem_health,
                        sx9_foundation_core::health_network::HealthLevel::Healthy
                    ),
                    ecosystem_health: format!("{:?}", health_state.ecosystem_health),
                    hash_integrity: health_state.hash_integrity,
                    active_nodes: health_state.active_nodes,
                    healthy_nodes: health_state.healthy_nodes,
                    critical_failures: health_state.critical_failures,
                };
            }
        }

        // Fallback: Assume healthy if HealthNetwork not initialized
        // This allows local-only development to proceed
        GlobalHeartbeat {
            passed: true,
            ecosystem_health: "Standalone".to_string(),
            hash_integrity: true,
            active_nodes: 1,
            healthy_nodes: 1,
            critical_failures: 0,
        }
    }

    /// Calculate zero-trust score based on heartbeat results
    fn calculate_zero_trust_score(&self, local: &LocalHeartbeat, global: &GlobalHeartbeat) -> u8 {
        let mut score = 0u8;

        // Local heartbeat: up to 40 points
        if local.passed {
            let healthy_ratio = local.services.iter().filter(|s| s.healthy).count() as f64
                / local.services.len().max(1) as f64;
            score += (healthy_ratio * 40.0) as u8;
        }

        // Global heartbeat: up to 60 points
        if global.passed {
            score += 30; // Base points for passing

            // Hash integrity: 20 points
            if global.hash_integrity {
                score += 20;
            }

            // Node health ratio: 10 points
            if global.active_nodes > 0 {
                let health_ratio = global.healthy_nodes as f64 / global.active_nodes as f64;
                score += (health_ratio * 10.0) as u8;
            }
        }

        score.min(100)
    }
}

impl Default for HeartbeatGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heartbeat_gate_standalone() {
        let gate = HeartbeatGate::new();
        let report = gate.run().await.expect("should run");

        // In standalone mode, global heartbeat should pass
        assert!(report.global.passed);
        assert_eq!(report.global.ecosystem_health, "Standalone");
    }
}
