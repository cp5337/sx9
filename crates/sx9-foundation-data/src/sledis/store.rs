//! Sledis Store - Sled-backed key-value storage
//!
//! Provides the storage layer for Sledis with < 3μs lookup target.

use super::{SledisEntry, SledisError, SledisResult, SledisValue};
use sled::Db;
use std::path::Path;
use std::sync::Arc;

/// Sledis store backed by Sled
pub struct SledisStore {
    db: Arc<Db>,
}

impl SledisStore {
    /// Open or create a Sledis store
    pub fn open<P: AsRef<Path>>(path: P) -> SledisResult<Self> {
        let db = sled::open(path)?;
        Ok(Self { db: Arc::new(db) })
    }

    /// Open with default path (~/.sx9/sledis)
    pub fn open_default() -> SledisResult<Self> {
        let path = shellexpand::tilde("~/.sx9/sledis").to_string();
        Self::open(path)
    }

    // =========================================================================
    // String Operations
    // =========================================================================

    /// GET key
    pub fn get(&self, key: &str) -> SledisResult<Option<SledisValue>> {
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let entry: SledisEntry = serde_json::from_slice(&data)
                    .map_err(|e| SledisError::Serialization(e.to_string()))?;

                if entry.is_expired() {
                    // Clean up expired key
                    self.db.remove(key.as_bytes())?;
                    Ok(None)
                } else {
                    Ok(Some(entry.value))
                }
            }
            None => Ok(None),
        }
    }

    /// SET key value [EX seconds]
    pub fn set(&self, key: &str, value: SledisValue, ttl: Option<u64>) -> SledisResult<()> {
        let mut entry = SledisEntry::new(value);
        if let Some(seconds) = ttl {
            entry = entry.with_ttl(seconds);
        }

        let data = serde_json::to_vec(&entry)
            .map_err(|e| SledisError::Serialization(e.to_string()))?;
        self.db.insert(key.as_bytes(), data)?;
        Ok(())
    }

    /// DEL key [key ...]
    pub fn del(&self, keys: &[&str]) -> SledisResult<usize> {
        let mut deleted = 0;
        for key in keys {
            if self.db.remove(key.as_bytes())?.is_some() {
                deleted += 1;
            }
        }
        Ok(deleted)
    }

    /// EXISTS key
    pub fn exists(&self, key: &str) -> SledisResult<bool> {
        match self.get(key)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// EXPIRE key seconds
    pub fn expire(&self, key: &str, seconds: u64) -> SledisResult<bool> {
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let mut entry: SledisEntry = serde_json::from_slice(&data)
                    .map_err(|e| SledisError::Serialization(e.to_string()))?;

                if entry.is_expired() {
                    self.db.remove(key.as_bytes())?;
                    return Ok(false);
                }

                entry = SledisEntry::new(entry.value).with_ttl(seconds);
                if let Some(hash) = entry.trivariate_hash.take() {
                    entry = entry.with_hash(hash);
                }

                let data = serde_json::to_vec(&entry)
                    .map_err(|e| SledisError::Serialization(e.to_string()))?;
                self.db.insert(key.as_bytes(), data)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// TTL key
    pub fn ttl(&self, key: &str) -> SledisResult<Option<i64>> {
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let entry: SledisEntry = serde_json::from_slice(&data)
                    .map_err(|e| SledisError::Serialization(e.to_string()))?;

                if entry.is_expired() {
                    self.db.remove(key.as_bytes())?;
                    return Ok(None);
                }

                Ok(entry.ttl())
            }
            None => Ok(None),
        }
    }

    // =========================================================================
    // Hash Operations
    // =========================================================================

    /// HGET key field
    pub fn hget(&self, key: &str, field: &str) -> SledisResult<Option<String>> {
        match self.get(key)? {
            Some(SledisValue::Hash(hash)) => Ok(hash.get(field).cloned()),
            Some(_) => Err(SledisError::WrongType),
            None => Ok(None),
        }
    }

    /// HSET key field value
    pub fn hset(&self, key: &str, field: &str, value: &str) -> SledisResult<bool> {
        let mut hash = match self.get(key)? {
            Some(SledisValue::Hash(h)) => h,
            Some(_) => return Err(SledisError::WrongType),
            None => std::collections::HashMap::new(),
        };

        let is_new = !hash.contains_key(field);
        hash.insert(field.to_string(), value.to_string());
        self.set(key, SledisValue::Hash(hash), None)?;
        Ok(is_new)
    }

    /// HDEL key field
    pub fn hdel(&self, key: &str, field: &str) -> SledisResult<bool> {
        match self.get(key)? {
            Some(SledisValue::Hash(mut hash)) => {
                let removed = hash.remove(field).is_some();
                self.set(key, SledisValue::Hash(hash), None)?;
                Ok(removed)
            }
            Some(_) => Err(SledisError::WrongType),
            None => Ok(false),
        }
    }

    /// HGETALL key
    pub fn hgetall(&self, key: &str) -> SledisResult<std::collections::HashMap<String, String>> {
        match self.get(key)? {
            Some(SledisValue::Hash(hash)) => Ok(hash),
            Some(_) => Err(SledisError::WrongType),
            None => Ok(std::collections::HashMap::new()),
        }
    }

    // =========================================================================
    // List Operations
    // =========================================================================

    /// LPUSH key value
    pub fn lpush(&self, key: &str, value: &str) -> SledisResult<usize> {
        let mut list = match self.get(key)? {
            Some(SledisValue::List(l)) => l,
            Some(_) => return Err(SledisError::WrongType),
            None => Vec::new(),
        };

        list.insert(0, value.to_string());
        let len = list.len();
        self.set(key, SledisValue::List(list), None)?;
        Ok(len)
    }

    /// RPUSH key value
    pub fn rpush(&self, key: &str, value: &str) -> SledisResult<usize> {
        let mut list = match self.get(key)? {
            Some(SledisValue::List(l)) => l,
            Some(_) => return Err(SledisError::WrongType),
            None => Vec::new(),
        };

        list.push(value.to_string());
        let len = list.len();
        self.set(key, SledisValue::List(list), None)?;
        Ok(len)
    }

    /// LPOP key
    pub fn lpop(&self, key: &str) -> SledisResult<Option<String>> {
        match self.get(key)? {
            Some(SledisValue::List(mut list)) => {
                if list.is_empty() {
                    return Ok(None);
                }
                let value = list.remove(0);
                self.set(key, SledisValue::List(list), None)?;
                Ok(Some(value))
            }
            Some(_) => Err(SledisError::WrongType),
            None => Ok(None),
        }
    }

    /// RPOP key
    pub fn rpop(&self, key: &str) -> SledisResult<Option<String>> {
        match self.get(key)? {
            Some(SledisValue::List(mut list)) => {
                let value = list.pop();
                self.set(key, SledisValue::List(list), None)?;
                Ok(value)
            }
            Some(_) => Err(SledisError::WrongType),
            None => Ok(None),
        }
    }

    /// LRANGE key start stop
    pub fn lrange(&self, key: &str, start: i64, stop: i64) -> SledisResult<Vec<String>> {
        match self.get(key)? {
            Some(SledisValue::List(list)) => {
                let len = list.len() as i64;
                let start = if start < 0 { (len + start).max(0) } else { start } as usize;
                let stop = if stop < 0 { (len + stop).max(0) } else { stop } as usize;

                if start >= list.len() {
                    return Ok(Vec::new());
                }

                let end = (stop + 1).min(list.len());
                Ok(list[start..end].to_vec())
            }
            Some(_) => Err(SledisError::WrongType),
            None => Ok(Vec::new()),
        }
    }

    // =========================================================================
    // Set Operations
    // =========================================================================

    /// SADD key member
    pub fn sadd(&self, key: &str, member: &str) -> SledisResult<bool> {
        let mut set = match self.get(key)? {
            Some(SledisValue::Set(s)) => s,
            Some(_) => return Err(SledisError::WrongType),
            None => std::collections::HashSet::new(),
        };

        let is_new = set.insert(member.to_string());
        self.set(key, SledisValue::Set(set), None)?;
        Ok(is_new)
    }

    /// SREM key member
    pub fn srem(&self, key: &str, member: &str) -> SledisResult<bool> {
        match self.get(key)? {
            Some(SledisValue::Set(mut set)) => {
                let removed = set.remove(member);
                self.set(key, SledisValue::Set(set), None)?;
                Ok(removed)
            }
            Some(_) => Err(SledisError::WrongType),
            None => Ok(false),
        }
    }

    /// SMEMBERS key
    pub fn smembers(&self, key: &str) -> SledisResult<std::collections::HashSet<String>> {
        match self.get(key)? {
            Some(SledisValue::Set(set)) => Ok(set),
            Some(_) => Err(SledisError::WrongType),
            None => Ok(std::collections::HashSet::new()),
        }
    }

    /// SISMEMBER key member
    pub fn sismember(&self, key: &str, member: &str) -> SledisResult<bool> {
        match self.get(key)? {
            Some(SledisValue::Set(set)) => Ok(set.contains(member)),
            Some(_) => Err(SledisError::WrongType),
            None => Ok(false),
        }
    }

    // =========================================================================
    // Trivariate Hash Operations (SX9 Extension)
    // =========================================================================

    /// HASHSET key value trivariate_hash - Set with trivariate hash
    pub fn set_with_hash(&self, key: &str, value: SledisValue, hash: &str, ttl: Option<u64>) -> SledisResult<()> {
        let mut entry = SledisEntry::new(value).with_hash(hash.to_string());
        if let Some(seconds) = ttl {
            entry = entry.with_ttl(seconds);
        }

        let data = serde_json::to_vec(&entry)
            .map_err(|e| SledisError::Serialization(e.to_string()))?;
        self.db.insert(key.as_bytes(), data)?;
        Ok(())
    }

    /// HASHGET key - Get trivariate hash for key
    pub fn get_hash(&self, key: &str) -> SledisResult<Option<String>> {
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let entry: SledisEntry = serde_json::from_slice(&data)
                    .map_err(|e| SledisError::Serialization(e.to_string()))?;

                if entry.is_expired() {
                    self.db.remove(key.as_bytes())?;
                    return Ok(None);
                }

                Ok(entry.trivariate_hash)
            }
            None => Ok(None),
        }
    }

    // =========================================================================
    // Admin Operations
    // =========================================================================

    /// FLUSHDB - Clear all keys
    pub fn flushdb(&self) -> SledisResult<()> {
        self.db.clear()?;
        Ok(())
    }

    /// DBSIZE - Get number of keys
    pub fn dbsize(&self) -> SledisResult<usize> {
        Ok(self.db.len())
    }

    /// Flush to disk
    pub fn flush(&self) -> SledisResult<()> {
        self.db.flush()?;
        Ok(())
    }
}

