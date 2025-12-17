//! SlotGraph Bridge
//!
//! Connects Plasma-Defender to the SlotGraph ECS system for
//! entity relationship queries and graph-based correlation.
//!
//! SlotGraph provides:
//! - Entity storage with component data
//! - Graph topology (relationships between entities)
//! - Real-time entity queries

use crate::ecs::components::{Hd4Phase, ThreatEntityComponent};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// =============================================================================
// CONFIGURATION
// =============================================================================

/// SlotGraph connection configuration
#[derive(Debug, Clone)]
pub struct SlotBridgeConfig {
    /// SlotGraph host
    pub host: String,
    /// SlotGraph port
    pub port: u16,
    /// Connection timeout (ms)
    pub timeout_ms: u64,
    /// Enable caching
    pub cache_enabled: bool,
}

impl Default for SlotBridgeConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 18200, // Default SlotGraph port
            timeout_ms: 5000,
            cache_enabled: true,
        }
    }
}

// =============================================================================
// QUERY TYPES
// =============================================================================

/// SlotGraph query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotQuery {
    /// Query type
    pub query_type: SlotQueryType,
    /// Entity ID filter
    pub entity_id: Option<u64>,
    /// Component type filter
    pub component_type: Option<String>,
    /// Relationship type filter
    pub relationship_type: Option<String>,
    /// Max depth for traversal
    pub max_depth: Option<u32>,
    /// Custom filter expression
    pub filter: Option<String>,
}

/// Query types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlotQueryType {
    /// Get entity by ID
    GetEntity,
    /// Get all entities
    GetAllEntities,
    /// Get entity neighbors
    GetNeighbors,
    /// Get entity path
    GetPath,
    /// Get entities by component
    GetByComponent,
    /// Execute custom query
    Custom,
}

impl SlotQuery {
    /// Query for single entity
    pub fn entity(entity_id: u64) -> Self {
        Self {
            query_type: SlotQueryType::GetEntity,
            entity_id: Some(entity_id),
            component_type: None,
            relationship_type: None,
            max_depth: None,
            filter: None,
        }
    }

    /// Query for all entities
    pub fn all() -> Self {
        Self {
            query_type: SlotQueryType::GetAllEntities,
            entity_id: None,
            component_type: None,
            relationship_type: None,
            max_depth: None,
            filter: None,
        }
    }

    /// Query for neighbors of an entity
    pub fn neighbors(entity_id: u64, max_depth: u32) -> Self {
        Self {
            query_type: SlotQueryType::GetNeighbors,
            entity_id: Some(entity_id),
            component_type: None,
            relationship_type: None,
            max_depth: Some(max_depth),
            filter: None,
        }
    }

    /// Query by component type
    pub fn by_component(component_type: impl Into<String>) -> Self {
        Self {
            query_type: SlotQueryType::GetByComponent,
            entity_id: None,
            component_type: Some(component_type.into()),
            relationship_type: None,
            max_depth: None,
            filter: None,
        }
    }

    /// Custom query
    pub fn custom(query: impl Into<String>) -> Self {
        Self {
            query_type: SlotQueryType::Custom,
            entity_id: None,
            component_type: None,
            relationship_type: None,
            max_depth: None,
            filter: Some(query.into()),
        }
    }

    /// Add relationship filter
    pub fn with_relationship(mut self, rel_type: impl Into<String>) -> Self {
        self.relationship_type = Some(rel_type.into());
        self
    }
}

/// SlotGraph entity result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotEntity {
    /// Entity ID (slot ID)
    pub slot_id: u64,
    /// Entity hash
    pub hash: u64,
    /// Component data
    pub components: HashMap<String, Value>,
    /// Relationships (target slot IDs)
    pub relationships: Vec<SlotRelationship>,
}

/// Entity relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotRelationship {
    /// Target entity slot ID
    pub target_id: u64,
    /// Relationship type
    pub rel_type: String,
    /// Relationship weight/strength
    pub weight: f32,
}

