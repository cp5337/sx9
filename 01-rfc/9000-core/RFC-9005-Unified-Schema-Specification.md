# RFC-9005: Unified Schema Specification

**Status:** Final
**Version:** 1.0.0
**Dependencies:** RFC-9000, RFC-9001
**Author:** SX9 Engineering
**Date:** 2025-12-25

---

## Abstract

This RFC defines the unified schema specification for the SX9 platform, establishing canonical data structures, database layer protocols, and the five foundation crate pillars. It specifies Sledis as the Redis-protocol-compatible embedded cache layer.

---

## 1. Foundation Crate Architecture (5 Pillars)

The SX9 foundation is built on five canonical crates that provide all core dependencies:

```
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 FOUNDATION PILLARS                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │    CORE      │  │    DATA      │  │  INTERFACE   │          │
│  │  (sx9-fnd-   │  │  (sx9-fnd-   │  │  (sx9-fnd-   │          │
│  │    core)     │  │    data)     │  │  interface)  │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│        │                  │                  │                  │
│  ┌──────────────┐  ┌──────────────┐                            │
│  │    MATH      │  │   DAEMON     │                            │
│  │  (sx9-fnd-   │  │  (sx9-fnd-   │                            │
│  │    math)     │  │   daemon)    │                            │
│  └──────────────┘  └──────────────┘                            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.1 sx9-foundation-core

**Purpose:** Core types, hashing, async runtime, serialization, diagnostics

**Re-exports:**
```rust
pub mod async_runtime {
    pub use tokio;
    pub use async_trait::async_trait;
    pub use futures;
}

pub mod data {
    pub use serde::{Serialize, Deserialize};
    pub use serde_json;
    pub use chrono::{DateTime, Utc};
    pub use uuid::Uuid;
}

pub mod diagnostics {
    pub use anyhow::{Result, Context};
    pub use thiserror::Error;
    pub use tracing::{info, warn, error, debug};
}

pub mod networking {
    pub use reqwest;
    pub use axum;
}
```

### 1.2 sx9-foundation-data

**Purpose:** Persistence, caching, data processing

**Components:**
- **Sledis:** Redis-protocol-compatible embedded cache (§2)
- **LanceDB:** Vector store for embeddings (§3)
- Serialization (JSON, YAML, TOML, CSV)
- Data validation

**Re-exports:**
```rust
#[cfg(feature = "embedded-db")]
pub use sled;

#[cfg(feature = "sledis")]
pub mod sledis;

#[cfg(feature = "vector-store")]
pub mod vector_store;
```

### 1.3 sx9-foundation-interface

**Purpose:** CLI, HTTP clients, WebSocket

**Components:**
- CLI argument parsing (clap)
- HTTP client (reqwest)
- WebSocket (tungstenite)
- URL parsing

### 1.4 sx9-foundation-math

**Purpose:** Mathematical computation, symbolic math

**Components:**
- Linear algebra (nalgebra)
- Statistics (statrs)
- Financial math
- Orbital mechanics

### 1.5 sx9-foundation-daemon

**Purpose:** Service orchestration, communications, networking

**Components:**
- HTTP server (axum, warp)
- gRPC (tonic)
- Container orchestration (bollard)
- Service discovery (mDNS)
- Process management

---

## 2. Trivariate Hash Schema (RFC-9001)

### 2.1 48-Position Structure

```
┌────────────────────────────────────────────────────────────────┐
│              TRIVARIATE HASH (48 Characters)                    │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Position 1-16     Position 17-32    Position 33-48            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │     SCH      │  │     CUID     │  │     UUID     │          │
│  │ (Semantic    │  │ (Contextual  │  │  (Unique     │          │
│  │  Core Hash)  │  │  Unique ID)  │  │ Identifier)  │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                 │
│  Hash Function: Murmur3 (64-bit)                               │
│                                                                 │
└────────────────────────────────────────────────────────────────┘
```

### 2.2 Components

| Component | Positions | Purpose |
|-----------|-----------|---------|
| **SCH**   | 1-16      | Semantic Core Hash - visual properties, primary color, gradient |
| **CUID**  | 17-32     | Contextual Unique ID - animation properties, timing |
| **UUID**  | 33-48     | Unique Identifier - session/entity identity |

### 2.3 Hash Schema Definition

```rust
/// Trivariate hash system (RFC-9001)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrivariateHash {
    /// Semantic Core Hash (16 positions)
    pub sch: String,
    /// Contextual Unique ID (16 positions)
    pub cuid: String,
    /// Unique Identifier (16 positions)
    pub uuid: String,
}

