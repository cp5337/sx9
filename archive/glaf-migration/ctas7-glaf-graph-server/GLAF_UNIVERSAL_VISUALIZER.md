# GLAF Universal Data Visualizer

## Architecture: MCP Server + CDN + Containerized API

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         GLAF UNIVERSAL VISUALIZER                           │
│                      (Browser / Electron / WASM)                            │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    ▼                              ▼
┌───────────────────────────┐      ┌───────────────────────────────────────────┐
│     MCP Server            │      │         Containerized API                 │
│  (Cursor/Claude/etc)      │      │      ctas7-glaf-visualizer-api            │
│     Port 18051            │      │           Port 18052                      │
├───────────────────────────┤      ├───────────────────────────────────────────┤
│ Tools:                    │      │ REST Endpoints:                           │
│ • query_graph             │      │ • GET  /api/v1/databases                  │
│ • query_sql               │      │ • GET  /api/v1/graph/{db}                 │
│ • query_geojson           │      │ • GET  /api/v1/table/{db}/{table}         │
│ • visualize_data          │      │ • GET  /api/v1/geojson/{layer}            │
│ • get_schema              │      │ • POST /api/v1/query                      │
│ • export_format           │      │ • WS   /ws/live                           │
└─────────────┬─────────────┘      └────────────────┬──────────────────────────┘
              │                                      │
              └──────────────────┬───────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          CDN AGGREGATION LAYER                              │
│                    ctas7-cdn-data-fabric (Port 18100)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│  • Database Registry (which DBs are online)                                 │
│  • Schema Cache (table/graph schemas from all DBs)                          │
│  • Query Router (route to appropriate DB)                                   │
│  • Format Transformer (graph ↔ table ↔ geojson ↔ cypher ↔ sql)             │
│  • Live Subscriptions (DBs push updates here)                               │
└─────────────────────────────────────────────────────────────────────────────┘
                                 │
        ┌────────────────────────┼────────────────────────┐
        │                        │                        │
        ▼                        ▼                        ▼
┌───────────────┐  ┌───────────────────┐  ┌───────────────────────────────────┐
│ Graph DBs     │  │ Relational DBs    │  │ Geo/Spatial                       │
├───────────────┤  ├───────────────────┤  ├───────────────────────────────────┤
│ SurrealDB     │  │ PostgreSQL        │  │ GeoJSON Files                     │
│ :8000 (main)  │  │ :5432 (supabase)  │  │ • ground-stations.geojson         │
│ :18019 (glaf) │  │                   │  │ • submarine-cables.geojson        │
│ :18025 (anlyt)│  │ SQLite            │  │ • cable-landings.geojson          │
│               │  │ (embedded)        │  │                                   │
│ Neo4j         │  │                   │  │ Cesium/Mapbox Tiles               │
│ :7687 (viz)   │  │                   │  │ • Orbital data                    │
├───────────────┤  ├───────────────────┤  │ • Ground station coverage         │
│ SlotGraph ECS │  │                   │  │                                   │
│ :9001         │  │                   │  │                                   │
└───────────────┘  └───────────────────┘  └───────────────────────────────────┘
        │                        │                        │
        └────────────────────────┴────────────────────────┘
                                 │
                          REPORT TO CDN
                          (on startup/change)
```

## Database Registry Protocol

When a database comes online, it registers with the CDN:

```json
POST /api/v1/registry/register
{
  "db_id": "surrealdb-glaf-core",
  "db_type": "surrealdb",
  "host": "localhost",
  "port": 18019,
  "namespace": "ctas7",
  "database": "glaf",
  "capabilities": ["graph", "sql", "live_queries"],
  "schema": {
    "tables": ["ptcc_configurations", "attack_scenarios", "osint_tools"],
    "graph_types": ["threat_actor", "technique", "uses", "correlates"]
  },
  "health_endpoint": "/health",
  "heartbeat_interval_ms": 30000
}
```

## Supported Views

### 1. Graph View (Neo4j Browser Style)
```
┌─────────────────────────────────────────────────────────────────┐
│ [SurrealDB ▼] [GLAF Core ▼]  Query: SELECT * FROM threat_actors │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│         ┌──────┐                    ┌──────┐                   │
│         │ APT29│───uses────────────▶│T1059 │                   │
│         └──────┘                    └──────┘                   │
│              │                          │                       │
│              │correlates                │mitigated_by           │
│              ▼                          ▼                       │
│         ┌──────┐                    ┌──────┐                   │
│         │Volt  │                    │M1042 │                   │
│         │Typhoon                    └──────┘                   │
│         └──────┘                                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2. Table View (SQL Results)
```
┌─────────────────────────────────────────────────────────────────┐
│ [PostgreSQL ▼] [supabase ▼]  Query: SELECT * FROM users LIMIT 10│
├─────────────────────────────────────────────────────────────────┤
│ id  │ name           │ email              │ created_at          │
│─────┼────────────────┼────────────────────┼─────────────────────│
│ 1   │ Alice          │ alice@example.com  │ 2025-01-01 00:00:00 │
│ 2   │ Bob            │ bob@example.com    │ 2025-01-02 00:00:00 │
└─────────────────────────────────────────────────────────────────┘
```

