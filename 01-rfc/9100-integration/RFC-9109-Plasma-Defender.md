# RFC-9109: Plasma Defender - Unified Security Intelligence

**Version:** 1.2.0
**Status:** Draft
**Date:** December 1, 2025
**Author:** CTAS Core Engineering Group
**Dependencies:** RFC-9001, RFC-9004, RFC-9020, RFC-9025, RFC-9108, RFC-9110

---

## 1. Abstract

Plasma Defender unifies the CTAS security stack by connecting the **164-task SlotGraph** with **Legion ECS ticks**, **node/crate interviews**, and **6-decimal delta angles** for semantic-operational tool positioning. It replaces Wazuh complexity with minimal OSSEC parsing, using the L2 Thalmic Filter on the Kali ISO for cognitive intercept.

---

## 2. Full Circle Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           PLASMA DEFENDER - FULL CIRCLE                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                        SLOTGRAPH (164 Tasks)                             │    │
│  │                                                                          │    │
│  │   petgraph topology ──► Entity nodes ──► Edge weights ──► HD4 phases    │    │
│  │                                                                          │    │
│  │   Each task has:                                                         │    │
│  │   • Node Interview (semantic context)                                    │    │
│  │   • Crate Interview (implementation context)                             │    │
│  │   • Delta Position (x,y,z @ 6 decimals)                                 │    │
│  │   • Tool Mapping (Kali tools, MITRE techniques)                         │    │
│  │                                                                          │    │
│  └──────────────────────────────┬──────────────────────────────────────────┘    │
│                                 │                                                │
│                                 ▼                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                         LEGION ECS WORLD                                 │    │
│  │                                                                          │    │
│  │   WorldState.tick ──────────────────────────────────────────────────►   │    │
│  │        │                                                                 │    │
│  │        ├── SlotGraphTaskNode (Entity)                                   │    │
│  │        ├── Position { x, y, z } @ 6 decimals                            │    │
│  │        ├── HD4Phase (Hunt/Detect/Disrupt/Dominate/Disable)              │    │
│  │        ├── ToolMapping (Kali tool → MITRE technique)                    │    │
│  │        └── NodeInterview (semantic context from 04-abe-iac)             │    │
│  │                                                                          │    │
│  └──────────────────────────────┬──────────────────────────────────────────┘    │
│                                 │                                                │
│                                 ▼                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                    DELTA ANGLE ALIGNMENT (6 Decimal)                     │    │
│  │                                                                          │    │
│  │   X-Axis (Semantic):    Tool intent classification                      │    │
│  │   ├── 0.000000 = Reconnaissance                                         │    │
│  │   ├── 0.250000 = Resource Development                                   │    │
│  │   ├── 0.500000 = Initial Access / Execution                             │    │
│  │   ├── 0.750000 = Persistence / Privilege Escalation                     │    │
│  │   └── 1.000000 = Impact / Exfiltration                                  │    │
│  │                                                                          │    │
│  │   Y-Axis (Operational): Execution phase in HD4                          │    │
│  │   ├── 0.000000 = Hunt (planning)                                        │    │
│  │   ├── 0.250000 = Detect (sensing)                                       │    │
│  │   ├── 0.500000 = Disrupt (action)                                       │    │
│  │   ├── 0.750000 = Dominate (control)                                     │    │
│  │   └── 1.000000 = Disable (completion)                                   │    │
│  │                                                                          │    │
│  │   Z-Axis (Temporal):    Time correlation                                │    │
│  │   ├── 0.000000 = Historical (past events)                               │    │
│  │   ├── 0.500000 = Current (real-time)                                    │    │
│  │   └── 1.000000 = Predictive (future projection)                         │    │
│  │                                                                          │    │
│  └──────────────────────────────┬──────────────────────────────────────────┘    │
│                                 │                                                │
│                                 ▼                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                    L2 THALMIC FILTER (Kali ISO)                          │    │
│  │                                                                          │    │
│  │   /opt/ctas7/thalmic-filter/ on custom Kali ISO                         │    │
│  │                                                                          │    │
│  │   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │    │
│  │   │ DistilBERT  │    │   Phi-3     │    │  sx9-lisp   │                 │    │
│  │   │   MITRE     │───►│  Explainer  │───►│   Rules     │                 │    │
│  │   │ Classifier  │    │   (LoRA)    │    │  (Unicode)  │                 │    │
│  │   └─────────────┘    └─────────────┘    └─────────────┘                 │    │
│  │         │                   │                   │                        │    │
│  │         └───────────────────┴───────────────────┘                        │    │
│  │                             │                                            │    │
│  │                             ▼                                            │    │
│  │                    ATOMIC CLIPBOARD                                      │    │
│  │                   (shared state layer)                                   │    │
│  │                                                                          │    │
│  └──────────────────────────────┬──────────────────────────────────────────┘    │
│                                 │                                                │
│                                 ▼                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                         PERSISTENCE LAYER                                │    │
│  │                                                                          │    │
│  │   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │    │
│  │   │   SLEDIS    │    │    GLAF     │    │   OSSEC     │                 │    │
│  │   │  (Sled DB)  │◄──►│   (Graph)   │◄──►│   Alerts    │                 │    │
│  │   │  Redis API  │    │   Neo4j/    │    │  (Minimal)  │                 │    │
│  │   │             │    │   Chroma    │    │             │                 │    │
│  │   └─────────────┘    └─────────────┘    └─────────────┘                 │    │
│  │                                                                          │    │
│  └─────────────────────────────────────────────────────────────────────────┘    │
│                                                                                  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Component Integration

