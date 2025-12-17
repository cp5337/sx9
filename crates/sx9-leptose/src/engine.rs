//! Leptose Engine - Main orchestrator
//!
//! Coordinates:
//! - NATS messaging (OSINT ingest, EEI answers)
//! - Knowledge graph (petgraph + GLAF)
//! - ChromaDB queries (existing vectors)
//! - EEI satisfaction routing

use crate::{
    chromadb_client::{ChromaDbClient, ChromaDbConfig, EeiSatisfiers},
    config::LeptoseConfig,
    graph::{KnowledgeGraph, KnowledgeNode, NodeType, RelationType},
    nats_bridge::NatsBridge,
    LeptoseError, Result,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// OSINT intelligence message from Python pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsintIntel {
    pub id: String,
    pub source: String,
    pub intel_type: String,
    pub title: String,
    pub content: String,
    pub confidence: f64,
    pub timestamp: String,
    pub metadata: HashMap<String, serde_json::Value>,
    /// Trivariate hash (if computed)
    pub trivariate: Option<String>,
    /// Unicode operations
    pub unicode_ops: Option<Vec<u32>>,
}

/// EEI query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiQuery {
    pub eei_id: String,
    pub question: String,
    pub category: String,
    pub priority: String,
    pub requester: Option<String>,
}

/// EEI answer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiAnswer {
    pub eei_id: String,
    pub question: String,
    pub satisfiers: EeiSatisfiers,
    pub confidence: f64,
    pub sources: Vec<String>,
    pub timestamp: String,
}

/// Leptose Engine state
#[derive(Debug, Default)]
pub struct EngineState {
    pub intel_processed: usize,
    pub eei_answered: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub last_activity: Option<chrono::DateTime<Utc>>,
}

/// Main Leptose Engine
pub struct LeptoseEngine {
    config: LeptoseConfig,
    graph: Arc<KnowledgeGraph>,
    chromadb: Arc<ChromaDbClient>,
    nats: Option<Arc<NatsBridge>>,
    state: Arc<RwLock<EngineState>>,
}

impl LeptoseEngine {
    /// Create new engine with configuration
    pub fn new(config: LeptoseConfig) -> Result<Self> {
        let chromadb_config = ChromaDbConfig {
            db_path: config.storage.base_path.join("chromadb"),
            http_endpoint: None,
            default_n_results: config.vector.max_results,
        };

        Ok(Self {
            config: config.clone(),
            graph: Arc::new(KnowledgeGraph::new(config.graph)),
            chromadb: Arc::new(ChromaDbClient::new(chromadb_config)),
            nats: None,
            state: Arc::new(RwLock::new(EngineState::default())),
        })
    }

    /// Start the engine with NATS connection
    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Leptose Knowledge Engine");

        // Connect to NATS
        let nats = NatsBridge::connect(&self.config.nats.url).await?;
        self.nats = Some(Arc::new(nats));

        // Start NATS listeners
        self.start_nats_listeners().await?;

        // Log ChromaDB stats
        match self.chromadb.get_stats().await {
            Ok(stats) => {
                tracing::info!(
                    "ChromaDB connected: {} total vectors (tools={}, tasks={}, ptcc={}, threats={})",
                    stats.total(),
                    stats.tools_count,
                    stats.tasks_count,
                    stats.ptcc_count,
                    stats.threat_count
                );
            }
            Err(e) => {
                tracing::warn!("ChromaDB not available: {}", e);
            }
        }

