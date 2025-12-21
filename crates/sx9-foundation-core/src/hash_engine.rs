//! Lightweight hashing engine for ecosystem integrity
//!
//! CTAS-7 v7.2 Trivariate hashing system integrated into the CTAS-7 ecosystem
//! Single source of truth for all hashing operations to avoid redundant compute

use crate::data::{Deserialize, Serialize};
use crate::trivariate_hash_v731::{ContextFrame, ExecEnv, ExecState, TrivariateHashEngineV731};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Lightweight hashing engine for ecosystem-wide integrity
#[derive(Debug, Clone)]
pub struct HashEngine {
    /// Current ecosystem hash state (CTAS-7 v7.2 trivariate hash)
    ecosystem_hash: String,
    /// Component hash registry
    component_hashes: HashMap<String, ComponentHash>,
    /// Hash chain for integrity verification
    hash_chain: Vec<HashChainEntry>,
}

/// Component hash with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHash {
    /// Component identifier
    pub component_id: String,
    /// Trivariate hash of component state
    pub hash: String,
    /// Last update timestamp
    pub updated_at: u64,
    /// Component type for categorization
    pub component_type: ComponentType,
    /// Health status derived from hash analysis
    pub health_status: HashHealthStatus,
}

/// Component types in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Orchestrator,
    Service,
    Foundation,
    Interface,
    Tactical,
    Data,
}

/// Health status derived from hash analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashHealthStatus {
    Healthy,
    Degraded(String),
    Compromised(String),
    Unknown,
}

/// Hash chain entry for integrity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChainEntry {
    /// Chain sequence number
    pub sequence: u64,
    /// Previous hash in chain
    pub previous_hash: String,
    /// Current ecosystem state hash
    pub current_hash: String,
    /// Timestamp of entry
    pub timestamp: u64,
    /// Components included in this hash
    pub components: Vec<String>,
}

/// Minimal hash verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashVerification {
    /// Overall ecosystem integrity
    pub ecosystem_integrity: bool,
    /// Number of healthy components
    pub healthy_components: u32,
    /// Number of compromised components
    pub compromised_components: u32,
    /// Critical integrity violations
    pub critical_violations: Vec<String>,
}

impl Default for HashEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HashEngine {
    /// Creates new lightweight hash engine
    /// Creates new lightweight hash engine
    #[must_use]
    pub fn new() -> Self {
        let trivariate_engine = TrivariateHashEngineV731::new();
        let context = ContextFrame::new(ExecEnv::Native, 0, ExecState::Hot);

        let hash_struct = trivariate_engine.generate_trivariate(
            "CTAS7_ECOSYSTEM_INIT",
            "System",
            "Core",
            "Init",
            &context,
        );
        let ecosystem_hash = hash_struct.to_canonical_format(); // Use canonical format: triv:SCH_CUID_UUID

        Self {
            ecosystem_hash: ecosystem_hash.clone(),
            component_hashes: HashMap::new(),
            hash_chain: vec![HashChainEntry {
                sequence: 0,
                previous_hash: "genesis".to_string(),
                current_hash: ecosystem_hash,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                components: vec!["genesis".to_string()],
            }],
        }
    }

    /// Update component hash - minimal overhead operation
    pub fn update_component_hash(
        &mut self,
        component_id: &str,
        data: &[u8],
        component_type: ComponentType,
    ) -> String {
        let trivariate_engine = TrivariateHashEngineV731::new();
        let data_str = String::from_utf8_lossy(data);

        let context = ContextFrame::new(ExecEnv::Native, 0, ExecState::Hot);

        // Map component type to domain
        let domain = match component_type {
            ComponentType::Data => "Data",
            ComponentType::Tactical | ComponentType::Orchestrator => "Tactical",
            _ => "Core",
        };

        let hash_struct = trivariate_engine.generate_trivariate(
            &data_str,
            component_id,
            domain,
            "Update",
            &context,
        );
        let hash_str = hash_struct.to_canonical_format();

        // Analyze hash for health indicators (lightweight)
        let health_status = self.analyze_hash_health(&hash_str, &component_type);

        let component_hash = ComponentHash {
            component_id: component_id.to_string(),
            hash: hash_str.clone(),
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            component_type,
            health_status,
        };

        self.component_hashes
            .insert(component_id.to_string(), component_hash);

        // Update ecosystem hash (minimal computation)
        self.update_ecosystem_hash();

        hash_str
    }

    /// Get component hash - O(1) lookup
    #[must_use]
    pub fn get_component_hash(&self, component_id: &str) -> Option<&ComponentHash> {
        self.component_hashes.get(component_id)
    }

    /// Verify ecosystem integrity - lightweight check
    #[must_use]
    pub fn verify_ecosystem_integrity(&self) -> HashVerification {
        let mut healthy_count = 0;
        let mut compromised_count = 0;
        let mut critical_violations = Vec::new();

        for (component_id, component_hash) in &self.component_hashes {
            match &component_hash.health_status {
                HashHealthStatus::Healthy => healthy_count += 1,
                HashHealthStatus::Compromised(reason) => {
                    compromised_count += 1;
                    critical_violations.push(format!("{component_id}: {reason}"));
                }
                HashHealthStatus::Degraded(_) => {} // Not critical
                HashHealthStatus::Unknown => {}
            }
        }

        HashVerification {
            ecosystem_integrity: compromised_count == 0,
            healthy_components: healthy_count,
            compromised_components: compromised_count,
            critical_violations,
        }
    }