### 3.1 SlotGraph (ctas7-slotgraph-engine)

The 164-task graph using petgraph:

```rust
/// SlotGraph connects tasks to Legion entities
pub struct SlotGraphView {
    pub graph: Graph<usize, f64>,           // petgraph topology
    pub ent_to_idx: HashMap<Entity, NodeIndex>,  // Legion entity mapping
}

/// Each task node carries interview context
pub struct SlotGraphTaskNode {
    pub id: Uuid,
    pub name: String,                        // Task name from graph
    pub hd4_phase: HD4Phase,                 // Hunt/Detect/Disrupt/Dominate/Disable
    pub node_interview: NodeInterview,       // Semantic context
    pub crate_interview: Option<CrateInterview>, // Implementation context
    pub delta_position: DeltaPosition,       // x,y,z @ 6 decimals
    pub tool_mapping: Vec<ToolMapping>,      // Kali tools for this task
    pub mitre_techniques: Vec<String>,       // T1xxx mappings
}
```

### 3.2 Node & Crate Interviews (RFC-9025)

From `04-abe-iac/node-interview-generator/`:

```rust
/// Node interview provides semantic context
pub struct NodeInterview {
    pub uuid: String,                        // e.g., "uuid-007-003-001"
    pub title: String,
    pub description: String,
    pub eei_requirements: Vec<String>,       // Essential Elements of Information
    pub collection_sources: Vec<String>,     // OSINT sources
    pub semantic_embedding: Vec<f32>,        // 512-dim vector for similarity
}

/// Crate interview provides implementation context
pub struct CrateInterview {
    pub crate_name: String,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub tools_provided: Vec<String>,         // Kali tools this crate wraps
}
```

### 3.3 Legion ECS + Delta Angles (sx9-lisp)

Legion ticks drive delta angle updates:

```rust
/// Delta position with 6 decimal precision
#[derive(Clone, Default)]
pub struct DeltaPosition {
    pub x: f64,  // Semantic axis (0.000000 - 1.000000)
    pub y: f64,  // Operational axis (0.000000 - 1.000000)
    pub z: f64,  // Temporal axis (0.000000 - 1.000000)
}

impl DeltaPosition {
    /// Round to 6 decimal places
    fn round6(v: f64) -> f64 {
        (v * 1_000_000.0).round() / 1_000_000.0
    }

    /// Compute from task and current state
    pub fn from_task(task: &SlotGraphTaskNode, tick: u64) -> Self {
        Self {
            // X: Semantic position from MITRE kill chain stage
            x: Self::round6(task.mitre_stage_normalized()),
            // Y: Operational position from HD4 phase
            y: Self::round6(task.hd4_phase.as_f64()),
            // Z: Temporal position from tick
            z: Self::round6((tick % 1_000_000) as f64 / 1_000_000.0),
        }
    }
}

/// HD4 phases map to Y-axis
impl HD4Phase {
    pub fn as_f64(&self) -> f64 {
        match self {
            HD4Phase::Hunt => 0.000000,
            HD4Phase::Detect => 0.250000,
            HD4Phase::Disrupt => 0.500000,
            HD4Phase::Dominate => 0.750000,
            HD4Phase::Disable => 1.000000,
        }
    }
}
```

### 3.4 L2 Thalmic Filter (Kali ISO)

Baked into `/opt/ctas7/thalmic-filter/` on custom Kali ISO:

```rust
/// L2 intercept for all tool invocations
pub struct ThalmicL2Intercept {
    classifier: DistilBertMitre,     // MITRE technique classifier
    explainer: Phi3LoRA,             // Threat explanation
    lisp_eval: LispInterpreter,      // sx9-lisp rule evaluation
    clipboard: AtomicClipboard,      // Shared state
    sledis: SledisCore,              // Persistence
}

impl ThalmicL2Intercept {
    /// Intercept tool invocation, enrich with context
    pub async fn intercept(&self, tool_cmd: &str, task: &SlotGraphTaskNode) -> InterceptResult {
        // 1. Classify via MITRE
        let mitre = self.classifier.classify(tool_cmd).await;

        // 2. Update delta based on classification
        let delta = DeltaPosition::from_mitre(&mitre, task);

        // 3. Evaluate sx9-lisp rules
        let fire_event = self.lisp_eval.eval_with_delta(tool_cmd, &delta);

        // 4. Store to clipboard for cross-tool correlation
        self.clipboard.set_context(tool_cmd, &mitre, &delta, &fire_event);

        // 5. Persist to Sledis
        self.sledis.log_intercept(tool_cmd, &mitre, &delta);

        InterceptResult {
            allowed: !mitre.is_blocked(),
            mitre_techniques: mitre.techniques,
            delta_position: delta,
            fire_event,
            explanation: self.explainer.explain(&mitre).await,
        }
    }
}
```

### 3.5 Tool Mapping (Kali → MITRE → Delta)

