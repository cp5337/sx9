# RFC-9301: Thyristor, Crystal, and Ring Bus Architecture

**Version:** 1.0.0  
**Status:** NORMATIVE  
**Date:** 2025-12-06  
**Author:** CTAS Architecture Team  
**Depends-On:** RFC-9300 (HD4), RFC-9001 (Trivariate Hashing), RFC-9024 (H2 Convergence)  

---

## Abstract

This RFC defines signal processing mechanisms for the SX9/CTAS architecture:

### Core: Agnostic Phase System (§1)
- **OTL/HD4 parallel 5-phase structure** — Plan/Hunt → Insert/Detect → Operate/Disrupt → Maintain/Disable → Retire/Dominate
- Phase transitions governed by **H1/H2 Convergence** (RFC-9024/9025)
- Simple threshold-based state machine for HD4 core

### Infrastructure: TCR Triad (§3-5)
- **Thyristor** — Latching gate with hysteresis for state commitment
- **Crystal** — 4D decision lattice for temporal decision propagation  
- **Ring Bus** — Circular interconnect topology for process communication

**TCR Triad applies to:**
| System | Use Case |
|--------|----------|
| **Plasma Defender** | Port latching, attack wave propagation, node mesh |
| **CDN Edge Nodes** | Connection state, cache invalidation, edge coordination |
| **TETH Toolchains** | Tool resonance, harmonic interference, operational waveforms |
| **Honeypots/Deception** | Trap triggers, lure propagation, decoy networks |
| **ICS/SCADA Gateways** | Safety interlocks, process state, PLC rings |
| **Sensor Networks** | Alert latching, detection waves, mesh topology |

**TCR Triad does NOT apply to:**
- HD4 core phase transitions (use H1/H2 convergence)
- Bernoulli Zone decisions (use Thalamic Filter)
- Agent orchestration (use NATS message bus)

---

## 1. Agnostic Core Principle

### 1.1 Dual Phase Systems

The SX9/CTAS architecture operates with **two parallel 5-phase systems** that serve different operational contexts:

| Index | OTL (Operational Lifecycle) | HD4 (Threat Response) |
|-------|----------------------------|----------------------|
| 0 | **PLAN** | **HUNT** |
| 1 | **INSERT** | **DETECT** |
| 2 | **OPERATE** | **DISRUPT** |
| 3 | **MAINTAIN** | **DISABLE** |
| 4 | **RETIRE** | **DOMINATE** |

**OTL (Operational Task Lifecycle):** Non-adversarial systems lifecycle for industrial, orbital, infrastructure, and friendly force operations.

**HD4 (Hunt-Detect-Disrupt-Disable-Dominate):** Adversarial threat response framework that overlays ANY phase of OTL.

### 1.2 Domain Applications

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         AGNOSTIC CORE MAPPING                               │
│                                                                             │
│  INDUSTRIAL (OTL)          THREAT (HD4)           COMBINED                  │
│  ─────────────────         ────────────           ────────────────────────  │
│  PLAN                      HUNT                   Plan while Hunting        │
│  INSERT                    DETECT                 Insert with Detection     │
│  OPERATE                   DISRUPT                Operate through Disruption│
│  MAINTAIN                  DISABLE                Maintain by Disabling     │
│  RETIRE                    DOMINATE               Retire with Dominance     │
│                                                                             │
│  Y-Axis Value: 0.000000    0.250000    0.500000    0.750000    1.000000    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.3 Phase Equivalence Table

| Y-Axis | OTL Phase | HD4 Phase | Industrial Context | Threat Context |
|--------|-----------|-----------|-------------------|----------------|
| 0.000000 | PLAN | HUNT | Mission design, requirements | Active threat-seeking |
| 0.250000 | INSERT | DETECT | Deployment, installation | Threat identification |
| 0.500000 | OPERATE | DISRUPT | Nominal operations | Active interference |
| 0.750000 | MAINTAIN | DISABLE | Sustainment, updates | Neutralization |
| 1.000000 | RETIRE | DOMINATE | Decommission, disposal | Full control |

### 1.4 Rust Implementation (Agnostic Core)

```rust
/// Agnostic phase system supporting both OTL and HD4
/// RFC-9301 §1.4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Phase {
    /// OTL: Plan / HD4: Hunt
    Phase0 = 0,
    /// OTL: Insert / HD4: Detect  
    Phase1 = 1,
    /// OTL: Operate / HD4: Disrupt
    Phase2 = 2,
    /// OTL: Maintain / HD4: Disable
    Phase3 = 3,
    /// OTL: Retire / HD4: Dominate
    Phase4 = 4,
}

impl Phase {
    /// Get OTL (Operational Task Lifecycle) label
    pub fn otl_label(&self) -> &'static str {
        match self {
            Phase::Phase0 => "PLAN",
            Phase::Phase1 => "INSERT",
            Phase::Phase2 => "OPERATE",
            Phase::Phase3 => "MAINTAIN",
            Phase::Phase4 => "RETIRE",
        }
    }
    
    /// Get HD4 (Threat Response) label
    pub fn hd4_label(&self) -> &'static str {
        match self {
            Phase::Phase0 => "HUNT",
            Phase::Phase1 => "DETECT",
            Phase::Phase2 => "DISRUPT",
            Phase::Phase3 => "DISABLE",
            Phase::Phase4 => "DOMINATE",
        }
    }
    
    /// Get Y-axis value (6-decimal precision)
    pub fn y_axis(&self) -> f64 {
        match self {
            Phase::Phase0 => 0.000000,
            Phase::Phase1 => 0.250000,
            Phase::Phase2 => 0.500000,
            Phase::Phase3 => 0.750000,
            Phase::Phase4 => 1.000000,
        }
    }
    
    /// From Y-axis value
    pub fn from_y_axis(y: f64) -> Self {
        match y {
            y if y < 0.125000 => Phase::Phase0,
            y if y < 0.375000 => Phase::Phase1,
            y if y < 0.625000 => Phase::Phase2,
            y if y < 0.875000 => Phase::Phase3,
            _ => Phase::Phase4,
        }
    }
}

/// Context selector for phase interpretation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseContext {
    /// Industrial/Orbital/Infrastructure operations
    Operational,
    /// Adversarial threat response
    Threat,
    /// Combined (both contexts active)
    Combined,
}
```

