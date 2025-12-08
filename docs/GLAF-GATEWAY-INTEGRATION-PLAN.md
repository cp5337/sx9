# GLAF Gateway Integration Plan
## Integrating Existing GLAF System into SX9 Gateway

**Date:** December 2025  
**Status:** Implementation Plan  
**Goal:** Integrate existing GLAF system as `sx9-glaf-core` for gateway topology mirroring

---

## Executive Summary

**Existing GLAF Components:**
- ✅ `ctas7-slotgraph-engine` - Core SlotGraph engine (Legion ECS-based)
- ✅ `ctas7-glaf-graph-server` - GLAF graph server (Rust/Axum backend)
- ✅ `ctas7-glaf-clients` - Client libraries (TETH/SurrealDB, ChromaDB, Neo4j)
- ✅ `CTAS7-GLAF-SYSTEM/` - Complete system with specs, workflows, integration

**Gateway Requirement (RFC-9114 Rev 1.1):**
- GLAF mirrors **routing topology** (hash/unicode routes), NOT data traversal
- GLAF observes SlotGraph state for topological feedback
- GLAF remains disabled by default until post-deployment verification

**Strategy:**
- Create `sx9-glaf-core` as a **thin wrapper/integration crate**
- Reuse existing `ctas7-glaf-graph-server` and `ctas7-slotgraph-engine`
- Focus on **topology mirroring** (routing state), not full graph analysis
- Keep existing GLAF system intact for other uses

---

## 1. Existing GLAF System Architecture

### 1.1 Component Structure

```
CTAS7-GLAF-SYSTEM/
├── ctas7-slotgraph-engine/      # Core SlotGraph (Legion ECS)
│   ├── src/
│   │   ├── lib.rs               # SlotGraph core
│   │   ├── graph.rs              # Graph operations
│   │   ├── components.rs         # ECS components
│   │   └── systems.rs            # ECS systems
│   └── Cargo.toml
│
├── ctas7-glaf-graph-server/     # GLAF API server
│   ├── src/
│   │   └── main.rs              # Axum server
│   └── Cargo.toml
│
├── ctas7-glaf-clients/          # Client libraries
│   ├── src/
│   │   ├── lib.rs
│   │   ├── teth_client.rs       # SurrealDB client
│   │   ├── chroma_client.rs     # ChromaDB client
│   │   └── neo4j_client.rs      # Neo4j client
│   └── Cargo.toml
│
└── specs/
    └── GLAF-CORE-SPEC.md        # Complete specification
```

### 1.2 Current GLAF Capabilities

**From GLAF-CORE-SPEC.md:**
- ✅ SlotGraph in-memory graph (Legion ECS-based)
- ✅ Cypher++ query language with math extensions
- ✅ 68 APOC++ procedures (TETH, L*, HMM, Matroid, etc.)
- ✅ SurrealDB, Supabase, Sled, Sledis backends
- ✅ Real-time ingestion (PLASMA SSE, Google Sheets, TOML)
- ✅ Cognigraph repulsion physics
- ✅ Auto-graph from markdown

**Ports:**
- GLAF UI: 18018
- GLAF API: 18019
- GLAF Analytics: 18025
- VFG (Visual Fabric Generator): 18030
- AGR Listener: 18040

---

## 2. Gateway Integration Requirements

### 2.1 RFC-9114 Rev 1.1 Requirements

**From RFC-9114 Rev 1.1 Section 7.2:**

```toml
[glaf]
engine = "sx9-glaf-core"
enabled = false
mirror_slotgraph = true
sync_interval_ms = 100
topology_feedback = true
```

**Key Points:**
1. **Mirror routing topology** (hash/unicode routes), NOT data
2. **Observe SlotGraph state** for topological feedback
3. **Disabled by default** until post-deployment verification
4. **Topology feedback** for routing optimization

### 2.2 Gateway Use Case

**Gateway needs GLAF for:**
- ✅ Mirroring routing topology (which routes exist, their performance)
- ✅ Observing SlotGraph routing state (hash → route mappings)
- ✅ Topological feedback (route efficiency, congestion)
- ❌ NOT for data traversal or full graph analysis

**Distinction:**
- **GLAF-CORE-SPEC**: Full graph analysis system (nodes, edges, Cypher++ queries)
- **Gateway GLAF**: Topology mirroring only (routing state observation)

---

## 3. Integration Strategy

### 3.1 Create `sx9-glaf-core` Wrapper Crate

