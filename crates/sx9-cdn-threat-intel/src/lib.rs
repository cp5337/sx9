#![allow(clippy::items_after_statements)]
#![allow(clippy::format_push_string)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unused_async)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unused_self)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
//! CTAS-7 Threat Intelligence CDN
//!
//! Edge distribution network for threat intelligence content:
//! - DSL Playbooks (converted from YAML)
//! - MITRE ATT&CK/CAR mappings
//! - Wazuh detection rules
//! - Sigma rules and threat signatures
//! - OSINT feeds and indicators
//!
//! Architecture: Content-addressed storage with SCH-based routing

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// Core Types
// ═══════════════════════════════════════════════════════════════════════════

/// Threat intel content types supported by the CDN
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    /// DSL playbook (converted from YAML)
    DslPlaybook,
    /// MITRE ATT&CK technique
    MitreAttack,
    /// MITRE CAR analytic
    MitreCar,
    /// Wazuh detection rule
    WazuhRule,
    /// Sigma detection rule
    SigmaRule,
    /// YARA rule
    YaraRule,
    /// Indicator of Compromise
    Ioc,
    /// OSINT feed entry
    OsintFeed,
    /// Threat actor profile
    ThreatActor,
    /// Campaign tracking
    Campaign,
    /// Custom playbook
    CustomPlaybook,
}

/// Threat intel content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelContent {
    /// Content-addressed hash (SCH format)
    pub sch: String,
    /// Content type
    pub content_type: ContentType,
    /// Original source (e.g., "mitre-car", "wazuh", "sigma")
    pub source: String,
    /// Content payload (JSON or DSL)
    pub payload: serde_json::Value,
    /// MITRE ATT&CK mappings if applicable
    pub mitre_mappings: Vec<MitreMapping>,
    /// Version/revision
    pub version: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Tags for filtering
    pub tags: Vec<String>,
    /// Severity level (1-10)
    pub severity: u8,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// MITRE ATT&CK mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMapping {
    /// Technique ID (e.g., T1059.001)
    pub technique_id: String,
    /// Technique name
    pub technique_name: String,
    /// Tactic (e.g., "execution", "persistence")
    pub tactic: String,
    /// Sub-technique if applicable
    pub sub_technique: Option<String>,
}

/// Wazuh rule integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WazuhRule {
    /// Rule ID
    pub rule_id: u32,
    /// Rule level (0-15)
    pub level: u8,
    /// Description
    pub description: String,
    /// Groups
    pub groups: Vec<String>,
    /// MITRE mappings
    pub mitre: Vec<MitreMapping>,
    /// Rule XML content
    pub xml_content: String,
}

/// Plasma integration config (for security orchestration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaConfig {
    /// Plasma endpoint
    pub endpoint: String,
    /// Sync interval in seconds
    pub sync_interval: u64,
    /// Content types to sync
    pub content_types: Vec<ContentType>,
}

// ═══════════════════════════════════════════════════════════════════════════
// CDN Node
// ═══════════════════════════════════════════════════════════════════════════

/// Threat Intel CDN Node
pub struct ThreatIntelCdnNode {
    /// Node identifier
    pub node_id: String,
    /// Content cache (SCH -> Content)
    cache: Arc<DashMap<String, ThreatIntelContent>>,
    /// Index by content type
    type_index: Arc<DashMap<ContentType, Vec<String>>>,
    /// Index by MITRE technique
    mitre_index: Arc<DashMap<String, Vec<String>>>,
    /// Upstream nodes for cache miss
    upstreams: Arc<RwLock<Vec<String>>>,
    /// Local sled database for persistence
    db: Option<sled::Db>,
    /// Wazuh integration state
    wazuh_rules: Arc<DashMap<u32, WazuhRule>>,
    /// Node metrics
    metrics: Arc<RwLock<NodeMetrics>>,
}

/// Node performance metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_requests: u64,
    pub content_count: u64,
    pub wazuh_rules_count: u64,
    pub last_sync: Option<DateTime<Utc>>,
    pub uptime_seconds: u64,
}

