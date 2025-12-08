//! CTAS-7 Storage Backends
//! Supabase for blockchain/certificates + Sled KVS for fast health checks

use serde::{Deserialize, Serialize};
use crate::usim_blockchain::{UsimBlockchain, UsimBlock, UsimTransaction};
use crate::usim_pgp_integration::{UsimBuildSignature, UsimVerificationResult};

#[cfg(not(feature = "embedded-firefly"))]
use std::{string::String, vec::Vec, collections::HashMap};

#[cfg(feature = "embedded-firefly")]
use alloc::{string::String, vec::Vec};

/// Storage backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Supabase configuration for persistent storage
    pub supabase: SupabaseConfig,
    /// Sled KVS configuration for fast health checks
    pub sled: SledConfig,
}

/// Supabase backend for blockchain and certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
    pub service_role_key: Option<String>,
    /// Database tables
    pub blockchain_table: String,
    pub certificates_table: String,
    pub pgp_keys_table: String,
    pub build_signatures_table: String,
}

/// Sled KVS for fast artifact health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SledConfig {
    pub db_path: String,
    /// Tree names for different data types
    pub health_tree: String,
    pub artifacts_tree: String,
    pub quick_verify_tree: String,
}

/// Blockchain storage interface
#[derive(Debug)]
pub struct BlockchainStorage {
    supabase_client: SupabaseClient,
    sled_db: SledDatabase,
    config: StorageConfig,
}

/// Supabase client wrapper
#[derive(Debug)]
pub struct SupabaseClient {
    client: reqwest::Client,
    config: SupabaseConfig,
}

/// Sled database wrapper for fast operations
#[derive(Debug)]
pub struct SledDatabase {
    db: sled::Db,
    health_tree: sled::Tree,
    artifacts_tree: sled::Tree,
    quick_verify_tree: sled::Tree,
}

/// Health check artifact for Sled KVS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthArtifact {
    pub crate_name: String,
    pub hash: String,
    pub last_verified: u64,
    pub quality_grade: String,
    pub loc: u32,
    pub complexity: u32,
    pub tesla_grade: bool,
}

/// Quick verification bytecode for ultra-fast checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickVerifyBytecode {
    pub artifact_hash: String,
    pub pgp_signature_hash: String,
    pub blockchain_block_hash: String,
    pub verification_bytecode: Vec<u8>, // Compressed verification data
    pub expiry: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            supabase: SupabaseConfig {
                url: "https://your-project.supabase.co".to_string(),
                anon_key: "your-anon-key".to_string(),
                service_role_key: None,
                blockchain_table: "ctas7_blockchain".to_string(),
                certificates_table: "ctas7_certificates".to_string(),
                pgp_keys_table: "ctas7_pgp_keys".to_string(),
                build_signatures_table: "ctas7_build_signatures".to_string(),
            },
            sled: SledConfig {
                db_path: "/var/lib/ctas7/sled.db".to_string(),
                health_tree: "health_checks".to_string(),
                artifacts_tree: "artifacts".to_string(),
                quick_verify_tree: "quick_verify".to_string(),
            },
        }
    }
}

impl BlockchainStorage {
    /// Initialize storage backends
    pub async fn new(config: StorageConfig) -> Result<Self, String> {
        let supabase_client = SupabaseClient::new(config.supabase.clone())?;
        let sled_db = SledDatabase::new(&config.sled)?;

        Ok(Self {
            supabase_client,
            sled_db,
            config,
        })
    }

    /// Store blockchain to Supabase
    pub async fn store_blockchain(&self, blockchain: &UsimBlockchain) -> Result<(), String> {
        let serialized = serde_json::to_string(blockchain)
            .map_err(|e| format!("Failed to serialize blockchain: {}", e))?;

        self.supabase_client.insert_blockchain(serialized).await
    }

    /// Store individual block to Supabase
    pub async fn store_block(&self, block: &UsimBlock) -> Result<(), String> {
        let block_data = serde_json::json!({
            "index": block.index,
            "previous_hash": block.previous_hash,
            "timestamp": block.timestamp,
            "transactions": block.transactions,
            "merkle_root": block.merkle_root,
            "hash": block.hash,
            "nonce": block.nonce
        });

        self.supabase_client.insert_block(block_data).await
    }

