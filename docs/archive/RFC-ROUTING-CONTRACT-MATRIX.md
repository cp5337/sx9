# RFC Routing Contract Matrix

**Version:** 1.0  
**Date:** December 2025  
**Purpose:** Comprehensive matrix of all RFCs and their routing contract requirements for `sx9-gateway` integration

---

## Executive Summary

This document maps every RFC in the CTAS-7/SX9 ecosystem to its routing contract needs. A **routing contract** is defined as:
- Service discovery requirements
- Port allocation needs
- Neural Mux routing requirements
- CDN routing needs
- Agent dispatch requirements
- IAC trigger needs
- OODA escalation routing
- Resource escalation routing

---

## Current Implementation Status

### DSL (Domain-Specific Language)
- **Status**: ✅ Defined in RFC-9011-B (YAML → DSL Conversion)
- **Location**: Specification exists, implementation pending
- **Routing Need**: DSL playbooks need routing to ATLAS daemon for execution

### ATLAS Daemon
- **Status**: ✅ Implemented (`ctas7-atlas-daemon`)
- **Port**: 18106
- **Zone**: B (1ms tick)
- **Routing Need**: ⚠️ NOT yet integrated with `sx9-atlas-bus` ring buffer
- **Integration Required**: Connect ATLAS daemon to `sx9-atlas-bus` for Legion/apecs IPC

### Neural Context Engine
- **Status**: ✅ Uses Legion ECS
- **Routing Need**: Cognitive operations need routing contracts

### CDN Data Fabric
- **Status**: ✅ Has query router (SQL/SurrealQL/Cypher)
- **Routing Need**: ⚠️ Missing deterministic routing via Neural Mux (RFC-9004)

---

## Core Series (9000-9009)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9000** | Agnostic Core & Ontology | **Service Discovery** | HIGH | ✅ | Core entities need routing |
| **RFC-9001** | Trivariate Hashing | **Neural Mux** (SCH prefix) | CRITICAL | ✅ | Foundation for all routing |
| **RFC-9002** | Unicode Routing | **Neural Mux** (Unicode ranges) | CRITICAL | ✅ | U+E000-E9FF routing |
| **RFC-9003** | Operation Classifier | **Escalation Routing** | HIGH | ✅ | WASM→Kernel→Container routing |
| **RFC-9004** | Deterministic Routing | **ALL** (Primary Spec) | CRITICAL | ⚠️ | Neural Mux, Port Manager, CDN |
| **RFC-9005** | Unified Schema | **Service Discovery** | HIGH | ✅ | Database routing |
| **RFC-9006** | Secure Transport | **Port Allocation** | MEDIUM | ⚠️ | TLS/secure port routing |
| **RFC-9007** | Obfuscation & Honeypot | **CDN Routing** | MEDIUM | ⚠️ | Honeypot endpoint routing |
| **RFC-9008** | Ephemeral Rooms | **Dynamic Port Allocation** | HIGH | ⚠️ | Temporary service routing |
| **RFC-9009** | Quantum Crypto | **Secure Routing** | LOW | ⚠️ | Future quantum-safe routing |

---

## Pipeline Series (9010-9019)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9010** | Enterprise Extraction | **Service Discovery** | MEDIUM | ⚠️ | Extraction service routing |
| **RFC-9011** | Threat Ingestion | **CDN Routing** | HIGH | ⚠️ | Threat content → CDN |
| **RFC-9011-A** | Ingestion Pipeline | **Pipeline Routing** | HIGH | ⚠️ | Multi-stage routing |
| **RFC-9011-B** | YAML→DSL Conversion | **ATLAS Routing** | HIGH | ⚠️ | DSL playbooks → ATLAS daemon |
| **RFC-9012** | GNN Embeddings | **CDN Routing** (Models) | MEDIUM | ⚠️ | Model CDN routing |
| **RFC-9013** | Sensory Substrate | **Service Discovery** | MEDIUM | ⚠️ | Sensor data routing |

---

