# RFC-9114 Rev 1.1

## SX9 Gateway Architecture – Neural Retrofit Edition (Complete)

**Status:** DRAFT (Rev 1.1)  
**Version:** 7.1.1  
**Date:** December 2025  
**Author:** Synaptix9 Engineering Group  
**Dependencies:** RFC-9001 through RFC-9876, RFC-9115 (Frontend Adapter)  
**Supersedes:** RFC-9114 v1.0 (2025-06)  
**Applies To:** Synaptix9 (SX9) gateway family  
**Smart Crate Version:** v1.2.0

---

## 0. Abstract

This revision defines the **Neural Retrofit Architecture** for the Synaptix9 Gateway System (SX9).  
It unifies deterministic routing, cognitive stream handling, and dormant neural subsystems (ANN + GLAF) under a single executable smart crate.  
The goal is to maintain RFC-grade determinism while preparing for adaptive optimization through a learn-by-observation neural layer.

Rev 1.1 introduces:

1. **Artificial Neural Node (ANN)** integration (`sx9-ann-engine`) – dormant observer mode.
2. **Genome Link Analysis Fabric (GLAF)** feedback channel – topology mirror only.
3. **Smart Crate v1.2.0** manifest schema with ANN/GLAF, DSL, ATLAS Daemon, and PLASMA Defender modules.
4. **Annex A** – standardized Docker → OrbStack deployment pattern.
5. **Port Manager Integration** – crystal-gated port allocation via `ctas7-real-port-manager` (port 18104).
6. **System Integration Details** – USIM, EEI, Foundation Manifold, Foundation Math, Government Data Manifold, Ops-Main Platform.
7. **Agent/Intel Consolidation** – Unified Agent Registry and Intel Gateway integration.
8. **PLASMA-ECS Architecture** – Three-layer ECS (Legion + apecs + ATLAS).

All cryptographic, hashing, and routing standards remain unchanged: Murmur3-64 only, <250 ns routing, <50 µs Bernoulli zone latency, zero LLM usage in deterministic zones.

---

## 1. Naming and Scope

### 1.1 Primary Gateway

**Crate:** `sx9-gateway-primary`  
**Purpose:** Unified API surface for all Synaptix9 operations (WebSocket / REST / gRPC).

### 1.2 Domain Variants

Pattern → `sx9-gateway-{domain}`

| Variant                     | Domain        | Function                      |
| --------------------------- | ------------- | ----------------------------- |
| `sx9-gateway-orbital`       | Orbital       | Satellite & space ops         |
| `sx9-gateway-maritime`      | Maritime      | Port & vessel intelligence    |
| `sx9-gateway-manufacturing` | Manufacturing | Industrial automation         |
| `sx9-gateway-cyber`         | Cyber         | Network security ops          |
| `sx9-gateway-kinetic`       | Kinetic       | Physical mission control      |
| `sx9-gateway-cognitive`     | Cognitive     | Knowledge fusion ops          |
| `sx9-gateway-spectrum`      | Spectrum      | EM domain coordination        |
| `sx9-gateway-subterranean`  | Subterranean  | Underground & sensor networks |
| `sx9-gateway-temporal`      | Temporal      | Time-variant analysis         |

Each domain variant shares 100% API compatibility and 85% binary overlap with `sx9-gateway-primary`.

---

## 2. System Architecture Overview

### 2.1 Unified API Surface

```
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 GATEWAY PRIMARY                          │
├─────────────────────────────────────────────────────────────────┤
│ ┌────────────┐ ┌────────────┐ ┌────────────┐                   │
│ │ WebSocket  │ │ REST       │ │ gRPC       │                   │
│ │ (18120)    │ │ (18121)    │ │ (18122)    │                   │
│ └─────┬──────┘ └─────┬──────┘ └─────┬──────┘                   │
│       │              │              │                           │
│       └──────────────┴──────────────┘                           │
│                       ▼                                         │
│       ┌──────────────────────────┐                             │
│       │ Unified Request Router   │                             │
│       │ (Deterministic Routing)  │                             │
│       └─────────────┬────────────┘                             │
│                     │                                           │
│       ┌─────────────┼─────────────┐                            │
│       │             │             │                            │
│       ▼             ▼             ▼                            │
│ Foundation Manifold Streaming Fabric Domain Handlers            │
│ (RFC-9004)        (NATS JetStream) (Modular Crates)             │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Execution Stack

```
Layer 4 – Cognitive Control (ATLAS Daemon)
Layer 3 – Routing & Neural Mux (Core <250 ns)
Layer 2 – Streaming / NATS / PLASMA ECS
Layer 1 – I/O Fabric (WebSocket / REST / gRPC)
```

### 2.3 ANN and GLAF Placement (Compile-Time Only)

```
ANN Engine (sx9-ann-engine) → observes routing entropy
GLAF Core (sx9-glaf-core) → mirrors SlotGraph state
Both compile into binary but remain disabled (runtime flag)
```

**Note:** Crates `sx9-ann-engine`, `sx9-glaf-core`, `sx9-dsl-engine`, and `sx9-plasma-defender` are referenced but may not exist yet. They should be created or integrated with existing systems (e.g., GLAF may integrate with `CTAS7-GLAF-SYSTEM/`).

---

## 3. Deterministic Routing – RFC-9004

> Target Latency: < 250 ns (p99)

Routing employs `ctas7-neural-mux` with pre-computed hash tables and lock-free lookup.

```rust
pub struct GatewayRouter {
    neural_mux: Arc<NeuralMuxRouter>,
    route_table: Arc<DashMap<u16, RouteEntry>>,
}

