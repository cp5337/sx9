# GLAF Engine Gateway Integration
## Using GLAF as the Graphing Engine for Gateway

**Date:** December 2025  
**Status:** Integration Plan  
**Goal:** Use existing `ctas7-glaf-graph-server` as the graphing engine for gateway graph operations

---

## Executive Summary

**Three Integration Points:**

1. **sx9-glaf-core** - Neural operations (ANN/GNN)
   - ANN/GNN model integration
   - Topology mirroring for routing feedback
   - Neural network inference and training
   - **Purpose:** Cognitive/neural operations

2. **GLAF Client (HTTP to ctas7-glaf-graph-server)** - Data analytics engine
   - Full graph operations (queries, analysis, visualization)
   - Cypher++ queries, APOC++ procedures
   - Matroid, Hawkes, Convergence calculations
   - **Purpose:** Data analytics and analytic workbench

3. **ctas7-glaf-graph-server (Port 18050)** - GLAF Graph Server
   - Full graph operations engine
   - Already built and running
   - **Purpose:** Backend for GLAF client

**Strategy:**
- **Gateway neural operations (ANN/GNN)** → Use `sx9-glaf-core` (direct integration)
- **Gateway data analytics** → Use `GLAFClient` → HTTP API to `ctas7-glaf-graph-server` (port 18050)
- **Gateway topology mirroring** → Use `sx9-glaf-core` (routing state observation)
- **Analytic workbench (future)** → Use `GLAFClient` for all graph operations

---

## 1. GLAF Graph Server (The Engine)

### 1.1 Existing GLAF Server

**Location:** `ctas7-glaf-graph-server/`  
**Port:** 18050  
**Status:** ✅ Already built and running

**API Endpoints:**
```rust
GET  /health
GET  /api/graph/nodes?limit=100&hd4_phase=Hunt&label=Agent
GET  /api/graph/relationships
POST /api/query                    // SurrealQL queries
POST /api/glaf/matroid-rank        // Matroid independence
POST /api/glaf/hawkes-intensity    // Event prediction
POST /api/glaf/convergence         // H1/H2 convergence
WS   /ws/stream                    // Real-time updates
```

### 1.2 Gateway Integration Pattern

**Gateway should use GLAF server via HTTP client:**

```rust
// In sx9-gateway/src/handlers.rs

use reqwest::Client;

pub struct GLAFClient {
    client: Client,
    base_url: String,  // http://localhost:18050
}

impl GLAFClient {
    pub async fn query_graph(&self, query: &str) -> Result<GraphResult> {
        let response = self.client
            .post(&format!("{}/api/query", self.base_url))
            .json(&serde_json::json!({ "surql": query }))
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
    
    pub async fn get_nodes(&self, filter: &NodeFilter) -> Result<Vec<GlafNode>> {
        let mut url = format!("{}/api/graph/nodes", self.base_url);
        // Add query params from filter
        if let Some(limit) = filter.limit {
            url.push_str(&format!("?limit={}", limit));
        }
        // ... more params
        
        let response = self.client.get(&url).send().await?;
        Ok(response.json().await?)
    }
    
    pub async fn calculate_matroid_rank(&self, fragment_ids: &[String]) -> Result<MatroidResult> {
        let response = self.client
            .post(&format!("{}/api/glaf/matroid-rank", self.base_url))
            .json(&serde_json::json!({ "fragment_ids": fragment_ids }))
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
}
```

---

## 2. Gateway Graph Handler Updates

### 2.1 Current Implementation

**File:** `sx9-gateway/src/handlers.rs`

**Current:** Gateway queries SurrealDB directly for graph operations

```rust
async fn handle_get_graph(
    filter: GraphFilter,
    state: SharedState,
) -> Result<WsResponse> {
    // Currently: Direct SurrealDB query
    let query = String::from("SELECT * FROM entity");
    // ... builds query ...
    
    if let Some(ref surreal) = state.surrealdb {
        match surreal.query(&query).await {
            // ... transforms to GraphNode/GraphEdge ...
        }
    }
}
```

### 2.2 Updated Implementation (Use GLAF Engine)

**Updated:** Gateway calls GLAF server API