## Cognitive Series (9020-9029)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9020** | HD4 Framework | **HD4 Phase Routing** (OODA, Delta Angle, Convergence) | CRITICAL | ✅ | Phase-based routing (see HD4 section below) |
| **RFC-9300** | HD4 Canonical Specification | **HD4 Phase Routing** (Canonical) | CRITICAL | ✅ | Canonical HD4 routing (supersedes RFC-9020 conflicts) |
| **RFC-9021** | Graph Convergence | **Neural Mux** (Convergence ops) | HIGH | ✅ | Graph operations routing |
| **RFC-9022** | OODA Vertical Escalation | **Escalation Routing** | CRITICAL | ✅ | Tactical→Operational→Strategic |
| **RFC-9023** | Security Framework | **Service Discovery** | MEDIUM | ⚠️ | Security service routing |
| **RFC-9024** | Neurological Foundation | **ATLAS Routing** | HIGH | ✅ | Dual trivariate routing |
| **RFC-9025** | Unified Interview | **Service Discovery** | MEDIUM | ⚠️ | Interview service routing |
| **RFC-9026** | Hourglass-Bernoulli | **Bernoulli Zone Routing** | HIGH | ⚠️ | Zone-aware routing |

---

## Integration Series (9100-9149)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9100** | Dual-Trivariate PTCC | **Neural Mux** (PTCC ops) | CRITICAL | ✅ | PTCC primitive routing |
| **RFC-9101** | Smart Crate System | **ALL** (Port Manager, Neural Mux, ATLAS, CDN, Service Discovery) | CRITICAL | ⚠️ | Comprehensive routing needs (see Smart Crate section below) |
| **RFC-9102** | Executable Documents | **Service Discovery** | MEDIUM | ⚠️ | Document execution routing |
| **RFC-9103** | IAC Adaptive Infrastructure | **IAC Trigger Routing** | CRITICAL | ⚠️ | Unicode trigger → IAC spawn |
| **RFC-9104** | CTE Cognitive Execution | **ATLAS Routing** | HIGH | ⚠️ | CTE → ATLAS routing |
| **RFC-9105** | SPIRES Extraction | **CDN Routing** | MEDIUM | ⚠️ | SPIRES content → CDN |
| **RFC-9106** | sx9-conda Python Layer | **Port Allocation** | MEDIUM | ⚠️ | Conda service routing |
| **RFC-9107** | Unified Agent Infrastructure | **ALL** (Agent Dispatch, QA Routing, Voice, IDE, Service Discovery) | CRITICAL | ⚠️ | Comprehensive agent routing (see Agent Infrastructure section below) |
| **RFC-9108** | Thalmic Filter Registry | **Service Discovery** | MEDIUM | ⚠️ | Filter service routing |
| **RFC-9109** | CX9 Custom Kali ISO | **CDN Routing** (Tools) | MEDIUM | ⚠️ | Kali tools → CDN |
| **RFC-9110** | SX9 Lisp Interpreter | **ATLAS Routing** | HIGH | ⚠️ | Lisp bytecode → ATLAS |
| **RFC-9111** | Zero-License Data Fabric | **CDN Routing** | MEDIUM | ⚠️ | Data fabric → CDN |
| **RFC-9112** | Deterministic Prompt Engineering | **ALL** (NATS Fabric, L2 Routing, Tool Chain, ANN, Plasma) | CRITICAL | ⚠️ | Comprehensive prompt-to-execution routing (see Prompt Engineering section below) |
| **RFC-9113** | TOML Executable Docs | **Service Discovery** | MEDIUM | ⚠️ | TOML execution routing |

---

## Operational Series (9130-9139, 9876)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9130** | L2 NATS Kali Platform | **L2 Routing** (eBPF/XDP) | CRITICAL | ⚠️ | L2 frame routing |
| **RFC-9131** | Dynamic Resource Escalation | **Resource Escalation Routing** | CRITICAL | ⚠️ | WASM→Kali→GPU routing |
| **RFC-9876** | Layer-Two Unicode Orchestration | **L2 Unicode Routing** | CRITICAL | ⚠️ | U+E000 triggers → L2 |

---

## Application Series (9150-9199)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9150** | GIS UI | **CDN Routing** (Geo) | MEDIUM | ⚠️ | Geo data → CDN |
| **RFC-9151** | Patrolman's Notebook | **Service Discovery** | MEDIUM | ⚠️ | Notebook service routing |

