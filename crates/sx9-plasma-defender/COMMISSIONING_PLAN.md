# PLASMA-DEFENDER COMMISSIONING PLAN

**Date:** December 16, 2025  
**Target:** Full operational status with OSSEC integration, agent implementation, and intel correlation  
**Estimated Effort:** 3-4 focused sessions  

---

## 🎯 COMPACTION CHECKPOINT (Session 1 Complete)

### Implemented Components (Dec 16, 2025)

| Phase | Component | Files | Status |
|-------|-----------|-------|--------|
| **0** | ECS Integration | `src/ecs/*.rs` (6 files) | ✅ COMPLETE |
| **0** | Ring Bus + JetStream | `src/ring_bus.rs` | ✅ COMPLETE |
| **0** | ATLAS Daemon | `src/atlas_integration.rs` | ✅ COMPLETE |
| **1** | OSSEC Integration | `src/ossec/*.rs` (4 files) | ✅ COMPLETE |
| **2** | Threat Agents | `src/agents.rs` | ✅ COMPLETE |
| **3** | EEI Bridge | `src/bridges/eei_bridge.rs` | ✅ COMPLETE |
| **3** | SlotGraph Bridge | `src/bridges/slot_bridge.rs` | ✅ COMPLETE |
| **4** | Testing | Unit tests in modules | ✅ COMPLETE (11 tests pass) |

### Key Structures Implemented

**ECS (src/ecs/):**
- `ThreatEntityComponent`, `OssecAlertComponent`, `Hd4PhaseComponent`
- `CrystalEvalComponent`, `SdtGateComponent`, `EeiCorrelationComponent`
- `ToolOutputComponent`, `ThreatObserverComponent`, `AgentComponent`
- `DefenderLegionWorld`, `DefenderApecsWorld`, `DefenderWorld`

**Ring Bus (src/ring_bus.rs):**
- JetStream streams: `SX9_DEFENDER_THREATS`, `SX9_DEFENDER_TOOL_OUTPUTS`, `SX9_DEFENDER_OSSEC`, `SX9_DEFENDER_EEI`
- `RingBusNode`, `ThreatEvent`, `ToolOutput`, `OssecAlert`, `RingMessage`

**ATLAS (src/atlas_integration.rs):**
- `DefenderAtlas` with 1ms OODA loop
- `ThreatObservation`, `ThreatDecision`, `ThreatAction`
- HD4 phase transitions, Crystal resonance evaluation

**OSSEC (src/ossec/):**
- `MitreMapping` with 32 Primitives
- `OssecAlertParser` for JSON alerts
- `OssecAgent` for NATS consumption

**Agents (src/agents.rs):**
- `NetworkMonitor` - Delta angle variance detection
- `ThreatHunter` - SDT state, entropy, supersession tracking
- `CanaryWatcher` - Trigger count, plasma excitation
- `AnomalyDetector` - Entropy z-score analysis

**Bridges (src/bridges/):**
- `EeiBridge` - Leptose EEI queries via NATS
- `EeiQueryRequest`, `EeiQueryResponse` - Query/response types
- `SlotBridge` - SlotGraph ECS queries via HTTP
- `SlotQuery`, `SlotEntity`, `SlotRelationship` - Query/entity types

### Dependencies Added to Cargo.toml

```toml
legion = "0.4"
sx9-atlas-daemon = { path = "../sx9-atlas-daemon" }
reqwest = { version = "0.11", features = ["json"] }
rand = "0.8"
```

### Compilation Status: ✅ PASSES (15 warnings)
### Test Status: ✅ 11 TESTS PASS

### Files Created This Session

```
src/ecs/
├── mod.rs           # ECS module exports
├── components.rs    # 12 threat-specific components
├── legion_layer.rs  # Legion hot-path world
├── apecs_layer.rs   # apecs async I/O world
├── systems.rs       # Threat processing systems
└── world.rs         # Unified DefenderWorld

src/ring_bus.rs      # Ring Bus + JetStream integration

src/atlas_integration.rs  # ATLAS daemon with OODA loop

src/ossec/
├── mod.rs           # OSSEC module exports
├── mitre_map.rs     # MITRE ATT&CK → 32 Primitives
├── alert_parser.rs  # OSSEC JSON alert parsing
└── ossec_agent.rs   # OSSEC NATS consumer

src/agents.rs        # Fully implemented threat agents

src/bridges/
├── mod.rs           # Bridges module exports
├── eei_bridge.rs    # EEI/Leptose integration
└── slot_bridge.rs   # SlotGraph ECS integration
```

### Total New Code: ~4,200+ lines of Rust

---

## NEXT SESSION TASKS

1. **Wire up PlasmaDefender main struct** with new components (ECS, ATLAS, Ring Bus, bridges)
2. **Add API endpoints** for ECS world status, ATLAS status, Ring Bus stats
3. **Integration test** with real NATS server
4. **Connect to running Leptose** for live EEI queries
5. **Performance tuning** for 1ms ATLAS tick compliance

---

## CURRENT STATE ASSESSMENT

### ✅ Operational Components
| Component | File | Status |
|-----------|------|--------|
| PlasmaDefender | `lib.rs` | Core orchestrator working |
| PlasmaDefenderServer | `server.rs` | Axum endpoints functional |
| PlasmaBus | `plasma_bus.rs` | NATS telemetry working |
| CrystalIntegration | `crystal.rs` | Ring strength evaluation working |
| SdtIntegration | `sdt.rs` | Gate control working |
| AnnDaemon | `ann_daemon.rs` | Advisory generation working |
| HealthMonitor | `health.rs` | Health checks working |
| MetricsCollector | `metrics.rs` | Metrics collection working |

### ❌ Stub/Missing Components
| Component | File | Issue |
|-----------|------|-------|
| ThreatAgent | `agents.rs` | All 4 agent methods return `Ok(None)` |
| OssecAgent | N/A | Not implemented |
| SlotGraph Bridge | N/A | Not connected |
| EEI Bridge | N/A | Not connected |
| MITRE Mapping | N/A | No rule-to-technique mapping |
| **ECS Integration** | N/A | **NOT using sx9-plasma-ecs (Legion/apecs)** |

### ⚠️ CRITICAL GAP: ECS Integration Missing

Per RFC-9116, Plasma-Defender should integrate with the three-layer ECS stack:

```
LAYER 3: ATLAS (Cognitive)     ← PlasmaDefender uses sx9-atlas-bus ✅
LAYER 2: Legion (Hot-Path)     ← NOT CONNECTED ❌
LAYER 1: apecs (Async I/O)     ← NOT CONNECTED ❌
```

The `sx9-plasma-ecs` crate exists with:
- `LegionPlasmaWorld` - Legion ECS world
- `ApecsPlasmaWorld` - apecs ECS world  
- `PlasmaComponent` - Delta angle, entropy, SDT state, crystal family
- `ThreatAgentComponent` - Agent ID, type, threat level
- `AnnObserverComponent` - Neural pattern tracking
- `AnnNeuronComponent` - Neuron activations

**But Plasma-Defender doesn't use any of it!**

---

## PHASE 0: ECS INTEGRATION (Session 0 - PREREQUISITE)

### 0.1 Copy ECS Layer from sx9-plasma-ecs

Plasma-Defender needs its own ECS world for threat entities. Copy and adapt from `sx9-plasma-ecs`:

**Files to copy/adapt:**
```
sx9-plasma-ecs/src/           →  sx9-plasma-defender/src/ecs/
├── components.rs             →  ecs/components.rs (adapt for threats)
├── legion_layer.rs           →  ecs/legion_layer.rs
├── apecs_layer.rs            →  ecs/apecs_layer.rs  
├── ann_layer.rs              →  ecs/ann_layer.rs (already have ann_daemon.rs)
├── systems.rs                →  ecs/systems.rs (adapt for threat processing)
└── world.rs                  →  ecs/world.rs
```

### 0.2 Add Threat-Specific ECS Components

