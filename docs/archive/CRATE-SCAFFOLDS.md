# Crate Scaffolds - Intended Structure
## Complete Implementation Scaffolds for All Crates

**Date:** December 2025  
**Status:** Scaffold Definitions  
**Purpose:** Show intended structure before implementation

---

## 1. sx9-ann-engine Scaffold

### Directory Structure
```
sx9-ann-engine/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs              # Public API
    ├── observer.rs         # Observer mode implementation
    ├── entropy.rs          # Routing entropy tracking
    ├── weights.rs          # Weight map storage/loading
    └── config.rs           # Configuration
```

### lib.rs Structure
```rust
//! sx9-ann-engine - ANN Observer Mode for Neural Retrofit
//! 
//! RFC-9114 Rev 1.1: Observer mode only (no training)
//! - Observes routing entropy
//! - Stores weight maps
//! - Sync interval: 1000ms
//! - Entropy seed: murmur3-64

pub mod observer;
pub mod entropy;
pub mod weights;
pub mod config;

pub use observer::AnnObserver;
pub use entropy::RoutingEntropy;
pub use weights::WeightMap;
pub use config::AnnConfig;

/// ANN Engine - Observer Mode
pub struct AnnEngine {
    observer: AnnObserver,
    entropy: RoutingEntropy,
    weights: WeightMap,
    config: AnnConfig,
}

impl AnnEngine {
    /// Create new ANN engine in observer mode
    pub fn new(config: AnnConfig) -> Result<Self>;
    
    /// Observe routing decision
    pub fn observe_route(&mut self, route: &RouteDecision) -> Result<()>;
    
    /// Get current entropy
    pub fn get_entropy(&self) -> f64;
    
    /// Load weights from file
    pub fn load_weights(&mut self, path: &Path) -> Result<()>;
    
    /// Save weights to file
    pub fn save_weights(&self, path: &Path) -> Result<()>;
}
```

### observer.rs Structure
```rust
use sx9_atlas_bus::PlasmaState;
use ctas7_foundation_core::TrivariateHash;

pub struct AnnObserver {
    enabled: bool,
    mode: ObserverMode,
    sync_interval_ms: u64,
}

pub enum ObserverMode {
    Observe,  // Only observe, no training
}

impl AnnObserver {
    pub fn new(config: &AnnConfig) -> Self;
    
    pub fn observe(&mut self, event: &RoutingEvent) -> Result<()>;
    
    pub fn is_enabled(&self) -> bool;
}
```

### entropy.rs Structure
```rust
use murmur3::murmur3_64::MurmurHasher;
use std::hash::{Hash, Hasher};

pub struct RoutingEntropy {
    entropy_seed: String,  // "murmur3-64"
    current_entropy: f64,
    history: Vec<EntropySnapshot>,
}

impl RoutingEntropy {
    pub fn new(seed: &str) -> Self;
    
    pub fn calculate(&mut self, route: &RouteDecision) -> f64;
    
    pub fn get_current(&self) -> f64;
}
```

### weights.rs Structure
```rust
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightMap {
    version: String,
    weights: Vec<f64>,
    metadata: WeightMetadata,
}

impl WeightMap {
    pub fn load(path: &Path) -> Result<Self>;
    
    pub fn save(&self, path: &Path) -> Result<()>;
    
    pub fn default() -> Self;
}
```

---

## 2. sx9-glaf-core Scaffold

### Directory Structure
```
sx9-glaf-core/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs              # Public API
    ├── ann.rs              # ANN/GNN operations
    ├── topology.rs         # Topology mirroring
    ├── routing.rs           # Routing feedback
    └── config.rs           # Configuration
```

### lib.rs Structure
```rust
//! sx9-glaf-core - GLAF Neural Operations
//! 
//! Purpose: ANN/GNN operations + topology mirroring
//! - ANN/GNN model inference
//! - Topology mirroring (SlotGraph observation)
//! - Routing feedback

pub mod ann;
pub mod topology;
pub mod routing;
pub mod config;

pub use ann::GlafAnn;
pub use topology::TopologyMirror;
pub use routing::RoutingFeedback;
pub use config::GlafConfig;

pub struct GlafCore {
    ann: GlafAnn,
    topology: TopologyMirror,
    routing: RoutingFeedback,
    config: GlafConfig,
}

impl GlafCore {
    pub fn new(config: GlafConfig) -> Result<Self>;
    
    /// Run ANN/GNN inference
    pub fn infer(&self, input: &InferenceInput) -> Result<InferenceOutput>;
    
    /// Mirror SlotGraph topology
    pub fn mirror_topology(&mut self) -> Result<()>;
    
    /// Get routing feedback
    pub fn get_routing_feedback(&self) -> RoutingFeedback;
}
```

