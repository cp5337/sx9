use crate::temporal_intelligence::{EEIFramework, IntelligenceType, TemporalEntry, TemporalIntelManager};
use crate::usim_header::{UsimHeader, IntelSource};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, warn, debug};
use uuid::Uuid;

/// Distributed EEI Node System
/// Each node maintains its own EEI priorities and shares intelligence with TTL/LTOV
#[derive(Debug)]
pub struct DistributedEEINode {
    pub node_id: Uuid,
    pub node_name: String,
    pub location: String,
    pub specialization: NodeSpecialization,

    // Local EEI management
    pub local_eei: TemporalIntelManager,
    pub eei_priorities: HashMap<IntelligenceType, f32>, // Local priority weights

    // Network intelligence sharing
    pub known_nodes: HashMap<Uuid, EEINodeInfo>,
    pub shared_intelligence: Arc<RwLock<HashMap<[u8; 32], SharedIntelEntry>>>,
    pub subscription_matrix: HashMap<Uuid, Vec<IntelligenceType>>, // What each node wants

    // Background propagation
    pub propagation_queue: Arc<RwLock<Vec<PropagationEntry>>>,
    pub last_sync: HashMap<Uuid, u64>,

    // Performance tracking
    pub network_stats: NetworkStats,
}

/// Node specialization affects EEI priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeSpecialization {
    NetworkDefense,        // Prioritizes C2, network IOCs
    EndpointSecurity,      // Prioritizes malware, host IOCs
    ThreatHunting,         // Prioritizes TTPs, campaigns
    IncidentResponse,      // Prioritizes active threats, attribution
    ThreatIntelligence,    // Prioritizes strategic intel, profiling
    Forensics,             // Prioritizes attribution, historical data
    GeneralPurpose,        // Balanced priorities
}

/// Information about other EEI nodes in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEINodeInfo {
    pub node_id: Uuid,
    pub name: String,
    pub specialization: NodeSpecialization,
    pub endpoint: String,
    pub last_seen: u64,
    pub trust_level: f32,  // 0.0-1.0
    pub eei_capabilities: Vec<IntelligenceType>,
    pub latency_ms: u32,
}

/// Shared intelligence entry with distributed metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedIntelEntry {
    pub hash: [u8; 32],
    pub intelligence_type: IntelligenceType,
    pub confidence: f32,
    pub original_node: Uuid,
    pub creation_timestamp: u64,
    pub ttl: Duration,              // Time To Live
    pub ltov: Duration,             // Least Time of Operational Value
    pub hop_count: u8,              // How many nodes it's traversed
    pub propagation_path: Vec<Uuid>, // Path through network
    pub access_count: u32,          // How many times accessed
    pub last_validated: u64,        // Last validation timestamp
    pub validation_source: Option<String>,
}

/// Entry in propagation queue for background updates
#[derive(Debug, Clone)]
pub struct PropagationEntry {
    pub target_nodes: Vec<Uuid>,
    pub intelligence_entry: SharedIntelEntry,
    pub priority: PropagationPriority,
    pub created_at: u64,
    pub retry_count: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropagationPriority {
    Critical,    // Immediate threats (C2, ongoing attacks)
    High,        // Active threats (malware, phishing)
    Normal,      // Standard intelligence
    Low,         // Background intelligence
    Bulk,        // Mass updates
}

/// Network performance statistics
#[derive(Debug, Default)]
pub struct NetworkStats {
    pub nodes_online: usize,
    pub total_shared_entries: usize,
    pub avg_propagation_time_ms: f64,
    pub cache_hit_rate: f32,
    pub validation_success_rate: f32,
    pub network_utilization: f32,
}

impl DistributedEEINode {
    /// Create new EEI node with specialization-based priorities
    pub fn new(
        name: String,
        location: String,
        specialization: NodeSpecialization,
        endpoint: String,
    ) -> Self {
        let node_id = Uuid::new_v4();
        let eei_priorities = Self::calculate_specialization_priorities(&specialization);

        info!("🌐 Creating EEI node: {} ({:?}) at {}", name, specialization, location);

        Self {
            node_id,
            node_name: name,
            location,
            specialization,
            local_eei: TemporalIntelManager::new(),
            eei_priorities,
            known_nodes: HashMap::new(),
            shared_intelligence: Arc::new(RwLock::new(HashMap::new())),
            subscription_matrix: HashMap::new(),
            propagation_queue: Arc::new(RwLock::new(Vec::new())),
            last_sync: HashMap::new(),
            network_stats: NetworkStats::default(),
        }
    }