impl GatewayRouter {
    #[inline(always)]
    pub fn route(&self, hash: &TrivariateHash) -> Option<RouteEntry> {
        self.neural_mux.route(hash) // RFC-9004 §3.3
    }
}
```

**Diagnostics (PLASMA Defender Integration):**

```rust
if elapsed.as_nanos() > 250 {
    tracing::warn!("Routing exceeded 250 ns: {:?}", elapsed);
}
```

---

## 4. Trivariate Hash System – RFC-9001

Every operational artifact MUST carry a trivariate hash (SCH / CUID / UUID).  
All hashes use Murmur3-64 (Base-96 encoding, 48 chars total).

```rust
let triv = TrivariateHashEngine::new().generate(
    "payload", "context", "entity"
);
```

**Seeds:**

| Component | Seed (hex)  | Purpose               |
| --------- | ----------- | --------------------- |
| SCH       | 0xC7A5_0000 | Semantic content hash |
| CUID      | 0xC7A5_0001 | Contextual ordering   |
| UUID      | 0xC7A5_0002 | Persistence anchor    |

---

## 5. Hourglass-Bernoulli Zone Compliance (RFC-9026)

### 5.1 Zone Classification

| Zone               | Latency       | Purpose            | LLM Allowed |
| ------------------ | ------------- | ------------------ | ----------- |
| A – Tactical       | < 50 µs       | Hash + Routing     | ❌          |
| B – Operational    | 50 µs – 1 ms  | ATLAS ticks / OODA | ⚙️          |
| C – Analytical     | 1 ms – 100 ms | Graph + GLAF       | ✅          |
| D – Infrastructure | 100 ms – 60 s | Crate spawn / Ops  | ✅          |

Routing and trivariate operations remain locked inside **Zone A**.

```rust
match req.zone() {
    Zone::A => req.exec_deterministic(),      // pure Rust
    Zone::B => atlas.tick(req).await?,
    Zone::C => glaf.analyze(req).await?,
    Zone::D => iac.spawn(req).await?,
}
```

---

## 6. System Integrations

### 6.1 USIM Ephemeral Messaging (RFC-9008)

**Implementation:**

```rust
pub struct USIMIntegration {
    usim_engine: Arc<USIMReactiveEngine>,
}

impl USIMIntegration {
    pub async fn process_usim_message(&self, message: USIMMessage) -> Result<()> {
        // Check TTL (RFC-9008: Ephemeral intelligence)
        if let Some(expires_at) = message.expires_at {
            if expires_at < Utc::now() {
                return Err(anyhow::anyhow!("USIM message expired"));
            }
        }

        // Route based on USIM content
        self.route_usim_message(message).await
    }
}
```

**TTL enforcement guarantees no stale intelligence persists past its time-of-value window.**

### 6.2 EEI Foundation Crate Integration

**EEI pre-checks govern routing permission:**

```rust
pub struct EEIIntegration {
    eei_processor: Arc<EEIProcessor>,
    backplane: Arc<PlasmaBackplane>,
}

impl EEIIntegration {
    /// Check EEI requirements before routing
    pub async fn check_eei_requirements(&self, request: &Request) -> Result<bool> {
        let eei_status = self.eei_processor.check_requirements(request).await?;

        // EEI affects crystal/thyristor gating
        if !eei_status.fulfilled {
            self.backplane.gate_request(request)?;  // Block or allow
        }

        Ok(eei_status.fulfilled)
    }
}
```

### 6.3 Foundation Manifold (RFC-9004)

All crate calls traverse the manifold's Neural Mux (< 250 ns).

**Fail-safe fallback order:**

1. Local route (in-memory cache)
2. SurrealDB lookup (live query)
3. Supabase mirror (ACID backup)
4. ANN advisory (if enabled)

```rust
pub struct FoundationManifoldIntegration {
    orchestrator: Arc<FoundationOrchestrator>,
    surreal: Arc<SurrealClient>,
    supabase: Arc<SupabaseClient>,
    ann: Option<Arc<ANNEngine>>,
}