### topology.rs Structure
```rust
use ctas7_slotgraph_engine::SlotGraph;
use sx9_atlas_bus::PlasmaState;

pub struct TopologyMirror {
    slotgraph: Arc<SlotGraph>,
    mirror_enabled: bool,
    sync_interval_ms: u64,
    last_sync: Instant,
}

impl TopologyMirror {
    pub fn new(slotgraph: Arc<SlotGraph>, config: &GlafConfig) -> Self;
    
    /// Mirror SlotGraph routing topology
    pub fn mirror(&mut self) -> Result<TopologySnapshot>;
    
    /// Get current topology state
    pub fn get_topology(&self) -> &TopologySnapshot;
}
```

---

## 3. GLAFClient Scaffold

### File Structure
```
sx9-gateway-primary/src/
└── glaf_client.rs
```

### glaf_client.rs Structure
```rust
//! GLAF Client - HTTP client to GLAF Graph Server
//! 
//! Purpose: Data analytics, graph queries, analytic workbench
//! Server: ctas7-glaf-graph-server (port 18050)

use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct GLAFClient {
    client: Client,
    base_url: String,  // http://localhost:18050
}

#[derive(Debug, Serialize)]
struct QueryRequest {
    surql: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryResult {
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
    /// Create new GLAF client
    pub async fn new(base_url: impl Into<String>) -> Result<Self>;
    
    /// Health check
    pub async fn health(&self) -> Result<bool>;
    
    /// Query graph with SurrealQL
    pub async fn query_graph(&self, surql: &str) -> Result<QueryResult>;
    
    /// Get nodes with filters
    pub async fn get_nodes(
        &self,
        limit: Option<usize>,
        hd4_phase: Option<&str>,
        label: Option<&str>,
    ) -> Result<Vec<GlafNode>>;
    
    /// Calculate matroid rank
    pub async fn calculate_matroid_rank(&self, fragment_ids: &[String]) -> Result<MatroidResult>;
    
    /// Calculate Hawkes intensity
    pub async fn calculate_hawkes(&self, event_type: &str, window_hours: f64) -> Result<HawkesResult>;
    
    /// Calculate convergence
    pub async fn calculate_convergence(&self, h1_hash: &str, h2_hash: &str) -> Result<ConvergenceResult>;
}
```

---

## 4. sx9-dsl-engine Scaffold

### Directory Structure
```
sx9-dsl-engine/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs              # Public API
    ├── runtime.rs           # WASM runtime
    ├── parser.rs            # DSL parser
    ├── executor.rs          # Playbook executor
    └── config.rs            # Configuration
```

### lib.rs Structure
```rust
//! sx9-dsl-engine - DSL Symbolic Control
//! 
//! Purpose: DSL runtime for symbolic control
//! - Symbolic language runtime
//! - WASM execution
//! - Playbook execution
//! - Reload on change

pub mod runtime;
pub mod parser;
pub mod executor;
pub mod config;

pub use runtime::DslRuntime;
pub use parser::DslParser;
pub use executor::PlaybookExecutor;
pub use config::DslConfig;

pub struct DslEngine {
    runtime: DslRuntime,
    parser: DslParser,
    executor: PlaybookExecutor,
    config: DslConfig,
}

impl DslEngine {
    pub fn new(config: DslConfig) -> Result<Self>;
    
    /// Execute DSL playbook
    pub async fn execute_playbook(&self, playbook: &str) -> Result<PlaybookResult>;
    
    /// Reload DSL runtime
    pub fn reload(&mut self) -> Result<()>;
}
```

### runtime.rs Structure
```rust
use wasmtime::*;

pub struct DslRuntime {
    engine: Engine,
    store: Store<()>,
    module: Option<Module>,
    reload_on_change: bool,
}

impl DslRuntime {
    pub fn new(config: &DslConfig) -> Result<Self>;
    
    pub fn load_module(&mut self, wasm_bytes: &[u8]) -> Result<()>;
    
    pub async fn execute(&mut self, function: &str, args: &[Value]) -> Result<Vec<Value>>;
}
```