```rust
// src/ecs/components.rs

use serde::{Deserialize, Serialize};

/// Threat entity component (Legion Layer 2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEntityComponent {
    pub entity_id: u64,
    pub threat_hash: u64,           // Trivariate hash
    pub unicode_trigger: u32,       // U+E000-E9FF
    pub primitive_bitfield: u64,    // 32 primitives
    pub speed_class: u8,            // Hot/Warm/Cold
    pub slot_id: u64,               // SlotGraph slot
}

/// OSSEC alert component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssecAlertComponent {
    pub rule_id: u32,
    pub level: u8,
    pub mitre_technique: Option<String>,
    pub mitre_tactic: Option<String>,
    pub timestamp: u64,
}

/// HD4 phase component
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Hd4PhaseComponent {
    pub phase: Hd4Phase,
    pub progress: f64,              // 0.0-1.0 within phase
    pub delta_x: f64,               // Semantic (MITRE stage)
    pub delta_y: f64,               // Operational (HD4)
    pub delta_z: f64,               // Temporal
}

/// Crystal evaluation component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalEvalComponent {
    pub ring_strength: f32,
    pub resonance_allowed: bool,
    pub sdt_state: SdtState,
}

/// EEI correlation component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiCorrelationComponent {
    pub query_id: Option<String>,
    pub correlation_score: f64,
    pub related_tasks: Vec<String>,  // SlotGraph task UUIDs
}
```

### 0.3 Create Defender-Specific ECS World

```rust
// src/ecs/world.rs

use crate::ecs::components::*;
use legion::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Plasma Defender ECS World
/// Manages threat entities through Legion (hot-path) and apecs (async I/O)
pub struct DefenderWorld {
    legion: Arc<RwLock<World>>,
    entities: Arc<RwLock<HashMap<u64, Entity>>>,
    next_id: Arc<RwLock<u64>>,
}

impl DefenderWorld {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            legion: Arc::new(RwLock::new(World::default())),
            entities: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        })
    }

    /// Add threat entity to ECS world
    pub async fn add_threat(&self, threat: ThreatEntityComponent) -> anyhow::Result<u64> {
        let mut world = self.legion.write().await;
        let mut entities = self.entities.write().await;
        let mut next_id = self.next_id.write().await;

        let entity_id = *next_id;
        *next_id += 1;

        let entity = world.push((
            threat,
            Hd4PhaseComponent::default(),
            CrystalEvalComponent::default(),
            EeiCorrelationComponent::default(),
        ));
        
        entities.insert(entity_id, entity);
        Ok(entity_id)
    }

    /// Add OSSEC alert as entity
    pub async fn add_ossec_alert(&self, alert: OssecAlertComponent) -> anyhow::Result<u64> {
        let threat = ThreatEntityComponent {
            entity_id: 0,  // Will be set
            threat_hash: Self::hash_alert(&alert),
            unicode_trigger: Self::alert_to_trigger(&alert),
            primitive_bitfield: 0,
            speed_class: 1,  // Warm
            slot_id: 0,
        };
        
        let mut world = self.legion.write().await;
        let mut entities = self.entities.write().await;
        let mut next_id = self.next_id.write().await;

        let entity_id = *next_id;
        *next_id += 1;

        let entity = world.push((
            threat,
            alert,
            Hd4PhaseComponent::default(),
            CrystalEvalComponent::default(),
        ));
        
        entities.insert(entity_id, entity);
        Ok(entity_id)
    }

    /// Run threat processing systems
    pub async fn tick(&self) -> anyhow::Result<()> {
        // Run Legion systems for threat evaluation
        let world = self.legion.read().await;
        
        // Query all threats needing evaluation
        let mut query = <(&ThreatEntityComponent, &mut CrystalEvalComponent)>::query();
        
        // Systems would process here
        Ok(())
    }

    fn hash_alert(alert: &OssecAlertComponent) -> u64 {
        // Generate trivariate hash from alert
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        alert.rule_id.hash(&mut hasher);
        alert.timestamp.hash(&mut hasher);
        hasher.finish()
    }

    fn alert_to_trigger(alert: &OssecAlertComponent) -> u32 {
        // Map OSSEC level to Unicode trigger range
        match alert.level {
            0..=3 => 0xE500,   // Low - intelligence primitive
            4..=7 => 0xE508,   // Medium - defensive primitive
            8..=11 => 0xE510,  // High - offensive primitive
            12..=15 => 0xE51C, // Critical - control primitive
            _ => 0xE500,
        }
    }
}
```

### 0.4 Integrate ATLAS Daemon

```rust
// src/atlas_integration.rs

use sx9_atlas_bus::{AtlasBus, AtlasCommand, AtlasResponse, PlasmaState};
use crate::ecs::world::DefenderWorld;
use std::sync::Arc;
use tokio::sync::RwLock;

/// ATLAS Daemon integration for Plasma Defender
/// Provides 1ms OODA loop cognitive processing
pub struct AtlasIntegration {
    bus: Arc<AtlasBus>,
    plasma: Arc<PlasmaState>,
    world: Arc<DefenderWorld>,
    tick_interval_ms: u64,
}

impl AtlasIntegration {
    pub fn new(
        plasma: Arc<PlasmaState>,
        world: Arc<DefenderWorld>,
    ) -> anyhow::Result<Self> {
        let bus = Arc::new(AtlasBus::new()?);
        Ok(Self {
            bus,
            plasma,
            world,
            tick_interval_ms: 1,  // 1ms OODA loop
        })
    }

    /// Run ATLAS cognitive loop
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_millis(self.tick_interval_ms)
        );

        loop {
            interval.tick().await;
            
            // OODA Loop
            // 1. Observe - Read plasma state
            let entropy = self.plasma.entropy();
            let delta = self.plasma.delta_angle_raw();
            
            // 2. Orient - Calculate threat context
            let threat_level = self.calculate_threat_level(entropy, delta);
            
            // 3. Decide - Crystal resonance check
            let allowed = self.crystal_decision(threat_level);
            
            // 4. Act - Update ECS world
            self.world.tick().await?;
            
            // Publish tick telemetry
            self.bus.publish_tick(entropy, delta, allowed).await?;
        }
    }

    fn calculate_threat_level(&self, entropy: u32, delta: u16) -> f32 {
        // Normalize and combine
        let e = entropy as f32 / 1000.0;
        let d = delta as f32 / 65535.0;
        (e * 0.6 + d * 0.4).min(1.0)
    }

    fn crystal_decision(&self, threat_level: f32) -> bool {
        // SDT gate threshold
        threat_level < 0.7
    }
}
```

### 0.5 Update Cargo.toml Dependencies

```toml
# Add to Cargo.toml
[dependencies]
# ECS
legion.workspace = true

# Existing
sx9-foundation-core = { path = "../sx9-foundation-core" }
sx9-atlas-bus = { path = "../sx9-atlas-bus", features = ["std", "nats"] }
```

### 0.6 Update lib.rs Structure

```rust
// src/lib.rs - Updated structure

pub mod advisory;
pub mod agents;
pub mod ann_daemon;
pub mod config;
pub mod crystal;
pub mod health;
pub mod metrics;
pub mod monitor;
pub mod plasma_bus;
pub mod sdt;
pub mod server;
pub mod tool_handler;

// NEW: ECS Integration
pub mod ecs;
pub mod atlas_integration;

pub use ecs::world::DefenderWorld;
pub use ecs::components::*;
pub use atlas_integration::AtlasIntegration;
```

### 0.7 Wire ECS into PlasmaDefender

```rust
// Update PlasmaDefender struct in lib.rs

pub struct PlasmaDefender {
    server: PlasmaDefenderServer,
    health: HealthMonitor,
    metrics: MetricsCollector,
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    monitor: Arc<RwLock<ThreatMonitor>>,
    plasma: Arc<PlasmaState>,
    plasma_bus: Arc<PlasmaBus>,
    ann_daemon: Arc<AnnDaemon>,
    config: DefenderConfig,
    
    // NEW: ECS Integration
    ecs_world: Arc<DefenderWorld>,
    atlas: Arc<AtlasIntegration>,
    
    // NEW: Ring Bus + JetStream
    ring_bus: Arc<RingBusNode>,
    nats_bridge: Arc<NatsBridge>,
}
```