impl ThreatIntelCdnNode {
    /// Create new CDN node
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            cache: Arc::new(DashMap::new()),
            type_index: Arc::new(DashMap::new()),
            mitre_index: Arc::new(DashMap::new()),
            upstreams: Arc::new(RwLock::new(Vec::new())),
            db: None,
            wazuh_rules: Arc::new(DashMap::new()),
            metrics: Arc::new(RwLock::new(NodeMetrics::default())),
        }
    }

    /// Initialize with persistent storage
    pub fn with_storage(mut self, db_path: &str) -> anyhow::Result<Self> {
        self.db = Some(sled::open(db_path)?);
        Ok(self)
    }

    /// Add upstream node
    pub async fn add_upstream(&self, upstream_url: String) {
        let mut upstreams = self.upstreams.write().await;
        upstreams.push(upstream_url);
    }

    /// Store threat intel content
    pub async fn store(&self, content: ThreatIntelContent) -> anyhow::Result<String> {
        let sch = content.sch.clone();

        // Update type index
        self.type_index
            .entry(content.content_type.clone())
            .or_default()
            .push(sch.clone());

        // Update MITRE index
        for mapping in &content.mitre_mappings {
            self.mitre_index
                .entry(mapping.technique_id.clone())
                .or_default()
                .push(sch.clone());
        }

        // Store in cache
        self.cache.insert(sch.clone(), content.clone());

        // Persist to sled if available
        if let Some(db) = &self.db {
            let key = format!("content:{}", sch);
            let value = serde_json::to_vec(&content)?;
            db.insert(key.as_bytes(), value)?;
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.content_count = self.cache.len() as u64;
        }

        Ok(sch)
    }

    /// Get content by SCH
    pub async fn get(&self, sch: &str) -> Option<ThreatIntelContent> {
        // Try cache first
        if let Some(content) = self.cache.get(sch) {
            let mut metrics = self.metrics.write().await;
            metrics.cache_hits += 1;
            metrics.total_requests += 1;
            return Some(content.clone());
        }

        // Try persistent storage
        if let Some(db) = &self.db {
            let key = format!("content:{}", sch);
            if let Ok(Some(data)) = db.get(key.as_bytes()) {
                if let Ok(content) = serde_json::from_slice::<ThreatIntelContent>(&data) {
                    self.cache.insert(sch.to_string(), content.clone());
                    return Some(content);
                }
            }
        }

        let mut metrics = self.metrics.write().await;
        metrics.cache_misses += 1;
        metrics.total_requests += 1;

        None
    }

    /// Query by content type
    pub async fn query_by_type(&self, content_type: ContentType) -> Vec<ThreatIntelContent> {
        let mut results = Vec::new();
        if let Some(schs) = self.type_index.get(&content_type) {
            for sch in schs.iter() {
                if let Some(content) = self.get(sch).await {
                    results.push(content);
                }
            }
        }
        results
    }

    /// Query by MITRE technique
    pub async fn query_by_mitre(&self, technique_id: &str) -> Vec<ThreatIntelContent> {
        let mut results = Vec::new();
        if let Some(schs) = self.mitre_index.get(technique_id) {
            for sch in schs.iter() {
                if let Some(content) = self.get(sch).await {
                    results.push(content);
                }
            }
        }
        results
    }

    /// Store Wazuh rule
    pub async fn store_wazuh_rule(&self, rule: WazuhRule) {
        self.wazuh_rules.insert(rule.rule_id, rule);
        let mut metrics = self.metrics.write().await;
        metrics.wazuh_rules_count = self.wazuh_rules.len() as u64;
    }

    /// Get Wazuh rule by ID
    pub fn get_wazuh_rule(&self, rule_id: u32) -> Option<WazuhRule> {
        self.wazuh_rules.get(&rule_id).map(|r| r.clone())
    }

    /// Get all Wazuh rules
    pub fn get_all_wazuh_rules(&self) -> Vec<WazuhRule> {
        self.wazuh_rules.iter().map(|r| r.clone()).collect()
    }

    /// Get node metrics
    pub async fn get_metrics(&self) -> NodeMetrics {
        self.metrics.read().await.clone()
    }

    /// Health check
    pub async fn health_check(&self) -> HealthStatus {
        let metrics = self.metrics.read().await;
        HealthStatus {
            status: "healthy".to_string(),
            node_id: self.node_id.clone(),
            content_count: metrics.content_count,
            cache_hit_ratio: if metrics.total_requests > 0 {
                metrics.cache_hits as f64 / metrics.total_requests as f64
            } else {
                0.0
            },
            wazuh_rules_count: metrics.wazuh_rules_count,
        }
    }
}

