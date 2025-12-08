# SX9 Gateway Build Plan
## sx9-gateway-primary Implementation Roadmap

**Date:** December 2025  
**Status:** Build Plan  
**Gateway Name:** `sx9-gateway-primary`  
**Domain Variants:** `sx9-gateway-orbital`, `sx9-gateway-maritime`, `sx9-gateway-manufacturing`, etc.

---

## Executive Summary

**Goal:** Build `sx9-gateway-primary` as the unified API gateway for all Synaptix9 operations, implementing RFC-9114 compliance with deterministic routing, streaming intelligence, foundation crate integration, and domain-specific variants.

**Architecture:** Minimal startup (12 crates) + Escalation components (14 crates activated via NATS + DSL playbooks)

**Critical Path:**
1. Gateway Core (sx9-gateway, sx9-atlas-bus, neural-mux, port-manager)
2. Database Layer (SurrealDB, data-fabric, nats-fabric)
3. Frontend Bridge (ops-main-platform)
4. Foundation Integrations (USIM, EEI, Foundation Manifold, Foundation Math, Government Data Manifold)
5. L2 Execution (Kali ISO Layer 2)

---

## 1. Build Phases

### Phase 1: Foundation Setup (Week 1-2)

**Goal:** Core gateway infrastructure with minimal dependencies

#### 1.1 Project Structure

**Create:** `synaptix9-workflow-system/crates/sx9-gateway-primary/`

```
sx9-gateway-primary/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── server/
│   │   ├── mod.rs
│   │   ├── websocket.rs      # WebSocket server
│   │   ├── rest.rs           # REST API server
│   │   └── grpc.rs           # gRPC server
│   ├── routing/
│   │   ├── mod.rs
│   │   ├── neural_mux.rs     # RFC-9004 Neural Mux integration
│   │   └── foundation_manifold.rs  # Foundation Manifold routing
│   ├── hashing/
│   │   ├── mod.rs
│   │   └── trivariate.rs     # RFC-9001 trivariate hashing
│   ├── integrations/
│   │   ├── mod.rs
│   │   ├── usim.rs           # RFC-9008 USIM integration
│   │   ├── eei.rs            # EEI foundation crate
│   │   ├── foundation_manifold.rs
│   │   ├── foundation_math.rs
│   │   ├── government_data.rs
│   │   └── ops_main.rs       # RFC-9200 Ops-Main-Platform
│   ├── streaming/
│   │   ├── mod.rs
│   │   └── nats.rs           # NATS JetStream integration
│   ├── l2_execution/
│   │   ├── mod.rs
│   │   └── kali.rs           # RFC-9130, RFC-9876 L2 execution
│   ├── bernoulli/
│   │   ├── mod.rs
│   │   └── zone_classifier.rs  # RFC-9026 zone classification
│   └── config/
│       ├── mod.rs
│       └── gateway.toml
├── tests/
└── README.md
```

#### 1.2 Cargo.toml Dependencies

```toml
[package]
name = "sx9-gateway-primary"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core Foundation
ctas7-foundation-core = { path = "../../ctas-7-shipyard-staging/ctas7-foundation-core" }
ctas7-foundation-manifold = { path = "../../ctas-7-shipyard-staging/ctas7-foundation-manifold" }
ctas7-real-port-manager = { path = "../../ctas-7-shipyard-staging/ctas7-real-port-manager" }
sx9-atlas-bus = { path = "../sx9-atlas-bus" }

# Gateway Infrastructure
ctas7-neural-mux = { path = "../../ctas-7-shipyard-staging/ctas7-neural-mux" }
ctas7-cdn-data-fabric = { path = "../../ctas-7-shipyard-staging/ctas7-cdn-data-fabric" }
ctas7-nats-fabric = { path = "../../ctas-7-shipyard-staging/ctas7-nats-fabric" }

# Web Server
axum = { version = "0.7", features = ["ws", "macros"] }
tokio = { version = "1.39", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "compression", "trace"] }

# gRPC
tonic = "0.11"
prost = "0.12"

# NATS
async-nats = "0.33"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
surreal = { version = "1.5", features = ["kv-sled", "protocol-ws"] }

# Hashing (RFC-9001: Murmur3 only)
murmur3 = "0.5"  # NO Blake3, NO SHA256

# Utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.8", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }

[dev-dependencies]
tokio-test = "0.4"
```

#### 1.3 Core Gateway Server

**File:** `src/server/mod.rs`

