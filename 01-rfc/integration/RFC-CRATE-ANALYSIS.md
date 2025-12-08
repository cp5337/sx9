# RFC Crate Analysis — Required Crates by Specification

**Version:** 1.0
**Date:** November 26, 2025
**Status:** Analysis Complete

---

## Executive Summary

This document maps all RFC specifications (9000-9104) to required Rust crates, identifying existing implementations, gaps, and dependencies.

### Compliance Summary

| RFC | Title | Required Crates | Existing | Missing | Compliance |
|-----|-------|-----------------|----------|---------|------------|
| RFC-9000 | Agnostic Core & Ontology | 5 | 3 | 2 | 60% |
| RFC-9001 | Trivariate Hashing | 4 | 2 | 2 | 50% |
| RFC-9002 | Unicode Routing | 3 | 1 | 2 | 33% |
| RFC-9003 | Operation Classifier | 4 | 2 | 2 | 50% |
| RFC-9004 | Deterministic Routing | 6 | 2 | 4 | 33% |
| RFC-9005 | Unified Schema | 3 | 1 | 2 | 33% |
| RFC-9100 | Dual-Trivariate PTCC | 4 | 1 | 3 | 25% |
| RFC-9101 | Smart Crate System | 5 | 3 | 2 | 60% |

---

## RFC-9000: Agnostic Core & Ontology

### Requirements
1. PTCC 32/33 primitive system
2. HD4 phase integration
3. Ontology engine (TS + Rust)
4. SPIRES/OntoGPT integration
5. Graph fabric (Cognigraph)

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-foundation-core` | Core primitives, types | **EXISTS** | `ctas7-foundation-core/` |
| `ctas7-ontology-engine` | Ontology processing | **MISSING** | — |
| `ctas7-hd4-system` | HD4 phase management | **PARTIAL** | In `foundation-tactical` |
| `ctas7-cognigraph` | Graph knowledge fabric | **MISSING** | — |
| `ctas7-ptcc-primitives` | 32 primitives impl | **PARTIAL** | In `foundation-core` |

### Dependencies
```toml
[dependencies]
murmur3 = "0.5"          # Hashing (RFC-9001)
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v7"] }
petgraph = "0.6"         # Graph structures
```

---

## RFC-9001: Trivariate Hashing Standard

### Requirements
1. SCH (Synaptic Convergent Hash) - Murmur3-128
2. CUID (Contextual Unique Identifier) - Base96, 16 chars
3. UUIDv7 for lineage
4. Delta-angle supersession logic
5. N-V-N-N grammar tokenization

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-hashing-engine` | Trivariate hash generation | **EXISTS** | `ctas7-hashing-engine/` |
| `ctas7-foundation-core` | Base96, CUID slots | **EXISTS** | `ctas7-foundation-core/` |
| `ctas7-nvnn-grammar` | N-V-N-N tokenizer | **MISSING** | — |
| `ctas7-delta-angle` | Supersession logic | **MISSING** | — |

### Key Implementation

```rust
// Required struct per RFC-9001
pub struct TrivariateHash {
    pub sch: u128,   // Murmur3-128
    pub cuid: [u8; 16], // Base96 encoded
    pub uuid: uuid::Uuid, // v7
}

// CUID Slot Map (16 chars)
// 1-4:   Timestamp shard
// 5-7:   Execution Env
// 8-9:   Agent ID
// 10-11: Δ-Angle Derivative
// 12:    State Flag (Cold/Warm/Hot/L2)
// 13-14: Lineage
// 15-16: Nonce
```

### Dependencies
```toml
[dependencies]
murmur3 = "0.5"
uuid = { version = "1.0", features = ["v7"] }
base64 = "0.21"  # For Base96 extension
```

---

## RFC-9002: Unicode Operational Routing

### Requirements
1. U+E000–E9FF allocation
2. CUID → Unicode encoding
3. Neural Mux routing
4. Semantic affinity routing

### Unicode Allocation

