# RFC-9116: APECS → Legion Bridge ECS Architecture

**Status:** DRAFT  
**Version:** 1.0.0  
**Date:** 2025-12-07  
**Author:** CTAS-7 Team  
**Related RFCs:** RFC-9021 (Graph Convergence), RFC-9022 (OODA), RFC-9026 (Hourglass-Bernoulli), RFC-9001 (Trivariate Hashing), RFC-9002 (Unicode Assembly)

---

## 🎯 **EXECUTIVE SUMMARY**

This RFC defines the **three-layer ECS (Entity Component System) architecture** that bridges **apecs (Layer 1: Async I/O)** → **Legion (Layer 2: Hot-Path)** with **ATLAS Daemon (Layer 3: Cognitive)** integration, SlotGraph hash → archetype mapping, and ANN (Artificial Neural Network) cognitive processing.

**Key Innovation:** Permanent duality between cold-path (apecs) and hot-path (Legion) with microsecond-speed hash → Unicode → action routing via SlotGraph.

---

## 📊 **ARCHITECTURE OVERVIEW**

### **Three-Layer ECS Stack**

```
┌─────────────────────────────────────────────────────────────┐
│                    PLASMA-ECS LAYERS                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  LAYER 3: ATLAS (Cognitive)                                 │
│  ═══════════════════════════                                │
│  • ATLAS Daemon (1ms OODA loop)                            │
│  • sx9-atlas-bus (ring buffer, PlasmaState)                │
│  • Crystal resonance, SDT gate control                     │
│  • Priority routing (critical/urgent/normal)                │
│  • NATS bridge for distributed ops                         │
│  • ANN integration for cognitive analysis                  │
│                                                              │
│  LAYER 2: Legion (Deterministic Batch)                       │
│  ═══════════════════════════════════════                    │
│  • High-performance batch processing                       │
│  • Deterministic tick-based world state                    │
│  • Hot-path operations (<1ms latency)                      │
│  • Entity-component queries                                │
│  • Schedule execution                                      │
│  • SlotGraph integration (hash → archetype, zero lookup)  │
│  • Pure integers only (no strings in hot-path)             │
│                                                              │
│  LAYER 1: apecs (Async I/O)                                 │
│  ═══════════════════════════                                │
│  • Async-friendly operations                               │
│  • WASM-compatible                                         │
│  • I/O-bound tasks (network, database)                     │
│  • Mission entity creation                                 │
│  • JSON/TOML parsing (cold-path only)                      │
│  • Database queries (Supabase, SurrealDB, Sled)            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔄 **DATA FLOW**

### **Complete Pipeline**

```
┌─────────────────────────────────────────────────────────────┐
│  1. NATS Event: sx9.threat.honeypot                        │
│     └─> HoneypotEvent { entity_id, threat_hash, ... }      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  2. apecs (Layer 1: Async I/O)                             │
│     └─> Create MissionEntity                               │
│         • Parse JSON/TOML (cold-path)                      │
│         • Extract: threat_hash, unicode_trigger,            │
│           primitive_bitfield, speed_class                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  3. ATLAS Daemon (Layer 3: Cognitive)                      │
│     └─> Process Mission (OODA Loop)                        │
│         • Observe: Extract mission context                 │
│         • Orient: Calculate convergence, delta angle        │
│         • Decide: Crystal resonance check                   │
│         • Act: Return AtlasOutcome                          │
│     └─> Output: { ooda_phase, crystal_resonance,           │
│                   delta_angle, allowed }                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  4. ANN Processor (Cognitive Analysis)                     │
│     └─> Process Mission + AtlasOutcome                      │
│         • ONNX model inference                              │
│         • Input: crystal_resonance, delta_angle,           │
│           speed_class, allowed                              │
│         • Output: confidence, recommendation,              │
│           reason_trace                                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Sled KVS (Hot-Path Storage)                            │
│     └─> Store: hash → (unicode + bitfield + speed_class)   │
│         • Target: < 3μs lookup                             │
│         • Key: threat_hash (trivariate)                     │
│         • Value: JSON { unicode_trigger,                    │
│                        primitive_bitfield,                 │
│                        speed_class }                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  6. SlotGraph (Hash → Archetype Mapping)                    │
│     └─> Get Archetype: hash → (slot_id, archetype_id)      │
│         • Zero lookup (direct pointer)                      │
│         • O(1) hash map lookup                             │
│         • Creates new archetype if not found                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  7. Legion (Layer 2: Hot-Path)                             │
│     └─> Insert Entity: HotPathEntity                        │
│         • Pure integers only (no strings)                   │
│         • Components: entity_id (u64),                      │
│           unicode_trigger (u32),                            │
│           primitive_bitfield (u64),                        │
│           speed_class (u8), slot_id (u64)                  │
│         • Direct archetype pointer                          │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  8. NATS Event: sx9.hotpath.load                           │
│     └─> HotpathLoadEvent { entity_id, hash,                │
│                            slot_id, archetype_id, ... }     │
└─────────────────────────────────────────────────────────────┘
```

---

## 🏗️ **COMPONENT SPECIFICATIONS**

### **Layer 1: apecs (Async I/O)**

**Purpose:** Handle I/O-bound operations, mission entity creation, JSON/TOML parsing.

**Responsibilities:**
- Create `MissionEntity` from NATS events
- Parse JSON/TOML (cold-path only)
- Database queries (Supabase, SurrealDB, Sled)
- WebSocket connections
- File operations
- Network calls

**Data Structures:**
```rust
pub struct MissionEntity {
    pub entity_id: String,           // Mission entity ID
    pub threat_hash: String,         // Trivariate hash (SCH_CUID_UUID)
    pub unicode_trigger: u32,        // Unicode operation (U+E000-E9FF)
    pub primitive_bitfield: u64,     // PTCC primitive bitfield
    pub speed_class: u8,             // Speed class (0-255)
}
```

**Performance:** Async I/O operations, no strict latency requirements.

---

### **Layer 2: Legion (Hot-Path)**

**Purpose:** Deterministic batch processing, hot-path operations, microsecond-speed execution.

**Responsibilities:**
- High-performance entity-component queries
- Deterministic tick-based world state
- Batch processing
- Schedule execution
- SlotGraph hash → archetype routing

**Data Structures:**
```rust
// Hot-path entity (pure integers, no strings)
#[derive(Debug, Clone, Copy)]
pub struct HotPathEntity {
    pub entity_id: u64,              // Archetype ID (direct pointer)
    pub unicode_trigger: u32,        // Unicode operation
    pub primitive_bitfield: u64,     // PTCC primitive bitfield
    pub speed_class: u8,              // Speed class
    pub slot_id: u64,                // SlotGraph slot ID
}
```

**Performance Targets:**
- Entity insertion: < 1μs
- Component queries: < 100ns
- Batch processing: < 1ms per 1000 entities
- **No strings in hot-path** (all data must be integers or bitfields)

---

### **Layer 3: ATLAS (Cognitive)**

**Purpose:** Cognitive orchestration, OODA loop, crystal resonance, decision-making.

**Responsibilities:**
- 1ms OODA loop (Zone B compliance)
- Crystal resonance evaluation
- Delta angle calculation
- SDT gate control
- Priority routing
- Convergence calculation

**Data Structures:**
```rust
#[derive(Debug, Clone)]
pub struct AtlasOutcome {
    pub ooda_phase: String,           // "observe" | "orient" | "decide" | "act"
    pub crystal_resonance: f32,      // 0.0-1.0
    pub delta_angle: u16,             // 0-65535 (0-360°)
    pub allowed: bool,                // Crystal resonance + delta angle check
}
```

**Performance Targets:**
- OODA cycle: < 1ms (Zone B)
- Crystal resonance: < 100μs
- Delta angle calculation: < 50μs

---

## 🧠 **ANN INTEGRATION**

### **ANN Processor**

**Purpose:** Cognitive analysis using ONNX neural network models.

**Input Features:**
- `crystal_resonance` (f32): 0.0-1.0
- `delta_angle` (f32): Normalized 0-1 (from u16 0-65535)
- `speed_class` (f32): Normalized 0-1 (from u8 0-255)
- `allowed` (f32): 1.0 if allowed, 0.0 if blocked

**Output:**
```rust
pub struct AnnResult {
    pub confidence: f32,              // 0.0-1.0
    pub recommendation: String,      // "proceed" | "escalate" | "block"
    pub reason_trace: Vec<String>,   // Decision trace
}
```

**Model:** ONNX format, loaded at startup.

**Performance:** < 500μs inference time.

---

## 🎯 **SLOTGRAPH INTEGRATION**

### **Hash → Archetype Mapping**

**Purpose:** Zero-lookup routing from trivariate hash to Legion archetype.

**Implementation:**
```rust
pub struct SlotGraphIntegration {
    /// Hash → (slot_id, archetype_id) mapping
    hash_to_archetype: Arc<RwLock<HashMap<String, (String, u64)>>>,
    legion_world: Arc<RwLock<World>>,
}
```

**Lookup Process:**
1. Hash lookup in `HashMap` (O(1))
2. If found: Return `(slot_id, archetype_id)` (zero lookup)
3. If not found: Generate new archetype from hash, register, return

**Performance:** O(1) hash map lookup, < 100ns.

---

## 💾 **SLED KVS INTEGRATION**

### **Hot-Path Storage**

**Purpose:** Microsecond-speed hash → (unicode + bitfield + speed_class) lookups.

**Data Structure:**
```rust
pub struct HotPathData {
    pub unicode_trigger: u32,
    pub primitive_bitfield: u64,
    pub speed_class: u8,
}
```

**Storage:**
- **Key:** `threat_hash` (trivariate hash, bytes)
- **Value:** JSON-encoded `HotPathData`

**Performance Targets:**
- **Lookup:** < 3μs
- **Write:** < 3μs

---

## ⚡ **PERFORMANCE TARGETS**

### **End-to-End Pipeline**

| Stage | Target | Measurement |
|-------|--------|-------------|
| **Total (honeypot → tarpit)** | < 9.2μs | From NATS event to Legion insertion |
| **apecs (mission entity)** | < 1μs | JSON parsing, entity creation |
| **ATLAS (OODA loop)** | < 1ms | Zone B compliance |
| **ANN (inference)** | < 500μs | ONNX model inference |
| **Sled KVS (lookup)** | < 3μs | Hash → data lookup |
| **SlotGraph (archetype)** | < 100ns | Hash → archetype lookup |
| **Legion (insertion)** | < 1μs | Entity insertion |

### **Hot-Path Requirements**

- **No strings in Legion entities** (all data must be integers or bitfields)
- **No JSON parsing in hot-path** (all parsing in apecs Layer 1)
- **Direct pointers only** (SlotGraph archetype IDs, no hash lookups in Legion)

---

## 🔌 **NATS INTEGRATION**

### **Subjects**

| Subject | Direction | Payload | Purpose |
|---------|-----------|---------|---------|
| `sx9.threat.honeypot` | Input | `HoneypotEvent` | Receive honeypot events from Plasma |
| `sx9.hotpath.load` | Output | `HotpathLoadEvent` | Publish hot-path load events to Legion consumers |

### **Event Structures**

```rust
// Input: sx9.threat.honeypot
pub struct HoneypotEvent {
    pub entity_id: String,
    pub threat_hash: String,         // Trivariate hash
    pub unicode_trigger: u32,
    pub primitive_bitfield: u64,
    pub speed_class: u8,
    pub timestamp: i64,
}