---

## Platform Series (9200-9299)

| RFC | Title | Routing Contract Type | Priority | Status | Notes |
|-----|-------|----------------------|----------|--------|-------|
| **RFC-9200** | SX9 Development Center | **Service Discovery** | HIGH | ⚠️ | Dev center routing |

---

## Smart Crate System (RFC-9101) - Detailed Routing Requirements

### Component Ports & Routing

| Component | Port | Routing Contract Type | Integration Points | Latency Target |
|-----------|------|----------------------|-------------------|----------------|
| **Port Manager** | 18104 | Dynamic Port Allocation | Smart Crate Orchestrator | <1ms |
| **Trivariate Hash Engine** | 18105 | Neural Mux (Hash ops) | All services | 9.3ns |
| **ATLAS Daemon** | 18106 | ATLAS Routing (OODA tick) | Neural Mux, Health Dashboard | 1ms tick |
| **Neural Mux** | 18107 | Neural Mux (Primary) | All services | <250ns |
| **Health Dashboard** | 18108 | Service Discovery | Smart Crate, Lightning QA | 100ms |
| **Lightning QA** | 18109 | CDN Routing | Statistical CDN, ATLAS | 50ms |
| **PLASMA Monitor** | 18110 | Threat Routing | Neural Mux, Orchestrator | 10ms |
| **Smart Crate Orchestrator** | 18111 | Service Discovery | Docker, NATS, Port Manager | 500ms |
| **Statistical CDN Nodes** | 18112-18122 | CDN Routing | Hash-based routing | 5ms |

### Smart Crate Routing Flow

```
User Request / OODA Decision
    │
    ▼
┌─────────────────────┐
│  Neural Mux Router │ ◄─── ATLAS cognitive tick (1ms, port 18106)
│   (<250ns route)    │
│   Port 18107        │
└──────────┬──────────┘
           │
    ┌──────┴──────┐
    │             │
    ▼             ▼
┌─────────┐  ┌──────────────────────┐
│ Cached  │  │ Smart Crate          │
│Response │  │ Orchestrator         │
│(CDN)    │  │ Port 18111           │
└─────────┘  └──────┬────────────────┘
                    │
         ┌──────────┼──────────┐
         │          │          │
         ▼          ▼          ▼
    ┌────────┐ ┌────────┐ ┌────────┐
    │ Port   │ │ NATS   │ │ Docker │
    │Manager │ │ Pub/Sub│ │Spawn   │
    │18104   │ │ 4222   │ │        │
    └───┬────┘ └───┬────┘ └───┬────┘
        │          │          │
        └──────────┴──────────┘
                   │
         ┌─────────┼─────────┐
         ▼         ▼         ▼
    ┌────────┐ ┌────────┐ ┌────────┐
    │ Crate  │ │ Crate  │ │ Crate  │
    │ :1800  │ │ :1801  │ │ :18XX  │
    └───┬────┘ └───┬────┘ └───┬────┘
        │          │          │
        └──────────┴──────────┘
                   │
                   ▼
         ┌──────────────────┐
         │ Health Dashboard │ ◄─── Lightning QA (port 18109)
         │   Port 18108     │
         └──────────────────┘
                   │
                   ▼
         ┌──────────────────┐
         │ PLASMA Analysis  │ ◄─── Threat scoring (port 18110)
         │   (Optional)     │
         └──────────────────┘
```

### Smart Crate Routing Contracts

1. **Port Manager Routing** (RFC-9003)
   - Dynamic port allocation: 1800-1900
   - Port exhaustion detection
   - Port release on crate termination
   - **Required by**: Smart Crate Orchestrator

2. **Neural Mux Routing** (RFC-9002)
   - Route to Smart Crate Orchestrator (port 18111)
   - Route to Trivariate Hash Engine (port 18105)
   - Route to Health Dashboard (port 18108)
   - **Latency**: <250ns

3. **ATLAS Integration** (RFC-9022)
   - 1ms cognitive tick for OODA decisions
   - Autonomous crate spawning decisions
   - Threat-based scaling decisions
   - **Port**: 18106

4. **CDN Routing** (RFC-9004)
   - Statistical CDN nodes (18112-18122)
   - Hash-based routing (SCH prefix)
   - 3x replication
   - **Latency**: <5ms

