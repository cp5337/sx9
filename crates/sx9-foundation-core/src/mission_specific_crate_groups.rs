//! Mission-Specific Crate Groups - Optimized Resource Allocation
//! 
//! This module defines optimized crate groupings for different mission types
//! with resource profiles and operational requirements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Mission-specific crate group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionCrateGroup {
    pub group_id: String,
    pub mission_type: String,
    pub core_crates: Vec<String>,
    pub optional_crates: Vec<String>,
    pub ephemeral_crates: Vec<String>,
    pub resource_profile: ResourceProfile,
    pub dependencies: Vec<String>,
    pub priority: u8,
}

/// Resource profile for mission group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProfile {
    pub estimated_memory_mb: u64,
    pub estimated_cpu_cores: u32,
    pub estimated_storage_gb: u64,
    pub estimated_network_mbps: u32,
    pub startup_time_seconds: u32,
}

/// Mission Crate Group Manager
pub struct MissionCrateGroupManager {
    mission_groups: HashMap<String, MissionCrateGroup>,
}

impl MissionCrateGroupManager {
    /// Create a new mission crate group manager
    pub fn new() -> Self {
        let mut manager = Self {
            mission_groups: HashMap::new(),
        };
        manager.initialize_default_groups();
        manager
    }

    /// Initialize default mission groups
    fn initialize_default_groups(&mut self) {
        // Port Scanning Mission Group
        let port_scan_group = MissionCrateGroup {
            group_id: "port-scanning".to_string(),
            mission_type: "portscan".to_string(),
            core_crates: vec![
                "ctas-port-manager".to_string(),
                "ctas-network-scanner".to_string(),
                "ctas-vulnerability-scanner".to_string(),
            ],
            optional_crates: vec![
                "ctas-service-detector".to_string(),
                "ctas-banner-grabber".to_string(),
            ],
            ephemeral_crates: vec![
                "ctas-scan-optimizer".to_string(),
                "ctas-result-aggregator".to_string(),
            ],
            resource_profile: ResourceProfile {
                estimated_memory_mb: 1024,
                estimated_cpu_cores: 4,
                estimated_storage_gb: 5,
                estimated_network_mbps: 100,
                startup_time_seconds: 30,
            },
            dependencies: vec![],
            priority: 1,
        };

        // OSINT Mission Group
        let osint_group = MissionCrateGroup {
            group_id: "osint-gathering".to_string(),
            mission_type: "osint".to_string(),
            core_crates: vec![
                "ctas-intelligence-hub".to_string(),
                "ctas-osint-collector".to_string(),
                "ctas-data-processor".to_string(),
            ],
            optional_crates: vec![
                "ctas-social-media-analyzer".to_string(),
                "ctas-domain-intelligence".to_string(),
            ],
            ephemeral_crates: vec![
                "ctas-data-enricher".to_string(),
                "ctas-pattern-analyzer".to_string(),
            ],
            resource_profile: ResourceProfile {
                estimated_memory_mb: 2048,
                estimated_cpu_cores: 8,
                estimated_storage_gb: 20,
                estimated_network_mbps: 200,
                startup_time_seconds: 60,
            },
            dependencies: vec![],
            priority: 2,
        };

        // Threat Hunting Mission Group
        let threat_hunting_group = MissionCrateGroup {
            group_id: "threat-hunting".to_string(),
            mission_type: "threat_hunting".to_string(),
            core_crates: vec![
                "ctas-threat-detector".to_string(),
                "ctas-malware-analyzer".to_string(),
                "ctas-behavior-analyzer".to_string(),
            ],
            optional_crates: vec![
                "ctas-signature-matcher".to_string(),
                "ctas-anomaly-detector".to_string(),
            ],
            ephemeral_crates: vec![
                "ctas-threat-correlator".to_string(),
                "ctas-incident-responder".to_string(),
            ],
            resource_profile: ResourceProfile {
                estimated_memory_mb: 4096,
                estimated_cpu_cores: 16,
                estimated_storage_gb: 50,
                estimated_network_mbps: 500,
                startup_time_seconds: 120,
            },
            dependencies: vec![],
            priority: 3,
        };

        // Incident Response Mission Group
        let incident_response_group = MissionCrateGroup {
            group_id: "incident-response".to_string(),
            mission_type: "incident_response".to_string(),
            core_crates: vec![
                "ctas-incident-manager".to_string(),
                "ctas-forensic-analyzer".to_string(),
                "ctas-evidence-collector".to_string(),
            ],
            optional_crates: vec![
                "ctas-timeline-analyzer".to_string(),
                "ctas-memory-analyzer".to_string(),
            ],
            ephemeral_crates: vec![
                "ctas-remediation-engine".to_string(),
                "ctas-recovery-manager".to_string(),
            ],
            resource_profile: ResourceProfile {
                estimated_memory_mb: 8192,
                estimated_cpu_cores: 32,
                estimated_storage_gb: 100,
                estimated_network_mbps: 1000,
                startup_time_seconds: 180,
            },
            dependencies: vec![],
            priority: 4,
        };

        // Red Team Mission Group
        let red_team_group = MissionCrateGroup {
            group_id: "red-team".to_string(),
            mission_type: "red_team".to_string(),
            core_crates: vec![
                "ctas-penetration-tester".to_string(),
                "ctas-exploit-framework".to_string(),
                "ctas-payload-generator".to_string(),
            ],
            optional_crates: vec![
                "ctas-social-engineer".to_string(),
                "ctas-physical-security".to_string(),
            ],
            ephemeral_crates: vec![
                "ctas-attack-simulator".to_string(),
                "ctas-report-generator".to_string(),
            ],
            resource_profile: ResourceProfile {
                estimated_memory_mb: 6144,
                estimated_cpu_cores: 24,
                estimated_storage_gb: 75,
                estimated_network_mbps: 750,
                startup_time_seconds: 150,
            },
            dependencies: vec![],
            priority: 5,
        };

        // Add all groups
        self.mission_groups.insert(port_scan_group.group_id.clone(), port_scan_group);
        self.mission_groups.insert(osint_group.group_id.clone(), osint_group);
        self.mission_groups.insert(threat_hunting_group.group_id.clone(), threat_hunting_group);
        self.mission_groups.insert(incident_response_group.group_id.clone(), incident_response_group);
        self.mission_groups.insert(red_team_group.group_id.clone(), red_team_group);

        info!("📋 Initialized {} mission-specific crate groups", self.mission_groups.len());
    }