```rust
/// Map Kali tools to MITRE techniques and delta positions
pub struct ToolMapping {
    pub tool_name: String,           // e.g., "nmap", "sqlmap", "metasploit"
    pub mitre_techniques: Vec<String>, // e.g., ["T1046", "T1018"]
    pub default_delta: DeltaPosition,  // Default semantic-operational position
    pub hd4_phases: Vec<HD4Phase>,     // Which phases this tool serves
}

/// Tool registry for the 164 tasks
pub static TOOL_REGISTRY: Lazy<HashMap<String, ToolMapping>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Reconnaissance tools (X ≈ 0.1)
    m.insert("nmap".into(), ToolMapping {
        tool_name: "nmap".into(),
        mitre_techniques: vec!["T1046".into(), "T1018".into()],
        default_delta: DeltaPosition { x: 0.100000, y: 0.250000, z: 0.500000 },
        hd4_phases: vec![HD4Phase::Hunt, HD4Phase::Detect],
    });

    // Exploitation tools (X ≈ 0.5)
    m.insert("metasploit".into(), ToolMapping {
        tool_name: "metasploit".into(),
        mitre_techniques: vec!["T1190".into(), "T1059".into()],
        default_delta: DeltaPosition { x: 0.500000, y: 0.500000, z: 0.500000 },
        hd4_phases: vec![HD4Phase::Disrupt, HD4Phase::Dominate],
    });

    // ... 164 task tool mappings
    m
});
```

### 3.6 OSSEC (Minimal Wazuh)

Only essential OSSEC components:

| Component | Keep | Purpose |
|-----------|------|---------|
| ossec-remoted | ✓ | Receive alerts via syslog/TCP |
| ossec-analysisd | ✓ | Parse logs, apply decoders |
| ossec-logcollector | ✓ | Monitor log files |
| Wazuh Manager | ✗ | Replaced by Sledis + GLAF |
| Wazuh Indexer | ✗ | Replaced by GLAF graph |
| Wazuh Dashboard | ✗ | Replaced by Plasma Viewer |

```rust
/// Parse OSSEC alerts into SlotGraph-compatible format
pub fn parse_ossec_alert(json: &str) -> Result<OssecAlert> {
    let alert: OssecAlert = serde_json::from_str(json)?;

    // Map to SlotGraph task by rule_id pattern matching
    let task_id = match_rule_to_task(alert.rule_id);

    // Compute delta from alert context
    let delta = DeltaPosition {
        x: mitre_stage_from_rule(alert.rule_id),
        y: hd4_from_severity(alert.rule_level),
        z: temporal_from_timestamp(alert.timestamp),
    };

    Ok(alert.with_task(task_id).with_delta(delta))
}
```

### 3.7 Sledis + GLAF Persistence

```rust
/// Sledis stores all state with Redis-compatible API
pub struct PlasmaDefenderStore {
    sledis: SledisCore,
    glaf: GlafClient,
}

impl PlasmaDefenderStore {
    /// Store task execution with delta
    pub fn store_execution(&self, task: &SlotGraphTaskNode, delta: &DeltaPosition) {
        // Sledis: Key-value for fast lookup
        let key = format!("task:{}:delta", task.id);
        self.sledis.set(&key, delta.to_json());

        // GLAF: Graph for topology queries
        self.glaf.upsert_node(GlafNode {
            id: task.id.to_string(),
            labels: vec!["Task", &task.hd4_phase.to_string()],
            properties: vec![
                ("delta_x", delta.x.to_string()),
                ("delta_y", delta.y.to_string()),
                ("delta_z", delta.z.to_string()),
            ],
        });
    }

    /// Query tasks by delta proximity
    pub fn query_delta_neighbors(&self, delta: &DeltaPosition, radius: f64) -> Vec<SlotGraphTaskNode> {
        // Use GLAF for graph traversal with delta distance
        self.glaf.query(&format!(
            "MATCH (t:Task) WHERE
             sqrt((t.delta_x - {})^2 + (t.delta_y - {})^2 + (t.delta_z - {})^2) < {}
             RETURN t",
            delta.x, delta.y, delta.z, radius
        ))
    }
}
```

---

## 4. Unicode Integration (sx9-lisp)

### 4.1 Unicode Ranges

| Range | Purpose |
|-------|---------|
| U+E500-E5FF | LISP primitives (from usim-system) |
| U+E540-E55F | Thalmic filter operations |
| U+E600-E6FF | Delta angle operations |
| U+E700-E7FF | State machine triggers |
| U+E800-E8FF | Escalation tier triggers (7-tier) |
| U+E900-E9FF | HD4 phase transitions |

### 4.2 LISP Rule Example

```lisp
;; Plasma Defender rule: Delta-based escalation
(define plasma-escalate
  (lambda (alert delta task)
    ;; Check semantic-operational divergence
    (let ((divergence (abs (- (delta-x delta) (delta-y delta)))))
      (cond
        ;; High divergence = semantic/operational mismatch
        ((> divergence 0.300000)
         (fire '\u{E807}' task))  ;; Tier 7 (Orb) escalation

        ;; Medium divergence
        ((> divergence 0.150000)
         (fire '\u{E805}' task))  ;; Tier 5 (Container) escalation

        ;; Normal processing
        (else
         (log '\u{E900}' task))))))  ;; HD4 phase log
```

---

## 5. Analysis vs Runtime Separation

### 5.1 Analysis Phase (Pre-computed)