/// Health status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub node_id: String,
    pub content_count: u64,
    pub cache_hit_ratio: f64,
    pub wazuh_rules_count: u64,
}

// ═══════════════════════════════════════════════════════════════════════════
// DSL Conversion Helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Convert YAML threat content to DSL-compatible format
pub fn yaml_to_threat_intel(
    yaml_content: &str,
    source: &str,
    content_type: ContentType,
) -> anyhow::Result<ThreatIntelContent> {
    let payload: serde_json::Value = serde_yaml::from_str(yaml_content)?;

    // Generate SCH from content hash
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(yaml_content.as_bytes());
    let hash = hasher.finalize();
    let sch = hex::encode(&hash[..16]); // First 16 bytes for SCH

    // Extract MITRE mappings if present
    let mitre_mappings = extract_mitre_mappings(&payload);

    // Extract tags
    let tags = extract_tags(&payload);

    Ok(ThreatIntelContent {
        sch,
        content_type,
        source: source.to_string(),
        payload,
        mitre_mappings,
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        tags,
        severity: 5, // Default medium severity
        confidence: 0.8,
    })
}

fn extract_mitre_mappings(payload: &serde_json::Value) -> Vec<MitreMapping> {
    let mut mappings = Vec::new();

    // Try common MITRE mapping locations
    if let Some(mitre) = payload.get("mitre_attack") {
        if let Some(techniques) = mitre.as_array() {
            for tech in techniques {
                if let Some(id) = tech.get("technique_id").and_then(|v| v.as_str()) {
                    mappings.push(MitreMapping {
                        technique_id: id.to_string(),
                        technique_name: tech
                            .get("technique_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        tactic: tech
                            .get("tactic")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        sub_technique: tech
                            .get("sub_technique")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    });
                }
            }
        }
    }

    // Also check coverage field (CAR analytics format)
    if let Some(coverage) = payload.get("coverage") {
        if let Some(items) = coverage.as_array() {
            for item in items {
                if let Some(technique) = item.get("technique") {
                    if let Some(id) = technique.as_str() {
                        mappings.push(MitreMapping {
                            technique_id: id.to_string(),
                            technique_name: String::new(),
                            tactic: item
                                .get("tactic")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            sub_technique: None,
                        });
                    }
                }
            }
        }
    }

    mappings
}

fn extract_tags(payload: &serde_json::Value) -> Vec<String> {
    let mut tags = Vec::new();

    if let Some(t) = payload.get("tags").and_then(|v| v.as_array()) {
        for tag in t {
            if let Some(s) = tag.as_str() {
                tags.push(s.to_string());
            }
        }
    }

    if let Some(groups) = payload.get("groups").and_then(|v| v.as_array()) {
        for group in groups {
            if let Some(s) = group.as_str() {
                tags.push(s.to_string());
            }
        }
    }

    tags
}

// ═══════════════════════════════════════════════════════════════════════════
// Re-exports
// ═══════════════════════════════════════════════════════════════════════════

pub use serde_json::Value as JsonValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cdn_node_creation() {
        let node = ThreatIntelCdnNode::new("test-node".to_string());
        let health = node.health_check().await;
        assert_eq!(health.status, "healthy");
    }

    #[tokio::test]
    async fn test_content_storage() {
        let node = ThreatIntelCdnNode::new("test-node".to_string());

        let content = ThreatIntelContent {
            sch: "abc123".to_string(),
            content_type: ContentType::SigmaRule,
            source: "sigma".to_string(),
            payload: serde_json::json!({"title": "Test Rule"}),
            mitre_mappings: vec![],
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: vec!["test".to_string()],
            severity: 5,
            confidence: 0.9,
        };

        node.store(content.clone()).await.unwrap();

        let retrieved = node.get("abc123").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().source, "sigma");
    }
}
