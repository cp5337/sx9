//! Neo4j graph database client for ATLAS cognitive queries

use anyhow::Result;
use neo4rs::{Graph, Query};
use serde::{Deserialize, Serialize};

/// Neo4j client bridges ATLAS to graph database
#[derive(Clone)]
pub struct Neo4jClient {
    graph: Graph,
}

/// Node representation from Neo4j query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub labels: Vec<String>,
    pub properties: serde_json::Value,
}

impl Neo4jClient {
    /// Create new Neo4j client with connection URI
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self> {
        let graph = Graph::new(uri, user, password).await?;
        Ok(Self { graph })
    }

    /// Execute Cypher query and return results
    pub async fn execute_query(&self, cypher: &str) -> Result<Vec<GraphNode>> {
        let query = Query::new(cypher.to_string());
        let mut result = self.graph.execute(query).await?;

        let mut nodes = Vec::new();
        while let Some(row) = result.next().await? {
            // Client extracts node data from query result
            if let Ok(node) = row.get::<neo4rs::Node>("n") {
                nodes.push(GraphNode {
                    id: node.id().to_string(),
                    labels: node.labels().iter().map(|s| s.to_string()).collect(),
                    properties: serde_json::json!({}),
                });
            }
        }
        Ok(nodes)
    }
}