**Location:** `/Users/cp5337/Developer/synaptix9-workflow-system/crates/sx9-glaf-core/`

**Purpose:**
- Thin wrapper around existing GLAF components
- Focused on topology mirroring (not full graph analysis)
- Gateway-specific integration

**Structure:**
```
sx9-glaf-core/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs                    # Public API
    ├── topology_mirror.rs         # Topology mirroring logic
    ├── slotgraph_observer.rs      # SlotGraph state observation
    └── gateway_integration.rs     # Gateway-specific integration
```

### 3.2 Cargo.toml Dependencies

```toml
[package]
name = "sx9-glaf-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Existing GLAF components (reuse, don't duplicate)
ctas7-slotgraph-engine = { path = "../../ctas-7-shipyard-staging/ctas7-slotgraph-engine" }
ctas7-glaf-clients = { path = "../../ctas-7-shipyard-staging/ctas7-glaf-clients" }

# Gateway dependencies
sx9-atlas-bus = { path = "../sx9-atlas-bus" }
sx9-gateway = { path = "../sx9-gateway" }

# Foundation
ctas7-foundation-core = { path = "../../ctas-7-shipyard-staging/ctas7-foundation-core" }

# Async runtime
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tracing = "0.1"
```

### 3.3 Topology Mirroring Implementation

**File:** `src/topology_mirror.rs`

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use ctas7_slotgraph_engine::SlotGraph;
use ctas7_glaf_clients::teth_client::TethClient;

/// Topology mirror for gateway routing state
pub struct TopologyMirror {
    slotgraph: Arc<SlotGraph>,
    surrealdb: Arc<TethClient>,
    routing_topology: Arc<RwLock<RoutingTopology>>,
    enabled: Arc<tokio::sync::RwLock<bool>>,
}

#[derive(Clone, Debug)]
pub struct RoutingTopology {
    /// Hash → Route mappings (from SlotGraph)
    hash_routes: std::collections::HashMap<String, RouteInfo>,
    /// Unicode → Route mappings
    unicode_routes: std::collections::HashMap<char, RouteInfo>,
    /// Route performance metrics
    route_metrics: std::collections::HashMap<String, RouteMetrics>,
}

#[derive(Clone, Debug)]
pub struct RouteInfo {
    pub route_id: String,
    pub target: String,
    pub latency_ns: u64,
    pub success_rate: f64,
}

#[derive(Clone, Debug)]
pub struct RouteMetrics {
    pub request_count: u64,
    pub avg_latency_ns: u64,
    pub p99_latency_ns: u64,
    pub error_count: u64,
}

impl TopologyMirror {
    /// Mirror SlotGraph routing topology (not data)
    pub async fn mirror_topology(&self) -> Result<()> {
        if !*self.enabled.read().await {
            return Ok(()); // Disabled by default
        }

        // Query SlotGraph for routing topology (hash/unicode routes)
        let routing_state = self.slotgraph.get_routing_topology().await?;

        // Update mirror
        let mut topology = self.routing_topology.write().await;
        topology.hash_routes = routing_state.hash_routes;
        topology.unicode_routes = routing_state.unicode_routes;

        // Sync to SurrealDB for persistence (optional)
        if let Ok(client) = self.surrealdb.clone() {
            client.sync_topology(&topology).await?;
        }

        Ok(())
    }

    /// Get topology feedback for routing optimization
    pub async fn get_topology_feedback(&self) -> Result<TopologyFeedback> {
        let topology = self.routing_topology.read().await;

        Ok(TopologyFeedback {
            congested_routes: topology.route_metrics
                .iter()
                .filter(|(_, m)| m.avg_latency_ns > 250_000) // > 250μs
                .map(|(route, _)| route.clone())
                .collect(),
            optimal_routes: topology.route_metrics
                .iter()
                .filter(|(_, m)| m.avg_latency_ns < 200_000 && m.success_rate > 0.99) // < 200μs, > 99% success
                .map(|(route, _)| route.clone())
                .collect(),
        })
    }
}
```

### 3.4 SlotGraph Observer

**File:** `src/slotgraph_observer.rs`

```rust
use std::sync::Arc;
use tokio::sync::broadcast;
use ctas7_slotgraph_engine::SlotGraph;

/// Observer for SlotGraph routing state changes
pub struct SlotGraphObserver {
    slotgraph: Arc<SlotGraph>,
    change_tx: broadcast::Sender<TopologyChange>,
}