---

## 2. Tier 1: HD4 Core (Simple Implementation)

### 2.1 Simple Gate

A basic threshold gate for phase transitions. No hysteresis, no latching - just clean threshold comparison.

```rust
/// Simple threshold gate for HD4 phase transitions
/// RFC-9301 §2.1
#[derive(Debug, Clone)]
pub struct SimpleGate {
    /// Threshold for transition (0.0 to 1.0)
    threshold: f64,
    
    /// Current input value
    current_value: f64,
    
    /// Gate state
    open: bool,
}

impl SimpleGate {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            current_value: 0.0,
            open: false,
        }
    }
    
    /// Process input, return whether gate is open
    pub fn process(&mut self, input: f64) -> bool {
        self.current_value = input;
        self.open = input >= self.threshold;
        self.open
    }
    
    /// Check if gate would open at given input
    pub fn would_open(&self, input: f64) -> bool {
        input >= self.threshold
    }
}
```

### 2.2 HD4 State Machine

Deterministic phase transitions based on convergence scores.

```rust
/// HD4 Phase State Machine
/// RFC-9301 §2.2
#[derive(Debug, Clone)]
pub struct Hd4StateMachine {
    /// Current phase
    current_phase: Phase,
    
    /// Phase context (OTL or HD4 labels)
    context: PhaseContext,
    
    /// Transition thresholds per phase
    thresholds: [f64; 5],
    
    /// Allow backward transitions?
    allow_regression: bool,
    
    /// Transition history
    history: Vec<PhaseTransition>,
}

#[derive(Debug, Clone)]
pub struct PhaseTransition {
    pub from: Phase,
    pub to: Phase,
    pub score: f64,
    pub timestamp: u64,
    pub delta_angle: DeltaAngle,
}

impl Hd4StateMachine {
    /// Default thresholds aligned with H1/H2 convergence (RFC-9024)
    pub const DEFAULT_THRESHOLDS: [f64; 5] = [
        0.000000,  // Phase0 (Hunt/Plan) - always accessible
        0.750000,  // Phase1 (Detect/Insert) - 75% convergence
        0.800000,  // Phase2 (Disrupt/Operate) - 80% convergence
        0.850000,  // Phase3 (Disable/Maintain) - 85% convergence
        0.900000,  // Phase4 (Dominate/Retire) - 90% convergence
    ];
    
    pub fn new(context: PhaseContext) -> Self {
        Self {
            current_phase: Phase::Phase0,
            context,
            thresholds: Self::DEFAULT_THRESHOLDS,
            allow_regression: true,
            history: Vec::new(),
        }
    }
    
    /// Evaluate convergence score and potentially transition
    pub fn evaluate(&mut self, h2_score: f64, delta_angle: DeltaAngle) -> Option<PhaseTransition> {
        let target_phase = self.score_to_phase(h2_score);
        
        if target_phase == self.current_phase {
            return None;
        }
        
        // Check if transition is allowed
        let forward = target_phase as u8 > self.current_phase as u8;
        if !forward && !self.allow_regression {
            return None;
        }
        
        // Execute transition
        let transition = PhaseTransition {
            from: self.current_phase,
            to: target_phase,
            score: h2_score,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            delta_angle,
        };
        
        self.current_phase = target_phase;
        self.history.push(transition.clone());
        
        Some(transition)
    }
    
    fn score_to_phase(&self, score: f64) -> Phase {
        match score {
            s if s < self.thresholds[1] => Phase::Phase0,
            s if s < self.thresholds[2] => Phase::Phase1,
            s if s < self.thresholds[3] => Phase::Phase2,
            s if s < self.thresholds[4] => Phase::Phase3,
            _ => Phase::Phase4,
        }
    }
    
    /// Get current phase with context-appropriate label
    pub fn current_label(&self) -> &'static str {
        match self.context {
            PhaseContext::Operational => self.current_phase.otl_label(),
            PhaseContext::Threat => self.current_phase.hd4_label(),
            PhaseContext::Combined => self.current_phase.hd4_label(), // Default to HD4
        }
    }
    
    /// Get current Y-axis value
    pub fn current_y(&self) -> f64 {
        self.current_phase.y_axis()
    }
}
```

### 2.3 Event Bus (Simple Pub/Sub)

Basic publish/subscribe for phase transition events.

```rust
/// Simple event bus for HD4 events
/// RFC-9301 §2.3
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum Hd4Event {
    /// Phase transition occurred
    PhaseTransition(PhaseTransition),
    
    /// Convergence score updated
    ConvergenceUpdate {
        h1: f64,
        h2: f64,
        delta_angle: DeltaAngle,
    },
    
    /// Delta angle changed
    DeltaAngleUpdate(DeltaAngle),
    
    /// Alert/notification
    Alert {
        severity: AlertSeverity,
        message: String,
        phase: Phase,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

pub struct Hd4EventBus {
    /// Broadcast sender
    tx: broadcast::Sender<Hd4Event>,
    
    /// Event history (bounded)
    history: Vec<Hd4Event>,
    
    /// Max history size
    max_history: usize,
}

impl Hd4EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            history: Vec::new(),
            max_history: 1000,
        }
    }
    
    /// Publish an event
    pub fn publish(&mut self, event: Hd4Event) {
        // Store in history
        self.history.push(event.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        
        // Broadcast (ignore if no receivers)
        let _ = self.tx.send(event);
    }
    
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Hd4Event> {
        self.tx.subscribe()
    }
    
    /// Get recent events
    pub fn recent(&self, n: usize) -> &[Hd4Event] {
        let start = self.history.len().saturating_sub(n);
        &self.history[start..]
    }
}
```

### 2.4 HD4 Core Controller

Combines gate, state machine, and event bus into a simple controller.