| Range | Class | Purpose | Owner Crate |
|-------|-------|---------|-------------|
| E000–E1FF | A | Execution runes | `ctas7-foundation-core` |
| E200–E2FF | B | CUID slot mapping | `ctas7-hashing-engine` |
| E300–E3FF | C | Semantic routing | `ctas7-neural-mux` |
| E400–E6FF | D | Neural Mux ops | `ctas7-neural-mux` |
| E700–E7FF | Reserved | Future | — |
| E800–E9FF | Experimental | Research | — |
| EA00–EAFF | IAC | Infrastructure triggers | `ctas7-foundation-daemon` |

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-unicode-router` | Unicode → route mapping | **MISSING** | — |
| `ctas7-neural-mux` | <250ns routing | **EXISTS** | `neural-mux/` |
| `ctas7-dsl-unicode` | DSL Unicode layer | **MISSING** | — |

---

## RFC-9003: Operation Classifier & Escalation

### Requirements
1. Operation categories (Intelligence, Defensive, Offensive, Admin)
2. Escalation tiers (WASM → Microkernel → Kernel → Container → Firefly → Orb)
3. Authentication per tier
4. Delta gate evaluation

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-foundation-tactical` | Operation classification | **EXISTS** | `ctas7-foundation-tactical/` |
| `ctas7-escalation-engine` | Tier management | **MISSING** | — |
| `ctas7-wasm-ground-station` | WASM tier | **EXISTS** | `ctas7-wasm-ground-station/` |
| `ctas7-wasm-primitives` | WASM primitives | **EXISTS** | `ctas7-wasm-primitives/` |

### Escalation Tier Mapping

| Tier | Port Range | Crate |
|------|------------|-------|
| 1. WASM | — | `ctas7-wasm-primitives` |
| 2. Microkernel | — | `kali-microkernel-tasks` |
| 3. Kernel Crate | 1800-1900 | `ctas7-smart-crate-orchestrator` |
| 4. Multi-Crate | 1800-1900 | `ctas7-smart-crate-orchestrator` |
| 5. Container | — | Docker integration |
| 6. Firefly | — | `firefly` (Terraform) |
| 7. Orb | — | OrbitalOS |

---

## RFC-9004: Deterministic Routing Architecture

### Requirements
1. Neural Mux (<250ns routing)
2. Port Manager (deterministic allocation)
3. CDN Architecture (consistent hashing)
4. Bernoulli Zones (latency bounds)
5. IAC Integration (Unicode triggers)
6. ATLAS cognitive tick

### Required Crates

| Crate | Purpose | Status | Port | Location |
|-------|---------|--------|------|----------|
| `ctas7-neural-mux` | Lock-free routing | **EXISTS** | 18107 | `neural-mux/` |
| `ctas7-real-port-manager` | Deterministic ports | **EXISTS** | 18100-18199 | `ctas7-real-port-manager/` |
| `ctas7-statistical-cdn` | Hash-based CDN | **EXISTS** | 18112-18134 | `ctas7-statistical-analysis-cdn/` |
| `ctas7-bernoulli-monitor` | Zone compliance | **MISSING** | — | — |
| `ctas7-iac-controller` | Manifold spawning | **MISSING** | — | — |
| `ctas7-atlas` | ATLAS daemon (cognitive ticks) | **EXISTS** | 18106 | `ctas7-foundation-daemon/` (pkg: ctas7-atlas) |

### Bernoulli Zone Implementation

```rust
pub enum BernoulliZone {
    Tactical,      // < 50μs
    Operational,   // 50μs - 1ms
    Analytical,    // 1ms - 100ms
    Infrastructure // 100ms - 60s
}
```

### Port Allocation (RFC-9004)

| Range | Purpose | Crate |
|-------|---------|-------|
| 18104 | Port Manager | `ctas7-real-port-manager` |
| 18105 | Trivariate Hash Engine | `ctas7-hashing-engine` |
| 18106 | ATLAS Daemon | `ctas7-atlas-daemon` |
| 18107 | Neural Mux | `ctas7-neural-mux` |
| 18108 | Health Dashboard | `ctas7-health-dashboard` |
| 18109 | Lightning QA | `lightning-qa-engine` |
| 18110 | PLASMA Monitor | `ctas7-plasma-monitor` |
| 18111 | Smart Crate Orchestrator | `ctas7-smart-crate-orchestrator` |
| 18112-18134 | CDN Nodes | `ctas7-statistical-cdn` |
| 18500 | Foundation Daemon Core | `ctas7-foundation-daemon` |
| 18600 | Backend MCP | `ctas7-foundation-daemon` |
| 18630 | ABE Controlled Access | `ctas7-foundation-daemon` |
| 18650 | Service Discovery | `ctas7-foundation-daemon` |
| 1800-1900 | Dynamic Crates | `ctas7-smart-crate-orchestrator` |