    /// Get ecosystem hash for external verification
    #[must_use]
    pub fn get_ecosystem_hash(&self) -> String {
        self.ecosystem_hash.clone()
    }

    /// Get hash chain length for integrity metrics
    #[must_use]
    pub fn get_hash_chain_length(&self) -> u64 {
        self.hash_chain.len() as u64
    }

    /// Export minimal hash state for network transmission
    #[must_use]
    pub fn export_hash_state(&self) -> HashMap<String, String> {
        let mut state = HashMap::new();

        state.insert("ecosystem_hash".to_string(), self.get_ecosystem_hash());
        state.insert(
            "chain_length".to_string(),
            self.get_hash_chain_length().to_string(),
        );
        state.insert(
            "component_count".to_string(),
            self.component_hashes.len().to_string(),
        );

        // Add critical component hashes only
        for (id, component) in &self.component_hashes {
            if matches!(
                component.component_type,
                ComponentType::Foundation | ComponentType::Orchestrator
            ) {
                state.insert(format!("component_{id}"), component.hash.clone());
            }
        }

        state
    }

    /// Update ecosystem hash with minimal computation
    fn update_ecosystem_hash(&mut self) {
        let trivariate_engine = TrivariateHashEngineV731::new();

        // Hash only critical components to keep computation light
        let mut critical_hashes: Vec<String> = self
            .component_hashes
            .values()
            .filter(|c| {
                matches!(
                    c.component_type,
                    ComponentType::Foundation | ComponentType::Orchestrator
                )
            })
            .map(|c| c.hash.clone())
            .collect();

        critical_hashes.sort(); // Deterministic ordering
        let combined_hashes = critical_hashes.join(":");
        let combined_data = format!("{}:{}", self.ecosystem_hash, combined_hashes);

        let context = ContextFrame::new(ExecEnv::Native, 0, ExecState::Hot);

        let hash_struct = trivariate_engine.generate_trivariate(
            &combined_data,
            "Ecosystem",
            "Core",
            "Sync",
            &context,
        );
        let new_hash = hash_struct.to_canonical_format();

        // Add to hash chain if significantly different
        if new_hash != self.ecosystem_hash {
            let chain_entry = HashChainEntry {
                sequence: self.hash_chain.len() as u64,
                previous_hash: self.ecosystem_hash.clone(),
                current_hash: new_hash.clone(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                components: critical_hashes,
            };

            self.hash_chain.push(chain_entry);
            self.ecosystem_hash = new_hash;

            // Keep chain lightweight - only last 100 entries
            if self.hash_chain.len() > 100 {
                self.hash_chain.remove(0);
            }
        }
    }

    /// Lightweight hash health analysis
    fn analyze_hash_health(&self, hash: &str, component_type: &ComponentType) -> HashHealthStatus {
        // Validation for Trivariate Hash format (triv:SCH_CUID_UUID)
        if !hash.starts_with("triv:") {
            return HashHealthStatus::Compromised(
                "Invalid hash format (missing triv: prefix)".to_string(),
            );
        }

        let parts: Vec<&str> = hash.split(':').collect();
        if parts.len() < 2 {
            return HashHealthStatus::Compromised("Invalid hash structure".to_string());
        }

        // Simple length check as proxy for entropy/validity in this lightweight engine
        let payload = parts[1];
        let entropy = self.calculate_simple_entropy(payload.as_bytes());

        match component_type {
            ComponentType::Foundation | ComponentType::Orchestrator => {
                if entropy < 6.0 {
                    HashHealthStatus::Compromised("Low entropy in critical component".to_string())
                } else if entropy < 7.0 {
                    HashHealthStatus::Degraded("Reduced entropy".to_string())
                } else {
                    HashHealthStatus::Healthy
                }
            }
            _ => {
                if entropy < 5.0 {
                    HashHealthStatus::Degraded("Low entropy".to_string())
                } else {
                    HashHealthStatus::Healthy
                }
            }
        }
    }

    /// Simple entropy calculation for hash health
    fn calculate_simple_entropy(&self, bytes: &[u8]) -> f64 {
        let mut counts = [0u32; 256];
        for &byte in bytes {
            counts[byte as usize] += 1;
        }

        let len = bytes.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = f64::from(count) / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }
}

use std::sync::{Arc, Mutex, OnceLock};

/// Global hash engine instance for ecosystem integration (thread-safe)
static GLOBAL_HASH_ENGINE: OnceLock<Arc<Mutex<HashEngine>>> = OnceLock::new();

/// Initialize global hash engine (call once on startup)
pub fn init_global_hash_engine() {
    let _ = GLOBAL_HASH_ENGINE.set(Arc::new(Mutex::new(HashEngine::new())));
}

/// Update global component hash (thread-safe)
pub fn update_global_component_hash(
    component_id: &str,
    data: &[u8],
    component_type: ComponentType,
) -> Option<String> {
    let engine = GLOBAL_HASH_ENGINE.get()?;
    let mut engine = engine.lock().ok()?;
    Some(engine.update_component_hash(component_id, data, component_type))
}

/// Get global ecosystem verification (thread-safe)
pub fn get_global_ecosystem_verification() -> Option<HashVerification> {
    let engine = GLOBAL_HASH_ENGINE.get()?;
    let engine = engine.lock().ok()?;
    Some(engine.verify_ecosystem_integrity())
}

/// Get global hash state for network transmission (thread-safe)
pub fn get_global_hash_state() -> Option<HashMap<String, String>> {
    let engine = GLOBAL_HASH_ENGINE.get()?;
    let engine = engine.lock().ok()?;
    Some(engine.export_hash_state())
}
