//! Status reporting and TOML serialization with ontology integration

use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unified orchestration status with TOML serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationStatus {
    pub orchestrator_id: Uuid,
    pub orchestrator_type: OrchestratorType,
    pub state: OrchestrationState,
    pub xsd_status: XsdValidationStatus,
    pub health: HealthMetrics,
    pub resources: ResourceMetrics,
    pub active_operations: u32,
    pub last_updated: DateTime<Utc>,
    pub toml_status: String,
}

/// Foundation crates availability status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationCrateStatus {
    pub available: bool,
    pub version: String,
    pub status: CrateStatus,
    pub fallback_used: bool,
    pub last_checked: DateTime<Utc>,
}

/// Crate status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrateStatus {
    Active,
    Stub,
    Missing,
    Error(String),
}

/// Ontology integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyStatus {
    pub frontend_connected: bool,
    pub entities_synced: u32,
    pub last_sync: DateTime<Utc>,
    pub active_namespaces: Vec<String>,
    pub sync_errors: Vec<String>,
}

/// TOML status reporter for external system integration
pub struct TomlStatusReporter {
    orchestrator_statuses: HashMap<Uuid, OrchestrationStatus>,
    foundation_status: HashMap<String, FoundationCrateStatus>,
    ontology_status: OntologyStatus,
}

impl TomlStatusReporter {
    pub fn new() -> Self {
        Self {
            orchestrator_statuses: HashMap::new(),
            foundation_status: Self::init_foundation_status(),
            ontology_status: OntologyStatus::default(),
        }
    }

    fn init_foundation_status() -> HashMap<String, FoundationCrateStatus> {
        let mut status = HashMap::new();

        for crate_name in &["ctas7-data-foundation", "ctas7-interface-foundation", "ctas7-tactical-foundation"] {
            status.insert(
                crate_name.to_string(),
                FoundationCrateStatus {
                    available: false,
                    version: "0.1.0".to_string(),
                    status: CrateStatus::Missing,
                    fallback_used: false,
                    last_checked: Utc::now(),
                }
            );
        }

        status
    }

    pub fn update_status(&mut self, status: OrchestrationStatus) {
        let toml_status = self.generate_toml_status(&status);
        let mut updated_status = status;
        updated_status.toml_status = toml_status;
        self.orchestrator_statuses.insert(updated_status.orchestrator_id, updated_status);
    }

    pub fn update_foundation_crate(&mut self, crate_name: &str, status: FoundationCrateStatus) {
        self.foundation_status.insert(crate_name.to_string(), status);
    }

    pub fn update_ontology_status(&mut self, status: OntologyStatus) {
        self.ontology_status = status;
    }