### 0.8 Ring Bus Integration (RFC-9301)

**Current State:** `PlasmaBus` is basic pub/sub with no JetStream or Ring Bus

**Required:** Full Ring Bus node with JetStream persistence for tool outputs

```rust
// src/ring_bus.rs

use sx9_atlas_bus::bridge::{NatsBridge, NatsBridgeConfig, WireCommand, WireResult};
use async_nats::jetstream;
use std::sync::Arc;

/// Ring Bus Node ID for Plasma Defender
/// Per RFC-9301, each node has a unique ID in the ring topology
pub const DEFENDER_NODE_ID: u8 = 9;  // Node 9 = Forge/Defender

/// JetStream streams for Plasma Defender
pub mod streams {
    pub const THREAT_EVENTS: &str = "SX9_DEFENDER_THREATS";
    pub const TOOL_OUTPUTS: &str = "SX9_DEFENDER_TOOL_OUTPUTS";
    pub const OSSEC_ALERTS: &str = "SX9_DEFENDER_OSSEC";
    pub const EEI_QUERIES: &str = "SX9_DEFENDER_EEI";
}

/// NATS subjects for Plasma Defender
pub mod subjects {
    pub const THREAT_DETECTED: &str = "sx9.defender.threat.detected";
    pub const THREAT_BLOCKED: &str = "sx9.defender.threat.blocked";
    pub const TOOL_RESULT: &str = "sx9.defender.tool.result";
    pub const OSSEC_ALERT: &str = "sx9.defender.ossec.alert";
    pub const EEI_QUERY: &str = "sx9.defender.eei.query";
    pub const EEI_RESPONSE: &str = "sx9.defender.eei.response";
    pub const RING_BUS: &str = "sx9.ring"; // + .{node_id}.{message_type}
}

/// Ring Bus Node for Plasma Defender
pub struct RingBusNode {
    node_id: u8,
    nats_bridge: Arc<NatsBridge>,
    jetstream: jetstream::Context,
}

impl RingBusNode {
    pub async fn new(nats_url: &str) -> anyhow::Result<Self> {
        let config = NatsBridgeConfig {
            url: nats_url.to_string(),
            instance_id: format!("defender-{}", DEFENDER_NODE_ID),
            jetstream: true,
            batch_size: 100,
            timeout_ms: 100,
        };
        
        let bridge = NatsBridge::connect(config).await?;
        let client = async_nats::connect(nats_url).await?;
        let jetstream = jetstream::new(client);
        
        let node = Self {
            node_id: DEFENDER_NODE_ID,
            nats_bridge: Arc::new(bridge),
            jetstream,
        };
        
        node.init_streams().await?;
        Ok(node)
    }

    /// Initialize JetStream streams for Defender
    async fn init_streams(&self) -> anyhow::Result<()> {
        use std::time::Duration;
        
        // Threat events stream (persistent for correlation)
        let _ = self.jetstream.get_or_create_stream(jetstream::stream::Config {
            name: streams::THREAT_EVENTS.to_string(),
            subjects: vec![
                format!("{}.*", subjects::THREAT_DETECTED),
                format!("{}.*", subjects::THREAT_BLOCKED),
            ],
            max_messages: 100_000,
            max_age: Duration::from_secs(86400 * 7),  // 7 days retention
            ..Default::default()
        }).await?;

        // Tool outputs stream (persistent for re-processing)
        let _ = self.jetstream.get_or_create_stream(jetstream::stream::Config {
            name: streams::TOOL_OUTPUTS.to_string(),
            subjects: vec![format!("{}.*", subjects::TOOL_RESULT)],
            max_messages: 500_000,
            max_age: Duration::from_secs(86400 * 30),  // 30 days retention
            ..Default::default()
        }).await?;

        // OSSEC alerts stream
        let _ = self.jetstream.get_or_create_stream(jetstream::stream::Config {
            name: streams::OSSEC_ALERTS.to_string(),
            subjects: vec![format!("{}.*", subjects::OSSEC_ALERT)],
            max_messages: 200_000,
            max_age: Duration::from_secs(86400 * 14),  // 14 days retention
            ..Default::default()
        }).await?;

        // EEI queries stream
        let _ = self.jetstream.get_or_create_stream(jetstream::stream::Config {
            name: streams::EEI_QUERIES.to_string(),
            subjects: vec![
                format!("{}.*", subjects::EEI_QUERY),
                format!("{}.*", subjects::EEI_RESPONSE),
            ],
            max_messages: 50_000,
            max_age: Duration::from_secs(3600),  // 1 hour retention
            ..Default::default()
        }).await?;

        tracing::info!("✅ Defender JetStream streams initialized");
        Ok(())
    }

    /// Publish threat event to Ring Bus with JetStream persistence
    pub async fn publish_threat(&self, event: &ThreatEvent) -> anyhow::Result<()> {
        let subject = format!("{}.{}", subjects::THREAT_DETECTED, self.node_id);
        let payload = serde_json::to_vec(event)?;
        
        self.jetstream.publish(subject, payload.into()).await?;
        Ok(())
    }

    /// Publish tool output to JetStream for re-processing
    pub async fn publish_tool_output(&self, output: &ToolOutput) -> anyhow::Result<()> {
        let subject = format!("{}.{}", subjects::TOOL_RESULT, output.tool_hash);
        let payload = serde_json::to_vec(output)?;
        
        self.jetstream.publish(subject, payload.into()).await?;
        Ok(())
    }

    /// Send message on Ring Bus to next node
    pub async fn ring_forward(&self, msg: RingMessage) -> anyhow::Result<()> {
        let next_node = (self.node_id + 1) % 9;  // 9-node ring
        let subject = format!("{}.{}.forward", subjects::RING_BUS, next_node);
        let payload = serde_json::to_vec(&msg)?;
        
        self.nats_bridge.client.publish(subject, payload.into()).await?;
        Ok(())
    }

    /// Subscribe to Ring Bus messages for this node
    pub async fn subscribe_ring(&self) -> anyhow::Result<async_nats::Subscriber> {
        let subject = format!("{}.{}.>", subjects::RING_BUS, self.node_id);
        let sub = self.nats_bridge.client.subscribe(subject).await?;
        Ok(sub)
    }
}

/// Ring Bus message format
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RingMessage {
    pub source_node: u8,
    pub target_node: Option<u8>,  // None = broadcast
    pub msg_type: RingMessageType,
    pub payload: Vec<u8>,
    pub hop_count: u8,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RingMessageType {
    ThreatAlert,
    ToolTrigger,
    SdtGate,
    CrystalResonance,
    TickSync,
    EeiQuery,
}

/// Tool output structure for JetStream persistence
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolOutput {
    pub tool_hash: u64,           // Trivariate hash
    pub tool_name: String,
    pub output_raw: Vec<u8>,      // Raw output bytes
    pub output_hash: u64,         // Hash of output for dedup
    pub timestamp: u64,
    pub operator_id: String,
    pub success: bool,
    pub mitre_technique: Option<String>,
}
```

### 0.9 Port Addressing

Per Cargo.toml metadata, Plasma Defender uses ports 18110-18115:

```toml
# From Cargo.toml
port_range_start = 18110
port_range_end = 18115
```

**Port Allocation:**
| Port | Purpose |
|------|---------|
| 18110 | Main HTTP API (health, metrics, endpoints) |
| 18111 | Ring Bus L2 (inter-node communication) |
| 18112 | OSSEC Alert Receiver |
| 18113 | Tool Output Receiver |
| 18114 | EEI Bridge |
| 18115 | Reserved |

### 0.10 Update Cargo.toml for Ring Bus

```toml
[dependencies]
# Existing
sx9-atlas-bus = { path = "../sx9-atlas-bus", features = ["std", "nats"] }

# Ring Bus uses atlas-bus bridge
# No additional deps needed - NatsBridge is in sx9-atlas-bus
```

### 0.11 ATLAS Daemon Integration (RFC-9022)