5. **Service Discovery** (NATS)
   - `smart-crate.spawned` events
   - `smart-crate.killed` events
   - `smart-crate.health` metrics
   - `smart-crate.metrics` telemetry

6. **Health Dashboard Integration**
   - Crate health aggregation
   - Lightning QA report integration
   - Metrics collection
   - **Port**: 18108

7. **PLASMA Threat Routing**
   - High-severity threat detection (>0.8)
   - Automatic threat analysis crate spawning
   - Threat scoring integration
   - **Port**: 18110

### Smart Crate OODA Loop Routing

The Smart Crate system uses OODA loop decisions for autonomous routing:

```rust
// OBSERVE: Gather system state
let observations = Observations {
    cpu_load: tick.state.load,
    active_threats: tick.state.threats,
    available_capacity: tick.state.capacity,
    request_queue_depth: measure_queue_depth(),
};

// ORIENT: Analyze context
let orientation = orient(&observations);

// DECIDE: Determine action
let decision = decide(&orientation);
// Decision types:
// - SpawnThreatCrate (threat > 0.8)
// - ScaleUp(n) (capacity available)
// - ScaleDown(n) (over-provisioned)
// - NoOp (maintain state)

// ACT: Execute decision via routing
act(decision).await;
// Routes to:
// - Smart Crate Orchestrator (spawn/kill)
// - Port Manager (allocate/release)
// - Health Dashboard (register/unregister)
// - NATS (publish events)
```

### Smart Crate Routing Gaps

**Current Status**: ⚠️ Partially Implemented

**Missing Integrations**:
1. ❌ ATLAS daemon NOT connected to `sx9-atlas-bus` ring buffer
2. ❌ Neural Mux routing incomplete (<250ns not guaranteed)
3. ❌ Health Dashboard not integrated with Smart Crate Orchestrator
4. ❌ Lightning QA not connected to Statistical CDN
5. ❌ PLASMA Monitor not integrated with Neural Mux
6. ❌ Port Manager not integrated with gateway

**Required Actions**:
1. Connect ATLAS daemon to `sx9-atlas-bus` for Legion/apecs IPC
2. Implement Neural Mux router in `sx9-gateway` with <250ns guarantee
3. Integrate Health Dashboard with Smart Crate Orchestrator
4. Wire Lightning QA to Statistical CDN for QA report routing
5. Connect PLASMA Monitor to Neural Mux for threat-based routing
6. Integrate Port Manager with gateway for dynamic port allocation

---

## Unified Agent Infrastructure (RFC-9107) - Detailed Routing Requirements

### Agent Mesh Architecture

| Agent | Port | LLM | Primary Role (Ops) | Secondary Role (Dev) | Routing Contract |
|-------|------|-----|-------------------|---------------------|------------------|
| Grok | 50051 | Grok-1.5 | Space Engineering | Starlink Integration | gRPC routing |
| Natasha | 50052 | Claude-3.5-Sonnet | Red Team Operations | Voice Command Interface | gRPC + Voice routing |
| Cove | 50053 | GPT-4-Turbo | DevOps Orchestration | QA5/XSD Validation | gRPC + QA routing |
| Altair | 50054 | Claude-GPT-Hybrid | Space Domain Awareness | SDA Analytics | gRPC routing |
| Claude | 50055 | Claude-3.5-Sonnet | Meta-Agent Orchestration | IDE Integration | gRPC + MCP routing |
| Zoe | 50056 | Gemini-1.5-Pro | Orbital Operations | CesiumJS Visualization | gRPC + Orbital routing |
| GPT | 50057 | GPT-4-Turbo | Tactical Operations | Code Generation | gRPC routing |
| Elena | 50058 | GPT-4-Turbo | LATAM Intelligence | Cartel Threat Analysis | gRPC + Voice routing |

### Agent Routing Contracts

1. **Agent Dispatch Routing** (RFC-9004 Neural Mux)
   - Trivariate hash prefix routing (0xC7A5_0000 = Ops, 0xC7A5_0100 = Dev)
   - Linear issue type routing (`type:bug`/`type:security` → Ops, `type:feature` → Dev)
   - Voice command prefix routing ("execute..." → Ops, "implement..." → Dev)
   - QA severity routing (Critical/High → Ops, Medium/Low → Dev)