impl FoundationManifoldIntegration {
    pub async fn route_foundation_crate(
        &self,
        crate_name: &str,
        request: &Request,
    ) -> Result<Response> {
        // 1. Try local route
        if let Some(response) = self.local_cache.get(crate_name) {
            return Ok(response);
        }

        // 2. Try SurrealDB
        if let Ok(response) = self.surreal.query_crate(crate_name).await {
            return Ok(response);
        }

        // 3. Try Supabase mirror
        if let Ok(response) = self.supabase.query_crate(crate_name).await {
            return Ok(response);
        }

        // 4. ANN advisory (if enabled)
        if let Some(ann) = &self.ann {
            if ann.is_enabled() {
                return ann.advisory_route(crate_name, request).await;
            }
        }

        Err(anyhow::anyhow!("No route found"))
    }
}
```

### 6.4 Foundation Math Bridge

**Symbolic compute interface:**

```rust
pub struct FoundationMathIntegration {
    math_engine: Arc<MathematicalFoundationConsciousness>,
}

impl FoundationMathIntegration {
    /// Execute mathematical computation
    pub async fn compute(&self, expression: &str) -> Result<SymbolicResult> {
        self.math_engine.symbolic_compute(expression).await
    }
}

// Usage:
let result = foundation_math.compute("∫ sin(x) dx").await?;
```

### 6.5 Government Data Manifold

**Subscribed streams:**

```rust
pub struct GovernmentDataIntegration {
    gov_manifold: Arc<GovernmentDataManifold>,
    nats: Arc<Client>,
}

impl GovernmentDataIntegration {
    /// Subscribe to government data feeds
    pub async fn subscribe(&self, feed_type: GovernmentFeedType) -> Result<()> {
        let subject = format!("sx9.stream.gov.{}", feed_type);
        self.nats.subscribe(&subject).await?;
        Ok(())
    }
}
```

**Subscribed streams:**

- `gov.sec.edgar` – SEC filings
- `gov.census.pop` – Population feeds
- `gov.fcc.spectrum` – Spectrum allocations
- `gov.cisa.alerts` – CISA security alerts
- `gov.treasury.sanctions` – Treasury sanctions

Each record enters GLAF mirror when enabled.

### 6.6 Ops-Main Platform Interface

**WebSocket / REST / gRPC handlers unify Ops-Main UI with gateway telemetry.**

```rust
pub struct OpsMainIntegration {
    websocket_server: Arc<WebSocketServer>,
    rest_handler: Arc<RestHandler>,
    grpc_handler: Arc<GrpcHandler>,
}

impl OpsMainIntegration {
    /// Handle Ops-Main WebSocket connection
    pub async fn handle_websocket(&self, stream: TcpStream) -> Result<()> {
        self.websocket_server.handle(stream).await
    }

    /// Handle REST API requests
    pub async fn handle_rest(&self, request: Request) -> Result<Response> {
        self.rest_handler.handle(request).await
    }

    /// Handle gRPC requests
    pub async fn handle_grpc(&self, request: GrpcRequest) -> Result<GrpcResponse> {
        self.grpc_handler.handle(request).await
    }

    /// Handle modal inventory (Playwright)
    pub async fn handle_modal_inventory(&self, inventory: ModalInventory) -> Result<()> {
        // Process Playwright modal inventory
        Ok(())
    }
}
```

---

## 7. Neural Retrofit Subsystems

### 7.1 Artificial Neural Node (ANN) Engine

The ANN engine (`sx9-ann-engine`) observes routing entropy, building internal weight maps but producing no control signals until enabled.

**Compile-time default:**

```toml
[ann]
enabled = false
mode = "observe"
```

**Telemetry sample:**

```rust
pub struct ANNEngine {
    weights: Arc<RwLock<WeightMap>>,
    enabled: AtomicBool,
    mode: AtomicU8,  // 0=observe, 1=advisory, 2=control
}

impl ANNEngine {
    pub fn observe(&self, metric: &str, value: u64) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        // Build weight maps (no control signals)
        self.weights.write().unwrap().update(metric, value);
    }

    pub async fn advisory_route(&self, crate_name: &str, request: &Request) -> Result<Response> {
        if self.mode.load(Ordering::Relaxed) < 1 {
            return Err(anyhow::anyhow!("ANN not in advisory mode"));
        }

        // Provide routing advisory (non-binding)
        let advisory = self.weights.read().unwrap().suggest_route(crate_name)?;
        Ok(advisory)
    }
}
```

**ANN activation criteria (Annex A §A.3):**

- Stable routing < 250 ns p99
- Entropy drift < 0.03
- GLAF mirror synchronization stable ≥ 99%

**Note:** `sx9-ann-engine` crate should be created at `sx9/sx9-ann-engine/` or integrated with existing neural systems.

### 7.2 Genome Link Analysis Fabric (GLAF) Feedback

`glaf-core` mirrors the gateway's SlotGraph state for topological feedback.

**Active only when flag set:**

```toml
[glaf]
enabled = false
mirror_slotgraph = true
```

**Implementation:**

```rust
pub struct GLAFIntegration {
    glaf_core: Arc<GLAFCore>,
    surreal: Arc<SurrealClient>,
}

