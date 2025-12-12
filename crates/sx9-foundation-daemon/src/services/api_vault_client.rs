use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APICredential {
    pub service: String,
    pub key: String,
    pub rate_limit: RateLimit,
    pub expires_at: Option<DateTime<Utc>>,
    pub tier: CredentialTier,
    pub effective_rate_limit: Option<u32>,
    pub audit_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit_type: String,
    pub value: u32,
    pub window: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialTier {
    Local,
    GCPSecret,
    RuntimeInjected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialScope {
    pub operation: String,
    pub escalation_level: String,
    pub requesting_service: String,
    pub audit_context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIVaultRequest {
    pub service: String,
    pub scope: CredentialScope,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIVaultResponse {
    pub credential: Option<APICredential>,
    pub error: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CredentialCache {
    cache: HashMap<String, (APICredential, DateTime<Utc>)>,
    ttl_seconds: u64,
}

impl CredentialCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: HashMap::new(),
            ttl_seconds,
        }
    }

    pub fn get(&self, key: &str) -> Option<APICredential> {
        if let Some((credential, cached_at)) = self.cache.get(key) {
            let age = Utc::now().signed_duration_since(*cached_at);
            if age.num_seconds() < self.ttl_seconds as i64 {
                return Some(credential.clone());
            }
        }
        None
    }

    pub fn set(&mut self, key: String, credential: APICredential) {
        self.cache.insert(key, (credential, Utc::now()));
    }

    pub fn invalidate(&mut self, key: &str) {
        self.cache.remove(key);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProfile {
    pub tool_name: String,
    pub category: String,
    pub capability: String,
    pub required_apis: Vec<String>,
    pub free_tier_available: bool,
    pub rate_limits: HashMap<String, String>,
    pub authentication_type: String,
    pub data_classification: String,
    pub kali_linux_package: Option<String>,
    pub docker_image: Option<String>,
    pub integration_status: String,
    pub ctas_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegistry {
    pub categories: HashMap<String, Vec<String>>,
    pub tools: HashMap<String, ToolProfile>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            categories: Self::default_categories(),
            tools: HashMap::new(),
        }
    }

    fn default_categories() -> HashMap<String, Vec<String>> {
        let mut categories = HashMap::new();

        categories.insert(
            "osint".to_string(),
            vec![
                "domain_recon".to_string(),
                "social_media".to_string(),
                "email_recon".to_string(),
                "ip_geolocation".to_string(),
                "dns_enumeration".to_string(),
            ],
        );

        categories.insert(
            "exploitation".to_string(),
            vec![
                "vulnerability_scanning".to_string(),
                "exploit_frameworks".to_string(),
                "web_testing".to_string(),
            ],
        );

        categories.insert(
            "password_testing".to_string(),
            vec![
                "credential_testing".to_string(),
                "breach_databases".to_string(),
                "wordlist_generation".to_string(),
            ],
        );

        categories.insert(
            "intelligence".to_string(),
            vec![
                "threat_intel".to_string(),
                "geolocation".to_string(),
                "malware_analysis".to_string(),
            ],
        );

        categories.insert(
            "data_processing".to_string(),
            vec![
                "etl".to_string(),
                "data_validation".to_string(),
                "transformations".to_string(),
            ],
        );

        categories
    }

    pub fn register_tool(&mut self, profile: ToolProfile) {
        self.tools.insert(profile.tool_name.clone(), profile);
    }

    pub fn get_tools_by_category(&self, category: &str) -> Vec<String> {
        self.tools
            .values()
            .filter(|t| t.category == category)
            .map(|t| t.tool_name.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSyncState {
    pub last_sync: Option<DateTime<Utc>>,
    pub synced_services: HashMap<String, String>,
    pub sync_errors: Vec<String>,
}

impl VaultSyncState {
    pub fn new() -> Self {
        Self {
            last_sync: None,
            synced_services: HashMap::new(),
            sync_errors: Vec::new(),
        }
    }

    pub fn record_sync(&mut self, service: String, hash: String) {
        self.last_sync = Some(Utc::now());
        self.synced_services.insert(service, hash);
    }

    pub fn record_error(&mut self, error: String) {
        self.sync_errors.push(error);
        if self.sync_errors.len() > 100 {
            self.sync_errors.remove(0);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub service: String,
    pub requesting_service: String,
    pub trivariate_hash: String,
    pub escalation_level: String,
    pub tier: String,
    pub status: String,
    pub operation_id: String,
}
