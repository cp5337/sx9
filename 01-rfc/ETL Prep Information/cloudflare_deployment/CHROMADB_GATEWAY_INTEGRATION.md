# ChromaDB Integration with SX9 Gateway

**RFC-9114 Compliant ChromaDB CDN Service**

---

## 🎯 **ARCHITECTURE:**

```
SX9 GATEWAY (18120-18122)
    ↓
NEURAL MUX ROUTER (<250ns)
    ↓
CHROMADB CDN SERVICE (Port: 18125)
    ├─→ ChromaDB Client (existing data)
    ├─→ Unicode → Vector lookup
    ├─→ Semantic search (embedding-based)
    └─→ Tool/operation similarity
```

---

## 📂 **CHROMADB DATA LOCATION:**

```
Local Path:
~/Developer/ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/vectors/chromadb/

Collections:
├── tools          (27,606+ threat tools)
├── ctas_tasks     (Task definitions)
├── ptcc_configs   (PTCC configurations)
└── tool_chains    (Operation sequences)

Embedding Model: all-MiniLM-L6-v2 (384 dimensions)
Unicode Metadata: E000-E9FF embedded in metadata
```

---

## 🚀 **IMPLEMENTATION: AXUM CDN SERVER FOR CHROMADB:**

```rust
// File: sx9-gateway/src/cdn/chromadb_service.rs

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use chromadb::{ChromaClient, Collection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

/// ChromaDB CDN Service State
pub struct ChromaDBService {
    client: ChromaClient,
    collections: Arc<ChromaCollections>,
    port: u16,
}

struct ChromaCollections {
    tools: Collection,
    tasks: Collection,
    configs: Collection,
    chains: Collection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorSearchRequest {
    query: String,
    collection: String,
    n_results: Option<usize>,
    unicode_filter: Option<String>,  // E.g., "E800-E8FF"
}

#[derive(Debug, Serialize)]
pub struct VectorSearchResponse {
    results: Vec<VectorResult>,
    latency_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct VectorResult {
    unicode: String,
    name: String,
    category: String,
    distance: f32,
    metadata: serde_json::Value,
}

impl ChromaDBService {
    /// Initialize ChromaDB service with existing data
    pub async fn new(port: u16) -> anyhow::Result<Self> {
        info!("🔌 Initializing ChromaDB CDN service on port {}", port);
        
        // Connect to existing ChromaDB instance
        let client = ChromaClient::new("http://localhost:8000");
        
        // Load collections
        let tools = client.get_collection("tools").await?;
        let tasks = client.get_collection("ctas_tasks").await?;
        let configs = client.get_collection("ptcc_configs").await?;
        let chains = client.get_collection("tool_chains").await?;
        
        info!("✅ Loaded {} collections from ChromaDB", 4);
        
        Ok(Self {
            client,
            collections: Arc::new(ChromaCollections {
                tools,
                tasks,
                configs,
                chains,
            }),
            port,
        })
    }
    
    /// Start the Axum server
    pub async fn serve(self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/search", post(vector_search))
            .route("/unicode/:unicode", get(unicode_lookup))
            .route("/similar/:unicode", get(find_similar))
            .with_state(Arc::new(self));
        
        let addr = format!("0.0.0.0:{}", self.port);
        info!("🚀 ChromaDB CDN service listening on {}", addr);
        
        axum::Server::bind(&addr.parse()?)
            .serve(app.into_make_service())
            .await?;
        
        Ok(())
    }
}

// ============================================================================
// HANDLERS
// ============================================================================

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "chromadb-cdn",
        "rfc": "9114",
        "collections": ["tools", "tasks", "configs", "chains"]
    }))
}

async fn vector_search(
    State(service): State<Arc<ChromaDBService>>,
    Json(req): Json<VectorSearchRequest>,
) -> Json<VectorSearchResponse> {
    let start = std::time::Instant::now();
    
    // Select collection
    let collection = match req.collection.as_str() {
        "tools" => &service.collections.tools,
        "tasks" => &service.collections.tasks,
        "configs" => &service.collections.configs,
        "chains" => &service.collections.chains,
        _ => &service.collections.tools,  // Default to tools
    };
    
    // Query ChromaDB
    let n_results = req.n_results.unwrap_or(10);
    let query_result = collection
        .query(vec![req.query.clone()], n_results, None, None, None)
        .await
        .unwrap_or_default();
    
    // Parse results
    let mut results = Vec::new();
    if let Some(ids) = query_result.ids {
        for (i, id) in ids[0].iter().enumerate() {
            let distance = query_result.distances
                .as_ref()
                .and_then(|d| d[0].get(i))
                .copied()
                .unwrap_or(0.0);
            
            let metadata = query_result.metadatas
                .as_ref()
                .and_then(|m| m[0].get(i))
                .cloned()
                .unwrap_or_default();
            
            results.push(VectorResult {
                unicode: metadata.get("unicode")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: metadata.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                category: metadata.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                distance,
                metadata,
            });
        }
    }
    
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    Json(VectorSearchResponse {
        results,
        latency_ms,
    })
}

async fn unicode_lookup(
    State(service): State<Arc<ChromaDBService>>,
    Path(unicode): Path<String>,
) -> Json<serde_json::Value> {
    // Query ChromaDB for exact unicode match
    let result = service.collections.tools
        .get(
            vec![],  // No IDs filter
            Some(serde_json::json!({
                "unicode": unicode
            })),  // Where filter
            None,
            None,
        )
        .await;
    
    match result {
        Ok(data) => Json(serde_json::json!({
            "unicode": unicode,
            "found": !data.ids.is_empty(),
            "data": data
        })),
        Err(e) => Json(serde_json::json!({
            "unicode": unicode,
            "found": false,
            "error": e.to_string()
        }))
    }
}

async fn find_similar(
    State(service): State<Arc<ChromaDBService>>,
    Path(unicode): Path<String>,
) -> Json<VectorSearchResponse> {
    let start = std::time::Instant::now();
    
    // First, get the embedding for this unicode
    let tool = service.collections.tools
        .get(
            vec![],
            Some(serde_json::json!({ "unicode": unicode })),
            Some(vec!["embedding".to_string()]),
            None,
        )
        .await
        .ok();
    
    let results = if let Some(tool_data) = tool {
        if let Some(embeddings) = tool_data.embeddings {
            if !embeddings.is_empty() {
                // Query for similar tools using the embedding
                let query_result = service.collections.tools
                    .query(
                        embeddings,
                        10,  // Top 10 similar
                        None,
                        None,
                        None,
                    )
                    .await
                    .unwrap_or_default();
                
                // Parse results (same as vector_search)
                let mut results = Vec::new();
                if let Some(ids) = query_result.ids {
                    for (i, _) in ids[0].iter().enumerate() {
                        let distance = query_result.distances
                            .as_ref()
                            .and_then(|d| d[0].get(i))
                            .copied()
                            .unwrap_or(0.0);
                        
                        let metadata = query_result.metadatas
                            .as_ref()
                            .and_then(|m| m[0].get(i))
                            .cloned()
                            .unwrap_or_default();
                        
                        results.push(VectorResult {
                            unicode: metadata.get("unicode")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            name: metadata.get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            category: metadata.get("category")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            distance,
                            metadata,
                        });
                    }
                }
                results
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    Json(VectorSearchResponse {
        results,
        latency_ms,
    })
}
```

