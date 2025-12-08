//! CTAS-7 v7.2 Enhanced Sled KVS with Trivariate Hash Integration
//! Hash-centric KVS implementation bridging Supabase (ACID) → Sled (KVS) → SurrealDB (Document+SVM)

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::hash::TrivariateHashEngine;

/// CTAS task structure from Supabase ACID layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtasTask {
    pub task_id: String,
    pub task_name: String,
    pub category: String,
    pub hd4_phase: String,
    pub predecessors: Option<String>,
    pub successors: Option<String>,
    pub p: f64,  // probability
    pub t: f64,  // time
    pub h: f64,  // harshness
}

/// CTAS hash-compressed task for KVS storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtasHashTask {
    pub trivariate_hash: String,
    pub unicode_compressed: String,
    pub task_data: CtasTask,
    pub environmental_mask: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// CTAS Assembly Language Unicode operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodeAssemblyOp {
    pub operation_type: String,
    pub unicode_range: String,  // U+E000-E9FF range
    pub compressed_data: String,
    pub hash_reference: String,
}

/// Enhanced Sled KVS for CTAS hash operations
pub struct CtasSledKvs {
    db: Arc<Db>,
    hash_engine: TrivariateHashEngine,
    task_cache: Arc<RwLock<HashMap<String, CtasHashTask>>>,
    unicode_operations: Arc<RwLock<HashMap<String, UnicodeAssemblyOp>>>,
}

