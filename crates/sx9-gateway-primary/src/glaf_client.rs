//! GLAF Client - HTTP client to GLAF Graph Server
//! 
//! Purpose: Data analytics, graph queries, analytic workbench
//! Server: ctas7-glaf-graph-server (port 18050)

use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct GLAFClient {
    client: Client,
    base_url: String,  // http://localhost:18050
}

#[derive(Debug, Serialize)]
struct QueryRequest {
    surql: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryResult {
    nodes: Vec<GlafNode>,
    relationships: Vec<GlafRelationship>,
    stats: QueryStats,
}

#[derive(Debug, Deserialize)]
pub struct GlafNode {
    pub id: String,
    pub element_id: String,
    pub labels: Vec<String>,
    pub properties: serde_json::Value,
    #[serde(rename = "_glaf")]
    pub glaf_meta: GlafNodeMeta,
}

#[derive(Debug, Deserialize)]
pub struct GlafNodeMeta {
    pub triv_hash: Option<String>,
    pub hd4_phase: Option<String>,
    pub teth_entropy: Option<f64>,
    pub matroid_rank: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct GlafRelationship {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relationship_type: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct QueryStats {
    pub nodes_returned: usize,
    pub relationships_returned: usize,
    pub query_time_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct MatroidResult {
    pub rank: f64,
    pub fragment_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HawkesResult {
    pub intensity: f64,
    pub event_type: String,
    pub window_hours: f64,
}

#[derive(Debug, Deserialize)]
pub struct ConvergenceResult {
    pub convergence_score: f64,
    pub h1_hash: String,
    pub h2_hash: String,
}

impl GLAFClient {
    /// Create new GLAF client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }
    
    /// Health check
    pub async fn health(&self) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?;
        Ok(response.status().is_success())
    }
    
    /// Query graph with SurrealQL
    pub async fn query_graph(&self, surql: &str) -> Result<QueryResult> {
        let request = QueryRequest {
            surql: surql.to_string(),
        };
        
        let response = self.client
            .post(&format!("{}/api/query", self.base_url))
            .json(&request)
            .send()
            .await?;
        
        let result: QueryResult = response.json().await?;
        Ok(result)
    }
    
    /// Get nodes with filters
    pub async fn get_nodes(
        &self,
        limit: Option<usize>,
        hd4_phase: Option<&str>,
        label: Option<&str>,
    ) -> Result<Vec<GlafNode>> {
        let mut query = String::from("SELECT * FROM entity");
        
        if let Some(label) = label {
            query.push_str(&format!(" WHERE labels CONTAINS '{}'", label));
        }
        
        if let Some(phase) = hd4_phase {
            query.push_str(&format!(" WHERE _glaf.hd4_phase = '{}'", phase));
        }
        
        if let Some(lim) = limit {
            query.push_str(&format!(" LIMIT {}", lim));
        }
        
        let result = self.query_graph(&query).await?;
        Ok(result.nodes)
    }
    
    /// Calculate matroid rank
    pub async fn calculate_matroid_rank(&self, fragment_ids: &[String]) -> Result<MatroidResult> {
        let response = self.client
            .post(&format!("{}/api/glaf/matroid-rank", self.base_url))
            .json(&serde_json::json!({ "fragment_ids": fragment_ids }))
            .send()
            .await?;
        
        let result: MatroidResult = response.json().await?;
        Ok(result)
    }
    
    /// Calculate Hawkes intensity
    pub async fn calculate_hawkes(&self, event_type: &str, window_hours: f64) -> Result<HawkesResult> {
        let response = self.client
            .post(&format!("{}/api/glaf/hawkes-intensity", self.base_url))
            .json(&serde_json::json!({
                "event_type": event_type,
                "window_hours": window_hours,
            }))
            .send()
            .await?;
        
        let result: HawkesResult = response.json().await?;
        Ok(result)
    }
    
    /// Calculate convergence
    pub async fn calculate_convergence(&self, h1_hash: &str, h2_hash: &str) -> Result<ConvergenceResult> {
        let response = self.client
            .post(&format!("{}/api/glaf/convergence", self.base_url))
            .json(&serde_json::json!({
                "h1_hash": h1_hash,
                "h2_hash": h2_hash,
            }))
            .send()
            .await?;
        
        let result: ConvergenceResult = response.json().await?;
        Ok(result)
    }
}



