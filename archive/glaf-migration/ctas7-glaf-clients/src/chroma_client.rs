//! ChromaDB Vector Client - RFC-9021 Cognitive Inference
//!
//! 384-dimensional vector search using all-MiniLM-L6-v2 embeddings.
//! Collections per RFC-9021: techniques, detection_rules, tools, interviews, threat_reports

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// ChromaDB canonical endpoint (docker-compose: vector-store-service)
const CHROMA_ENDPOINT: &str = "http://vector-store-service:8000";

/// Embedding dimension per RFC-9021 (all-MiniLM-L6-v2)
pub const EMBEDDING_DIM: usize = 384;

/// RFC-9021 defined collections
pub mod collections {
    pub const TECHNIQUES: &str = "techniques";
    pub const DETECTION_RULES: &str = "detection_rules";
    pub const TOOLS: &str = "tools";
    pub const INTERVIEWS: &str = "interviews";
    pub const THREAT_REPORTS: &str = "threat_reports";
}

/// Vector search result from ChromaDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorResult {
    pub id: String,
    pub distance: f32,
    pub document: Option<String>,
    pub metadata: serde_json::Value,
}

/// ChromaDB query request
#[derive(Debug, Serialize)]
struct QueryRequest {
    query_embeddings: Vec<Vec<f32>>,
    n_results: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#where: Option<serde_json::Value>,
    include: Vec<String>,
}

/// ChromaDB query response
#[derive(Debug, Deserialize)]
struct QueryResponse {
    ids: Vec<Vec<String>>,
    distances: Option<Vec<Vec<f32>>>,
    documents: Option<Vec<Vec<Option<String>>>>,
    metadatas: Option<Vec<Vec<serde_json::Value>>>,
}

/// ChromaDB vector search client (RFC-9021)
pub struct ChromaClient {
    client: reqwest::Client,
    endpoint: String,
}

impl ChromaClient {
    /// Connect to ChromaDB at default endpoint
    pub fn new() -> Self {
        Self::with_endpoint(CHROMA_ENDPOINT.to_string())
    }

    /// Connect to ChromaDB at specified endpoint
    pub fn with_endpoint(endpoint: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
        }
    }

    /// Query collection for similar vectors
    ///
    /// embedding: 384-dimensional vector (all-MiniLM-L6-v2)
    /// collection: One of RFC-9021 collections (techniques, detection_rules, etc.)
    /// n_results: Number of results to return
    pub async fn query(
        &self,
        collection: &str,
        embedding: Vec<f32>,
        n_results: usize,
        filter: Option<serde_json::Value>,
    ) -> Result<Vec<VectorResult>> {
        if embedding.len() != EMBEDDING_DIM {
            anyhow::bail!(
                "Embedding dimension mismatch: expected {}, got {} (RFC-9021 requires all-MiniLM-L6-v2)",
                EMBEDDING_DIM,
                embedding.len()
            );
        }

        let request = QueryRequest {
            query_embeddings: vec![embedding],
            n_results,
            r#where: filter,
            include: vec![
                "documents".to_string(),
                "metadatas".to_string(),
                "distances".to_string(),
            ],
        };

        let url = format!(
            "{}/api/v1/collections/{}/query",
            self.endpoint, collection
        );

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        let query_response: QueryResponse = response.json().await?;

        // Parse response into VectorResult list
        let mut results = Vec::new();

        if let Some(ids) = query_response.ids.first() {
            let distances = query_response.distances.as_ref().and_then(|d| d.first());
            let documents = query_response.documents.as_ref().and_then(|d| d.first());
            let metadatas = query_response.metadatas.as_ref().and_then(|m| m.first());

            for (i, id) in ids.iter().enumerate() {
                results.push(VectorResult {
                    id: id.clone(),
                    distance: distances.and_then(|d| d.get(i).copied()).unwrap_or(0.0),
                    document: documents.and_then(|d| d.get(i).cloned()).flatten(),
                    metadata: metadatas
                        .and_then(|m| m.get(i).cloned())
                        .unwrap_or(serde_json::Value::Null),
                });
            }
        }

        debug!(
            collection = collection,
            results = results.len(),
            "ChromaDB query complete"
        );

        Ok(results)
    }

    /// Query techniques collection (MITRE ATT&CK)
    pub async fn query_techniques(
        &self,
        embedding: Vec<f32>,
        n_results: usize,
        tactic_filter: Option<Vec<&str>>,
    ) -> Result<Vec<VectorResult>> {
        let filter = tactic_filter.map(|tactics| {
            serde_json::json!({
                "tactic": {"$in": tactics}
            })
        });
        self.query(collections::TECHNIQUES, embedding, n_results, filter)
            .await
    }

    /// Query detection rules collection (Sigma/YARA/Wazuh)
    pub async fn query_detection_rules(
        &self,
        embedding: Vec<f32>,
        n_results: usize,
    ) -> Result<Vec<VectorResult>> {
        self.query(collections::DETECTION_RULES, embedding, n_results, None)
            .await
    }

    /// Query tools collection (Kali/LOLTL)
    pub async fn query_tools(
        &self,
        embedding: Vec<f32>,
        n_results: usize,
    ) -> Result<Vec<VectorResult>> {
        self.query(collections::TOOLS, embedding, n_results, None)
            .await
    }

    /// Query interviews collection
    pub async fn query_interviews(
        &self,
        embedding: Vec<f32>,
        n_results: usize,
    ) -> Result<Vec<VectorResult>> {
        self.query(collections::INTERVIEWS, embedding, n_results, None)
            .await
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/v1/heartbeat", self.endpoint);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    /// Get endpoint
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get expected embedding dimension (384)
    pub fn embedding_dim(&self) -> usize {
        EMBEDDING_DIM
    }
}

impl Default for ChromaClient {
    fn default() -> Self {
        Self::new()
    }
}