**Current State:** Plasma-Defender uses `sx9-atlas-bus` for command dispatch but has NO ATLAS daemon

**Required:** Embedded ATLAS daemon for cognitive processing (1ms OODA loop)

The existing `sx9-atlas-daemon` crate provides:
- `AtlasDaemon` - Cognitive engine with OODA loop
- `OodaLoop` - Observe → Orient → Decide → Act cycle
- `HD4Phase` - Hunt/Detect/Disrupt/Disable/Dominate phases
- `ConvergenceCalculator` - H1/H2 convergence scoring

**Plasma-Defender needs its own ATLAS instance** - not shared with other systems.

```rust
// src/atlas_integration.rs

use sx9_atlas_bus::{AtlasBus, PlasmaState, Command, CommandKind};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// ATLAS Configuration for Plasma Defender
/// Different from sx9-atlas-daemon defaults - tuned for threat response
#[derive(Debug, Clone)]
pub struct DefenderAtlasConfig {
    /// Cognitive tick rate (1ms per RFC-9022)
    pub tick_rate_ms: u64,
    /// Maximum tick duration before zone violation
    pub max_tick_duration_ms: u64,
    /// Starting HD4 phase
    pub initial_phase: Hd4Phase,
    /// Crystal family for threat evaluation
    pub crystal_family: sx9_atlas_bus::CrystalFamily,
    /// Enable NATS bridge for distributed ATLAS
    pub nats_enabled: bool,
    /// NATS URL
    pub nats_url: String,
}

impl Default for DefenderAtlasConfig {
    fn default() -> Self {
        Self {
            tick_rate_ms: 1,
            max_tick_duration_ms: 1,
            initial_phase: Hd4Phase::Hunt,
            crystal_family: sx9_atlas_bus::CrystalFamily::Defensive,  // Defensive crystal for threats
            nats_enabled: true,
            nats_url: "nats://localhost:4222".to_string(),
        }
    }
}

/// HD4 Kill Chain phases for threat response
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum Hd4Phase {
    Hunt = 0,      // Active threat-seeking
    Detect = 1,    // Threat identification
    Disrupt = 2,   // Active interference
    Disable = 3,   // Neutralization
    Dominate = 4,  // Full control
}

impl Hd4Phase {
    /// Y-axis value per RFC-9301
    pub fn y_axis(&self) -> f64 {
        match self {
            Self::Hunt => 0.0,
            Self::Detect => 0.25,
            Self::Disrupt => 0.5,
            Self::Disable => 0.75,
            Self::Dominate => 1.0,
        }
    }

    /// Delta angle for plasma state
    pub fn delta_angle(&self) -> f32 {
        match self {
            Self::Hunt => 5.0,
            Self::Detect => 15.0,
            Self::Disrupt => 30.0,
            Self::Disable => 60.0,
            Self::Dominate => 90.0,
        }
    }
}

/// OODA State for threat processing
#[derive(Debug, Clone)]
pub struct OodaState {
    pub phase: OodaPhase,
    pub hd4_phase: Hd4Phase,
    pub tick_count: u64,
    pub last_observation: Option<ThreatObservation>,
    pub last_decision: Option<ThreatDecision>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OodaPhase {
    Observe,
    Orient,
    Decide,
    Act,
}

/// Threat observation from agents
#[derive(Debug, Clone)]
pub struct ThreatObservation {
    pub source: String,           // Agent that observed
    pub threat_hash: u64,         // Trivariate hash
    pub confidence: f32,          // 0.0-1.0
    pub mitre_technique: Option<String>,
    pub timestamp: Instant,
}

/// Decision from OODA cycle
#[derive(Debug, Clone)]
pub struct ThreatDecision {
    pub action: ThreatAction,
    pub target_hash: u64,
    pub priority: u8,
    pub hd4_phase: Hd4Phase,
}

#[derive(Debug, Clone)]
pub enum ThreatAction {
    Monitor,           // Continue observation
    Alert,             // Generate alert
    Block,             // Block threat
    Isolate,           // Network isolation
    Escalate,          // Escalate to higher level
    Neutralize,        // Full neutralization
}

/// Defender's ATLAS Integration
pub struct DefenderAtlas {
    config: DefenderAtlasConfig,
    bus: Arc<AtlasBus>,
    plasma: Arc<PlasmaState>,
    state: OodaState,
    
    // Channels for threat processing
    observation_rx: mpsc::Receiver<ThreatObservation>,
    observation_tx: mpsc::Sender<ThreatObservation>,
    decision_tx: mpsc::Sender<ThreatDecision>,
    
    // Ring Bus integration
    ring_bus: Option<Arc<RingBusNode>>,
}

impl DefenderAtlas {
    pub fn new(config: DefenderAtlasConfig) -> Self {
        let bus = Arc::new(AtlasBus::new());
        let plasma = Arc::new(PlasmaState::new());
        
        // Prime SDT gate for threat response
        plasma.prime();
        
        let (observation_tx, observation_rx) = mpsc::channel(1024);
        let (decision_tx, _) = mpsc::channel(1024);
        
        let state = OodaState {
            phase: OodaPhase::Observe,
            hd4_phase: config.initial_phase,
            tick_count: 0,
            last_observation: None,
            last_decision: None,
        };
        
        Self {
            config,
            bus,
            plasma,
            state,
            observation_rx,
            observation_tx,
            decision_tx,
            ring_bus: None,
        }
    }

    /// Connect to Ring Bus
    pub fn with_ring_bus(mut self, ring_bus: Arc<RingBusNode>) -> Self {
        self.ring_bus = Some(ring_bus);
        self
    }

    /// Get observation sender for agents
    pub fn observation_sender(&self) -> mpsc::Sender<ThreatObservation> {
        self.observation_tx.clone()
    }

    /// Start the 1ms cognitive tick loop (Zone B)
    pub async fn start_cognitive_tick(&mut self) {
        use tokio::time::{interval, Duration};
        
        let mut ticker = interval(Duration::from_millis(self.config.tick_rate_ms));
        
        tracing::info!(
            "🧠 Defender ATLAS starting cognitive tick loop ({}ms interval)",
            self.config.tick_rate_ms
        );

        loop {
            ticker.tick().await;
            let start = Instant::now();
            
            // Execute OODA cycle
            self.ooda_cycle().await;
            
            // Check Zone B compliance
            let elapsed = start.elapsed();
            if elapsed > Duration::from_millis(self.config.max_tick_duration_ms) {
                tracing::error!(
                    "⚠️ BERNOULLI ZONE B VIOLATION: Tick {} took {:?}",
                    self.state.tick_count,
                    elapsed
                );
            }
            
            self.state.tick_count += 1;
        }
    }

    /// Execute single OODA cycle
    async fn ooda_cycle(&mut self) {
        match self.state.phase {
            OodaPhase::Observe => {
                // Check for new threat observations
                if let Ok(obs) = self.observation_rx.try_recv() {
                    self.state.last_observation = Some(obs);
                    self.state.phase = OodaPhase::Orient;
                }
            }
            OodaPhase::Orient => {
                // Evaluate threat against crystal resonance
                if let Some(ref obs) = self.state.last_observation {
                    let ring_strength = self.evaluate_crystal_resonance(obs);
                    
                    // Update HD4 phase based on threat level
                    self.state.hd4_phase = self.determine_hd4_phase(obs.confidence, ring_strength);
                    
                    // Update plasma state
                    self.plasma.set_delta_angle(self.state.hd4_phase.delta_angle());
                }
                self.state.phase = OodaPhase::Decide;
            }
            OodaPhase::Decide => {
                // Make threat response decision
                if let Some(ref obs) = self.state.last_observation {
                    let decision = self.make_decision(obs);
                    self.state.last_decision = Some(decision.clone());
                    
                    // Dispatch to bus
                    self.dispatch_decision(&decision);
                }
                self.state.phase = OodaPhase::Act;
            }
            OodaPhase::Act => {
                // Execute decision (via Ring Bus if connected)
                if let Some(ref decision) = self.state.last_decision {
                    if let Some(ref ring_bus) = self.ring_bus {
                        // Forward to Ring Bus for distributed execution
                        let msg = RingMessage {
                            source_node: DEFENDER_NODE_ID,
                            target_node: None,  // Broadcast
                            msg_type: RingMessageType::ThreatAlert,
                            payload: serde_json::to_vec(decision).unwrap_or_default(),
                            hop_count: 0,
                            timestamp_ns: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_nanos() as u64,
                        };
                        let _ = ring_bus.ring_forward(msg).await;
                    }
                }
                
                // Reset for next cycle
                self.state.last_observation = None;
                self.state.last_decision = None;
                self.state.phase = OodaPhase::Observe;
            }
        }
    }

    /// Evaluate crystal resonance for threat
    fn evaluate_crystal_resonance(&self, obs: &ThreatObservation) -> f32 {
        // Use AtlasBus crystal evaluation
        // This is simplified - real implementation uses full crystal lattice
        obs.confidence * 0.8 + 0.2  // Base resonance
    }

    /// Determine HD4 phase based on threat assessment
    fn determine_hd4_phase(&self, confidence: f32, ring_strength: f32) -> Hd4Phase {
        let threat_score = confidence * ring_strength;
        
        match threat_score {
            s if s >= 0.9 => Hd4Phase::Dominate,
            s if s >= 0.7 => Hd4Phase::Disable,
            s if s >= 0.5 => Hd4Phase::Disrupt,
            s if s >= 0.3 => Hd4Phase::Detect,
            _ => Hd4Phase::Hunt,
        }
    }

    /// Make threat response decision
    fn make_decision(&self, obs: &ThreatObservation) -> ThreatDecision {
        let action = match self.state.hd4_phase {
            Hd4Phase::Hunt => ThreatAction::Monitor,
            Hd4Phase::Detect => ThreatAction::Alert,
            Hd4Phase::Disrupt => ThreatAction::Block,
            Hd4Phase::Disable => ThreatAction::Isolate,
            Hd4Phase::Dominate => ThreatAction::Neutralize,
        };

        ThreatDecision {
            action,
            target_hash: obs.threat_hash,
            priority: (obs.confidence * 255.0) as u8,
            hd4_phase: self.state.hd4_phase,
        }
    }

    /// Dispatch decision to AtlasBus
    fn dispatch_decision(&self, decision: &ThreatDecision) {
        let cmd = Command::new(CommandKind::SdtTrigger {
            gate_id: decision.target_hash as u32,
            reason: decision.hd4_phase as u16,
        });
        
        self.bus.dispatch(cmd);
    }

    /// Get current state
    pub fn state(&self) -> &OodaState {
        &self.state
    }

    /// Get plasma snapshot
    pub fn plasma_snapshot(&self) -> sx9_atlas_bus::PlasmaSnapshot {
        self.plasma.snapshot()
    }

    /// Get AtlasBus reference
    pub fn bus(&self) -> &Arc<AtlasBus> {
        &self.bus
    }
}
```