        Ok(())
    }

    /// Start NATS message listeners
    async fn start_nats_listeners(&self) -> Result<()> {
        let nats = self
            .nats
            .as_ref()
            .ok_or_else(|| LeptoseError::NatsError("NATS not connected".to_string()))?;

        // Clone Arcs for async tasks
        let graph = Arc::clone(&self.graph);
        let chromadb = Arc::clone(&self.chromadb);
        let state = Arc::clone(&self.state);
        let nats_clone = Arc::clone(nats);
        let osint_subject = self.config.nats.osint_subject.clone();
        let eei_subject = self.config.nats.eei_subject.clone();

        // OSINT ingest listener
        tokio::spawn(async move {
            if let Err(e) = Self::osint_listener(nats_clone, graph, state, osint_subject).await {
                tracing::error!("OSINT listener error: {}", e);
            }
        });

        // EEI query listener
        let chromadb_clone = Arc::clone(&self.chromadb);
        let nats_clone2 = Arc::clone(nats);
        tokio::spawn(async move {
            if let Err(e) = Self::eei_listener(nats_clone2, chromadb_clone, eei_subject).await {
                tracing::error!("EEI listener error: {}", e);
            }
        });

        Ok(())
    }

    /// Listen for OSINT intelligence messages
    async fn osint_listener(
        nats: Arc<NatsBridge>,
        graph: Arc<KnowledgeGraph>,
        state: Arc<RwLock<EngineState>>,
        subject: String,
    ) -> Result<()> {
        let mut subscriber = nats.subscribe(&subject).await?;

        while let Some(msg) = subscriber.next().await {
            match serde_json::from_slice::<OsintIntel>(&msg.payload) {
                Ok(intel) => {
                    tracing::debug!("Received OSINT: {} - {}", intel.source, intel.title);

                    // Add to knowledge graph
                    let node = KnowledgeNode {
                        id: Uuid::new_v4(),
                        node_type: Self::intel_type_to_node_type(&intel.intel_type),
                        name: intel.title.clone(),
                        description: intel.content.clone(),
                        properties: intel.metadata.clone(),
                        confidence: intel.confidence,
                        source: intel.source.clone(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        hash_h1: None,
                        hash_h2: None,
                        entropy: 0.0,
                        embedding: None,
                    };

                    if let Err(e) = graph.add_node(node).await {
                        tracing::warn!("Failed to add node: {}", e);
                    }

                    // Update state
                    let mut s = state.write().await;
                    s.intel_processed += 1;
                    s.last_activity = Some(Utc::now());
                }
                Err(e) => {
                    tracing::warn!("Failed to parse OSINT message: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Listen for EEI query requests
    async fn eei_listener(
        nats: Arc<NatsBridge>,
        chromadb: Arc<ChromaDbClient>,
        subject: String,
    ) -> Result<()> {
        let mut subscriber = nats.subscribe(&format!("{}.query", subject)).await?;

        while let Some(msg) = subscriber.next().await {
            match serde_json::from_slice::<EeiQuery>(&msg.payload) {
                Ok(query) => {
                    tracing::debug!("EEI query: {} - {}", query.eei_id, query.question);

                    // Find satisfiers from ChromaDB
                    let satisfiers = chromadb
                        .find_eei_satisfiers(&query.question)
                        .await
                        .unwrap_or_else(|_| EeiSatisfiers {
                            tools: vec![],
                            tasks: vec![],
                            threat_intel: vec![],
                        });

                    // Calculate confidence based on results
                    let total_results = satisfiers.tools.len()
                        + satisfiers.tasks.len()
                        + satisfiers.threat_intel.len();

                    let confidence = if total_results > 10 {
                        0.9
                    } else if total_results > 5 {
                        0.7
                    } else if total_results > 0 {
                        0.5
                    } else {
                        0.1
                    };

                    // Build sources list
                    let mut sources = vec![];
                    if !satisfiers.tools.is_empty() {
                        sources.push("ChromaDB:tools".to_string());
                    }
                    if !satisfiers.tasks.is_empty() {
                        sources.push("ChromaDB:ctas_tasks".to_string());
                    }
                    if !satisfiers.threat_intel.is_empty() {
                        sources.push("ChromaDB:threat_content".to_string());
                    }

                    let answer = EeiAnswer {
                        eei_id: query.eei_id,
                        question: query.question,
                        satisfiers,
                        confidence,
                        sources,
                        timestamp: Utc::now().to_rfc3339(),
                    };

                    // Publish answer
                    if let Some(reply) = msg.reply {
                        let payload = serde_json::to_vec(&answer).unwrap_or_default();
                        if let Err(e) = nats.publish(&reply, payload.into()).await {
                            tracing::warn!("Failed to publish EEI answer: {}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to parse EEI query: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Convert intel type string to NodeType
    fn intel_type_to_node_type(intel_type: &str) -> NodeType {
        match intel_type.to_lowercase().as_str() {
            "entity" | "person" | "organization" => NodeType::Entity,
            "document" | "report" | "article" => NodeType::Document,
            "event" | "incident" | "activity" => NodeType::Event,
            "indicator" | "ioc" | "ttp" => NodeType::Indicator,
            "infrastructure" | "domain" | "ip" => NodeType::Infrastructure,
            "campaign" | "operation" => NodeType::Campaign,
            "actor" | "threat_actor" | "group" => NodeType::Actor,
            "vulnerability" | "cve" | "exploit" => NodeType::Vulnerability,
            "artifact" | "malware" | "tool" => NodeType::Artifact,
            "eei" | "requirement" => NodeType::EEI,
            "technique" | "attack" => NodeType::AttackTechnique,
            _ => NodeType::Entity,
        }
    }

    /// Query knowledge graph
    pub async fn query_graph(&self, query: &str, n: usize) -> Result<Vec<KnowledgeNode>> {
        // First try ChromaDB for semantic search
        let results = self.chromadb.query_threats(query, n).await?;

        // Convert to KnowledgeNodes
        let nodes: Vec<KnowledgeNode> = results
            .iter()
            .map(|r| KnowledgeNode {
                id: Uuid::new_v4(),
                node_type: NodeType::Document,
                name: r
                    .metadata
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&r.id)
                    .to_string(),
                description: r.document.clone(),
                properties: r.metadata.clone(),
                confidence: 1.0 - r.distance as f64,
                source: r
                    .metadata
                    .get("source")
                    .and_then(|v| v.as_str())
                    .unwrap_or("chromadb")
                    .to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                hash_h1: None,
                hash_h2: None,
                entropy: 0.0,
                embedding: None,
            })
            .collect();

        Ok(nodes)
    }

    /// Get engine state
    pub async fn get_state(&self) -> EngineState {
        self.state.read().await.clone()
    }

    /// Get graph statistics
    pub async fn get_graph_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        // Get ChromaDB stats
        if let Ok(chromadb_stats) = self.chromadb.get_stats().await {
            stats.insert("chromadb_tools".to_string(), chromadb_stats.tools_count);
            stats.insert("chromadb_tasks".to_string(), chromadb_stats.tasks_count);
            stats.insert("chromadb_ptcc".to_string(), chromadb_stats.ptcc_count);
            stats.insert("chromadb_threats".to_string(), chromadb_stats.threat_count);
            stats.insert("chromadb_total".to_string(), chromadb_stats.total());
        }

        // Get in-memory graph stats
        let state = self.state.read().await;
        stats.insert("graph_nodes".to_string(), state.graph_nodes);
        stats.insert("graph_edges".to_string(), state.graph_edges);
        stats.insert("intel_processed".to_string(), state.intel_processed);
        stats.insert("eei_answered".to_string(), state.eei_answered);

        stats
    }
}

impl Clone for EngineState {
    fn clone(&self) -> Self {
        Self {
            intel_processed: self.intel_processed,
            eei_answered: self.eei_answered,
            graph_nodes: self.graph_nodes,
            graph_edges: self.graph_edges,
            last_activity: self.last_activity,
        }
    }
}
