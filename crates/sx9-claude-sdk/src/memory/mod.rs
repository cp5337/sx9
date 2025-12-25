//! Memory providers for persistent context
//!
//! This module provides abstractions for storing and retrieving conversation
//! context and other persistent data.

mod context;
mod persistent;
mod semantic;

pub use context::ContextWindow;
pub use persistent::SledMemory;
pub use semantic::SemanticMemory;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Result;

/// A memory entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique key for this entry
    pub key: String,

    /// The stored content
    pub content: String,

    /// When this entry was created
    pub created_at: DateTime<Utc>,

    /// When this entry was last accessed
    pub accessed_at: DateTime<Utc>,

    /// Optional metadata tags
    pub tags: Vec<String>,

    /// Relevance score (for search results)
    #[serde(default)]
    pub relevance: f32,
}

impl MemoryEntry {
    /// Create a new memory entry
    pub fn new(key: impl Into<String>, content: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            key: key.into(),
            content: content.into(),
            created_at: now,
            accessed_at: now,
            tags: Vec::new(),
            relevance: 0.0,
        }
    }

    /// Add tags to this entry
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Trait for memory providers
#[async_trait]
pub trait MemoryProvider: Send + Sync {
    /// Store a value with the given key
    async fn store(&self, key: &str, content: &str) -> Result<()>;

    /// Retrieve a value by key
    async fn retrieve(&self, key: &str) -> Result<Option<String>>;

    /// Search for entries matching a query
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>>;

    /// Delete an entry by key
    async fn delete(&self, key: &str) -> Result<bool>;

    /// List all keys
    async fn list_keys(&self) -> Result<Vec<String>>;

    /// Clear all entries
    async fn clear(&self) -> Result<()>;
}

/// In-memory provider for testing
#[derive(Default)]
pub struct InMemoryProvider {
    entries: std::sync::RwLock<std::collections::HashMap<String, MemoryEntry>>,
}

impl InMemoryProvider {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl MemoryProvider for InMemoryProvider {
    async fn store(&self, key: &str, content: &str) -> Result<()> {
        let mut entries = self.entries.write().unwrap();
        let entry = MemoryEntry::new(key, content);
        entries.insert(key.to_string(), entry);
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>> {
        let entries = self.entries.read().unwrap();
        Ok(entries.get(key).map(|e| e.content.clone()))
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>> {
        let entries = self.entries.read().unwrap();
        let query_lower = query.to_lowercase();

        let mut results: Vec<_> = entries
            .values()
            .filter(|e| e.content.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();

        results.truncate(limit);
        Ok(results)
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        let mut entries = self.entries.write().unwrap();
        Ok(entries.remove(key).is_some())
    }

    async fn list_keys(&self) -> Result<Vec<String>> {
        let entries = self.entries.read().unwrap();
        Ok(entries.keys().cloned().collect())
    }

    async fn clear(&self) -> Result<()> {
        let mut entries = self.entries.write().unwrap();
        entries.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_provider() {
        let provider = InMemoryProvider::new();

        // Store and retrieve
        provider.store("test-key", "test-content").await.unwrap();
        let result = provider.retrieve("test-key").await.unwrap();
        assert_eq!(result, Some("test-content".to_string()));

        // Search
        let results = provider.search("test", 10).await.unwrap();
        assert_eq!(results.len(), 1);

        // Delete
        let deleted = provider.delete("test-key").await.unwrap();
        assert!(deleted);

        let result = provider.retrieve("test-key").await.unwrap();
        assert!(result.is_none());
    }
}
