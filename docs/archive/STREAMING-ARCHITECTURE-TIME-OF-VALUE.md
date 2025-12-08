# Streaming Architecture with Time-of-Value Decay
## Pub/Sub + Time-of-Value + WASM Ground Stations + Intel/OSINT

**Date:** December 2025  
**Status:** Architecture Specification  
**Goal:** Complete streaming architecture with time-of-value decay, integrating all databases, WASM ground stations, and planning intel/OSINT equivalent for sx9-ops-orbital

---

## Overview

**Current Stack:**
- **Databases**: Supabase (ACID), SurrealDB (Graph), Sled (Local KV), Sledis (Redis-compatible)
- **Analytical**: GLAF (Graph analysis engine)
- **Missing**: Pub/sub streaming with time-of-value decay
- **WASM Ground Stations**: `ctas7-wasm-ground-station`, `kali-microkernel-tasks`
- **Next Build**: Intel/OSINT equivalent for `sx9-ops-orbital`

**Architecture Principle:** All intelligence fragments have finite operational value with predictable decay curves (RFC-9026).

---

## 1. Streaming Architecture

### 1.1 NATS JetStream + Time-of-Value

**Primary Streaming Backbone:** NATS JetStream with time-of-value decay

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    STREAMING ARCHITECTURE                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │   Supabase   │  │  SurrealDB   │  │     Sled     │                  │
│  │   (ACID)     │  │   (Graph)    │  │   (Local)    │                  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘                  │
│         │                 │                 │                            │
│         └─────────────────┼─────────────────┘                            │
│                          │                                                │
│                          ▼                                                │
│              ┌───────────────────────┐                                    │
│              │   NATS JetStream      │                                    │
│              │  (Pub/Sub + Time-of-  │                                    │
│              │   Value Decay)        │                                    │
│              └───────────┬───────────┘                                    │
│                          │                                                │
│         ┌────────────────┼────────────────┐                              │
│         │                │                │                              │
│         ▼                ▼                ▼                              │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐                            │
│  │  GLAF    │    │  WASM    │    │  Intel   │                            │
│  │(Analytic)│    │  Ground  │    │  OSINT   │                            │
│  │          │    │ Stations │    │ Stations │                            │
│  └──────────┘    └──────────┘    └──────────┘                            │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 1.2 NATS Subjects with Time-of-Value

**Subject Hierarchy:**
```
sx9.stream.intel.{type}.{tier}
sx9.stream.intel.{type}.{tier}.{decay_curve}
sx9.stream.intel.{type}.{tier}.{half_life}
```

**Intel Types:**
- `sx9.stream.intel.sigint.{tier}` - Signals intelligence
- `sx9.stream.intel.humint.{tier}` - Human intelligence
- `sx9.stream.intel.geoint.{tier}` - Geospatial intelligence
- `sx9.stream.intel.osint.{tier}` - Open source intelligence
- `sx9.stream.intel.techint.{tier}` - Technical intelligence
- `sx9.stream.intel.finint.{tier}` - Financial intelligence

**Tiers (Time-of-Value):**
- `realtime` - Half-life: 1 hour (SIGINT: 48hr, HUMINT: 7day, GEOINT: 30day)
- `tactical` - Half-life: 24 hours
- `operational` - Half-life: 7 days
- `strategic` - Half-life: 30 days

### 1.3 Time-of-Value Decay Implementation

**Per RFC-9026 §5.3:**

