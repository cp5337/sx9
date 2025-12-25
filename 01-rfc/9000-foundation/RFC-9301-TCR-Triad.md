# RFC-9301: Thyristor-Crystal-RingBus (TCR) Triad

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

The TCR Triad provides infrastructure-layer signal processing for CTAS 7.x systems. It consists of three interconnected components that process trivariate streams through gating, lattice filtering, and mesh broadcast.

### 1.1 Signal Flow Architecture

```
Input (Trivariate Stream) → THYRISTOR (Gate/Latch) → CRYSTAL (Lattice/Phonon) 
    → RING BUS (Broadcast) → BERNOULLI ZONE (Probabilistic Gate) → Response Continuum
```

### 1.2 TCR Scope (Critical Architectural Decision)

**APPLIES TO:**
- Plasma Defender (port latching, attack waves, node mesh)
- CDN edge nodes (connection state, cache invalidation)
- Honeypots/deception (trap triggers, lure propagation)
- ICS/SCADA gateways (safety interlocks, PLC rings)
- Sensor networks (alert latching, mesh topology)

**DOES NOT APPLY TO:**
- HD4 core phase transitions (use H1/H2 convergence RFC-9024/9025)
- Bernoulli Zone decisions (use Thalamic Filter)
- Agent orchestration (use NATS message bus)

> TCR is infrastructure-layer signal processing, NOT core HD4 logic.

---

## 2. Thyristor (Software Defined Thyristor)

### 2.1 Mathematical Model

Latching gate mechanism with hysteresis behavior:

```
S(t+1) = {
    ON      if S(t) = OFF and input ≥ θ_activate
    ON      if S(t) = ON and input ≥ θ_hold
    RECOVERY if S(t) = ON and input < θ_hold
    OFF     if S(t) = RECOVERY and t > τ_r
}
```

### 2.2 Default Parameters

| Parameter | Symbol | Default Value |
|-----------|--------|---------------|
| Activation Threshold | θ_activate | 0.750000 |
| Hold Threshold | θ_hold | 0.500000 |
| Hysteresis Gap | Δθ | 0.250000 |
| Recovery Time | τ_r | 100ms |

### 2.3 HD4 Phase Integration

Escalating thresholds per operational phase:

| HD4 Phase | θ_activate | θ_hold |
|-----------|------------|--------|
| Hunt | 0.600000 | 0.400000 |
| Detect | 0.700000 | 0.500000 |
| Disrupt | 0.800000 | 0.600000 |
| Disable | 0.850000 | 0.700000 |
| Dominate | 0.900000 | 0.800000 |

### 2.4 Rust Implementation

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThyristorState {
    Off,
    On,
    Recovery { started_at: Instant },
}

pub struct SoftwareDefinedThyristor {
    pub state: ThyristorState,
    pub theta_activate: f64,      // 6-decimal: 0.750000
    pub theta_hold: f64,          // 6-decimal: 0.500000
    pub recovery_time: Duration,  // Default: 100ms
    pub input_history: VecDeque<f64>, // 64 samples
    pub delta_angle: DeltaAngle,
}

impl SoftwareDefinedThyristor {
    pub fn process(&mut self, input: f64) -> ThyristorState {
        // Input normalized [0.0, 1.0]
        let normalized = input.clamp(0.0, 1.0);
        self.input_history.push_back(normalized);
        if self.input_history.len() > 64 {
            self.input_history.pop_front();
        }
        
        match self.state {
            ThyristorState::Off => {
                if normalized >= self.theta_activate {
                    self.state = ThyristorState::On;
                }
            }
            ThyristorState::On => {
                if normalized < self.theta_hold {
                    self.state = ThyristorState::Recovery { 
                        started_at: Instant::now() 
                    };
                }
            }
            ThyristorState::Recovery { started_at } => {
                if started_at.elapsed() >= self.recovery_time {
                    self.state = ThyristorState::Off;
                }
            }
        }
        self.state
    }
}
```

### 2.5 Plasma Defender Integration

- Port blocking via thyristor latching
- Connection latching for persistent threats
- Deception triggers for honeypot activation
- Rate limiting with hysteresis
- Quarantine gates for isolation

---

## 3. Crystal (4D Decision Lattice)

### 3.1 Temporal Decision Propagation

Phonon mechanics for decision wave propagation through lattice structure.

### 3.2 Lattice Structure

```rust
pub struct Crystal4D {
    pub facets: [[f64; 9]; 9],           // 9 facets, 9 frequencies each
    pub coupling_matrix: [[f64; 9]; 9],  // Cross-facet interactions
    pub q_factor: f64,                   // Resonance sharpness
    pub mode: CrystalMode,
    pub resolution: (u32, u32, u32, u32), // Default: 32×32×32×64
}

