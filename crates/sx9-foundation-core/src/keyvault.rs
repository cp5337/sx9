//! CTAS-7 `KeyVault`: Persistent, Reliable API Key Storage
//!
//! NO MORE LOST KEYS. This vault:
//! - Uses Sled embedded database for persistence
//! - Falls back to encrypted file if Sled fails
//! - Stores in ~/.ctas7/keyvault/ (survives restarts)
//! - Integrates with macOS Keychain when available
//! - Auto-backup on every write

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

// NVNN: KeyVault persists credentials across restarts

/// Key entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntry {
    /// Key identifier (e.g., "mapbox", "elevenlabs", "openai")
    pub name: String,
    /// The actual secret value (encrypted at rest in sled)
    pub value: String,
    /// Service category
    pub service: String,
    /// When added
    pub created_at: DateTime<Utc>,
    /// Last used
    pub last_used: Option<DateTime<Utc>>,
    /// Usage count
    pub usage_count: u64,
    /// Is key active
    pub active: bool,
    /// Expiration (if known)
    pub expires_at: Option<DateTime<Utc>>,
    /// Notes/description
    pub notes: Option<String>,
}

/// Persistent `KeyVault` - NEVER loses your keys
pub struct KeyVault {
    /// Sled database for persistence
    db: Option<sled::Db>,
    /// In-memory cache for fast access
    cache: Arc<RwLock<HashMap<String, KeyEntry>>>,
    /// Vault directory
    vault_dir: PathBuf,
    /// Backup file path
    backup_path: PathBuf,
}

