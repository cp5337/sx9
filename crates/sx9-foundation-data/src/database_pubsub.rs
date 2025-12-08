//! Database Pub-Sub Integration
//! Connects SurrealDB, Sled, Legion ECS, and Supabase with streaming pub-sub
//! Hash-based content addressable storage with real-time subscriptions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use blake3::Hasher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseEvent {
    pub id: String,
    pub event_type: DatabaseEventType,
    pub table_name: String,
    pub content_hash: String,
    pub operation: DatabaseOperation,
    pub data: serde_json::Value,
    pub hash_chain: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub source_db: DatabaseSource,
    pub linear_sync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseEventType {
    Create,
    Update,
    Delete,
    HashChange,
    LinearSync,
    PersonaAssignment,
    DomainMapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseOperation {
    Insert,
    Update,
    Delete,
    Upsert,
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseSource {
    SurrealDB,
    SledKVS,
    LegionECS,
    Supabase,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionFilter {
    pub table_names: Vec<String>,
    pub event_types: Vec<DatabaseEventType>,
    pub hash_patterns: Vec<String>,
    pub persona_ids: Vec<String>,
    pub ctas_domains: Vec<i32>,
}

pub struct DatabasePubSubEngine {
    // Pub-Sub channels
    surreal_tx: broadcast::Sender<DatabaseEvent>,
    sled_tx: broadcast::Sender<DatabaseEvent>,
    legion_tx: broadcast::Sender<DatabaseEvent>,
    supabase_tx: broadcast::Sender<DatabaseEvent>,
    unified_tx: broadcast::Sender<DatabaseEvent>,

    // Database connections
    surreal_client: Option<surrealdb::Surreal<surrealdb::engine::any::Any>>,
    sled_db: Option<sled::Db>,

    // Subscription management
    subscriptions: RwLock<HashMap<String, SubscriptionFilter>>,
    hash_tracking: RwLock<HashMap<String, Vec<String>>>, // content_hash -> subscriber_ids
}

impl DatabasePubSubEngine {
    pub async fn new() -> anyhow::Result<Self> {
        let (surreal_tx, _) = broadcast::channel(1000);
        let (sled_tx, _) = broadcast::channel(1000);
        let (legion_tx, _) = broadcast::channel(1000);
        let (supabase_tx, _) = broadcast::channel(1000);
        let (unified_tx, _) = broadcast::channel(2000);

        Ok(Self {
            surreal_tx,
            sled_tx,
            legion_tx,
            supabase_tx,
            unified_tx,
            surreal_client: None,
            sled_db: None,
            subscriptions: RwLock::new(HashMap::new()),
            hash_tracking: RwLock::new(HashMap::new()),
        })
    }

    pub async fn connect_databases(&mut self) -> anyhow::Result<()> {
        // Connect to SurrealDB
        self.surreal_client = Some(
            surrealdb::Surreal::new::<surrealdb::engine::any::Any>("http://localhost:8000")
                .await?
        );

        if let Some(db) = &self.surreal_client {
            db.use_ns("ctas7").use_db("linear").await?;
        }

        // Connect to Sled KVS
        self.sled_db = Some(sled::open("./ctas7_hash_storage")?);

        // Initialize database event listeners
        self.start_surreal_listener().await?;
        self.start_sled_listener().await?;

        Ok(())
    }

    async fn start_surreal_listener(&self) -> anyhow::Result<()> {
        if let Some(db) = &self.surreal_client {
            let tx = self.surreal_tx.clone();
            let unified_tx = self.unified_tx.clone();

            tokio::spawn(async move {
                // SurrealDB Live Query for real-time subscriptions
                let live_queries = vec![
                    "LIVE SELECT * FROM linear_issues",
                    "LIVE SELECT * FROM linear_projects",
                    "LIVE SELECT * FROM hash_change_log",
                    "LIVE SELECT * FROM persona_assignments",
                    "LIVE SELECT * FROM domain_mapping",
                ];

                for query in live_queries {
                    let mut stream = match db.query(query).await {
                        Ok(mut response) => {
                            match response.stream::<serde_json::Value>(0) {
                                Ok(stream) => stream,
                                Err(e) => {
                                    tracing::error!("Failed to create stream for {}: {}", query, e);
                                    continue;
                                }
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to execute live query {}: {}", query, e);
                            continue;
                        }
                    };

                    let tx_clone = tx.clone();
                    let unified_tx_clone = unified_tx.clone();

                    tokio::spawn(async move {
                        while let Some(result) = stream.next().await {
                            match result {
                                Ok(notification) => {
                                    let event = DatabaseEvent {
                                        id: Uuid::new_v4().to_string(),
                                        event_type: DatabaseEventType::Update,
                                        table_name: extract_table_name(query),
                                        content_hash: calculate_content_hash(&notification),
                                        operation: DatabaseOperation::Update,
                                        data: notification,
                                        hash_chain: Vec::new(),
                                        timestamp: Utc::now(),
                                        source_db: DatabaseSource::SurrealDB,
                                        linear_sync: true,
                                    };

                                    let _ = tx_clone.send(event.clone());
                                    let _ = unified_tx_clone.send(event);
                                },
                                Err(e) => {
                                    tracing::error!("SurrealDB stream error: {}", e);
                                }
                            }
                        }
                    });
                }
            });
        }

        Ok(())
    }

    async fn start_sled_listener(&self) -> anyhow::Result<()> {
        if let Some(db) = &self.sled_db {
            let tx = self.sled_tx.clone();
            let unified_tx = self.unified_tx.clone();
            let db_clone = db.clone();

            tokio::spawn(async move {
                let mut subscriber = db_clone.watch_prefix(b"");

                while let Some(event) = (&mut subscriber).await {
                    match event {
                        sled::Event::Insert { key, value } => {
                            let hash_key = String::from_utf8_lossy(&key);
                            let content = serde_json::from_slice(&value).unwrap_or_default();

                            let db_event = DatabaseEvent {
                                id: Uuid::new_v4().to_string(),
                                event_type: DatabaseEventType::Create,
                                table_name: "sled_kvs".to_string(),
                                content_hash: hash_key.to_string(),
                                operation: DatabaseOperation::Insert,
                                data: content,
                                hash_chain: vec![hash_key.to_string()],
                                timestamp: Utc::now(),
                                source_db: DatabaseSource::SledKVS,
                                linear_sync: false,
                            };

                            let _ = tx.send(db_event.clone());
                            let _ = unified_tx.send(db_event);
                        },
                        sled::Event::Remove { key } => {
                            let hash_key = String::from_utf8_lossy(&key);

                            let db_event = DatabaseEvent {
                                id: Uuid::new_v4().to_string(),
                                event_type: DatabaseEventType::Delete,
                                table_name: "sled_kvs".to_string(),
                                content_hash: hash_key.to_string(),
                                operation: DatabaseOperation::Delete,
                                data: serde_json::json!({"deleted_key": hash_key}),
                                hash_chain: vec![hash_key.to_string()],
                                timestamp: Utc::now(),
                                source_db: DatabaseSource::SledKVS,
                                linear_sync: false,
                            };

                            let _ = tx.send(db_event.clone());
                            let _ = unified_tx.send(db_event);
                        }
                    }
                }
            });
        }

        Ok(())
    }

    pub async fn subscribe_to_database(&self, subscriber_id: String, filter: SubscriptionFilter) -> broadcast::Receiver<DatabaseEvent> {
        // Store subscription
        self.subscriptions.write().await.insert(subscriber_id.clone(), filter.clone());

        // Track hash-based subscriptions
        let mut hash_tracking = self.hash_tracking.write().await;
        for hash_pattern in &filter.hash_patterns {
            hash_tracking.entry(hash_pattern.clone()).or_insert_with(Vec::new).push(subscriber_id.clone());
        }

        // Return unified receiver
        self.unified_tx.subscribe()
    }

    pub async fn publish_to_database(&self, event: DatabaseEvent) -> anyhow::Result<()> {
        match event.source_db {
            DatabaseSource::SurrealDB => {
                if let Some(db) = &self.surreal_client {
                    // Insert into appropriate SurrealDB table
                    self.insert_to_surreal(db, &event).await?;
                }
            },
            DatabaseSource::SledKVS => {
                if let Some(db) = &self.sled_db {
                    // Store in Sled with Blake3 hash key
                    let content_bytes = serde_json::to_vec(&event.data)?;
                    db.insert(&event.content_hash, content_bytes)?;
                }
            },
            DatabaseSource::LegionECS => {
                // Publish to Legion ECS event bus
                self.publish_to_legion(&event).await?;
            },
            DatabaseSource::Supabase => {
                // Publish to Supabase real-time
                self.publish_to_supabase(&event).await?;
            },
            _ => {}
        }

        // Broadcast to all subscribers
        let _ = self.unified_tx.send(event);
        Ok(())
    }

    async fn insert_to_surreal(&self, db: &surrealdb::Surreal<surrealdb::engine::any::Any>, event: &DatabaseEvent) -> anyhow::Result<()> {
        let query = match event.table_name.as_str() {
            "linear_issues" => {
                format!(
                    "INSERT INTO linear_issues CONTENT {}",
                    event.data.to_string()
                )
            },
            "hash_change_log" => {
                format!(
                    "INSERT INTO hash_change_log {{
                        table_name: '{}',
                        new_hash: '{}',
                        change_type: 'pubsub_create',
                        linear_operation: 'streaming_engine',
                        metadata: {}
                    }}",
                    event.table_name, event.content_hash, event.data
                )
            },
            _ => return Ok(())
        };

        db.query(query).await?;
        Ok(())
    }

    async fn publish_to_legion(&self, event: &DatabaseEvent) -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        // Publish to Legion ECS event bus
        let legion_payload = serde_json::json!({
            "event_type": "database_change",
            "entity_data": event.data,
            "content_hash": event.content_hash,
            "timestamp": event.timestamp,
            "source": "ctas7-streaming-engine"
        });

        let _ = client
            .post("http://localhost:18115/legion/events") // Legion ECS port
            .json(&legion_payload)
            .send()
            .await;

        Ok(())
    }

    async fn publish_to_supabase(&self, event: &DatabaseEvent) -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        // Publish to Supabase real-time channels
        let supabase_payload = serde_json::json!({
            "type": "broadcast",
            "event": "database_change",
            "payload": {
                "table": event.table_name,
                "content_hash": event.content_hash,
                "operation": event.operation,
                "data": event.data,
                "timestamp": event.timestamp
            }
        });

        let _ = client
            .post("http://localhost:54321/realtime/v1/api/broadcast") // Supabase real-time
            .header("apikey", "your-supabase-anon-key")
            .header("authorization", "Bearer your-supabase-anon-key")
            .json(&supabase_payload)
            .send()
            .await;

        Ok(())
    }

    pub async fn get_subscription_count(&self) -> usize {
        self.subscriptions.read().await.len()
    }
}