2. **QA-to-Agent Routing** (Lightning QA Port 18109)
   - Critical findings → Natasha (Red Team, port 50052)
   - High findings → Altair (SDA, port 50054)
   - Medium findings → Cove (DevOps, port 50053)
   - Low findings → GPT (Tactical, port 50057)

3. **Voice System Routing** (ElevenLabs Port 18260)
   - Voice ID routing: Natasha (EXAVITQu4vr4xnSDxMaL), Elena (oWAxZDx7w5VEj9dCyTzz), Zoe (21m00Tcm4TlvDq8ikWAM)
   - Cloudflare Edge CDN routing: `voice.sx9.io/synthesize`
   - Voice-to-agent mapping via persona registry

4. **IDE Unification Routing** (MCP)
   - Cursor: `.mcp.json` → Agent mesh (port 50055)
   - VSCode (Antigravity): `.vscode/settings.json` → Agent mesh
   - Custom GPT: OpenAPI Schema → Natasha/Zoe endpoints

5. **Service Discovery**
   - Agent capability registry
   - Agent health monitoring
   - Agent load balancing

### Agent Routing Gaps

**Current Status**: ⚠️ Partially Implemented

**Missing Integrations**:
1. ❌ QA-to-agent routing not wired (Lightning QA port 18109 → agents)
2. ❌ Voice system routing incomplete (ElevenLabs → agents)
3. ❌ IDE MCP routing not unified
4. ❌ Agent capability-based routing not implemented
5. ❌ Dual-role (Ops/Dev) routing not functional

---

## Deterministic Prompt Engineering (RFC-9112 v3.0) - Detailed Routing Requirements

### NATS Message Fabric Routing

**Subject Hierarchy**:
```
sx9.tick.*              # Legion ECS tick synchronization
sx9.workflow.*          # Workflow lifecycle
sx9.ann.*               # ANN synthesis events
sx9.l2.*                # Layer-Two orchestration (RFC-9876)
sx9.threat.*            # Threat intelligence
sx9.hash.*              # Hash events
sx9.error.*             # Error events
```

### Routing Contracts

1. **NATS Fabric Routing** (Port 4222)
   - Workflow spawned → `sx9.workflow.spawned`
   - Tool execution → `sx9.l2.tool.{name}.{started|completed}`
   - ANN synthesis → `sx9.ann.{corpus.scanned|architecture.selected|training.*|model.ready}`
   - L2 trigger → `sx9.l2.trigger`
   - L2 response → `sx9.l2.response`

2. **L2 Routing** (eBPF/XDP, RFC-9876)
   - Unicode trigger → L2 frame (U+E000-U+F8FF)
   - XDP/eBPF packet-level routing
   - <50μs latency (Zone A)
   - L2 response byte: U+F8FF

3. **Tool Chain Routing** (Hermetic Execution)
   - Unicode trigger → Tool wrapper (U+E001 = ReconNG, U+E002 = masscan, U+E003 = nmap, etc.)
   - NATS KV state store routing (no filesystem)
   - Tool-to-tool state flow via NATS KV
   - Hermetic execution (no shell, no files, no logs)

4. **ANN Synthesis Routing**
   - Corpus scan → Architecture selection
   - Training events → Model ready
   - Model deployment → Plasma runtime

5. **Plasma + Legion Runtime Routing**
   - 250ns tick resolution
   - GLAF matroid slot allocation
   - Agent slot state routing (Empty → Loading → Ready → Executing → Converged)

### Prompt Engineering Routing Gaps

**Current Status**: ⚠️ Partially Implemented

**Missing Integrations**:
1. ❌ NATS subject hierarchy not fully implemented
2. ❌ L2 eBPF/XDP routing not integrated
3. ❌ Hermetic tool wrappers not complete
4. ❌ NATS KV state store not configured
5. ❌ Plasma + Legion runtime not connected to gateway

---

## HD4 Canonical Specification (RFC-9300) - Detailed Routing Requirements

### HD4 Phase Routing (Canonical Order)