```rust
// Time-of-value decay trait
pub trait TimeOfValue: Send + Sync {
    /// Calculate current value (0.0-1.0) based on age
    fn current_value(&self, age: Duration) -> f64;
    
    /// Half-life for exponential decay
    fn half_life(&self) -> Duration;
    
    /// Zero-value threshold (when to archive/delete)
    fn zero_value_threshold(&self) -> Duration;
    
    /// Decay curve type
    fn decay_curve(&self) -> DecayCurve;
}

// Intel type implementations
pub struct SIGINTTimeOfValue;
impl TimeOfValue for SIGINTTimeOfValue {
    fn half_life(&self) -> Duration {
        Duration::from_secs(48 * 3600) // 48 hours
    }
    
    fn current_value(&self, age: Duration) -> f64 {
        let half_life_secs = self.half_life().as_secs_f64();
        let age_secs = age.as_secs_f64();
        // Exponential decay: value = e^(-ln(2) * age / half_life)
        0.5_f64.powf(age_secs / half_life_secs)
    }
    
    fn zero_value_threshold(&self) -> Duration {
        Duration::from_secs(7 * 24 * 3600) // 7 days
    }
    
    fn decay_curve(&self) -> DecayCurve {
        DecayCurve::Exponential { half_life: self.half_life() }
    }
}

pub struct HUMINTTimeOfValue;
impl TimeOfValue for HUMINTTimeOfValue {
    fn half_life(&self) -> Duration {
        Duration::from_secs(7 * 24 * 3600) // 7 days
    }
    
    fn current_value(&self, age: Duration) -> f64 {
        let half_life_secs = self.half_life().as_secs_f64();
        let age_secs = age.as_secs_f64();
        0.5_f64.powf(age_secs / half_life_secs)
    }
    
    fn zero_value_threshold(&self) -> Duration {
        Duration::from_secs(30 * 24 * 3600) // 30 days
    }
    
    fn decay_curve(&self) -> DecayCurve {
        DecayCurve::Exponential { half_life: self.half_life() }
    }
}

pub struct GEOINTTimeOfValue;
impl TimeOfValue for GEOINTTimeOfValue {
    fn half_life(&self) -> Duration {
        Duration::from_secs(30 * 24 * 3600) // 30 days
    }
    
    fn current_value(&self, age: Duration) -> f64 {
        let half_life_secs = self.half_life().as_secs_f64();
        let age_secs = age.as_secs_f64();
        0.5_f64.powf(age_secs / half_life_secs)
    }
    
    fn zero_value_threshold(&self) -> Duration {
        Duration::from_secs(90 * 24 * 3600) // 90 days
    }
    
    fn decay_curve(&self) -> DecayCurve {
        DecayCurve::Exponential { half_life: self.half_life() }
    }
}
```

---

## 2. Database Integration

### 2.1 Supabase (ACID) → NATS Stream

**CTAS Tasks (ACID):**
```rust
// Stream CTAS tasks to NATS with time-of-value
async fn stream_ctas_tasks_to_nats(
    supabase: &SupabaseClient,
    nats: &NatsClient
) -> Result<()> {
    // Query active tasks
    let tasks = supabase
        .from("ctas_tasks")
        .select("*")
        .eq("current_state", "active")
        .execute()
        .await?;
    
    for task in tasks {
        // Generate trivariate hash
        let triv_hash = generate_trivariate_hash(&task)?;
        
        // Calculate time-of-value
        let age = now() - task.created_at;
        let time_of_value = match task.hd4_phase {
            "Hunt" => SIGINTTimeOfValue.current_value(age),
            "Detect" => HUMINTTimeOfValue.current_value(age),
            _ => GEOINTTimeOfValue.current_value(age),
        };
        
        // Publish to NATS with time-of-value metadata
        nats.publish(
            &format!("sx9.stream.intel.techint.{}", tier_from_value(time_of_value)),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "supabase",
                entity_type: "ctas_task",
                entity_id: task.id,
                time_of_value,
                age,
                half_life: SIGINTTimeOfValue.half_life(),
                collected_at: task.created_at,
            }
        ).await?;
    }
    
    Ok(())
}
```

### 2.2 SurrealDB (Graph) → NATS Stream

**Graph Relationships:**
```rust
// Stream graph relationships to NATS
async fn stream_surreal_graph_to_nats(
    surreal: &SurrealClient,
    nats: &NatsClient
) -> Result<()> {
    // Query graph relationships
    let relationships = surreal
        .query("SELECT * FROM relationships WHERE active = true")
        .await?;
    
    for rel in relationships {
        // Generate trivariate hash for relationship
        let triv_hash = generate_trivariate_hash(&rel)?;
        
        // Calculate time-of-value (relationships decay slower)
        let age = now() - rel.created_at;
        let time_of_value = GEOINTTimeOfValue.current_value(age);
        
        // Publish to NATS
        nats.publish(
            &format!("sx9.stream.intel.geoint.{}", tier_from_value(time_of_value)),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "surrealdb",
                entity_type: "relationship",
                entity_id: rel.id,
                time_of_value,
                age,
                half_life: GEOINTTimeOfValue.half_life(),
                collected_at: rel.created_at,
            }
        ).await?;
    }
    
    Ok(())
}
```

