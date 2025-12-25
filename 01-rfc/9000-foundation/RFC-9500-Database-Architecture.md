# RFC-9500: Database Architecture

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

Multi-database architecture with specialized stores for different data patterns.

### 1.1 Database Stack

| Database | Purpose | Data Type |
|----------|---------|-----------|
| **Supabase** | Persistence, auth, real-time | Structured entities |
| **SurrealDB** | Graph relationships, 165-node detector | Threat graphs |
| **Sled** | Embedded key-value, hot cache | Fast lookups |
| **SlotGraph** | Legion ECS spatial coordination | Entity positions |
| **Sledis** | Redis-compatible in-process | Session state |

> **Critical:** Use **Sledis** NOT Redis for in-process cache.

---

## 2. Supabase (PostgreSQL)

### 2.1 Role

- Primary persistence layer
- Authentication & authorization
- Real-time subscriptions
- Structured entity storage

### 2.2 Schema Pattern

```sql
-- Trivariate entities
CREATE TABLE entities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    trivariate TEXT NOT NULL UNIQUE,
    sch TEXT NOT NULL,
    cuid TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_entities_sch ON entities(sch);
CREATE INDEX idx_entities_type ON entities(entity_type);

-- Delta angles with 6-decimal precision
CREATE TABLE delta_positions (
    entity_id UUID REFERENCES entities(id),
    x NUMERIC(7,6) NOT NULL,  -- 0.000000 - 1.000000
    y NUMERIC(7,6) NOT NULL,
    z NUMERIC(7,6) NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (entity_id, timestamp)
);
```

---

## 3. SurrealDB (Graph + Document)

### 3.1 Role

- Graph relationships and traversal
- 165-node threat detector
- Multi-model queries
- Real-time graph updates

### 3.2 Schema Pattern

```surql
-- Define threat actor
DEFINE TABLE threat_actor SCHEMAFULL;
DEFINE FIELD name ON threat_actor TYPE string;
DEFINE FIELD trivariate ON threat_actor TYPE string;
DEFINE FIELD realm ON threat_actor TYPE string;
DEFINE FIELD confidence ON threat_actor TYPE float;

-- Define relationship
DEFINE TABLE targets SCHEMAFULL;
DEFINE FIELD created_at ON targets TYPE datetime;
DEFINE FIELD delta_angle ON targets TYPE object;

-- Graph query example
SELECT *, ->targets->* FROM threat_actor 
WHERE realm = 'CYBER' 
AND confidence > 0.7;
```

### 3.3 Ground Station Schema

```surql
-- 257 LaserLight FSO ground stations
DEFINE TABLE ground_station SCHEMAFULL;
DEFINE FIELD station_id ON ground_station TYPE string;
DEFINE FIELD lat ON ground_station TYPE float;
DEFINE FIELD lon ON ground_station TYPE float;
DEFINE FIELD wasm_microkernel_deployed ON ground_station TYPE bool;
DEFINE FIELD collection_capabilities ON ground_station TYPE array;
DEFINE FIELD last_heartbeat ON ground_station TYPE datetime;
```

---

## 4. Sled (Embedded Key-Value)

### 4.1 Role

- Hot cache for Legion ECS
- Sub-microsecond reads
- Embedded (no network hop)
- Persistent to disk

### 4.2 Usage Pattern

```rust
use sled::Db;

pub struct SledCache {
    db: Db,
}

impl SledCache {
    pub fn new(path: &str) -> Self {
        Self {
            db: sled::open(path).unwrap(),
        }
    }
    
    pub fn get_trivariate(&self, sch: &str) -> Option<Vec<u8>> {
        self.db.get(sch.as_bytes()).ok().flatten().map(|v| v.to_vec())
    }
    
    pub fn set_trivariate(&self, sch: &str, data: &[u8]) -> sled::Result<()> {
        self.db.insert(sch.as_bytes(), data)?;
        Ok(())
    }
}
```

---

## 5. SlotGraph (Legion ECS Spatial)

### 5.1 Role

- Spatial coordination for Legion ECS entities
- 3D position indexing
- Neighbor queries
- Realm-specific coordinate systems

### 5.2 Structure