### 0.12 Update PlasmaDefender Struct (Final)

```rust
// lib.rs - Final struct with all integrations

pub struct PlasmaDefender {
    // HTTP/Health
    server: PlasmaDefenderServer,
    health: HealthMonitor,
    metrics: MetricsCollector,
    config: DefenderConfig,
    
    // Threat Processing
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    monitor: Arc<RwLock<ThreatMonitor>>,
    
    // Signal Processing (TCR Triad)
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    plasma: Arc<PlasmaState>,
    
    // Neural
    ann_daemon: Arc<AnnDaemon>,
    
    // ECS (Layer 1 & 2)
    ecs_world: Arc<DefenderWorld>,
    
    // ATLAS (Layer 3 - Cognitive)
    atlas: Arc<tokio::sync::RwLock<DefenderAtlas>>,
    
    // Communication
    ring_bus: Arc<RingBusNode>,
    nats_bridge: Arc<NatsBridge>,
    plasma_bus: Arc<PlasmaBus>,  // Legacy - migrate to ring_bus
}

impl PlasmaDefender {
    pub async fn new(config: DefenderConfig) -> anyhow::Result<Self> {
        // Initialize Ring Bus first (JetStream streams)
        let ring_bus = Arc::new(RingBusNode::new(&config.nats_url).await?);
        
        // Initialize ATLAS with Ring Bus
        let atlas_config = DefenderAtlasConfig {
            nats_url: config.nats_url.clone(),
            ..Default::default()
        };
        let atlas = DefenderAtlas::new(atlas_config)
            .with_ring_bus(ring_bus.clone());
        
        // Initialize ECS world
        let ecs_world = Arc::new(DefenderWorld::new());
        
        // ... rest of initialization ...
        
        Ok(Self {
            // ... fields ...
            ring_bus,
            atlas: Arc::new(tokio::sync::RwLock::new(atlas)),
            ecs_world,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        // Start ATLAS cognitive tick (spawned task)
        let atlas = self.atlas.clone();
        tokio::spawn(async move {
            atlas.write().await.start_cognitive_tick().await;
        });
        
        // Start Ring Bus listener
        let ring_bus = self.ring_bus.clone();
        tokio::spawn(async move {
            if let Ok(mut sub) = ring_bus.subscribe_ring().await {
                while let Some(msg) = sub.next().await {
                    // Process Ring Bus messages
                    tracing::debug!("Ring Bus message: {:?}", msg.subject);
                }
            }
        });
        
        // Start existing components...
        self.server.start().await?;
        
        Ok(())
    }
}
```

### 0.13 Dependencies Update (Final)

```toml
# Cargo.toml additions

[dependencies]
# ECS
legion.workspace = true

# ATLAS (already have atlas-bus, add daemon types)
sx9-atlas-daemon = { path = "../sx9-atlas-daemon" }

# Existing
sx9-atlas-bus = { path = "../sx9-atlas-bus", features = ["std", "nats"] }
```

---

## PHASE 1: OSSEC MINIMAL INTEGRATION (Session 1)

### 1.1 Create `ossec.rs` Module

**Purpose:** Parse minimal OSSEC alerts without Wazuh complexity

```rust
// src/ossec.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use anyhow::Result;

/// Minimal OSSEC alert structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OssecAlert {
    pub timestamp: String,
    pub rule: OssecRule,
    pub agent: OssecAgentInfo,
    pub location: String,
    pub full_log: String,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OssecRule {
    pub level: u8,
    pub description: String,
    pub id: String,
    #[serde(default)]
    pub mitre: Option<OssecMitre>,
    #[serde(default)]
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OssecMitre {
    pub id: Vec<String>,      // e.g., ["T1110", "T1110.001"]
    pub tactic: Vec<String>,  // e.g., ["Credential Access"]
    pub technique: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OssecAgentInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub ip: Option<String>,
}

/// OSSEC alert reader with file position tracking
pub struct OssecReader {
    alert_path: PathBuf,
    last_position: u64,
}

impl OssecReader {
    pub fn new(alert_path: PathBuf) -> Self {
        Self {
            alert_path,
            last_position: 0,
        }
    }

    /// Read new alerts since last poll
    pub fn poll(&mut self) -> Result<Vec<OssecAlert>> {
        let file = File::open(&self.alert_path)?;
        let metadata = file.metadata()?;
        
        // Check if file was rotated (smaller than last position)
        if metadata.len() < self.last_position {
            self.last_position = 0;
        }
        
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(self.last_position))?;
        
        let mut alerts = Vec::new();
        let mut line = String::new();
        
        while reader.read_line(&mut line)? > 0 {
            if let Ok(alert) = serde_json::from_str::<OssecAlert>(&line) {
                alerts.push(alert);
            }
            line.clear();
        }
        
        self.last_position = reader.stream_position()?;
        Ok(alerts)
    }

    /// Get current file position
    pub fn position(&self) -> u64 {
        self.last_position
    }
}
```

### 1.2 Create MITRE Mapping Table

**Purpose:** Map OSSEC rule IDs to ATT&CK techniques (for rules without embedded MITRE)