### 2.3 Sled/Sledis (Cache) → NATS Stream

**High-Speed Cache:**
```rust
// Stream cache entries to NATS (fast decay)
async fn stream_sledis_cache_to_nats(
    sledis: &SledisClient,
    nats: &NatsClient
) -> Result<()> {
    // Scan cache entries
    for (key, value) in sledis.scan("cache:*") {
        // Calculate age
        let age = now() - value.timestamp;
        
        // Cache has fast decay (1 hour half-life)
        let time_of_value = 0.5_f64.powf(age.as_secs_f64() / 3600.0);
        
        if time_of_value > 0.1 { // Only stream if value > 10%
            nats.publish(
                "sx9.stream.intel.techint.realtime",
                &IntelligenceFragment {
                    trivariate_hash: key_to_hash(&key),
                    source: "sledis",
                    entity_type: "cache",
                    entity_id: key,
                    time_of_value,
                    age,
                    half_life: Duration::from_secs(3600),
                    collected_at: value.timestamp,
                }
            ).await?;
        }
    }
    
    Ok(())
}
```

### 2.4 GLAF (Analytical) → NATS Stream

**Graph Analysis Results:**
```rust
// Stream GLAF analysis to NATS
async fn stream_glaf_analysis_to_nats(
    glaf: &GLAFClient,
    nats: &NatsClient
) -> Result<()> {
    // Get network analysis results
    let networks = glaf.generate_networks().await?;
    
    for network in networks {
        // Generate trivariate hash for network
        let triv_hash = generate_trivariate_hash(&network)?;
        
        // Analytical results have medium decay (24 hour half-life)
        let age = now() - network.analyzed_at;
        let time_of_value = 0.5_f64.powf(age.as_secs_f64() / (24.0 * 3600.0));
        
        nats.publish(
            &format!("sx9.stream.intel.techint.{}", tier_from_value(time_of_value)),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "glaf",
                entity_type: "network_analysis",
                entity_id: network.id,
                time_of_value,
                age,
                half_life: Duration::from_secs(24 * 3600),
                collected_at: network.analyzed_at,
            }
        ).await?;
    }
    
    Ok(())
}
```

---

## 3. WASM Ground Stations

### 3.1 Current WASM Ground Stations

**`ctas7-wasm-ground-station`:**
- Cesium WebSocket server (Port 18400)
- 257 ground stations
- Real-time telemetry
- WASM-compatible (browser/Firefly/edge)

**`kali-microkernel-tasks`:**
- Microkernel task executor
- Unicode task expressions
- Ephemeral containers
- Port scanner, web scanner, password cracker

### 3.2 WASM Ground Station → NATS Stream

**Streaming Architecture:**
```rust
// WASM Ground Station streaming to NATS
pub struct WASMGroundStationStream {
    station_id: String,
    nats_client: NatsClient,
    time_of_value: Box<dyn TimeOfValue>,
}

impl WASMGroundStationStream {
    pub async fn stream_telemetry(&self, telemetry: TelemetryData) -> Result<()> {
        // Generate trivariate hash
        let triv_hash = generate_trivariate_hash(&telemetry)?;
        
        // Calculate time-of-value (telemetry has fast decay)
        let age = now() - telemetry.timestamp;
        let time_of_value = self.time_of_value.current_value(age);
        
        // Publish to NATS
        self.nats_client.publish(
            &format!("sx9.stream.intel.geoint.realtime"),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "wasm_ground_station",
                entity_type: "telemetry",
                entity_id: self.station_id.clone(),
                time_of_value,
                age,
                half_life: self.time_of_value.half_life(),
                collected_at: telemetry.timestamp,
                payload: telemetry,
            }
        ).await?;
        
        Ok(())
    }
}
```

### 3.3 Kali Microkernel → NATS Stream

