//! KeyVault Integration for SX9 Dev Forge
//! 
//! Compatible with sx9-foundation-core KeyVault
//! Shares the same Sled database at ~/.sx9/keyvault/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Standard API key names (matches foundation-core)
pub mod keys {
    pub const MAPBOX: &str = "mapbox";
    pub const ELEVENLABS: &str = "elevenlabs";
    pub const OPENAI: &str = "openai";
    pub const ANTHROPIC: &str = "anthropic";
    pub const LINEAR: &str = "linear";
    pub const GITHUB: &str = "github";
    pub const SUPABASE: &str = "supabase";
    pub const NATS: &str = "nats";
    pub const AWS_ACCESS: &str = "aws_access_key";
    pub const AWS_SECRET: &str = "aws_secret_key";
    pub const GOOGLE: &str = "google";
    pub const CLOUDFLARE: &str = "cloudflare";
    pub const GEMINI: &str = "gemini";
    pub const GROK: &str = "grok";
    
    /// All standard keys in order
    pub const ALL: &[&str] = &[
        MAPBOX, ELEVENLABS, OPENAI, ANTHROPIC, LINEAR, GITHUB,
        SUPABASE, NATS, AWS_ACCESS, AWS_SECRET, GOOGLE, 
        CLOUDFLARE, GEMINI, GROK
    ];
}

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Key not found: {0}")]
    NotFound(String),
}

/// Key entry with metadata (compatible with foundation-core)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntry {
    pub name: String,
    pub value: String,
    pub service: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Summary without exposing secret value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntrySummary {
    pub name: String,
    pub service: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub has_value: bool,
}

/// KeyVault - persistent API key storage
/// Compatible with ~/.sx9/keyvault/ from foundation-core
pub struct KeyVault {
    db: Option<sled::Db>,
    cache: Arc<RwLock<HashMap<String, KeyEntry>>>,
    vault_dir: PathBuf,
    backup_path: PathBuf,
}