### 3. GeoJSON/Map View
```
┌─────────────────────────────────────────────────────────────────┐
│ [GeoJSON ▼] [ground-stations ▼]  Layer: Ground Stations         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│    🛰️ Stockholm          🛰️ Tokyo                              │
│         ╲                    ╱                                  │
│          ╲    🌐 CDN       ╱                                   │
│           ╲   Hub        ╱                                     │
│            ╲            ╱                                       │
│    🛰️ NYC ──────────────── 🛰️ Singapore                        │
│                                                                 │
│    [Submarine Cables] [Landing Points] [Coverage Zones]        │
└─────────────────────────────────────────────────────────────────┘
```

### 4. Cypher/SurrealQL Editor
```
┌─────────────────────────────────────────────────────────────────┐
│ Query Language: [SurrealQL ▼]  Target: [GLAF Core ▼]            │
├─────────────────────────────────────────────────────────────────┤
│ SELECT                                                          │
│   *,                                                            │
│   ->uses->technique.* as techniques,                            │
│   <-correlates<-threat_actor.* as related                       │
│ FROM threat_actor:volt_typhoon                                  │
│ FETCH techniques, related                                       │
├─────────────────────────────────────────────────────────────────┤
│ [▶ Run] [Format] [Explain] [Export JSON] [Export CSV]           │
└─────────────────────────────────────────────────────────────────┘
```

## MCP Server Tools

```typescript
// mcp-glaf-visualizer/src/tools.ts

export const tools = [
  {
    name: "list_databases",
    description: "List all registered databases and their status",
    parameters: {}
  },
  {
    name: "query_graph",
    description: "Execute a graph query (SurrealQL, Cypher, or Gremlin)",
    parameters: {
      database: "string - database ID",
      query: "string - the query",
      format: "string - output format (graph|table|json)"
    }
  },
  {
    name: "query_sql",
    description: "Execute SQL query on relational database",
    parameters: {
      database: "string - database ID",
      query: "string - SQL query"
    }
  },
  {
    name: "get_geojson",
    description: "Get GeoJSON layer data",
    parameters: {
      layer: "string - layer name (ground-stations, cables, etc)"
    }
  },
  {
    name: "get_schema",
    description: "Get schema for a database",
    parameters: {
      database: "string - database ID"
    }
  },
  {
    name: "transform_format",
    description: "Transform data between formats",
    parameters: {
      data: "object - input data",
      from_format: "string - source format",
      to_format: "string - target format"
    }
  },
  {
    name: "subscribe_live",
    description: "Subscribe to live updates from a database",
    parameters: {
      database: "string - database ID",
      query: "string - subscription query"
    }
  }
];
```

## Docker Compose