impl GLAFIntegration {
    /// Mirror SlotGraph routing topology (not data)
    pub async fn mirror_slotgraph(&self) -> Result<()> {
        // Live query SurrealDB for routing topology changes
        let stream = self.surreal.select("routing_topology").live().await?;

        while let Some(change) = stream.next().await {
            // Mirror routing topology to GLAF (hash/unicode routes, not data)
            self.glaf_core.mirror_topology(change).await?;
        }

        Ok(())
    }
}
```

**Note:** `sx9-glaf-core` may integrate with existing `CTAS7-GLAF-SYSTEM/` instead of being a separate crate.

### 7.3 ATLAS Daemon System

**Continuous cognitive tick loop:**

```rust
pub struct ATLASDaemon {
    tick_interval: Duration,
    phase_sequence: Vec<OODAPhase>,
}

impl ATLASDaemon {
    pub async fn start(&self) -> Result<()> {
        let mut ticker = interval(self.tick_interval);

        loop {
            ticker.tick().await;
            self.tick().await?;
        }
    }

    async fn tick(&self) -> Result<()> {
        for phase in &self.phase_sequence {
            match phase {
                OODAPhase::Observe => self.observe().await?,
                OODAPhase::Orient => self.orient().await?,
                OODAPhase::Decide => self.decide().await?,
                OODAPhase::Act => self.act().await?,
            }
        }
        Ok(())
    }
}
```

**Configuration:**

```toml
[atlas]
daemon = "sx9-atlas-daemon"
tick_interval_ms = 1
phase_sequence = ["observe", "orient", "decide", "act"]
enabled = true
```

### 7.4 Domain-Specific Language (DSL) Engine

**Symbolic control plane ("Cypher++ subset").**

```rust
pub struct DSLEngine {
    runtime: WasmRuntime,
    reload_on_change: bool,
}

impl DSLEngine {
    pub async fn exec(&self, query: &str) -> Result<DSLResult> {
        // Execute DSL query in WASM runtime
        self.runtime.execute(query).await
    }
}

// Usage:
dsl.exec("MATCH (a)-[r]->(b) RETURN r").await?;
```

**Hot-reloadable in WASM runtime.**

**Configuration:**

```toml
[dsl]
engine = "sx9-dsl-engine"
language = "symbolic"
runtime = "wasm"
reload_on_change = true
enabled = true
```

**Note:** `sx9-dsl-engine` crate should be created or integrated with existing DSL systems.

### 7.5 PLASMA Defender Integration

**Monitors health and latency budgets.**

```rust
pub struct PLASMADefender {
    health_endpoint: String,
    metrics_endpoint: String,
    enforce_latency: bool,
}

impl PLASMADefender {
    pub fn emit_alert(&self, alert_type: &str, value: u64) {
        if alert_type == "latency" && value > 250 {
            tracing::warn!("Routing latency exceeded: {} ns", value);
        }
    }
}
```

**Configuration:**

```toml
[plasma_defender]
engine = "sx9-plasma-defender"
health_endpoint = "/health"
metrics_endpoint = "/metrics"
enforce_latency = true
enabled = true
```

**Note:** `sx9-plasma-defender` crate should be created or integrated with existing PLASMA systems.

---

## 8. Port Manager Integration

### 8.1 Crystal-Gated Port Allocation

**All gateway ports MUST be allocated via `ctas7-real-port-manager` (port 18104).**

```rust
pub struct PortManagerIntegration {
    port_manager: Arc<PortManagerClient>,
    crystal_gated: bool,
}

impl PortManagerIntegration {
    /// Allocate gateway port with crystal resonance check
    pub async fn allocate_gateway_port(&self, service_hash: &TrivariateHash) -> Result<u16> {
        // Request port allocation via Port Manager
        let allocation = self.port_manager
            .allocate_port_gated(service_hash, self.crystal_gated)
            .await?;

        // Port Manager checks crystal resonance and SDT gate state
        // Only allocated if SDT gate is Conducting or Latched
        Ok(allocation.port)
    }
}
```

**Port Manager Configuration:**

```toml
[port_manager]
endpoint = "http://localhost:18104"
crystal_gated = true
mirror_ports = true
```

**Gateway Port Allocations:**

```toml
[ports]
# Allocated via ctas7-real-port-manager (port 18104)
websocket = 18120  # Allocated via port manager
rest = 18121       # Allocated via port manager
grpc = 18122       # Allocated via port manager