**Tool Execution Results:**
```rust
// Kali microkernel task results → NATS
pub struct KaliMicrokernelStream {
    nats_client: NatsClient,
    time_of_value: Box<dyn TimeOfValue>,
}

impl KaliMicrokernelStream {
    pub async fn stream_task_result(&self, result: TaskResult) -> Result<()> {
        // Generate trivariate hash from unicode task expression
        let triv_hash = generate_trivariate_hash(&result.task_expression)?;
        
        // Tool results have medium decay (12 hour half-life)
        let age = now() - result.executed_at;
        let time_of_value = 0.5_f64.powf(age.as_secs_f64() / (12.0 * 3600.0));
        
        // Publish to NATS
        self.nats_client.publish(
            &format!("sx9.stream.intel.techint.{}", tier_from_value(time_of_value)),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "kali_microkernel",
                entity_type: "tool_result",
                entity_id: result.task_id,
                time_of_value,
                age,
                half_life: Duration::from_secs(12 * 3600),
                collected_at: result.executed_at,
                payload: result,
            }
        ).await?;
        
        Ok(())
    }
}
```

---

## 4. Intel/OSINT Ground Stations (sx9-ops-orbital)

### 4.1 Architecture

**Intel/OSINT equivalent of WASM ground stations:**

```
┌─────────────────────────────────────────────────────────────────────────┐
│              SX9-OPS-ORBITAL: Intel/OSINT Ground Stations                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │   OSINT      │  │    Intel     │  │   Threat     │                  │
│  │  Collector   │  │   Processor  │  │   Feeds      │                  │
│  │              │  │              │  │              │                  │
│  │ • Shodan    │  │ • PTCC-TETH  │  │ • MITRE      │                  │
│  │ • Maltego   │  │ • GLAF       │  │ • ATT&CK     │                  │
│  │ • theHarvester│ │ • GNN       │  │ • CVE        │                  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘                  │
│         │                 │                 │                            │
│         └─────────────────┼─────────────────┘                            │
│                          │                                                │
│                          ▼                                                │
│              ┌───────────────────────┐                                    │
│              │   NATS JetStream      │                                    │
│              │  (Time-of-Value)     │                                    │
│              └───────────┬───────────┘                                    │
│                          │                                                │
│                          ▼                                                │
│              ┌───────────────────────┐                                    │
│              │   GLAF Analytical     │                                    │
│              │   Engine              │                                    │
│              └───────────────────────┘                                    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2 OSINT Collector

**Similar to WASM ground station but for OSINT:**

```rust
// OSINT Ground Station (Intel equivalent)
pub struct OSINTGroundStation {
    station_id: String,
    collectors: Vec<Box<dyn OSINTCollector>>,
    nats_client: NatsClient,
    time_of_value: Box<dyn TimeOfValue>,
}

pub trait OSINTCollector: Send + Sync {
    async fn collect(&self) -> Result<OSINTData>;
    fn source_name(&self) -> &str;
    fn half_life(&self) -> Duration;
}

// Shodan collector
pub struct ShodanCollector;
impl OSINTCollector for ShodanCollector {
    async fn collect(&self) -> Result<OSINTData> {
        // Collect from Shodan API
        Ok(OSINTData {
            source: "shodan",
            data: shodan_api.query().await?,
            timestamp: now(),
        })
    }
    
    fn source_name(&self) -> &str { "shodan" }
    fn half_life(&self) -> Duration { Duration::from_secs(24 * 3600) } // 24 hours
}

// Maltego collector
pub struct MaltegoCollector;
impl OSINTCollector for MaltegoCollector {
    async fn collect(&self) -> Result<OSINTData> {
        // Collect from Maltego transforms
        Ok(OSINTData {
            source: "maltego",
            data: maltego_transform.execute().await?,
            timestamp: now(),
        })
    }
    
    fn source_name(&self) -> &str { "maltego" }
    fn half_life(&self) -> Duration { Duration::from_secs(7 * 24 * 3600) } // 7 days
}

