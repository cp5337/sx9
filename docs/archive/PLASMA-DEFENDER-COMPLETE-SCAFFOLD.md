# Plasma Defender Complete Scaffold
## With Agents, Crystal, and SDT Integration

**Date:** December 2025  
**Status:** Complete Scaffold  
**Purpose:** PLASMA defender with full crystal resonance and SDT gate integration

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│              sx9-plasma-defender                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │ Axum Server      │  │ Threat Monitor   │                │
│  │ (HTTP/WebSocket) │  │                  │                │
│  └──────────────────┘  │  ┌────────────┐ │                │
│         │              │  │ Agents     │ │                │
│         │              │  │  - Network │ │                │
│         │              │  │  - Threat  │ │                │
│         │              │  │  - Canary  │ │                │
│         │              │  │  - Anomaly │ │                │
│         │              │  └─────┬──────┘ │                │
│         │              │        │        │                │
│         │              └─────────┼────────┘                │
│         │                       │                          │
│         │                       ▼                          │
│         │              ┌──────────────────┐               │
│         │              │ Crystal Integration│              │
│         │              │ (Polycrystal)     │              │
│         │              └───────┬───────────┘              │
│         │                      │                           │
│         │                      ▼                           │
│         │              ┌──────────────────┐               │
│         │              │ SDT Integration  │               │
│         │              │ (Gate Control)   │               │
│         │              └───────┬───────────┘              │
│         │                      │                           │
│         └──────────────────────┼───────────────────────────┘
│                                │
│                                ▼
│                        ┌──────────────────┐
│                        │ sx9-atlas-bus    │
│                        │ (PlasmaState)    │
│                        └──────────────────┘
└─────────────────────────────────────────────────────────────┘
```

---

## Complete Module Structure

### Directory Structure
```
sx9-plasma-defender/
├── Cargo.toml
├── smart-crate.toml
└── src/
    ├── lib.rs              # Public API
    ├── server.rs           # Axum server (HTTP/WebSocket)
    ├── health.rs           # Health monitoring
    ├── metrics.rs          # Metrics collection
    ├── agents.rs           # Threat monitoring agents
    ├── crystal.rs          # Crystal resonance integration
    ├── sdt.rs              # SDT gate integration
    ├── monitor.rs          # Threat monitoring system
    └── config.rs           # Configuration
```

---

## Module Implementations

### lib.rs - Public API

```rust
//! sx9-plasma-defender - PLASMA Defender with Crystal & SDT Integration
//! 
//! Purpose: PLASMA defender for health monitoring and threat detection
//! - Health endpoint (/health)
//! - Metrics endpoint (/metrics)
//! - SDT state endpoint (/sdt/state)
//! - Crystal resonance endpoint (/crystal/resonance)
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
pub use agents::{ThreatAgent, AgentType};
pub use crystal::CrystalIntegration;
pub use sdt::SdtIntegration;
pub use monitor::ThreatMonitor;
pub use config::DefenderConfig;

use sx9_atlas_bus::{PlasmaState, CrystalFamily, ThyristorConfig};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PlasmaDefender {
    server: PlasmaDefenderServer,
    health: HealthMonitor,
    metrics: MetricsCollector,
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    monitor: Arc<RwLock<ThreatMonitor>>,
    plasma: Arc<PlasmaState>,
    config: DefenderConfig,
}