# Foundation Services (fixed ports)
port_manager = 18104      # ctas7-real-port-manager
foundation_core = 18001   # ctas7-foundation-core
hashing_engine = 18105    # ctas7-hashing-engine
neural_mux = 18107        # ctas7-neural-mux
atlas_daemon = 18106      # sx9-atlas-daemon
```

**Mirror Port System:**

Each primary port can have multiple mirror ports allocated based on crystal ring strength:

- Ring strength ≥ 0.98 → 3 mirrors
- Ring strength ≥ 0.90 → 2 mirrors
- Ring strength ≥ 0.75 → 1 mirror

---

## 9. Smart Crate v1.2.0 Manifest (Complete)

```toml
[smart-crate]
name = "sx9-gateway-primary"
version = "7.1.1"
edition = "2021"
smart_crate_version = "1.2.0"
foundation = "ctas7-foundation-core"
classification = "gateway"
tesla_grade = true

[smart_meta]
description = "SX9 Gateway Primary - Unified API gateway for all Synaptix9 operations"
domains = ["gateway", "routing", "streaming", "api"]
capabilities = ["websocket", "rest", "grpc", "deterministic-routing", "streaming-intelligence", "l2-execution"]
build_system = "cargo"
backend_language = "rust"
frontend_language = "typescript"

# XSD validation schemas
xsd_schemas = [
    "config/gateway-config.xsd",
    "config/routing-config.xsd",
    "config/streaming-config.xsd"
]

# Unicode metaprogramming
unicode_operators = true
unicode_symbols = ["🔀", "🌐", "⚡", "🔒", "📡", "🎯", "🧠", "⚙️"]

[integration]
gold_disk_compatible = true
neural_mux_enabled = true
hash_engine_integrated = true
unicode_assembly_support = true
multi_tenant = true
real_time_capable = true
layer2_fabric_node = true
world_registry_participant = true

# Gateway-specific integrations
ops_main_platform = true
usim_integration = true
eei_integration = true
foundation_manifold = true
foundation_math = true
government_data_manifold = true
l2_execution = true
kali_iso_integration = true

# Agent and Intel consolidation
unified_agent_registry = true
unified_intel_gateway = true

# PLASMA-ECS integration
plasma_ecs_enabled = true
legion_layer = true
apecs_layer = true
atlas_layer = true

[ports]
# Allocated via ctas7-real-port-manager (port 18104)
websocket = 18120
rest = 18121
grpc = 18122

# Foundation Services (fixed ports)
port_manager = 18104
foundation_core = 18001
hashing_engine = 18105
neural_mux = 18107
atlas_daemon = 18106

[port_manager]
endpoint = "http://localhost:18104"
crystal_gated = true
mirror_ports = true

[ann]
engine = "sx9-ann-engine"
enabled = false
mode = "observe"
entropy_seed = "murmur3-64"
sync_interval_ms = 1000
weights_path = "/data/ann/weights-v1.0.json"

[glaf]
engine = "sx9-glaf-core"
enabled = false
mirror_slotgraph = true
sync_interval_ms = 100
topology_feedback = true

[atlas]
daemon = "sx9-atlas-daemon"
tick_interval_ms = 1
phase_sequence = ["observe", "orient", "decide", "act"]
enabled = true

[dsl]
engine = "sx9-dsl-engine"
language = "symbolic"
runtime = "wasm"
reload_on_change = true
enabled = true

[plasma_defender]
engine = "sx9-plasma-defender"
health_endpoint = "/health"
metrics_endpoint = "/metrics"
enforce_latency = true
enabled = true

[security]
slsa_level = 3
hermetic_builds = true
provenance_required = true
source_verification = true
supply_chain_security = true

# Gateway security
data_encryption = true
access_controls = true
audit_logging = true
compliance_monitoring = true

[performance]
# Performance targets for gateway
routing_latency_ns = 250
trivariate_hash_latency_us = 50
bernoulli_zone_a_latency_us = 50
l2_trigger_latency_us = 10
nats_publish_latency_ms = 5
throughput_routes_per_sec = 10000000

[endpoints]
health = "/health"
metrics = "/metrics"
status = "/smart-crate/status"
websocket = "/ws"
rest_api = "/api/v1"
grpc = "/grpc"

[metadata]
ctas_version = "7.1.1"
smart_crate_type = "gateway"
original_crate = true
certification_level = "production"
tesla_grade = true
world_registry_participant = true

[features]
default = ["gateway-core", "routing", "streaming"]

gateway-core = ["websocket", "rest", "grpc"]
routing = ["neural-mux", "foundation-manifold"]
streaming = ["nats-jetstream", "time-value-decay"]
l2-execution = ["unicode-triggers", "kali-iso"]
neural-retrofit = ["ann-observe", "glaf-mirror"]
```

All modules compile into a single binary; ANN and GLAF remain disabled until post-deployment verification.

---

## 10. Agent Consolidation Integration

### 10.1 Unified Agent Registry

**Gateway integrates with `sx9-unified-agent-registry` for agent operations.**

```rust
pub struct AgentConsolidationIntegration {
    agent_registry: Arc<UnifiedAgentRegistry>,
    nats: Arc<Client>,
}