**Immutable Phase Sequence**:
```
HUNT → DETECT → DISRUPT → DISABLE → DOMINATE
```

**Unicode Allocation**:
- U+E700: HD4_HUNT
- U+E701: HD4_DETECT
- U+E702: HD4_DISRUPT
- U+E703: HD4_DISABLE
- U+E704: HD4_DOMINATE

### Routing Contracts

1. **HD4 Phase Transition Routing**
   - Phase-based service routing (Hunt → Detect → Disrupt → Disable → Dominate)
   - Convergence-based transitions (H1/H2 ≥ 0.75 → next phase)
   - Delta angle Y-axis routing (0.0 = Hunt, 0.25 = Detect, 0.5 = Disrupt, 0.75 = Disable, 1.0 = Dominate)

2. **OODA Loop Integration Routing**
   - OODA cycles within HD4 phases
   - Observe → Hunt phase routing
   - Orient → Detect phase routing
   - Decide → Disrupt phase routing
   - Act → Disable/Dominate phase routing

3. **Delta Angle Routing** (Y-Axis Operational)
   - Y = 0.000000: Hunt (baseline seeking)
   - Y = 0.250000: Detect (identification)
   - Y = 0.500000: Disrupt (interference)
   - Y = 0.750000: Disable (neutralization)
   - Y = 1.000000: Dominate (control)
   - Intermediate values: Phase transitions

4. **Convergence-Based Routing** (H1/H2 Thresholds)
   - 0.00-0.74: Hunt phase (continue reconnaissance)
   - 0.75-0.79: Detect phase (confirm indicators)
   - 0.80-0.84: Disrupt phase (begin interference)
   - 0.85-0.89: Disable phase (neutralize)
   - 0.90-1.00: Dominate phase (assert control)

5. **Framework Subsumption Routing**
   - MITRE ATT&CK tactics → HD4 phases
   - MITRE D3FEND tactics → HD4 phases
   - Kill Chain phases → HD4 phases
   - STRIDE categories → HD4 phases

### HD4 Routing Gaps

**Current Status**: ⚠️ Partially Implemented

**Missing Integrations**:
1. ❌ HD4 phase routing not fully integrated with Neural Mux
2. ❌ Delta angle Y-axis routing not implemented
3. ❌ Convergence-based phase transitions not automated
4. ❌ OODA loop → HD4 phase routing incomplete
5. ❌ Framework subsumption routing not functional

**Note**: RFC-9300 supersedes conflicting HD4 definitions in RFC-9020, RFC-9024, RFC-9025, RFC-9109, RFC-9110. All implementations MUST use the canonical order: Hunt → Detect → Disrupt → Disable → Dominate.

---

## Routing Contract Types

### 1. Neural Mux Routing (RFC-9004)
**Required By**: RFC-9001, RFC-9002, RFC-9004, RFC-9021, RFC-9100  
**Implementation**: `NeuralMuxRouter` in `sx9-gateway`  
**Latency Target**: <250ns (Zone A)  
**Key Features**:
- Trivariate hash prefix lookup (SCH top 16 bits)
- Unicode range routing (U+E000-E9FF)
- Domain mask fallback
- Bernoulli zone classification

### 2. Port Manager Routing (RFC-9004)
**Required By**: RFC-9004, RFC-9006, RFC-9008, RFC-9101, RFC-9106  
**Implementation**: `PortManager` in `sx9-gateway`  
**Key Features**:
- Service registry (name → port)
- Port allocation (1800-1900 dynamic)
- CDN routing table (8 CDNs: 19000-19013)
- Port exhaustion detection

### 3. CDN Routing (RFC-9004)
**Required By**: RFC-9004, RFC-9007, RFC-9011, RFC-9012, RFC-9105, RFC-9109, RFC-9111, RFC-9150  
**Implementation**: `CdnManager` in `sx9-gateway`  
**CDN Types**:
- Static (19000)
- Crates (19001)
- Geo (19002)
- Models (19003)
- Conda (19010)
- Tools (19011)
- WASM (19012)
- Plasma (19013)