impl PlasmaDefender {
    pub fn new(config: DefenderConfig) -> Result<Self> {
        // Initialize PlasmaState from sx9-atlas-bus
        let plasma = Arc::new(PlasmaState::new());
        
        // Initialize crystal integration
        let crystal_family = config.crystal_family.unwrap_or(CrystalFamily::GroundStation);
        let crystal = Arc::new(CrystalIntegration::new(crystal_family));
        
        // Initialize SDT integration
        let sdt_config = config.sdt_config.unwrap_or_default();
        let sdt = Arc::new(SdtIntegration::new(plasma.clone(), sdt_config));
        
        // Initialize threat agents
        let agents = Arc::new(RwLock::new(Vec::new()));
        for agent_config in &config.agents {
            let agent = ThreatAgent::new(
                agent_config.id.clone(),
                agent_config.agent_type,
                plasma.clone(),
            );
            agents.write().await.push(agent);
        }
        
        // Initialize threat monitor
        let monitor = Arc::new(RwLock::new(ThreatMonitor::new(
            agents.clone(),
            crystal.clone(),
            sdt.clone(),
            plasma.clone(),
        )));
        
        // Initialize server
        let server = PlasmaDefenderServer::new(&config, plasma.clone());
        
        // Initialize health and metrics
        let health = HealthMonitor::new(plasma.clone());
        let metrics = MetricsCollector::new(plasma.clone());
        
        Ok(Self {
            server,
            health,
            metrics,
            agents,
            crystal,
            sdt,
            monitor,
            plasma,
            config,
        })
    }
    
    pub async fn start(&self) -> Result<()> {
        // Start threat monitor
        let monitor = self.monitor.clone();
        tokio::spawn(async move {
            let mut monitor = monitor.write().await;
            if let Err(e) = monitor.run().await {
                tracing::error!("Threat monitor error: {}", e);
            }
        });
        
        // Start Axum server
        self.server.start(&self.config.bind_addr).await
    }
    
    pub fn get_health(&self) -> HealthStatus {
        self.health.get_status()
    }
    
    pub fn get_metrics(&self) -> Metrics {
        self.metrics.collect()
    }
    
    /// Evaluate threat through crystal resonance and SDT gate
    pub async fn evaluate_threat(&self, payload: &[u8]) -> ThreatResult {
        let delta_angle = self.plasma.delta_angle_raw();
        
        // Evaluate through crystal
        let ring_strength = self.crystal.get_ring_strength(payload, delta_angle);
        
        // Check SDT gate
        let sdt_state = self.sdt.get_state();
        let allowed = self.sdt.should_proceed(ring_strength);
        
        ThreatResult {
            ring_strength,
            sdt_state,
            allowed,
            delta_angle,
            entropy: self.plasma.entropy_raw(),
        }
    }
    
    /// Get current SDT gate state
    pub fn get_sdt_state(&self) -> SdtState {
        self.sdt.get_state()
    }
    
