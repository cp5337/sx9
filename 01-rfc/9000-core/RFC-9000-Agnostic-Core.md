# RFC-9000 — Synaptix9 Agnostic Core & Ontology Standard

**Version:** 1.0  
**Status:** Final  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1, All Verticals  
**Author:** CTAS Core Engineering Group  
**Dependencies:** None (root specification)

---

## 1. Purpose and Scope

### 1.1 Why SX9 Exists

Synaptix9 (SX9) provides a **domain-agnostic cognitive foundation** that enables:
- Universal threat intelligence processing
- Cross-domain operational analysis
- Standardized hash addressing and routing
- Unified ontology for knowledge representation

### 1.2 Relationship Hierarchy

```
SX9 Agnostic Core (this RFC)
    ├── CTAS-7 (Tactical Analysis System)
    ├── Cognigraph (Knowledge Graph Engine)
    ├── PTCC 32 Primitives (Universal Operations)
    └── HD4 (Hunt/Detect/Disrupt/Disable/Dominate)
```

**This RFC is the parent of all others.**

---

## 2. Agnostic Core Principles

### 2.1 Domain-Agnostic First

SX9 makes **no assumptions** about the operational domain. The same primitives that model stock market operations model threat actor behaviors.

### 2.2 PTCC 32 Primitives as Core Mental Model

All operations ultimately decompose into 32 universal primitives (see RFC-9100):

| Category | Primitives |
|----------|-----------|
| **Core CRUD** (4) | CREATE, READ, UPDATE, DELETE |
| **Communication** (2) | SEND, RECEIVE |
| **Data Processing** (2) | TRANSFORM, VALIDATE |
| **Control Flow** (4) | BRANCH, LOOP, RETURN, CALL |
| **Network Operations** (4) | CONNECT, DISCONNECT, ROUTE, FILTER |
| **Security** (4) | AUTHENTICATE, AUTHORIZE, ENCRYPT, DECRYPT |
| **Resource Management** (4) | ALLOCATE, DEALLOCATE, LOCK, UNLOCK |
| **State Management** (4) | SAVE, RESTORE, CHECKPOINT, ROLLBACK |
| **Coordination** (4) | COORDINATE, SYNCHRONIZE, SIGNAL, WAIT |

### 2.3 Separation of Concerns

SX9 enforces strict separation:

| Layer | Purpose | RFCs |
|-------|---------|------|
| **Ontology** | Knowledge structure and relationships | RFC-9000 |
| **Identity** | Trivariate hashing | RFC-9001 |
| **Routing** | Unicode operational routing | RFC-9002 |
| **Execution** | Primitive operations and classification | RFC-9003 |
| **Transport** | Deterministic routing | RFC-9004 |
| **Storage** | Persistence and retrieval | RFC-9005 |

---

## 3. HD4 Framework

### 3.1 The Five Phases

| Phase | Code | Purpose |
|-------|------|---------|
| **Hunt** | H | Proactive threat seeking |
| **Detect** | D1 | Identification of anomalies |
| **Disrupt** | D2 | Interrupt adversary operations |
| **Disable** | D3 | Neutralize threat capabilities |
| **Dominate** | D4 | Achieve operational superiority |

### 3.2 HD4 Phase Encoding

Encoded in SCH bits 16-23:

```
┌────────────────────────────────────────┐
│  HD4 Phase Bit Encoding                │
├────────────────────────────────────────┤
│  Bit 16: Hunt active                   │
│  Bit 17: Detect active                 │
│  Bit 18: Disrupt active                │
│  Bit 19: Disable active                │
│  Bit 20: Dominate active               │
│  Bits 21-23: Phase intensity (0-7)     │
└────────────────────────────────────────┘
```

---

## 4. Ontology Structure

### 4.1 Entity Types

| Type | Code | Description |
|------|------|-------------|
| ACTOR | 0x01 | Human or automated agent |
| ASSET | 0x02 | Protected resource |
| THREAT | 0x03 | Adversarial element |
| INDICATOR | 0x04 | Observable artifact |
| TECHNIQUE | 0x05 | Method or procedure |
| TOOL | 0x06 | Capability or instrument |
| CAMPAIGN | 0x07 | Coordinated activity set |
| INCIDENT | 0x08 | Detected event |

### 4.2 Relationship Types

| Relationship | Inverse | Description |
|--------------|---------|-------------|
| USES | USED_BY | Tool/technique usage |
| TARGETS | TARGETED_BY | Attack direction |
| INDICATES | INDICATED_BY | Evidence relationship |
| MITIGATES | MITIGATED_BY | Defense relationship |
| CONTAINS | CONTAINED_IN | Hierarchical |
| CORRELATES | CORRELATED_WITH | Statistical link |