#[derive(Debug, Clone, Copy)]
pub enum CrystalMode {
    Bandpass,   // Pass specific frequency range
    Notch,      // Block specific frequency range
    Harmonic,   // Pattern detection mode
}
```

### 3.3 Crystal Tuning Capabilities

- Center frequency adjustment
- Bandwidth control
- Coupling strength modulation
- Q-factor tuning for sharpness
- Facet-based filtering (9 facets = Nonagon dimensions)

### 3.4 Resonant Frequencies

Each facet corresponds to a Nonagon vertex with specific resonant characteristics for its domain.

---

## 4. Ring Bus (Circular Interconnect)

### 4.1 Topology

9-node ring for process communication with token-based fair arbitration.

### 4.2 Message Format

```rust
pub struct RingMessage {
    pub id: u64,                    // Message ID (unique)
    pub source: u16,                // Source node ID
    pub destination: u16,           // Destination (0xFFFF = broadcast)
    pub msg_type: RingMessageType,  // Message type enum
    pub payload: RingPayload,       // Variable length payload
    pub delta_angle: DeltaAngle,    // Associated delta angle (all 3 axes)
    pub hop_count: u8,              // Hop count (TTL)
    pub timestamp_us: u64,          // Timestamp (microseconds)
    pub checksum: u32,              // CRC32 checksum
}

#[repr(u8)]
pub enum RingMessageType {
    ThyristorStateChange = 0x01,
    CrystalPhononInject  = 0x02,
    CrystalDefectNotify  = 0x03,
    Hd4PhaseTransition   = 0x04,
    DeltaAngleUpdate     = 0x05,
    ConvergenceScore     = 0x06,
    Heartbeat            = 0x07,
    Token                = 0x08,
    Fault                = 0xFF,
}
```

### 4.3 Ring Bus Features

- Bidirectional ring for fault tolerance
- Token passing for arbitration
- Broadcast support (destination = 0xFFFF)
- Deterministic latency characteristics
- Max hops TTL enforcement

---

## 5. Delta Angle Integration

### 5.1 DeltaAngle Structure

```rust
pub struct DeltaAngle {
    pub x: f64,  // Semantic axis (6-decimal precision)
    pub y: f64,  // Operational axis (6-decimal precision)
    pub z: f64,  // Temporal axis (6-decimal precision)
}

impl DeltaAngle {
    /// Calculate magnitude in degrees
    pub fn magnitude_degrees(&self) -> f64 {
        ((self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()) * 180.0
    }
    
    /// Map Y-axis to HD4 phase
    pub fn hd4_phase(&self) -> Hd4Phase {
        match self.y {
            y if y < 0.200000 => Hd4Phase::Hunt,
            y if y < 0.400000 => Hd4Phase::Detect,
            y if y < 0.600000 => Hd4Phase::Disrupt,
            y if y < 0.800000 => Hd4Phase::Disable,
            _ => Hd4Phase::Dominate,
        }
    }
}
```

### 5.2 Axis Definitions

| Axis | Domain | Range Mapping |
|------|--------|---------------|
| X | Semantic | Recon → Exfil (MITRE Kill Chain) |
| Y | Operational | Hunt → Dominate (HD4 Phases) |
| Z | Temporal | Historical → Predictive |

---

## 6. Unicode Allocation (U+E710-E733)

### 6.1 Thyristor Glyphs

| Glyph | Codepoint | Meaning |
|-------|-----------|---------|
| 🜚 | U+E710 | OFF |
| 🜛 | U+E711 | ON |
| 🜜 | U+E712 | RECOVERY |

### 6.2 Crystal Glyphs

| Glyph | Codepoint | Meaning |
|-------|-----------|---------|
| 🜠 | U+E720 | INJECT |
| 🜡 | U+E721 | PROPAGATE |
| 🜢 | U+E722 | INTERFERE |
| 🜣 | U+E723 | DEFECT |

### 6.3 Ring Bus Glyphs

| Glyph | Codepoint | Meaning |
|-------|-----------|---------|
| 🜰 | U+E730 | SEND |
| 🜱 | U+E731 | RECEIVE |
| 🜲 | U+E732 | TOKEN |
| 🜳 | U+E733 | BROADCAST |

---

## 7. Implementation Requirements

### 7.1 MUST

- θ_activate > θ_hold (hysteresis requirement)
- 4D+ crystal lattices
- 6-decimal delta angles (all 3 axes)
- Ring Bus broadcast support (0xFFFF)
- State change broadcasts

### 7.2 SHOULD

- Bidirectional ring topology
- 32×32×32×64 crystal resolution
- Token arbitration
- Configurable recovery times

### 7.3 MAY

- Multiple Ring Bus instances
- Custom phonon decay functions
- Priority queuing

---

## References

- RFC-9024: H1 Operational Layer
- RFC-9025: H2 Semantic Layer
- RFC-9302: Nonagon Analytic Node
- RFC-9303: Crystal Realms Kinematics