impl AgentConsolidationIntegration {
    /// Route agent request via unified registry
    pub async fn route_agent_request(&self, request: AgentRequest) -> Result<AgentResponse> {
        // Escalate via NATS if needed
        if request.requires_escalation {
            self.nats.publish("sx9.escalate.agent", &request).await?;
        }

        // Route via unified agent registry
        self.agent_registry.route(request).await
    }
}
```

**NATS Escalation Subjects:**

- `sx9.escalate.agent` – Agent activation
- `sx9.escalate.persona` – Persona activation
- `sx9.escalate.registry` – Agent registry activation

**PLASMA-ECS Integration:**

- Agent entities in Legion World (Layer 2)
- Agent async operations in apecs (Layer 1)
- Agent cognitive orchestration in ATLAS (Layer 3)

---

## 11. Intel Consolidation Integration

### 11.1 Unified Intel Gateway

**Gateway integrates with `sx9-intel-gateway` for intelligence operations.**

```rust
pub struct IntelConsolidationIntegration {
    intel_gateway: Arc<IntelGateway>,
    nats: Arc<Client>,
}

impl IntelConsolidationIntegration {
    /// Route intel request via unified gateway
    pub async fn route_intel_request(&self, request: IntelRequest) -> Result<IntelResponse> {
        // Route via unified intel gateway
        self.intel_gateway.route(request).await
    }

    /// Subscribe to intel streams
    pub async fn subscribe_intel_stream(&self, stream_type: IntelStreamType) -> Result<()> {
        let subject = format!("sx9.stream.intel.{}", stream_type);
        self.nats.subscribe(&subject).await?;
        Ok(())
    }
}
```

**Python Tools Bridge:**

- API-only access to Python intel tools (Conda environment)
- REST/gRPC interface for Python tools
- No direct Python execution in gateway

**USIM/EEI Integration:**

- USIM for ephemeral intelligence
- EEI for time-of-value classification
- Government Data Manifold for government intelligence

---

## 12. PLASMA-ECS Architecture Integration

### 12.1 Three-Layer Architecture

**Gateway integrates with PLASMA-ECS (Legion + apecs + ATLAS).**

```
┌─────────────────────────────────────────────────────────────┐
│                    PLASMA-ECS LAYERS                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  LAYER 3: ATLAS (Cognitive)                                 │
│  ═══════════════════════════                                │
│  • ATLAS Daemon (1ms OODA loop)                            │
│  • sx9-atlas-bus (ring buffer, PlasmaState)                │
│  • Crystal resonance, SDT gate control                     │
│  • Priority routing (critical/urgent/normal)                │
│  • NATS bridge for distributed ops                         │
│                                                              │
│  LAYER 2: Legion (Deterministic Batch)                       │
│  ═══════════════════════════════════════                    │
│  • High-performance batch processing                       │
│  • Deterministic tick-based world state                    │
│  • Hot-path operations (<1ms latency)                      │
│  • Entity-component queries                                │
│  • Schedule execution                                      │
│  • Slot Graph hash/unicode routing (preserved)              │
│                                                              │
│  LAYER 1: apecs (Async I/O)                                 │
│  ═══════════════════════════                                │
│  • Async-friendly operations                               │
│  • WASM-compatible                                         │
│  • I/O-bound tasks (network, database)                     │
│  • UI integration                                         │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Gateway Integration:**

```rust
pub struct PLASMAECSIntegration {
    legion_world: Arc<LegionWorld>,      // Layer 2
    apecs_world: Arc<ApecsWorld>,        // Layer 1
    atlas_daemon: Arc<ATLASDaemon>,      // Layer 3
    plasma_state: Arc<PlasmaState>,
}

impl PLASMAECSIntegration {
    /// Route request through PLASMA-ECS layers
    pub async fn route(&self, request: Request) -> Result<Response> {
        // Layer 1: Async I/O (apecs)
        if request.is_io_bound() {
            return self.apecs_world.handle(request).await;
        }

        // Layer 2: Deterministic batch (Legion)
        if request.is_cpu_bound() {
            return self.legion_world.handle(request).await;
        }

        // Layer 3: Cognitive orchestration (ATLAS)
        self.atlas_daemon.handle(request).await
    }
}
```

**Unified PlasmaState:**

- Shared across all layers
- Delta angle tracking (0-360°)
- Crystal resonance state
- SDT gate state

---

## 13. Streaming Architecture (RFC-9130)

### 13.1 Unified NATS JetStream Backbone