---

## 5. Execution Tiers

### 5.1 Latency Zones

```
┌─────────────────────────────────────────────────────────────────────┐
│                    EXECUTION TIER DIAGRAM                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Zone A (< 1μs)     │  Zone B (< 1ms)    │  Zone C (< 100ms)       │
│  ───────────────    │  ──────────────    │  ────────────────       │
│  Ring Bus           │  Legion ECS        │  Apecs Async            │
│  Lock-free          │  Direct dispatch   │  Queue-based            │
│  L1 cache           │  L2/L3 cache       │  Memory/disk            │
│                     │                    │                         │
│  SmartCrate: ✓      │  SmartCrate: ✓     │  SmartCrate: ✓          │
│  NATS: ✗            │  NATS: ✓           │  NATS: ✓                │
│  Database: ✗        │  Database: Sled    │  Database: Full         │
│                                                                     │
│  Zone D (> 100ms)                                                   │
│  ────────────────                                                   │
│  Orchestrator                                                       │
│  Batch processing                                                   │
│  External APIs                                                      │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.2 Tier Selection Logic

```rust
pub fn select_tier(operation: &Operation) -> ExecutionTier {
    match operation.latency_budget {
        budget if budget < Duration::from_micros(1) => ExecutionTier::RingBus,
        budget if budget < Duration::from_millis(1) => ExecutionTier::Legion,
        budget if budget < Duration::from_millis(100) => ExecutionTier::Apecs,
        _ => ExecutionTier::Orchestrator,
    }
}
```

---

## 6. Database Stack

### 6.1 Canonical Stack (Post-SurrealDB Deprecation)

| Database | Purpose | Latency |
|----------|---------|---------|
| **Sled** | Embedded KV, SmartCrate manifests | < 1ms |
| **Sledis** | Redis-compatible cache | < 1ms |
| **SlotGraph** | Graph operations | < 10ms |
| **Neon** | PostgreSQL ACID transactions | < 50ms |
| **Supabase** | Auth, realtime, edge functions | Variable |
| **NATS** | Pub/sub messaging | < 1ms |

### 6.2 SurrealDB Deprecation Notice

SurrealDB has been **deprecated** due to:
- 180-crate dependency bloat
- Inconsistent query performance
- Complexity in edge deployments

All SurrealDB references SHALL be migrated to SlotGraph + Sled.

---

## 7. Port Allocation

### 7.1 Standard Blocks

| Block | Range | Purpose |
|-------|-------|---------|
| Core | 18100-18109 | Foundation services |
| Gateway | 18110-18119 | API gateways |
| Orbital | 18120-18129 | Satellite systems |
| CDN | 18130-18139 | Content delivery |
| Neural | 18140-18149 | ML/AI services |
| Messaging | 18150-18159 | NATS, queues |
| Analytics | 18160-18169 | Metrics, logging |
| Monitoring | 18170-18179 | Health, status |
| Deception | 18180-18189 | Honeypots, decoys |

### 7.2 Mirror Block

Each block has a mirror at +10000 for redundancy:
- Core mirror: 28100-28109
- Gateway mirror: 28110-28119
- etc.

---

## 8. Crate Architecture

### 8.1 ECS Backend Selection

| Backend | Use Case | Status |
|---------|----------|--------|
| **Legion** | Hot path execution | Primary |
| **Apecs** | Async operations | Secondary |
| **Bevy** | — | **FORBIDDEN** |

### 8.2 TCR (Type-Crate Registry)

All types must be registered in `sx9-foundation-core`:

```rust
// Correct: Use foundation types
use sx9_foundation_core::types::{Rune, Slot, TrivariateHash};

// FORBIDDEN: Local type definitions that shadow foundation
struct Rune { ... } // ← REJECTED by QA
```

---

## 9. Quality Gates

All code must pass:

1. **Static Analysis** - No forbidden patterns
2. **Architecture Compliance** - Correct layer usage
3. **Type Compliance** - TCR validation
4. **Lightning QA** - Grade B+ minimum

See RFC-9121 for details.

---

## 10. References

- RFC-9001: Trivariate Hashing Standard
- RFC-9002: Unicode Operational Routing System
- RFC-9003: Operation Classifier & Escalation Logic
- RFC-9004: Deterministic Routing Architecture
- RFC-9005: Unified Schema Specification
- RFC-9100: Dual-Trivariate PTCC Integration

---

**End of RFC-9000**