```rust
/// HD4 Core Controller - Simple implementation for early deployment
/// RFC-9301 §2.4
pub struct Hd4Controller {
    /// State machine
    state_machine: Hd4StateMachine,
    
    /// Phase gates (one per phase)
    gates: [SimpleGate; 5],
    
    /// Event bus
    event_bus: Hd4EventBus,
    
    /// Current delta angle
    delta_angle: DeltaAngle,
}

impl Hd4Controller {
    pub fn new(context: PhaseContext) -> Self {
        Self {
            state_machine: Hd4StateMachine::new(context),
            gates: [
                SimpleGate::new(0.000000),
                SimpleGate::new(0.750000),
                SimpleGate::new(0.800000),
                SimpleGate::new(0.850000),
                SimpleGate::new(0.900000),
            ],
            event_bus: Hd4EventBus::new(256),
            delta_angle: DeltaAngle::zero(),
        }
    }
    
    /// Process convergence scores
    pub fn process(&mut self, h1: f64, h2: f64) -> Option<PhaseTransition> {
        // Update delta angle Y-axis based on h2
        self.delta_angle.y = h2;
        
        // Publish convergence update
        self.event_bus.publish(Hd4Event::ConvergenceUpdate {
            h1,
            h2,
            delta_angle: self.delta_angle,
        });
        
        // Evaluate state machine
        if let Some(transition) = self.state_machine.evaluate(h2, self.delta_angle) {
            self.event_bus.publish(Hd4Event::PhaseTransition(transition.clone()));
            Some(transition)
        } else {
            None
        }
    }
    
    /// Get current phase
    pub fn current_phase(&self) -> Phase {
        self.state_machine.current_phase
    }
    
    /// Get current phase label
    pub fn current_label(&self) -> &'static str {
        self.state_machine.current_label()
    }
    
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Hd4Event> {
        self.event_bus.subscribe()
    }
}
```

### 2.5 Tier 1 Summary

| Component | Purpose | Complexity |
|-----------|---------|------------|
| SimpleGate | Threshold comparison | O(1) |
| Hd4StateMachine | Phase transitions | O(1) |
| Hd4EventBus | Pub/sub messaging | O(n) subscribers |
| Hd4Controller | Unified interface | Combines above |

**Implement Tier 1 first.** It provides full HD4/OTL phase management with minimal complexity.

---

## 3. Tier 2: TCR Triad (Advanced Implementation)

> **Note:** Tier 2 adds sophisticated signal processing. Implement only after Tier 1 is stable.

### 3.0 TCR Triad Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           TCR TRIAD ARCHITECTURE                            │
│                                                                             │
│   SENSORY INPUT (Trivariate Streams)                                       │
│          │                                                                  │
│          ▼                                                                  │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                         RING BUS                                     │  │
│   │                                                                      │  │
│   │     ┌──────────┐      ┌──────────┐      ┌──────────┐               │  │
│   │     │ THYRISTOR│──────│ CRYSTAL  │──────│ THYRISTOR│               │  │
│   │     │   T₀     │      │   C₀     │      │   T₁     │               │  │
│   │     └────┬─────┘      └────┬─────┘      └────┬─────┘               │  │
│   │          │                 │                 │                      │  │
│   │          └────────────┬────┴────┬────────────┘                      │  │
│   │                       │         │                                   │  │
│   │     ┌──────────┐      │         │      ┌──────────┐               │  │
│   │     │ CRYSTAL  │──────┘         └──────│ THYRISTOR│               │  │
│   │     │   C₁     │                       │   T₂     │               │  │
│   │     └──────────┘                       └──────────┘               │  │
│   │                                                                      │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│          │                                                                  │
│          ▼                                                                  │
│   ┌─────────────────┐                                                      │
│   │ BERNOULLI ZONE  │ ← Thalamic Filter (DistilBERT)                       │
│   │ (Probabilistic  │                                                      │
│   │  Decision Gate) │                                                      │
│   └────────┬────────┘                                                      │
│            │                                                                │
│            ▼                                                                │
│   RESPONSE CONTINUUM (Script → Microkernel → Crate → Container)            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Thyristor Process

### 6.1 Definition

A **Thyristor** is a latching gate mechanism that, once triggered past a threshold, commits to a state and maintains it until an explicit reset condition is met. Named after the semiconductor device, it provides:

1. **Latching Behavior** — Once ON, stays ON
2. **Hysteresis** — Different thresholds for activation vs. deactivation
3. **State Commitment** — Prevents oscillation around decision boundaries

### 6.2 Mathematical Model

```
Thyristor State Function:

S(t+1) = {
    ON   if S(t) = OFF AND input(t) ≥ θ_activate
    ON   if S(t) = ON  AND input(t) ≥ θ_hold
    OFF  if S(t) = ON  AND input(t) < θ_hold
    OFF  if S(t) = OFF AND input(t) < θ_activate
}

Where:
    θ_activate > θ_hold  (hysteresis gap)
    θ_activate: Activation threshold
    θ_hold:     Holding threshold (lower)
```

### 6.3 Hysteresis Parameters

| Parameter | Symbol | Default | Range | Description |
|-----------|--------|---------|-------|-------------|
| Activation Threshold | θ_a | 0.750000 | [0.000000, 1.000000] | Input level to trigger ON |
| Holding Threshold | θ_h | 0.500000 | [0.000000, θ_a) | Input level to maintain ON |
| Hysteresis Gap | Δθ | 0.250000 | (0.000000, 0.500000] | θ_a - θ_h |
| Recovery Time | τ_r | 100ms | [10ms, 10000ms] | Minimum time before re-trigger after OFF |

### 6.4 Rust Implementation (NORMATIVE)