impl OSINTGroundStation {
    pub async fn collect_and_stream(&self) -> Result<()> {
        for collector in &self.collectors {
            let osint_data = collector.collect().await?;
            
            // Generate trivariate hash
            let triv_hash = generate_trivariate_hash(&osint_data)?;
            
            // Calculate time-of-value
            let age = now() - osint_data.timestamp;
            let time_of_value = self.time_of_value.current_value(age);
            
            // Publish to NATS
            self.nats_client.publish(
                &format!("sx9.stream.intel.osint.{}", tier_from_value(time_of_value)),
                &IntelligenceFragment {
                    trivariate_hash: triv_hash,
                    source: "osint_ground_station",
                    entity_type: "osint_data",
                    entity_id: self.station_id.clone(),
                    time_of_value,
                    age,
                    half_life: collector.half_life(),
                    collected_at: osint_data.timestamp,
                    payload: osint_data,
                }
            ).await?;
        }
        
        Ok(())
    }
}
```

### 4.3 Intel Processor

**PTCC-TETH + GLAF + GNN integration:**

```rust
// Intel Processor (Cognitive equivalent)
pub struct IntelProcessor {
    ptcc_teth: PTCC_TETH_Client,
    glaf: GLAFClient,
    gnn: GNNClient,
    nats_client: NatsClient,
    time_of_value: Box<dyn TimeOfValue>,
}

impl IntelProcessor {
    pub async fn process_intelligence(&self, intel: IntelligenceData) -> Result<()> {
        // 1. Process through PTCC-TETH
        let ptcc_result = self.ptcc_teth.analyze(&intel).await?;
        
        // 2. Generate GLAF graph
        let glaf_graph = self.glaf.generate_graph(&intel).await?;
        
        // 3. GNN analysis
        let gnn_result = self.gnn.analyze(&glaf_graph).await?;
        
        // 4. Generate trivariate hash
        let triv_hash = generate_trivariate_hash(&intel)?;
        
        // 5. Calculate time-of-value (intel has slow decay)
        let age = now() - intel.collected_at;
        let time_of_value = self.time_of_value.current_value(age);
        
        // 6. Publish to NATS
        self.nats_client.publish(
            &format!("sx9.stream.intel.humint.{}", tier_from_value(time_of_value)),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "intel_processor",
                entity_type: "intelligence",
                entity_id: intel.id,
                time_of_value,
                age,
                half_life: self.time_of_value.half_life(),
                collected_at: intel.collected_at,
                payload: IntelResult {
                    ptcc: ptcc_result,
                    glaf: glaf_graph,
                    gnn: gnn_result,
                },
            }
        ).await?;
        
        Ok(())
    }
}
```

---

## 5. OODA Vertical Escalation Integration

### 5.1 OODA Loop → NATS Stream

**Per RFC-9022:**

```rust
// OODA loop streaming with time-of-value
pub struct OODAStream {
    ooda_loop: OODALoop,
    nats_client: NatsClient,
    time_of_value: Box<dyn TimeOfValue>,
}

impl OODAStream {
    pub async fn stream_ooda_cycle(&mut self) -> Result<()> {
        // OBSERVE
        self.ooda_loop.phase = OODAPhase::Observe;
        let observations = self.ooda_loop.collect_observations().await?;
        
        // Stream observations with time-of-value
        for obs in &observations {
            let triv_hash = generate_trivariate_hash(obs)?;
            let age = now() - obs.timestamp;
            let time_of_value = self.time_of_value.current_value(age);
            
            self.nats_client.publish(
                &format!("sx9.stream.ooda.{}.observe", self.ooda_loop.level),
                &IntelligenceFragment {
                    trivariate_hash: triv_hash,
                    source: "ooda_loop",
                    entity_type: "observation",
                    entity_id: obs.id,
                    time_of_value,
                    age,
                    half_life: self.time_of_value.half_life(),
                    collected_at: obs.timestamp,
                    payload: obs.clone(),
                }
            ).await?;
        }
        
        // ORIENT (Convergence calculation)
        self.ooda_loop.phase = OODAPhase::Orient;
        let convergence = self.ooda_loop.calculate_convergence(&observations);
        
        // Stream convergence with time-of-value
        let triv_hash = generate_trivariate_hash(&convergence)?;
        self.nats_client.publish(
            &format!("sx9.stream.ooda.{}.orient", self.ooda_loop.level),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "ooda_loop",
                entity_type: "convergence",
                entity_id: self.ooda_loop.id,
                time_of_value: 1.0, // Convergence is current
                age: Duration::ZERO,
                half_life: Duration::from_secs(3600), // 1 hour
                collected_at: now(),
                payload: convergence,
            }
        ).await?;
        
        // DECIDE
        self.ooda_loop.phase = OODAPhase::Decide;
        let decision = if convergence.above_threshold() {
            self.ooda_loop.decide_action()
        } else {
            Decision::HuntMore
        };
        