impl std::fmt::Debug for SledisStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SledisStore")
            .field("keys", &self.db.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_store() -> (SledisStore, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let store = SledisStore::open(temp_dir.path().join("sledis.db")).unwrap();
        (store, temp_dir)
    }

    #[test]
    fn test_string_operations() {
        let (store, _dir) = create_test_store();

        // SET and GET
        store.set("key1", SledisValue::String("value1".to_string()), None).unwrap();
        let value = store.get("key1").unwrap();
        assert_eq!(value, Some(SledisValue::String("value1".to_string())));

        // EXISTS
        assert!(store.exists("key1").unwrap());
        assert!(!store.exists("nonexistent").unwrap());

        // DEL
        assert_eq!(store.del(&["key1"]).unwrap(), 1);
        assert!(!store.exists("key1").unwrap());
    }

    #[test]
    fn test_hash_operations() {
        let (store, _dir) = create_test_store();

        // HSET and HGET
        assert!(store.hset("hash1", "field1", "value1").unwrap());
        assert_eq!(store.hget("hash1", "field1").unwrap(), Some("value1".to_string()));

        // HGETALL
        store.hset("hash1", "field2", "value2").unwrap();
        let all = store.hgetall("hash1").unwrap();
        assert_eq!(all.len(), 2);

        // HDEL
        assert!(store.hdel("hash1", "field1").unwrap());
        assert_eq!(store.hget("hash1", "field1").unwrap(), None);
    }

    #[test]
    fn test_list_operations() {
        let (store, _dir) = create_test_store();

        // LPUSH and RPUSH
        store.lpush("list1", "a").unwrap();
        store.rpush("list1", "b").unwrap();
        store.lpush("list1", "c").unwrap();

        // LRANGE
        let range = store.lrange("list1", 0, -1).unwrap();
        assert_eq!(range, vec!["c", "a", "b"]);

        // LPOP and RPOP
        assert_eq!(store.lpop("list1").unwrap(), Some("c".to_string()));
        assert_eq!(store.rpop("list1").unwrap(), Some("b".to_string()));
    }

    #[test]
    fn test_set_operations() {
        let (store, _dir) = create_test_store();

        // SADD
        assert!(store.sadd("set1", "a").unwrap());
        assert!(store.sadd("set1", "b").unwrap());
        assert!(!store.sadd("set1", "a").unwrap()); // Already exists

        // SMEMBERS
        let members = store.smembers("set1").unwrap();
        assert!(members.contains("a"));
        assert!(members.contains("b"));

        // SISMEMBER
        assert!(store.sismember("set1", "a").unwrap());
        assert!(!store.sismember("set1", "c").unwrap());

        // SREM
        assert!(store.srem("set1", "a").unwrap());
        assert!(!store.sismember("set1", "a").unwrap());
    }

    #[test]
    fn test_trivariate_hash() {
        let (store, _dir) = create_test_store();

        let hash = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4";
        store.set_with_hash("hashed_key", SledisValue::String("data".to_string()), hash, None).unwrap();

        assert_eq!(store.get_hash("hashed_key").unwrap(), Some(hash.to_string()));
    }
}