impl KeyVault {
    /// Standard vault directory (~/.sx9/keyvault)
    pub fn default_vault_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sx9")
            .join("keyvault")
    }

    /// Create vault with default path
    pub fn new() -> Result<Self, VaultError> {
        Self::with_path(&Self::default_vault_dir())
    }

    /// Create vault at specific path
    pub fn with_path(vault_dir: &PathBuf) -> Result<Self, VaultError> {
        fs::create_dir_all(vault_dir)
            .map_err(|e| VaultError::Io(format!("Failed to create vault dir: {}", e)))?;

        let db_path = vault_dir.join("keys.sled");
        let backup_path = vault_dir.join("keys.backup.json");

        let db = match sled::open(&db_path) {
            Ok(db) => Some(db),
            Err(e) => {
                tracing::warn!("Sled DB failed ({}), using file fallback", e);
                None
            }
        };

        // Load existing keys synchronously
        let mut loaded = HashMap::new();

        // Load from sled
        if let Some(ref db) = db {
            for item in db.iter() {
                if let Ok((key, value)) = item {
                    if let (Ok(name), Ok(entry)) = (
                        String::from_utf8(key.to_vec()),
                        serde_json::from_slice::<KeyEntry>(&value),
                    ) {
                        loaded.insert(name, entry);
                    }
                }
            }
        }

        // Fallback to backup file
        if loaded.is_empty() && backup_path.exists() {
            if let Ok(content) = fs::read_to_string(&backup_path) {
                if let Ok(entries) = serde_json::from_str::<Vec<KeyEntry>>(&content) {
                    for entry in entries {
                        loaded.insert(entry.name.clone(), entry);
                    }
                    tracing::info!("Recovered {} keys from backup", loaded.len());

                    // Restore to sled
                    if let Some(ref db) = db {
                        for (name, entry) in &loaded {
                            if let Ok(json) = serde_json::to_vec(entry) {
                                let _ = db.insert(name.as_bytes(), json);
                            }
                        }
                        let _ = db.flush();
                    }
                }
            }
        }

        let vault = Self {
            db,
            cache: Arc::new(RwLock::new(loaded)),
            vault_dir: vault_dir.clone(),
            backup_path,
        };

        Ok(vault)
    }

    /// Store a key
    pub async fn set(&self, name: &str, value: &str, service: &str) -> Result<(), VaultError> {
        let entry = KeyEntry {
            name: name.to_string(),
            value: value.to_string(),
            service: service.to_string(),
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
            active: true,
            expires_at: None,
            notes: None,
        };
        self.store_entry(entry).await
    }

    /// Store full entry
    pub async fn store_entry(&self, entry: KeyEntry) -> Result<(), VaultError> {
        let name = entry.name.clone();

        // Persist to sled
        if let Some(db) = &self.db {
            let json = serde_json::to_vec(&entry)
                .map_err(|e| VaultError::Serialization(e.to_string()))?;
            db.insert(name.as_bytes(), json)
                .map_err(|e| VaultError::Database(e.to_string()))?;
            db.flush()
                .map_err(|e| VaultError::Database(e.to_string()))?;
        }

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(name, entry);
        }

        // Backup
        self.backup().await?;

        Ok(())
    }

    /// Get key value
    pub async fn get(&self, name: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get(name).and_then(|e| {
            if e.active {
                Some(e.value.clone())
            } else {
                None
            }
        })
    }

    /// Get full entry
    pub async fn get_entry(&self, name: &str) -> Option<KeyEntry> {
        let cache = self.cache.read().await;
        cache.get(name).cloned()
    }

    /// Delete key
    pub async fn delete(&self, name: &str) -> Result<bool, VaultError> {
        if let Some(db) = &self.db {
            db.remove(name.as_bytes())
                .map_err(|e| VaultError::Database(e.to_string()))?;
            db.flush()
                .map_err(|e| VaultError::Database(e.to_string()))?;
        }

        let existed = {
            let mut cache = self.cache.write().await;
            cache.remove(name).is_some()
        };

        self.backup().await?;
        Ok(existed)
    }

    /// List all key names
    pub async fn list_keys(&self) -> Vec<String> {
        let cache = self.cache.read().await;
        cache.keys().cloned().collect()
    }

    /// List entries without values
    pub async fn list_entries(&self) -> Vec<KeyEntrySummary> {
        let cache = self.cache.read().await;
        cache
            .values()
            .map(|e| KeyEntrySummary {
                name: e.name.clone(),
                service: e.service.clone(),
                active: e.active,
                created_at: e.created_at,
                last_used: e.last_used,
                usage_count: e.usage_count,
                has_value: !e.value.is_empty(),
            })
            .collect()
    }

    /// Check if key exists
    pub async fn exists(&self, name: &str) -> bool {
        let cache = self.cache.read().await;
        cache.contains_key(name)
    }

    /// Deactivate key
    pub async fn deactivate(&self, name: &str) -> Result<bool, VaultError> {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(name) {
            entry.active = false;
            if let Some(db) = &self.db {
                if let Ok(json) = serde_json::to_vec(&entry) {
                    db.insert(name.as_bytes(), json)
                        .map_err(|e| VaultError::Database(e.to_string()))?;
                    db.flush()
                        .map_err(|e| VaultError::Database(e.to_string()))?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Activate key
    pub async fn activate(&self, name: &str) -> Result<bool, VaultError> {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(name) {
            entry.active = true;
            if let Some(db) = &self.db {
                if let Ok(json) = serde_json::to_vec(&entry) {
                    db.insert(name.as_bytes(), json)
                        .map_err(|e| VaultError::Database(e.to_string()))?;
                    db.flush()
                        .map_err(|e| VaultError::Database(e.to_string()))?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Backup to JSON file
    async fn backup(&self) -> Result<(), VaultError> {
        let cache = self.cache.read().await;
        let entries: Vec<&KeyEntry> = cache.values().collect();

        let json = serde_json::to_string_pretty(&entries)
            .map_err(|e| VaultError::Serialization(e.to_string()))?;

        fs::write(&self.backup_path, &json).map_err(|e| VaultError::Io(e.to_string()))?;

        Ok(())
    }

    /// Get vault stats
    pub async fn stats(&self) -> VaultStats {
        let cache = self.cache.read().await;
        let total = cache.len();
        let active = cache.values().filter(|e| e.active).count();
        let with_value = cache.values().filter(|e| !e.value.is_empty()).count();
        
        VaultStats {
            total,
            active,
            inactive: total - active,
            with_value,
            vault_path: self.vault_dir.display().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultStats {
    pub total: usize,
    pub active: usize,
    pub inactive: usize,
    pub with_value: usize,
    pub vault_path: String,
}

/// Global vault instance
static GLOBAL_VAULT: std::sync::OnceLock<Arc<KeyVault>> = std::sync::OnceLock::new();

/// Get or create global vault
pub fn global_vault() -> Result<Arc<KeyVault>, VaultError> {
    if let Some(vault) = GLOBAL_VAULT.get() {
        return Ok(vault.clone());
    }

    let vault = Arc::new(KeyVault::new()?);
    let _ = GLOBAL_VAULT.set(vault.clone());
    Ok(vault)
}