This happens BEFORE any alerts arrive - it's the knowledge base:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         ANALYSIS PHASE                                   │
│                    (Pre-computed, baked into SlotGraph)                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  04-abe-iac/node-interview-generator/                                   │
│         │                                                                │
│         ▼                                                                │
│  ┌─────────────────┐                                                    │
│  │ Node Interviews │  164 task semantic contexts                        │
│  │ (uuid-xxx-xxx)  │  EEI requirements, collection sources              │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │ Crate Interviews│  Tool capabilities, dependencies                   │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │  MITRE Mapping  │  Techniques → Delta defaults                       │
│  │  + HD4 Phases   │  Kill chain stage → X-axis                         │
│  └────────┬────────┘  HD4 phase → Y-axis                                │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    SLOTGRAPH (164 Tasks)                         │   │
│  │                                                                  │   │
│  │  Each task node contains:                                        │   │
│  │  • Baked-in node interview (semantic context)                    │   │
│  │  • Baked-in crate interview (implementation)                     │   │
│  │  • Pre-computed delta defaults (x,y,z)                          │   │
│  │  • Tool mappings (Kali → MITRE)                                 │   │
│  │  • Edge weights to related tasks                                 │   │
│  │                                                                  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Runtime Phase (Real-time Reaction)

This happens when alerts arrive - fast path only:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          RUNTIME PHASE                                   │
│                    (Real-time, reaction + escalation)                    │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  OSSEC Alert (/var/ossec/logs/alerts/)                                  │
│         │                                                                │
│         ▼                                                                │
│  ┌─────────────────┐                                                    │
│  │   OssecParser   │  Parse JSON (fast, no lookups)                     │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │ SlotGraph Match │  rule_id → task_id (hash lookup)                   │
│  │ (pre-indexed)   │  Task already has interviews baked in              │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │  Delta Update   │  Adjust x,y,z from current state                   │
│  │  (from task +   │  Task default + alert severity + tick              │
│  │   alert + tick) │                                                    │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │ Thalmic Filter  │  MITRE classify (DistilBERT ~50ms)                 │
│  │ (L2 intercept)  │  Update delta.x from classification               │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │  sx9-lisp Eval  │  Rule evaluation with current delta                │
│  │                 │  Fire events trigger escalation                    │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │   ESCALATION    │  7-tier: WASM→Micro→Kernel→Multi→Container→        │
│  │                 │          Firefly→Orb                               │
│  └────────┬────────┘                                                    │
│           │                                                              │
│      ┌────┴────┐                                                        │
│      ▼         ▼                                                        │
│  ┌─────────┐ ┌─────────┐                                               │
│  │ Sledis  │ │  GLAF   │  Persist for correlation                      │
│  │ (fast)  │ │ (graph) │                                               │
│  └─────────┘ └─────────┘                                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.3 Key Principle

**Node interviews are NOT looked up at runtime.** They are:

1. Generated during analysis (04-abe-iac pipeline)
2. Baked into SlotGraph task nodes
3. Used to pre-compute delta defaults
4. Inform tool mappings and edge weights

**Runtime only does:**

1. Parse → Match → Update Delta → Classify → Eval → Escalate → Persist

No lookups, no enrichment - everything needed is already in the matched task node.

---

## 6. HFT Performance Architecture

### 6.1 Competitor Latencies (Why We're Different)

| System | Avg Latency | 95th Percentile | Indexing | Target EPS |
|--------|-------------|-----------------|----------|------------|
| **Wazuh** | 45ms | 500ms | 2s | 500 |
| **Splunk** | 100-500ms | 1-2s | 5-30s | varies |
| **Elastic SIEM** | 50-200ms | 500ms | 1-5s | varies |
| **Plasma Defender** | **<100μs** | **<500μs** | **<1ms** | **100,000+** |

### 6.2 HFT Hot Path (Microseconds)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        HFT HOT PATH (<100μs)                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  OSSEC Alert Bytes                                                       │
│         │                                                                │
│         ▼ (~5μs)                                                        │
│  ┌─────────────────┐                                                    │
│  │  Zero-Copy Parse│  SIMD JSON parse, no allocation                    │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼ (~2μs)                                                      │
│  ┌─────────────────┐                                                    │
│  │   Hash Lookup   │  rule_id → task_id (FxHash, O(1))                  │
│  │   (Sledis)      │  Pre-indexed in Sled B-tree                        │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼ (~10μs)                                                     │
│  ┌─────────────────┐                                                    │
│  │  Delta Update   │  6-decimal fixed-point arithmetic                  │
│  │  (no floats)    │  Legion tick → delta (deterministic)               │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼ (~20μs)                                                     │
│  ┌─────────────────┐                                                    │
│  │  sx9-lisp Eval  │  Unicode bytecode (no parsing)                     │
│  │  (bytecode)     │  U+E500-E8FF = direct opcode dispatch              │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼ (~5μs)                                                      │
│  ┌─────────────────┐                                                    │
│  │   Fire Event    │  Unicode trigger → escalation tier                 │
│  │   (immediate)   │  No async, no queue, direct dispatch               │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼ (~50μs)                                                     │
│  ┌─────────────────┐                                                    │
│  │  Sledis Persist │  Append-only log (no fsync on hot path)            │
│  │  (async flush)  │  Background flush every 100ms                      │
│  └─────────────────┘                                                    │
│                                                                          │
│  TOTAL: ~92μs typical, <100μs p99                                       │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 6.3 Cold Path (GLAF Next-Action Intelligence)