#[derive(Clone, Debug)]
pub enum TopologyChange {
    RouteAdded { hash: String, route: RouteInfo },
    RouteRemoved { hash: String },
    RouteUpdated { hash: String, metrics: RouteMetrics },
}

impl SlotGraphObserver {
    /// Start observing SlotGraph routing topology changes
    pub async fn observe(&self) -> Result<()> {
        // Subscribe to SlotGraph live queries for routing topology
        let mut stream = self.slotgraph.subscribe_routing_changes().await?;

        while let Some(change) = stream.next().await {
            // Broadcast topology change
            let _ = self.change_tx.send(change);
        }

        Ok(())
    }

    /// Subscribe to topology changes
    pub fn subscribe(&self) -> broadcast::Receiver<TopologyChange> {
        self.change_tx.subscribe()
    }
}
```

### 3.5 Gateway Integration

**File:** `src/gateway_integration.rs`

```rust
use crate::topology_mirror::TopologyMirror;
use crate::slotgraph_observer::SlotGraphObserver;
use sx9_gateway::state::GatewayState;

/// Gateway-specific GLAF integration
pub struct GatewayGLAFIntegration {
    topology_mirror: Arc<TopologyMirror>,
    observer: Arc<SlotGraphObserver>,
    gateway_state: Arc<GatewayState>,
}

impl GatewayGLAFIntegration {
    /// Initialize GLAF integration for gateway
    pub async fn new(gateway_state: Arc<GatewayState>) -> Result<Self> {
        let topology_mirror = Arc::new(TopologyMirror::new().await?);
        let observer = Arc::new(SlotGraphObserver::new().await?);

        Ok(Self {
            topology_mirror,
            observer,
            gateway_state,
        })
    }

    /// Start topology mirroring (if enabled)
    pub async fn start(&self) -> Result<()> {
        // Start observer
        let observer = self.observer.clone();
        tokio::spawn(async move {
            observer.observe().await.ok();
        });

        // Periodic topology mirror sync
        let mirror = self.topology_mirror.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
            loop {
                interval.tick().await;
                mirror.mirror_topology().await.ok();
            }
        });

        Ok(())
    }

    /// Get topology feedback for routing decisions
    pub async fn get_routing_feedback(&self) -> Result<TopologyFeedback> {
        self.topology_mirror.get_topology_feedback().await
    }
}
```

---

## 4. Smart Crate Manifest

**File:** `sx9-glaf-core/smart-crate.toml`

```toml
[smart-crate]
name = "sx9-glaf-core"
version = "0.1.0"
edition = "2021"
smart_crate_version = "1.2.0"
foundation = "ctas7-foundation-core"
classification = "gateway-component"
tesla_grade = true

[smart_meta]
description = "SX9 GLAF Core - Gateway topology mirroring integration"
domains = ["gateway", "routing", "graph"]
capabilities = ["topology-mirroring", "slotgraph-observation", "routing-feedback"]
build_system = "cargo"
backend_language = "rust"

[integration]
gold_disk_compatible = true
neural_mux_enabled = true
hash_engine_integrated = true
unicode_assembly_support = true
multi_tenant = true
real_time_capable = true

# GLAF-specific integrations
slotgraph_engine = true
glaf_graph_server = true
glaf_clients = true

[ports]
# GLAF system ports (from existing system)
glaf_ui = 18018
glaf_api = 18019
glaf_analytics = 18025
vfg = 18030
agr_listener = 18040

# Foundation Services
foundation_core = 18001
slotgraph_engine = 18020  # If standalone

[glaf]
# Gateway-specific GLAF configuration
enabled = false
mirror_slotgraph = true
sync_interval_ms = 100
topology_feedback = true

# Reuse existing GLAF components
use_existing_glaf = true
glaf_graph_server_port = 18019
slotgraph_engine_path = "../../ctas-7-shipyard-staging/ctas7-slotgraph-engine"

[deployment.docker]
base_image = "ctas7-foundation-core:gold-disk"
multi_stage = true
layer_caching = true
security_scanning = true

[metadata]
ctas_version = "7.1.1"
smart_crate_type = "gateway-component"
original_crate = false
certification_level = "production"
tesla_grade = true
```

---

## 5. Gateway Integration Code

**Update:** `sx9-gateway/src/lib.rs`

```rust
// Add GLAF integration module
#[cfg(feature = "glaf")]
pub mod glaf_integration;

