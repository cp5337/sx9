//! Sled-backed persistent memory provider
//!
//! This module provides a persistent memory store using the Sled embedded database.
//! Sled is obtained from sx9-foundation-data (canonical source).

use async_trait::async_trait;
use chrono::Utc;
use std::path::Path;

// Sled from foundation-data (canonical dependency source)
use sx9_foundation_data::sled;

use super::{MemoryEntry, MemoryProvider};
use crate::{ClaudeError, Result};

/// Sled-backed persistent memory provider
pub struct SledMemory {
    db: sled::Db,
}

impl SledMemory {
    /// Open or create a Sled database at the given path
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path).map_err(|e| ClaudeError::Memory(e.to_string()))?;
        Ok(Self { db })
    }

    /// Create a temporary in-memory Sled database (for testing)
    pub fn temporary() -> Result<Self> {
        let config = sled::Config::new().temporary(true);
        let db = config.open().map_err(|e| ClaudeError::Memory(e.to_string()))?;
        Ok(Self { db })
    }

    /// Get the underlying Sled database
    pub fn db(&self) -> &sled::Db {
        &self.db
    }
}

#[async_trait]
impl MemoryProvider for SledMemory {
    async fn store(&self, key: &str, content: &str) -> Result<()> {
        let entry = MemoryEntry::new(key, content);
        let value = serde_json::to_vec(&entry)?;

        self.db
            .insert(key.as_bytes(), value)
            .map_err(|e| ClaudeError::Memory(e.to_string()))?;

        self.db
            .flush_async()
            .await
            .map_err(|e| ClaudeError::Memory(e.to_string()))?;

        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>> {
        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                let mut entry: MemoryEntry = serde_json::from_slice(&value)?;

                // Update access time
                entry.accessed_at = Utc::now();
                let updated_value = serde_json::to_vec(&entry)?;
                self.db
                    .insert(key.as_bytes(), updated_value)
                    .map_err(|e| ClaudeError::Memory(e.to_string()))?;

                Ok(Some(entry.content))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ClaudeError::Memory(e.to_string())),
        }
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for item in self.db.iter() {
            let (_, value) = item.map_err(|e| ClaudeError::Memory(e.to_string()))?;

            if let Ok(entry) = serde_json::from_slice::<MemoryEntry>(&value) {
                if entry.content.to_lowercase().contains(&query_lower)
                    || entry.key.to_lowercase().contains(&query_lower)
                    || entry.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                {
                    results.push(entry);
                    if results.len() >= limit {
                        break;
                    }
                }
            }
        }

        Ok(results)
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        let existed = self
            .db
            .remove(key.as_bytes())
            .map_err(|e| ClaudeError::Memory(e.to_string()))?
            .is_some();

        self.db
            .flush_async()
            .await
            .map_err(|e| ClaudeError::Memory(e.to_string()))?;

        Ok(existed)
    }

    async fn list_keys(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();

        for item in self.db.iter() {
            let (key, _) = item.map_err(|e| ClaudeError::Memory(e.to_string()))?;
            if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                keys.push(key_str);
            }
        }

        Ok(keys)
    }

    async fn clear(&self) -> Result<()> {
        self.db.clear().map_err(|e| ClaudeError::Memory(e.to_string()))?;

        self.db
            .flush_async()
            .await
            .map_err(|e| ClaudeError::Memory(e.to_string()))?;

        Ok(())
    }
}

impl std::fmt::Debug for SledMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SledMemory").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sled_memory() {
        let memory = SledMemory::temporary().unwrap();

        // Store
        memory.store("key1", "value1").await.unwrap();
        memory.store("key2", "value2").await.unwrap();

        // Retrieve
        let value = memory.retrieve("key1").await.unwrap();
        assert_eq!(value, Some("value1".to_string()));

        // Search
        let results = memory.search("value", 10).await.unwrap();
        assert_eq!(results.len(), 2);

        // List keys
        let keys = memory.list_keys().await.unwrap();
        assert_eq!(keys.len(), 2);

        // Delete
        let deleted = memory.delete("key1").await.unwrap();
        assert!(deleted);

        let value = memory.retrieve("key1").await.unwrap();
        assert!(value.is_none());

        // Clear
        memory.clear().await.unwrap();
        let keys = memory.list_keys().await.unwrap();
        assert!(keys.is_empty());
    }
}