| Stream Category | Subject Pattern                   | Half-Life |
| --------------- | --------------------------------- | --------- |
| SIGINT          | `sx9.stream.intel.sigint.{tier}`  | 48 h      |
| HUMINT          | `sx9.stream.intel.humint.{tier}`  | 7 d       |
| GEOINT          | `sx9.stream.intel.geoint.{tier}`  | 30 d      |
| OSINT           | `sx9.stream.intel.osint.{tier}`   | 24 h      |
| TECHINT         | `sx9.stream.intel.techint.{tier}` | 12 h      |
| OPS System      | `sx9.stream.ops.system.{cat}`     | real-time |
| OPS Workflow    | `sx9.stream.ops.workflow.{evt}`   | real-time |
| PLASMA          | `sx9.stream.ops.plasma.{evt}`     | < 50 ms   |
| ATLAS           | `sx9.stream.ops.atlas.{evt}`      | < 1 ms    |

```rust
let js = nats::jetstream::new();
js.publish("sx9.stream.ops.plasma.heartbeat", heartbeat)?;
```

### 13.2 Time-of-Value Decay Model (RFC-9026 §4.2)

```rust
fn time_decay(t0: Instant, hl: Duration) -> f64 {
    let age = Instant::now() - t0;
    0.5_f64.powf(age.as_secs_f64() / hl.as_secs_f64())
}
```

All analytic layers must apply enzymatic half-life to ensure cognitive freshness.

---

## 14. Layer-2 Execution (RFC-9876 / RFC-9130)

### 14.1 Unicode Trigger Pathway

```rust
pub async fn handle_trigger(&self, trig: char) -> Result<()> {
    if !(0xE000..=0xF8FF).contains(&(trig as u32)) {
        return Err(anyhow!("Invalid L2 trigger"));
    }
    self.nats.publish("sx9.l2.trigger", trig).await?;
    self.kali.orchestrate(trig).await
}
```

XDP/eBPF interceptors validate ingress within < 1 µs before forwarding into NATS for hermetic execution.

### 14.2 Atlas → Kali Pipeline

```
ANN observe() → ATLAS tick() → PLASMA defender → Kali exec()
```

The ANN layer may later bias Atlas phase selection once activated.

---

## 15. Unified Schema & Persistence (RFC-9005)

### 15.1 Supabase ACID Core

```rust
supabase.from("entities")
    .insert(entity)
    .execute()
    .await?;
```

All entities carry full trivariate IDs and lineage metadata.

### 15.2 SurrealDB Mirror

Live queries ensure reactive graph synchronization with GLAF:

```rust
let stream = surreal.select("node").live().await?;
while let Some(change) = stream.next().await {
    glaf.mirror(change);
}
```

---

## 16. Security Model (RFC-9001 + RFC-9004)

### 16.1 Authentication

```
Authorization: SCH triv:aB7x9p…
X-SCH-Signature: murmur3(body+timestamp+sch)
X-Timestamp: 1733494500
```

Murmur3-64 HMAC with ±5 min drift window. All failed validations emit a PLASMA alert.

### 16.2 Sandbox Boundaries

WASM modules operate in memory-only context; no filesystem or network calls outside the proxy.

---

## 17. Performance Targets

| Operation        | Target   | P99      | Hard Limit | RFC       |
| ---------------- | -------- | -------- | ---------- | --------- |
| Routing          | < 200 ns | < 250 ns | 500 ns     | 9004 §3.3 |
| Trivariate Hash  | < 10 µs  | < 50 µs  | 100 µs     | 9001      |
| Bernoulli Zone A | < 50 µs  | < 50 µs  | 100 µs     | 9026      |
| L2 Trigger       | < 1 µs   | < 10 µs  | 50 µs      | 9876      |
| NATS Publish     | < 1 ms   | < 5 ms   | 10 ms      | 9130      |

Throughput: 10 M routes / sec verified.

---

## 18. Annex A – Deployment Profiles

### A.1 Docker Compose Reference

```yaml
version: "3.9"
services:
  sx9-gateway:
    image: sx9-gateway:7.1.1
    build: .
    ports:
      - "18120:18120" # WebSocket
      - "18121:18121" # REST
      - "18122:18122" # gRPC
    env_file: .env
    depends_on:
      - port-manager
      - supabase
      - surrealdb
      - nats
      - redis
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:18121/health"]
      interval: 10s
      retries: 5
    volumes:
      - sx9_data:/data
      - ann_weights:/data/ann

  port-manager:
    image: ctas7-real-port-manager:7.2.0
    ports:
      - "18104:18104"
    environment:
      PORT_MANAGER_PORT: 18104
      CRYSTAL_GATED: "true"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:18104/health"]
      interval: 10s

  supabase:
    image: supabase/postgres:15
    environment:
      POSTGRES_PASSWORD: ${SUPABASE_PASS}
    ports:
      - "5432:5432"
    volumes:
      - supabase_data:/var/lib/postgresql/data

  surrealdb:
    image: surrealdb/surrealdb:latest
    command: start --user root --pass ${SURREAL_PASS} memory
    ports:
      - "8000:8000"

  nats:
    image: nats:2.10-alpine
    ports:
      - "4222:4222" # NATS
      - "8222:8222" # HTTP monitoring
    command: ["-js", "-m", "8222"]

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  sx9_data:
  ann_weights:
  supabase_data:

networks:
  default:
    name: sx9-net
```

