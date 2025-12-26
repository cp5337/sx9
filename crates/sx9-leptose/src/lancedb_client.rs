//! LanceDB Client - Native Rust vector database
//!
//! Replaces ChromaDB with a Rust-native solution for:
//! - Embedded mode (iPad PWA, edge devices)
//! - Server mode (production backend)
//! - Billion-scale vector search
//!
//! Collections:
//! - `tools` - Threat intel tools with Unicode ops (U+E800-E8FF)
//! - `ctas_tasks` - CTAS tasks with Unicode ops (U+E000-E0FF)
//! - `ptcc_configs` - PTCC configurations (U+E300-E3FF)
//! - `tool_chains` - Tool chains (U+E400-E6FF)
//! - `threat_content` - Vectorized threat intel (MITRE, Atomic, Sigma, etc.)
//!
//! RFC References:
//! - RFC-9001: Dual-trivariate hashing
//! - RFC-9002: Unicode routing (U+E000-E9FF)
//! - RFC-9021: Vector search service (384-dim embeddings)

use arrow_array::{Float32Array, RecordBatch, RecordBatchIterator, StringArray};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::connect;
use lancedb::query::{ExecutableQuery, QueryBase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::{LeptoseError, Result};

/// LanceDB collection names (matches ChromaDB for migration)
pub mod collections {
    pub const TOOLS: &str = "tools";
    pub const CTAS_TASKS: &str = "ctas_tasks";
    pub const PTCC_CONFIGS: &str = "ptcc_configs";
    pub const TOOL_CHAINS: &str = "tool_chains";
    pub const THREAT_CONTENT: &str = "threat_content";
}

/// Embedding dimensions (all-MiniLM-L6-v2)
pub const EMBEDDING_DIM: usize = 384;

/// Query result from LanceDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorQueryResult {
    pub id: String,
    pub document: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub distance: f32,
    pub score: f32, // 1.0 - distance for similarity
}

/// Document to be indexed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub text: String,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl VectorDocument {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            embedding: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = embedding;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Add Unicode operations from trivariate hash
    pub fn with_unicode_ops(mut self, unicode_ops: &[u32]) -> Self {
        let ops_string: String = unicode_ops
            .iter()
            .filter_map(|&cp| char::from_u32(cp))
            .collect();
        let ops_csv = unicode_ops
            .iter()
            .map(|cp| cp.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.metadata
            .insert("unicode_ops".to_string(), serde_json::json!(ops_csv));
        self.metadata
            .insert("unicode_string".to_string(), serde_json::json!(ops_string));
        self
    }
}

/// LanceDB client configuration
#[derive(Debug, Clone)]
pub struct LanceDbConfig {
    /// Path to database (local file or S3/GCS URI)
    pub uri: String,
    /// Default number of results
    pub default_n_results: usize,
    /// Use embedded mode (in-process)
    pub embedded: bool,
}

impl Default for LanceDbConfig {
    fn default() -> Self {
        Self {
            uri: "~/.sx9/lancedb".to_string(),
            default_n_results: 10,
            embedded: true,
        }
    }
}

impl LanceDbConfig {
    /// Create config for local embedded database
    pub fn local(path: impl AsRef<Path>) -> Self {
        Self {
            uri: path.as_ref().to_string_lossy().to_string(),
            default_n_results: 10,
            embedded: true,
        }
    }

    /// Create config for S3 storage
    pub fn s3(bucket: &str, prefix: &str) -> Self {
        Self {
            uri: format!("s3://{}/{}", bucket, prefix),
            default_n_results: 10,
            embedded: false,
        }
    }
}

/// LanceDB client for vector operations
pub struct LanceDbClient {
    db: lancedb::Connection,
    config: LanceDbConfig,
}