### 4. ATLAS Routing (sx9-atlas-bus)
**Required By**: RFC-9011-B, RFC-9020, RFC-9022, RFC-9024, RFC-9104, RFC-9110  
**Implementation**: `AtlasBridge` in `sx9-gateway`  
**Key Features**:
- Command dispatch to ATLAS daemon (Legion ECS)
- PlasmaState exposure
- Result streaming
- Ring buffer IPC

### 5. Agent Dispatch Routing (RFC-9107)
**Required By**: RFC-9107  
**Implementation**: Agent router in `sx9-gateway`  
**Agent Ports**: 50051-50058 (gRPC)  
**Key Features**:
- Agent capability routing
- Ops vs Dev mode routing
- QA-to-agent routing

### 6. IAC Trigger Routing (RFC-9103)
**Required By**: RFC-9004, RFC-9103  
**Implementation**: IAC trigger handler in `sx9-gateway`  
**Unicode Triggers**: U+EA00-EAFF  
**Key Features**:
- Unicode trigger → IAC manifold spawn
- CDN node spawning (U+EA10)
- GPU cluster spawning (U+EA02)
- Port expansion (U+EA21)

### 7. OODA Escalation Routing (RFC-9022)
**Required By**: RFC-9020, RFC-9022  
**Implementation**: Escalation router in `sx9-gateway`  
**Key Features**:
- Tactical → Operational escalation
- Operational → Strategic escalation
- Strategic → National escalation
- Vertical level routing

### 8. Resource Escalation Routing (RFC-9131)
**Required By**: RFC-9131  
**Implementation**: Resource escalation router in `sx9-gateway`  
**Escalation Levels**:
- Level 0: In-Process
- Level 1: WASM Microkernel
- Level 2: Shell Script
- Level 3: Rust Binary
- Level 4: Single Kali
- Level 5: Parallel Kalis
- Level 6: GPU Cluster

### 9. L2 Routing (RFC-9130, RFC-9876)
**Required By**: RFC-9130, RFC-9876  
**Implementation**: L2 router (eBPF/XDP)  
**Key Features**:
- Unicode trigger → L2 frame
- XDP/eBPF routing
- <50μs latency (Zone A)

### 10. Service Discovery
**Required By**: RFC-9000, RFC-9005, RFC-9010, RFC-9013, RFC-9023, RFC-9025, RFC-9102, RFC-9108, RFC-9112, RFC-9113, RFC-9151, RFC-9200  
**Implementation**: Service registry in `sx9-gateway`  
**Key Features**:
- Service name → endpoint mapping
- Health check aggregation
- Service versioning

### 11. Agent Dispatch Routing (RFC-9107)
**Required By**: RFC-9107  
**Implementation**: Agent router in `sx9-gateway`  
**Agent Ports**: 50051-50058 (gRPC)  
**Key Features**:
- Agent capability routing
- Ops vs Dev mode routing (trivariate hash prefix: 0xC7A5_0000 = Ops, 0xC7A5_0100 = Dev)
- QA-to-agent routing (Critical/High → Ops agents, Medium/Low → Dev agents)
- Voice-to-agent mapping (ElevenLabs voice IDs)
- IDE MCP routing (Cursor, VSCode, Custom GPT)

### 12. NATS Fabric Routing (RFC-9112)
**Required By**: RFC-9112, RFC-9130, RFC-9876  
**Implementation**: NATS client in `sx9-gateway`  
**Port**: 4222 (NATS), 8222 (NATS HTTP)  
**Key Features**:
- Subject hierarchy routing (`sx9.tick.*`, `sx9.workflow.*`, `sx9.l2.*`, etc.)
- JetStream persistence for audit trails
- NATS KV state store (no filesystem)
- Workflow lifecycle routing
- Tool chain execution routing

### 13. L2 Routing (RFC-9112, RFC-9130, RFC-9876)
**Required By**: RFC-9112, RFC-9130, RFC-9876  
**Implementation**: eBPF/XDP stack  
**Key Features**:
- Unicode trigger → L2 frame (U+E000-U+F8FF)
- XDP/eBPF packet-level routing
- <50μs latency (Zone A)
- L2 response byte: U+F8FF
- Hermetic tool execution (no shell, no files, no logs)