```rust
//! SX9 Gateway Primary - Unified API Gateway
//! RFC-9114 Compliant Implementation

use axum::{
    extract::ws::WebSocketUpgrade,
    response::Response,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};

use crate::routing::GatewayRouter;
use crate::integrations::GatewayIntegrations;
use crate::streaming::NATSStreaming;
use crate::bernoulli::ZoneClassifier;

pub struct GatewayServer {
    router: Arc<GatewayRouter>,
    integrations: Arc<GatewayIntegrations>,
    streaming: Arc<NATSStreaming>,
    zone_classifier: Arc<ZoneClassifier>,
}

impl GatewayServer {
    pub fn new(
        router: Arc<GatewayRouter>,
        integrations: Arc<GatewayIntegrations>,
        streaming: Arc<NATSStreaming>,
        zone_classifier: Arc<ZoneClassifier>,
    ) -> Self {
        Self {
            router,
            integrations,
            streaming,
            zone_classifier,
        }
    }
    
    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/ws", get(websocket_handler))
            .route("/api/v1/query", post(rest_query))
            .route("/api/v1/execute", post(rest_execute))
            .with_state(self.clone());
        
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("🚀 sx9-gateway-primary listening on port {}", port);
        
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    state: axum::extract::State<GatewayServer>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: GatewayServer) {
    // WebSocket message handling
    // Route via GatewayRouter
    // Stream via NATS
}

async fn rest_query(
    axum::extract::State(state): axum::extract::State<GatewayServer>,
    axum::Json(payload): axum::Json<QueryRequest>,
) -> axum::Json<QueryResponse> {
    // REST query handling
    // Route via GatewayRouter
    // Return JSON response
}
```

---

### Phase 2: Routing & Hashing (Week 2-3)

**Goal:** Implement deterministic routing and trivariate hashing

#### 2.1 Neural Mux Integration (RFC-9004)

**File:** `src/routing/neural_mux.rs`

```rust
//! Neural Mux Routing - RFC-9004 Compliant
//! Target: <250ns routing decisions

use ctas7_neural_mux::NeuralMuxRouter;
use ctas7_foundation_core::TrivariateHash;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Instant;

pub struct GatewayNeuralMux {
    neural_mux: Arc<NeuralMuxRouter>,
    route_cache: Arc<DashMap<u16, RouteEntry>>,
}

impl GatewayNeuralMux {
    /// Route request - MUST complete in <250ns per RFC-9004 §3.3
    #[inline(always)]
    pub fn route(&self, hash: &TrivariateHash) -> Option<RouteEntry> {
        let start = Instant::now();
        
        // Extract SCH prefix for O(1) lookup
        let sch_prefix = (hash.sch >> 112) as u16;
        
        // O(1) lookup in lock-free DashMap
        if let Some(entry) = self.route_cache.get(&sch_prefix) {
            let elapsed = start.elapsed();
            if elapsed.as_nanos() > 250 {
                tracing::warn!("Routing exceeded 250ns: {:?}", elapsed);
            }
            return Some(entry.clone());
        }
        
        // Fallback to Neural Mux
        self.neural_mux.route(hash)
    }
}
```

#### 2.2 Trivariate Hashing (RFC-9001)

**File:** `src/hashing/trivariate.rs`

```rust
//! Trivariate Hashing - RFC-9001 Compliant
//! MUST use Murmur3-64 only (no Blake3, no SHA256)

use ctas7_foundation_core::{
    TrivariateHashEngine,
    murmur3_64_base96,
    seeds,
};
use std::io::Cursor;

pub struct GatewayTrivariateHasher {
    engine: TrivariateHashEngine,
}

impl GatewayTrivariateHasher {
    pub fn new() -> Self {
        Self {
            engine: TrivariateHashEngine::new(),
        }
    }
    
    /// Generate trivariate hash for gateway entity
    /// RFC-9001 §3.1: Every operational artifact MUST have a trivariate hash
    pub fn generate_hash(
        &self,
        content: &str,
        context: &str,
        entity_type: &str,
    ) -> TrivariateHash {
        // RFC-9001 §4.1: Murmur3-64 only
        let sch = murmur3_64_base96(
            &mut Cursor::new(content.as_bytes()),
            seeds::SCH,  // 0xC7A5_0000
        );
        
        let cuid = murmur3_64_base96(
            &mut Cursor::new(context.as_bytes()),
            seeds::CUID,  // 0xC7A5_0001
        );
        
        let uuid = self.engine.generate_uuid_v7();
        
        TrivariateHash {
            sch,
            cuid,
            uuid,
        }
    }
}
```

