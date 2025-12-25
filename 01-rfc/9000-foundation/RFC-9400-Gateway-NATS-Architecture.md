# RFC-9400: Gateway & NATS Architecture

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

Gateway serves as the NATS facade, providing protocol translation from external (HTTP/WS/gRPC) to internal NATS messaging.

### 1.1 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                      EXTERNAL                                   │
│              HTTP/WS/gRPC (:18120)                              │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 GATEWAY                                  │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   REST      │  │  WebSocket  │  │    gRPC     │             │
│  │   API       │  │   (live)    │  │  (internal) │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          │                                      │
│                   ┌──────▼──────┐                               │
│                   │  NATS Client │  ← Embedded                  │
│                   │  + JetStream │                              │
│                   └──────┬──────┘                               │
│                          │                                      │
└──────────────────────────┼──────────────────────────────────────┘
                           │
            ┌──────────────┼──────────────┐
            │              │              │
            ▼              ▼              ▼
     ┌──────────┐   ┌──────────┐   ┌──────────┐
     │  ATLAS   │   │  Kali    │   │  Hash    │
     │  Daemon  │   │  Daemon  │   │  Engine  │
     └──────────┘   └──────────┘   └──────────┘
            │              │              │
            └──────────────┼──────────────┘
                           │
                    NATS (:4222)
                    Internal only
```

---

## 2. Port Assignments

| Service | Port | NATS Subject |
|---------|------|--------------|
| ATLAS Daemon | 18106 | `sx9.atlas.*`, `sx9.tick.*` |
| Neural Mux | 18107 | `sx9.mux.*` |
| Hashing Engine | 18105 | `sx9.hash.*` |
| Kali Daemon | 18200 | `sx9.kali.*` |
| **Gateway** | **18120** | `sx9.gateway.*` |
| NATS Server | 4222 | Internal |
| NATS Monitor | 8222 | Internal |

---

## 3. NATS Subject Hierarchy

```
sx9.
├── tick.                    # Cognitive tick (ATLAS heartbeat)
│   ├── 1ms                  # 1ms tick
│   └── sync                 # Tick synchronization
│
├── atlas.                   # ATLAS daemon
│   ├── ooda.>               # OODA loop phases
│   ├── state.>              # State updates
│   └── command.>            # Commands
│
├── bus.                     # AtlasBus IPC
│   ├── critical.>           # Critical path (SDT, etc.)
│   └── standard.>           # Standard messages
│
├── plasma.                  # Plasma ECS
│   ├── entity.>             # Entity lifecycle
│   ├── component.>          # Component updates
│   └── field.>              # Field state
│
├── hash.                    # Hashing engine
│   ├── compute.>            # Hash computation requests
│   ├── result.>             # Hash results
│   └── verify.>             # Verification requests
│
├── mux.                     # Neural Mux
│   ├── route.>              # Routing decisions
│   ├── affinity.>           # Affinity updates
│   └── stats.>              # Routing statistics
│
├── kali.                    # Kali execution
│   ├── exec.>               # Execution requests
│   ├── result.>             # Execution results
│   ├── chain.>              # Tool chain orchestration
│   └── telemetry.>          # Execution telemetry
│
├── crate.                   # Smart crates
│   ├── spawn.>              # Crate spawning
│   ├── health.>             # Crate health
│   └── retire.>             # Crate retirement
│
├── cdn.                     # CDN operations
│   ├── store.>              # Store requests
│   ├── retrieve.>           # Retrieve requests
│   └── replicate.>          # Replication events
│
├── iac.                     # Infrastructure as Code
│   ├── trigger.>            # Manifold triggers
│   ├── spawn.>              # Spawn events
│   └── teardown.>           # Teardown events
│
├── health.                  # System health
│   ├── heartbeat.>          # Service heartbeats
│   ├── metrics.>            # Metrics export
│   └── alert.>              # Alerts
│
├── gateway.                 # Gateway
│   ├── request.>            # Inbound requests
│   ├── response.>           # Outbound responses
│   └── session.>            # Session management
│
└── telemetry.               # System-wide telemetry
    ├── trace.>              # Distributed traces
    ├── span.>               # Trace spans
    └── event.>              # Audit events
```

---

## 4. Core NATS vs JetStream Decision Matrix

| Subject Pattern | Transport | Rationale |
|-----------------|-----------|-----------|
| `sx9.tick.*` | **Core NATS** | Real-time, stale ticks useless |
| `sx9.atlas.ooda.*` | **Core NATS** | Real-time cognitive state |
| `sx9.bus.*` | **Core NATS** | HFT, AtlasBus is already durable |
| `sx9.plasma.*` | **Core NATS** | ECS updates, high frequency |
| `sx9.mux.route.*` | **Core NATS** | Sub-microsecond routing |
| `sx9.health.heartbeat.*` | **Core NATS** | Real-time health |
| | | |
| `sx9.kali.exec.*` | **JetStream** | Durable execution queue |
| `sx9.kali.result.*` | **JetStream** | Results must persist |
| `sx9.kali.chain.*` | **JetStream** | Chain orchestration needs replay |
| `sx9.hash.compute.*` | **JetStream** | Hash requests must not be lost |
| `sx9.crate.spawn.*` | **JetStream** | Spawn requests are imperative |
| `sx9.cdn.*` | **JetStream** | Content operations need durability |
| `sx9.iac.*` | **JetStream** | Infrastructure changes must persist |
| `sx9.telemetry.*` | **JetStream** | Audit trail |
| `sx9.gateway.request.*` | **JetStream** | Request durability |

---

## 5. JetStream Streams Configuration

```yaml
# sx9-streams.yaml