impl LanceDbClient {
    /// Connect to LanceDB
    pub async fn connect(config: LanceDbConfig) -> Result<Self> {
        let uri = shellexpand::tilde(&config.uri).to_string();

        // Ensure directory exists for local databases
        if config.embedded && !uri.starts_with("s3://") && !uri.starts_with("gs://") {
            if let Some(parent) = Path::new(&uri).parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    LeptoseError::VectorError(format!("Failed to create database directory: {}", e))
                })?;
            }
        }

        let db = connect(&uri).execute().await.map_err(|e| {
            LeptoseError::VectorError(format!("Failed to connect to LanceDB: {}", e))
        })?;

        Ok(Self { db, config })
    }

    /// Connect with default local config
    pub async fn connect_local() -> Result<Self> {
        Self::connect(LanceDbConfig::default()).await
    }

    /// Create the standard schema for vector collections
    fn create_schema() -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    EMBEDDING_DIM as i32,
                ),
                false,
            ),
            Field::new("metadata_json", DataType::Utf8, true),
        ]))
    }

    /// Create or open a collection (table)
    pub async fn get_or_create_collection(&self, name: &str) -> Result<lancedb::Table> {
        let tables = self.db.table_names().execute().await.map_err(|e| {
            LeptoseError::VectorError(format!("Failed to list tables: {}", e))
        })?;

        if tables.contains(&name.to_string()) {
            self.db.open_table(name).execute().await.map_err(|e| {
                LeptoseError::VectorError(format!("Failed to open table {}: {}", name, e))
            })
        } else {
            // Create empty table with schema
            let schema = Self::create_schema();

            // Create empty arrays
            let ids: Vec<String> = vec![];
            let texts: Vec<String> = vec![];
            let vectors: Vec<Vec<f32>> = vec![];
            let metadata: Vec<String> = vec![];

            let batch = self.create_record_batch(&ids, &texts, &vectors, &metadata)?;
            let batches = RecordBatchIterator::new(vec![Ok(batch)], schema);

            self.db
                .create_table(name, Box::new(batches))
                .execute()
                .await
                .map_err(|e| {
                    LeptoseError::VectorError(format!("Failed to create table {}: {}", name, e))
                })
        }
    }

    /// Create a record batch from document data
    fn create_record_batch(
        &self,
        ids: &[String],
        texts: &[String],
        vectors: &[Vec<f32>],
        metadata: &[String],
    ) -> Result<RecordBatch> {
        let schema = Self::create_schema();

        let id_array = StringArray::from(ids.to_vec());
        let text_array = StringArray::from(texts.to_vec());
        let metadata_array = StringArray::from(metadata.to_vec());

        // Flatten vectors for FixedSizeList
        let flat_vectors: Vec<f32> = vectors.iter().flatten().cloned().collect();
        let vector_array = Float32Array::from(flat_vectors);

        // Create FixedSizeList from flat array
        let vector_list = arrow_array::FixedSizeListArray::try_new_from_values(
            vector_array,
            EMBEDDING_DIM as i32,
        )
        .map_err(|e| LeptoseError::VectorError(format!("Failed to create vector array: {}", e)))?;

        RecordBatch::try_new(
            schema,
            vec![
                Arc::new(id_array),
                Arc::new(text_array),
                Arc::new(vector_list),
                Arc::new(metadata_array),
            ],
        )
        .map_err(|e| LeptoseError::VectorError(format!("Failed to create record batch: {}", e)))
    }

    /// Add documents to a collection
    pub async fn add_documents(
        &self,
        collection: &str,
        documents: Vec<VectorDocument>,
    ) -> Result<usize> {
        if documents.is_empty() {
            return Ok(0);
        }

        let table = self.get_or_create_collection(collection).await?;

        let ids: Vec<String> = documents.iter().map(|d| d.id.clone()).collect();
        let texts: Vec<String> = documents.iter().map(|d| d.text.clone()).collect();
        let vectors: Vec<Vec<f32>> = documents.iter().map(|d| d.embedding.clone()).collect();
        let metadata: Vec<String> = documents
            .iter()
            .map(|d| serde_json::to_string(&d.metadata).unwrap_or_default())
            .collect();

        let batch = self.create_record_batch(&ids, &texts, &vectors, &metadata)?;
        let count = batch.num_rows();

        let schema = Self::create_schema();
        let batches = RecordBatchIterator::new(vec![Ok(batch)], schema);

        table.add(Box::new(batches)).execute().await.map_err(|e| {
            LeptoseError::VectorError(format!("Failed to add documents: {}", e))
        })?;

        Ok(count)
    }

    /// Query a collection by vector similarity
    pub async fn query(
        &self,
        collection: &str,
        query_vector: Vec<f32>,
        n_results: Option<usize>,
    ) -> Result<Vec<VectorQueryResult>> {
        let table = self.get_or_create_collection(collection).await?;
        let limit = n_results.unwrap_or(self.config.default_n_results);

        let results = table
            .vector_search(query_vector)
            .map_err(|e| LeptoseError::VectorError(format!("Failed to create query: {}", e)))?
            .limit(limit)
            .execute()
            .await
            .map_err(|e| LeptoseError::VectorError(format!("Failed to execute query: {}", e)))?;

        let mut query_results = Vec::new();

        // Process results
        let batches: Vec<RecordBatch> = results
            .try_collect()
            .await
            .map_err(|e| LeptoseError::VectorError(format!("Failed to collect results: {}", e)))?;

        for batch in batches {
            let id_col = batch
                .column_by_name("id")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let text_col = batch
                .column_by_name("text")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let metadata_col = batch
                .column_by_name("metadata_json")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let distance_col = batch
                .column_by_name("_distance")
                .and_then(|c| c.as_any().downcast_ref::<Float32Array>());

            if let (Some(ids), Some(texts), Some(metadata), Some(distances)) =
                (id_col, text_col, metadata_col, distance_col)
            {
                for i in 0..batch.num_rows() {
                    let id = ids.value(i).to_string();
                    let text = texts.value(i).to_string();
                    let distance = distances.value(i);
                    let meta_str = metadata.value(i);

                    let meta: HashMap<String, serde_json::Value> =
                        serde_json::from_str(meta_str).unwrap_or_default();

                    query_results.push(VectorQueryResult {
                        id,
                        document: text,
                        metadata: meta,
                        distance,
                        score: 1.0 - distance,
                    });
                }
            }
        }

        Ok(query_results)
    }

    /// Query tools collection
    pub async fn query_tools(&self, query_vector: Vec<f32>, n: usize) -> Result<Vec<VectorQueryResult>> {
        self.query(collections::TOOLS, query_vector, Some(n)).await
    }

    /// Query CTAS tasks collection
    pub async fn query_tasks(&self, query_vector: Vec<f32>, n: usize) -> Result<Vec<VectorQueryResult>> {
        self.query(collections::CTAS_TASKS, query_vector, Some(n)).await
    }

    /// Query threat content collection
    pub async fn query_threats(&self, query_vector: Vec<f32>, n: usize) -> Result<Vec<VectorQueryResult>> {
        self.query(collections::THREAT_CONTENT, query_vector, Some(n)).await
    }

    /// Get collection statistics
    pub async fn get_stats(&self) -> Result<LanceDbStats> {
        let mut stats = LanceDbStats::default();

        for (name, count_ref) in [
            (collections::TOOLS, &mut stats.tools_count),
            (collections::CTAS_TASKS, &mut stats.tasks_count),
            (collections::PTCC_CONFIGS, &mut stats.ptcc_count),
            (collections::TOOL_CHAINS, &mut stats.chains_count),
            (collections::THREAT_CONTENT, &mut stats.threat_count),
        ] {
            if let Ok(table) = self.db.open_table(name).execute().await {
                *count_ref = table.count_rows(None).await.unwrap_or(0);
            }
        }

        Ok(stats)
    }

    /// Delete a collection
    pub async fn delete_collection(&self, name: &str) -> Result<()> {
        self.db.drop_table(name).await.map_err(|e| {
            LeptoseError::VectorError(format!("Failed to drop table {}: {}", name, e))
        })
    }

    /// List all collections
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        self.db.table_names().execute().await.map_err(|e| {
            LeptoseError::VectorError(format!("Failed to list tables: {}", e))
        })
    }
}