impl TrivariateHash {
    /// Generate canonical 48-char format
    pub fn to_canonical(&self) -> String {
        format!("{}{}{}", self.sch, self.cuid, self.uuid)
    }
}
```

### 2.4 Unicode Tail (U+E000-E9FF)

Trivariate hashes can have Unicode tails for routing (RFC-9002):

```rust
/// Unicode operation ranges
pub mod unicode_ranges {
    pub const THREAT_INTEL: (u32, u32) = (0xE800, 0xE8FF);  // Tools
    pub const CTAS_TASKS: (u32, u32) = (0xE000, 0xE0FF);    // Tasks
    pub const PTCC_CONFIGS: (u32, u32) = (0xE300, 0xE3FF);  // PTCC
    pub const TOOL_CHAINS: (u32, u32) = (0xE400, 0xE6FF);   // Chains
}
```

---

## 3. Sledis: Redis-Protocol Cache Layer

Sledis provides Redis protocol compatibility over the Sled embedded database.

### 3.1 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         SLEDIS                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   Redis Protocol Parser                                          │
│         │                                                        │
│         ▼                                                        │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Command Handler                             │   │
│   │  GET, SET, DEL, EXPIRE, HGET, HSET, LPUSH, RPUSH, etc. │   │
│   └─────────────────────────────────────────────────────────┘   │
│         │                                                        │
│         ▼                                                        │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Sled Engine                             │   │
│   │           (Embedded B+ Tree KVS)                         │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 Port Allocation

| Service | Port  | Protocol |
|---------|-------|----------|
| Sled    | 18400 | Native   |
| Sledis  | 18401 | RESP     |

### 3.3 Supported Commands

**String Operations:**
- `GET key`
- `SET key value [EX seconds]`
- `DEL key [key ...]`
- `EXPIRE key seconds`
- `TTL key`
- `EXISTS key`

**Hash Operations:**
- `HGET key field`
- `HSET key field value`
- `HDEL key field`
- `HGETALL key`

**List Operations:**
- `LPUSH key value`
- `RPUSH key value`
- `LPOP key`
- `RPOP key`
- `LRANGE key start stop`

**Set Operations:**
- `SADD key member`
- `SREM key member`
- `SMEMBERS key`
- `SISMEMBER key member`

### 3.4 Schema Definition

```rust
/// Sledis value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SledisValue {
    String(String),
    Integer(i64),
    Float(f64),
    Hash(HashMap<String, String>),
    List(Vec<String>),
    Set(HashSet<String>),
}

/// Sledis entry with TTL support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SledisEntry {
    pub value: SledisValue,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub trivariate_hash: Option<String>,
}
```

---

## 3. Vector Store Schema (LanceDB)

### 3.1 Collections

| Collection       | Purpose                          | Embedding Dim |
|------------------|----------------------------------|---------------|
| `tools`          | Threat intel tools               | 384           |
| `tasks`          | Operational tasks                | 384           |
| `ptcc_configs`   | PTCC configurations              | 384           |
| `tool_chains`    | Tool execution chains            | 384           |
| `threat_content` | Vectorized threat intel          | 384           |

### 3.2 Document Schema

```rust
/// Vector document for LanceDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub text: String,
    pub embedding: Vec<f32>,  // 384 dimensions
    pub metadata: HashMap<String, Value>,
    pub unicode_ops: Option<String>,  // U+E000-E9FF
    pub trivariate_hash: Option<String>,
}

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorQueryResult {
    pub id: String,
    pub document: String,
    pub metadata: HashMap<String, Value>,
    pub distance: f32,
    pub score: f32,  // 1.0 - distance
}
```

---

## 4. Database Layer Schema

### 4.1 Database Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Database {
    /// Supabase (PostgreSQL + realtime)
    Supabase,
    /// Neon (serverless PostgreSQL)
    Neon,
    /// Sled (embedded key-value)
    Sled,
    /// Sledis (Redis-compatible layer over Sled)
    Sledis,
    /// NATS (messaging/pubsub)
    Nats,
    /// LanceDB (vector store)
    LanceDb,
}
```

