# Plasma - Canonical Definition

**Version:** 1.0  
**Status:** Canonical  
**Date:** December 2025  

---

## What is Plasma?

**Plasma** is the **unified state field** that tracks cognitive, energetic, and operational state across the entire SX9/CTAS system. It is the **medium through which all commands flow** and the **gate that controls whether operations proceed**.

Think of it like **physical plasma** (the 4th state of matter):
- **Physical plasma**: Ionized gas that conducts electricity, responds to magnetic fields, and carries energy
- **SX9 Plasma**: Unified state field that conducts commands, responds to crystal resonance, and carries cognitive energy

---

## Plasma Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        PLASMA FIELD                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │              PLASMA STATE (PlasmaState)                   │  │
│   │                                                            │  │
│   │   • Delta Angle (Δθ):  Cognitive state (0-65535 → 0-360°) │  │
│   │   • Entropy (H):        Randomness/chaos (Monte Carlo)     │  │
│   │   • Excited:            Crystal ringing? (bool)           │  │
│   │   • SDT Gate:           Thyristor state (Off/Primed/      │  │
│   │                         Conducting/Latched)                │  │
│   │   • Last Ring Strength: Crystal resonance score (0.0-1.0) │  │
│   │   • Trigger Count:      How many times gate fired         │  │
│   │   • Supersession Count: How many lineages killed         │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │              CRYSTAL RESONANCE LAYER                       │  │
│   │                                                            │  │
│   │   Payload + Entropy + Delta Angle → Ring Strength         │  │
│   │                                                            │  │
│   │   Crystal Families:                                       │  │
│   │   • Orbital:      High entropy tolerance                  │  │
│   │   • GroundStation: Stable, strict thresholds             │  │
│   │   • TarPit:       Inverted (rings on anomalies)           │  │
│   │   • Silent:       Only perfect matches                    │  │
│   │   • Adaptive:     Learns from patterns                    │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │              SDT GATE (Software-Defined Thyristor)        │  │
│   │                                                            │  │
│   │   Ring Strength → Gate State                             │  │
│   │                                                            │  │
│   │   States:                                                 │  │
│   │   • Off:        Blocked, no flow                         │  │
│   │   • Primed:     Waiting for trigger                       │  │
│   │   • Conducting: Gate open, flow allowed                   │  │
│   │   • Latched:    Permanent, requires reset                │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │              COMMAND FLOW DECISION                         │  │
│   │                                                            │  │
│   │   Conducting/Latched → Command Lives                      │  │
│   │   Off/Primed        → Command Dies                        │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Plasma Components

### 1. PlasmaState

**Location:** `sx9-atlas-bus/src/plasma.rs`

**Definition:** The atomic, lock-free state structure that tracks the plasma field.

```rust
pub struct PlasmaState {
    delta_angle: AtomicU16,      // Cognitive state (0-65535 → 0-360°)
    entropy: AtomicU32,           // Randomness/chaos
    excited: AtomicBool,          // Crystal ringing?
    sdt_state: AtomicU8,         // Thyristor gate state
    last_trigger_tick: AtomicU64, // When gate last fired
    trigger_count: AtomicU32,     // How many times fired
    last_ring_strength: AtomicU32, // Crystal resonance (f32 as bits)
    supersession_count: AtomicU32, // Lineages killed
}
```

**Properties:**
- **Cache-line aligned** (64 bytes) for zero false sharing
- **Lock-free** (atomic operations only)
- **Zero-allocation** (stack-only)
- **Thread-safe** (multiple readers, single writer)

### 2. Crystal Resonance

**Definition:** The physics-based evaluation that determines if a payload "rings" the crystal.

**Input:**
- Payload hash (Murmur3 of command/data)
- Entropy (Monte Carlo randomness)
- Delta angle (current cognitive state)

**Output:**
- Ring strength (0.0-1.0)
- Delta class (None/Micro/Soft/Hard/Critical)

**Formula:**
```
ring_strength = (entropy_weight × normalized_entropy)
              + (delta_weight × (1.0 - normalized_delta))
              + (hash_weight × hash_coherence)
```

### 3. SDT Gate (Software-Defined Thyristor)

**Definition:** The gate control that decides if commands flow based on crystal resonance.

**States:**
- **Off**: Blocked, no flow allowed
- **Primed**: Forward-biased, waiting for trigger
- **Conducting**: Gate open, flow allowed
- **Latched**: Permanent, requires explicit reset

**Firing Rules:**
```
if ring_strength >= 0.98 → Latched (perfect ring, permanent)
if ring_strength >= gate_thresh → Conducting (fire)
if ring_strength < holding_thresh → Off (anode drop)
```

---

## Plasma Metaphor

### Physical Plasma vs. SX9 Plasma

| Physical Plasma | SX9 Plasma |
|----------------|------------|
| Ionized gas | Unified state field |
| Conducts electricity | Conducts commands |
| Responds to magnetic fields | Responds to crystal resonance |
| Carries energy | Carries cognitive energy |
| 4th state of matter | 4th layer of control |

### Why "Plasma"?

