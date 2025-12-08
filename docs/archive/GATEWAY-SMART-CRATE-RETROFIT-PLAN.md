# Gateway Smart Crate Retrofit Plan
## Smart Crate Features, Agent Consolidation, Intel Consolidation, and ECS Fixes

**Date:** December 2025  
**Status:** Implementation Plan  
**Goal:** Retrofit all gateway crates with smart crate features, consolidate agents/intel, fix ECS

---

## Executive Summary

**Critical Tasks:**
1. **Smart Crate Features** - Identify and retrofit all gateway crates with smart-crate.toml
2. **Agent Consolidation** - Unified agent infrastructure (RFC-9107)
3. **Intel Consolidation** - Unified intel gateway
4. **ECS Fixes** - PLASMA-ECS integration (Legion + apecs + ATLAS)

---

## 1. Smart Crate Features for Gateway Build

### 1.1 Smart Crate Feature Matrix

Based on `ctas7-government-data-manifold/smart-crate.toml` and RFC-9101, here are the **required smart crate features**:

#### Core Smart Crate Features (All Crates)

```toml
[smart-crate]
name = "crate-name"
version = "7.0.0"
edition = "2021"
smart_crate_version = "7.0"
foundation = "ctas7-foundation-core"
classification = "gateway|foundation|integration|application"
tesla_grade = true

[smart_meta]
description = "Crate description"
domains = ["domain1", "domain2"]
capabilities = ["capability1", "capability2"]
build_system = "cargo"
backend_language = "rust"
frontend_language = "typescript"  # if applicable

[integration]
gold_disk_compatible = true
neural_mux_enabled = true
hash_engine_integrated = true
unicode_assembly_support = true
multi_tenant = true
real_time_capable = true
layer2_fabric_node = true  # if applicable
world_registry_participant = true  # if applicable

[ports]
# Port allocations via ctas7-real-port-manager
# All ports must be allocated through port manager (18104)

[security]
slsa_level = 3
hermetic_builds = true
provenance_required = true
source_verification = true
supply_chain_security = true

[performance]
# Performance targets specific to crate

[endpoints]
health = "/health"
metrics = "/metrics"
status = "/smart-crate/status"
```

#### Gateway-Specific Features

**For `sx9-gateway-primary`:**
```toml
[integration]
# Gateway-specific integrations
ops_main_platform = true
usim_integration = true
eei_integration = true
foundation_manifold = true
foundation_math = true
government_data_manifold = true
l2_execution = true
kali_iso_integration = true

[gateway]
websocket_enabled = true
rest_api_enabled = true
grpc_enabled = true
bernoulli_zone_compliance = true
routing_latency_ns = 250  # RFC-9004 requirement
```

**For Foundation Crates:**
```toml
[foundation]
deterministic_routing = true
trivariate_hashing = true
ptcc_primitives = true
unicode_routing = true
```

**For Integration Crates:**
```toml
[integration]
nats_jetstream = true
supabase_integration = true
surrealdb_integration = true
sled_integration = true
```

### 1.2 Gateway Crates Requiring Smart Crate Retrofit

#### Phase 1: Core Gateway (Week 1)

| Crate | Current Status | Smart Crate Features Needed |
|-------|---------------|---------------------------|
| `sx9-gateway-primary` | Basic WebSocket | Full smart-crate.toml, gateway features |
| `sx9-atlas-bus` | ✅ Has smart-crate.toml | Verify compliance, update to v7.0 |
| `ctas7-neural-mux` | Partial | Complete smart-crate.toml, <250ns routing |
| `ctas7-real-port-manager` | Partial | Complete smart-crate.toml, crystal gating |
| `ctas7-foundation-core` | ✅ Has smart-crate.toml | Verify RFC-9001 compliance |
| `ctas7-foundation-manifold` | Partial | Complete smart-crate.toml, routing features |

#### Phase 2: Integration Crates (Week 2)

| Crate | Current Status | Smart Crate Features Needed |
|-------|---------------|---------------------------|
| `ctas7-cdn-data-fabric` | Partial | Complete smart-crate.toml, multi-DB features |
| `ctas7-nats-fabric` | Partial | Complete smart-crate.toml, JetStream features |
| `ctas7-wasm-primitives` | Partial | Complete smart-crate.toml, WASM features |
| `ctas7-cognitive-execution-tool` | Partial | Complete smart-crate.toml, tool execution |
| `ctas7-usim-system` | Partial | Complete smart-crate.toml, ephemeral intelligence |
| `ctas7-eei-system` | Partial | Complete smart-crate.toml, EEI features |
| `ctas7-foundation-math` | Partial | Complete smart-crate.toml, math features |
| `ctas7-government-data-manifold` | ✅ Has smart-crate.toml | Reference implementation |