```rust
async fn handle_get_graph(
    filter: GraphFilter,
    state: SharedState,
) -> Result<WsResponse> {
    // Use GLAF engine instead of direct SurrealDB
    if let Some(ref glaf_client) = state.glaf_client {
        // Convert filter to GLAF query
        let glaf_query = build_glaf_query(&filter);
        
        match glaf_client.query_graph(&glaf_query).await {
            Ok(glaf_result) => {
                // Transform GLAF result to gateway format
                let nodes = transform_glaf_nodes(glaf_result.nodes);
                let edges = transform_glaf_edges(glaf_result.relationships);
                
                Ok(WsResponse::GraphData { nodes, edges })
            }
            Err(e) => Ok(WsResponse::Error {
                code: "GLAF_ERROR".to_string(),
                message: e.to_string(),
                details: None,
            }),
        }
    } else {
        // Fallback to direct SurrealDB if GLAF not available
        // ... existing SurrealDB code ...
    }
}
```

---

## 3. Gateway State Updates

### 3.1 Add GLAF Client to State

**File:** `sx9-gateway/src/state.rs`

```rust
use crate::glaf_client::GLAFClient;

pub struct GatewayState {
    // ... existing fields ...
    
    /// GLAF Graph Server client (port 18050)
    pub glaf_client: Option<Arc<GLAFClient>>,
    
    /// GLAF topology mirror (for routing feedback)
    #[cfg(feature = "glaf")]
    pub glaf_topology: Option<Arc<sx9_glaf_core::TopologyMirror>>,
}

impl GatewayState {
    pub async fn new() -> Result<Self> {
        // ... existing initialization ...
        
        // Initialize GLAF client
        let glaf_client = if let Ok(client) = GLAFClient::new("http://localhost:18050").await {
            Some(Arc::new(client))
        } else {
            tracing::warn!("GLAF server not available, graph operations will use SurrealDB directly");
            None
        };
        
        Ok(Self {
            // ... existing fields ...
            glaf_client,
            #[cfg(feature = "glaf")]
            glaf_topology: None,  // Initialized separately if enabled
        })
    }
}
```

---

## 4. Complete Integration Architecture

### 4.1 Gateway → GLAF Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    Gateway Client (UI)                        │
└───────────────────────────┬─────────────────────────────────┘
                            │ WebSocket
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              sx9-gateway (Port 18120-18122)                 │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Graph Handler│  │ Query Handler│ │ Workflow     │      │
│  └──────┬───────┘  └──────┬───────┘ │ Handler      │      │
│         │                  │         └──────────────┘      │
│         │                  │                                │
│         ▼                  ▼                                │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │ GLAF Client  │  │ SurrealDB    │                        │
│  │ (HTTP API)   │  │ (Direct)     │                        │
│  └──────┬───────┘  └──────┬───────┘                        │
└─────────┼──────────────────┼────────────────────────────────┘
          │                  │
          │ HTTP             │ Direct
          ▼                  ▼
┌──────────────────┐  ┌──────────────┐
│ GLAF Graph Server│  │  SurrealDB   │
│ (Port 18050)     │  │  (Port 8000) │
│                  │  │              │
│ • Cypher++       │  │ • Graph DB   │
│ • APOC++ (68)    │  │ • ACID       │
│ • Matroid        │  │ • Live Query │
│ • Hawkes         │  │              │
│ • Convergence    │  │              │
└──────────────────┘  └──────────────┘
          │
          │ Uses
          ▼