impl CtasSledKvs {
    /// Initialize CTAS Sled KVS with hash integration
    pub fn new(db_path: &str) -> Result<Self> {
        let db = sled::open(db_path)?;

        Ok(Self {
            db: Arc::new(db),
            hash_engine: TrivariateHashEngine::new(),
            task_cache: Arc::new(RwLock::new(HashMap::new())),
            unicode_operations: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Store CTAS task with trivariate hash compression
    pub async fn store_ctas_task(&self, task: CtasTask) -> Result<String> {
        // Generate trivariate hash for the task
        let task_key = format!("ctas_task_{}", task.task_id);
        let task_data = serde_json::to_string(&task)?;
        let trivariate_hash = self.hash_engine.generate_hash(&task_key, &task_data)?;

        // Generate environmental mask based on HD4 phase
        let environmental_mask = self.hash_engine.generate_environmental_hash(&task.hd4_phase)?;

        // Generate Unicode compressed representation
        let unicode_compressed = self.hash_engine.generate_unicode_hash(&task.task_name)?;

        let hash_task = CtasHashTask {
            trivariate_hash: trivariate_hash.clone(),
            unicode_compressed,
            task_data: task,
            environmental_mask,
            timestamp: chrono::Utc::now(),
        };

        // Store in Sled with hash as key
        let hash_task_json = serde_json::to_string(&hash_task)?;
        self.db.insert(trivariate_hash.as_bytes(), hash_task_json.as_bytes())?;

        // Cache in memory for fast access
        let mut cache = self.task_cache.write().await;
        cache.insert(trivariate_hash.clone(), hash_task);

        Ok(trivariate_hash)
    }

    /// Retrieve CTAS task by trivariate hash
    pub async fn get_ctas_task(&self, trivariate_hash: &str) -> Result<Option<CtasHashTask>> {
        // Check cache first
        {
            let cache = self.task_cache.read().await;
            if let Some(task) = cache.get(trivariate_hash) {
                return Ok(Some(task.clone()));
            }
        }

        // Fallback to Sled storage
        if let Some(data) = self.db.get(trivariate_hash.as_bytes())? {
            let hash_task: CtasHashTask = serde_json::from_slice(&data)?;

            // Update cache
            let mut cache = self.task_cache.write().await;
            cache.insert(trivariate_hash.to_string(), hash_task.clone());

            Ok(Some(hash_task))
        } else {
            Ok(None)
        }
    }

    /// Store Unicode Assembly Language operation
    pub async fn store_unicode_operation(&self, op: UnicodeAssemblyOp) -> Result<String> {
        let op_hash = self.hash_engine.generate_hash(&op.operation_type, &op.compressed_data)?;
        let op_json = serde_json::to_string(&op)?;

        // Store in Sled
        let unicode_key = format!("unicode_op_{}", op_hash);
        self.db.insert(unicode_key.as_bytes(), op_json.as_bytes())?;

        // Cache operation
        let mut ops_cache = self.unicode_operations.write().await;
        ops_cache.insert(op_hash.clone(), op);

        Ok(op_hash)
    }

    /// Get all tasks for a specific HD4 phase
    pub async fn get_tasks_by_hd4_phase(&self, phase: &str) -> Result<Vec<CtasHashTask>> {
        let mut matching_tasks = Vec::new();

        // Scan through all stored tasks
        for result in self.db.iter() {
            let (key, value) = result?;
            let key_str = String::from_utf8(key.to_vec())?;

            // Skip non-task entries
            if !key_str.starts_with("ctas_task_") && key_str.len() == 48 {
                continue;
            }

            if let Ok(hash_task) = serde_json::from_slice::<CtasHashTask>(&value) {
                if hash_task.task_data.hd4_phase == phase {
                    matching_tasks.push(hash_task);
                }
            }
        }

        Ok(matching_tasks)
    }

    /// Export tasks for SurrealDB Document+SVM integration
    pub async fn export_for_surrealdb(&self) -> Result<Vec<serde_json::Value>> {
        let mut export_data = Vec::new();

        for result in self.db.iter() {
            let (key, value) = result?;
            let key_str = String::from_utf8(key.to_vec())?;

            // Only export CTAS task data
            if key_str.len() == 48 {  // Trivariate hash length
                if let Ok(hash_task) = serde_json::from_slice::<CtasHashTask>(&value) {
                    let export_entry = serde_json::json!({
                        "trivariate_hash": hash_task.trivariate_hash,
                        "unicode_compressed": hash_task.unicode_compressed,
                        "task_id": hash_task.task_data.task_id,
                        "task_name": hash_task.task_data.task_name,
                        "category": hash_task.task_data.category,
                        "hd4_phase": hash_task.task_data.hd4_phase,
                        "p": hash_task.task_data.p,
                        "t": hash_task.task_data.t,
                        "h": hash_task.task_data.h,
                        "environmental_mask": hash_task.environmental_mask,
                        "timestamp": hash_task.timestamp
                    });
                    export_data.push(export_entry);
                }
            }
        }

        Ok(export_data)
    }

    /// Sync from Supabase ACID layer (import tasks)
    pub async fn sync_from_supabase(&self, supabase_tasks: Vec<CtasTask>) -> Result<usize> {
        let mut imported_count = 0;

        for task in supabase_tasks {
            self.store_ctas_task(task).await?;
            imported_count += 1;
        }

        Ok(imported_count)
    }

    /// Generate comprehensive hash analytics
    pub async fn get_hash_analytics(&self) -> Result<serde_json::Value> {
        let mut task_count_by_phase = HashMap::new();
        let mut total_tasks = 0;
        let mut unique_hashes = 0;

        for result in self.db.iter() {
            let (key, value) = result?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.len() == 48 {  // Trivariate hash
                unique_hashes += 1;

                if let Ok(hash_task) = serde_json::from_slice::<CtasHashTask>(&value) {
                    total_tasks += 1;
                    *task_count_by_phase.entry(hash_task.task_data.hd4_phase.clone()).or_insert(0) += 1;
                }
            }
        }

        Ok(serde_json::json!({
            "total_tasks": total_tasks,
            "unique_hashes": unique_hashes,
            "tasks_by_hd4_phase": task_count_by_phase,
            "cache_size": self.task_cache.read().await.len(),
            "unicode_operations": self.unicode_operations.read().await.len()
        }))
    }

    /// Flush cache to disk and sync
    pub async fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

/// CTAS Hash Bridge for multi-database synchronization
pub struct CtasHashBridge {
    sled_kvs: CtasSledKvs,
}

impl CtasHashBridge {
    pub fn new(sled_kvs: CtasSledKvs) -> Self {
        Self { sled_kvs }
    }

    /// Bridge ACID (Supabase) → KVS (Sled) → Document+SVM (SurrealDB)
    pub async fn bridge_databases(&self, supabase_tasks: Vec<CtasTask>) -> Result<Vec<serde_json::Value>> {
        // 1. Import from Supabase (ACID layer)
        let imported_count = self.sled_kvs.sync_from_supabase(supabase_tasks).await?;
        tracing::info!("Imported {} tasks from Supabase ACID layer", imported_count);

        // 2. Process through Sled KVS with hash compression
        self.sled_kvs.flush().await?;

        // 3. Export for SurrealDB Document+SVM layer
        let export_data = self.sled_kvs.export_for_surrealdb().await?;
        tracing::info!("Exported {} records for SurrealDB Document+SVM layer", export_data.len());

        Ok(export_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_ctas_sled_kvs_operations() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_ctas.db");
        let kvs = CtasSledKvs::new(db_path.to_str().unwrap()).unwrap();

        let test_task = CtasTask {
            task_id: "uuid-000-000-001".to_string(),
            task_name: "Ideological Formation".to_string(),
            category: "Ideation".to_string(),
            hd4_phase: "Hunt".to_string(),
            predecessors: None,
            successors: Some("uuid-000-000-002".to_string()),
            p: 0.95,
            t: 0.97,
            h: 0.15,
        };

        // Store task
        let hash = kvs.store_ctas_task(test_task.clone()).await.unwrap();
        assert_eq!(hash.len(), 48);  // Trivariate hash length

        // Retrieve task
        let retrieved = kvs.get_ctas_task(&hash).await.unwrap();
        assert!(retrieved.is_some());

        let retrieved_task = retrieved.unwrap();
        assert_eq!(retrieved_task.task_data.task_id, test_task.task_id);
        assert_eq!(retrieved_task.task_data.hd4_phase, "Hunt");
    }

    #[tokio::test]
    async fn test_hash_bridge_flow() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_bridge.db");
        let kvs = CtasSledKvs::new(db_path.to_str().unwrap()).unwrap();
        let bridge = CtasHashBridge::new(kvs);

        let test_tasks = vec![
            CtasTask {
                task_id: "uuid-000-000-001".to_string(),
                task_name: "Ideological Formation".to_string(),
                category: "Ideation".to_string(),
                hd4_phase: "Hunt".to_string(),
                predecessors: None,
                successors: Some("uuid-000-000-002".to_string()),
                p: 0.95,
                t: 0.97,
                h: 0.15,
            }
        ];

        let export_data = bridge.bridge_databases(test_tasks).await.unwrap();
        assert_eq!(export_data.len(), 1);

        let exported_task = &export_data[0];
        assert!(exported_task["trivariate_hash"].as_str().unwrap().len() == 48);
        assert!(exported_task["unicode_compressed"].as_str().is_some());
    }
}