    /// Calculate EEI priorities based on node specialization
    fn calculate_specialization_priorities(spec: &NodeSpecialization) -> HashMap<IntelligenceType, f32> {
        let mut priorities = HashMap::new();

        match spec {
            NodeSpecialization::NetworkDefense => {
                priorities.insert(IntelligenceType::ActiveC2Communication, 10.0);
                priorities.insert(IntelligenceType::ThreatActorInfrastructure, 9.0);
                priorities.insert(IntelligenceType::OngoingDataExfiltration, 9.5);
                priorities.insert(IntelligenceType::MalwareIOCs, 7.0);
                priorities.insert(IntelligenceType::ThreatActorTTP, 6.0);
                priorities.insert(IntelligenceType::ThreatActorProfiling, 3.0);
            },
            NodeSpecialization::EndpointSecurity => {
                priorities.insert(IntelligenceType::MalwareIOCs, 10.0);
                priorities.insert(IntelligenceType::NewVulnerabilityExploits, 9.0);
                priorities.insert(IntelligenceType::MalwareFamilyEvolution, 8.0);
                priorities.insert(IntelligenceType::CompromisedCredentials, 8.5);
                priorities.insert(IntelligenceType::ActiveC2Communication, 7.0);
            },
            NodeSpecialization::ThreatHunting => {
                priorities.insert(IntelligenceType::ThreatActorTTP, 10.0);
                priorities.insert(IntelligenceType::CampaignPatterns, 9.5);
                priorities.insert(IntelligenceType::MalwareFamilyEvolution, 8.0);
                priorities.insert(IntelligenceType::ThreatActorProfiling, 8.5);
                priorities.insert(IntelligenceType::InfrastructurePatterns, 7.5);
            },
            NodeSpecialization::IncidentResponse => {
                priorities.insert(IntelligenceType::BreachInProgress, 10.0);
                priorities.insert(IntelligenceType::OngoingDataExfiltration, 10.0);
                priorities.insert(IntelligenceType::ActiveC2Communication, 9.5);
                priorities.insert(IntelligenceType::CompromisedCredentials, 9.0);
                priorities.insert(IntelligenceType::AttributionEvidence, 7.0);
            },
            NodeSpecialization::ThreatIntelligence => {
                priorities.insert(IntelligenceType::ThreatActorProfiling, 10.0);
                priorities.insert(IntelligenceType::GeopoliticalContext, 9.0);
                priorities.insert(IntelligenceType::StrategicWarnings, 8.5);
                priorities.insert(IntelligenceType::AttributionEvidence, 9.5);
                priorities.insert(IntelligenceType::IndustryTrends, 7.0);
            },
            NodeSpecialization::Forensics => {
                priorities.insert(IntelligenceType::AttributionEvidence, 10.0);
                priorities.insert(IntelligenceType::BaselineAnalytics, 9.0);
                priorities.insert(IntelligenceType::TTechnologyTrends, 8.0);
                priorities.insert(IntelligenceType::ThreatActorTTP, 8.5);
            },
            NodeSpecialization::GeneralPurpose => {
                // Balanced priorities for all intelligence types
                for intel_type in [
                    IntelligenceType::ActiveC2Communication,
                    IntelligenceType::LivePhishingCampaign,
                    IntelligenceType::MalwareIOCs,
                    IntelligenceType::ThreatActorTTP,
                    IntelligenceType::ThreatActorProfiling,
                ] {
                    priorities.insert(intel_type, 5.0);
                }
            },
        }

        priorities
    }

    /// Register another EEI node in the network
    pub async fn register_node(&mut self, node_info: EEINodeInfo) {
        info!("🔗 Registering EEI node: {} ({:?})", node_info.name, node_info.specialization);

        // Set up subscription matrix based on specializations
        let subscriptions = self.calculate_subscription_interests(&node_info.specialization);
        self.subscription_matrix.insert(node_info.node_id, subscriptions);

        self.known_nodes.insert(node_info.node_id, node_info);
        self.network_stats.nodes_online = self.known_nodes.len();
    }