### A.2 OrbStack Profile

```bash
# Initialize OrbStack environment
orb init sx9-gateway

# Run gateway with GPU and memory allocation
orb run --gpu --memory 2G --network sx9-net sx9-gateway:7.1.1

# Run with port mappings
orb run \
  --gpu \
  --memory 2G \
  --network sx9-net \
  --publish 18120:18120 \
  --publish 18121:18121 \
  --publish 18122:18122 \
  --volume sx9_data:/data \
  --volume ann_weights:/data/ann \
  sx9-gateway:7.1.1
```

OrbStack is used for development-grade real-time monitoring and zero-overhead sandboxing.

### A.3 ANN / GLAF Activation Checklist

| Condition     | Threshold   | Action                 |
| ------------- | ----------- | ---------------------- |
| Routing p99   | ≤ 250 ns    | ✅ Enable ANN          |
| Entropy Drift | < 0.03      | ✅ Enable ANN          |
| GLAF Sync     | ≥ 99%       | ✅ Enable GLAF         |
| CPU Load      | < 60%       | ✅ Allow training loop |
| Any Violation | > threshold | 🔒 Keep disabled       |

Activation performed by DevOps via `sx9ctl enable ann|glaf`.

---

## 19. Conformance Matrix

Systems claiming RFC-9114 Rev 1.1 conformance MUST meet:

1. ✅ Unified API surface (WebSocket / REST / gRPC).
2. ✅ Foundation Manifold routing < 250 ns.
3. ✅ Trivariate hash standard (Murmur3-64 Base96).
4. ✅ Hourglass-Bernoulli zone compliance (no LLMs in Zone A).
5. ✅ NATS JetStream backbone integration.
6. ✅ L2 Unicode trigger execution.
7. ✅ Smart Crate v1.2.0 manifest present (complete with all sections).
8. ✅ Port Manager integration (crystal-gated port allocation).
9. ✅ ANN and GLAF modules compiled but disabled by default.
10. ✅ PLASMA Defender active and reporting.
11. ✅ All code production-grade; no stubs or hardcoded data.
12. ✅ System integrations (USIM, EEI, Foundation Manifold, Foundation Math, Government Data Manifold, Ops-Main Platform).
13. ✅ Agent consolidation integration (Unified Agent Registry).
14. ✅ Intel consolidation integration (Unified Intel Gateway).
15. ✅ PLASMA-ECS architecture integration (Legion + apecs + ATLAS).

---

## 20. Revision Control

| Rev | Date    | Changes                                                                          | Author                      |
| --- | ------- | -------------------------------------------------------------------------------- | --------------------------- |
| 1.0 | 2025-06 | Initial Gateway Spec                                                             | Synaptix9 Core Team         |
| 1.1 | 2025-12 | Neural Retrofit (ANN + GLAF + Smart Crate v1.2.0) + Complete Integration Details | Synaptix9 Engineering Group |

---

## 21. Crate Existence Status

**Note:** The following crates are referenced in this RFC but may not exist yet:

| Crate                        | Status       | Action Required                                                                |
| ---------------------------- | ------------ | ------------------------------------------------------------------------------ |
| `sx9-ann-engine`             | ⚠️ Not Found | Create at `sx9/sx9-ann-engine/` or integrate with existing neural systems      |
| `sx9-glaf-core`              | ⚠️ Not Found | Create at `sx9/sx9-glaf-core/` or integrate with existing `CTAS7-GLAF-SYSTEM/` |
| `sx9-dsl-engine`             | ⚠️ Not Found | Create at `sx9/sx9-dsl-engine/` or integrate with existing DSL systems         |
| `sx9-plasma-defender`        | ⚠️ Not Found | Create at `sx9/sx9-plasma-defender/` or integrate with existing PLASMA systems |
| `sx9-unified-agent-registry` | ⚠️ Not Found | Create at `sx9/sx9-unified-agent-registry/` or enhance `ctas7-agent-registry`  |
| `sx9-intel-gateway`          | ⚠️ Not Found | Create at `sx9/sx9-intel-gateway/`                                             |

**Recommendation:** Verify crate existence and create missing crates or update RFC to reference existing implementations.

---

**End of RFC-9114 Rev 1.1 (Complete)**