```rust
pub struct SlotGraph {
    pub slots: HashMap<SlotKey, Entity>,
    pub spatial_index: RTree<SlotEntry>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SlotKey {
    pub realm: Realm,
    pub x: i32,  // Fixed-point position
    pub y: i32,
    pub z: i32,
}

impl SlotGraph {
    pub fn insert(&mut self, entity: Entity, position: &DeltaPosition, realm: Realm) {
        let key = SlotKey {
            realm,
            x: (position.x * 1_000_000.0) as i32,
            y: (position.y * 1_000_000.0) as i32,
            z: (position.z * 1_000_000.0) as i32,
        };
        self.slots.insert(key.clone(), entity);
        self.spatial_index.insert(SlotEntry { key, entity });
    }
    
    pub fn neighbors(&self, position: &DeltaPosition, radius: f64, realm: Realm) -> Vec<Entity> {
        // R-tree range query
    }
}
```

---

## 6. Sledis (In-Process Redis-Compatible)

### 6.1 Role

- Redis-compatible API
- In-process (no network)
- Session state
- Pub/sub for local events

### 6.2 Why NOT Redis

| Aspect | Redis | Sledis |
|--------|-------|--------|
| Network | Required | None |
| Latency | ~100µs+ | <1µs |
| Deployment | Separate process | Embedded |
| Memory | Shared with Redis | Process-local |

### 6.3 Usage

```rust
use sledis::Sledis;

pub struct SessionStore {
    db: Sledis,
}

impl SessionStore {
    pub fn set_session(&self, key: &str, value: &str, ttl_secs: u64) {
        self.db.setex(key, value, ttl_secs);
    }
    
    pub fn get_session(&self, key: &str) -> Option<String> {
        self.db.get(key)
    }
}
```

---

## 7. Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          DATA FLOW                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  HOT PATH (<1µs)                                                       │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐                         │
│  │  Legion  │◄──►│   Sled   │◄──►│ SlotGraph│                         │
│  │   ECS    │    │  Cache   │    │ (Spatial)│                         │
│  └────┬─────┘    └──────────┘    └──────────┘                         │
│       │                                                                 │
│       │ Async write-behind                                             │
│       ▼                                                                 │
│  WARM PATH (1ms-100ms)                                                 │
│  ┌──────────┐    ┌──────────┐                                         │
│  │ Supabase │◄──►│ SurrealDB│                                         │
│  │ (Entity) │    │ (Graph)  │                                         │
│  └──────────┘    └──────────┘                                         │
│                                                                         │
│  SESSION (in-process)                                                  │
│  ┌──────────┐                                                          │
│  │  Sledis  │  ← Redis-compatible, NO network                          │
│  └──────────┘                                                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 8. Connection Configuration

```toml
# database.toml

[supabase]
url = "https://xxx.supabase.co"
anon_key = "..."
service_key = "..."  # Server-side only

[surrealdb]
url = "ws://localhost:8000"
namespace = "sx9"
database = "threat_intel"
username = "root"
password = "..."

[sled]
path = "/var/lib/sx9/sled"
cache_capacity = 1073741824  # 1GB

[sledis]
max_memory = 268435456  # 256MB
eviction_policy = "lru"
```

---

## 9. 165-Node Graph Detector

```surql
-- SurrealDB schema for threat detection graph

-- Node types
DEFINE TABLE indicator SCHEMAFULL;
DEFINE TABLE tactic SCHEMAFULL;
DEFINE TABLE technique SCHEMAFULL;
DEFINE TABLE procedure SCHEMAFULL;
DEFINE TABLE actor SCHEMAFULL;
DEFINE TABLE campaign SCHEMAFULL;

-- Relationship types (edges)
DEFINE TABLE uses SCHEMAFULL;
DEFINE TABLE targets SCHEMAFULL;
DEFINE TABLE attributed_to SCHEMAFULL;
DEFINE TABLE indicates SCHEMAFULL;

-- Detection query (traverses up to 165 nodes)
LET $start = (SELECT * FROM indicator WHERE ioc = $ioc);
SELECT 
    *,
    ->uses->technique.*,
    ->uses->technique->attributed_to->actor.*,
    <-indicates<-campaign.*
FROM $start
LIMIT 165;
```

---

## Critical Constraints

- **Supabase** for persistence and auth
- **SurrealDB** for graph queries
- **Sled** for embedded hot cache
- **SlotGraph** for Legion ECS spatial
- **Sledis NOT Redis** - in-process only
- **Legion ECS** - NOT alternative ECS frameworks
- **6-decimal precision** for all delta angles

---

## References

- RFC-9020: Trivariate Hashing
- RFC-9024/9025: Dual H1/H2 Architecture
- RFC-9302: Nonagon Analytic Node