---

### Phase 3: System Integrations (Week 3-4)

**Goal:** Integrate USIM, EEI, Foundation Manifold, Foundation Math, Government Data Manifold

#### 3.1 USIM Integration (RFC-9008)

**File:** `src/integrations/usim.rs`

```rust
//! USIM Integration - RFC-9008 Ephemeral Engagement Rooms
//! Ephemeral intelligence with TTL

use ctas7_usim_system::USIMReactiveEngine;
use chrono::{DateTime, Utc};
use std::sync::Arc;

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

#### 3.2 EEI Integration

**File:** `src/integrations/eei.rs`

```rust
//! EEI Integration - Foundation Crate
//! EEI affects backplane/crystal decisions

use ctas7_eei_system::EEIProcessor;
use sx9_atlas_bus::PlasmaBackplane;
use std::sync::Arc;

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

#### 3.3 Foundation Manifold Integration

**File:** `src/integrations/foundation_manifold.rs`

```rust
//! Foundation Manifold Integration - RFC-9004
//! Route all foundation crates via Foundation Manifold

use ctas7_foundation_manifold::FoundationOrchestrator;
use std::sync::Arc;

pub struct FoundationManifoldIntegration {
    orchestrator: Arc<FoundationOrchestrator>,
}

impl FoundationManifoldIntegration {
    /// Route foundation crate request
    pub async fn route_foundation_crate(
        &self,
        crate_name: &str,
        request: &Request,
    ) -> Result<Response> {
        // Route via Foundation Manifold
        self.orchestrator.route_crate_request(crate_name, request).await
    }
}
```

#### 3.4 Foundation Math Integration

**File:** `src/integrations/foundation_math.rs`

```rust
//! Foundation Math Integration
//! Mathematical algorithms (replaces Wolfram Alpha)

use ctas7_foundation_math::MathematicalFoundationConsciousness;
use std::sync::Arc;

pub struct FoundationMathIntegration {
    math_engine: Arc<MathematicalFoundationConsciousness>,
}

impl FoundationMathIntegration {
    /// Execute mathematical computation
    pub async fn compute(&self, expression: &str) -> Result<SymbolicResult> {
        self.math_engine.symbolic_compute(expression).await
    }
}
```

#### 3.5 Government Data Manifold Integration

**File:** `src/integrations/government_data.rs`

```rust
//! Government Data Manifold Integration
//! Real-time government intelligence distribution

use ctas7_government_data_manifold::GovernmentDataManifold;
use async_nats::Client;
use std::sync::Arc;

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

#### 3.6 Ops-Main-Platform Integration

**File:** `src/integrations/ops_main.rs`

```rust
//! Ops-Main-Platform Integration - RFC-9200
//! React frontend bridge

use axum::extract::ws::WebSocket;
use std::sync::Arc;

pub struct OpsMainIntegration {
    websocket_server: Arc<WebSocketServer>,
}

impl OpsMainIntegration {
    /// Handle Ops-Main WebSocket connection
    pub async fn handle_websocket(&self, stream: TcpStream) -> Result<()> {
        self.websocket_server.handle(stream).await
    }
    
    /// Handle modal inventory (Playwright)
    pub async fn handle_modal_inventory(&self, inventory: ModalInventory) -> Result<()> {
        // Process Playwright modal inventory
        Ok(())
    }
}
```

---

### Phase 4: Streaming & L2 Execution (Week 4-5)

**Goal:** NATS JetStream integration and L2 execution support

#### 4.1 NATS JetStream Integration

**File:** `src/streaming/nats.rs`

```rust
//! NATS JetStream Integration
//! Unified streaming backbone

use async_nats::{Client, jetstream::Context};
use std::sync::Arc;

pub struct NATSStreaming {
    client: Arc<Client>,
    jetstream: Arc<Context>,
}

impl NATSStreaming {
    /// Publish to intel stream with time-of-value decay
    pub async fn publish_intel(
        &self,
        intel_type: IntelType,
        tier: Tier,
        data: &[u8],
    ) -> Result<()> {
        let subject = format!("sx9.stream.intel.{}.{}", intel_type, tier);
        self.jetstream.publish(&subject, data.into()).await?;
        Ok(())
    }
    