    /// Calculate what intelligence types a node specialization is interested in
    fn calculate_subscription_interests(&self, specialization: &NodeSpecialization) -> Vec<IntelligenceType> {
        match specialization {
            NodeSpecialization::NetworkDefense => vec![
                IntelligenceType::ActiveC2Communication,
                IntelligenceType::ThreatActorInfrastructure,
                IntelligenceType::OngoingDataExfiltration,
            ],
            NodeSpecialization::EndpointSecurity => vec![
                IntelligenceType::MalwareIOCs,
                IntelligenceType::NewVulnerabilityExploits,
                IntelligenceType::CompromisedCredentials,
            ],
            NodeSpecialization::ThreatHunting => vec![
                IntelligenceType::ThreatActorTTP,
                IntelligenceType::CampaignPatterns,
                IntelligenceType::ThreatActorProfiling,
            ],
            NodeSpecialization::IncidentResponse => vec![
                IntelligenceType::BreachInProgress,
                IntelligenceType::OngoingDataExfiltration,
                IntelligenceType::ActiveC2Communication,
                IntelligenceType::CompromisedCredentials,
            ],
            _ => vec![], // Others subscribe to all
        }
    }

    /// Share intelligence with network (background propagation)
    pub async fn share_intelligence(
        &mut self,
        hash: [u8; 32],
        intelligence_type: IntelligenceType,
        confidence: f32,
        ttl: Duration,
        ltov: Duration,
    ) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let shared_entry = SharedIntelEntry {
            hash,
            intelligence_type: intelligence_type.clone(),
            confidence,
            original_node: self.node_id,
            creation_timestamp: current_time,
            ttl,
            ltov,
            hop_count: 0,
            propagation_path: vec![self.node_id],
            access_count: 0,
            last_validated: current_time,
            validation_source: Some(self.node_name.clone()),
        };

        // Add to local shared intelligence
        {
            let mut shared = self.shared_intelligence.write().await;
            shared.insert(hash, shared_entry.clone());
        }

        // Determine propagation priority
        let priority = self.determine_propagation_priority(&intelligence_type);

        // Find interested nodes
        let target_nodes = self.find_interested_nodes(&intelligence_type);

        if !target_nodes.is_empty() {
            let propagation_entry = PropagationEntry {
                target_nodes,
                intelligence_entry: shared_entry,
                priority,
                created_at: current_time,
                retry_count: 0,
            };

            // Add to propagation queue
            let mut queue = self.propagation_queue.write().await;
            queue.push(propagation_entry);

            // Sort by priority (critical first)
            queue.sort_by(|a, b| a.priority.cmp(&b.priority));
        }

        debug!("📤 Queued intelligence for propagation: {:02x?}", &hash[0..8]);
    }

    /// Determine propagation priority based on intelligence type
    fn determine_propagation_priority(&self, intelligence_type: &IntelligenceType) -> PropagationPriority {
        match intelligence_type {
            IntelligenceType::ActiveC2Communication |
            IntelligenceType::BreachInProgress |
            IntelligenceType::OngoingDataExfiltration => PropagationPriority::Critical,

            IntelligenceType::LivePhishingCampaign |
            IntelligenceType::CompromisedCredentials |
            IntelligenceType::NewVulnerabilityExploits => PropagationPriority::High,

            IntelligenceType::MalwareIOCs |
            IntelligenceType::ThreatActorInfrastructure |
            IntelligenceType::ThreatActorTTP => PropagationPriority::Normal,

            IntelligenceType::ThreatActorProfiling |
            IntelligenceType::CampaignPatterns => PropagationPriority::Low,

            _ => PropagationPriority::Bulk,
        }
    }

    /// Find nodes interested in specific intelligence type
    fn find_interested_nodes(&self, intelligence_type: &IntelligenceType) -> Vec<Uuid> {
        let mut interested = Vec::new();

        for (node_id, subscriptions) in &self.subscription_matrix {
            if subscriptions.is_empty() || subscriptions.contains(intelligence_type) {
                interested.push(*node_id);
            }
        }

        interested
    }

    /// Process background propagation queue
    pub async fn process_propagation_queue(&mut self) {
        let mut queue = self.propagation_queue.write().await;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut processed_indices = Vec::new();

        for (i, entry) in queue.iter_mut().enumerate() {
            // Skip if too many retries
            if entry.retry_count > 3 {
                processed_indices.push(i);
                continue;
            }

            // Check if still within TTL
            let age = current_time.saturating_sub(entry.intelligence_entry.creation_timestamp);
            if Duration::from_secs(age) > entry.intelligence_entry.ttl {
                processed_indices.push(i);
                continue;
            }

            // Attempt to propagate to target nodes
            let mut successful_nodes = Vec::new();
            for target_node in &entry.target_nodes {
                if let Some(node_info) = self.known_nodes.get(target_node) {
                    // In real implementation, this would send via network
                    debug!("📡 Propagating to node: {} ({})", node_info.name, node_info.endpoint);
                    successful_nodes.push(*target_node);
                }
            }

            if successful_nodes.len() == entry.target_nodes.len() {
                // All nodes reached successfully
                processed_indices.push(i);
            } else {
                // Retry failed nodes
                entry.retry_count += 1;
                entry.target_nodes.retain(|node| !successful_nodes.contains(node));
            }
        }

        // Remove processed entries (in reverse order to maintain indices)
        for &i in processed_indices.iter().rev() {
            queue.remove(i);
        }

        debug!("🔄 Processed {} propagation entries, {} remaining",
               processed_indices.len(), queue.len());
    }

