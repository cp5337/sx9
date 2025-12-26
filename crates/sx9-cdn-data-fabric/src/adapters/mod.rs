//! Database Adapters
//!
//! Each adapter handles communication with a specific database type.

pub mod geojson;
pub mod neo4j;
pub mod network_flow;
pub mod postgres;
pub mod slotgraph;
pub mod supabase;  // Replaces SurrealDB - PostgREST API, no heavy SDK
// pub mod surreal;  // DEPRECATED: ~100 crate deps, replaced by supabase

use crate::registry::DatabaseInfo;
use serde_json::Value;

/// Common trait for database adapters
#[async_trait::async_trait]
pub trait DatabaseAdapter: Send + Sync {
    /// Get database type identifier
    fn db_type(&self) -> &str;

    /// Check if database is healthy
    async fn health_check(&self, info: &DatabaseInfo) -> bool;

    /// Execute a query
    async fn execute(&self, info: &DatabaseInfo, query: &str) -> anyhow::Result<Vec<Value>>;

    /// Get database schema
    async fn get_schema(&self, info: &DatabaseInfo) -> anyhow::Result<Value>;
}

/// Adapter capabilities
#[derive(Debug, Clone, PartialEq)]
pub enum AdapterCapability {
    Graph,
    Table,
    GeoJson,
    NetworkFlow,
    Ecs,
    KeyValue,
    LiveQuery,
}