┌──────────────────┐
│ SlotGraph Engine │
│ (Legion ECS)     │
└──────────────────┘
```

### 4.2 Three GLAF Integration Points

**1. sx9-glaf-core (Neural Operations) - Internal**
- **Purpose:** ANN/GNN operations, topology mirroring
- **Access:** Direct integration in gateway
- **Use Cases:**
  - ANN/GNN model inference
  - Neural network training (observer mode)
  - Topology mirroring (routing state observation)
  - Routing feedback for optimization
  - Cognitive graph operations

**2. GLAF Client (Data Analytics) - HTTP API**
- **Purpose:** Data analytics, graph queries, analytic workbench
- **Access:** HTTP client to `ctas7-glaf-graph-server`
- **Use Cases:**
  - Graph queries (`GetGraph`, `GetFusionNodes`, `ExpandNode`)
  - GLAF math operations (`MatroidRank`, `HawkesIntensity`, `Convergence`)
  - Correlation analysis (`RunCorrelation`)
  - Real-time graph streaming
  - Analytic workbench operations (future)

**3. ctas7-glaf-graph-server (Backend) - Port 18050**
- **Purpose:** Full graph operations engine backend
- **Access:** HTTP API (called by GLAF Client)
- **Use Cases:**
  - Cypher++ query execution
  - APOC++ procedure execution (68 procedures)
  - Graph storage and retrieval
  - Real-time graph updates

---

## 5. Implementation Plan

### 5.1 Create GLAF Client Module

**File:** `sx9-gateway/src/glaf_client.rs`

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct GLAFClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct QueryRequest {
    surql: String,
}

#[derive(Debug, Deserialize)]
struct QueryResult {
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

impl GLAFClient {
    pub async fn new(base_url: impl Into<String>) -> Result<Self> {
        let client = Client::new();
        let base_url = base_url.into();
        
        // Health check
        let health_url = format!("{}/health", base_url);
        let response = client.get(&health_url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("GLAF server not healthy"));
        }
        
        Ok(Self { client, base_url })
    }
    
    pub async fn query_graph(&self, surql: &str) -> Result<QueryResult> {
        let response = self.client
            .post(&format!("{}/api/query", self.base_url))
            .json(&QueryRequest { surql: surql.to_string() })
            .send()
            .await?;
        
        response.json().await.map_err(Into::into)
    }
    
    pub async fn get_nodes(
        &self,
        limit: Option<usize>,
        hd4_phase: Option<&str>,
        label: Option<&str>,
    ) -> Result<Vec<GlafNode>> {
        let mut url = format!("{}/api/graph/nodes", self.base_url);
        let mut params = Vec::new();
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(phase) = hd4_phase {
            params.push(format!("hd4_phase={}", phase));
        }
        if let Some(label) = label {
            params.push(format!("label={}", label));
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        
        let response = self.client.get(&url).send().await?;
        response.json().await.map_err(Into::into)
    }
    
    pub async fn calculate_matroid_rank(&self, fragment_ids: &[String]) -> Result<MatroidResult> {
        let response = self.client
            .post(&format!("{}/api/glaf/matroid-rank", self.base_url))
            .json(&serde_json::json!({ "fragment_ids": fragment_ids }))
            .send()
            .await?;
        
        response.json().await.map_err(Into::into)
    }
    
    pub async fn calculate_hawkes(&self, event_type: &str, window_hours: f64) -> Result<HawkesResult> {
        let response = self.client
            .post(&format!("{}/api/glaf/hawkes-intensity", self.base_url))
            .json(&serde_json::json!({
                "event_type": event_type,
                "window_hours": window_hours
            }))
            .send()
            .await?;
        
        response.json().await.map_err(Into::into)
    }
}
```

### 5.2 Update Gateway Handlers

**File:** `sx9-gateway/src/handlers.rs`