    /// Clean up expired intelligence entries (background task)
    pub async fn cleanup_expired_intelligence(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut shared = self.shared_intelligence.write().await;
        let initial_count = shared.len();

        shared.retain(|_hash, entry| {
            let age = current_time.saturating_sub(entry.creation_timestamp);
            Duration::from_secs(age) <= entry.ttl
        });

        let removed_count = initial_count - shared.len();
        if removed_count > 0 {
            info!("🧹 Cleaned up {} expired intelligence entries", removed_count);
        }

        self.network_stats.total_shared_entries = shared.len();
    }

    /// Query intelligence with network fallback
    pub async fn query_intelligence(&mut self, hash: [u8; 32]) -> Option<SharedIntelEntry> {
        // Check local shared intelligence first
        {
            let mut shared = self.shared_intelligence.write().await;
            if let Some(entry) = shared.get_mut(&hash) {
                entry.access_count += 1;
                return Some(entry.clone());
            }
        }

        // Query known nodes if not found locally
        for (node_id, node_info) in &self.known_nodes {
            // In real implementation, this would query via network
            debug!("🔍 Querying node {} for hash {:02x?}", node_info.name, &hash[0..8]);

            // Simulate network query delay based on node latency
            if node_info.latency_ms < 100 {
                // Fast node - might have the intelligence
                // This would be actual network call in production
            }
        }

        None
    }

    /// Get node's current EEI status summary
    pub async fn get_eei_status(&self) -> EEINodeStatus {
        let shared = self.shared_intelligence.read().await;
        let queue = self.propagation_queue.read().await;

        // Calculate intelligence distribution by type
        let mut intel_distribution = HashMap::new();
        for entry in shared.values() {
            *intel_distribution.entry(entry.intelligence_type.clone()).or_insert(0) += 1;
        }

        EEINodeStatus {
            node_id: self.node_id,
            node_name: self.node_name.clone(),
            specialization: self.specialization.clone(),
            local_intelligence_count: shared.len(),
            pending_propagations: queue.len(),
            known_nodes_count: self.known_nodes.len(),
            intelligence_distribution: intel_distribution,
            network_stats: self.network_stats.clone(),
        }
    }
}

/// Status summary for an EEI node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEINodeStatus {
    pub node_id: Uuid,
    pub node_name: String,
    pub specialization: NodeSpecialization,
    pub local_intelligence_count: usize,
    pub pending_propagations: usize,
    pub known_nodes_count: usize,
    pub intelligence_distribution: HashMap<IntelligenceType, usize>,
    pub network_stats: NetworkStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = DistributedEEINode::new(
            "TestNode".to_string(),
            "DC1".to_string(),
            NodeSpecialization::NetworkDefense,
            "localhost:8080".to_string(),
        );

        assert_eq!(node.node_name, "TestNode");
        assert_eq!(node.location, "DC1");
        assert!(matches!(node.specialization, NodeSpecialization::NetworkDefense));
    }

    #[test]
    fn test_specialization_priorities() {
        let priorities = DistributedEEINode::calculate_specialization_priorities(
            &NodeSpecialization::NetworkDefense
        );

        assert!(priorities.get(&IntelligenceType::ActiveC2Communication).unwrap() > &8.0);
        assert!(priorities.get(&IntelligenceType::ThreatActorProfiling).unwrap() < &5.0);
    }

    #[tokio::test]
    async fn test_intelligence_sharing() {
        let mut node = DistributedEEINode::new(
            "TestNode".to_string(),
            "DC1".to_string(),
            NodeSpecialization::GeneralPurpose,
            "localhost:8080".to_string(),
        );

        let test_hash = [1u8; 32];
        node.share_intelligence(
            test_hash,
            IntelligenceType::ActiveC2Communication,
            0.9,
            Duration::from_secs(3600),
            Duration::from_secs(900),
        ).await;

        let shared = node.shared_intelligence.read().await;
        assert!(shared.contains_key(&test_hash));
    }
}