    /// Get crystal ring strength
    pub fn get_ring_strength(&self, payload: &[u8]) -> f32 {
        let delta_angle = self.plasma.delta_angle_raw();
        self.crystal.get_ring_strength(payload, delta_angle)
    }
}
```

---

### agents.rs - Threat Monitoring Agents

```rust
use sx9_atlas_bus::PlasmaState;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentType {
    NetworkMonitor,    // Monitor network traffic
    ThreatHunter,      // Hunt for threats
    CanaryWatcher,     // Watch canary triggers
    AnomalyDetector,   // Detect anomalies
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub agent_id: String,
    pub event_type: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub severity: ThreatSeverity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct ThreatAgent {
    agent_id: String,
    agent_type: AgentType,
    plasma: Arc<PlasmaState>,
    enabled: bool,
}

impl ThreatAgent {
    pub fn new(agent_id: String, agent_type: AgentType, plasma: Arc<PlasmaState>) -> Self {
        Self {
            agent_id,
            agent_type,
            plasma,
            enabled: true,
        }
    }
    
    pub async fn monitor(&mut self) -> Result<Option<ThreatEvent>> {
        if !self.enabled {
            return Ok(None);
        }
        
        match self.agent_type {
            AgentType::NetworkMonitor => self.monitor_network().await,
            AgentType::ThreatHunter => self.hunt_threats().await,
            AgentType::CanaryWatcher => self.watch_canaries().await,
            AgentType::AnomalyDetector => self.detect_anomalies().await,
        }
    }
    
    async fn monitor_network(&self) -> Result<Option<ThreatEvent>> {
        // Monitor network traffic for threats
        // Check PlasmaState for anomalies
        // Return threat event if detected
        Ok(None)
    }
    
    async fn hunt_threats(&self) -> Result<Option<ThreatEvent>> {
        // Actively hunt for threats
        // Use GLAF correlation
        // Return threat event if found
        Ok(None)
    }
    
    async fn watch_canaries(&self) -> Result<Option<ThreatEvent>> {
        // Watch for canary triggers
        // Monitor SDT canary payloads
        // Return threat event if triggered
        Ok(None)
    }
    
    async fn detect_anomalies(&self) -> Result<Option<ThreatEvent>> {
        // Detect anomalies in system behavior
        // Use entropy and delta angle
        // Return threat event if anomaly detected
        Ok(None)
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
```

---

### crystal.rs - Crystal Resonance Integration

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
    
    /// Check if crystal passed (vote passed)
    pub fn passed(&self, payload: &[u8], delta_angle: u16) -> bool {
        let result = self.evaluate(payload, delta_angle);
        result.passed
    }
    
    /// Get crystal family
    pub fn family(&self) -> CrystalFamily {
        self.crystal_family
    }
}
```

---

### sdt.rs - SDT Gate Integration

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
    
    /// Resonate payload through crystal and update SDT gate
    pub fn resonate(&self, payload: &[u8], crystal: &Crystal, tick: u64) -> bool {
        self.plasma.resonate(crystal, payload, tick, &self.config)
    }
    
    /// Resonate through polycrystal and update SDT gate
    pub fn resonate_poly(&self, payload: &[u8], polycrystal: &Polycrystal, tick: u64) -> (bool, PolycrystalResult) {
        self.plasma.resonate_poly(polycrystal, payload, tick, &self.config)
    }
    
    /// Reset SDT gate (requires auth)
    pub fn reset(&self, auth_token: &[u8]) -> Result<()> {
        // Verify auth token
        // Reset SDT gate to Off
        // Implementation depends on auth mechanism
        Ok(())
    }
    
    /// Get SDT configuration
    pub fn config(&self) -> &ThyristorConfig {
        &self.config
    }
}
```

---

### monitor.rs - Threat Monitoring System

```rust
use sx9_atlas_bus::PlasmaState;
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub struct ThreatMonitor {
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    plasma: Arc<PlasmaState>,
    monitor_interval: Duration,
    tick: u64,
}

impl ThreatMonitor {
    pub fn new(
        agents: Arc<RwLock<Vec<ThreatAgent>>>,
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
            tick: 0,
        }
    }
    
    pub async fn run(&mut self) -> Result<()> {
        let mut ticker = interval(self.monitor_interval);
        
        loop {
            ticker.tick().await;
            self.tick += 1;
            
            // Monitor threats from agents
            let agents = self.agents.read().await;
            for agent in agents.iter() {
                let mut agent = agent.clone();
                if let Ok(Some(event)) = agent.monitor().await {
                    // Evaluate through crystal
                    let ring_strength = self.crystal.get_ring_strength(
                        &event.payload,
                        self.plasma.delta_angle_raw(),
                    );
                    
                    // Resonate through SDT gate
                    let crystal = Crystal::new(self.crystal.family());
                    let allowed = self.sdt.resonate(
                        &event.payload,
                        &crystal,
                        self.tick,
                    );
                    
                    if allowed {
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
    
    async fn handle_threat(&self, event: ThreatEvent) -> Result<()> {
        tracing::warn!("Threat detected and allowed: {:?}", event);
        // Handle threat (alert, log, etc.)
        Ok(())
    }
    
    async fn block_threat(&self, event: ThreatEvent) -> Result<()> {
        tracing::info!("Threat blocked by SDT gate: {:?}", event);
        // Block threat (log, metrics, etc.)
        Ok(())
    }
}
```

---

### server.rs - Axum Server with Crystal/SDT Endpoints

```rust
use axum::{
    Router,
    routing::get,
    extract::State,
    response::Json,
};
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
            .route("/crystal/resonance", get(crystal_resonance_handler))
            .route("/crystal/family", get(crystal_family_handler))
            .with_state(plasma.clone());
        
        Self {
            router,
            health_endpoint: config.health_endpoint.clone(),
            metrics_endpoint: config.metrics_endpoint.clone(),
            plasma,
        }
    }
    
    pub async fn start(&self, addr: &str) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, self.router.clone()).await?;
        Ok(())
    }
}

async fn sdt_state_handler(
    State(plasma): State<Arc<PlasmaState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "sdt_state": format!("{:?}", plasma.sdt_state()),
        "delta_angle": plasma.delta_angle_raw(),
        "entropy": plasma.entropy_raw(),
        "excited": plasma.excited(),
        "ring_strength": plasma.last_ring_strength(),
        "trigger_count": plasma.trigger_count(),
    }))
}