```rust
/// Thyristor - Latching gate with hysteresis
/// RFC-9301 §3
#[derive(Debug, Clone)]
pub struct Thyristor {
    /// Current state
    state: ThyristorState,
    
    /// Activation threshold (must exceed to turn ON)
    theta_activate: f64,
    
    /// Holding threshold (must stay above to remain ON)
    theta_hold: f64,
    
    /// Last state transition timestamp
    last_transition: Instant,
    
    /// Minimum recovery time after OFF before re-activation
    recovery_time: Duration,
    
    /// Input history for delta angle computation
    input_history: VecDeque<(Instant, f64)>,
    
    /// Associated trivariate delta angle (all 3 axes, 6-decimal precision)
    delta_angle: DeltaAngle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ThyristorState {
    Off = 0,
    On = 1,
    Recovery = 2,  // Refractory period after OFF
}

impl Thyristor {
    pub const DEFAULT_THETA_ACTIVATE: f64 = 0.750000;
    pub const DEFAULT_THETA_HOLD: f64 = 0.500000;
    pub const DEFAULT_RECOVERY_MS: u64 = 100;
    
    pub fn new(theta_activate: f64, theta_hold: f64) -> Self {
        assert!(theta_activate > theta_hold, "Hysteresis requires θ_a > θ_h");
        assert!(theta_activate >= 0.0 && theta_activate <= 1.0);
        assert!(theta_hold >= 0.0 && theta_hold < theta_activate);
        
        Self {
            state: ThyristorState::Off,
            theta_activate,
            theta_hold,
            last_transition: Instant::now(),
            recovery_time: Duration::from_millis(Self::DEFAULT_RECOVERY_MS),
            input_history: VecDeque::with_capacity(64),
            delta_angle: DeltaAngle::zero(),
        }
    }
    
    /// Process input and return new state
    /// Input MUST be normalized to [0.0, 1.0]
    pub fn process(&mut self, input: f64, delta: DeltaAngle) -> ThyristorState {
        let now = Instant::now();
        
        // Record input for history
        self.input_history.push_back((now, input));
        if self.input_history.len() > 64 {
            self.input_history.pop_front();
        }
        
        // Update delta angle
        self.delta_angle = delta;
        
        let new_state = match self.state {
            ThyristorState::Off => {
                if input >= self.theta_activate {
                    self.last_transition = now;
                    ThyristorState::On
                } else {
                    ThyristorState::Off
                }
            }
            ThyristorState::On => {
                if input < self.theta_hold {
                    self.last_transition = now;
                    ThyristorState::Recovery
                } else {
                    ThyristorState::On
                }
            }
            ThyristorState::Recovery => {
                if now.duration_since(self.last_transition) >= self.recovery_time {
                    ThyristorState::Off
                } else {
                    ThyristorState::Recovery
                }
            }
        };
        
        self.state = new_state;
        new_state
    }
    
    /// Force reset to OFF state
    pub fn reset(&mut self) {
        self.state = ThyristorState::Off;
        self.last_transition = Instant::now();
    }
    
    /// Get current delta angle (all 3 axes, 6-decimal)
    pub fn delta_angle(&self) -> DeltaAngle {
        self.delta_angle
    }
}
```

### 6.5 Plasma Defender Integration

Thyristors in Plasma Defender provide latching behavior for network infrastructure:

| Component | Thyristor | θ_activate | θ_hold | Purpose |
|-----------|-----------|------------|--------|---------|
| **Port Guard** | T_port | 0.700000 | 0.400000 | Block malicious port after threshold |
| **Connection Latch** | T_conn | 0.600000 | 0.300000 | Maintain session state |
| **Deception Trigger** | T_decoy | 0.800000 | 0.500000 | Activate honeypot/lure |
| **Rate Limiter** | T_rate | 0.750000 | 0.450000 | Throttle after burst detected |
| **Quarantine Gate** | T_quar | 0.900000 | 0.700000 | Isolate compromised node |

**Example: Port Blocking Thyristor**

```rust
// Plasma Defender port blocking with hysteresis
let mut port_thyristor = Thyristor::new(0.700000, 0.400000);

// Malicious traffic score exceeds activation threshold
let threat_score = 0.85;
let state = port_thyristor.process(threat_score, delta_angle);

match state {
    ThyristorState::On => {
        // Block port - stays blocked until threat_score < 0.400000
        firewall.block_port(port);
    }
    ThyristorState::Off => {
        // Port open
    }
    ThyristorState::Recovery => {
        // Recently unblocked, monitoring
    }
}
```

---

## 5. Crystal Process

### 6.1 Definition

A **Crystal** is a 4-dimensional decision lattice structure where decisions propagate through space-time like phonons through a crystal lattice. It provides:

1. **Temporal Decision Propagation** — Decisions ripple through time
2. **Interference Patterns** — Multiple decisions can constructively/destructively interfere
3. **Lattice Defects** — Anomalies that alter propagation
4. **Symmetry Operations** — Invariants across transformations

### 6.2 Mathematical Model

```
Crystal Lattice Definition:

C = (L, S, P, D)

Where:
    L = 4D lattice points: L ⊂ ℝ⁴ (x, y, z, t)
    S = Symmetry group: S ⊂ O(4)
    P = Phonon modes: P = {p₁, p₂, ..., pₙ}
    D = Defect set: D ⊂ L

Decision Propagation:

ψ(r, t+Δt) = ∫ G(r, r', Δt) × ψ(r', t) dr'

Where:
    ψ(r, t) = Decision amplitude at position r, time t
    G(r, r', Δt) = Green's function (propagator)
```

### 6.3 Delta Angle Mapping (6-Decimal Precision)

The Crystal maps 3D trivariate delta angles to 4D lattice coordinates:

```
Lattice Mapping:

x_lattice = Δ_semantic × scale_x      // X-axis: Semantic
y_lattice = Δ_operational × scale_y   // Y-axis: Operational (HD4)
z_lattice = Δ_temporal × scale_z      // Z-axis: Temporal
t_lattice = system_tick               // T-axis: Discrete time

All values maintain 6-decimal precision minimum:
    Δ(0.500000, 0.750000, 0.333333) → L(50.000000, 75.000000, 33.333300, t)
```

### 6.4 Rust Implementation (NORMATIVE)

