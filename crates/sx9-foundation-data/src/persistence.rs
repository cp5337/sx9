//! Persistence layer for CTAS-7 v7.2 foundation data

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::storage::{Storage, StorageBackend};
use crate::hash::TrivariateHashEngine;

/// Persistent data record with CTAS-7 v7.2 compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentRecord {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub record_type: String,
    pub data_hash: String,
    pub content: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub version: u32,
}

impl PersistentRecord {
    /// Create new persistent record
    pub fn new(record_type: String, content: serde_json::Value) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            record_type,
            data_hash: String::new(),
            content,
            metadata: HashMap::new(),
            version: 1,
        }
    }

    /// Update record content
    pub fn update_content(&mut self, content: serde_json::Value) {
        self.content = content;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = Utc::now();
    }

    /// Set data hash
    pub fn set_hash(&mut self, hash: String) {
        self.data_hash = hash;
        self.updated_at = Utc::now();
    }
}

/// Persistence manager for CTAS-7 v7.2
#[derive(Debug, Clone)]
pub struct PersistenceManager {
    storage: StorageBackend,
    hash_engine: TrivariateHashEngine,
}

impl PersistenceManager {
    /// Create new persistence manager
    pub fn new() -> Result<Self> {
        let storage = StorageBackend::new()?;
        let hash_engine = TrivariateHashEngine::new();

        Ok(Self {
            storage,
            hash_engine,
        })
    }

    /// Create with specific storage backend
    pub fn with_storage(storage: StorageBackend) -> Self {
        let hash_engine = TrivariateHashEngine::new();
        Self {
            storage,
            hash_engine,
        }
    }

    /// Store persistent record
    pub async fn store_record(&self, mut record: PersistentRecord) -> Result<String> {
        // Generate hash for the record
        let record_key = format!("record:{}", record.id);
        let serialized = serde_json::to_string(&record.content)?;
        let hash = self.hash_engine.generate_hash(&record_key, &serialized)?;

        record.set_hash(hash.clone());

        // Store the record
        let record_data = serde_json::to_string(&record)?;
        self.storage.store(&record_key, &record_data).await?;

        // Store hash index
        let hash_key = format!("hash:{}", hash);
        self.storage.store(&hash_key, &record.id.to_string()).await?;

        Ok(hash)
    }