#### Phase 3: Supporting Crates (Week 3)

| Crate | Current Status | Smart Crate Features Needed |
|-------|---------------|---------------------------|
| `tools/kali-plasma/agent` | Partial | Complete smart-crate.toml, L2 execution |
| `ctas7-orchestrator` | Partial | Complete smart-crate.toml, orchestration |

### 1.3 Smart Crate Retrofit Checklist

**For Each Crate:**
- [ ] Create/update `smart-crate.toml`
- [ ] Set `foundation = "ctas7-foundation-core"`
- [ ] Configure `[integration]` section
- [ ] Define `[ports]` (allocate via port manager)
- [ ] Set `[security]` requirements (SLSA Level 3)
- [ ] Define `[performance]` targets
- [ ] Configure `[endpoints]` (health, metrics, status)
- [ ] Remove Blake3 references (replace with Murmur3-128)
- [ ] Add trivariate hashing (RFC-9001)
- [ ] Integrate Neural Mux routing (RFC-9004)
- [ ] Add Health Dashboard integration
- [ ] Connect Lightning QA (port 18109)
- [ ] Integrate ATLAS Daemon (1ms tick)
- [ ] Connect PLASMA Monitoring

---

## 2. Agent Consolidation

### 2.1 Current Agent Systems

**Multiple Agent Implementations:**
- `ctas7-agentic-core` - Base agent types, traits
- `ctas7-persona-core` - Persona definitions, lifecycle
- `ctas7-agent-registry` - gRPC mesh registry
- `ctas7-agent-dispatch` - Agent dispatch system
- Various domain-specific agents (cyber, orbital, maritime, etc.)

**Problem:**
- Fragmented agent infrastructure
- No unified agent registry
- Different agent models across domains
- No consistent agent lifecycle management

### 2.2 Unified Agent Infrastructure (RFC-9107)

**Solution:** Create `sx9-unified-agent-registry` (or enhance `ctas7-agent-registry`)

**Architecture:**
```
┌─────────────────────────────────────────────────────────┐
│         sx9-unified-agent-registry                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Agent Types  │  │  Personas    │  │  Registry    │ │
│  │ (RFC-9107)   │  │  (Lifecycle) │  │  (gRPC Mesh) │ │
│  └──────┬───────┘  └──────┬──────┘  └──────┬───────┘ │
│         │                  │                  │         │
│         └──────────────────┴──────────────────┘         │
│                        │                              │
│         ┌──────────────▼──────────────┐              │
│         │   PLASMA-ECS Integration    │              │
│         │   (Legion + apecs + ATLAS)  │              │
│         └─────────────────────────────┘              │
│                        │                              │
│         ┌──────────────▼──────────────┐              │
│         │   Gateway Integration       │              │
│         │   (NATS Escalation)         │              │
│         └─────────────────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

**Consolidation Plan:**

**Phase 1: Create Unified Registry (Week 1)**
1. Create `sx9-unified-agent-registry` crate
2. Integrate all agent types from `ctas7-agentic-core`
3. Integrate personas from `ctas7-persona-core`
4. Integrate registry from `ctas7-agent-registry`
5. Integrate dispatch from `ctas7-agent-dispatch`

**Phase 2: PLASMA-ECS Integration (Week 2)**
1. Integrate with PLASMA-ECS (Legion + apecs + ATLAS)
2. Agent entities in Legion World (Layer 2)
3. Agent async operations in apecs (Layer 1)
4. Agent cognitive orchestration in ATLAS (Layer 3)

**Phase 3: Gateway Integration (Week 3)**
1. NATS escalation subjects (`sx9.escalate.agent`)
2. Gateway routing for agent requests
3. Ops-Main-Platform agent UI
4. Agent lifecycle management

**Phase 4: Domain Agent Consolidation (Week 4)**
1. Consolidate cyber agents
2. Consolidate orbital agents
3. Consolidate maritime agents
4. Consolidate manufacturing agents
5. Unified agent model across all domains

### 2.3 Agent Consolidation Checklist

- [ ] Create `sx9-unified-agent-registry` crate
- [ ] Integrate `ctas7-agentic-core` agent types
- [ ] Integrate `ctas7-persona-core` personas
- [ ] Integrate `ctas7-agent-registry` registry
- [ ] Integrate `ctas7-agent-dispatch` dispatch
- [ ] PLASMA-ECS integration (Legion + apecs + ATLAS)
- [ ] Gateway NATS escalation integration
- [ ] Ops-Main-Platform agent UI
- [ ] Domain agent consolidation
- [ ] Agent lifecycle management
- [ ] Agent health monitoring
- [ ] Agent performance metrics

---

## 3. Intel Consolidation

### 3.1 Current Intel Systems

**Multiple Intel Implementations:**
- `ctas7-ingestion-pipeline` - Intel ingestion
- `ctas7-yaml-dsl-converter` - DSL conversion
- `ctas7-osint-machine` - OSINT collection
- `ctas7-cdn-threat-intel` - Threat intelligence
- `ctas7-intel-system` - Intel system
- Various Python intel tools (conda environment)

**Problem:**
- Fragmented intel infrastructure
- Python tools not integrated with Rust gateway
- No unified intel API
- Different intel formats across systems

### 3.2 Unified Intel Gateway

**Solution:** Create `sx9-intel-gateway` crate

**Architecture:**
```
┌─────────────────────────────────────────────────────────┐
│         sx9-intel-gateway                               │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │  Ingestion   │  │   Processing │  │  Distribution│ │
│  │  Pipeline    │  │   (USIM/EEI) │  │  (NATS)     │ │
│  └──────┬───────┘  └──────┬──────┘  └──────┬───────┘ │
│         │                  │                  │         │
│         └──────────────────┴──────────────────┘         │
│                        │                              │
│         ┌──────────────▼──────────────┐              │
│         │   Python Tools Bridge       │              │
│         │   (Conda Environment)      │              │
│         └─────────────────────────────┘              │
│                        │                              │
│         ┌──────────────▼──────────────┐              │
│         │   Gateway Integration       │              │
│         │   (REST/WebSocket/gRPC)     │              │
│         └─────────────────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

