//! CTAS-7 GLAF Clients
//!
//! Analysis clients bridging ATLAS to external data stores (RFC-9021):
//! - TETH (SurrealDB): H2 Score persistence (Zone C pre-computed)
//! - ChromaDB: 384-dim vector search (all-MiniLM-L6-v2, Layer 3A)
//! - Neo4j: Graph queries (temporary for data modeling)

pub mod teth_client;
pub mod chroma_client;
pub mod neo4j_client;

pub use teth_client::{TethClient, H2Score};
pub use chroma_client::{ChromaClient, VectorResult, EMBEDDING_DIM, collections};
pub use neo4j_client::{Neo4jClient, GraphNode};