    pub fn get_consolidated_toml(&self) -> String {
        let mut consolidated = String::from("[orchestration]\n");
        consolidated.push_str(&format!("timestamp = \"{}\"\n", Utc::now().to_rfc3339()));
        consolidated.push_str(&format!("total_orchestrators = {}\n", self.orchestrator_statuses.len()));

        let system_health = if self.orchestrator_statuses.values().all(|s| matches!(s.health.status, HealthStatus::Healthy)) {
            "Healthy"
        } else if self.orchestrator_statuses.values().any(|s| matches!(s.health.status, HealthStatus::Unhealthy(_))) {
            "Unhealthy"
        } else {
            "Degraded"
        };
        consolidated.push_str(&format!("system_health = \"{}\"\n\n", system_health));

        // Foundation crates status
        consolidated.push_str("# Foundation crates status\n");
        consolidated.push_str("[foundation_crates]\n");
        for (name, status) in &self.foundation_status {
            let clean_name = name.replace("ctas7-", "").replace("-", "_");
            consolidated.push_str(&format!(
                "{} = {{ available = {}, version = \"{}\", status = \"{:?}\", fallback_used = {} }}\n",
                clean_name, status.available, status.version, status.status, status.fallback_used
            ));
        }
        consolidated.push_str("\n");

        // Orchestrator instances
        consolidated.push_str("# Orchestrator instances\n");
        for (id, status) in &self.orchestrator_statuses {
            consolidated.push_str("[[orchestrators]]\n");
            consolidated.push_str(&format!("id = \"{}\"\n", id));
            consolidated.push_str(&format!("type = \"{:?}\"\n", status.orchestrator_type));
            consolidated.push_str(&format!("state = \"{:?}\"\n", status.state));
            consolidated.push_str(&format!("health = \"{:?}\"\n", status.health.status));
            consolidated.push_str(&format!("operations = {}\n", status.active_operations));
            consolidated.push_str(&format!("cpu_percent = {:.2}\n", status.resources.cpu_usage_percent));
            consolidated.push_str(&format!("memory_mb = {}\n", status.resources.memory_usage_mb));
            consolidated.push_str(&format!("xsd_valid = {}\n", status.xsd_status.is_valid));
            consolidated.push_str(&format!("last_updated = \"{}\"\n\n", status.last_updated.to_rfc3339()));
        }

        // Ontology integration status
        consolidated.push_str("# Ontology integration status\n");
        consolidated.push_str("[ontology]\n");
        consolidated.push_str(&format!("frontend_connected = {}\n", self.ontology_status.frontend_connected));
        consolidated.push_str(&format!("entities_synced = {}\n", self.ontology_status.entities_synced));
        consolidated.push_str(&format!("last_sync = \"{}\"\n", self.ontology_status.last_sync.to_rfc3339()));
        consolidated.push_str(&format!("namespaces = {:?}\n", self.ontology_status.active_namespaces));
        if !self.ontology_status.sync_errors.is_empty() {
            consolidated.push_str(&format!("sync_errors = {:?}\n", self.ontology_status.sync_errors));
        }

        consolidated
    }

    fn generate_toml_status(&self, status: &OrchestrationStatus) -> String {
        format!(
            r#"[status]
id = "{}"
type = "{:?}"
state = "{:?}"
health = "{:?}"
operations = {}
cpu_percent = {:.2}
memory_mb = {}
xsd_valid = {}
last_updated = "{}"

[dependencies]
data_foundation = "{:?}"
interface_foundation = "{:?}"
tactical_foundation = "{:?}"

[ontology]
connected = {}
entities = {}
"#,
            status.orchestrator_id,
            status.orchestrator_type,
            status.state,
            status.health.status,
            status.active_operations,
            status.resources.cpu_usage_percent,
            status.resources.memory_usage_mb,
            status.xsd_status.is_valid,
            status.last_updated.to_rfc3339(),
            self.foundation_status.get("ctas7-data-foundation").map(|s| &s.status).unwrap_or(&CrateStatus::Missing),
            self.foundation_status.get("ctas7-interface-foundation").map(|s| &s.status).unwrap_or(&CrateStatus::Missing),
            self.foundation_status.get("ctas7-tactical-foundation").map(|s| &s.status).unwrap_or(&CrateStatus::Missing),
            self.ontology_status.frontend_connected,
            self.ontology_status.entities_synced
        )
    }

    /// Sync with frontend ontology manager
    pub async fn sync_with_ontology(&mut self, manager_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        // Fetch ontology status from frontend
        let response = client
            .get(&format!("{}/status", manager_url))
            .send()
            .await?;

        if response.status().is_success() {
            let ontology_data: serde_json::Value = response.json().await?;

            self.ontology_status = OntologyStatus {
                frontend_connected: true,
                entities_synced: ontology_data["entities_count"].as_u64().unwrap_or(0) as u32,
                last_sync: Utc::now(),
                active_namespaces: ontology_data["namespaces"]
                    .as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_else(|| vec!["ctas-core".to_string()]),
                sync_errors: vec![],
            };
        } else {
            self.ontology_status.frontend_connected = false;
            self.ontology_status.sync_errors.push(format!("HTTP {}", response.status()));
        }

        Ok(())
    }
}

impl Default for OntologyStatus {
    fn default() -> Self {
        Self {
            frontend_connected: false,
            entities_synced: 0,
            last_sync: Utc::now(),
            active_namespaces: vec![
                "ctas-core".to_string(),
                "nist-csf".to_string(),
                "mitre-attack".to_string(),
                "cwe-cve".to_string(),
            ],
            sync_errors: vec![],
        }
    }
}