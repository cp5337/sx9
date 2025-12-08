//! Ephemeral Toolchain Manager - Temporary Resource Management
//! 
//! This module manages temporary, task-specific toolchain deployments
//! with lifecycle management and resource optimization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tracing::{info, warn};

/// Ephemeral toolchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralToolchain {
    pub toolchain_id: String,
    pub mission_type: String,
    pub tools: Vec<EphemeralTool>,
    pub lifecycle: ToolchainLifecycle,
    pub resource_limits: ResourceLimits,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Individual ephemeral tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralTool {
    pub tool_name: String,
    pub tool_version: String,
    pub resource_usage: ResourceUsage,
    pub dependencies: Vec<String>,
    pub status: ToolStatus,
}

/// Toolchain lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainLifecycle {
    pub state: LifecycleState,
    pub auto_cleanup: bool,
    pub max_duration_minutes: u32,
    pub cleanup_on_completion: bool,
}

/// Lifecycle states
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum LifecycleState {
    Creating,
    Active,
    Suspended,
    Terminating,
    Terminated,
}

/// Resource limits for toolchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_percent: f32,
    pub max_network_mbps: f32,
    pub max_disk_gb: u64,
}

/// Current resource usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_mb: u64,
    pub cpu_percent: f32,
    pub network_mbps: f32,
    pub disk_gb: u64,
}

/// Tool status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolStatus {
    Loading,
    Ready,
    Running,
    Completed,
    Failed(String),
}

/// Ephemeral Toolchain Manager
pub struct EphemeralToolchainManager {
    active_toolchains: HashMap<String, EphemeralToolchain>,
    mission_templates: HashMap<String, Vec<String>>,
}

impl EphemeralToolchainManager {
    /// Create a new ephemeral toolchain manager
    pub fn new() -> Self {
        Self {
            active_toolchains: HashMap::new(),
            mission_templates: HashMap::new(),
        }
    }

    /// Create an ephemeral toolchain for a mission
    pub fn create_ephemeral_toolchain(&mut self, mission_type: &str, duration_minutes: u32) -> String {
        let toolchain_id = format!("ephemeral-{}-{}", mission_type, chrono::Utc::now().timestamp());
        
        let tools = self.select_tools_for_mission(mission_type);
        
        let toolchain = EphemeralToolchain {
            toolchain_id: toolchain_id.clone(),
            mission_type: mission_type.to_string(),
            tools,
            lifecycle: ToolchainLifecycle {
                state: LifecycleState::Creating,
                auto_cleanup: true,
                max_duration_minutes: duration_minutes,
                cleanup_on_completion: true,
            },
            resource_limits: ResourceLimits {
                max_memory_mb: 2048,
                max_cpu_percent: 50.0,
                max_network_mbps: 100.0,
                max_disk_gb: 10,
            },
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::minutes(duration_minutes as i64),
        };
        
        self.active_toolchains.insert(toolchain_id.clone(), toolchain);
        info!("🔧 Created ephemeral toolchain: {} for mission: {}", toolchain_id, mission_type);
        
        toolchain_id
    }

    /// Select appropriate tools for mission type
    fn select_tools_for_mission(&self, mission_type: &str) -> Vec<EphemeralTool> {
        let mut tools = Vec::new();
        
        match mission_type {
            "portscan" => {
                tools.push(EphemeralTool {
                    tool_name: "nmap".to_string(),
                    tool_version: "7.94".to_string(),
                    resource_usage: ResourceUsage {
                        memory_mb: 256,
                        cpu_percent: 25.0,
                        network_mbps: 50.0,
                        disk_gb: 1,
                    },
                    dependencies: vec![],
                    status: ToolStatus::Loading,
                });
            }
            "osint" => {
                tools.push(EphemeralTool {
                    tool_name: "theharvester".to_string(),
                    tool_version: "4.0.3".to_string(),
                    resource_usage: ResourceUsage {
                        memory_mb: 512,
                        cpu_percent: 30.0,
                        network_mbps: 75.0,
                        disk_gb: 2,
                    },
                    dependencies: vec!["python3".to_string()],
                    status: ToolStatus::Loading,
                });
            }
            "threat_hunting" => {
                tools.push(EphemeralTool {
                    tool_name: "yara".to_string(),
                    tool_version: "4.3.1".to_string(),
                    resource_usage: ResourceUsage {
                        memory_mb: 1024,
                        cpu_percent: 40.0,
                        network_mbps: 25.0,
                        disk_gb: 5,
                    },
                    dependencies: vec![],
                    status: ToolStatus::Loading,
                });
            }
            _ => {
                // Default tools for unknown mission types
                tools.push(EphemeralTool {
                    tool_name: "generic_tool".to_string(),
                    tool_version: "1.0.0".to_string(),
                    resource_usage: ResourceUsage {
                        memory_mb: 256,
                        cpu_percent: 20.0,
                        network_mbps: 25.0,
                        disk_gb: 1,
                    },
                    dependencies: vec![],
                    status: ToolStatus::Loading,
                });
            }
        }
        
        tools
    }

    /// Get active toolchain by ID
    pub fn get_toolchain(&self, toolchain_id: &str) -> Option<&EphemeralToolchain> {
        self.active_toolchains.get(toolchain_id)
    }

    /// Update toolchain lifecycle state
    pub fn update_toolchain_state(&mut self, toolchain_id: &str, state: LifecycleState) -> bool {
        if let Some(toolchain) = self.active_toolchains.get_mut(toolchain_id) {
            toolchain.lifecycle.state = state;
            info!("🔄 Updated toolchain {} state: {:?}", toolchain_id, state);
            true
        } else {
            warn!("⚠️ Toolchain not found: {}", toolchain_id);
            false
        }
    }

    /// Clean up expired toolchains
    pub fn cleanup_expired_toolchains(&mut self) {
        let now = Utc::now();
        let expired: Vec<String> = self.active_toolchains
            .iter()
            .filter(|(_, tc)| tc.expires_at < now)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired {
            self.active_toolchains.remove(&id);
            info!("🗑️ Cleaned up expired toolchain: {}", id);
        }
    }

    /// Get all active toolchains
    pub fn get_active_toolchains(&self) -> Vec<&EphemeralToolchain> {
        self.active_toolchains.values().collect()
    }

    /// Add mission template
    pub fn add_mission_template(&mut self, mission_type: &str, tools: Vec<String>) {
        let tools_count = tools.len();
        self.mission_templates.insert(mission_type.to_string(), tools);
        info!("📋 Added mission template: {} with {} tools", mission_type, tools_count);
    }
}

impl Default for EphemeralToolchainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = EphemeralToolchainManager::new();
        assert_eq!(manager.active_toolchains.len(), 0);
    }

    #[test]
    fn test_toolchain_creation() {
        let mut manager = EphemeralToolchainManager::new();
        let toolchain_id = manager.create_ephemeral_toolchain("portscan", 30);
        assert!(manager.active_toolchains.contains_key(&toolchain_id));
    }
}