    /// Subscribe to operational stream
    pub async fn subscribe_ops(&self, category: &str) -> Result<()> {
        let subject = format!("sx9.stream.ops.{}", category);
        self.client.subscribe(&subject).await?;
        Ok(())
    }
}
```

#### 4.2 L2 Execution Integration

**File:** `src/l2_execution/kali.rs`

```rust
//! L2 Execution Integration - RFC-9130, RFC-9876
//! Unicode triggers, XDP/eBPF, hermetic execution

use aya::maps::perf::PerfEventArray;
use async_nats::Client;
use std::sync::Arc;

pub struct L2ExecutionIntegration {
    xdp_handler: Arc<XDPHandler>,      // RFC-9876
    nats_client: Arc<Client>,          // RFC-9130
    kali_orchestrator: Arc<KaliOrchestrator>,
}

impl L2ExecutionIntegration {
    /// Handle L2 Unicode trigger
    pub async fn handle_l2_trigger(&self, trigger: char) -> Result<()> {
        // Validate Unicode trigger (U+E000-F8FF)
        if !(0xE000..=0xF8FF).contains(&(trigger as u32)) {
            return Err(anyhow::anyhow!("Invalid L2 trigger"));
        }
        
        // Route to Kali orchestrator via NATS
        self.nats_client.publish(
            "sx9.l2.trigger",
            &L2Trigger { unicode: trigger }
        ).await?;
        
        Ok(())
    }
}
```

---

### Phase 5: Bernoulli Zone Compliance (Week 5)

**Goal:** Implement RFC-9026 Hourglass-Bernoulli compliance

#### 5.1 Zone Classifier

**File:** `src/bernoulli/zone_classifier.rs`

```rust
//! Bernoulli Zone Classifier - RFC-9026
//! NO LLMs in Bernoulli zone, <50μs latency

use std::time::Duration;

pub enum BernoulliZone {
    Tactical,      // < 50μs - NO LLMs
    Operational,   // 50μs - 1ms
    Analytical,    // 1ms - 100ms
    Infrastructure // 100ms - 60s
}

pub struct ZoneClassifier;

impl ZoneClassifier {
    /// Classify request into Bernoulli zone
    pub fn classify(&self, operation: &Operation) -> BernoulliZone {
        match operation.operation_type {
            OperationType::TrivariateHash => BernoulliZone::Tactical,
            OperationType::Routing => BernoulliZone::Tactical,
            OperationType::CognitiveTick => BernoulliZone::Operational,
            OperationType::GraphAnalysis => BernoulliZone::Analytical,
            OperationType::IACSpawn => BernoulliZone::Infrastructure,
        }
    }
    
    /// Check if LLM usage is allowed
    pub fn allows_llm(&self, zone: BernoulliZone) -> bool {
        match zone {
            BernoulliZone::Tactical => false,  // RFC-9026: NO LLMs
            _ => true,
        }
    }
}
```

---

### Phase 6: Testing & Validation (Week 6)

**Goal:** Comprehensive testing and RFC compliance verification

#### 6.1 Performance Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_routing_latency() {
        // RFC-9004: MUST complete in <250ns
        let router = GatewayRouter::new();
        let hash = generate_test_hash();
        
        let start = Instant::now();
        let _route = router.route(&hash).await;
        let elapsed = start.elapsed();
        
        assert!(elapsed.as_nanos() < 250, "Routing exceeded 250ns");
    }
    
    #[tokio::test]
    async fn test_bernoulli_zone_compliance() {
        // RFC-9026: NO LLMs in Tactical zone
        let classifier = ZoneClassifier::new();
        let zone = classifier.classify(&Operation::TrivariateHash);
        
        assert!(!classifier.allows_llm(zone), "LLM not allowed in Tactical zone");
    }
    
    #[tokio::test]
    async fn test_trivariate_hashing() {
        // RFC-9001: MUST use Murmur3-64
        let hasher = GatewayTrivariateHasher::new();
        let hash = hasher.generate_hash("test", "context", "entity");
        
        assert_eq!(hash.sch.len(), 16);  // Base96 encoded
        assert_eq!(hash.cuid.len(), 16);
        assert_eq!(hash.uuid.len(), 16);
    }
}
```

---

## 2. Build Dependencies

### 2.1 Foundation Crates (Must Build First)

**Order:**
1. `ctas7-foundation-core` — Trivariate hashing, PTCC primitives
2. `sx9-atlas-bus` — PlasmaState, crystal, SDT gate
3. `ctas7-foundation-manifold` — Deterministic routing
4. `ctas7-real-port-manager` — Port allocation
5. `ctas7-neural-mux` — <250ns routing

### 2.2 Integration Crates (Build in Parallel)