        // ACT (or ESCALATE)
        self.ooda_loop.phase = OODAPhase::Act;
        match decision {
            Decision::Execute(hd4_phase) => {
                // Stream execution
                self.stream_execution(hd4_phase).await?;
            }
            Decision::Escalate(reason) => {
                // Stream escalation
                self.stream_escalation(reason).await?;
            }
            Decision::HuntMore => {
                // Cycle back to Observe
                return self.stream_ooda_cycle().await;
            }
        }
        
        Ok(())
    }
}
```

### 5.2 Vertical Escalation Streaming

**Tactical → Operational → Strategic → National:**

```rust
// Vertical escalation streaming
pub async fn stream_vertical_escalation(
    escalation: EscalationPacket,
    nats: &NatsClient
) -> Result<()> {
    // Generate trivariate hash
    let triv_hash = generate_trivariate_hash(&escalation)?;
    
    // Escalations have immediate value (no decay)
    nats.publish(
        &format!("sx9.stream.escalation.{}.{}", escalation.source_level, escalation.target_level),
        &IntelligenceFragment {
            trivariate_hash: triv_hash,
            source: "ooda_escalation",
            entity_type: "escalation",
            entity_id: escalation.id,
            time_of_value: 1.0, // Immediate value
            age: Duration::ZERO,
            half_life: Duration::from_secs(3600), // 1 hour
            collected_at: now(),
            payload: escalation,
        }
    ).await?;
    
    Ok(())
}
```

---

## 6. Time-of-Value Decay Engine

### 6.1 Decay Monitor

**Monitors all streams and applies decay:**

```rust
// Time-of-value decay monitor
pub struct TimeOfValueDecayMonitor {
    nats_client: NatsClient,
    decay_engine: DecayEngine,
}

impl TimeOfValueDecayMonitor {
    pub async fn monitor_and_decay(&self) -> Result<()> {
        // Subscribe to all intelligence streams
        let mut subscriber = self.nats_client
            .subscribe("sx9.stream.intel.>")
            .await?;
        
        while let Some(msg) = subscriber.next().await {
            let fragment: IntelligenceFragment = serde_json::from_slice(&msg.data)?;
            
            // Calculate current value
            let age = now() - fragment.collected_at;
            let current_value = self.decay_engine.calculate_value(
                &fragment.decay_curve,
                age,
                fragment.half_life
            );
            
            // If value < threshold, archive or delete
            if current_value < 0.1 {
                // Archive to cold storage
                self.archive_fragment(&fragment).await?;
                
                // Delete from active stream
                self.nats_client.delete_message(&msg).await?;
            } else {
                // Update fragment with current value
                fragment.time_of_value = current_value;
                
                // Republish with updated value
                self.nats_client.publish(
                    &msg.subject,
                    &fragment
                ).await?;
            }
        }
        
        Ok(())
    }
}
```

### 6.2 Decay Curves

**Per RFC-9026:**

| Intel Type | Half-Life | Decay Curve | Zero Threshold |
|------------|-----------|-------------|----------------|
| SIGINT | 48 hours | Exponential | 7 days |
| HUMINT | 7 days | Exponential | 30 days |
| GEOINT | 30 days | Exponential | 90 days |
| OSINT | 24 hours | Exponential | 7 days |
| TECHINT | 12 hours | Exponential | 3 days |
| Cache | 1 hour | Exponential | 6 hours |

---

## 7. Integration with GLAF

### 7.1 GLAF Ingestion from NATS

**GLAF consumes from NATS streams:**

```rust
// GLAF ingestion from NATS
pub struct GLAFNatsIngestion {
    glaf: GLAFClient,
    nats_client: NatsClient,
}