---

## 5. sx9-plasma-defender Scaffold

### Directory Structure
```
sx9-plasma-defender/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs              # Public API
    ├── server.rs           # Axum server
    ├── health.rs            # Health monitoring
    ├── metrics.rs           # Metrics collection
    ├── agents.rs            # Threat monitoring agents
    ├── crystal.rs           # Crystal resonance integration
    ├── sdt.rs               # SDT gate integration
    ├── monitor.rs           # Threat monitoring system
    └── config.rs            # Configuration
```

### lib.rs Structure
```rust
//! sx9-plasma-defender - PLASMA Defender with Crystal & SDT Integration
//! 
//! Purpose: PLASMA defender for health monitoring and threat detection
//! - Health endpoint (/health)
//! - Metrics endpoint (/metrics)
//! - Latency enforcement
//! - PLASMA state monitoring
//! - Crystal resonance evaluation
//! - SDT gate control
//! - Threat monitoring agents

pub mod server;
pub mod health;
pub mod metrics;
pub mod agents;
pub mod crystal;
pub mod sdt;
pub mod monitor;
pub mod config;

pub use server::PlasmaDefenderServer;
pub use health::HealthMonitor;
pub use metrics::MetricsCollector;
pub use agents::ThreatAgent;
pub use crystal::CrystalIntegration;
pub use sdt::SdtIntegration;
pub use monitor::ThreatMonitor;
pub use config::DefenderConfig;

pub struct PlasmaDefender {
    server: PlasmaDefenderServer,
    health: HealthMonitor,
    metrics: MetricsCollector,
    agents: Vec<ThreatAgent>,
    crystal: CrystalIntegration,
    sdt: SdtIntegration,
    monitor: ThreatMonitor,
    config: DefenderConfig,
}

impl PlasmaDefender {
    pub fn new(config: DefenderConfig) -> Result<Self>;
    
    pub async fn start(&self) -> Result<()>;
    
    pub fn get_health(&self) -> HealthStatus;
    
    pub fn get_metrics(&self) -> Metrics;
    
    /// Evaluate threat through crystal resonance
    pub async fn evaluate_threat(&self, payload: &[u8]) -> ThreatResult;
    
    /// Check SDT gate state
    pub fn get_sdt_state(&self) -> SdtState;
}
```

### agents.rs Structure
```rust
use sx9_atlas_bus::PlasmaState;
use std::sync::Arc;

pub struct ThreatAgent {
    agent_id: String,
    agent_type: AgentType,
    plasma: Arc<PlasmaState>,
    enabled: bool,
}

pub enum AgentType {
    NetworkMonitor,    // Monitor network traffic
    ThreatHunter,      // Hunt for threats
    CanaryWatcher,     // Watch canary triggers
    AnomalyDetector,   // Detect anomalies
}

impl ThreatAgent {
    pub fn new(agent_id: String, agent_type: AgentType, plasma: Arc<PlasmaState>) -> Self;
    
    pub async fn monitor(&mut self) -> Result<ThreatEvent>;
    
    pub fn is_enabled(&self) -> bool;
}
```

### crystal.rs Structure
```rust
use sx9_atlas_bus::{Crystal, CrystalFamily, Polycrystal, PolycrystalResult};
use std::sync::Arc;

pub struct CrystalIntegration {
    polycrystal: Arc<Polycrystal>,
    crystal_family: CrystalFamily,
}

impl CrystalIntegration {
    pub fn new(family: CrystalFamily) -> Self {
        let polycrystal = Polycrystal::new(family);
        Self {
            polycrystal: Arc::new(polycrystal),
            crystal_family: family,
        }
    }
    
    /// Evaluate payload through crystal resonance
    pub fn evaluate(&self, payload: &[u8], delta_angle: u16) -> PolycrystalResult {
        self.polycrystal.resonate_payload(payload, delta_angle)
    }
    
    /// Get ring strength for threat evaluation
    pub fn get_ring_strength(&self, payload: &[u8], delta_angle: u16) -> f32 {
        let result = self.evaluate(payload, delta_angle);
        result.ring_strength
    }
}
```