    /// Get mission group by type
    pub fn get_mission_group(&self, mission_type: &str) -> Option<&MissionCrateGroup> {
        self.mission_groups.values().find(|group| group.mission_type == mission_type)
    }

    /// Get all mission groups
    pub fn get_all_groups(&self) -> Vec<&MissionCrateGroup> {
        self.mission_groups.values().collect()
    }

    /// Estimate total resources for multiple mission types
    pub fn estimate_total_resources(&self, mission_types: &[String]) -> ResourceProfile {
        let mut total_memory = 0;
        let mut total_cpu = 0;
        let mut total_storage = 0;
        let mut total_network = 0;
        let mut max_startup_time = 0;

        for mission_type in mission_types {
            if let Some(group) = self.get_mission_group(mission_type) {
                total_memory += group.resource_profile.estimated_memory_mb;
                total_cpu += group.resource_profile.estimated_cpu_cores;
                total_storage += group.resource_profile.estimated_storage_gb;
                total_network += group.resource_profile.estimated_network_mbps;
                max_startup_time = max_startup_time.max(group.resource_profile.startup_time_seconds);
            }
        }

        ResourceProfile {
            estimated_memory_mb: total_memory,
            estimated_cpu_cores: total_cpu,
            estimated_storage_gb: total_storage,
            estimated_network_mbps: total_network,
            startup_time_seconds: max_startup_time,
        }
    }

    /// Add custom mission group
    pub fn add_mission_group(&mut self, group: MissionCrateGroup) {
        info!("📋 Adding custom mission group: {}", group.group_id);
        self.mission_groups.insert(group.group_id.clone(), group);
    }

    /// Remove mission group
    pub fn remove_mission_group(&mut self, group_id: &str) -> bool {
        if self.mission_groups.remove(group_id).is_some() {
            info!("🗑️ Removed mission group: {}", group_id);
            true
        } else {
            false
        }
    }
}

impl Default for MissionCrateGroupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = MissionCrateGroupManager::new();
        assert!(!manager.mission_groups.is_empty());
    }

    #[test]
    fn test_get_mission_group() {
        let manager = MissionCrateGroupManager::new();
        let group = manager.get_mission_group("portscan");
        assert!(group.is_some());
        assert_eq!(group.unwrap().mission_type, "portscan");
    }

    #[test]
    fn test_resource_estimation() {
        let manager = MissionCrateGroupManager::new();
        let mission_types = vec!["portscan".to_string(), "osint".to_string()];
        let resources = manager.estimate_total_resources(&mission_types);
        assert!(resources.estimated_memory_mb > 0);
        assert!(resources.estimated_cpu_cores > 0);
    }
}