```rust
use crate::glaf_client::GLAFClient;

async fn handle_get_graph(
    filter: GraphFilter,
    state: SharedState,
) -> Result<WsResponse> {
    // Try GLAF engine first
    if let Some(ref glaf_client) = state.glaf_client {
        // Build SurrealQL query from filter
        let mut surql = String::from("SELECT * FROM entity");
        
        if let Some(ref node_type) = filter.node_type {
            surql.push_str(&format!(" WHERE type = '{}'", node_type));
        }
        
        if filter.fusion_only {
            surql.push_str(" WHERE fusion_score IS NOT NULL");
        }
        
        surql.push_str(" FETCH edges");
        
        match glaf_client.query_graph(&surql).await {
            Ok(glaf_result) => {
                // Transform GLAF nodes to gateway format
                let nodes: Vec<GraphNode> = glaf_result.nodes.iter().map(|n| {
                    GraphNode {
                        id: n.id.clone(),
                        label: n.properties.get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        node_type: n.labels.first().cloned().unwrap_or_default(),
                        shape: if n.glaf_meta.matroid_rank.is_some() {
                            "nonagon"
                        } else {
                            "circle"
                        }.to_string(),
                        color: if n.glaf_meta.hd4_phase.as_ref().map(|s| s == "Hunt").unwrap_or(false) {
                            "#ff0000"
                        } else {
                            "#00ffff"
                        }.to_string(),
                        size: 1.0,
                        trivariate_hash: n.glaf_meta.triv_hash.clone(),
                        source_db: Database::GLAF,  // New database type
                        properties: n.properties.clone(),
                    }
                }).collect();
                
                // Transform GLAF relationships to edges
                let edges: Vec<GraphEdge> = glaf_result.relationships.iter().map(|r| {
                    GraphEdge {
                        id: r.id.clone(),
                        source: r.start_node_id.clone(),
                        target: r.end_node_id.clone(),
                        edge_type: r.rel_type.clone(),
                        weight: 1.0,
                        properties: r.properties.clone(),
                    }
                }).collect();
                
                Ok(WsResponse::GraphData { nodes, edges })
            }
            Err(e) => {
                tracing::warn!("GLAF query failed: {}, falling back to SurrealDB", e);
                // Fallback to SurrealDB
                handle_get_graph_surrealdb(filter, state).await
            }
        }
    } else {
        // No GLAF client, use SurrealDB directly
        handle_get_graph_surrealdb(filter, state).await
    }
}

// Keep existing SurrealDB handler as fallback
async fn handle_get_graph_surrealdb(
    filter: GraphFilter,
    state: SharedState,
) -> Result<WsResponse> {
    // ... existing SurrealDB code ...
}
```

### 5.3 Add GLAF Database Type

**File:** `sx9-gateway/src/protocol.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Database {
    Surrealdb,
    Supabase,
    Sled,
    Sledis,
    Nats,
    GLAF,  // NEW: GLAF Graph Server
}

impl Database {
    pub fn brand_color(&self) -> &'static str {
        match self {
            Database::Surrealdb => "#ff00ff",
            Database::Supabase => "#3ecf8e",
            Database::Sled => "#00ff00",
            Database::Sledis => "#ffff00",
            Database::Nats => "#00ffff",
            Database::GLAF => "#00ff00",  // Green for GLAF
        }
    }
}
```

---

## 6. Smart Crate Manifest Updates

### 6.1 Gateway Smart Crate

**File:** `sx9-gateway/smart-crate.toml`

```toml
[glaf]
engine = "sx9-glaf-core"
enabled = false
mirror_slotgraph = true
sync_interval_ms = 100
topology_feedback = true

# GLAF Graph Server integration
graph_server_enabled = true
graph_server_url = "http://localhost:18050"
graph_server_port = 18050

# Use GLAF engine for graph operations
use_glaf_engine = true
fallback_to_surrealdb = true  # Fallback if GLAF unavailable
```

---

## 7. Summary

**Architecture: Three-Tier GLAF Integration**

**1. sx9-glaf-core (Neural Operations)**
- **Purpose:** ANN/GNN operations, topology mirroring
- **Integration:** Direct in gateway
- **Use Cases:**
  - ANN/GNN model inference
  - Neural network training (observer mode)
  - Topology mirroring (routing state)
  - Routing feedback

**2. GLAF Client (Data Analytics)**
- **Purpose:** Data analytics, graph queries, analytic workbench
- **Integration:** HTTP client to `ctas7-glaf-graph-server`
- **Use Cases:**
  - Graph queries and analysis
  - GLAF math operations
  - Correlation analysis
  - Analytic workbench (future)

**3. ctas7-glaf-graph-server (Backend)**
- **Purpose:** Full graph operations engine
- **Port:** 18050
- **Access:** Via GLAF Client HTTP API

**Benefits:**
- ✅ Clear separation: Neural (core) vs Analytics (client)
- ✅ Reuses existing GLAF engine for analytics
- ✅ Direct neural operations for ANN/GNN
- ✅ Analytic workbench ready (via GLAF client)
- ✅ Fallback to SurrealDB if GLAF unavailable

**Implementation:**
1. Create `sx9-glaf-core` for ANN/GNN + topology mirroring
2. Create `GLAFClient` in gateway for data analytics
3. Update graph handlers to use GLAF client
4. Add both to gateway state
5. Test with existing GLAF server

---

**Status:** Integration plan complete. Three-tier architecture: GLAF core (neural) + GLAF client (analytics) + GLAF server (backend).