1. **Conductivity**: Like physical plasma conducts electricity, SX9 plasma conducts commands
2. **Responsiveness**: Like plasma responds to fields, SX9 plasma responds to crystal resonance
3. **Energy Carrier**: Like plasma carries energy, SX9 plasma carries cognitive state
4. **Excitation**: Like plasma can be excited (ionized), SX9 plasma can be excited (crystal ringing)
5. **Gate Control**: Like plasma can be gated (magnetic confinement), SX9 plasma is gated (SDT thyristor)

---

## Plasma Lifecycle

```
1. INITIALIZATION
   └─ PlasmaState::new()
      • Delta angle: 0
      • Entropy: 0
      • Excited: false
      • SDT Gate: Off

2. COMMAND ARRIVES
   └─ Payload + metadata
      • Build payload hash
      • Get current entropy
      • Get current delta angle

3. CRYSTAL RESONANCE
   └─ Crystal.resonate_payload()
      • Compute ring strength (0.0-1.0)
      • Classify delta class
      • Update excited state

4. SDT GATE EVALUATION
   └─ PlasmaState.resonate_poly()
      • Check ring strength vs. thresholds
      • Update SDT gate state
      • Return: command lives or dies

5. COMMAND FLOW
   └─ If Conducting/Latched:
      • Command proceeds
      • Update trigger count
      • Update last trigger tick
   └─ If Off/Primed:
      • Command blocked
      • Increment supersession count
      • Kill lineage

6. ANODE DROP (if entropy drought)
   └─ Entropy < entropy_drought
      • SDT Gate: Conducting → Off
      • Commands blocked
      • Requires entropy recovery
```

---

## Plasma Integration Points

### 1. Message/Command Flow

**Location:** `sx9-atlas-bus/src/bus.rs`

**Usage:**
```rust
// Every command must pass through plasma
if !bus.plasma.resonate_poly(&polycrystal, &cmd.payload, tick, &config) {
    // Crystal didn't ring, command dies
    bus.supersede(cmd.lineage);
    return;
}
```

### 2. Port Allocation

**Location:** `ctas7-real-port-manager` (future integration)

**Usage:**
```rust
// Port allocation gated by plasma
if !plasma_state.resonate_poly(&polycrystal, &service_payload, tick, &config) {
    return Err(PortManagerError::CrystalRejected);
}
```

### 3. Security Monitoring

**Location:** `ctas7-plasma-defender` (future integration)

**Usage:**
```rust
// Threat severity based on crystal resonance
let ring_strength = plasma_state.resonate_poly(&polycrystal, &alert_payload, tick, &config);
let severity = match ring_strength.final_strength {
    r if r >= 0.90 => AlertSeverity::Critical,
    r if r >= 0.75 => AlertSeverity::High,
    _ => AlertSeverity::Low,
};
```

---

## Plasma Properties

### 1. Atomicity

Plasma state is **atomic** — all operations are lock-free and thread-safe.

### 2. Persistence

**Current:** In-memory only (lost on restart)  
**Future:** Snapshot/restore capability

### 3. Observability

**Current:** Limited (trigger_count, supersession_count)  
**Future:** Full metrics, history, streaming

### 4. Performance

- **Latency:** <250ns for resonance check
- **Memory:** 64 bytes (cache-line aligned)
- **Allocations:** 0 (stack-only)

---

## Plasma vs. Other Systems

| System | Purpose | Plasma Role |
|--------|---------|-------------|
| **Neural Mux** | Routing | Plasma gates routing decisions |
| **Port Manager** | Port allocation | Plasma gates port allocation |
| **Plasma Defender** | Security monitoring | Plasma determines threat severity |
| **ATLAS Daemon** | Cognitive processing | Plasma gates cognitive commands |
| **Gateway** | API surface | Plasma gates API requests |

**Plasma is the universal gate** — everything flows through it.

---

## Plasma Definition (One Sentence)

**Plasma is the unified state field that tracks cognitive, energetic, and operational state, and gates all commands through crystal resonance and SDT thyristor control.**

**Extended Definition (Dual-Layer):**

**Plasma consists of two layers:**
1. **PlasmaPhysics (PlasmaState)**: Physics-based gate control aligned with CUID (Cognitive Unique Identifier)
2. **PlasmaSemantic (NEW)**: Semantic understanding and LISP rule evaluation aligned with SCH (Semantic Content Hash)

**Together, they provide both physics-based and meaning-based command gating.**

---

## Plasma Principles

1. **Everything flows through plasma** — no command bypasses the gate
2. **Dual-layer evaluation** — physics (CUID-aligned) + semantic (SCH-aligned)
3. **Physics-based security** — crystal resonance is not bypassable
4. **Semantic understanding** — LISP rules evaluate meaning, not just physics
5. **State-aware** — plasma tracks cognitive state (delta angle, entropy)
6. **Atomic and lock-free** — high performance, thread-safe
7. **Unified control** — combined decision from both layers

## Plasma Architecture Evolution

**Current (v1.0):**
- PlasmaState only (physics-based, CUID-aligned)

**Future (v2.0):**
- DualPlasma (PlasmaState + PlasmaSemantic)
- Physics layer: CUID-aligned, crystal/thyristor
- Semantic layer: SCH-aligned, LISP rules

See `PLASMA-DUAL-LAYER-ARCHITECTURE.md` for full specification.

---

**The crystal is the quartz. The thyristor is the switch it triggers. Plasma is the field they operate in.**