    /// Store PGP signature to Supabase
    pub async fn store_pgp_signature(&self, signature: &UsimBuildSignature) -> Result<(), String> {
        let sig_data = serde_json::json!({
            "artifact_hash": signature.artifact_hash,
            "signature": signature.signature,
            "signer": signature.signer,
            "timestamp": signature.timestamp,
            "algorithm": signature.algorithm
        });

        self.supabase_client.insert_signature(sig_data).await
    }

    /// Store health check artifact to Sled for fast access
    pub fn store_health_artifact(&self, artifact: &HealthArtifact) -> Result<(), String> {
        self.sled_db.store_health_check(artifact)
    }

    /// Store quick verification bytecode to Sled
    pub fn store_quick_verify(&self, bytecode: &QuickVerifyBytecode) -> Result<(), String> {
        self.sled_db.store_quick_verify(bytecode)
    }

    /// Fast health check from Sled (microsecond response)
    pub fn quick_health_check(&self, crate_name: &str) -> Result<Option<HealthArtifact>, String> {
        self.sled_db.get_health_check(crate_name)
    }

    /// Ultra-fast verification from Sled bytecode
    pub fn quick_verify(&self, artifact_hash: &str) -> Result<Option<QuickVerifyBytecode>, String> {
        self.sled_db.get_quick_verify(artifact_hash)
    }

    /// Retrieve blockchain from Supabase
    pub async fn load_blockchain(&self) -> Result<Option<UsimBlockchain>, String> {
        self.supabase_client.get_latest_blockchain().await
    }
}

impl SupabaseClient {
    pub fn new(config: SupabaseConfig) -> Result<Self, String> {
        let client = reqwest::Client::new();
        Ok(Self { client, config })
    }

    pub async fn insert_blockchain(&self, blockchain_json: String) -> Result<(), String> {
        let url = format!("{}/rest/v1/{}", self.config.url, self.config.blockchain_table);

        let payload = serde_json::json!({
            "blockchain_data": blockchain_json,
            "created_at": chrono::Utc::now().to_rfc3339(),
            "version": "7.0.0"
        });

        let response = self.client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", self.config.anon_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Supabase request failed: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Supabase error: {}", response.status()))
        }
    }

    pub async fn insert_block(&self, block_data: serde_json::Value) -> Result<(), String> {
        let url = format!("{}/rest/v1/ctas7_blocks", self.config.url);

        let response = self.client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", self.config.anon_key))
            .header("Content-Type", "application/json")
            .json(&block_data)
            .send()
            .await
            .map_err(|e| format!("Failed to store block: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Block storage failed: {}", response.status()))
        }
    }

    pub async fn insert_signature(&self, sig_data: serde_json::Value) -> Result<(), String> {
        let url = format!("{}/rest/v1/{}", self.config.url, self.config.build_signatures_table);

        let response = self.client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", self.config.anon_key))
            .header("Content-Type", "application/json")
            .json(&sig_data)
            .send()
            .await
            .map_err(|e| format!("Failed to store signature: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Signature storage failed: {}", response.status()))
        }
    }

    pub async fn get_latest_blockchain(&self) -> Result<Option<UsimBlockchain>, String> {
        let url = format!("{}/rest/v1/{}?order=created_at.desc&limit=1",
                         self.config.url, self.config.blockchain_table);

        let response = self.client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", self.config.anon_key))
            .send()
            .await
            .map_err(|e| format!("Failed to retrieve blockchain: {}", e))?;

        if response.status().is_success() {
            let data: Vec<serde_json::Value> = response.json().await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            if let Some(record) = data.first() {
                if let Some(blockchain_data) = record.get("blockchain_data") {
                    let blockchain: UsimBlockchain = serde_json::from_str(
                        blockchain_data.as_str().unwrap_or("{}")
                    ).map_err(|e| format!("Failed to deserialize blockchain: {}", e))?;

                    return Ok(Some(blockchain));
                }
            }
        }

        Ok(None)
    }
}