/// LanceDB statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanceDbStats {
    pub tools_count: usize,
    pub tasks_count: usize,
    pub ptcc_count: usize,
    pub chains_count: usize,
    pub threat_count: usize,
}

impl LanceDbStats {
    pub fn total(&self) -> usize {
        self.tools_count
            + self.tasks_count
            + self.ptcc_count
            + self.chains_count
            + self.threat_count
    }
}

/// EEI satisfiers from multiple collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiSatisfiers {
    pub tools: Vec<VectorQueryResult>,
    pub tasks: Vec<VectorQueryResult>,
    pub threat_intel: Vec<VectorQueryResult>,
}

impl LanceDbClient {
    /// Find EEI satisfiers - tools/tasks that can answer an EEI
    pub async fn find_eei_satisfiers(&self, query_vector: Vec<f32>) -> Result<EeiSatisfiers> {
        let tools = self.query_tools(query_vector.clone(), 5).await?;
        let tasks = self.query_tasks(query_vector.clone(), 5).await?;
        let threats = self.query_threats(query_vector, 5).await?;

        Ok(EeiSatisfiers {
            tools,
            tasks,
            threat_intel: threats,
        })
    }
}

impl std::fmt::Debug for LanceDbClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanceDbClient")
            .field("uri", &self.config.uri)
            .field("embedded", &self.config.embedded)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lancedb_config() {
        let config = LanceDbConfig::default();
        assert!(config.embedded);
        assert_eq!(config.default_n_results, 10);
    }

    #[tokio::test]
    async fn test_vector_document() {
        let doc = VectorDocument::new("test-id", "test content")
            .with_embedding(vec![0.1; EMBEDDING_DIM])
            .with_unicode_ops(&[0xE000, 0xE100, 0xE200])
            .with_metadata("source", serde_json::json!("test"));

        assert_eq!(doc.id, "test-id");
        assert_eq!(doc.embedding.len(), EMBEDDING_DIM);
        assert!(doc.metadata.contains_key("unicode_ops"));
        assert!(doc.metadata.contains_key("source"));
    }
}