```rust
/// TemporalDecisionCrystal - 4D decision lattice
/// RFC-9301 §4
#[derive(Debug, Clone)]
pub struct Crystal {
    /// 4D lattice dimensions [x, y, z, t]
    dimensions: [usize; 4],
    
    /// Decision amplitudes at each lattice point
    /// Stored as flattened array for cache efficiency
    amplitudes: Vec<f64>,
    
    /// Phonon modes (propagating decisions)
    phonons: Vec<Phonon>,
    
    /// Lattice defects (anomalies)
    defects: Vec<Defect>,
    
    /// Current time slice
    current_tick: u64,
    
    /// Precision (minimum 6 decimals)
    precision: usize,
}

#[derive(Debug, Clone)]
pub struct Phonon {
    /// Origin point in 4D space
    pub origin: [f64; 4],
    
    /// Wave vector (direction of propagation)
    pub wave_vector: [f64; 4],
    
    /// Amplitude (decision strength)
    pub amplitude: f64,
    
    /// Frequency (decision urgency)
    pub frequency: f64,
    
    /// Phase
    pub phase: f64,
    
    /// Associated trivariate delta angle
    pub delta_angle: DeltaAngle,
    
    /// Creation tick
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct Defect {
    /// Position in lattice
    pub position: [f64; 4],
    
    /// Defect type
    pub defect_type: DefectType,
    
    /// Radius of influence
    pub radius: f64,
    
    /// Strength (how much it alters propagation)
    pub strength: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefectType {
    /// Point defect - local anomaly
    Point,
    /// Line defect - dislocation
    Line,
    /// Planar defect - boundary
    Planar,
    /// Volume defect - inclusion
    Volume,
}

impl Crystal {
    pub const MIN_PRECISION: usize = 6;
    
    pub fn new(dimensions: [usize; 4]) -> Self {
        let total_size: usize = dimensions.iter().product();
        
        Self {
            dimensions,
            amplitudes: vec![0.0; total_size],
            phonons: Vec::new(),
            defects: Vec::new(),
            current_tick: 0,
            precision: Self::MIN_PRECISION,
        }
    }
    
    /// Inject a decision at a point (creates a phonon)
    pub fn inject_decision(
        &mut self,
        delta_angle: DeltaAngle,
        amplitude: f64,
        urgency: f64,
    ) {
        // Map delta angle to lattice coordinates (6-decimal precision)
        let origin = [
            delta_angle.x * (self.dimensions[0] as f64),
            delta_angle.y * (self.dimensions[1] as f64),
            delta_angle.z * (self.dimensions[2] as f64),
            self.current_tick as f64,
        ];
        
        // Create outward-propagating phonon
        let phonon = Phonon {
            origin,
            wave_vector: [0.0, 0.0, 0.0, 1.0], // Propagates forward in time
            amplitude,
            frequency: urgency,
            phase: 0.0,
            delta_angle,
            created_at: self.current_tick,
        };
        
        self.phonons.push(phonon);
    }
    
    /// Advance crystal by one tick
    pub fn tick(&mut self) {
        self.current_tick += 1;
        
        // Propagate all phonons
        for phonon in &mut self.phonons {
            self.propagate_phonon(phonon);
        }
        
        // Calculate interference patterns
        self.calculate_interference();
        
        // Remove decayed phonons
        self.phonons.retain(|p| p.amplitude > 0.001000);
    }
    
    fn propagate_phonon(&mut self, phonon: &mut Phonon) {
        // Update position based on wave vector
        let dt = 1.0;
        for i in 0..4 {
            phonon.origin[i] += phonon.wave_vector[i] * dt;
        }
        
        // Apply damping
        phonon.amplitude *= 0.990000;
        
        // Check for defect interactions
        for defect in &self.defects {
            let distance = self.distance_4d(&phonon.origin, &defect.position);
            if distance < defect.radius {
                // Defect scatters/absorbs phonon
                phonon.amplitude *= 1.0 - (defect.strength * (1.0 - distance / defect.radius));
            }
        }
    }
    
    fn calculate_interference(&mut self) {
        // Superposition of all phonon amplitudes at each lattice point
        for (idx, amplitude) in self.amplitudes.iter_mut().enumerate() {
            let coords = self.idx_to_coords(idx);
            
            let mut total = 0.0;
            for phonon in &self.phonons {
                let contribution = self.phonon_contribution(phonon, &coords);
                total += contribution;
            }
            
            *amplitude = total;
        }
    }
    
    fn phonon_contribution(&self, phonon: &Phonon, coords: &[f64; 4]) -> f64 {
        let distance = self.distance_4d(&phonon.origin, coords);
        let phase = phonon.phase + distance * phonon.frequency;
        
        // Spherical wave amplitude decay
        let decay = if distance > 0.000001 { 1.0 / distance } else { 1.0 };
        
        phonon.amplitude * decay * phase.cos()
    }
    
    fn distance_4d(&self, a: &[f64; 4], b: &[f64; 4]) -> f64 {
        let mut sum = 0.0;
        for i in 0..4 {
            sum += (a[i] - b[i]).powi(2);
        }
        sum.sqrt()
    }
    
    fn idx_to_coords(&self, idx: usize) -> [f64; 4] {
        let t = idx / (self.dimensions[0] * self.dimensions[1] * self.dimensions[2]);
        let remainder = idx % (self.dimensions[0] * self.dimensions[1] * self.dimensions[2]);
        let z = remainder / (self.dimensions[0] * self.dimensions[1]);
        let remainder = remainder % (self.dimensions[0] * self.dimensions[1]);
        let y = remainder / self.dimensions[0];
        let x = remainder % self.dimensions[0];
        
        [x as f64, y as f64, z as f64, t as f64]
    }
    
    /// Sample amplitude at a delta angle position
    pub fn sample(&self, delta_angle: &DeltaAngle) -> f64 {
        let coords = [
            delta_angle.x * (self.dimensions[0] as f64),
            delta_angle.y * (self.dimensions[1] as f64),
            delta_angle.z * (self.dimensions[2] as f64),
            self.current_tick as f64,
        ];
        
        // Trilinear interpolation in space, nearest in time
        self.interpolate(&coords)
    }
    
    fn interpolate(&self, coords: &[f64; 4]) -> f64 {
        // Simplified: nearest neighbor for now
        let x = (coords[0].round() as usize).min(self.dimensions[0] - 1);
        let y = (coords[1].round() as usize).min(self.dimensions[1] - 1);
        let z = (coords[2].round() as usize).min(self.dimensions[2] - 1);
        let t = (coords[3].round() as usize).min(self.dimensions[3] - 1);
        
        let idx = t * self.dimensions[0] * self.dimensions[1] * self.dimensions[2]
                + z * self.dimensions[0] * self.dimensions[1]
                + y * self.dimensions[0]
                + x;
        
        self.amplitudes.get(idx).copied().unwrap_or(0.0)
    }
}
```