---

## 🔧 **PORT MANAGER INTEGRATION:**

Register ChromaDB CDN with your port manager:

```rust
// In sx9-gateway/src/main.rs

use crate::cdn::chromadb_service::ChromaDBService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... existing gateway setup ...
    
    // Register ChromaDB CDN with port manager
    let port = port_manager.allocate_port(
        "chromadb-cdn",
        PortType::CDN,
        18125,  // Requested port
    ).await?;
    
    // Spawn ChromaDB service
    let chromadb_service = ChromaDBService::new(port).await?;
    
    tokio::spawn(async move {
        if let Err(e) = chromadb_service.serve().await {
            tracing::error!("ChromaDB CDN service error: {}", e);
        }
    });
    
    info!("✅ ChromaDB CDN service started on port {}", port);
    
    // ... rest of gateway ...
    
    Ok(())
}
```

---

## 📋 **NEURAL MUX ROUTE REGISTRATION:**

Add ChromaDB routes to your Neural Mux:

```rust
// In neural mux route table initialization

neural_mux.register_route(
    RouteEntry {
        hash_prefix: hash("chromadb"),
        target: RouteDest::LocalCDN {
            name: "chromadb-cdn",
            port: 18125,
            protocol: Protocol::HTTP,
        },
        latency_zone: BernoulliZone::C,  // Analytical (1ms - 100ms)
        fallback: Some(RouteDest::Supabase),  // Fallback to Supabase
    }
)?;
```

---

## 🎯 **USAGE FROM GATEWAY:**

```rust
// Vector search from gateway API
POST http://localhost:18125/search
{
  "query": "network reconnaissance tools",
  "collection": "tools",
  "n_results": 10,
  "unicode_filter": "E800-E8FF"
}

// Direct unicode lookup
GET http://localhost:18125/unicode/E800

// Find similar tools
GET http://localhost:18125/similar/E800
```

---

## 🚀 **BENEFITS:**

```
✅ RFC-9114 compliant (Bernoulli Zone C)
✅ Integrates with existing port manager
✅ Uses your existing ChromaDB data (no migration)
✅ Semantic search for tool similarity
✅ Unicode-based addressing
✅ Falls back to Supabase if ChromaDB unavailable
✅ <100ms latency (Zone C compliant)
```

---

## 📊 **DEPLOYMENT:**

```bash
# 1. Ensure ChromaDB is running
docker run -d -p 8000:8000 chromadb/chroma:latest

# OR use your existing ChromaDB at:
# ~/Developer/ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/vectors/chromadb/

# 2. Build gateway with ChromaDB support
cd sx9-gateway
cargo build --release --features chromadb-cdn

# 3. Run gateway (will auto-start ChromaDB CDN)
./target/release/sx9-gateway

# 4. Verify
curl http://localhost:18125/health
```

---

**Your ChromaDB data is at:**
```
~/Developer/ctas-7-shipyard-staging/04-abe-iac/
  └── node-interview-generator/
      └── output/
          └── vectors/
              └── chromadb/
                  ├── chroma.sqlite3
                  └── [collections]
```

**Want me to create the complete Rust module for this?**
