//! Database Registry
//!
//! Tracks all registered databases and their health status.

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Information about a registered database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub db_id: String,
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub namespace: Option<String>,
    pub database: Option<String>,
    pub capabilities: Vec<String>,
    pub schema: Option<serde_json::Value>,
    pub health_endpoint: Option<String>,
    pub last_heartbeat: DateTime<Utc>,
    pub status: String,
}

/// Database registry using concurrent hashmap
pub struct DatabaseRegistry {
    databases: DashMap<String, DatabaseInfo>,
}

impl DatabaseRegistry {
    pub fn new() -> Self {
        let registry = Self {
            databases: DashMap::new(),
        };

        // Pre-register known databases
        registry.register_defaults();
        registry
    }

    /// Pre-register known CTAS databases
    fn register_defaults(&self) {
        // SurrealDB Main
        self.register(DatabaseInfo {
            db_id: "surrealdb-main".to_string(),
            db_type: "surrealdb".to_string(),
            host: "localhost".to_string(),
            port: 8000,
            namespace: Some("ctas7".to_string()),
            database: Some("main".to_string()),
            capabilities: vec!["graph".to_string(), "sql".to_string(), "live".to_string()],
            schema: None,
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // GLAF Core
        self.register(DatabaseInfo {
            db_id: "glaf-core".to_string(),
            db_type: "surrealdb".to_string(),
            host: "localhost".to_string(),
            port: 18019,
            namespace: Some("ctas7".to_string()),
            database: Some("glaf".to_string()),
            capabilities: vec!["graph".to_string(), "sql".to_string(), "live".to_string()],
            schema: None,
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // GLAF Analytics
        self.register(DatabaseInfo {
            db_id: "glaf-analytics".to_string(),
            db_type: "surrealdb".to_string(),
            host: "localhost".to_string(),
            port: 18025,
            namespace: Some("ctas7".to_string()),
            database: Some("analytics".to_string()),
            capabilities: vec!["graph".to_string(), "sql".to_string()],
            schema: None,
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // PostgreSQL (Supabase)
        self.register(DatabaseInfo {
            db_id: "postgres-supabase".to_string(),
            db_type: "postgres".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            namespace: None,
            database: Some("postgres".to_string()),
            capabilities: vec!["sql".to_string()],
            schema: None,
            health_endpoint: None,
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // SlotGraph ECS
        self.register(DatabaseInfo {
            db_id: "slotgraph-ecs".to_string(),
            db_type: "slotgraph".to_string(),
            host: "localhost".to_string(),
            port: 9001,
            namespace: None,
            database: None,
            capabilities: vec!["graph".to_string(), "ecs".to_string()],
            schema: None,
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // Sledis Cache
        self.register(DatabaseInfo {
            db_id: "sledis-cache".to_string(),
            db_type: "redis".to_string(),
            host: "localhost".to_string(),
            port: 6380,
            namespace: None,
            database: None,
            capabilities: vec!["kv".to_string(), "cache".to_string()],
            schema: None,
            health_endpoint: None,
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // GeoJSON (static files)
        self.register(DatabaseInfo {
            db_id: "geojson-cdn".to_string(),
            db_type: "geojson".to_string(),
            host: "localhost".to_string(),
            port: 0,
            namespace: None,
            database: None,
            capabilities: vec!["geojson".to_string(), "static".to_string()],
            schema: Some(serde_json::json!({
                "layers": [
                    "ground-stations",
                    "submarine-cables",
                    "cable-landings",
                    "landing-points"
                ]
            })),
            health_endpoint: None,
            last_heartbeat: Utc::now(),
            status: "online".to_string(),
        });

        // SX9 Service Orchestrator (Network Flow)
        self.register(DatabaseInfo {
            db_id: "sx9-orchestrator".to_string(),
            db_type: "network_flow".to_string(),
            host: "localhost".to_string(),
            port: 15174,
            namespace: None,
            database: None,
            capabilities: vec![
                "network_flow".to_string(),
                "workflow".to_string(),
                "taps_buffer".to_string(),
                "neural_mux".to_string(),
            ],
            schema: Some(serde_json::json!({
                "event_types": [
                    "WorkflowStarted", "WorkflowCompleted", "WorkflowFailed",
                    "TapsBufferWrite", "TapsBufferRead",
                    "NeuralMuxConnect", "NeuralMuxTranslate",
                    "ServiceStartup", "ServiceHealthCheck",
                    "DataIngestion", "DataTransformation"
                ],
                "workflow_types": ["OSINT", "THREAT", "MEDIA", "STREAM", "GEOSPATIAL", "HD4"],
                "services_managed": [
                    "surrealdb", "glaf-core", "neo4j", "supabase", "sledis", "legion-ecs"
                ]
            })),
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // Forge Workflow System
        self.register(DatabaseInfo {
            db_id: "forge-workflows".to_string(),
            db_type: "workflow".to_string(),
            host: "localhost".to_string(),
            port: 18200,
            namespace: None,
            database: None,
            capabilities: vec![
                "workflow".to_string(),
                "automation".to_string(),
                "escalation".to_string(),
            ],
            schema: Some(serde_json::json!({
                "workflow_types": [
                    "DockerContainerWorkflow",
                    "GoogleCloudWorkflow",
                    "MultiTenantWorkflow",
                    "ModelDeployment",
                    "BernoulliAdaptationWorkflow"
                ]
            })),
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });

        // SYNAPTIX9 Neural Mux
        self.register(DatabaseInfo {
            db_id: "synaptix9-neural-mux".to_string(),
            db_type: "neural_mux".to_string(),
            host: "localhost".to_string(),
            port: 18300,
            namespace: None,
            database: None,
            capabilities: vec![
                "neural_mux".to_string(),
                "cognitive_atoms".to_string(),
                "translation".to_string(),
            ],
            schema: Some(serde_json::json!({
                "node_types": [
                    "source", "sink", "transformer", "router", "buffer",
                    "gate", "monitor", "catalyst", "inhibitor", "relay"
                ],
                "cognitive_dimensions": [
                    "physical", "temporal", "energetic", "spatial", "relational", "economic"
                ]
            })),
            health_endpoint: Some("/health".to_string()),
            last_heartbeat: Utc::now(),
            status: "unknown".to_string(),
        });
    }

    pub fn register(&self, info: DatabaseInfo) {
        self.databases.insert(info.db_id.clone(), info);
    }

    pub fn unregister(&self, db_id: &str) {
        self.databases.remove(db_id);
    }

    pub fn get(&self, db_id: &str) -> Option<DatabaseInfo> {
        self.databases.get(db_id).map(|r| r.clone())
    }

    pub fn heartbeat(&self, db_id: &str) -> bool {
        if let Some(mut db) = self.databases.get_mut(db_id) {
            db.last_heartbeat = Utc::now();
            db.status = "online".to_string();
            true
        } else {
            false
        }
    }

    pub fn list_all(&self) -> Vec<DatabaseInfo> {
        self.databases.iter().map(|r| r.clone()).collect()
    }

    pub fn count(&self) -> usize {
        self.databases.len()
    }

    pub fn get_by_type(&self, db_type: &str) -> Vec<DatabaseInfo> {
        self.databases
            .iter()
            .filter(|r| r.db_type == db_type)
            .map(|r| r.clone())
            .collect()
    }

    pub fn get_by_capability(&self, capability: &str) -> Vec<DatabaseInfo> {
        self.databases
            .iter()
            .filter(|r| r.capabilities.contains(&capability.to_string()))
            .map(|r| r.clone())
            .collect()
    }
}

impl Default for DatabaseRegistry {
    fn default() -> Self {
        Self::new()
    }
}