```yaml
version: '3.8'

services:
  # CDN Data Fabric - Central Hub
  glaf-cdn-fabric:
    build: ./ctas7-cdn-data-fabric
    ports:
      - "18100:18100"
    environment:
      - RUST_LOG=info
    volumes:
      - ./data/cdn-geo:/data/geo:ro
    depends_on:
      - glaf-core
      - surrealdb-main

  # MCP Server for AI Integration
  glaf-mcp-server:
    build: ./mcp-glaf-visualizer
    ports:
      - "18051:18051"
    environment:
      - CDN_URL=http://glaf-cdn-fabric:18100

  # Containerized API
  glaf-visualizer-api:
    build: ./ctas7-glaf-visualizer-api
    ports:
      - "18052:18052"
    environment:
      - CDN_URL=http://glaf-cdn-fabric:18100

  # GLAF Core (SurrealDB)
  glaf-core:
    image: surrealdb/surrealdb:latest
    command: start --log info --user root --pass root --bind 0.0.0.0:18019 memory
    ports:
      - "18019:18019"
    environment:
      - REGISTER_CDN=http://glaf-cdn-fabric:18100/api/v1/registry/register

  # Main SurrealDB
  surrealdb-main:
    image: surrealdb/surrealdb:latest
    command: start --log info --user root --pass root --bind 0.0.0.0:8000 file:/data/surreal.db
    ports:
      - "8000:8000"
    volumes:
      - surrealdb-data:/data

volumes:
  surrealdb-data:
```

## Rust Crate Structure

```
ctas7-glaf-visualizer/
├── Cargo.toml
├── crates/
│   ├── glaf-cdn-fabric/        # CDN aggregation layer
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── registry.rs     # DB registration
│   │   │   ├── router.rs       # Query routing
│   │   │   ├── transform.rs    # Format transformation
│   │   │   └── live.rs         # Live subscriptions
│   │   └── Cargo.toml
│   │
│   ├── glaf-visualizer-api/    # REST/WS API
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── handlers/
│   │   │   │   ├── graph.rs
│   │   │   │   ├── table.rs
│   │   │   │   ├── geojson.rs
│   │   │   │   └── query.rs
│   │   │   └── ws.rs
│   │   └── Cargo.toml
│   │
│   └── mcp-glaf-visualizer/    # MCP Server
│       ├── src/
│       │   ├── main.rs
│       │   ├── tools.rs
│       │   └── handlers.rs
│       └── Cargo.toml
│
└── docker/
    ├── Dockerfile.cdn
    ├── Dockerfile.api
    ├── Dockerfile.mcp
    └── docker-compose.yml
```

## Database Adapters

Each database type has an adapter that:
1. Registers with CDN on startup
2. Reports schema changes
3. Pushes live updates
4. Handles queries in native format

```rust
// Adapter trait
pub trait DatabaseAdapter: Send + Sync {
    fn db_type(&self) -> &str;
    fn capabilities(&self) -> Vec<Capability>;
    fn schema(&self) -> Schema;
    
    async fn query(&self, q: &str) -> Result<QueryResult>;
    async fn subscribe(&self, q: &str) -> Result<Subscription>;
    
    // Transform to universal format
    fn to_graph(&self, result: &QueryResult) -> GraphData;
    fn to_table(&self, result: &QueryResult) -> TableData;
    fn to_geojson(&self, result: &QueryResult) -> GeoJsonData;
}

// Implementations
pub struct SurrealAdapter { /* ... */ }
pub struct PostgresAdapter { /* ... */ }
pub struct Neo4jAdapter { /* ... */ }
pub struct GeoJsonAdapter { /* ... */ }
pub struct SlotGraphAdapter { /* ... */ }
```

## Current Database Inventory

| Database | Port | Type | Status | Capabilities |
|----------|------|------|--------|--------------|
| SurrealDB Main | 8000 | Graph/Doc | ✅ Running | graph, sql, live |
| GLAF Core | 18019 | Graph/Doc | ✅ Running | graph, sql, live |
| GLAF Analytics | 18025 | Graph/Doc | ✅ Running | graph, sql |
| PostgreSQL | 5432 | Relational | ✅ Running | sql |
| SlotGraph ECS | 9001 | Graph/ECS | ✅ Running | graph, ecs |
| Sledis Cache | 6380 | KV/Cache | ✅ Running | kv |
| Neo4j (viz) | 7687 | Graph | ⚠️ Stopped | cypher, graph |
| GeoJSON Files | - | Static | ✅ Available | geojson |

## Next Steps

1. Create `ctas7-cdn-data-fabric` crate (CDN hub)
2. Create `mcp-glaf-visualizer` (MCP server)
3. Create `ctas7-glaf-visualizer-api` (REST API)
4. Fork Neo4j Browser UI to connect to CDN
5. Add database adapters for each DB type