### sdt.rs Structure
```rust
use sx9_atlas_bus::{PlasmaState, SdtState, ThyristorConfig};
use std::sync::Arc;

pub struct SdtIntegration {
    plasma: Arc<PlasmaState>,
    config: ThyristorConfig,
}

impl SdtIntegration {
    pub fn new(plasma: Arc<PlasmaState>, config: ThyristorConfig) -> Self {
        Self { plasma, config }
    }
    
    /// Get current SDT gate state
    pub fn get_state(&self) -> SdtState {
        self.plasma.sdt_state()
    }
    
    /// Evaluate if command should proceed (crystal + SDT gate)
    pub fn should_proceed(&self, ring_strength: f32) -> bool {
        // Check if SDT gate allows flow
        let state = self.get_state();
        matches!(state, SdtState::Conducting | SdtState::Latched)
    }
    
    /// Reset SDT gate (requires auth)
    pub fn reset(&self, auth_token: &[u8]) -> Result<()>;
}
```

### monitor.rs Structure
```rust
use sx9_atlas_bus::PlasmaState;
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub struct ThreatMonitor {
    agents: Vec<ThreatAgent>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    plasma: Arc<PlasmaState>,
    monitor_interval: Duration,
}

impl ThreatMonitor {
    pub fn new(
        agents: Vec<ThreatAgent>,
        crystal: Arc<CrystalIntegration>,
        sdt: Arc<SdtIntegration>,
        plasma: Arc<PlasmaState>,
    ) -> Self {
        Self {
            agents,
            crystal,
            sdt,
            plasma,
            monitor_interval: Duration::from_millis(100), // 100ms tick
        }
    }
    
    pub async fn run(&mut self) -> Result<()> {
        let mut ticker = interval(self.monitor_interval);
        
        loop {
            ticker.tick().await;
            
            // Monitor threats from agents
            for agent in &mut self.agents {
                if let Ok(event) = agent.monitor().await {
                    // Evaluate through crystal
                    let ring_strength = self.crystal.get_ring_strength(
                        &event.payload,
                        self.plasma.delta_angle_raw(),
                    );
                    
                    // Check SDT gate
                    if self.sdt.should_proceed(ring_strength) {
                        // Threat detected and allowed by SDT
                        self.handle_threat(event).await?;
                    } else {
                        // SDT gate blocked - threat killed
                        self.block_threat(event).await?;
                    }
                }
            }
        }
    }
    
    async fn handle_threat(&self, event: ThreatEvent) -> Result<()>;
    
    async fn block_threat(&self, event: ThreatEvent) -> Result<()>;
}
```

### server.rs Structure
```rust
use axum::{Router, routing::get, Json};
use sx9_atlas_bus::PlasmaState;
use std::sync::Arc;

pub struct PlasmaDefenderServer {
    router: Router,
    health_endpoint: String,
    metrics_endpoint: String,
    plasma: Arc<PlasmaState>,
}

impl PlasmaDefenderServer {
    pub fn new(config: &DefenderConfig, plasma: Arc<PlasmaState>) -> Self {
        let router = Router::new()
            .route("/health", get(health_handler))
            .route("/metrics", get(metrics_handler))
            .route("/sdt/state", get(sdt_state_handler))
            .route("/crystal/resonance", get(crystal_resonance_handler));
        
        Self {
            router,
            health_endpoint: config.health_endpoint.clone(),
            metrics_endpoint: config.metrics_endpoint.clone(),
            plasma,
        }
    }
    
    pub async fn start(&self, addr: &str) -> Result<()>;
}

async fn sdt_state_handler(State(plasma): State<Arc<PlasmaState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "sdt_state": format!("{:?}", plasma.sdt_state()),
        "delta_angle": plasma.delta_angle_raw(),
        "entropy": plasma.entropy_raw(),
        "excited": plasma.excited(),
    }))
}

async fn crystal_resonance_handler(State(plasma): State<Arc<PlasmaState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ring_strength": plasma.last_ring_strength(),
        "excited": plasma.excited(),
    }))
}
```

---

## 6. sx9-atlas-daemon Scaffold

### Directory Structure
```
sx9-atlas-daemon/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── main.rs             # Binary entry point
    ├── daemon.rs            # Daemon implementation
    ├── ooda.rs              # OODA loop
    └── config.rs            # Configuration
```