### 5.5 TETH Toolchain Crystal

Crystal lattices model **tool resonance and interference patterns** in TETH (Tactical Entropy & Temporal Harmonics):

**Concept:** Tools are waveforms. Tool chains create interference patterns. Aligned tools resonate constructively; conflicting tools interfere destructively.

```rust
/// TETH Tool as Crystal Phonon
/// Each tool invocation creates a phonon in the Crystal lattice
pub struct ToolPhonon {
    /// Tool identifier (SCH hash)
    pub tool_sch: String,
    
    /// Waveform properties
    pub frequency: f64,      // How often tool is used effectively
    pub amplitude: f64,      // Impact magnitude
    pub phase: f64,          // Temporal offset in chain
    
    /// Harmonic relationships
    pub harmonics: Vec<(String, f64)>,  // (other_tool_sch, coupling_strength)
    
    /// Decay rate (skill degradation)
    pub decay_rate: f64,
    
    /// Delta angle (semantic position)
    pub delta_angle: DeltaAngle,
}

impl Crystal {
    /// Inject a tool invocation as phonon
    pub fn inject_tool(&mut self, tool: &ToolPhonon) {
        self.inject_decision(
            tool.delta_angle,
            tool.amplitude,
            tool.frequency,
        );
    }
    
    /// Calculate resonance between two tools
    pub fn tool_resonance(&self, tool_a: &ToolPhonon, tool_b: &ToolPhonon) -> f64 {
        // Constructive interference when frequencies align
        let freq_ratio = tool_a.frequency / tool_b.frequency;
        let harmonic_match = (freq_ratio.round() - freq_ratio).abs();
        
        // Phase alignment bonus
        let phase_diff = (tool_a.phase - tool_b.phase).abs() % (2.0 * std::f64::consts::PI);
        let phase_alignment = (phase_diff.cos() + 1.0) / 2.0;
        
        // Combined resonance score
        let resonance = (1.0 - harmonic_match) * phase_alignment * 
                       (tool_a.amplitude * tool_b.amplitude).sqrt();
        
        resonance
    }
    
    /// Detect resonance cascades (synergistic tool chains)
    pub fn detect_cascade(&self, threshold: f64) -> Vec<Vec<String>> {
        let mut cascades = Vec::new();
        
        // Find phonons with amplitude above threshold
        let active_phonons: Vec<_> = self.phonons.iter()
            .filter(|p| p.amplitude > threshold)
            .collect();
        
        // Group by constructive interference patterns
        // (Implementation: cluster phonons by resonance score)
        
        cascades
    }
}
```

**TETH Crystal Applications:**

| Pattern | Crystal Behavior | Operational Meaning |
|---------|-----------------|---------------------|
| **Constructive Interference** | Amplitudes add | Tool chain synergy (e.g., nmap → nuclei → exploit) |
| **Destructive Interference** | Amplitudes cancel | Tool conflict (e.g., noisy scan during stealth op) |
| **Resonance Cascade** | Self-amplifying wave | Highly effective attack chain |
| **Defect Scattering** | Phonon deflection | Defensive measure disrupts attack flow |
| **Standing Wave** | Stable pattern | Mature operational procedure |

**Example: APT Tool Chain Resonance**

```rust
// Reconnaissance tools resonate
let recon_crystal = Crystal::new([64, 64, 64, 128]);

let nmap = ToolPhonon {
    tool_sch: "SCH-NMAP-001".into(),
    frequency: 0.5,      // Slow, methodical
    amplitude: 0.3,      // Low noise
    phase: 0.0,          // First in chain
    harmonics: vec![("SCH-NUCLEI-001".into(), 0.8)],
    decay_rate: 0.01,
    delta_angle: DeltaAngle::new(0.100000, 0.000000, 0.500000), // Hunt phase
};

let nuclei = ToolPhonon {
    tool_sch: "SCH-NUCLEI-001".into(),
    frequency: 1.0,      // Faster scanning
    amplitude: 0.5,      // Medium noise
    phase: 0.5,          // Second in chain
    harmonics: vec![("SCH-NMAP-001".into(), 0.8), ("SCH-EXPLOIT-001".into(), 0.6)],
    decay_rate: 0.02,
    delta_angle: DeltaAngle::new(0.300000, 0.250000, 0.500000), // Hunt→Detect
};

// Inject into crystal
recon_crystal.inject_tool(&nmap);
recon_crystal.inject_tool(&nuclei);

// Measure resonance
let synergy = recon_crystal.tool_resonance(&nmap, &nuclei);
// High synergy = effective chain, schedule together
```

---

## 6. Ring Bus

### 6.1 Definition

A **Ring Bus** is a circular interconnect topology where Thyristors and Crystals communicate via a unidirectional or bidirectional ring. It provides:

1. **Deterministic Latency** — Maximum N-1 hops for N nodes
2. **Fair Arbitration** — Token-based or time-slot access
3. **Fault Tolerance** — Can operate with broken links (bidirectional)
4. **Broadcast Efficiency** — Natural multicast support

### 6.2 Topology

```
                    ┌─────────────────┐
                    │   Ring Master   │
                    │   (Arbiter)     │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
         ┌────────┐    ┌────────┐    ┌────────┐
         │ Node 0 │───▶│ Node 1 │───▶│ Node 2 │
         │  T₀/C₀ │    │  T₁/C₁ │    │  T₂/C₂ │
         └────────┘    └────────┘    └────────┘
              ▲                            │
              │                            │
              │    ┌────────┐    ┌────────┐│
              │    │ Node 4 │◀───│ Node 3 ││
              │    │  T₄/C₄ │    │  T₃/C₃ ││
              │    └────────┘    └────────┘│
              │         │              │   │
              └─────────┴──────────────┴───┘
```