### 4.2 Port Assignments

| Service   | Port  | Protocol    |
|-----------|-------|-------------|
| Supabase  | 18000 | PostgreSQL  |
| Neon      | 18015 | PostgreSQL  |
| Sled      | 18400 | Native      |
| Sledis    | 18401 | RESP        |
| LanceDB   | 18402 | Native      |
| NATS      | 18020 | NATS        |

---

## 5. Three-Layer ECS Architecture

### 5.1 Layer Stack

```
┌─────────────────────────────────────────────────────────────────┐
│                    PLASMA-ECS LAYERS                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  LAYER 3: ATLAS (Cognitive)                                     │
│  ═══════════════════════════                                    │
│  • ATLAS Daemon (1ms OODA loop)                                │
│  • sx9-atlas-bus (ring buffer, PlasmaState)                    │
│  • Crystal resonance, SDT gate control                         │
│  • Priority routing (critical/urgent/normal)                    │
│  • NATS bridge for distributed ops                             │
│                                                                  │
│  LAYER 2: Legion (Deterministic Batch)                          │
│  ═══════════════════════════════════════                        │
│  • High-performance batch processing                           │
│  • Deterministic tick-based world state                        │
│  • Hot-path operations (<1μs latency)                          │
│  • Entity-component queries                                    │
│  • SlotGraph integration (hash → archetype, zero lookup)       │
│  • Pure integers only (no strings in hot-path)                 │
│                                                                  │
│  LAYER 1: apecs (Async I/O)                                     │
│  ═══════════════════════════                                    │
│  • Async-friendly operations                                   │
│  • WASM-compatible                                             │
│  • I/O-bound tasks (network, database)                         │
│  • Mission entity creation                                     │
│  • JSON/TOML parsing (cold-path only)                          │
│  • Database queries (Supabase, Neon, Sled, Sledis)             │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 SlotGraph Hash → Archetype Mapping

SlotGraph provides zero-lookup routing from trivariate hash to Legion archetype:

```rust
pub struct SlotGraphIntegration {
    /// Hash → (slot_id, archetype_id) mapping
    hash_to_archetype: HashMap<String, (String, u64)>,
    legion_world: World,
}
```

**Performance:** O(1) lookup, < 100ns.

### 5.3 Hot-Path Entity Schema

```rust
/// Hot-path entity (pure integers, no strings)
#[derive(Debug, Clone, Copy)]
pub struct HotPathEntity {
    pub entity_id: u64,           // Archetype ID (direct pointer)
    pub unicode_trigger: u32,     // Unicode operation (U+E000-E9FF)
    pub primitive_bitfield: u64,  // PTCC primitive bitfield
    pub speed_class: u8,          // Speed class
    pub slot_id: u64,             // SlotGraph slot ID
}
```

### 5.4 Complete Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. NATS Event: sx9.threat.honeypot                            │
│     └─> HoneypotEvent { entity_id, threat_hash, ... }          │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. apecs (Layer 1: Async I/O)                                 │
│     └─> Create MissionEntity                                   │
│         • Parse JSON/TOML (cold-path)                          │
│         • Extract: threat_hash, unicode_trigger,                │
│           primitive_bitfield, speed_class                       │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. ATLAS Daemon (Layer 3: Cognitive)                          │
│     └─> Process Mission (OODA Loop)                            │
│         • Observe: Extract mission context                     │
│         • Orient: Calculate convergence, delta angle            │
│         • Decide: Crystal resonance check                       │
│         • Act: Return AtlasOutcome                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. Sledis (Hot-Path Cache)                                    │
│     └─> Store: hash → (unicode + bitfield + speed_class)       │
│         • Target: < 3μs lookup                                 │
│         • Key: threat_hash (trivariate)                         │
│         • Value: { unicode_trigger, primitive_bitfield }        │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. SlotGraph (Hash → Archetype Mapping)                        │
│     └─> Get Archetype: hash → (slot_id, archetype_id)          │
│         • Zero lookup (direct pointer)                          │
│         • O(1) hash map lookup                                 │
│         • Creates new archetype if not found                    │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  6. Legion (Layer 2: Hot-Path)                                 │
│     └─> Insert Entity: HotPathEntity                            │
│         • Pure integers only (no strings)                       │
│         • Components: entity_id, unicode_trigger,               │
│           primitive_bitfield, speed_class, slot_id              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  7. NATS Output: sx9.hotpath.load                              │
│     └─> HotpathLoadEvent { entity_id, hash, slot_id, ... }     │
└─────────────────────────────────────────────────────────────────┘
```