---

## RFC-9005: Unified Schema

### Requirements
1. Supabase as single source of truth
2. Unified `entities` table
3. GLAF-compatible relationships
4. ATLAS cognitive nodes
5. IAC manifold schema

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-sledis` | Universal cognitive storage | **EXISTS** | `ctas7-sledis/` |
| `ctas7-atlas` | GLAF correlation engine | **EXISTS** | `ctas7-foundation-daemon/src/threat_reaction/glaf_correlation.rs` |
| `ctas7-schema-sync` | Schema migration | **MISSING** | — |

---

## RFC-9100/9016: Dual-Trivariate PTCC Integration

### Requirements
1. 32 PTCC primitives encoded in SCH
2. Primary trivariate (tactical) + Secondary trivariate (semantic)
3. Delta-angle expansion
4. Lisp compression operators
5. Legion ECS integration
6. SlotGraph integration

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-foundation-core` | PTCC primitives | **PARTIAL** | `ctas7-foundation-core/` |
| `ctas7-slotgraph` | SlotGraph engine | **MISSING** | — |
| `ctas7-world-ecs` | Legion ECS world | **EXISTS** | `ctas7-world-ecs/` |
| `ctas7-lisp-compression` | Lisp operators | **MISSING** | — |

### 32 Primitives Encoding

```rust
pub enum Primitive {
    // Core CRUD (0x00-0x03)
    Create = 0x00, Read = 0x01, Update = 0x02, Delete = 0x03,
    // Communication (0x04-0x05)
    Send = 0x04, Receive = 0x05,
    // Data Processing (0x06-0x07)
    Transform = 0x06, Validate = 0x07,
    // Control Flow (0x08-0x0B)
    Branch = 0x08, Loop = 0x09, Return = 0x0A, Call = 0x0B,
    // Network (0x0C-0x0F)
    Connect = 0x0C, Disconnect = 0x0D, Route = 0x0E, Filter = 0x0F,
    // Security (0x10-0x13)
    Authenticate = 0x10, Authorize = 0x11, Encrypt = 0x12, Decrypt = 0x13,
    // Resource (0x14-0x17)
    Allocate = 0x14, Deallocate = 0x15, Lock = 0x16, Unlock = 0x17,
    // State (0x18-0x1B)
    Save = 0x18, Restore = 0x19, Checkpoint = 0x1A, Rollback = 0x1B,
    // Coordination (0x1C-0x1F)
    Coordinate = 0x1C, Synchronize = 0x1D, Signal = 0x1E, Wait = 0x1F,
}
```

---

## RFC-9101: Smart Crate System

### Requirements
1. Blake3-free (Murmur3-128 only)
2. Health Dashboard integration
3. Lightning QA Engine (port 18109)
4. ATLAS 1ms cognitive ticks
5. PLASMA threat analysis
6. Neural Mux <250ns

### Required Crates

| Crate | Purpose | Status | Location |
|-------|---------|--------|----------|
| `ctas7-smart-crate-orchestrator` | Crate management | **EXISTS** | `smart-crate-system/` |
| `ctas7-smart-cdn-gateway` | CDN gateway | **EXISTS** | `ctas7-smart-cdn-gateway/` |
| `ctas7-cognivault-smart-crate` | CogniVault | **EXISTS** | `smart-crate-system/` |
| `ctas7-plasma-monitor` | Threat analysis | **MISSING** | — |
| `ctas7-lightning-qa` | QA engine | **PARTIAL** | Python only |

---

## Missing Crates Summary

### Critical (Required for RFC compliance)

| Priority | Crate | RFC | Purpose |
|----------|-------|-----|---------|
| P0 | `ctas7-bernoulli-monitor` | 9004 | Latency zone compliance |
| P1 | `ctas7-unicode-router` | 9002 | Unicode → route mapping |
| P1 | `ctas7-escalation-engine` | 9003 | Tier escalation logic |
| P1 | `ctas7-nvnn-grammar` | 9001 | N-V-N-N tokenizer |
| P1 | `ctas7-delta-angle` | 9001/9100 | Supersession logic |
| P2 | `ctas7-slotgraph` | 9100 | SlotGraph engine |
| P2 | `ctas7-ontology-engine` | 9000 | Ontology processing |
| P2 | `ctas7-lisp-compression` | 9100 | Lisp operators |
| P2 | `ctas7-iac-controller` | 9004 | IAC manifold spawning |
| P2 | `ctas7-schema-sync` | 9005 | Schema migration |