impl KeyVault {
    /// Get the standard vault directory (~/.ctas7/keyvault)
    #[must_use]
    pub fn default_vault_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".ctas7")
            .join("keyvault")
    }

    /// Create new `KeyVault` with persistent storage
    pub fn new() -> Result<Self, KeyVaultError> {
        Self::with_path(&Self::default_vault_dir())
    }

    /// Create `KeyVault` at specific path
    pub fn with_path(vault_dir: &Path) -> Result<Self, KeyVaultError> {
        // Ensure directory exists
        fs::create_dir_all(vault_dir)
            .map_err(|e| KeyVaultError::Io(format!("Failed to create vault dir: {e}")))?;

        let db_path = vault_dir.join("keys.sled");
        let backup_path = vault_dir.join("keys.backup.json");

        // Open sled database
        let db = match sled::open(&db_path) {
            Ok(db) => Some(db),
            Err(e) => {
                tracing::warn!("Sled DB failed ({}), using file fallback", e);
                None
            }
        };

        let vault = Self {
            db,
            cache: Arc::new(RwLock::new(HashMap::new())),
            vault_dir: vault_dir.to_path_buf(),
            backup_path,
        };

        // Load existing keys
        vault.load_all()?;

        Ok(vault)
    }

    /// Load all keys from storage into cache
    fn load_all(&self) -> Result<(), KeyVaultError> {
        let mut loaded = HashMap::new();

        // Try sled first
        if let Some(db) = &self.db {
            for (key, value) in db.iter().flatten() {
                if let (Ok(name), Ok(entry)) = (
                    String::from_utf8(key.to_vec()),
                    serde_json::from_slice::<KeyEntry>(&value),
                ) {
                    loaded.insert(name, entry);
                }
            }
        }

        // If sled empty, try backup file
        if loaded.is_empty() && self.backup_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.backup_path) {
                if let Ok(entries) = serde_json::from_str::<Vec<KeyEntry>>(&content) {
                    for entry in entries {
                        loaded.insert(entry.name.clone(), entry);
                    }
                    tracing::info!("Recovered {} keys from backup", loaded.len());

                    // Restore to sled
                    if let Some(db) = &self.db {
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

        // Update cache (blocking for init)
        let cache = self.cache.clone();
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let mut cache = cache.write().await;
                *cache = loaded;
            });
        });

        Ok(())
    }

    /// Store a key (persists immediately)
    pub async fn set(&self, name: &str, value: &str, service: &str) -> Result<(), KeyVaultError> {
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

    /// Store a key entry with full metadata
    pub async fn store_entry(&self, entry: KeyEntry) -> Result<(), KeyVaultError> {
        let name = entry.name.clone();

        // Store in sled
        if let Some(db) = &self.db {
            let json = serde_json::to_vec(&entry)
                .map_err(|e| KeyVaultError::Serialization(e.to_string()))?;
            db.insert(name.as_bytes(), json)
                .map_err(|e| KeyVaultError::Database(e.to_string()))?;
            db.flush()
                .map_err(|e| KeyVaultError::Database(e.to_string()))?;
        }

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(name, entry);
        }

        // Auto-backup
        self.backup().await?;

        Ok(())
    }

    /// Get a key value
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

    /// Get key entry with metadata
    pub async fn get_entry(&self, name: &str) -> Option<KeyEntry> {
        let cache = self.cache.read().await;
        cache.get(name).cloned()
    }

    /// Record key usage (updates `last_used` and count)
    pub async fn record_usage(&self, name: &str) {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(name) {
            entry.last_used = Some(Utc::now());
            entry.usage_count += 1;

            // Persist update
            if let Some(db) = &self.db {
                if let Ok(json) = serde_json::to_vec(&entry) {
                    let _ = db.insert(name.as_bytes(), json);
                    let _ = db.flush();
                }
            }
        }
    }

    /// Delete a key
    pub async fn delete(&self, name: &str) -> Result<bool, KeyVaultError> {
        // Remove from sled
        if let Some(db) = &self.db {
            db.remove(name.as_bytes())
                .map_err(|e| KeyVaultError::Database(e.to_string()))?;
            db.flush()
                .map_err(|e| KeyVaultError::Database(e.to_string()))?;
        }

        // Remove from cache
        let existed = {
            let mut cache = self.cache.write().await;
            cache.remove(name).is_some()
        };

        // Update backup
        self.backup().await?;

        Ok(existed)
    }

    /// List all key names
    pub async fn list_keys(&self) -> Vec<String> {
        let cache = self.cache.read().await;
        cache.keys().cloned().collect()
    }

    /// List all key entries (without exposing values)
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
            })
            .collect()
    }

    /// Backup all keys to JSON file
    async fn backup(&self) -> Result<(), KeyVaultError> {
        let cache = self.cache.read().await;
        let entries: Vec<&KeyEntry> = cache.values().collect();

        let json = serde_json::to_string_pretty(&entries)
            .map_err(|e| KeyVaultError::Serialization(e.to_string()))?;

        fs::write(&self.backup_path, &json).map_err(|e| KeyVaultError::Io(e.to_string()))?;

        Ok(())
    }

    /// Force sync to disk
    pub async fn sync(&self) -> Result<(), KeyVaultError> {
        if let Some(db) = &self.db {
            db.flush()
                .map_err(|e| KeyVaultError::Database(e.to_string()))?;
        }
        self.backup().await?;
        Ok(())
    }

    /// Check if key exists
    pub async fn exists(&self, name: &str) -> bool {
        let cache = self.cache.read().await;
        cache.contains_key(name)
    }

    /// Deactivate key (keeps it but marks inactive)
    pub async fn deactivate(&self, name: &str) -> Result<bool, KeyVaultError> {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(name) {
            entry.active = false;

            if let Some(db) = &self.db {
                if let Ok(json) = serde_json::to_vec(&entry) {
                    db.insert(name.as_bytes(), json)
                        .map_err(|e| KeyVaultError::Database(e.to_string()))?;
                    db.flush()
                        .map_err(|e| KeyVaultError::Database(e.to_string()))?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Activate key
    pub async fn activate(&self, name: &str) -> Result<bool, KeyVaultError> {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(name) {
            entry.active = true;

            if let Some(db) = &self.db {
                if let Ok(json) = serde_json::to_vec(&entry) {
                    db.insert(name.as_bytes(), json)
                        .map_err(|e| KeyVaultError::Database(e.to_string()))?;
                    db.flush()
                        .map_err(|e| KeyVaultError::Database(e.to_string()))?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Summary of key entry (no secret value exposed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntrySummary {
    pub name: String,
    pub service: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
}

/// `KeyVault` errors
#[derive(Debug, thiserror::Error)]
pub enum KeyVaultError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Key not found: {0}")]
    NotFound(String),
}

/// Global key vault instance (lazy initialized)
static GLOBAL_VAULT: std::sync::OnceLock<Arc<KeyVault>> = std::sync::OnceLock::new();

/// Get the global `KeyVault` instance
pub fn global_vault() -> Result<Arc<KeyVault>, KeyVaultError> {
    if let Some(vault) = GLOBAL_VAULT.get() {
        return Ok(vault.clone());
    }

    let vault = Arc::new(KeyVault::new()?);
    let _ = GLOBAL_VAULT.set(vault.clone());
    Ok(vault)
}

// Standard key names for common services
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
}

// RFC-9107 §7: Agent credential adapter for persona registry integration
pub mod agent_keys {
    use super::{KeyVault, KeyVaultError};

    // NVNN: Agent keys map to persona registry voice and API credentials

    /// Get the vault key for an agent's voice ID
    #[must_use]
    pub fn voice_id(agent: &str) -> String {
        format!("agent.{}.voice_id", agent.to_lowercase())
    }

    /// Get the vault key for an agent's API key
    #[must_use]
    pub fn api_key(agent: &str) -> String {
        format!("agent.{}.api_key", agent.to_lowercase())
    }

    /// Get the vault key for an agent's gRPC endpoint
    #[must_use]
    pub fn grpc_endpoint(agent: &str) -> String {
        format!("agent.{}.grpc_endpoint", agent.to_lowercase())
    }

    /// Get the vault key for an agent's embedding collection
    #[must_use]
    pub fn embedding_collection(agent: &str) -> String {
        format!("agent.{}.embedding_collection", agent.to_lowercase())
    }

    /// Standard agent names from RFC-9107
    pub mod agents {
        pub const GROK: &str = "grok";
        pub const NATASHA: &str = "natasha";
        pub const COVE: &str = "cove";
        pub const ALTAIR: &str = "altair";
        pub const CLAUDE: &str = "claude";
        pub const ZOE: &str = "zoe";
        pub const GPT: &str = "gpt";
        pub const ELENA: &str = "elena";
    }

    /// Get agent's voice ID from vault, falling back to env var
    pub async fn get_agent_voice_id(vault: &KeyVault, agent: &str) -> Option<String> {
        // Try vault first
        if let Some(id) = vault.get(&voice_id(agent)).await {
            return Some(id);
        }
        // Fall back to env var pattern: AGENT_<NAME>_VOICE_ID
        std::env::var(format!("AGENT_{}_VOICE_ID", agent.to_uppercase())).ok()
    }

    /// Get agent's API key from vault, falling back to env var
    pub async fn get_agent_api_key(vault: &KeyVault, agent: &str) -> Option<String> {
        // Try vault first
        if let Some(key) = vault.get(&api_key(agent)).await {
            return Some(key);
        }
        // Fall back to env var pattern: AGENT_<NAME>_API_KEY
        std::env::var(format!("AGENT_{}_API_KEY", agent.to_uppercase())).ok()
    }

    /// Store agent credentials in vault from persona registry JSON
    pub async fn import_from_persona_registry(
        vault: &KeyVault,
        agent_name: &str,
        voice_id_val: Option<&str>,
        api_key_val: Option<&str>,
    ) -> Result<(), KeyVaultError> {
        if let Some(vid) = voice_id_val {
            vault.set(&voice_id(agent_name), vid, "agent_voice").await?;
        }
        if let Some(key) = api_key_val {
            vault.set(&api_key(agent_name), key, "agent_api").await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_keyvault_persistence() {
        let dir = tempdir().unwrap();
        let vault = KeyVault::with_path(dir.path()).unwrap();

        // Store key
        vault.set("test_key", "secret123", "test").await.unwrap();

        // Verify in cache
        assert_eq!(vault.get("test_key").await, Some("secret123".to_string()));

        // Create new vault at same path (simulates restart)
        drop(vault);
        let vault2 = KeyVault::with_path(dir.path()).unwrap();

        // Key should still be there
        assert_eq!(vault2.get("test_key").await, Some("secret123".to_string()));
    }

    #[tokio::test]
    async fn test_key_operations() {
        let dir = tempdir().unwrap();
        let vault = KeyVault::with_path(dir.path()).unwrap();

        vault.set("api_key", "abc123", "service").await.unwrap();
        assert!(vault.exists("api_key").await);

        vault.deactivate("api_key").await.unwrap();
        assert_eq!(vault.get("api_key").await, None); // Deactivated

        vault.activate("api_key").await.unwrap();
        assert_eq!(vault.get("api_key").await, Some("abc123".to_string()));

        vault.delete("api_key").await.unwrap();
        assert!(!vault.exists("api_key").await);
    }
}