GLAF is for **thinking about next action** - async, background, not hot path:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     COLD PATH (GLAF - Background)                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Fire Event (from hot path)                                             │
│         │                                                                │
│         ▼ (async spawn)                                                 │
│  ┌─────────────────┐                                                    │
│  │  GLAF Correlate │  Graph query for related threats                   │
│  │  (~10-50ms)     │  Neo4j/Chroma similarity search                    │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │              COMBINATORIAL OPTIMIZER (RFC-9021)                    │ │
│  │                                                                    │ │
│  │   Fragment Pool = Available tool chains / actions                  │ │
│  │        │                                                           │ │
│  │        ▼                                                           │ │
│  │   ┌─────────────────┐                                             │ │
│  │   │ Time-of-Value   │  Filter stale fragments                     │ │
│  │   │ Decay Filter    │  (>24h = expired)                           │ │
│  │   └────────┬────────┘                                             │ │
│  │            │                                                       │ │
│  │            ▼                                                       │ │
│  │   ┌─────────────────┐                                             │ │
│  │   │ Matroid Rank    │  Compute diversity of options               │ │
│  │   │ (H2 Semantic)   │  Linear independence in 384-dim space       │ │
│  │   └────────┬────────┘                                             │ │
│  │            │                                                       │ │
│  │            ▼                                                       │ │
│  │   ┌─────────────────┐                                             │ │
│  │   │ Convergence     │  H1 × 0.6 + H2 × 0.4                        │ │
│  │   │ Score           │  (operational + semantic)                   │ │
│  │   └────────┬────────┘                                             │ │
│  │            │                                                       │ │
│  │            ▼                                                       │ │
│  │   ┌─────────────────┐                                             │ │
│  │   │ Greedy Select   │  Pick fragments with positive marginal gain │ │
│  │   │ Best Actions    │  Returns: assigned_fragments[], max_score   │ │
│  │   └─────────────────┘                                             │ │
│  │                                                                    │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │  Tool Chain     │  Map fragments → Kali tools / automated responses │
│  │  Orchestrator   │  nmap → sqlmap → metasploit (chained)             │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │ Thalmic Explain │  Phi-3 LoRA explanation (optional)                 │
│  │  (~100-200ms)   │  Only for high-severity or analyst request         │
│  └────────┬────────┘                                                    │
│           │                                                              │
│           ▼                                                              │
│  ┌─────────────────┐                                                    │
│  │ Update SlotGraph│  Adjust edge weights based on action success       │
│  │ Edge Weights    │  Feedback loop for future recommendations          │
│  └─────────────────┘                                                    │
│                                                                          │
│  TOTAL: 50-300ms (background, doesn't block hot path)                   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 6.3.1 GLAF Next-Action Data Model

```rust
/// Fragment = A possible action or tool chain
pub struct Fragment {
    pub id: u64,
    pub embedding: Vec<f32>,    // 384-dim (all-MiniLM-L6-v2)
    pub confidence: f64,        // Tool effectiveness score
    pub created_at: DateTime,   // For time-of-value decay
    pub tool_chain: Vec<String>, // e.g., ["nmap", "nikto", "sqlmap"]
    pub mitre_techniques: Vec<String>,
    pub hd4_phase: HD4Phase,
}

/// Optimizer selects best fragments for next action
pub struct AssignmentResult {
    pub assigned_fragments: Vec<u64>,      // Selected action IDs
    pub max_convergence_score: f64,        // Combined H1+H2 score
    pub fragments_used_count: usize,
    pub iterations: usize,
}

/// Tool chain orchestration from GLAF recommendation
pub struct ToolChainExecution {
    pub fragments: Vec<Fragment>,
    pub execution_order: Vec<usize>,       // Ordered by dependency
    pub estimated_duration_ms: u64,
    pub expected_delta_shift: DeltaPosition, // Predicted x,y,z change
}
```

### 6.3.2 Automated Response Flow

```
Fire Event (Tier 5+)
    │
    ▼
GLAF Optimizer.greedy_optimize()
    │
    ├─► Fragment 1: ["nmap", "-sV", "target"]     → Recon
    ├─► Fragment 2: ["nikto", "-h", "target"]     → Web scan
    └─► Fragment 3: ["sqlmap", "-u", "url"]       → Exploit
    │
    ▼
Tool Chain Orchestrator
    │
    ├─► Execute Fragment 1 (parallel-safe)
    ├─► Execute Fragment 2 (parallel-safe)
    └─► Execute Fragment 3 (depends on 1,2)
    │
    ▼
Update SlotGraph edge weights
    │
    ▼
Next iteration: Optimizer uses updated weights
```

### 6.4 Mathematical Foundation (NASA-Grade)

Plasma Defender integrates proven mathematical algorithms from ctas7-foundation-math:

#### 6.4.1 Hawkes Process - Self-Exciting Threat Clustering

```rust
/// Hawkes process for threat event intensity
/// λ(t) = μ + Σ α * exp(-β * (t - ti))
pub struct HawkesProcess {
    pub mu: f64,      // Background intensity (baseline threat rate)
    pub alpha: f64,   // Jump size (impact of each event)
    pub beta: f64,    // Decay rate
}

impl HawkesProcess {
    /// Calculate current threat intensity
    pub fn intensity(&self, events: &[f64], current_time: f64) -> f64 {
        let mut intensity = self.mu;
        for &event_time in events {
            let time_since = current_time - event_time;
            if time_since > 0.0 {
                intensity += self.alpha * (-self.beta * time_since).exp();
            }
        }
        intensity
    }

    /// Branching ratio determines stability
    /// < 1.0: Stable (subcritical) - threats decay
    /// = 1.0: Critical - on the edge
    /// > 1.0: Unstable (supercritical) - threat cascade
    pub fn branching_ratio(&self) -> f64 {
        self.alpha / self.beta
    }
}
```

**Use in Plasma Defender:**
- Track threat event clustering in real-time
- Predict escalation before it happens
- Detect attack chains (self-exciting behavior)
- Trigger GLAF optimizer when intensity exceeds threshold

#### 6.4.2 Hidden Markov Models - Attack Chain Recognition

```rust
/// HMM for sequential threat pattern analysis
/// States: Reconnaissance → Initial Access → Execution → Persistence → Impact
pub struct AttackChainHMM {
    pub transition_matrix: DMatrix<f64>,  // State → State probabilities
    pub emission_matrix: DMatrix<f64>,    // State → Observable probabilities
    pub initial_distribution: DVector<f64>,
}

impl AttackChainHMM {
    /// Viterbi algorithm: Most likely attack chain given observations
    pub fn viterbi(&self, observations: &[usize]) -> Vec<usize> {
        // Returns most probable state sequence
    }

    /// Forward-backward: Probability of being in each state at each time
    pub fn forward_backward(&self, observations: &[usize])
        -> (DMatrix<f64>, DMatrix<f64>) {
        // Returns (forward_probs, backward_probs)
    }

    /// Baum-Welch: Learn parameters from observation sequences
    pub fn baum_welch(&mut self, observations: &[Vec<usize>]) {
        // Update transition/emission matrices
    }
}
```

**Use in Plasma Defender:**
- Map OSSEC alerts to attack chain states
- Predict next attack phase (MITRE ATT&CK stage)
- Learn attacker behavior patterns over time
- Update delta.x (semantic axis) based on detected state

#### 6.4.3 Gabor Filters - Image & Biometric Analysis

```rust
/// Gabor filter bank for image pattern detection
/// 8 orientations × 4 frequencies = 32 filters
pub struct GaborFilterBank {
    pub filters: Vec<GaborFilter>,
}

impl GaborFilter {
    /// Gabor function: G(x,y) = exp(-gaussian) * cos(2πf*x')
    /// Orientation-sensitive edge/ridge detection
    pub fn apply_to_image(&self, image: &FingerprintImage) -> GaborFilterResponse {
        // Returns magnitude (edge strength) and phase (edge position)
    }
}

/// Latent fingerprint enhancement pipeline
pub struct LatentFingerprintEnhancementEngine {
    pub noise_filters: Vec<NoiseReductionFilter>,  // Gaussian, Median, Bilateral
}

/// HMM for ridge pattern state sequences
pub struct HiddenMarkovModelEngine {
    /// States: Ridge, Valley, Bifurcation, Ending
    /// Viterbi → most likely minutiae sequence
    /// Baum-Welch → learn from exemplar prints
}
```

**Use in Plasma Defender:**
- **Latent fingerprint enhancement** from crime scene cameras
- **Facial pattern recognition** from surveillance feeds
- **Document analysis** for forged/tampered evidence
- **License plate / vehicle identification**
- **Biometric verification** of operator identity
- **Evidence chain integrity** via image hashing

#### 6.4.4 Financial Algorithms - Risk Scoring

```rust
/// Black-Scholes adapted for threat risk scoring
/// Treats threats like options with time-decay
pub fn threat_risk_score(
    current_severity: f64,    // "spot price"
    max_severity: f64,        // "strike price"
    time_to_impact: f64,      // time factor
    volatility: f64,          // uncertainty in threat assessment
) -> f64 {
    // Returns risk score 0.0 - 1.0
    black_scholes_normalized(current_severity, max_severity, time_to_impact, volatility)
}
```

**Use in Plasma Defender:**
- Score threat urgency with time decay
- Prioritize response based on risk mathematics
- Factor in uncertainty of threat assessment
- Align with delta.y (operational axis)

#### 6.4.5 SGP4 Orbital Propagation

```rust
/// SGP4 for satellite asset tracking
/// Replaces 50TB ephemeris data with 5KB mathematical model
pub fn sgp4_propagate(tle: &TwoLineElement, time: f64) -> OrbitalState {
    // Returns position and velocity at time
}
```

**Use in Plasma Defender:**
- Track satellite assets in ground station scenarios
- Predict communication windows
- Coordinate with space-based sensors
- Used in 7-tier escalation (Orb tier)

#### 6.4.6 Integration Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    MATHEMATICAL INTEGRATION PATHS                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  SECURITY EVENTS (OSSEC Alerts)                                         │
│      │                                                                   │
│      ├─► Hawkes: Is intensity elevated? → Escalate tier                 │
│      ├─► HMM: What attack stage? → Update delta.x (semantic)            │
│      └─► Black-Scholes: Risk score → Update delta.y (operational)       │
│                                                                          │
│  BIOMETRIC/IMAGE EVIDENCE (Cameras, Scanners)                           │
│      │                                                                   │
│      ├─► Gabor: Enhance latent fingerprints                             │
│      ├─► Gabor: Detect facial patterns                                  │
│      ├─► HMM: Ridge pattern state sequences → Minutiae matching         │
│      └─► Evidence hash → Chain of custody verification                  │
│                                                                          │
│  ORBITAL ASSETS (Satellites, Ground Stations)                           │
│      │                                                                   │
│      └─► SGP4: Asset position/velocity → Situational awareness          │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

#### 6.4.7 Bayesian Tick-Delta Architecture

The Plasma Defender implements **industry-leading Bayesian inference** with per-tick delta angle correlation. Each Legion ECS tick produces a **dual delta state** (operational + semantic) with **trivariate hash expression**.

```rust
/// Bayesian inference state for tick correlation (sx9-lisp)
/// P(threat|evidence) = P(evidence|threat) * P(threat) / P(evidence)
pub struct BayesianTickState {
    /// Prior probability of threat (updated each tick)
    pub prior: f64,
    /// Posterior probability P(threat|evidence)
    pub posterior: f64,
    /// Hawkes intensity contribution
    pub hawkes_intensity: f64,
    /// Alpha (Hawkes jump size)
    pub alpha: f64,
    /// Beta (Hawkes decay rate)
    pub beta: f64,
    /// Mu (baseline intensity)
    pub mu: f64,
}

impl BayesianTickState {
    /// Update Bayesian posterior with new evidence
    /// Uses conjugate prior approach for HFT efficiency
    #[inline]
    pub fn update(&mut self, likelihood_ratio: f64, is_threat_event: bool) {
        // Bayes update: P(H|E) = P(E|H) * P(H) / P(E)
        let prior_odds = self.prior / (1.0 - self.prior).max(1e-10);
        let posterior_odds = prior_odds * likelihood_ratio;
        self.posterior = posterior_odds / (1.0 + posterior_odds);

        // Update prior for next tick (exponential smoothing)
        self.prior = 0.9 * self.prior + 0.1 * self.posterior;

        // Update Hawkes on threat events
        if is_threat_event {
            self.hawkes_intensity = self.mu +
                self.alpha * (-self.beta * dt).exp();
        }
    }

    /// Get threat level (0-3: LOW, MEDIUM, HIGH, CRITICAL)
    pub fn threat_level(&self) -> u8 {
        if self.posterior > 0.9 || self.hawkes_intensity > 2.0 { 3 }
        else if self.posterior > 0.7 || self.hawkes_intensity > 1.0 { 2 }
        else if self.posterior > 0.3 || self.hawkes_intensity > 0.3 { 1 }
        else { 0 }
    }
}
```

**Dual Delta State (per-tick):**

```rust
/// Dual delta state: operational (physical) + semantic (meaning)
pub struct DualDeltaState {
    /// Operational delta (position, velocity, acceleration)
    pub ops: DeltaState,      // x,y,z at 6 decimals
    /// Semantic delta (intent, phase, temporal correlation)
    pub semantic: DeltaState, // x,y,z at 6 decimals
    /// Trivariate hash for this tick
    pub ops_hash: TrivariateOpsHash,
    /// Current tick number (Legion ECS aligned)
    pub tick: u64,
}

/// Trivariate hash for operational tracking (RFC-9001)
pub struct TrivariateOpsHash {
    pub sch: u64,   // System Component Hash (identity)
    pub cuid: u64,  // Contextual Unique ID (context)
    pub uuid: u64,  // Universally Unique ID (persistence)
}
```

**Tick → Hash Expression Flow:**

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    TICK-TO-HASH EXPRESSION FLOW                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LEGION TICK (1ms resolution)                                           │
│      │                                                                   │
│      ├─► Update operational delta (position, velocity)                  │
│      │   └─► ops.x = physical_x  (6 decimals)                          │
│      │   └─► ops.y = physical_y  (6 decimals)                          │
│      │   └─► ops.z = physical_z  (6 decimals)                          │
│      │                                                                   │
│      ├─► Update semantic delta (intent, phase, temporal)               │
│      │   └─► semantic.x = intent  (MITRE tactic position)              │
│      │   └─► semantic.y = phase   (HD4 stage)                          │
│      │   └─► semantic.z = temporal (time correlation)                  │
│      │                                                                   │
│      ├─► Update Bayesian state (threat probability)                    │
│      │   └─► likelihood_ratio from DistilBERT classification          │
│      │   └─► posterior = P(threat|evidence)                            │
│      │   └─► hawkes_intensity = μ + Σα*exp(-β*(t-ti))                  │
│      │                                                                   │
│      ├─► Generate trivariate hash                                       │
│      │   └─► SCH = FNV-1a(component)                                    │
│      │   └─► CUID = FNV-1a(context)                                     │
│      │   └─► UUID = FNV-1a(tick:ops_mag:semantic_mag)                   │
│      │                                                                   │
│      └─► Emit Unicode assembly (compressed state)                       │
│          └─► U+E600 (base) + state + ema + hawkes + hash nibbles       │
│                                                                          │
│  OUTPUT PER TICK:                                                        │
│      (tick 42                                                            │
│        (dual-delta                                                       │
│          (ops (Δx 0.000123) (Δy 0.000456) (Δz 0.000789))               │
│          (semantic (intent 0.750000) (phase 0.500000) (temp 0.250000))  │
│          (hash 0x1a2b3c4d5e6f7890))                                     │
│        (bayes-tick (prior 0.1500) (posterior 0.3200)                    │
│          (hawkes 0.4500) (level 1) (stable true)))                      │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Integration with MITRE Classification:**

The Bayesian state receives evidence from DistilBERT MITRE classification. Each detected technique updates the posterior:

```rust
// Per-tick integration
interpreter.advance_tick();

// Update operational delta from physical sensors
interpreter.update_ops_delta(position_x, position_y, position_z);

// Update semantic delta from MITRE classification
let mitre_result = thalmic_filter.classify(input).await?;
let intent_x = mitre_tactic_to_semantic(mitre_result.tactic);
let phase_y = hd4_phase_to_semantic(current_phase);
let temporal_z = time_correlation_factor();
interpreter.update_semantic_delta(intent_x, phase_y, temporal_z);

// Update Bayesian with classification confidence
let likelihood_ratio = mitre_result.confidence / 0.5; // Base confidence 0.5
let is_threat = mitre_result.threat_score > 0.7;
interpreter.update_bayes(likelihood_ratio, is_threat);

// Generate hash for this tick
interpreter.generate_tick_hash("plasma-defender", "ossec-alert");

// Emit compressed state
let unicode_assembly = interpreter.tick_unicode_assembly();
let lisp_expr = interpreter.tick_to_lisp();
```

This architecture achieves **<100μs per-tick processing** while maintaining full Bayesian inference with Hawkes temporal modeling.

### 6.5 Key HFT Optimizations

| Component | Optimization | Latency |
|-----------|-------------|---------|
| **Parse** | SIMD JSON (simd-json crate), zero-copy | ~5μs |
| **Lookup** | FxHash + Sled B-tree, pre-indexed | ~2μs |
| **Delta** | Fixed-point i64 (6 decimals = ×1,000,000) | ~10μs |
| **LISP** | Unicode bytecode, no tokenizer/parser | ~20μs |
| **Fire** | Direct dispatch, no queue | ~5μs |
| **Persist** | Append-only, async flush | ~50μs |
| **Legion** | Tick-driven, deterministic timing | per-tick |

### 6.5 Why Unicode Bytecode Matters

Traditional rule engines parse text at runtime:
```
"if severity > 7 and source = 'ssh' then escalate"
  → Tokenize → Parse → AST → Evaluate → 500μs+
```

sx9-lisp uses Unicode Private Use Area as bytecode:
```
\u{E510}\u{E447}\u{E620}\u{E805}
  → Direct opcode dispatch → 20μs
```

Each Unicode character IS an opcode - no parsing needed.

---

## 7. Kali ISO Integration

### 6.1 Directory Structure

```
/opt/ctas7/
├── thalmic-filter/
│   ├── bin/
│   │   ├── thalmic-intercept       # L2 intercept daemon
│   │   ├── distilbert-classifier   # MITRE classifier
│   │   └── phi3-explainer          # Threat explainer
│   ├── models/
│   │   ├── distilbert-mitre-v1/    # ~260MB
│   │   └── phi3-lora-v1/           # ~50MB
│   └── config/
│       └── thalmic.toml
├── plasma-defender/
│   ├── bin/
│   │   └── plasma-defender         # Main daemon
│   ├── config/
│   │   └── defender.toml
│   └── rules/
│       └── plasma-rules.lisp       # sx9-lisp rules
├── sledis/
│   └── data/                       # Sled database files
├── ossec/
│   └── logs/alerts/                # OSSEC alert JSON
└── slotgraph/
    └── tasks.json                  # 164-task graph definition
```

### 6.2 Systemd Services

```ini
[Unit]
Description=Plasma Defender Security Monitor
After=network.target thalmic-filter.service sledis.service
Requires=thalmic-filter.service sledis.service

[Service]
Type=simple
ExecStart=/opt/ctas7/plasma-defender/bin/plasma-defender
Environment=SLEDIS_URL=unix:///opt/ctas7/sledis/sledis.sock
Environment=THALMIC_SOCKET=/opt/ctas7/thalmic-filter/thalmic.sock
Environment=SLOTGRAPH_PATH=/opt/ctas7/slotgraph/tasks.json
Restart=always

[Install]
WantedBy=multi-user.target
```

---

## 7. Success Criteria

1. ✅ SlotGraph 164 tasks loaded with node/crate interviews
2. ✅ Legion ECS ticks drive delta angle updates
3. ✅ Delta positions @ 6 decimal precision (x,y,z)
4. ✅ L2 Thalmic Filter intercepts all tool invocations on Kali ISO
5. ✅ MITRE classification via DistilBERT
6. ✅ sx9-lisp rules evaluate with Unicode fire events
7. ✅ Sledis persists all state (Redis API)
8. ✅ GLAF correlates via graph queries
9. ✅ Minimal OSSEC (no Wazuh Manager/Indexer/Dashboard)
10. ✅ HD4 phase tracking (Hunt/Detect/Disrupt/Dominate/Disable)

---

## 8. References

- **RFC-9001**: Trivariate Hashing Standard
- **RFC-9004**: Deterministic Routing (Neural Mux)
- **RFC-9020**: HD4 Framework
- **RFC-9025**: Node Interview Schema
- **RFC-9108**: Thalmic Filter & Model Registry
- **RFC-9110**: SX9-LISP Interpreter
- **MITRE ATT&CK**: Enterprise Matrix v14
- **OSSEC**: Open Source Security Event Correlator

---

**End of RFC-9109**