    /// Retrieve record by ID
    pub async fn get_record(&self, id: Uuid) -> Result<Option<PersistentRecord>> {
        let record_key = format!("record:{}", id);

        if let Some(data) = self.storage.retrieve(&record_key).await? {
            let record: PersistentRecord = serde_json::from_str(&data)?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    /// Retrieve record by hash
    pub async fn get_record_by_hash(&self, hash: &str) -> Result<Option<PersistentRecord>> {
        let hash_key = format!("hash:{}", hash);

        if let Some(id_str) = self.storage.retrieve(&hash_key).await? {
            let id = Uuid::parse_str(&id_str)?;
            self.get_record(id).await
        } else {
            Ok(None)
        }
    }

    /// Update existing record
    pub async fn update_record(&self, id: Uuid, content: serde_json::Value) -> Result<Option<String>> {
        if let Some(mut record) = self.get_record(id).await? {
            record.update_content(content);
            let new_hash = self.store_record(record).await?;
            Ok(Some(new_hash))
        } else {
            Ok(None)
        }
    }

    /// Delete record
    pub async fn delete_record(&self, id: Uuid) -> Result<bool> {
        // Get record to find its hash
        if let Some(record) = self.get_record(id).await? {
            let record_key = format!("record:{}", id);
            let hash_key = format!("hash:{}", record.data_hash);

            // Delete both record and hash index
            let record_deleted = self.storage.delete(&record_key).await?;
            let _hash_deleted = self.storage.delete(&hash_key).await?;

            Ok(record_deleted)
        } else {
            Ok(false)
        }
    }

    /// List all records by type
    pub async fn list_records_by_type(&self, record_type: &str) -> Result<Vec<PersistentRecord>> {
        let keys = self.storage.list_keys().await?;
        let mut records = Vec::new();

        for key in keys {
            if key.starts_with("record:") {
                if let Some(data) = self.storage.retrieve(&key).await? {
                    if let Ok(record) = serde_json::from_str::<PersistentRecord>(&data) {
                        if record.record_type == record_type {
                            records.push(record);
                        }
                    }
                }
            }
        }

        Ok(records)
    }

    /// Get storage statistics
    pub async fn get_statistics(&self) -> Result<PersistenceStatistics> {
        let keys = self.storage.list_keys().await?;

        let mut record_count = 0;
        let mut hash_count = 0;
        let mut record_types = HashMap::new();

        for key in &keys {
            if key.starts_with("record:") {
                record_count += 1;

                // Count by type
                if let Some(data) = self.storage.retrieve(key).await? {
                    if let Ok(record) = serde_json::from_str::<PersistentRecord>(&data) {
                        *record_types.entry(record.record_type).or_insert(0) += 1;
                    }
                }
            } else if key.starts_with("hash:") {
                hash_count += 1;
            }
        }

        Ok(PersistenceStatistics {
            total_records: record_count,
            total_hashes: hash_count,
            records_by_type: record_types,
            timestamp: Utc::now(),
        })
    }

    /// Verify data integrity
    pub async fn verify_integrity(&self) -> Result<IntegrityReport> {
        let keys = self.storage.list_keys().await?;
        let mut report = IntegrityReport::new();

        for key in keys {
            if key.starts_with("record:") {
                if let Some(data) = self.storage.retrieve(&key).await? {
                    if let Ok(record) = serde_json::from_str::<PersistentRecord>(&data) {
                        // Verify hash
                        let record_key = format!("record:{}", record.id);
                        let serialized = serde_json::to_string(&record.content)?;

                        if self.hash_engine.verify_hash(&record_key, &serialized, &record.data_hash)? {
                            report.valid_records += 1;
                        } else {
                            report.invalid_records += 1;
                            report.errors.push(format!("Invalid hash for record {}", record.id));
                        }
                    } else {
                        report.corrupted_records += 1;
                        report.errors.push(format!("Corrupted record data for key {}", key));
                    }
                }
            }
        }

        Ok(report)
    }
}

/// Persistence statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceStatistics {
    pub total_records: usize,
    pub total_hashes: usize,
    pub records_by_type: HashMap<String, usize>,
    pub timestamp: DateTime<Utc>,
}

/// Data integrity report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityReport {
    pub valid_records: usize,
    pub invalid_records: usize,
    pub corrupted_records: usize,
    pub errors: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl IntegrityReport {
    fn new() -> Self {
        Self {
            valid_records: 0,
            invalid_records: 0,
            corrupted_records: 0,
            errors: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.invalid_records == 0 && self.corrupted_records == 0
    }
}

impl Default for PersistenceManager {
    fn default() -> Self {
        Self::new().expect("Failed to create persistence manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_persistence_manager() {
        let manager = PersistenceManager::with_storage(StorageBackend::memory());

        let record = PersistentRecord::new(
            "test".to_string(),
            serde_json::json!({"message": "Hello CTAS-7"}),
        );
        let id = record.id;

        let hash = manager.store_record(record).await.unwrap();
        assert!(!hash.is_empty());

        let retrieved = manager.get_record(id).await.unwrap();
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.record_type, "test");
        assert_eq!(retrieved.data_hash, hash);
    }

    #[tokio::test]
    async fn test_record_by_hash() {
        let manager = PersistenceManager::with_storage(StorageBackend::memory());

        let record = PersistentRecord::new(
            "test".to_string(),
            serde_json::json!({"message": "Hello CTAS-7"}),
        );

        let hash = manager.store_record(record).await.unwrap();
        let retrieved = manager.get_record_by_hash(&hash).await.unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data_hash, hash);
    }

    #[tokio::test]
    async fn test_integrity_verification() {
        let manager = PersistenceManager::with_storage(StorageBackend::memory());

        let record = PersistentRecord::new(
            "test".to_string(),
            serde_json::json!({"message": "Hello CTAS-7"}),
        );

        let _hash = manager.store_record(record).await.unwrap();
        let report = manager.verify_integrity().await.unwrap();

        assert!(report.is_healthy());
        assert_eq!(report.valid_records, 1);
    }
}