streams:
  - name: KALI
    subjects:
      - "sx9.kali.>"
    retention: limits
    max_msgs: 100000
    max_bytes: 1GB
    max_age: 24h
    max_msg_size: 1MB
    storage: file
    replicas: 1
    discard: old

  - name: HASH
    subjects:
      - "sx9.hash.>"
    retention: limits
    max_msgs: 1000000
    max_bytes: 500MB
    max_age: 1h
    max_msg_size: 64KB
    storage: file
    replicas: 1
    discard: old

  - name: CRATE
    subjects:
      - "sx9.crate.>"
    retention: limits
    max_msgs: 10000
    max_bytes: 100MB
    max_age: 7d
    max_msg_size: 256KB
    storage: file
    replicas: 1
    discard: old

  - name: CDN
    subjects:
      - "sx9.cdn.>"
    retention: limits
    max_msgs: 50000
    max_bytes: 500MB
    max_age: 24h
    max_msg_size: 10MB
    storage: file
    replicas: 1
    discard: old

  - name: TELEMETRY
    subjects:
      - "sx9.telemetry.>"
    retention: limits
    max_msgs: 1000000
    max_bytes: 5GB
    max_age: 30d
    max_msg_size: 64KB
    storage: file
    replicas: 1
    discard: old

  - name: GATEWAY
    subjects:
      - "sx9.gateway.>"
    retention: limits
    max_msgs: 100000
    max_bytes: 1GB
    max_age: 1h
    max_msg_size: 1MB
    storage: file
    replicas: 1
    discard: old
```

---

## 6. Gateway Routes → NATS Translation

```rust
// POST /api/kali/exec/nmap
//   → publish sx9.kali.exec.nmap
//   → return correlation_id

// GET /api/kali/result/{corr_id}
//   → request sx9.kali.result.{corr_id}
//   → return result JSON

// WS /api/stream/health
//   → subscribe sx9.health.heartbeat.>
//   → stream events to client
```

---

## 7. Message Format

```rust
#[derive(Serialize, Deserialize)]
pub struct NatsMessage {
    pub header: MessageHeader,
    pub payload: Value,  // Domain-specific JSON
}

#[derive(Serialize, Deserialize)]
pub struct MessageHeader {
    pub id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub timestamp_us: u64,
    pub source: String,
    pub subject: String,
    pub trivariate: Option<String>,
}
```

---

## 8. NATS Subject Constants (Rust)

```rust
// nats/subjects.rs

pub mod tick {
    pub const MS1: &str = "sx9.tick.1ms";
    pub const SYNC: &str = "sx9.tick.sync";
}

pub mod atlas {
    pub const OODA_OBSERVE: &str = "sx9.atlas.ooda.observe";
    pub const OODA_ORIENT: &str = "sx9.atlas.ooda.orient";
    pub const OODA_DECIDE: &str = "sx9.atlas.ooda.decide";
    pub const OODA_ACT: &str = "sx9.atlas.ooda.act";
}

pub mod kali {
    pub const EXEC: &str = "sx9.kali.exec.>";
    pub const RESULT: &str = "sx9.kali.result.>";
    pub const CHAIN: &str = "sx9.kali.chain.>";
    
    pub fn exec_for(tool: &str) -> String {
        format!("sx9.kali.exec.{}", tool)
    }
}

pub mod health {
    pub const HEARTBEAT: &str = "sx9.health.heartbeat.>";
    
    pub fn heartbeat_for(service: &str) -> String {
        format!("sx9.health.heartbeat.{}", service)
    }
}

/// Check if subject uses JetStream
pub fn is_jetstream_subject(subject: &str) -> bool {
    subject.starts_with("sx9.kali.")
        || subject.starts_with("sx9.hash.")
        || subject.starts_with("sx9.crate.")
        || subject.starts_with("sx9.cdn.")
        || subject.starts_with("sx9.iac.")
        || subject.starts_with("sx9.gateway.")
        || subject.starts_with("sx9.telemetry.")
}
```

---

## 9. Gateway Benefits

| Aspect | Standalone NATS | Gateway-Embedded |
|--------|-----------------|------------------|
| External exposure | NATS port exposed | Only Gateway port |
| Auth | NATS auth + Gateway auth | Single auth layer |
| Protocol translation | Client needs NATS | REST/WS/gRPC → NATS |
| Complexity | Two systems | One entry point |
| WebSocket streaming | Separate | Built-in |

---

## 10. Smart Crate NATS Configuration

```toml
# smart-crate.toml NATS section

[nats]
enabled = true
url = "nats://localhost:4222"
name = "sx9-gateway"

[nats.subjects]
publishes = [
    "sx9.kali.exec.>",
    "sx9.hash.compute.>",
    "sx9.crate.spawn.>",
    "sx9.gateway.request.>",
]
subscribes = [
    "sx9.kali.result.>",
    "sx9.hash.result.>",
    "sx9.gateway.response.>",
    "sx9.health.heartbeat.>",
]

[nats.jetstream]
enabled = true
streams = ["KALI", "HASH", "GATEWAY", "TELEMETRY"]
```

---

## Critical Constraints

- **Gateway is NATS facade** - Single external entry point
- **Core NATS for real-time** - ~50µs latency
- **JetStream for durable** - ~200µs latency, persistence
- **Subject naming:** `sx9.{domain}.{action}.{qualifier}`
- **NATS internal only** - Port 4222 not exposed externally

---

## References

- RFC-9024/9025: Dual H1/H2 Architecture
- RFC-9050: QA Two-Heartbeat System
- RFC-9500: Database Architecture
