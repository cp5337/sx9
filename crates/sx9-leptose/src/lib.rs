//! SX9 Leptose Knowledge Engine
//!
//! Rust orchestration layer for the complete intelligence infrastructure:
//! - **ChromaDB** (Python) - Vector store with Unicode-aware embeddings
//! - **Threat Vector Pipeline** (Python) - 15+ threat intel sources
//! - **OSINT Machine** (Python) - Real-time intelligence gathering
//! - **GLAF Core** (Rust) - Graph neural operations
//! - **EEI System** (Rust) - Intelligence requirements management
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                      LEPTOSE KNOWLEDGE ENGINE                            │
//! │                    (Rust Orchestration Layer)                            │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐  │
//! │  │  OSINT Machine   │    │  Threat Vector   │    │    ChromaDB      │  │
//! │  │    (Python)      │    │    Pipeline      │    │   (4 collections)│  │
//! │  │ - 12+ sources    │    │ - 15+ sources    │    │ - tools          │  │
//! │  │ - GNN analysis   │    │ - Phi-3 training │    │ - ctas_tasks     │  │
//! │  │ - Parallel fetch │    │ - DistilBERT     │    │ - ptcc_configs   │  │
//! │  └────────┬─────────┘    └────────┬─────────┘    │ - tool_chains    │  │
//! │           │                       │              └────────┬─────────┘  │
//! │           │         NATS          │                       │            │
//! │           └───────────┬───────────┘                       │            │
//! │                       ▼                                   │            │
//! │  ┌──────────────────────────────────────────────────────────────────┐  │
//! │  │                    Leptose Orchestrator (Rust)                    │  │
//! │  │  - NATS bridge (osint.intel, eei.answer, leptose.*)              │  │
//! │  │  - Knowledge graph (petgraph + GLAF integration)                 │  │
//! │  │  - ChromaDB client (query existing vectors)                      │  │
//! │  │  - EEI satisfaction routing                                      │  │
//! │  └──────────────────────────────────────────────────────────────────┘  │
//! │                       │                                                │
//! │                       ▼                                                │
//! │  ┌──────────────────────────────────────────────────────────────────┐  │
//! │  │                    EEI Decision Engine                            │  │
//! │  │  - Time-of-Value decay                                           │  │
//! │  │  - Sliding windows                                               │  │
//! │  │  - PTCC → ATT&CK → EEI correlation                               │  │
//! │  └──────────────────────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Existing Infrastructure (DO NOT DUPLICATE)
//!
//! - `sx9/tools/abe/iac/add_to_chromadb_with_unicode.py` - ChromaDB loader
//! - `sx9/tools/abe/iac/node-interview-generator/threat_vector_pipeline.py` - Vectorization
//! - `ctas7-command-center/ctas7-intelligence-generator/` - OSINT machine
//! - `graph-db/` - TypeScript GLAF UI and threat intel

pub mod config;
pub mod engine;
pub mod graph;
pub mod lancedb_client;
pub mod nats_bridge;

// Legacy ChromaDB support (feature-gated)
#[cfg(feature = "chromadb")]
pub mod chromadb_client;

// Primary exports - LanceDB is the default vector store
pub use lancedb_client::{
    collections, EeiSatisfiers, LanceDbClient, LanceDbConfig, LanceDbStats, VectorDocument,
    VectorQueryResult, EMBEDDING_DIM,
};

pub use config::LeptoseConfig;
pub use engine::LeptoseEngine;
pub use graph::KnowledgeGraph;
pub use nats_bridge::NatsBridge;

#[cfg(feature = "chromadb")]
pub use chromadb_client::ChromaDbClient;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LeptoseError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Graph error: {0}")]
    GraphError(String),

    #[error("Vector error: {0}")]
    VectorError(String),

    #[error("Ingest error: {0}")]
    IngestError(String),

    #[error("NATS error: {0}")]
    NatsError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, LeptoseError>;
