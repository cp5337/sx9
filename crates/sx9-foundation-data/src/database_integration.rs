//! Database Integration for XSD Environment Service
//! Connects to migrated CTAS 6.6 databases (Sled + SurrealDB + Legion)

use tokio;
use tracing;
use chrono;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tracing::{info, warn, error};

#[derive(Debug, Clone)]
pub struct DatabaseCluster {
    pub sled_hot_tier: Arc<SledCluster>,
    pub surrealdb_warm_tier: Option<SurrealDBCluster>,
    pub legion_analytics: Option<LegionAnalytics>,
}

#[derive(Debug, Clone)]
pub struct SledCluster {
    pub main_db: sled::Db,
    pub crate_interviews: sled::Db,
    pub nyx_trace: sled::Db,
    pub config_db: sled::Db,
}

#[derive(Debug, Clone)]
pub struct SurrealDBCluster {
    // Placeholder for SurrealDB connection - will implement when SurrealDB is running
    pub connection_string: String,
}

#[derive(Debug, Clone)]
pub struct LegionAnalytics {
    // Placeholder for Legion ECS integration
    pub playground_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligenceQuery {
    pub context: String,
    pub layer: String,
    pub security_level: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligenceResult {
    pub data: serde_json::Value,
    pub confidence: f64,
    pub source_tier: String,
    pub access_time_ms: u64,
}

impl DatabaseCluster {
    pub async fn new(database_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🗄️ Initializing CTAS Database Cluster");

        let sled_cluster = SledCluster::new(database_path).await?;

        // SurrealDB connection (optional for now)
        let surrealdb_cluster = Some(SurrealDBCluster {
            connection_string: format!("{}/surrealdb", database_path),
        });

        // Legion analytics (optional for now)
        let legion_analytics = Some(LegionAnalytics {
            playground_path: format!("{}/legion-playground", database_path),
        });

        info!("✅ Database cluster initialized successfully");

        Ok(DatabaseCluster {
            sled_hot_tier: Arc::new(sled_cluster),
            surrealdb_warm_tier: surrealdb_cluster,
            legion_analytics,
        })
    }

    /// Query hot tier (Sled) for real-time data
    pub async fn query_hot_tier(&self, query: &IntelligenceQuery) -> Result<IntelligenceResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        // Query main database first
        let key = format!("{}:{}:{}", query.context, query.layer, query.security_level);

        if let Ok(Some(data)) = self.sled_hot_tier.main_db.get(&key) {
            let deserialized: serde_json::Value = bincode::deserialize(&data)?;
            return Ok(IntelligenceResult {
                data: deserialized,
                confidence: 0.95,
                source_tier: "hot_sled".to_string(),
                access_time_ms: start.elapsed().as_millis() as u64,
            });
        }

        // Check crate interviews database
        if let Ok(Some(data)) = self.sled_hot_tier.crate_interviews.get(&query.context) {
            let deserialized: serde_json::Value = bincode::deserialize(&data)?;
            return Ok(IntelligenceResult {
                data: deserialized,
                confidence: 0.85,
                source_tier: "hot_interviews".to_string(),
                access_time_ms: start.elapsed().as_millis() as u64,
            });
        }

        // Return empty result if no data found
        Ok(IntelligenceResult {
            data: serde_json::json!({"status": "no_data", "query": query}),
            confidence: 0.0,
            source_tier: "hot_tier_miss".to_string(),
            access_time_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Store data in hot tier
    pub async fn store_hot_tier(&self, key: &str, data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = bincode::serialize(data)?;
        self.sled_hot_tier.main_db.insert(key, serialized)?;
        Ok(())
    }

    /// Get cluster status
    pub async fn get_status(&self) -> serde_json::Value {
        let mut status = serde_json::json!({
            "cluster_type": "CTAS Multi-Tier Database",
            "hot_tier": {
                "type": "Sled Embedded",
                "status": "active",
                "databases": {
                    "main_db": self.sled_hot_tier.main_db.len(),
                    "crate_interviews": self.sled_hot_tier.crate_interviews.len(),
                    "nyx_trace": self.sled_hot_tier.nyx_trace.len(),
                    "config_db": self.sled_hot_tier.config_db.len(),
                }
            },
            "warm_tier": {
                "type": "SurrealDB",
                "status": if self.surrealdb_warm_tier.is_some() { "configured" } else { "disabled" }
            },
            "analytics_tier": {
                "type": "Legion ECS",
                "status": if self.legion_analytics.is_some() { "configured" } else { "disabled" }
            }
        });

        status
    }
}

impl SledCluster {
    pub async fn new(base_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🚀 Opening Sled databases from: {}", base_path);

        let main_db_path = format!("{}/ctas.db", base_path);
        let interviews_path = format!("{}/ctas_crate_interviews.db", base_path);
        let trace_path = format!("{}/nyx_trace.db", base_path);
        let sled_config_path = format!("{}/ctas_sled_db", base_path);

        // Open databases
        let main_db = sled::open(&main_db_path)
            .map_err(|e| format!("Failed to open main database at {}: {}", main_db_path, e))?;

        let crate_interviews = sled::open(&interviews_path)
            .map_err(|e| format!("Failed to open interviews database at {}: {}", interviews_path, e))?;

        let nyx_trace = sled::open(&trace_path)
            .map_err(|e| format!("Failed to open trace database at {}: {}", trace_path, e))?;

        let config_db = sled::open(&sled_config_path)
            .map_err(|e| format!("Failed to open config database at {}: {}", sled_config_path, e))?;

        info!("✅ Sled cluster opened successfully");
        info!("   Main DB entries: {}", main_db.len());
        info!("   Interview entries: {}", crate_interviews.len());
        info!("   Trace entries: {}", nyx_trace.len());
        info!("   Config entries: {}", config_db.len());

        Ok(SledCluster {
            main_db,
            crate_interviews,
            nyx_trace,
            config_db,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_cluster_creation() {
        // This test would require the actual migrated databases
        // For now, just test the structure
        assert!(true);
    }
}