impl SledDatabase {
    pub fn new(config: &SledConfig) -> Result<Self, String> {
        let db = sled::open(&config.db_path)
            .map_err(|e| format!("Failed to open Sled database: {}", e))?;

        let health_tree = db.open_tree(&config.health_tree)
            .map_err(|e| format!("Failed to open health tree: {}", e))?;

        let artifacts_tree = db.open_tree(&config.artifacts_tree)
            .map_err(|e| format!("Failed to open artifacts tree: {}", e))?;

        let quick_verify_tree = db.open_tree(&config.quick_verify_tree)
            .map_err(|e| format!("Failed to open quick verify tree: {}", e))?;

        Ok(Self {
            db,
            health_tree,
            artifacts_tree,
            quick_verify_tree,
        })
    }

    pub fn store_health_check(&self, artifact: &HealthArtifact) -> Result<(), String> {
        let serialized = bincode::serialize(artifact)
            .map_err(|e| format!("Failed to serialize health artifact: {}", e))?;

        self.health_tree.insert(&artifact.crate_name, serialized)
            .map_err(|e| format!("Failed to store health check: {}", e))?;

        self.health_tree.flush()
            .map_err(|e| format!("Failed to flush health tree: {}", e))?;

        Ok(())
    }

    pub fn store_quick_verify(&self, bytecode: &QuickVerifyBytecode) -> Result<(), String> {
        let serialized = bincode::serialize(bytecode)
            .map_err(|e| format!("Failed to serialize bytecode: {}", e))?;

        self.quick_verify_tree.insert(&bytecode.artifact_hash, serialized)
            .map_err(|e| format!("Failed to store quick verify: {}", e))?;

        self.quick_verify_tree.flush()
            .map_err(|e| format!("Failed to flush quick verify tree: {}", e))?;

        Ok(())
    }

    pub fn get_health_check(&self, crate_name: &str) -> Result<Option<HealthArtifact>, String> {
        if let Some(data) = self.health_tree.get(crate_name)
            .map_err(|e| format!("Failed to retrieve health check: {}", e))? {

            let artifact: HealthArtifact = bincode::deserialize(&data)
                .map_err(|e| format!("Failed to deserialize health artifact: {}", e))?;

            Ok(Some(artifact))
        } else {
            Ok(None)
        }
    }

    pub fn get_quick_verify(&self, artifact_hash: &str) -> Result<Option<QuickVerifyBytecode>, String> {
        if let Some(data) = self.quick_verify_tree.get(artifact_hash)
            .map_err(|e| format!("Failed to retrieve quick verify: {}", e))? {

            let bytecode: QuickVerifyBytecode = bincode::deserialize(&data)
                .map_err(|e| format!("Failed to deserialize bytecode: {}", e))?;

            Ok(Some(bytecode))
        } else {
            Ok(None)
        }
    }

    /// Cleanup expired quick verification entries
    pub fn cleanup_expired(&self) -> Result<u32, String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut removed = 0;

        for item in self.quick_verify_tree.iter() {
            let (key, value) = item.map_err(|e| format!("Iterator error: {}", e))?;

            if let Ok(bytecode) = bincode::deserialize::<QuickVerifyBytecode>(&value) {
                if bytecode.expiry < now {
                    self.quick_verify_tree.remove(&key)
                        .map_err(|e| format!("Failed to remove expired entry: {}", e))?;
                    removed += 1;
                }
            }
        }

        self.quick_verify_tree.flush()
            .map_err(|e| format!("Failed to flush after cleanup: {}", e))?;

        Ok(removed)
    }
}

/// Helper function to create storage config from environment
pub fn storage_config_from_env() -> StorageConfig {
    StorageConfig {
        supabase: SupabaseConfig {
            url: std::env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "https://your-project.supabase.co".to_string()),
            anon_key: std::env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| "your-anon-key".to_string()),
            service_role_key: std::env::var("SUPABASE_SERVICE_ROLE_KEY").ok(),
            blockchain_table: "ctas7_blockchain".to_string(),
            certificates_table: "ctas7_certificates".to_string(),
            pgp_keys_table: "ctas7_pgp_keys".to_string(),
            build_signatures_table: "ctas7_build_signatures".to_string(),
        },
        sled: SledConfig {
            db_path: std::env::var("SLED_DB_PATH")
                .unwrap_or_else(|_| "/var/lib/ctas7/sled.db".to_string()),
            health_tree: "health_checks".to_string(),
            artifacts_tree: "artifacts".to_string(),
            quick_verify_tree: "quick_verify".to_string(),
        },
    }
}