```rust
// src/mitre_map.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Static MITRE mapping for common OSSEC rules
/// Format: rule_id -> (technique_id, tactic, description)
pub static OSSEC_MITRE_MAP: Lazy<HashMap<u32, MitreMapping>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    // Authentication failures
    m.insert(5710, MitreMapping::new("T1110", "Credential Access", "Brute Force"));
    m.insert(5711, MitreMapping::new("T1110.001", "Credential Access", "Password Guessing"));
    m.insert(5712, MitreMapping::new("T1110.003", "Credential Access", "Password Spraying"));
    
    // SSH attacks
    m.insert(5701, MitreMapping::new("T1021.004", "Lateral Movement", "SSH"));
    m.insert(5702, MitreMapping::new("T1021.004", "Lateral Movement", "SSH"));
    m.insert(5703, MitreMapping::new("T1078", "Defense Evasion", "Valid Accounts"));
    
    // File integrity
    m.insert(550, MitreMapping::new("T1565.001", "Impact", "Stored Data Manipulation"));
    m.insert(553, MitreMapping::new("T1070.004", "Defense Evasion", "File Deletion"));
    m.insert(554, MitreMapping::new("T1222", "Defense Evasion", "File Permissions Modification"));
    
    // Rootkit detection
    m.insert(510, MitreMapping::new("T1014", "Defense Evasion", "Rootkit"));
    m.insert(511, MitreMapping::new("T1014", "Defense Evasion", "Rootkit"));
    
    // Web attacks
    m.insert(31101, MitreMapping::new("T1190", "Initial Access", "Exploit Public-Facing Application"));
    m.insert(31102, MitreMapping::new("T1059.007", "Execution", "JavaScript"));
    m.insert(31103, MitreMapping::new("T1059.004", "Execution", "Unix Shell"));
    
    // Privilege escalation
    m.insert(5401, MitreMapping::new("T1548.003", "Privilege Escalation", "Sudo and Sudo Caching"));
    m.insert(5402, MitreMapping::new("T1548.003", "Privilege Escalation", "Sudo and Sudo Caching"));
    
    // Network
    m.insert(5151, MitreMapping::new("T1046", "Discovery", "Network Service Discovery"));
    m.insert(5152, MitreMapping::new("T1046", "Discovery", "Network Service Discovery"));
    
    m
});

#[derive(Debug, Clone)]
pub struct MitreMapping {
    pub technique_id: &'static str,
    pub tactic: &'static str,
    pub description: &'static str,
}

impl MitreMapping {
    pub const fn new(technique_id: &'static str, tactic: &'static str, description: &'static str) -> Self {
        Self { technique_id, tactic, description }
    }
}

/// Get MITRE mapping for an OSSEC rule
pub fn get_mitre_mapping(rule_id: u32) -> Option<&'static MitreMapping> {
    OSSEC_MITRE_MAP.get(&rule_id)
}

/// Map MITRE tactic to HD4 phase
pub fn tactic_to_hd4(tactic: &str) -> crate::Hd4Phase {
    match tactic {
        "Reconnaissance" | "Resource Development" => crate::Hd4Phase::Hunt,
        "Initial Access" | "Execution" => crate::Hd4Phase::Detect,
        "Persistence" | "Privilege Escalation" | "Defense Evasion" => crate::Hd4Phase::Disrupt,
        "Credential Access" | "Discovery" | "Lateral Movement" => crate::Hd4Phase::Dominate,
        "Collection" | "Command and Control" | "Exfiltration" | "Impact" => crate::Hd4Phase::Disable,
        _ => crate::Hd4Phase::Detect,
    }
}
```

### 1.3 Create OssecAgent

**Purpose:** Agent that polls OSSEC alerts and converts to ThreatEvents

```rust
// src/ossec_agent.rs

use crate::agents::{ThreatEvent, ThreatSeverity};
use crate::ossec::{OssecAlert, OssecReader};
use crate::mitre_map::{get_mitre_mapping, tactic_to_hd4};
use std::path::PathBuf;
use std::sync::Arc;
use sx9_atlas_bus::PlasmaState;
use tokio::sync::Mutex;
use anyhow::Result;

pub struct OssecAgent {
    reader: Arc<Mutex<OssecReader>>,
    plasma: Arc<PlasmaState>,
    enabled: bool,
}

impl OssecAgent {
    pub fn new(alert_path: PathBuf, plasma: Arc<PlasmaState>) -> Self {
        Self {
            reader: Arc::new(Mutex::new(OssecReader::new(alert_path))),
            plasma,
            enabled: true,
        }
    }

    /// Poll for new OSSEC alerts and convert to ThreatEvents
    pub async fn poll(&self) -> Result<Vec<ThreatEvent>> {
        if !self.enabled {
            return Ok(Vec::new());
        }

        let mut reader = self.reader.lock().await;
        let alerts = reader.poll()?;
        
        let events: Vec<ThreatEvent> = alerts
            .into_iter()
            .map(|alert| self.alert_to_event(alert))
            .collect();
        
        Ok(events)
    }

    /// Convert OSSEC alert to ThreatEvent
    fn alert_to_event(&self, alert: OssecAlert) -> ThreatEvent {
        // Get MITRE info from alert or fallback to mapping table
        let (technique, tactic) = if let Some(mitre) = &alert.rule.mitre {
            (
                mitre.id.first().cloned().unwrap_or_default(),
                mitre.tactic.first().cloned().unwrap_or_default(),
            )
        } else if let Ok(rule_id) = alert.rule.id.parse::<u32>() {
            if let Some(mapping) = get_mitre_mapping(rule_id) {
                (mapping.technique_id.to_string(), mapping.tactic.to_string())
            } else {
                (String::new(), String::new())
            }
        } else {
            (String::new(), String::new())
        };

        // Map OSSEC level to severity
        let severity = match alert.rule.level {
            0..=3 => ThreatSeverity::Low,
            4..=7 => ThreatSeverity::Medium,
            8..=11 => ThreatSeverity::High,
            12..=15 => ThreatSeverity::Critical,
            _ => ThreatSeverity::Low,
        };

        ThreatEvent {
            agent_id: format!("ossec-{}", alert.agent.id),
            event_type: format!("ossec.{}.{}", alert.rule.id, technique),
            payload: alert.full_log.into_bytes(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            severity,
            // Extended fields (add to ThreatEvent struct)
            // mitre_technique: technique,
            // mitre_tactic: tactic,
            // hd4_phase: tactic_to_hd4(&tactic),
            // ossec_rule_id: alert.rule.id,
            // ossec_description: alert.rule.description,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
```

### 1.4 Update `lib.rs` Exports

```rust
// Add to lib.rs
pub mod ossec;
pub mod ossec_agent;
pub mod mitre_map;

pub use ossec::{OssecAlert, OssecReader};
pub use ossec_agent::OssecAgent;
pub use mitre_map::{get_mitre_mapping, tactic_to_hd4, MitreMapping};
```

### 1.5 Dependencies to Add

```toml
# Add to Cargo.toml
once_cell = "1.19"
```

---

## PHASE 2: IMPLEMENT THREAT AGENTS (Session 2)

### 2.1 Update ThreatEvent Structure

**Purpose:** Add MITRE and HD4 context to events

```rust
// src/agents.rs - Updated ThreatEvent

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub agent_id: String,
    pub event_type: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub severity: ThreatSeverity,
    
    // MITRE ATT&CK context
    #[serde(default)]
    pub mitre_technique: Option<String>,
    #[serde(default)]
    pub mitre_tactic: Option<String>,
    
    // HD4 phase mapping
    #[serde(default)]
    pub hd4_phase: Option<Hd4Phase>,
    
    // Source context
    #[serde(default)]
    pub source_rule_id: Option<String>,
    #[serde(default)]
    pub source_description: Option<String>,
    
    // Delta position (6 decimal)
    #[serde(default)]
    pub delta_x: Option<f64>,  // Semantic (MITRE stage)
    #[serde(default)]
    pub delta_y: Option<f64>,  // Operational (HD4 phase)
    #[serde(default)]
    pub delta_z: Option<f64>,  // Temporal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hd4Phase {
    Hunt = 0,
    Detect = 1,
    Disrupt = 2,
    Dominate = 3,
    Disable = 4,
}

impl Hd4Phase {
    pub fn to_delta_y(&self) -> f64 {
        (*self as u8 as f64) * 0.25
    }
}
```