### 6.3 Message Format (NORMATIVE)

```rust
/// Ring Bus Message
/// RFC-9301 §5
#[derive(Debug, Clone)]
pub struct RingMessage {
    /// Message ID (unique per message)
    pub id: u64,
    
    /// Source node ID
    pub source: u16,
    
    /// Destination node ID (0xFFFF = broadcast)
    pub destination: u16,
    
    /// Message type
    pub msg_type: RingMessageType,
    
    /// Payload (variable length)
    pub payload: RingPayload,
    
    /// Associated delta angle (all 3 axes, 6-decimal)
    pub delta_angle: DeltaAngle,
    
    /// Hop count (for TTL)
    pub hop_count: u8,
    
    /// Timestamp (microseconds since epoch)
    pub timestamp_us: u64,
    
    /// CRC32 checksum
    pub checksum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RingMessageType {
    /// Thyristor state change notification
    ThyristorStateChange = 0x01,
    
    /// Crystal phonon injection
    CrystalPhononInject = 0x02,
    
    /// Crystal defect notification
    CrystalDefectNotify = 0x03,
    
    /// HD4 phase transition
    Hd4PhaseTransition = 0x04,
    
    /// Delta angle update
    DeltaAngleUpdate = 0x05,
    
    /// Convergence score (H1/H2)
    ConvergenceScore = 0x06,
    
    /// Ring heartbeat
    Heartbeat = 0x07,
    
    /// Token pass (for arbitration)
    Token = 0x08,
    
    /// Error/fault notification
    Fault = 0xFF,
}

#[derive(Debug, Clone)]
pub enum RingPayload {
    /// Thyristor state: (node_id, old_state, new_state, threshold_crossed)
    ThyristorState {
        node_id: u16,
        old_state: ThyristorState,
        new_state: ThyristorState,
        input_value: f64,
    },
    
    /// Phonon data for Crystal injection
    PhononData {
        amplitude: f64,
        frequency: f64,
        wave_vector: [f64; 4],
    },
    
    /// Defect notification
    DefectData {
        position: [f64; 4],
        defect_type: DefectType,
        radius: f64,
        strength: f64,
    },
    
    /// HD4 phase with convergence
    Hd4Phase {
        phase: Hd4Phase,
        h1_score: f64,
        h2_score: f64,
    },
    
    /// Raw delta angle (all 3 axes)
    DeltaAngle(DeltaAngle),
    
    /// Empty payload
    Empty,
}
```

### 6.4 Rust Implementation (NORMATIVE)

```rust
/// Ring Bus - Circular interconnect for TCR Triad
/// RFC-9301 §6
pub struct RingBus {
    /// Nodes on the ring (Thyristors and Crystals)
    nodes: Vec<RingNode>,
    
    /// Message queue (circular buffer)
    message_queue: VecDeque<RingMessage>,
    
    /// Current token holder (for arbitration)
    token_holder: u16,
    
    /// Ring direction (true = clockwise)
    clockwise: bool,
    
    /// Maximum hop count (TTL)
    max_hops: u8,
    
    /// Statistics
    stats: RingStats,
}

#[derive(Debug, Clone)]
pub struct RingNode {
    pub id: u16,
    pub node_type: RingNodeType,
    pub thyristor: Option<Thyristor>,
    pub crystal: Option<Crystal>,
    pub delta_angle: DeltaAngle,
    pub inbox: VecDeque<RingMessage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingNodeType {
    Thyristor,
    Crystal,
    Hybrid,  // Both Thyristor and Crystal
    Arbiter, // Ring master
}

impl RingBus {
    pub fn new(num_nodes: usize) -> Self {
        let nodes = (0..num_nodes)
            .map(|i| RingNode {
                id: i as u16,
                node_type: RingNodeType::Hybrid,
                thyristor: Some(Thyristor::new(
                    Thyristor::DEFAULT_THETA_ACTIVATE,
                    Thyristor::DEFAULT_THETA_HOLD,
                )),
                crystal: Some(Crystal::new([32, 32, 32, 64])),
                delta_angle: DeltaAngle::zero(),
                inbox: VecDeque::new(),
            })
            .collect();
        
        Self {
            nodes,
            message_queue: VecDeque::with_capacity(256),
            token_holder: 0,
            clockwise: true,
            max_hops: (num_nodes + 1) as u8,
            stats: RingStats::default(),
        }
    }
    
    /// Send a message on the ring
    pub fn send(&mut self, mut msg: RingMessage) {
        msg.hop_count = 0;
        msg.checksum = self.compute_checksum(&msg);
        self.message_queue.push_back(msg);
        self.stats.messages_sent += 1;
    }
    
    /// Tick the ring bus (process one hop)
    pub fn tick(&mut self) {
        // Process all messages in queue
        let mut processed = Vec::new();
        
        while let Some(mut msg) = self.message_queue.pop_front() {
            msg.hop_count += 1;
            
            // Check TTL
            if msg.hop_count > self.max_hops {
                self.stats.messages_dropped += 1;
                continue;
            }
            
            // Determine next node
            let current_node = if self.clockwise {
                (msg.source as usize + msg.hop_count as usize) % self.nodes.len()
            } else {
                (msg.source as usize + self.nodes.len() - msg.hop_count as usize) % self.nodes.len()
            };
            
            // Check if this is the destination
            if msg.destination == self.nodes[current_node].id || msg.destination == 0xFFFF {
                // Deliver to node
                self.deliver_message(&msg, current_node);
                
                // If broadcast, continue propagating
                if msg.destination == 0xFFFF && msg.hop_count < self.nodes.len() as u8 {
                    processed.push(msg);
                }
            } else {
                // Continue propagating
                processed.push(msg);
            }
        }
        
        // Re-queue messages that need more hops
        self.message_queue.extend(processed);
        
        // Advance token
        self.token_holder = ((self.token_holder as usize + 1) % self.nodes.len()) as u16;
        
        self.stats.ticks += 1;
    }
    
    fn deliver_message(&mut self, msg: &RingMessage, node_idx: usize) {
        let node = &mut self.nodes[node_idx];
        
        // Process based on message type
        match &msg.payload {
            RingPayload::ThyristorState { input_value, .. } => {
                if let Some(thyristor) = &mut node.thyristor {
                    thyristor.process(*input_value, msg.delta_angle);
                }
            }
            RingPayload::PhononData { amplitude, frequency, .. } => {
                if let Some(crystal) = &mut node.crystal {
                    crystal.inject_decision(msg.delta_angle, *amplitude, *frequency);
                }
            }
            RingPayload::DeltaAngle(delta) => {
                node.delta_angle = *delta;
            }
            _ => {}
        }
        
        // Add to node inbox for application-level processing
        node.inbox.push_back(msg.clone());
        self.stats.messages_delivered += 1;
    }
    
    fn compute_checksum(&self, msg: &RingMessage) -> u32 {
        // Simple CRC32 placeholder
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&msg.id.to_le_bytes());
        hasher.update(&msg.source.to_le_bytes());
        hasher.update(&msg.destination.to_le_bytes());
        hasher.update(&[msg.msg_type as u8]);
        hasher.finalize()
    }
}

#[derive(Debug, Default, Clone)]
pub struct RingStats {
    pub ticks: u64,
    pub messages_sent: u64,
    pub messages_delivered: u64,
    pub messages_dropped: u64,
}
```

