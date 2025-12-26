//! LanceDB Vector Store Implementation
//!
//! Provides vector storage and semantic search capabilities.

use super::{VectorDocument, VectorError, VectorQueryResult, VectorResult};
use arrow_array::{Float32Array, RecordBatch, RecordBatchIterator, StringArray};
use arrow_schema::{DataType, Field, Schema};
use lancedb::connect;
use lancedb::query::QueryBase;
use std::path::Path;
use std::sync::Arc;

/// LanceDB-backed vector store
pub struct VectorStore {
    db: lancedb::Connection,
    dimension: usize,
}

impl VectorStore {
    /// Open or create a vector store at the given path
    pub async fn open(path: impl AsRef<Path>, dimension: usize) -> VectorResult<Self> {
        let db = connect(path.as_ref().to_str().unwrap())
            .execute()
            .await
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        Ok(Self { db, dimension })
    }

    /// Create schema for vector documents
    fn schema(&self) -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("content", DataType::Utf8, false),
            Field::new(
                "embedding",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    self.dimension as i32,
                ),
                false,
            ),
            Field::new("metadata", DataType::Utf8, true),
            Field::new("collection", DataType::Utf8, false),
            Field::new("created_at", DataType::Int64, false),
        ]))
    }

    /// Ensure a collection exists
    pub async fn ensure_collection(&self, name: &str) -> VectorResult<()> {
        let tables = self
            .db
            .table_names()
            .execute()
            .await
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        if !tables.contains(&name.to_string()) {
            // Create empty table with schema
            let schema = self.schema();
            let empty_batch = RecordBatch::new_empty(schema.clone());
            let batches = vec![empty_batch];
            let reader = RecordBatchIterator::new(batches.into_iter().map(Ok), schema);

            self.db
                .create_table(name, Box::new(reader))
                .execute()
                .await
                .map_err(|e| VectorError::Connection(e.to_string()))?;
        }

        Ok(())
    }

    /// Insert a document into a collection
    pub async fn insert(&self, doc: VectorDocument) -> VectorResult<()> {
        self.ensure_collection(&doc.collection).await?;

        let table = self
            .db
            .open_table(&doc.collection)
            .execute()
            .await
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let schema = self.schema();

        // Build arrays
        let id_array = StringArray::from(vec![doc.id.as_str()]);
        let content_array = StringArray::from(vec![doc.content.as_str()]);
        let metadata_array = StringArray::from(vec![doc.metadata.to_string().as_str()]);
        let collection_array = StringArray::from(vec![doc.collection.as_str()]);
        let created_at_array = arrow_array::Int64Array::from(vec![doc.created_at]);

        // Build embedding array
        let embedding_values = Float32Array::from(doc.embedding);
        let embedding_array = arrow_array::FixedSizeListArray::try_new_from_values(
            embedding_values,
            self.dimension as i32,
        )
        .map_err(|e| VectorError::Insert(e.to_string()))?;

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(id_array),
                Arc::new(content_array),
                Arc::new(embedding_array),
                Arc::new(metadata_array),
                Arc::new(collection_array),
                Arc::new(created_at_array),
            ],
        )
        .map_err(|e| VectorError::Insert(e.to_string()))?;

        let batches = vec![batch];
        let reader = RecordBatchIterator::new(batches.into_iter().map(Ok), schema);

        table
            .add(Box::new(reader))
            .execute()
            .await
            .map_err(|e| VectorError::Insert(e.to_string()))?;

        Ok(())
    }

    /// Search for similar documents
    pub async fn search(
        &self,
        collection: &str,
        query_embedding: Vec<f32>,
        limit: usize,
    ) -> VectorResult<Vec<VectorQueryResult>> {
        let table = self
            .db
            .open_table(collection)
            .execute()
            .await
            .map_err(|e| VectorError::Query(e.to_string()))?;

        let results = table
            .vector_search(query_embedding)
            .map_err(|e| VectorError::Query(e.to_string()))?
            .limit(limit)
            .execute()
            .await
            .map_err(|e| VectorError::Query(e.to_string()))?;

        let mut query_results = Vec::new();

        for batch in results {
            let batch = batch.map_err(|e| VectorError::Query(e.to_string()))?;

            // Extract data from batch
            let id_col = batch
                .column_by_name("id")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let content_col = batch
                .column_by_name("content")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let score_col = batch
                .column_by_name("_distance")
                .and_then(|c| c.as_any().downcast_ref::<Float32Array>());

            if let (Some(ids), Some(contents), Some(scores)) = (id_col, content_col, score_col) {
                for i in 0..batch.num_rows() {
                    let doc = VectorDocument::new(
                        ids.value(i),
                        contents.value(i),
                        vec![], // Don't return embedding in results
                        collection,
                    );

                    // Convert distance to similarity (1 - normalized_distance)
                    let distance = scores.value(i);
                    let score = 1.0 / (1.0 + distance);

                    query_results.push(VectorQueryResult { document: doc, score });
                }
            }
        }

        Ok(query_results)
    }

    /// Delete a document by ID
    pub async fn delete(&self, collection: &str, id: &str) -> VectorResult<bool> {
        let table = self
            .db
            .open_table(collection)
            .execute()
            .await
            .map_err(|e| VectorError::Query(e.to_string()))?;

        table
            .delete(&format!("id = '{}'", id))
            .await
            .map_err(|e| VectorError::Query(e.to_string()))?;

        Ok(true)
    }
}

impl std::fmt::Debug for VectorStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VectorStore")
            .field("dimension", &self.dimension)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_vector_store_basic() {
        let dir = tempdir().unwrap();
        let store = VectorStore::open(dir.path(), 384).await.unwrap();

        // Create test document
        let doc = VectorDocument::new(
            "test-id",
            "test content",
            vec![0.1; 384],
            super::super::collections::MEMORY,
        );

        store.insert(doc).await.unwrap();

        // Search
        let results = store
            .search(super::super::collections::MEMORY, vec![0.1; 384], 10)
            .await
            .unwrap();

        assert!(!results.is_empty());
    }
}