### 5.5 Performance Targets

| Stage | Target | Measurement |
|-------|--------|-------------|
| **Total pipeline** | < 9.2μs | From NATS event to Legion insertion |
| **apecs (mission entity)** | < 1μs | JSON parsing, entity creation |
| **ATLAS (OODA loop)** | < 1ms | Zone B compliance |
| **Sledis (lookup)** | < 3μs | Hash → data lookup |
| **SlotGraph (archetype)** | < 100ns | Hash → archetype lookup |
| **Legion (insertion)** | < 1μs | Entity insertion |

---

## 7. Message Schema

### 7.1 Trivariate-Tagged Messages

All messages in SX9 can carry trivariate hash metadata:

```rust
/// Base message with trivariate hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedMessage<T> {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub trivariate_hash: String,  // 48-char hash
    pub unicode_route: Option<char>,  // U+E000-E9FF
    pub payload: T,
}
```

### 7.2 NATS Subject Schema

```
sx9.{domain}.{action}.{resource}

Examples:
  sx9.cache.set.session
  sx9.vector.query.tools
  sx9.task.execute.hunt
```

---

## 6. Implementation Requirements

### 6.1 Crate Dependencies

All SX9 crates MUST use foundation crates for dependencies:

```toml
[dependencies]
# Use foundation crates - DO NOT add deps directly
sx9-foundation-core = { path = "../sx9-foundation-core" }
sx9-foundation-data = { path = "../sx9-foundation-data", features = ["sledis"] }
sx9-foundation-daemon = { path = "../sx9-foundation-daemon" }
```

### 6.2 Naming Convention

- **Crate names:** `sx9-{domain}` (NOT ctas7)
- **Module names:** snake_case
- **Types:** PascalCase
- **Constants:** SCREAMING_SNAKE_CASE

### 6.3 Error Handling

All errors MUST use thiserror with trivariate hash context:

```rust
#[derive(Debug, thiserror::Error)]
pub enum SledisError {
    #[error("Key not found: {key} (hash: {hash})")]
    KeyNotFound { key: String, hash: String },

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Storage error: {0}")]
    Storage(#[from] sled::Error),
}
```

---

## 7. Migration from CTAS7

All CTAS7 naming MUST be replaced with SX9:

| Old | New |
|-----|-----|
| `ctas7-*` | `sx9-*` |
| `CTAS7` | `SX9` |
| `ctas_*` | `sx9_*` |
| `Ctas*` | `Sx9*` |

---

## References

- RFC-9000: Agnostic Core & Ontology
- RFC-9001: Trivariate Hashing Standard
- RFC-9004: Deterministic Routing
- Redis Protocol Specification (RESP)
- Sled Documentation
- LanceDB Documentation

---

**Document Status:** Final
**Effective Date:** 2025-12-25
