//! Selective Crate Provisioner - Dynamic Resource Management
//! 
//! This module provides selective crate provisioning based on
//! operational requirements and resource availability.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Provisioning strategy
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum ProvisioningStrategy {
    OnDemand,
    Preemptive,
    Predictive,
    Hybrid,
}

/// Crate provisioning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisioningRequest {
    pub request_id: String,
    pub required_crates: Vec<String>,
    pub optional_crates: Vec<String>,
    pub strategy: ProvisioningStrategy,
    pub priority: u8,
    pub timeout_seconds: u32,
}

/// Provisioning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisioningResult {
    pub request_id: String,
    pub success: bool,
    pub provisioned_crates: Vec<String>,
    pub failed_crates: Vec<String>,
    pub resource_usage: ResourceUsage,
    pub duration_ms: u64,
}

/// Resource usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_mb: u64,
    pub cpu_percent: f32,
    pub active_crates: u32,
}

/// Selective Crate Provisioner
pub struct SelectiveCrateProvisioner {
    active_crates: HashMap<String, CrateInfo>,
    provisioning_history: Vec<ProvisioningResult>,
    strategy_config: HashMap<ProvisioningStrategy, StrategyConfig>,
}

/// Crate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateInfo {
    pub crate_name: String,
    pub status: CrateStatus,
    pub resource_usage: ResourceUsage,
    pub dependencies: Vec<String>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// Crate status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrateStatus {
    Available,
    InUse,
    Loading,
    Failed,
    Unavailable,
}

/// Strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub max_concurrent_crates: u32,
    pub resource_threshold: f32,
    pub cleanup_interval_seconds: u32,
}

impl SelectiveCrateProvisioner {
    /// Create a new selective crate provisioner
    pub fn new() -> Self {
        let mut provisioner = Self {
            active_crates: HashMap::new(),
            provisioning_history: Vec::new(),
            strategy_config: HashMap::new(),
        };
        provisioner.initialize_strategies();
        provisioner
    }

    /// Initialize provisioning strategies
    fn initialize_strategies(&mut self) {
        self.strategy_config.insert(ProvisioningStrategy::OnDemand, StrategyConfig {
            max_concurrent_crates: 10,
            resource_threshold: 0.8,
            cleanup_interval_seconds: 300,
        });

        self.strategy_config.insert(ProvisioningStrategy::Preemptive, StrategyConfig {
            max_concurrent_crates: 20,
            resource_threshold: 0.9,
            cleanup_interval_seconds: 600,
        });

        self.strategy_config.insert(ProvisioningStrategy::Predictive, StrategyConfig {
            max_concurrent_crates: 15,
            resource_threshold: 0.85,
            cleanup_interval_seconds: 450,
        });

        self.strategy_config.insert(ProvisioningStrategy::Hybrid, StrategyConfig {
            max_concurrent_crates: 25,
            resource_threshold: 0.95,
            cleanup_interval_seconds: 900,
        });
    }

    /// Provision crates based on request
    pub async fn provision_crates(&mut self, request: ProvisioningRequest) -> ProvisioningResult {
        let start_time = std::time::Instant::now();
        let mut provisioned = Vec::new();
        let mut failed = Vec::new();

        // Provision required crates first
        for crate_name in &request.required_crates {
            if self.provision_crate(crate_name).await {
                provisioned.push(crate_name.clone());
            } else {
                failed.push(crate_name.clone());
            }
        }

        // Provision optional crates if resources allow
        if failed.is_empty() {
            for crate_name in &request.optional_crates {
                if self.can_provision_crate(crate_name) && self.provision_crate(crate_name).await {
                    provisioned.push(crate_name.clone());
                }
            }
        }

        let duration = start_time.elapsed().as_millis() as u64;
        let resource_usage = self.get_current_resource_usage();

        let result = ProvisioningResult {
            request_id: request.request_id,
            success: failed.is_empty(),
            provisioned_crates: provisioned,
            failed_crates: failed,
            resource_usage,
            duration_ms: duration,
        };

        self.provisioning_history.push(result.clone());
        info!("📦 Provisioned {} crates, failed: {}", result.provisioned_crates.len(), result.failed_crates.len());

        result
    }

    /// Provision a single crate
    async fn provision_crate(&mut self, crate_name: &str) -> bool {
        // Simulate crate provisioning
        let crate_info = CrateInfo {
            crate_name: crate_name.to_string(),
            status: CrateStatus::Available,
            resource_usage: ResourceUsage {
                memory_mb: 256,
                cpu_percent: 10.0,
                active_crates: 1,
            },
            dependencies: vec![],
            last_accessed: chrono::Utc::now(),
        };

        self.active_crates.insert(crate_name.to_string(), crate_info);
        true
    }

    /// Check if crate can be provisioned
    fn can_provision_crate(&self, crate_name: &str) -> bool {
        !self.active_crates.contains_key(crate_name)
    }

    /// Get current resource usage
    fn get_current_resource_usage(&self) -> ResourceUsage {
        let mut total_memory = 0;
        let mut total_cpu = 0.0;
        let active_count = self.active_crates.len() as u32;

        for crate_info in self.active_crates.values() {
            total_memory += crate_info.resource_usage.memory_mb;
            total_cpu += crate_info.resource_usage.cpu_percent;
        }

        ResourceUsage {
            memory_mb: total_memory,
            cpu_percent: total_cpu,
            active_crates: active_count,
        }
    }

    /// Get provisioning history
    pub fn get_provisioning_history(&self) -> &[ProvisioningResult] {
        &self.provisioning_history
    }

    /// Clean up unused crates
    pub fn cleanup_unused_crates(&mut self, max_age_hours: u32) {
        let cutoff = chrono::Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let unused: Vec<String> = self.active_crates
            .iter()
            .filter(|(_, info)| info.last_accessed < cutoff)
            .map(|(name, _)| name.clone())
            .collect();

        for crate_name in unused {
            self.active_crates.remove(&crate_name);
            info!("🗑️ Cleaned up unused crate: {}", crate_name);
        }
    }
}

impl Default for SelectiveCrateProvisioner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provisioner_creation() {
        let provisioner = SelectiveCrateProvisioner::new();
        assert_eq!(provisioner.active_crates.len(), 0);
    }

    #[tokio::test]
    async fn test_crate_provisioning() {
        let mut provisioner = SelectiveCrateProvisioner::new();
        let request = ProvisioningRequest {
            request_id: "test-request".to_string(),
            required_crates: vec!["test-crate".to_string()],
            optional_crates: vec![],
            strategy: ProvisioningStrategy::OnDemand,
            priority: 1,
            timeout_seconds: 30,
        };

        let result = provisioner.provision_crates(request).await;
        assert!(result.success);
        assert_eq!(result.provisioned_crates.len(), 1);
    }
}