**Note:** `ctas7-atlas` (in foundation-daemon) already provides ATLAS daemon and GLAF correlation.

---

## Dependency Graph

```
RFC-9000 (Agnostic Core)
    │
    ├── RFC-9001 (Trivariate Hashing)
    │   ├── ctas7-hashing-engine ✅
    │   ├── ctas7-foundation-core ✅
    │   ├── ctas7-nvnn-grammar ❌
    │   └── ctas7-delta-angle ❌
    │
    ├── RFC-9002 (Unicode Routing)
    │   ├── ctas7-neural-mux ✅
    │   ├── ctas7-unicode-router ❌
    │   └── ctas7-dsl-unicode ❌
    │
    ├── RFC-9003 (Operation Classifier)
    │   ├── ctas7-foundation-tactical ✅
    │   ├── ctas7-escalation-engine ❌
    │   └── ctas7-wasm-primitives ✅
    │
    ├── RFC-9004 (Deterministic Routing)
    │   ├── ctas7-neural-mux ✅
    │   ├── ctas7-real-port-manager ✅
    │   ├── ctas7-statistical-cdn ✅
    │   ├── ctas7-atlas (foundation-daemon) ✅
    │   ├── ctas7-bernoulli-monitor ❌
    │   └── ctas7-iac-controller ❌
    │
    ├── RFC-9005 (Unified Schema)
    │   ├── ctas7-sledis ✅
    │   ├── ctas7-atlas (GLAF correlation) ✅
    │   └── ctas7-schema-sync ❌
    │
    └── RFC-9100 (Dual-Trivariate PTCC)
        ├── ctas7-foundation-core ✅
        ├── ctas7-world-ecs ✅
        ├── ctas7-slotgraph ❌
        └── ctas7-lisp-compression ❌

Legend: ✅ Exists | ❌ Missing
```

---

## Recommended Implementation Order

### Phase 1: Core Infrastructure (Week 1-2)
1. `ctas7-bernoulli-monitor` — Zone compliance monitoring
   - Note: `ctas7-real-port-manager` already exists (port 18104)
   - Note: `ctas7-atlas` already exists in foundation-daemon

### Phase 2: Hashing & Routing (Week 3-4)
3. `ctas7-nvnn-grammar` — N-V-N-N tokenizer
4. `ctas7-delta-angle` — Supersession logic
5. `ctas7-unicode-router` — Unicode routing

### Phase 3: Escalation & Control (Week 5-6)
6. `ctas7-escalation-engine` — Tier management
7. `ctas7-iac-controller` — Manifold spawning

### Phase 4: Graph & Analysis (Week 7-8)
8. `ctas7-slotgraph` — SlotGraph engine
9. `ctas7-glaf-processor` — Graph operations
10. `ctas7-ontology-engine` — Ontology processing
11. `ctas7-lisp-compression` — Lisp operators

---

## Workspace Cargo.toml Addition

```toml
[workspace]
members = [
    # Existing (RFC Compliant)
    "ctas7-foundation-core",           # RFC-9000, 9001, 9100
    "ctas7-foundation-daemon",         # RFC-9004, 9005 (publishes as ctas7-atlas)
    "ctas7-foundation-tactical",       # RFC-9003
    "ctas7-hashing-engine",            # RFC-9001
    "ctas7-neural-mux",                # RFC-9002, 9004
    "ctas7-real-port-manager",         # RFC-9004
    "ctas7-sledis",                    # RFC-9005
    "ctas7-smart-crate-orchestrator",  # RFC-9101
    "ctas7-statistical-analysis-cdn",  # RFC-9004
    "ctas7-wasm-primitives",           # RFC-9003
    "ctas7-world-ecs",                 # RFC-9100

    # NEW - RFC Compliance
    "ctas7-bernoulli-monitor",
    "ctas7-nvnn-grammar",
    "ctas7-delta-angle",
    "ctas7-unicode-router",
    "ctas7-escalation-engine",
    "ctas7-iac-controller",
    "ctas7-slotgraph",
    "ctas7-glaf-processor",
    "ctas7-ontology-engine",
    "ctas7-lisp-compression",
]
```

---

**End of RFC Crate Analysis**