### 14. HD4 Phase Routing (RFC-9300)
**Required By**: RFC-9020, RFC-9300  
**Implementation**: HD4 phase router in `sx9-gateway`  
**Canonical Order**: Hunt → Detect → Disrupt → Disable → Dominate  
**Unicode**: U+E700-U+E704  
**Key Features**:
- Phase-based service routing
- Convergence-based transitions (H1/H2 ≥ 0.75)
- Delta angle Y-axis routing (0.0 = Hunt, 0.25 = Detect, 0.5 = Disrupt, 0.75 = Disable, 1.0 = Dominate)
- OODA loop integration (Observe→Hunt, Orient→Detect, Decide→Disrupt, Act→Disable/Dominate)
- Framework subsumption (MITRE ATT&CK, D3FEND, Kill Chain → HD4)

---

## Priority Matrix

### CRITICAL (Must Implement First)
1. **RFC-9004** - Deterministic Routing (ALL contract types)
2. **RFC-9001** - Trivariate Hashing (Neural Mux foundation)
3. **RFC-9002** - Unicode Routing (Neural Mux foundation)
4. **RFC-9100** - PTCC Integration (Neural Mux routing)
5. **RFC-9020** - HD4 Framework (OODA escalation)
6. **RFC-9300** - HD4 Canonical Specification (supersedes RFC-9020 conflicts)
7. **RFC-9022** - OODA Vertical Escalation
8. **RFC-9103** - IAC Adaptive Infrastructure (IAC triggers)
9. **RFC-9130** - L2 NATS Kali Platform
10. **RFC-9131** - Dynamic Resource Escalation
11. **RFC-9107** - Unified Agent Infrastructure
12. **RFC-9112** - Deterministic Prompt Engineering (NATS fabric, L2 routing)

### HIGH (Implement Second)
1. **RFC-9003** - Operation Classifier (Escalation routing)
2. **RFC-9005** - Unified Schema (Service discovery)
3. **RFC-9008** - Ephemeral Rooms (Dynamic ports)
4. **RFC-9011-B** - YAML→DSL (ATLAS routing)
5. **RFC-9021** - Graph Convergence (Neural Mux)
6. **RFC-9024** - Neurological Foundation (ATLAS routing)
7. **RFC-9101** - Smart Crate System (Dynamic ports)
8. **RFC-9112** - Deterministic Prompt Engineering
9. **RFC-9200** - SX9 Development Center

### MEDIUM (Implement Third)
- All remaining RFCs with routing contract needs

---

## Implementation Checklist

### Phase 1: Foundation (CRITICAL)
- [ ] Neural Mux Router (RFC-9004)
- [ ] Port Manager (RFC-9004)
- [ ] CDN Manager (RFC-9004)
- [ ] ATLAS Bridge (sx9-atlas-bus integration)
- [ ] Trivariate Hash utilities (RFC-9001)

### Phase 2: Core Routing (CRITICAL)
- [ ] OODA Escalation Router (RFC-9022)
- [ ] IAC Trigger Handler (RFC-9103)
- [ ] Agent Dispatch Router (RFC-9107)
- [ ] Resource Escalation Router (RFC-9131)

### Phase 3: L2 Routing (CRITICAL)
- [ ] L2 Unicode Router (RFC-9876)
- [ ] eBPF/XDP integration (RFC-9130)

### Phase 4: Service Discovery (HIGH)
- [ ] Service Registry
- [ ] Health Check Aggregation
- [ ] Service Versioning

### Phase 5: Specialized Routing (MEDIUM)
- [ ] DSL Playbook Router (RFC-9011-B)
- [ ] Graph Operations Router (RFC-9021)
- [ ] PTCC Primitive Router (RFC-9100)

---

## Notes

1. **ATLAS Daemon Integration**: Currently NOT connected to `sx9-atlas-bus`. This is a critical gap that must be addressed.
2. **DSL Implementation**: DSL spec exists (RFC-9011-B) but implementation is pending. DSL playbooks need routing to ATLAS daemon.
3. **CDN Data Fabric**: Has query router but missing deterministic routing via Neural Mux.
4. **Path Dependencies**: `ctas7-foundation-core` may need to be extracted or made available as path dependency for `sx9-gateway`.

---

**End of RFC Routing Contract Matrix**