#[cfg(feature = "glaf")]
pub use glaf_integration::GatewayGLAFIntegration;
```

**Update:** `sx9-gateway/src/state.rs`

```rust
use sx9_glaf_core::GatewayGLAFIntegration;

pub struct GatewayState {
    // ... existing fields ...
    
    #[cfg(feature = "glaf")]
    pub glaf: Option<Arc<GatewayGLAFIntegration>>,
}
```

**Update:** `sx9-gateway/src/server.rs`

```rust
use sx9_glaf_core::GatewayGLAFIntegration;

impl GatewayServer {
    pub async fn new() -> Result<Self> {
        let state = GatewayState::new().await?;

        // Initialize GLAF integration (if enabled)
        #[cfg(feature = "glaf")]
        {
            if state.config.glaf_enabled {
                let glaf = GatewayGLAFIntegration::new(state.clone()).await?;
                glaf.start().await?;
                state.glaf = Some(Arc::new(glaf));
            }
        }

        Ok(Self { state })
    }
}
```

---

## 6. Implementation Checklist

### Phase 1: Create sx9-glaf-core Crate ✅
- [ ] Create `sx9-glaf-core/` directory structure
- [ ] Create `Cargo.toml` with dependencies
- [ ] Create `smart-crate.toml` manifest
- [ ] Implement `topology_mirror.rs`
- [ ] Implement `slotgraph_observer.rs`
- [ ] Implement `gateway_integration.rs`

### Phase 2: Gateway Integration
- [ ] Add `sx9-glaf-core` dependency to `sx9-gateway/Cargo.toml`
- [ ] Add `glaf` feature flag to `sx9-gateway/Cargo.toml`
- [ ] Update `sx9-gateway/src/state.rs` to include GLAF
- [ ] Update `sx9-gateway/src/server.rs` to initialize GLAF
- [ ] Test topology mirroring

### Phase 3: Testing
- [ ] Test topology mirroring with disabled GLAF
- [ ] Test topology mirroring with enabled GLAF
- [ ] Test topology feedback for routing decisions
- [ ] Verify SlotGraph state observation
- [ ] Performance test (sync_interval_ms = 100)

### Phase 4: Documentation
- [ ] Document GLAF integration in gateway
- [ ] Update RFC-9114 Rev 1.1 with implementation details
- [ ] Create integration examples

---

## 7. Key Distinctions

### 7.1 GLAF-CORE-SPEC vs Gateway GLAF

| Aspect | GLAF-CORE-SPEC | Gateway GLAF (sx9-glaf-core) |
|--------|----------------|------------------------------|
| **Purpose** | Full graph analysis system | Topology mirroring only |
| **Use Case** | OSINT, threat analysis, graph queries | Routing topology feedback |
| **Data** | Nodes, edges, properties | Hash/unicode route mappings |
| **Queries** | Cypher++ with math extensions | Topology state observation |
| **Backends** | SurrealDB, Supabase, Sled, Sledis | SlotGraph routing state |
| **Enabled** | Always enabled | Disabled by default |

### 7.2 Reuse vs Duplicate

**Reuse (Don't Duplicate):**
- ✅ `ctas7-slotgraph-engine` - Core SlotGraph engine
- ✅ `ctas7-glaf-graph-server` - GLAF API server (if needed)
- ✅ `ctas7-glaf-clients` - Client libraries
- ✅ `CTAS7-GLAF-SYSTEM/` - Complete system specs

**Create New:**
- ✅ `sx9-glaf-core` - Gateway-specific wrapper
- ✅ Topology mirroring logic (gateway-specific)
- ✅ SlotGraph observer (gateway-specific)
- ✅ Gateway integration layer

---

## 8. Summary

**Strategy:**
1. **Reuse existing GLAF components** - Don't duplicate `ctas7-slotgraph-engine`, `ctas7-glaf-graph-server`, etc.
2. **Create `sx9-glaf-core` wrapper** - Thin integration layer for gateway
3. **Focus on topology mirroring** - Routing state observation, not full graph analysis
4. **Keep disabled by default** - Enable only after post-deployment verification

**Benefits:**
- ✅ Leverages existing, tested GLAF system
- ✅ Minimal code duplication
- ✅ Gateway-specific focus (topology mirroring)
- ✅ Maintains separation of concerns

**Next Steps:**
1. Create `sx9-glaf-core` crate structure
2. Implement topology mirroring logic
3. Integrate into `sx9-gateway`
4. Test and verify

---

**Status:** Integration plan complete. Ready to implement `sx9-glaf-core`.