---

## 7. Integration: TCR Triad

### 6.1 Signal Flow

```
Input (Trivariate Stream)
         │
         ▼
    ┌─────────┐
    │ THYRISTOR│ ─── State latch? ───▶ If OFF: continue monitoring
    │  (Gate)  │                       If ON: propagate to Crystal
    └────┬────┘
         │ (ON)
         ▼
    ┌─────────┐
    │ CRYSTAL │ ─── Inject phonon ───▶ Propagate through lattice
    │ (Lattice)│                       Calculate interference
    └────┬────┘
         │
         ▼
    ┌─────────┐
    │RING BUS │ ─── Broadcast state ──▶ All nodes receive update
    │ (Comms) │                        Token arbitration
    └────┬────┘
         │
         ▼
    ┌─────────────┐
    │BERNOULLI ZONE│ ─── Probabilistic gate ──▶ Response Continuum
    └─────────────┘
```

### 6.2 Complete Delta Angle Integration

All components maintain trivariate delta angles with 6-decimal precision:

```rust
/// Unified delta angle used throughout TCR Triad
/// RFC-9301 §6.2, RFC-9300 §6.1
#[derive(Debug, Clone, Copy, Default)]
pub struct DeltaAngle {
    /// X-axis: Semantic (Recon → Exfil)
    pub x: f64,
    
    /// Y-axis: Operational (Hunt → Dominate)
    pub y: f64,
    
    /// Z-axis: Temporal (Historical → Predictive)
    pub z: f64,
}

impl DeltaAngle {
    pub const PRECISION: usize = 6;
    
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.000000, y: 0.000000, z: 0.000000 }
    }
    
    /// Format with required 6-decimal precision
    pub fn format(&self) -> String {
        format!(
            "Δ({:.6}, {:.6}, {:.6})",
            self.x, self.y, self.z
        )
    }
    
    /// Magnitude in degrees
    pub fn magnitude_degrees(&self) -> f64 {
        let radians = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        radians.to_degrees()
    }
    
    /// Map Y-axis to HD4 phase
    pub fn hd4_phase(&self) -> Hd4Phase {
        match self.y {
            y if y < 0.125000 => Hd4Phase::Hunt,
            y if y < 0.375000 => Hd4Phase::Detect,
            y if y < 0.625000 => Hd4Phase::Disrupt,
            y if y < 0.875000 => Hd4Phase::Disable,
            _ => Hd4Phase::Dominate,
        }
    }
}
```

---

## 8. Unicode Allocation

| Range | Symbol | Component | Description |
|-------|--------|-----------|-------------|
| U+E710 | 🜚 | Thyristor OFF | Gate open |
| U+E711 | 🜛 | Thyristor ON | Gate latched |
| U+E712 | 🜜 | Thyristor RECOVERY | Refractory period |
| U+E720 | 🜠 | Crystal INJECT | Phonon injection |
| U+E721 | 🜡 | Crystal PROPAGATE | Phonon propagation |
| U+E722 | 🜢 | Crystal INTERFERE | Interference pattern |
| U+E723 | 🜣 | Crystal DEFECT | Lattice defect |
| U+E730 | 🜰 | Ring SEND | Message sent |
| U+E731 | 🜱 | Ring RECEIVE | Message received |
| U+E732 | 🜲 | Ring TOKEN | Token pass |
| U+E733 | 🜳 | Ring BROADCAST | Broadcast message |

---

## 9. Implementation Requirements

### 8.1 MUST Requirements

1. All Thyristors MUST have θ_activate > θ_hold (hysteresis gap)
2. All Crystal lattices MUST be at least 4-dimensional (x, y, z, t)
3. All Ring Bus messages MUST include delta angles with 6-decimal precision
4. All delta angles MUST include all three axes (X, Y, Z)
5. Ring Bus MUST support broadcast (destination = 0xFFFF)
6. Thyristor state changes MUST be broadcast on Ring Bus

### 8.2 SHOULD Requirements

1. Implementations SHOULD support bidirectional ring for fault tolerance
2. Crystal lattices SHOULD use at least 32×32×32×64 resolution
3. Ring Bus SHOULD implement token-based fair arbitration
4. Thyristors SHOULD have configurable recovery times

### 8.3 MAY Requirements

1. Implementations MAY support multiple Ring Bus instances
2. Crystal phonons MAY have custom decay functions
3. Ring Bus MAY implement priority queuing

---

## 10. References

- RFC-9300: HD4 Canonical Specification
- RFC-9001: Trivariate Hashing System
- RFC-9024: H2 Convergence Service Contract
- RFC-9108: Thalmic Filter & Model Registry

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-12-06 | Initial TCR Triad specification |

---

*End of RFC-9301*