**Consolidation Plan:**

**Phase 1: Create Intel Gateway (Week 1)**
1. Create `sx9-intel-gateway` crate
2. Integrate `ctas7-ingestion-pipeline`
3. Integrate `ctas7-yaml-dsl-converter`
4. Integrate `ctas7-osint-machine`
5. Integrate `ctas7-cdn-threat-intel`
6. Integrate `ctas7-intel-system`

**Phase 2: Python Tools Bridge (Week 2)**
1. Create Conda environment bridge
2. API-only access to Python intel tools
3. REST/gRPC interface for Python tools
4. No direct Python execution in gateway

**Phase 3: USIM/EEI Integration (Week 3)**
1. USIM integration for ephemeral intelligence
2. EEI integration for time-of-value classification
3. Government Data Manifold integration
4. PTCC-TETH database intelligence

**Phase 4: Gateway Integration (Week 4)**
1. Gateway routing for intel requests
2. NATS streaming for intel distribution
3. Ops-Main-Platform intel UI
4. Real-time intel feeds

### 3.3 Intel Consolidation Checklist

- [ ] Create `sx9-intel-gateway` crate
- [ ] Integrate `ctas7-ingestion-pipeline`
- [ ] Integrate `ctas7-yaml-dsl-converter`
- [ ] Integrate `ctas7-osint-machine`
- [ ] Integrate `ctas7-cdn-threat-intel`
- [ ] Integrate `ctas7-intel-system`
- [ ] Create Python tools bridge (Conda)
- [ ] USIM integration
- [ ] EEI integration
- [ ] Government Data Manifold integration
- [ ] Gateway routing integration
- [ ] NATS streaming integration
- [ ] Ops-Main-Platform intel UI

---

## 4. ECS Fixes

### 4.1 Current ECS Issues

**Problem:**
- `ctas7-network-world` uses `hecs` + `bevy_ecs` (async I/O)
- `ctas7-world-ecs` uses `Legion` (deterministic batch)
- Different ECS implementations
- No unified state management
- Worlds can't easily share entities

**Solution:** PLASMA-ECS (Legion + apecs + ATLAS)

### 4.2 PLASMA-ECS Architecture