**Order:**
1. `ctas7-nats-fabric` — NATS messaging
2. `ctas7-cdn-data-fabric` — Multi-DB aggregation
3. `ctas7-wasm-primitives` — 32 Universal Primitives
4. `ctas7-cognitive-execution-tool` — Tool execution

### 2.3 Gateway Crate (Build Last)

**Order:**
1. `sx9-gateway-primary` — Main gateway (depends on all above)

---

## 3. Port Allocation

### 3.1 Gateway Ports

**Primary Gateway:**
- WebSocket: Port TBD (allocate via port-manager)
- REST API: Port TBD (allocate via port-manager)
- gRPC: Port TBD (allocate via port-manager)

**Port Manager Integration:**
- Use `ctas7-real-port-manager` (port 18104)
- All port allocations must go through port manager
- Crystal/thyristor gating for port assignments

### 3.2 Service Ports

**Foundation Services:**
- Port Manager: 18104 (fixed)
- Neural Mux: 18107 (fixed)
- ATLAS Daemon: 18106 (fixed)
- Foundation Manifold: TBD (dynamic)

**CDN Services:**
- Core CDN: 18112-18114 (fixed)
- Burst CDN: 18115-18119 (dynamic)
- Edge CDN: 18120-18129 (dynamic)

---

## 4. Deployment Strategy

### 4.1 Minimal Startup (Always Running)

**12 Crates:**
1. `sx9-gateway-primary` — Gateway core
2. `sx9-atlas-bus` — PlasmaState, crystal, SDT gate
3. `ctas7-neural-mux` — <250ns routing
4. `ctas7-real-port-manager` — Port allocation
5. `ctas7-cdn-data-fabric` — Multi-DB aggregation
6. `ctas7-nats-fabric` — NATS messaging
7. `ctas7-foundation-core` — Trivariate hash, PTCC primitives
8. `ctas7-foundation-manifold` — Deterministic routing
9. `ctas7-wasm-primitives` — 32 Universal Primitives
10. `ctas7-cognitive-execution-tool` — Tool execution
11. `tools/kali-plasma/agent` — Kali ISO agent
12. `ctas7-orchestrator` — Smart Crate Orchestrator (basic)

### 4.2 Escalation Components (Activate via NATS)

**14 Crates:**
- Agentic Infrastructure (4 crates) → NATS subject: `sx9.escalate.agent`
- Cognitive Computing (4 crates) → NATS subject: `sx9.escalate.cognitive`
- Defense Components (1 crate) → NATS subject: `sx9.escalate.defense`
- Supporting Infrastructure (3 crates) → NATS subject: `sx9.escalate.support`
- ML Models (2 components) → NATS subject: `sx9.escalate.ml`

---

## 5. Testing Strategy

### 5.1 Unit Tests

- Trivariate hashing (RFC-9001 compliance)
- Routing latency (<250ns, RFC-9004)
- Bernoulli zone classification (RFC-9026)
- USIM TTL management (RFC-9008)
- L2 trigger validation (RFC-9876)

### 5.2 Integration Tests

- Foundation Manifold routing
- NATS JetStream streaming
- Ops-Main-Platform WebSocket
- L2 execution pipeline
- Port Manager integration

### 5.3 Performance Tests

- Routing throughput (10M routes/sec)
- Hash generation (1M hashes/sec)
- WebSocket connections (100K concurrent)
- REST API requests (1M/sec)

---

## 6. Success Criteria

**Gateway Ready for Production When:**
- ✅ All RFC-9114 requirements met
- ✅ Routing <250ns (RFC-9004)
- ✅ Bernoulli zone compliance (RFC-9026)
- ✅ All system integrations working
- ✅ Performance targets met
- ✅ No Blake3/SHA256 (except USIM integrity)
- ✅ Production-ready code (no stubs, demos, hardcoded data)

---

## 7. Timeline

**Week 1-2:** Foundation Setup (Project structure, Cargo.toml, core server)  
**Week 2-3:** Routing & Hashing (Neural Mux, trivariate hashing)  
**Week 3-4:** System Integrations (USIM, EEI, Foundation Manifold, etc.)  
**Week 4-5:** Streaming & L2 Execution (NATS, Kali integration)  
**Week 5:** Bernoulli Zone Compliance (Zone classifier, LLM restrictions)  
**Week 6:** Testing & Validation (Unit, integration, performance tests)

**Total:** 6 weeks to production-ready gateway

---

**Status:** Build plan complete, ready for implementation