// Helper functions
fn extract_table_name(query: &str) -> String {
    if query.contains("linear_issues") {
        "linear_issues".to_string()
    } else if query.contains("linear_projects") {
        "linear_projects".to_string()
    } else if query.contains("hash_change_log") {
        "hash_change_log".to_string()
    } else {
        "unknown".to_string()
    }
}

fn calculate_content_hash(data: &serde_json::Value) -> String {
    let content = data.to_string();
    let mut hasher = Hasher::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize().as_bytes())
}

// REST API endpoints for database pub-sub
pub async fn subscribe_to_database_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let subscriber_id = request["subscriber_id"].as_str().unwrap_or_else(|| {
        &Uuid::new_v4().to_string()
    }).to_string();

    let filter = SubscriptionFilter {
        table_names: request["tables"].as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        event_types: vec![DatabaseEventType::Create, DatabaseEventType::Update],
        hash_patterns: Vec::new(),
        persona_ids: Vec::new(),
        ctas_domains: Vec::new(),
    };

    axum::Json(serde_json::json!({
        "status": "subscribed",
        "subscriber_id": subscriber_id,
        "filter": filter,
        "websocket_endpoint": format!("ws://localhost:18112/database/subscribe/{}", subscriber_id)
    }))
}

pub async fn publish_database_event(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let event = DatabaseEvent {
        id: Uuid::new_v4().to_string(),
        event_type: DatabaseEventType::Create,
        table_name: request["table"].as_str().unwrap_or("unknown").to_string(),
        content_hash: calculate_content_hash(&request["data"]),
        operation: DatabaseOperation::Insert,
        data: request["data"].clone(),
        hash_chain: Vec::new(),
        timestamp: Utc::now(),
        source_db: DatabaseSource::Internal,
        linear_sync: request["linear_sync"].as_bool().unwrap_or(false),
    };

    axum::Json(serde_json::json!({
        "status": "published",
        "event_id": event.id,
        "content_hash": event.content_hash,
        "timestamp": event.timestamp
    }))
}