### 2.2 Implement NetworkMonitor Agent

```rust
// src/agents.rs - NetworkMonitor implementation

async fn monitor_network(&self) -> anyhow::Result<Option<ThreatEvent>> {
    // Check plasma state for network anomalies
    let entropy = self.plasma.entropy();
    let delta = self.plasma.delta_angle_raw();
    
    // High entropy + high delta = potential network attack
    if entropy > 800 && delta > 100 {
        let event = ThreatEvent {
            agent_id: self.agent_id.clone(),
            event_type: "network.anomaly.entropy".to_string(),
            payload: format!("entropy={},delta={}", entropy, delta).into_bytes(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            severity: if entropy > 900 { ThreatSeverity::High } else { ThreatSeverity::Medium },
            mitre_technique: Some("T1046".to_string()),  // Network Service Discovery
            mitre_tactic: Some("Discovery".to_string()),
            hd4_phase: Some(Hd4Phase::Detect),
            source_rule_id: None,
            source_description: Some("High entropy network activity".to_string()),
            delta_x: Some(0.5),  // Mid kill-chain
            delta_y: Some(Hd4Phase::Detect.to_delta_y()),
            delta_z: Some(1.0),  // Current
        };
        return Ok(Some(event));
    }
    
    Ok(None)
}
```

### 2.3 Implement ThreatHunter Agent

```rust
// src/agents.rs - ThreatHunter implementation

async fn hunt_threats(&self) -> anyhow::Result<Option<ThreatEvent>> {
    // Active threat hunting based on plasma state patterns
    // This would integrate with GLAF graph queries
    
    // For now, check for suspicious state combinations
    let entropy = self.plasma.entropy();
    let sdt_state = self.plasma.sdt_state();
    
    // SDT gate oscillating + high entropy = potential evasion attempt
    if sdt_state == 2 && entropy > 700 {  // BLOCKED state
        let event = ThreatEvent {
            agent_id: self.agent_id.clone(),
            event_type: "hunt.evasion.attempt".to_string(),
            payload: format!("sdt_blocked,entropy={}", entropy).into_bytes(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            severity: ThreatSeverity::High,
            mitre_technique: Some("T1562".to_string()),  // Impair Defenses
            mitre_tactic: Some("Defense Evasion".to_string()),
            hd4_phase: Some(Hd4Phase::Disrupt),
            source_rule_id: None,
            source_description: Some("SDT gate block with high entropy".to_string()),
            delta_x: Some(0.714286),  // Defense Evasion stage
            delta_y: Some(Hd4Phase::Disrupt.to_delta_y()),
            delta_z: Some(1.0),
        };
        return Ok(Some(event));
    }
    
    Ok(None)
}
```

### 2.4 Implement AnomalyDetector Agent

```rust
// src/agents.rs - AnomalyDetector implementation

async fn detect_anomalies(&self) -> anyhow::Result<Option<ThreatEvent>> {
    // Statistical anomaly detection using plasma metrics
    let entropy = self.plasma.entropy();
    let tick = self.plasma.tick();
    
    // Calculate entropy rate of change (would need historical data)
    // For now, use absolute thresholds
    
    // Critical entropy spike
    if entropy > 950 {
        let event = ThreatEvent {
            agent_id: self.agent_id.clone(),
            event_type: "anomaly.entropy.critical".to_string(),
            payload: format!("entropy={},tick={}", entropy, tick).into_bytes(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            severity: ThreatSeverity::Critical,
            mitre_technique: None,  // Unknown technique
            mitre_tactic: None,
            hd4_phase: Some(Hd4Phase::Detect),
            source_rule_id: None,
            source_description: Some("Critical entropy anomaly".to_string()),
            delta_x: Some(0.5),
            delta_y: Some(Hd4Phase::Detect.to_delta_y()),
            delta_z: Some(1.0),
        };
        return Ok(Some(event));
    }
    
    Ok(None)
}
```

### 2.5 Implement CanaryWatcher Agent

```rust
// src/agents.rs - CanaryWatcher implementation

async fn watch_canaries(&self) -> anyhow::Result<Option<ThreatEvent>> {
    // Monitor SDT canary payloads
    // Canaries are special payloads that should never be accessed
    
    // Check if any canary was triggered (would need canary registry)
    // For now, monitor SDT canary state
    
    let sdt_canary_triggered = false; // Would check actual canary state
    
    if sdt_canary_triggered {
        let event = ThreatEvent {
            agent_id: self.agent_id.clone(),
            event_type: "canary.triggered".to_string(),
            payload: b"canary_access_detected".to_vec(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            severity: ThreatSeverity::Critical,
            mitre_technique: Some("T1083".to_string()),  // File and Directory Discovery
            mitre_tactic: Some("Discovery".to_string()),
            hd4_phase: Some(Hd4Phase::Detect),
            source_rule_id: None,
            source_description: Some("Canary file accessed".to_string()),
            delta_x: Some(0.428571),  // Execution stage
            delta_y: Some(Hd4Phase::Detect.to_delta_y()),
            delta_z: Some(1.0),
        };
        return Ok(Some(event));
    }
    
    Ok(None)
}
```

---

## PHASE 3: INTEGRATION BRIDGES (Session 3)

### 3.1 EEI Bridge

**Purpose:** Publish detected threats to Leptose EEI system

```rust
// src/eei_bridge.rs

use crate::agents::ThreatEvent;
use async_nats::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiQuery {
    pub query_id: String,
    pub text: String,
    pub context: EeiContext,
    pub n_results: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiContext {
    pub source: String,
    pub mitre_technique: Option<String>,
    pub mitre_tactic: Option<String>,
    pub hd4_phase: Option<String>,
    pub severity: String,
    pub timestamp: u64,
}

pub struct EeiBridge {
    nats: Client,
    query_subject: String,
}

impl EeiBridge {
    pub fn new(nats: Client) -> Self {
        Self {
            nats,
            query_subject: "eei.query".to_string(),
        }
    }

    /// Generate EEI query from threat event
    pub async fn query_from_threat(&self, event: &ThreatEvent) -> Result<()> {
        let query = EeiQuery {
            query_id: uuid::Uuid::new_v4().to_string(),
            text: format!(
                "What intelligence exists for {} targeting {} phase?",
                event.mitre_technique.as_deref().unwrap_or("unknown technique"),
                event.hd4_phase.map(|p| format!("{:?}", p)).unwrap_or_default()
            ),
            context: EeiContext {
                source: "plasma-defender".to_string(),
                mitre_technique: event.mitre_technique.clone(),
                mitre_tactic: event.mitre_tactic.clone(),
                hd4_phase: event.hd4_phase.map(|p| format!("{:?}", p)),
                severity: format!("{:?}", event.severity),
                timestamp: event.timestamp,
            },
            n_results: 5,
        };

        let payload = serde_json::to_vec(&query)?;
        self.nats.publish(self.query_subject.clone(), payload.into()).await?;
        
        tracing::info!("Published EEI query: {}", query.query_id);
        Ok(())
    }
}
```

### 3.2 SlotGraph Bridge

**Purpose:** Correlate threats with 164-task graph