**Three-Layer Architecture:**
```
┌─────────────────────────────────────────────────────────┐
│                    PLASMA-ECS                           │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  LAYER 3: ATLAS (Cognitive)                             │
│  ═══════════════════════════                            │
│  • ATLAS Daemon (1ms OODA loop)                        │
│  • sx9-atlas-bus (ring buffer, PlasmaState)            │
│  • Crystal resonance, SDT gate control                 │
│  • Priority routing (critical/urgent/normal)            │
│  • NATS bridge for distributed ops                     │
│                                                          │
│  LAYER 2: Legion (Deterministic Batch)                   │
│  ═══════════════════════════════════════                │
│  • High-performance batch processing                   │
│  • Deterministic tick-based world state                 │
│  • Hot-path operations (<1ms latency)                   │
│  • Entity-component queries                            │
│  • Schedule execution                                  │
│  • **Slot Graph hash/unicode routing** (preserved)     │
│                                                          │
│  LAYER 1: apecs (Async I/O)                             │
│  ═══════════════════════════                            │
│  • Async-friendly operations                           │
│  • WASM-compatible                                     │
│  • I/O-bound tasks (network, database)                 │
│  • UI integration                                      │
│  • **Network World migration target**                   │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 4.3 ECS Fix Plan

**Phase 1: Create PLASMA-ECS Crate (Week 1)**
1. Create `ctas7-plasma-ecs` crate
2. Integrate Legion (Layer 2) - preserve Slot Graph integration
3. Integrate apecs (Layer 1)
4. Integrate ATLAS (Layer 3)
5. Unified PlasmaState
6. **Verify Slot Graph hash/unicode routing works**

**Phase 2: Network World Migration (Week 2)**
1. Migrate `ctas7-network-world` from `hecs`/`bevy_ecs` to `apecs` (Layer 1)
2. Keep async-friendly architecture
3. Bridge to PLASMA-ECS
4. Preserve I/O-bound operations

**Phase 3: Unified State Management (Week 3)**
1. All worlds use PLASMA-ECS
2. Shared PlasmaState
3. Unified entity IDs (trivariate hash)
4. Cross-world entity relationships

**Phase 4: Gateway Integration (Week 4)**
1. Gateway uses PLASMA-ECS for entity management
2. Agent entities in Legion World (Layer 2)
3. Intel entities in apecs (Layer 1)
4. Cognitive orchestration in ATLAS (Layer 3)

### 4.4 ECS Fix Checklist

- [ ] Create `ctas7-plasma-ecs` crate
- [ ] Integrate Legion (Layer 2) - preserve Slot Graph
- [ ] Integrate apecs (Layer 1)
- [ ] Integrate ATLAS (Layer 3)
- [ ] Unified PlasmaState
- [ ] Verify Slot Graph hash/unicode routing
- [ ] Migrate Network World to apecs (Layer 1)
- [ ] Unified state management
- [ ] Gateway PLASMA-ECS integration
- [ ] Agent entities in Legion World
- [ ] Intel entities in apecs
- [ ] Cognitive orchestration in ATLAS

---

## 5. Implementation Timeline

### Week 1: Foundation
- Smart crate retrofit for core gateway crates
- Create unified agent registry
- Create intel gateway
- Create PLASMA-ECS crate

### Week 2: Integration
- Smart crate retrofit for integration crates
- Agent consolidation (PLASMA-ECS integration)
- Intel consolidation (Python bridge)
- Network World migration to apecs

### Week 3: Gateway Integration
- Smart crate retrofit for supporting crates
- Gateway agent integration
- Gateway intel integration
- Gateway PLASMA-ECS integration

### Week 4: Testing & Validation
- Test all smart crate features
- Test agent consolidation
- Test intel consolidation
- Test ECS fixes
- Performance validation

---

## 6. Success Criteria

**Smart Crate Retrofit:**
- ✅ All gateway crates have smart-crate.toml
- ✅ All crates RFC-9001 through RFC-9005 compliant
- ✅ No Blake3 references (Murmur3-128 only)
- ✅ All integrations working (Health Dashboard, Lightning QA, ATLAS, PLASMA)

**Agent Consolidation:**
- ✅ Unified agent registry operational
- ✅ All agents use unified model
- ✅ PLASMA-ECS integration complete
- ✅ Gateway agent routing working

**Intel Consolidation:**
- ✅ Unified intel gateway operational
- ✅ Python tools bridge working
- ✅ USIM/EEI integration complete
- ✅ Gateway intel routing working

**ECS Fixes:**
- ✅ PLASMA-ECS operational (Legion + apecs + ATLAS)
- ✅ Network World migrated to apecs
- ✅ Unified state management
- ✅ Slot Graph hash/unicode routing preserved

---

**Status:** Plan complete, ready for implementation



