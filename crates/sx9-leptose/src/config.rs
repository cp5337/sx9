//! Leptose configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main Leptose configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeptoseConfig {
    /// NATS configuration
    pub nats: NatsConfig,

    /// Knowledge graph configuration
    pub graph: GraphConfig,

    /// Vector engine configuration
    pub vector: VectorConfig,

    /// Ingest service configuration
    pub ingest: IngestConfig,

    /// Storage paths
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsConfig {
    /// NATS server URL
    pub url: String,

    /// Subject prefix for Leptose messages
    pub subject_prefix: String,

    /// OSINT ingest subject
    pub osint_subject: String,

    /// EEI output subject
    pub eei_subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    /// Enable GLAF integration
    pub enable_glaf: bool,

    /// TETH entropy threshold for relevance
    pub teth_threshold: f64,

    /// Maximum nodes to keep in memory
    pub max_nodes: usize,

    /// Relationship types to track
    pub relationship_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorConfig {
    /// Embedding model name (for fastembed)
    pub model_name: String,

    /// Embedding dimension
    pub embedding_dimension: usize,

    /// Chunk size for text splitting
    pub chunk_size: usize,

    /// Chunk overlap
    pub chunk_overlap: usize,

    /// Similarity threshold for retrieval
    pub similarity_threshold: f32,

    /// Max results to return
    pub max_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestConfig {
    /// Watch directories for new files
    pub watch_dirs: Vec<PathBuf>,

    /// File extensions to process
    pub extensions: Vec<String>,

    /// Enable real-time processing
    pub realtime: bool,

    /// Batch size for bulk processing
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base storage path
    pub base_path: PathBuf,

    /// Knowledge graph database path
    pub graph_db_path: PathBuf,

    /// Vector index path
    pub vector_index_path: PathBuf,

    /// Cache path
    pub cache_path: PathBuf,
}

impl Default for LeptoseConfig {
    fn default() -> Self {
        Self {
            nats: NatsConfig {
                url: "nats://localhost:4222".to_string(),
                subject_prefix: "leptose".to_string(),
                osint_subject: "osint.intel".to_string(),
                eei_subject: "eei.answer".to_string(),
            },
            graph: GraphConfig {
                enable_glaf: true,
                teth_threshold: 0.5,
                max_nodes: 100_000,
                relationship_types: vec![
                    "ASSOCIATED_WITH".to_string(),
                    "BELONGS_TO".to_string(),
                    "COMMUNICATES_WITH".to_string(),
                    "USES".to_string(),
                    "TARGETS".to_string(),
                    "ATTRIBUTED_TO".to_string(),
                    "EXPLOITS".to_string(),
                    "HOSTS".to_string(),
                    "REFERENCES".to_string(),
                ],
            },
            vector: VectorConfig {
                model_name: "BAAI/bge-small-en-v1.5".to_string(),
                embedding_dimension: 384,
                chunk_size: 512,
                chunk_overlap: 64,
                similarity_threshold: 0.7,
                max_results: 10,
            },
            ingest: IngestConfig {
                watch_dirs: vec![],
                extensions: vec![
                    "md".to_string(),
                    "txt".to_string(),
                    "json".to_string(),
                    "toml".to_string(),
                ],
                realtime: true,
                batch_size: 100,
            },
            storage: StorageConfig {
                base_path: PathBuf::from("data/leptose"),
                graph_db_path: PathBuf::from("data/leptose/graph.db"),
                vector_index_path: PathBuf::from("data/leptose/vectors"),
                cache_path: PathBuf::from("data/leptose/cache"),
            },
        }
    }
}