```rust
// src/slotgraph_bridge.rs

use crate::agents::{ThreatEvent, Hd4Phase};
use std::collections::HashMap;
use anyhow::Result;

/// Task correlation from SlotGraph
#[derive(Debug, Clone)]
pub struct TaskCorrelation {
    pub task_id: String,
    pub task_name: String,
    pub relevance_score: f64,
    pub hd4_phase: Hd4Phase,
    pub tools: Vec<String>,
}

pub struct SlotGraphBridge {
    // Would hold reference to actual SlotGraph
    // For now, static mapping of MITRE techniques to tasks
    technique_to_tasks: HashMap<String, Vec<String>>,
}

impl SlotGraphBridge {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        
        // Map MITRE techniques to CTAS task UUIDs
        // These would come from the actual 164-task graph
        map.insert("T1110".to_string(), vec![
            "uuid-003-002-001".to_string(),  // Brute force detection
            "uuid-003-002-002".to_string(),  // Password policy enforcement
        ]);
        map.insert("T1046".to_string(), vec![
            "uuid-001-001-001".to_string(),  // Network scanning
            "uuid-001-001-002".to_string(),  // Port enumeration
        ]);
        map.insert("T1562".to_string(), vec![
            "uuid-004-001-001".to_string(),  // Defense monitoring
            "uuid-004-001-002".to_string(),  // Security control validation
        ]);
        
        Self { technique_to_tasks: map }
    }

    /// Find related tasks for a threat event
    pub fn correlate(&self, event: &ThreatEvent) -> Vec<TaskCorrelation> {
        let mut correlations = Vec::new();
        
        if let Some(technique) = &event.mitre_technique {
            if let Some(task_ids) = self.technique_to_tasks.get(technique) {
                for task_id in task_ids {
                    correlations.push(TaskCorrelation {
                        task_id: task_id.clone(),
                        task_name: format!("Task for {}", technique),
                        relevance_score: 0.85,  // Would calculate based on context
                        hd4_phase: event.hd4_phase.unwrap_or(Hd4Phase::Detect),
                        tools: vec![],  // Would populate from task definition
                    });
                }
            }
        }
        
        correlations
    }
}
```

### 3.3 Update ThreatMonitor to Use Bridges

```rust
// src/monitor.rs - Updated with bridges

use crate::eei_bridge::EeiBridge;
use crate::slotgraph_bridge::SlotGraphBridge;
use crate::ossec_agent::OssecAgent;

pub struct ThreatMonitor {
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    ossec_agent: Option<OssecAgent>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    plasma: Arc<PlasmaState>,
    plasma_bus: Arc<PlasmaBus>,
    eei_bridge: Option<EeiBridge>,
    slotgraph_bridge: SlotGraphBridge,
    monitor_interval: Duration,
    tick: u64,
}

impl ThreatMonitor {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut ticker = interval(self.monitor_interval);

        loop {
            ticker.tick().await;
            self.tick += 1;

            // Poll OSSEC alerts
            if let Some(ossec) = &self.ossec_agent {
                for event in ossec.poll().await? {
                    self.process_event(event).await?;
                }
            }

            // Monitor standard agents
            let agents = self.agents.read().await;
            for agent in agents.iter() {
                if let Ok(Some(event)) = agent.monitor().await {
                    self.process_event(event).await?;
                }
            }
        }
    }

    async fn process_event(&self, event: ThreatEvent) -> anyhow::Result<()> {
        // Evaluate through crystal
        let ring_strength = self.crystal
            .get_ring_strength(&event.payload, self.plasma.delta_angle_raw());

        // Check SDT gate
        let crystal = self.crystal.crystal();
        let allowed = self.sdt.resonate(&event.payload, &crystal, self.tick);

        // Correlate with SlotGraph
        let correlations = self.slotgraph_bridge.correlate(&event);
        
        tracing::info!(
            "Threat processed: {} (ring={:.2}, allowed={}, correlations={})",
            event.event_type, ring_strength, allowed, correlations.len()
        );

        if allowed {
            // Query EEI for intelligence
            if let Some(eei) = &self.eei_bridge {
                eei.query_from_threat(&event).await?;
            }
            
            self.handle_threat(event).await?;
        } else {
            self.block_threat(event).await?;
        }

        Ok(())
    }
}
```

---

## PHASE 4: TESTING & VALIDATION (Session 4)

### 4.1 Unit Tests

```rust
// tests/ossec_tests.rs

#[tokio::test]
async fn test_ossec_alert_parsing() {
    let alert_json = r#"{
        "timestamp": "2025-12-16T10:00:00Z",
        "rule": {
            "level": 10,
            "description": "Multiple authentication failures",
            "id": "5710",
            "mitre": {
                "id": ["T1110"],
                "tactic": ["Credential Access"],
                "technique": ["Brute Force"]
            },
            "groups": ["authentication_failures"]
        },
        "agent": {
            "id": "001",
            "name": "server1",
            "ip": "192.168.1.100"
        },
        "location": "/var/log/auth.log",
        "full_log": "Failed password for root from 10.0.0.1"
    }"#;
    
    let alert: OssecAlert = serde_json::from_str(alert_json).unwrap();
    assert_eq!(alert.rule.level, 10);
    assert_eq!(alert.rule.id, "5710");
    assert!(alert.rule.mitre.is_some());
}

#[tokio::test]
async fn test_mitre_mapping() {
    let mapping = get_mitre_mapping(5710);
    assert!(mapping.is_some());
    assert_eq!(mapping.unwrap().technique_id, "T1110");
}

#[tokio::test]
async fn test_tactic_to_hd4() {
    assert_eq!(tactic_to_hd4("Reconnaissance"), Hd4Phase::Hunt);
    assert_eq!(tactic_to_hd4("Initial Access"), Hd4Phase::Detect);
    assert_eq!(tactic_to_hd4("Defense Evasion"), Hd4Phase::Disrupt);
    assert_eq!(tactic_to_hd4("Lateral Movement"), Hd4Phase::Dominate);
    assert_eq!(tactic_to_hd4("Impact"), Hd4Phase::Disable);
}
```

### 4.2 Integration Tests

```rust
// tests/integration_tests.rs

#[tokio::test]
async fn test_threat_flow() {
    // Start NATS (mock or real)
    // Create PlasmaDefender
    // Inject OSSEC alert
    // Verify:
    //   1. Alert parsed correctly
    //   2. Crystal evaluation occurred
    //   3. SDT gate decision made
    //   4. EEI query published
    //   5. SlotGraph correlation found
}
```

### 4.3 Manual Test Procedure

```bash
# 1. Start NATS
nats-server -js

# 2. Start Plasma-Defender
cargo run -p sx9-plasma-defender

# 3. Create test OSSEC alert
echo '{"timestamp":"2025-12-16T10:00:00Z","rule":{"level":10,"description":"Test","id":"5710"},"agent":{"id":"001","name":"test"},"location":"/test","full_log":"test"}' >> /tmp/ossec-alerts.json

# 4. Verify processing
nats sub "sx9.plasma.*"
nats sub "eei.query"

# 5. Check health endpoint
curl http://localhost:18110/health
```

---

## DELIVERABLES CHECKLIST

### Phase 1 (OSSEC)
- [ ] `src/ossec.rs` - Alert structures and reader
- [ ] `src/mitre_map.rs` - OSSEC rule → MITRE mapping
- [ ] `src/ossec_agent.rs` - OSSEC polling agent
- [ ] Update `lib.rs` exports
- [ ] Add `once_cell` dependency

### Phase 2 (Agents)
- [ ] Update `ThreatEvent` with MITRE/HD4 fields
- [ ] Implement `monitor_network()`
- [ ] Implement `hunt_threats()`
- [ ] Implement `detect_anomalies()`
- [ ] Implement `watch_canaries()`

### Phase 3 (Bridges)
- [ ] `src/eei_bridge.rs` - EEI query generation
- [ ] `src/slotgraph_bridge.rs` - Task correlation
- [ ] Update `ThreatMonitor` to use bridges

### Phase 4 (Testing)
- [ ] Unit tests for OSSEC parsing
- [ ] Unit tests for MITRE mapping
- [ ] Integration test for full flow
- [ ] Manual test procedure validated

---

## SUCCESS CRITERIA

| Metric | Target |
|--------|--------|
| OSSEC alert parse rate | 100% for valid JSON |
| MITRE mapping coverage | >80% of common rules |
| Agent detection latency | <100ms |
| EEI query generation | On every allowed threat |
| SlotGraph correlation | >0 matches for mapped techniques |
| False positive rate | <20% (tunable via thresholds) |

---

**Next Step:** Start Phase 1 - Create `ossec.rs` module