impl SlotEntity {
    /// Try to extract threat component
    pub fn as_threat(&self) -> Option<ThreatEntityComponent> {
        self.components
            .get("threat")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Get component by name
    pub fn get_component<T: for<'de> Deserialize<'de>>(&self, name: &str) -> Option<T> {
        self.components
            .get(name)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Check if entity has component
    pub fn has_component(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    /// Get related entity IDs
    pub fn related_ids(&self) -> Vec<u64> {
        self.relationships.iter().map(|r| r.target_id).collect()
    }

    /// Get related entities of specific type
    pub fn related_of_type(&self, rel_type: &str) -> Vec<u64> {
        self.relationships
            .iter()
            .filter(|r| r.rel_type == rel_type)
            .map(|r| r.target_id)
            .collect()
    }
}

// =============================================================================
// SLOT BRIDGE
// =============================================================================

/// SlotGraph Bridge - connects to SlotGraph ECS
pub struct SlotBridge {
    config: SlotBridgeConfig,
    client: reqwest::Client,
    /// Entity cache
    cache: HashMap<u64, SlotEntity>,
}

impl SlotBridge {
    /// Create new SlotGraph bridge
    pub fn new(config: SlotBridgeConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            client,
            cache: HashMap::new(),
        }
    }

    /// Create with default config
    pub fn default_config() -> Self {
        Self::new(SlotBridgeConfig::default())
    }

    /// Base URL for SlotGraph API
    fn base_url(&self) -> String {
        format!("http://{}:{}/api", self.config.host, self.config.port)
    }

    /// Execute query
    pub async fn query(&mut self, query: &SlotQuery) -> Result<Vec<SlotEntity>> {
        match query.query_type {
            SlotQueryType::GetEntity => {
                if let Some(id) = query.entity_id {
                    let entity = self.get_entity(id).await?;
                    Ok(entity.map(|e| vec![e]).unwrap_or_default())
                } else {
                    Ok(Vec::new())
                }
            }
            SlotQueryType::GetAllEntities => self.get_all_entities().await,
            SlotQueryType::GetNeighbors => {
                if let Some(id) = query.entity_id {
                    self.get_neighbors(id, query.max_depth.unwrap_or(1)).await
                } else {
                    Ok(Vec::new())
                }
            }
            SlotQueryType::GetByComponent => {
                if let Some(ref comp) = query.component_type {
                    self.get_by_component(comp).await
                } else {
                    Ok(Vec::new())
                }
            }
            SlotQueryType::Custom => {
                if let Some(ref filter) = query.filter {
                    self.execute_custom(filter).await
                } else {
                    Ok(Vec::new())
                }
            }
            _ => Ok(Vec::new()),
        }
    }

    /// Get single entity by ID
    pub async fn get_entity(&mut self, slot_id: u64) -> Result<Option<SlotEntity>> {
        // Check cache first
        if self.config.cache_enabled {
            if let Some(entity) = self.cache.get(&slot_id) {
                return Ok(Some(entity.clone()));
            }
        }

        let url = format!("{}/entities/{}", self.base_url(), slot_id);

        let response = self.client.get(&url).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let entity: SlotEntity = resp.json().await?;

                // Cache result
                if self.config.cache_enabled {
                    self.cache.insert(slot_id, entity.clone());
                }

                Ok(Some(entity))
            }
            Ok(resp) if resp.status().as_u16() == 404 => Ok(None),
            Ok(resp) => Err(anyhow::anyhow!("SlotGraph error: {}", resp.status())),
            Err(e) => {
                tracing::warn!("SlotGraph connection failed: {}", e);
                Ok(None)
            }
        }
    }

