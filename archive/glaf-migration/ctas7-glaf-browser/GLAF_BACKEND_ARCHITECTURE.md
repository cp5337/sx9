# GLAF Browser Backend Architecture

## The Real Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│                      GLAF Browser (Frontend)                        │
│                   React + D3 Visualization                          │
│                      Port 8080 (dev)                                │
└──────────────────────────┬──────────────────────────────────────────┘
                           │ WebSocket / HTTP
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    GLAF Graph Server (Rust)                         │
│                 ctas7-glaf-graph-server                             │
│                      Port 18050                                     │
├─────────────────────────────────────────────────────────────────────┤
│  Axum/Actix HTTP Server                                            │
│  ├── GET  /api/graph/nodes                                         │
│  ├── GET  /api/graph/relationships                                 │
│  ├── POST /api/query (SurrealQL passthrough)                       │
│  ├── POST /api/glaf/matroid-rank                                   │
│  ├── POST /api/glaf/hawkes-intensity                               │
│  ├── POST /api/glaf/convergence                                    │
│  └── WS   /ws/stream (real-time graph updates)                     │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
┌─────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ SlotGraph   │  │ GLAF Matroid    │  │ SurrealDB       │
│ Engine      │  │ Core            │  │ (Data Store)    │
│ (Legion ECS)│  │ (Math)          │  │                 │
├─────────────┤  ├─────────────────┤  ├─────────────────┤
│ • petgraph  │  │ • nalgebra      │  │ Port 18019      │
│ • Entity    │  │ • Matroid rank  │  │ • PTCC configs  │
│ • Position  │  │ • Hawkes proc   │  │ • TETH entropy  │
│ • OODA      │  │ • Convergence   │  │ • Scenarios     │
│ • Topology  │  │ • H1/H2 scores  │  │ • OSINT tools   │
└─────────────┘  └─────────────────┘  └─────────────────┘
```

## Rust Crates Involved

### Core Graph Backend
```toml
[package]
name = "ctas7-glaf-graph-server"
version = "0.1.0"

[dependencies]
# ECS & Graph
ctas7-slotgraph-engine = { path = "../ctas7-slotgraph-engine" }
ctas7-world-ecs = { path = "../ctas7-world-ecs" }
ctas7-glaf-matroid-core = { path = "../ctas7-glaf-matroid-core" }

# Foundation
ctas7-foundation-core = { path = "../ctas7-foundation-core" }

# Web Server
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# SurrealDB client
surrealdb = "2.0"

# WebSocket
axum-extra = { version = "0.9", features = ["typed-header"] }
tokio-tungstenite = "0.21"
```

### API Endpoints

#### Graph Queries
```rust
// GET /api/graph/nodes?limit=100&hd4_phase=Hunt
async fn get_nodes(Query(params): Query<NodeParams>) -> Json<Vec<GlafNode>>

// GET /api/graph/relationships?from=threat_actor:volt_typhoon
async fn get_relationships(Query(params): Query<RelParams>) -> Json<Vec<GlafEdge>>

// POST /api/query
// Body: { "surql": "SELECT * FROM ptcc_configurations LIMIT 50" }
async fn run_query(Json(body): Json<QueryRequest>) -> Json<QueryResult>
```

#### GLAF Math (The Good Stuff)
```rust
// POST /api/glaf/matroid-rank
// Body: { "fragment_ids": ["node:1", "node:2", "node:3"] }
async fn calculate_matroid_rank(Json(body): Json<MatroidRequest>) -> Json<MatroidResult>

// POST /api/glaf/hawkes-intensity
// Body: { "event_type": "threat_detection", "window_hours": 24 }
async fn calculate_hawkes(Json(body): Json<HawkesRequest>) -> Json<HawkesResult>

// POST /api/glaf/convergence
// Body: { "fragment_indices": [0, 1, 2], "h1_input": 0.8 }
async fn calculate_convergence(Json(body): Json<ConvergenceRequest>) -> Json<ConvergenceResult>
```

#### Real-time Streaming
```rust
// WS /ws/stream
// Streams graph updates as entities change in Legion ECS
async fn graph_stream(ws: WebSocketUpgrade) -> impl IntoResponse
```

## Data Flow

### Query Flow
```
Browser → HTTP POST /api/query → GLAF Server → SurrealDB
                                      ↓
                              Transform to Graph
                                      ↓
                              Apply GLAF Math
                                      ↓
                              Return JSON
```

### Real-time Flow
```
Legion ECS World → Change Detection → WebSocket → Browser
       ↓
   SlotGraph View
       ↓
   petgraph topology
       ↓
   GLAF Math (Hawkes, Convergence)
```

## Why This Architecture?

1. **Rust Backend = Fast** - No JS/Python overhead for graph operations
2. **Legion ECS = Scalable** - Handle millions of entities
3. **GLAF Math in Rust** - Matroid/Hawkes runs at native speed
4. **SurrealDB = Persistence** - Graph data survives restarts
5. **Browser = Dumb Renderer** - Just visualizes what backend sends

## Neo4j Browser Compatibility

The GLAF server returns data in Neo4j-compatible format:

```json
{
  "nodes": [
    {
      "id": "threat_actor:volt_typhoon",
      "elementId": "threat_actor:volt_typhoon",
      "labels": ["ThreatActor", "APT"],
      "properties": {
        "name": "Volt Typhoon",
        "skill_level": 4.0,
        "region": "CN"
      },
      "_glaf": {
        "trivHash": "triv:abc123...",
        "hd4Phase": "Disrupt",
        "tethEntropy": 0.95,
        "matroidRank": 0.87
      }
    }
  ],
  "relationships": [...]
}
```

The `_glaf` field contains GLAF-specific data that the forked browser can use for:
- Color coding by HD4 phase
- Node sizing by TETH entropy
- Edge weight by Hawkes intensity