// Output: sx9.hotpath.load
pub struct HotpathLoadEvent {
    pub entity_id: String,
    pub hash: String,
    pub unicode_trigger: u32,
    pub primitive_bitfield: u64,
    pub speed_class: u8,
    pub slot_id: String,
    pub archetype_id: u64,           // Direct pointer
}
```

---

## 🧪 **LIVE FIRE TEST**

### **Test Scenario**

**Honeypot → Plasma → apecs → Legion → Scorpion Tarpit**

1. **Honeypot triggers** → Plasma-Defender detects threat
2. **Plasma publishes** → `sx9.threat.honeypot` event
3. **Bridge receives** → apecs creates mission entity
4. **ATLAS processes** → OODA loop, crystal resonance
5. **ANN analyzes** → Cognitive recommendation
6. **Sled stores** → Hash → unicode mapping
7. **SlotGraph routes** → Hash → archetype
8. **Legion inserts** → Hot-path entity
9. **Bridge publishes** → `sx9.hotpath.load` event
10. **Scorpion fires** → Tarpit deployment

**Target:** Complete pipeline in < 9.2μs (measured on bare-metal).

---

## 📋 **IMPLEMENTATION CHECKLIST**

- [x] **Bridge crate structure** (`ctas7-apecs-legion-bridge`)
- [x] **apecs Layer 1** (mission entity creation)
- [x] **ATLAS Layer 3** (OODA loop integration)
- [x] **ANN processor** (ONNX model integration)
- [x] **Legion Layer 2** (hot-path entity insertion)
- [x] **SlotGraph integration** (hash → archetype)
- [x] **Sled KVS** (hot-path storage)
- [x] **NATS integration** (event streaming)
- [ ] **ONNX model** (ANN inference model)
- [ ] **Live fire test** (honeypot → tarpit)
- [ ] **Performance benchmarks** (< 9.2μs target)

---

## 🔗 **RELATED RFCs**

- **RFC-9021**: Graph Convergence Theory (OODA as graph node state)
- **RFC-9022**: OODA Vertical Escalation (1ms cognitive tick)
- **RFC-9026**: Hourglass-Bernoulli Cognitive Architecture (Zone B compliance)
- **RFC-9001**: Dual-Trivariate Hashing (SCH, CUID, UUID)
- **RFC-9002**: Unicode Operational Routing System (U+E000-E9FF)
- **RFC-9130**: L2 NATS Kali Execution Platform (NATS JetStream)

---

## 🎯 **KEY PRINCIPLES**

1. **Permanent Duality:** Cold-path (apecs) and hot-path (Legion) are permanently separated.
2. **Zero Lookup:** SlotGraph provides direct pointer from hash to archetype.
3. **No Strings in Hot-Path:** All Legion entities use pure integers/bitfields.
4. **Microsecond Speed:** Total pipeline < 9.2μs from honeypot to tarpit.
5. **Cognitive Integration:** ATLAS + ANN provide intelligent routing decisions.

---

## 📝 **CHANGELOG**

### **Version 1.0.0 (2025-12-07)**
- Initial RFC draft
- Three-layer ECS architecture defined
- ANN integration specified
- SlotGraph hash → archetype mapping
- Performance targets established

---

**Status:** DRAFT - Awaiting review and implementation validation.

**The lattice is waiting to split.** 🔥


