//! Storage backend abstraction for CTAS-7 v7.2

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[cfg(feature = "embedded-db")]
use sled::Db;

/// Storage backend trait
#[async_trait]
pub trait Storage: Send + Sync {
    async fn store(&self, key: &str, value: &str) -> Result<()>;
    async fn retrieve(&self, key: &str) -> Result<Option<String>>;
    async fn delete(&self, key: &str) -> Result<bool>;
    async fn list_keys(&self) -> Result<Vec<String>>;
}

/// Storage backend implementation
#[derive(Debug, Clone)]
pub enum StorageBackend {
    Memory(Arc<RwLock<HashMap<String, String>>>),
    #[cfg(feature = "embedded-db")]
    Sled(Arc<Db>),
}

impl StorageBackend {
    /// Create new storage backend
    pub fn new() -> Result<Self> {
        #[cfg(feature = "embedded-db")]
        {
            let db = sled::open("ctas7-foundation-data.db")?;
            Ok(StorageBackend::Sled(Arc::new(db)))
        }
        #[cfg(not(feature = "embedded-db"))]
        {
            Ok(StorageBackend::Memory(Arc::new(
                RwLock::new(HashMap::new()),
            )))
        }
    }

    /// Create memory-based storage
    pub fn memory() -> Self {
        StorageBackend::Memory(Arc::new(RwLock::new(HashMap::new())))
    }
}

#[async_trait]
impl Storage for StorageBackend {
    async fn store(&self, key: &str, value: &str) -> Result<()> {
        match self {
            StorageBackend::Memory(map) => {
                let mut map = map.write().unwrap();
                map.insert(key.to_string(), value.to_string());
                Ok(())
            }
            #[cfg(feature = "embedded-db")]
            StorageBackend::Sled(db) => {
                db.insert(key.as_bytes(), value.as_bytes())?;
                Ok(())
            }
        }
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>> {
        match self {
            StorageBackend::Memory(map) => {
                let map = map.read().unwrap();
                Ok(map.get(key).cloned())
            }
            #[cfg(feature = "embedded-db")]
            StorageBackend::Sled(db) => {
                if let Some(value) = db.get(key.as_bytes())? {
                    Ok(Some(String::from_utf8(value.to_vec())?))
                } else {
                    Ok(None)
                }
            }
        }
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        match self {
            StorageBackend::Memory(map) => {
                let mut map = map.write().unwrap();
                Ok(map.remove(key).is_some())
            }
            #[cfg(feature = "embedded-db")]
            StorageBackend::Sled(db) => Ok(db.remove(key.as_bytes())?.is_some()),
        }
    }

    async fn list_keys(&self) -> Result<Vec<String>> {
        match self {
            StorageBackend::Memory(map) => {
                let map = map.read().unwrap();
                Ok(map.keys().cloned().collect())
            }
            #[cfg(feature = "embedded-db")]
            StorageBackend::Sled(db) => {
                let mut keys = Vec::new();
                for key in db.iter().keys() {
                    let key = key?;
                    keys.push(String::from_utf8(key.to_vec())?);
                }
                Ok(keys)
            }
        }
    }
}