impl GLAFNatsIngestion {
    pub async fn ingest_from_nats(&self) -> Result<()> {
        // Subscribe to all intelligence streams
        let mut subscriber = self.nats_client
            .subscribe("sx9.stream.intel.>")
            .await?;
        
        while let Some(msg) = subscriber.next().await {
            let fragment: IntelligenceFragment = serde_json::from_slice(&msg.data)?;
            
            // Only ingest if time-of-value > threshold
            if fragment.time_of_value > 0.5 {
                // Ingest into GLAF
                self.glaf.ingest(&fragment).await?;
            }
        }
        
        Ok(())
    }
}
```

### 7.2 GLAF Analysis → NATS Stream

**GLAF analysis results streamed back:**

```rust
// GLAF analysis streaming
pub async fn stream_glaf_analysis(
    glaf: &GLAFClient,
    nats: &NatsClient
) -> Result<()> {
    // Generate networks
    let networks = glaf.generate_networks().await?;
    
    for network in networks {
        // Generate trivariate hash
        let triv_hash = generate_trivariate_hash(&network)?;
        
        // Analysis has medium decay (24 hours)
        let age = now() - network.analyzed_at;
        let time_of_value = 0.5_f64.powf(age.as_secs_f64() / (24.0 * 3600.0));
        
        // Stream to NATS
        nats.publish(
            &format!("sx9.stream.glaf.analysis.{}", network.network_type),
            &IntelligenceFragment {
                trivariate_hash: triv_hash,
                source: "glaf",
                entity_type: "network_analysis",
                entity_id: network.id,
                time_of_value,
                age,
                half_life: Duration::from_secs(24 * 3600),
                collected_at: network.analyzed_at,
                payload: network,
            }
        ).await?;
    }
    
    Ok(())
}
```

---

## 8. Complete Data Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    COMPLETE STREAMING DATA FLOW                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  1. DATABASES → NATS                                                    │
│     Supabase (CTAS tasks) → sx9.stream.intel.techint.{tier}            │
│     SurrealDB (Graph) → sx9.stream.intel.geoint.{tier}                 │
│     Sled/Sledis (Cache) → sx9.stream.intel.techint.realtime             │
│                                                                          │
│  2. WASM GROUND STATIONS → NATS                                         │
│     ctas7-wasm-ground-station → sx9.stream.intel.geoint.realtime       │
│     kali-microkernel-tasks → sx9.stream.intel.techint.{tier}          │
│                                                                          │
│  3. INTEL/OSINT GROUND STATIONS → NATS (sx9-ops-orbital)                │
│     OSINT Collector → sx9.stream.intel.osint.{tier}                    │
│     Intel Processor → sx9.stream.intel.humint.{tier}                    │
│                                                                          │
│  4. OODA LOOPS → NATS                                                   │
│     Tactical OODA → sx9.stream.ooda.tactical.{phase}                    │
│     Operational OODA → sx9.stream.ooda.operational.{phase}                │
│     Strategic OODA → sx9.stream.ooda.strategic.{phase}                 │
│     National OODA → sx9.stream.ooda.national.{phase}                   │
│                                                                          │
│  5. TIME-OF-VALUE DECAY ENGINE                                          │
│     Monitor all streams → Calculate decay → Archive/Delete              │
│                                                                          │
│  6. GLAF ANALYTICAL ENGINE                                              │
│     Ingest from NATS → Generate networks → Stream analysis back         │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 9. Implementation Plan

### Phase 1: NATS JetStream Setup (Week 1)
1. Deploy NATS JetStream
2. Create streams for all intel types
3. Configure time-of-value subjects
4. Set up decay monitoring

### Phase 2: Database Integration (Week 2)
1. Supabase → NATS streaming
2. SurrealDB → NATS streaming
3. Sled/Sledis → NATS streaming
4. GLAF → NATS streaming

### Phase 3: WASM Ground Stations (Week 3)
1. ctas7-wasm-ground-station → NATS
2. kali-microkernel-tasks → NATS
3. Time-of-value decay for tool results

### Phase 4: Intel/OSINT Ground Stations (Week 4 - sx9-ops-orbital)
1. OSINT Collector implementation
2. Intel Processor (PTCC-TETH + GLAF + GNN)
3. Integration with NATS streams
4. Time-of-value decay

### Phase 5: OODA Integration (Week 5)
1. OODA loop streaming
2. Vertical escalation streaming
3. Integration with time-of-value

---

## 10. Performance Targets

| Operation | Target | Measurement |
|-----------|--------|-------------|
| NATS publish latency | <1ms | Message publish time |
| Time-of-value calculation | <10μs | Decay calculation |
| Stream ingestion | 100K msg/sec | Throughput |
| Decay monitoring | <100ms | Archive/delete latency |
| GLAF ingestion | <50ms | Network generation |

---

**Status:** Ready for implementation

**Next Build:** sx9-ops-orbital with Intel/OSINT ground stations