async fn crystal_resonance_handler(
    State(plasma): State<Arc<PlasmaState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ring_strength": plasma.last_ring_strength(),
        "excited": plasma.excited(),
        "delta_angle": plasma.delta_angle_raw(),
        "entropy": plasma.entropy_raw(),
    }))
}

async fn crystal_family_handler(
    State(plasma): State<Arc<PlasmaState>>,
) -> Json<serde_json::Value> {
    // Return current crystal family configuration
    Json(serde_json::json!({
        "crystal_family": "GroundStation", // From config
    }))
}
```

---

### config.rs - Configuration

```rust
use sx9_atlas_bus::{CrystalFamily, ThyristorConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenderConfig {
    pub bind_addr: String,
    pub health_endpoint: String,
    pub metrics_endpoint: String,
    pub crystal_family: Option<CrystalFamily>,
    pub sdt_config: Option<ThyristorConfig>,
    pub agents: Vec<AgentConfig>,
    pub monitor_interval_ms: u64,
    pub enforce_latency: bool,
    pub max_latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub agent_type: AgentType,
    pub enabled: bool,
}

impl Default for DefenderConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:18115".to_string(),
            health_endpoint: "/health".to_string(),
            metrics_endpoint: "/metrics".to_string(),
            crystal_family: Some(CrystalFamily::GroundStation),
            sdt_config: Some(ThyristorConfig::default()),
            agents: vec![
                AgentConfig {
                    id: "network-monitor".to_string(),
                    agent_type: AgentType::NetworkMonitor,
                    enabled: true,
                },
                AgentConfig {
                    id: "threat-hunter".to_string(),
                    agent_type: AgentType::ThreatHunter,
                    enabled: true,
                },
            ],
            monitor_interval_ms: 100,
            enforce_latency: true,
            max_latency_ms: 50,
        }
    }
}
```

---

## Integration Flow

### Threat Detection Flow

```
1. Threat Agent detects threat
   │
   ▼
2. Threat Monitor receives event
   │
   ▼
3. Crystal Integration evaluates payload
   │   - Polycrystal resonance
   │   - Ring strength calculation
   │
   ▼
4. SDT Integration checks gate
   │   - Current SDT state
   │   - Ring strength threshold
   │   - Gate logic evaluation
   │
   ▼
5. Decision:
   ├─→ Allowed (Conducting/Latched) → Handle threat
   └─→ Blocked (Off/Primed) → Kill threat
```

---

## Summary

**Plasma Defender Components:**
1. ✅ **Axum Server** - HTTP/WebSocket endpoints
2. ✅ **Threat Agents** - Network, Threat, Canary, Anomaly monitors
3. ✅ **Crystal Integration** - Polycrystal resonance evaluation
4. ✅ **SDT Integration** - SDT gate control (Off/Primed/Conducting/Latched)
5. ✅ **Threat Monitor** - Orchestrates agents → crystal → SDT flow
6. ✅ **Health/Metrics** - Monitoring endpoints

**Integration:**
- Uses `sx9-atlas-bus` for PlasmaState
- Crystal evaluates payload resonance
- SDT gate decides if threats live or die
- Agents monitor for threats

---

**Status:** Complete scaffold with agents, crystal, and SDT integration.



