//! Sledis: Redis-Protocol Cache Layer over Sled
//!
//! Provides Redis protocol (RESP) compatibility over the Sled embedded database.
//! Designed for hot-path cache operations with < 3μs lookup latency.
//!
//! ## RFC Reference
//! - RFC-9005: Unified Schema Specification
//! - RFC-9001: Trivariate Hashing Standard
//!
//! ## Port Allocation
//! - Sled (native): 18400
//! - Sledis (RESP): 18401

mod commands;
mod protocol;
mod server;
mod store;

pub use commands::*;
pub use protocol::*;
pub use server::*;
pub use store::*;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Default Sledis port (Redis-compatible)
pub const SLEDIS_PORT: u16 = 18401;

/// Sledis value types (Redis-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SledisValue {
    String(String),
    Integer(i64),
    Float(f64),
    Hash(HashMap<String, String>),
    List(Vec<String>),
    Set(HashSet<String>),
    Null,
}

impl SledisValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            SledisValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            SledisValue::Integer(i) => Some(*i),
            SledisValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

/// Sledis entry with TTL and trivariate hash support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SledisEntry {
    pub value: SledisValue,
    pub created_at: u64,  // Unix timestamp ms
    pub expires_at: Option<u64>,  // Unix timestamp ms
    pub trivariate_hash: Option<String>,  // 48-char hash
}

impl SledisEntry {
    pub fn new(value: SledisValue) -> Self {
        Self {
            value,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            expires_at: None,
            trivariate_hash: None,
        }
    }

    pub fn with_ttl(mut self, ttl_seconds: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        self.expires_at = Some(now + (ttl_seconds * 1000));
        self
    }

    pub fn with_hash(mut self, hash: String) -> Self {
        self.trivariate_hash = Some(hash);
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            now > expires_at
        } else {
            false
        }
    }

    pub fn ttl(&self) -> Option<i64> {
        self.expires_at.map(|expires_at| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            ((expires_at as i64) - (now as i64)) / 1000
        })
    }
}

/// Sledis error types
#[derive(Debug, thiserror::Error)]
pub enum SledisError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Wrong type operation")]
    WrongType,

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Connection error: {0}")]
    Connection(String),
}

impl From<sled::Error> for SledisError {
    fn from(e: sled::Error) -> Self {
        SledisError::Storage(e.to_string())
    }
}

pub type SledisResult<T> = Result<T, SledisError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sledis_entry_creation() {
        let entry = SledisEntry::new(SledisValue::String("test".to_string()));
        assert!(!entry.is_expired());
        assert!(entry.ttl().is_none());
    }

    #[test]
    fn test_sledis_entry_with_ttl() {
        let entry = SledisEntry::new(SledisValue::String("test".to_string()))
            .with_ttl(60);
        assert!(!entry.is_expired());
        let ttl = entry.ttl().unwrap();
        assert!(ttl > 0 && ttl <= 60);
    }

    #[test]
    fn test_sledis_entry_with_hash() {
        let hash = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6".to_string();
        let entry = SledisEntry::new(SledisValue::Integer(42))
            .with_hash(hash.clone());
        assert_eq!(entry.trivariate_hash, Some(hash));
    }
}