### main.rs Structure
```rust
//! sx9-atlas-daemon - ATLAS Cognitive Tick
//! 
//! Purpose: ATLAS daemon for cognitive operations
//! - 1ms tick interval
//! - OODA loop (observe, orient, decide, act)
//! - Phase sequence execution

use clap::Parser;
use sx9_atlas_daemon::{Daemon, DaemonConfig};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "1")]
    tick_interval_ms: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let config = DaemonConfig {
        tick_interval_ms: args.tick_interval_ms,
        phase_sequence: vec!["observe", "orient", "decide", "act"],
    };
    
    let daemon = Daemon::new(config).await?;
    daemon.run().await?;
    
    Ok(())
}
```

### daemon.rs Structure
```rust
use sx9_atlas_bus::AtlasBus;
use tokio::time::{interval, Duration};

pub struct Daemon {
    bus: AtlasBus,
    tick_interval: Duration,
    phase_sequence: Vec<String>,
    ooda: OodaLoop,
}

impl Daemon {
    pub async fn new(config: DaemonConfig) -> Result<Self>;
    
    pub async fn run(&mut self) -> Result<()> {
        let mut ticker = interval(self.tick_interval);
        
        loop {
            ticker.tick().await;
            
            // Execute OODA loop
            self.ooda.observe().await?;
            self.ooda.orient().await?;
            self.ooda.decide().await?;
            self.ooda.act().await?;
        }
    }
}
```

---

## 7. sx9-plasma-ecs Scaffold

### Directory Structure
```
sx9-plasma-ecs/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs              # Public API
    ├── layer1.rs            # apecs (Async I/O)
    ├── layer2.rs            # Legion (Deterministic Batch)
    ├── layer3.rs            # ATLAS (Cognitive)
    ├── plasma_state.rs      # Unified PlasmaState
    └── config.rs            # Configuration
```

### lib.rs Structure
```rust
//! sx9-plasma-ecs - PLASMA-ECS Unified Architecture
//! 
//! Purpose: Three-layer ECS architecture
//! - Layer 1: apecs (Async I/O)
//! - Layer 2: Legion (Deterministic Batch) + Slot Graph
//! - Layer 3: ATLAS (Cognitive)

pub mod layer1;
pub mod layer2;
pub mod layer3;
pub mod plasma_state;
pub mod config;

pub use layer1::ApecsLayer;
pub use layer2::LegionLayer;
pub use layer3::AtlasLayer;
pub use plasma_state::UnifiedPlasmaState;
pub use config::PlasmaEcsConfig;

pub struct PlasmaEcs {
    layer1: ApecsLayer,  // Async I/O
    layer2: LegionLayer, // Deterministic Batch + Slot Graph
    layer3: AtlasLayer,  // Cognitive
    plasma_state: UnifiedPlasmaState,
    config: PlasmaEcsConfig,
}

impl PlasmaEcs {
    pub fn new(config: PlasmaEcsConfig) -> Result<Self>;
    
    /// Execute in Layer 1 (async I/O)
    pub async fn execute_async(&self, task: AsyncTask) -> Result<()>;
    
    /// Execute in Layer 2 (deterministic batch)
    pub fn execute_batch(&self, tasks: Vec<BatchTask>) -> Result<()>;
    
    /// Execute in Layer 3 (cognitive)
    pub async fn execute_cognitive(&self, command: CognitiveCommand) -> Result<()>;
    
    /// Get unified PlasmaState
    pub fn get_plasma_state(&self) -> &UnifiedPlasmaState;
}
```

### layer2.rs Structure
```rust
use legion::*;
use ctas7_slotgraph_engine::SlotGraph;

pub struct LegionLayer {
    world: World,
    schedule: Schedule,
    slotgraph: Arc<SlotGraph>,
}

impl LegionLayer {
    pub fn new(slotgraph: Arc<SlotGraph>) -> Self;
    
    /// Execute Slot Graph hash/unicode routing
    pub fn route_hash(&self, hash: &TrivariateHash) -> Result<UnicodeRune>;
    
    /// Execute Task > Skill > Tool Chain > Tool hierarchy
    pub fn execute_hierarchy(&self, task: Task) -> Result<()>;
}
```

---

## Summary

**All Scaffolds Defined:**
- ✅ Module structure
- ✅ Public API interfaces
- ✅ Key functions and methods
- ✅ Integration points
- ✅ Configuration structures

**Next:** Implement each crate according to these scaffolds.

---

**Status:** Scaffolds complete. Ready for implementation.