    /// Get all entities
    pub async fn get_all_entities(&self) -> Result<Vec<SlotEntity>> {
        let url = format!("{}/entities", self.base_url());

        let response = self.client.get(&url).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: Value = resp.json().await?;
                let entities = result
                    .get("entities")
                    .and_then(|e| serde_json::from_value::<Vec<SlotEntity>>(e.clone()).ok())
                    .unwrap_or_default();
                Ok(entities)
            }
            Ok(resp) => Err(anyhow::anyhow!("SlotGraph error: {}", resp.status())),
            Err(e) => {
                tracing::warn!("SlotGraph connection failed: {}", e);
                Ok(Vec::new())
            }
        }
    }

    /// Get entity neighbors
    pub async fn get_neighbors(&self, slot_id: u64, max_depth: u32) -> Result<Vec<SlotEntity>> {
        let url = format!(
            "{}/entities/{}/neighbors?depth={}",
            self.base_url(),
            slot_id,
            max_depth
        );

        let response = self.client.get(&url).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: Value = resp.json().await?;
                let entities = result
                    .get("neighbors")
                    .and_then(|e| serde_json::from_value::<Vec<SlotEntity>>(e.clone()).ok())
                    .unwrap_or_default();
                Ok(entities)
            }
            Ok(_) | Err(_) => Ok(Vec::new()),
        }
    }

    /// Get entities by component type
    pub async fn get_by_component(&self, component_type: &str) -> Result<Vec<SlotEntity>> {
        let url = format!("{}/query", self.base_url());
        let body = json!({
            "component": component_type
        });

        let response = self.client.post(&url).json(&body).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: Value = resp.json().await?;
                let entities = result
                    .get("entities")
                    .and_then(|e| serde_json::from_value::<Vec<SlotEntity>>(e.clone()).ok())
                    .unwrap_or_default();
                Ok(entities)
            }
            Ok(_) | Err(_) => Ok(Vec::new()),
        }
    }

    /// Execute custom query
    pub async fn execute_custom(&self, query: &str) -> Result<Vec<SlotEntity>> {
        let url = format!("{}/query", self.base_url());
        let body = json!({
            "query": query
        });

        let response = self.client.post(&url).json(&body).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: Value = resp.json().await?;
                let entities = result
                    .get("entities")
                    .and_then(|e| serde_json::from_value::<Vec<SlotEntity>>(e.clone()).ok())
                    .unwrap_or_default();
                Ok(entities)
            }
            Ok(_) | Err(_) => Ok(Vec::new()),
        }
    }

    /// Get graph topology
    pub async fn get_graph(&self) -> Result<Value> {
        let url = format!("{}/graph", self.base_url());

        let response = self.client.get(&url).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let graph: Value = resp.json().await?;
                Ok(graph)
            }
            Ok(_) | Err(_) => Ok(json!({"nodes": [], "edges": []})),
        }
    }

    /// Get all threats from SlotGraph
    pub async fn get_threats(&self) -> Result<Vec<SlotEntity>> {
        self.get_by_component("threat").await
    }

    /// Get threats by HD4 phase
    pub async fn get_threats_by_phase(&self, phase: Hd4Phase) -> Result<Vec<SlotEntity>> {
        let url = format!("{}/query", self.base_url());
        let body = json!({
            "component": "hd4_phase",
            "filter": {
                "phase": phase as u8
            }
        });

        let response = self.client.post(&url).json(&body).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: Value = resp.json().await?;
                let entities = result
                    .get("entities")
                    .and_then(|e| serde_json::from_value::<Vec<SlotEntity>>(e.clone()).ok())
                    .unwrap_or_default();
                Ok(entities)
            }
            Ok(_) | Err(_) => Ok(Vec::new()),
        }
    }

    /// Find related threats
    pub async fn find_related_threats(&mut self, slot_id: u64) -> Result<Vec<u64>> {
        let neighbors = self.get_neighbors(slot_id, 2).await?;
        Ok(neighbors
            .iter()
            .filter(|e| e.has_component("threat"))
            .map(|e| e.slot_id)
            .collect())
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Check if SlotGraph is available
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/health", self.base_url());
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }
}

impl Default for SlotBridge {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_query() {
        let query = SlotQuery::entity(123);
        assert!(query.entity_id == Some(123));

        let query = SlotQuery::neighbors(456, 2);
        assert!(query.max_depth == Some(2));

        let query = SlotQuery::by_component("threat");
        assert!(query.component_type == Some("threat".to_string()));
    }

    #[test]
    fn test_slot_entity() {
        let entity = SlotEntity {
            slot_id: 1,
            hash: 12345,
            components: HashMap::new(),
            relationships: vec![SlotRelationship {
                target_id: 2,
                rel_type: "related".to_string(),
                weight: 0.5,
            }],
        };

        assert_eq!(entity.related_ids(), vec![2]);
    }
}
