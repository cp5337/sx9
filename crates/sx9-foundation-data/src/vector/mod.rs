//! Vector Store Module (RFC-9005 §3.1)
//!
//! LanceDB-based vector storage for semantic search and embeddings.
//!
//! ## Collections (RFC-9005 §3.1)
//! - tools: Tool definitions with embeddings
//! - tasks: Task descriptions for semantic matching
//! - ptcc_configs: PTCC configuration embeddings
//! - tool_chains: Multi-tool workflow embeddings

#[cfg(feature = "vector-db")]
mod store;

#[cfg(feature = "vector-db")]
pub use store::*;

use serde::{Deserialize, Serialize};

/// Vector document for embedding storage (RFC-9005 §3.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    /// Document ID (trivariate hash)
    pub id: String,
    /// Content text
    pub content: String,
    /// Embedding vector (dimension depends on model)
    pub embedding: Vec<f32>,
    /// Metadata as JSON
    pub metadata: serde_json::Value,
    /// Collection name
    pub collection: String,
    /// Creation timestamp
    pub created_at: i64,
}

impl VectorDocument {
    /// Create new vector document
    pub fn new(
        id: impl Into<String>,
        content: impl Into<String>,
        embedding: Vec<f32>,
        collection: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            embedding,
            metadata: serde_json::json!({}),
            collection: collection.into(),
            created_at: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Query result from vector search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorQueryResult {
    /// Document
    pub document: VectorDocument,
    /// Similarity score (0.0 to 1.0)
    pub score: f32,
}

/// Standard collections per RFC-9005 §3.1
pub mod collections {
    pub const TOOLS: &str = "tools";
    pub const TASKS: &str = "tasks";
    pub const PTCC_CONFIGS: &str = "ptcc_configs";
    pub const TOOL_CHAINS: &str = "tool_chains";
    pub const MEMORY: &str = "memory";
}

/// Vector store error types
#[derive(Debug, thiserror::Error)]
pub enum VectorError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Insert error: {0}")]
    Insert(String),

    #[error("Collection not found: {0}")]
    CollectionNotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type VectorResult<T> = Result<T, VectorError>;
