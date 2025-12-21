# RFC-9303: Crystal Realm Tunings & Unified Machine Kinematics

**Version:** 0.1.0  
**Status:** DRAFT  
**Date:** 2025-12-06  
**Author:** CTAS Architecture Team  
**Depends-On:** RFC-9301 (TCR), RFC-9302 (Nonagon)  

---

## Abstract

This RFC defines:

1. **Nine Realms** — Domain-specific Crystal tunings for the SX9 architecture
2. **Unified Machine Kinematics** — X, Y, Z motion control for physical systems

The Nine Realms map the Nonagon's 9 vertices to operational domains, each with its own Crystal lattice tuning parameters. Unified motion provides deterministic kinematics for robotics, drones, industrial systems, and autonomous platforms.

---

## 1. The Nine Realms

### 1.1 Realm Mapping

Drawing from the SX9 constant, the Nine Realms represent distinct operational domains:

```
                         ┌─────────────┐
                         │   REALM 0   │
                         │   AETHER    │
                         │  (Command)  │
                         └──────┬──────┘
                                │
              ┌─────────────────┼─────────────────┐
              │                 │                 │
       ┌──────┴──────┐   ┌──────┴──────┐   ┌──────┴──────┐
       │   REALM 1   │   │   REALM 2   │   │   REALM 3   │
       │    CYBER    │   │   KINETIC   │   │  COGNITIVE  │
       │  (Digital)  │   │  (Physical) │   │   (Mental)  │
       └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
              │                 │                 │
       ┌──────┴──────┐   ┌──────┴──────┐   ┌──────┴──────┐
       │   REALM 4   │   │   REALM 5   │   │   REALM 6   │
       │   ORBITAL   │   │  MARITIME   │   │ SUBTERRANEAN│
       │   (Space)   │   │   (Naval)   │   │(Underground)│
       └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
              │                 │                 │
              └─────────────────┼─────────────────┘
                                │
                         ┌──────┴──────┐
                         │   REALM 7   │   ┌──────────────┐
                         │  SPECTRUM   │───│   REALM 8    │
                         │    (EMS)    │   │   TEMPORAL   │
                         └─────────────┘   │   (Time)     │
                                           └──────────────┘
```

### 1.2 Realm Definitions

| Realm | Index | Domain | Crystal Tuning Focus |
|-------|-------|--------|---------------------|
| **AETHER** | 0 | Command & Control | Decision propagation, authority chains |
| **CYBER** | 1 | Digital/Network | Packet flow, intrusion waves, malware propagation |
| **KINETIC** | 2 | Physical/Mechanical | Force vectors, impact dynamics, ballistics |
| **COGNITIVE** | 3 | Mental/Psychological | Belief propagation, influence cascades |
| **ORBITAL** | 4 | Space Systems | Orbital mechanics, constellation dynamics |
| **MARITIME** | 5 | Naval/Underwater | Fluid dynamics, sonar propagation, currents |
| **SUBTERRANEAN** | 6 | Underground/Tunnel | Seismic waves, tunnel networks, mining |
| **SPECTRUM** | 7 | Electromagnetic | RF propagation, jamming, signal interference |
| **TEMPORAL** | 8 | Time-Domain | Scheduling, sequencing, predictive timing |

### 1.3 Realm-Nonagon Vertex Mapping

```rust
/// Nine Realms mapped to Nonagon vertices
/// RFC-9303 §1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Realm {
    Aether      = 0,  // A₀: Command & Control
    Cyber       = 1,  // A₁: Digital/Network
    Kinetic     = 2,  // A₂: Physical/Mechanical
    Cognitive   = 3,  // A₃: Mental/Psychological
    Orbital     = 4,  // A₄: Space Systems
    Maritime    = 5,  // A₅: Naval/Underwater
    Subterranean = 6, // A₆: Underground/Tunnel
    Spectrum    = 7,  // A₇: Electromagnetic
    Temporal    = 8,  // A₈: Time-Domain
}

impl Realm {
    /// Get all realms
    pub fn all() -> [Realm; 9] {
        [
            Realm::Aether, Realm::Cyber, Realm::Kinetic,
            Realm::Cognitive, Realm::Orbital, Realm::Maritime,
            Realm::Subterranean, Realm::Spectrum, Realm::Temporal,
        ]
    }
    
    /// Nonagon vertex index
    pub fn vertex_index(&self) -> usize {
        *self as usize
    }
    
    /// Angular position on nonagon (degrees)
    pub fn angle_degrees(&self) -> f64 {
        (*self as usize) as f64 * 40.0
    }
    
    /// Unit circle position
    pub fn position(&self) -> (f64, f64) {
        let angle = self.angle_degrees().to_radians();
        (angle.cos(), angle.sin())
    }
    
    /// Trivariate group (α=0-2, β=3-5, γ=6-8)
    pub fn trivariate_group(&self) -> char {
        match *self as u8 {
            0..=2 => 'α',
            3..=5 => 'β',
            6..=8 => 'γ',
            _ => unreachable!(),
        }
    }
}
```

---

## 2. Crystal Realm Tunings

### 2.1 Tuning Parameters

Each realm has specific Crystal lattice parameters optimized for its domain physics:

```rust
/// Crystal tuning parameters for a specific realm
/// RFC-9303 §2
#[derive(Debug, Clone)]
pub struct RealmTuning {
    /// Which realm this tuning applies to
    pub realm: Realm,
    
    /// Lattice dimensions [x, y, z, t]
    pub dimensions: [usize; 4],
    
    /// Phonon propagation speed (lattice units per tick)
    pub propagation_speed: f64,
    
    /// Damping coefficient (amplitude decay per tick)
    pub damping: f64,
    
    /// Dispersion relation parameters
    pub dispersion: DispersionParams,
    
    /// Boundary conditions
    pub boundary: BoundaryCondition,
    
    /// Coupling strength to adjacent realms
    pub realm_coupling: [f64; 9],
    
    /// Natural frequency (resonance)
    pub natural_frequency: f64,
    
    /// Anisotropy factors [x, y, z] (1.0 = isotropic)
    pub anisotropy: [f64; 3],
}

#[derive(Debug, Clone)]
pub struct DispersionParams {
    /// Linear coefficient
    pub linear: f64,
    /// Quadratic coefficient (dispersion strength)
    pub quadratic: f64,
    /// Cutoff frequency
    pub cutoff: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryCondition {
    /// Waves reflect at boundary
    Reflective,
    /// Waves pass through (infinite domain)
    Absorbing,
    /// Waves wrap around (toroidal)
    Periodic,
    /// Fixed boundary (zero amplitude)
    Fixed,
}
```

### 2.2 Default Realm Tunings

```rust
impl RealmTuning {
    /// AETHER: Command propagation - fast, low damping, isotropic
    pub fn aether() -> Self {
        Self {
            realm: Realm::Aether,
            dimensions: [64, 64, 64, 256],
            propagation_speed: 10.0,
            damping: 0.001,
            dispersion: DispersionParams {
                linear: 1.0,
                quadratic: 0.0,
                cutoff: 100.0,
            },
            boundary: BoundaryCondition::Absorbing,
            realm_coupling: [0.0, 0.8, 0.8, 0.8, 0.5, 0.5, 0.5, 0.7, 0.9],
            natural_frequency: 1.0,
            anisotropy: [1.0, 1.0, 1.0],
        }
    }
    
    /// CYBER: Network propagation - discrete, hop-based
    pub fn cyber() -> Self {
        Self {
            realm: Realm::Cyber,
            dimensions: [128, 128, 32, 512],
            propagation_speed: 5.0,
            damping: 0.01,
            dispersion: DispersionParams {
                linear: 1.0,
                quadratic: 0.1,
                cutoff: 50.0,
            },
            boundary: BoundaryCondition::Periodic, // Networks wrap
            realm_coupling: [0.8, 0.0, 0.3, 0.6, 0.4, 0.2, 0.1, 0.7, 0.5],
            natural_frequency: 2.5,
            anisotropy: [1.0, 1.0, 0.5], // Flatter in Z (logical layers)
        }
    }
    
    /// KINETIC: Physical propagation - inertial, momentum-preserving
    pub fn kinetic() -> Self {
        Self {
            realm: Realm::Kinetic,
            dimensions: [256, 256, 256, 128],
            propagation_speed: 1.0, // Speed of sound/impact
            damping: 0.05,
            dispersion: DispersionParams {
                linear: 0.8,
                quadratic: 0.2,
                cutoff: 20.0,
            },
            boundary: BoundaryCondition::Reflective,
            realm_coupling: [0.8, 0.3, 0.0, 0.2, 0.6, 0.7, 0.8, 0.4, 0.3],
            natural_frequency: 0.5,
            anisotropy: [1.0, 1.0, 0.8], // Slightly less vertical
        }
    }
    
    /// COGNITIVE: Belief propagation - social network dynamics
    pub fn cognitive() -> Self {
        Self {
            realm: Realm::Cognitive,
            dimensions: [64, 64, 64, 1024],
            propagation_speed: 0.5, // Slow spread
            damping: 0.005, // Beliefs persist
            dispersion: DispersionParams {
                linear: 0.5,
                quadratic: 0.5, // High dispersion (beliefs diffuse)
                cutoff: 10.0,
            },
            boundary: BoundaryCondition::Absorbing,
            realm_coupling: [0.8, 0.6, 0.2, 0.0, 0.3, 0.3, 0.2, 0.5, 0.7],
            natural_frequency: 0.1,
            anisotropy: [1.0, 1.2, 0.8], // Social hierarchies
        }
    }
    
    /// ORBITAL: Space dynamics - Keplerian mechanics
    pub fn orbital() -> Self {
        Self {
            realm: Realm::Orbital,
            dimensions: [512, 512, 512, 64],
            propagation_speed: 8.0, // Fast orbital mechanics
            damping: 0.0001, // Near-vacuum
            dispersion: DispersionParams {
                linear: 1.0,
                quadratic: 0.01,
                cutoff: 1000.0,
            },
            boundary: BoundaryCondition::Periodic, // Orbits wrap
            realm_coupling: [0.5, 0.4, 0.6, 0.3, 0.0, 0.2, 0.1, 0.8, 0.6],
            natural_frequency: 0.05, // Orbital periods
            anisotropy: [1.0, 1.0, 1.0], // Isotropic in space
        }
    }
    
    /// MARITIME: Fluid dynamics - wave propagation
    pub fn maritime() -> Self {
        Self {
            realm: Realm::Maritime,
            dimensions: [256, 256, 64, 256],
            propagation_speed: 1.5, // Speed of sound in water
            damping: 0.02,
            dispersion: DispersionParams {
                linear: 0.9,
                quadratic: 0.3,
                cutoff: 30.0,
            },
            boundary: BoundaryCondition::Reflective, // Coastlines reflect
            realm_coupling: [0.5, 0.2, 0.7, 0.3, 0.2, 0.0, 0.4, 0.3, 0.4],
            natural_frequency: 0.3,
            anisotropy: [1.0, 1.0, 0.3], // Very flat (ocean depth << area)
        }
    }
    
    /// SUBTERRANEAN: Seismic propagation - layered media
    pub fn subterranean() -> Self {
        Self {
            realm: Realm::Subterranean,
            dimensions: [128, 128, 256, 128],
            propagation_speed: 2.0, // Seismic velocity
            damping: 0.03,
            dispersion: DispersionParams {
                linear: 0.7,
                quadratic: 0.4,
                cutoff: 15.0,
            },
            boundary: BoundaryCondition::Reflective, // Layer boundaries
            realm_coupling: [0.5, 0.1, 0.8, 0.2, 0.1, 0.4, 0.0, 0.2, 0.3],
            natural_frequency: 0.2,
            anisotropy: [0.8, 0.8, 1.5], // Vertical propagation emphasized
        }
    }
    
    /// SPECTRUM: EM propagation - speed of light
    pub fn spectrum() -> Self {
        Self {
            realm: Realm::Spectrum,
            dimensions: [256, 256, 256, 64],
            propagation_speed: 100.0, // Fastest realm
            damping: 0.001,
            dispersion: DispersionParams {
                linear: 1.0,
                quadratic: 0.0, // No dispersion in vacuum
                cutoff: 10000.0,
            },
            boundary: BoundaryCondition::Absorbing,
            realm_coupling: [0.7, 0.7, 0.4, 0.5, 0.8, 0.3, 0.2, 0.0, 0.5],
            natural_frequency: 10.0,
            anisotropy: [1.0, 1.0, 1.0], // Isotropic
        }
    }
    
    /// TEMPORAL: Time-domain scheduling
    pub fn temporal() -> Self {
        Self {
            realm: Realm::Temporal,
            dimensions: [32, 32, 32, 2048],
            propagation_speed: 1.0, // Time flows at time-speed
            damping: 0.0,
            dispersion: DispersionParams {
                linear: 1.0,
                quadratic: 0.0,
                cutoff: f64::INFINITY,
            },
            boundary: BoundaryCondition::Absorbing, // Past is fixed
            realm_coupling: [0.9, 0.5, 0.3, 0.7, 0.6, 0.4, 0.3, 0.5, 0.0],
            natural_frequency: 1.0,
            anisotropy: [0.1, 0.1, 0.1], // Almost 1D (time axis dominates)
        }
    }
    
    /// Get tuning for a realm
    pub fn for_realm(realm: Realm) -> Self {
        match realm {
            Realm::Aether => Self::aether(),
            Realm::Cyber => Self::cyber(),
            Realm::Kinetic => Self::kinetic(),
            Realm::Cognitive => Self::cognitive(),
            Realm::Orbital => Self::orbital(),
            Realm::Maritime => Self::maritime(),
            Realm::Subterranean => Self::subterranean(),
            Realm::Spectrum => Self::spectrum(),
            Realm::Temporal => Self::temporal(),
        }
    }
}
```

### 2.3 Tuned Crystal Implementation

```rust
/// Crystal tuned for a specific realm
pub struct TunedCrystal {
    /// Base crystal lattice
    pub crystal: Crystal,
    
    /// Realm tuning parameters
    pub tuning: RealmTuning,
    
    /// Cross-realm phonon buffer (for inter-realm propagation)
    pub cross_realm_buffer: Vec<(Realm, Phonon)>,
}

impl TunedCrystal {
    pub fn new(realm: Realm) -> Self {
        let tuning = RealmTuning::for_realm(realm);
        let crystal = Crystal::new(tuning.dimensions);
        
        Self {
            crystal,
            tuning,
            cross_realm_buffer: Vec::new(),
        }
    }
    
    /// Inject decision with realm-specific propagation
    pub fn inject(&mut self, delta_angle: DeltaAngle, amplitude: f64, urgency: f64) {
        // Apply anisotropy to amplitude
        let adjusted_amplitude = amplitude * self.tuning.propagation_speed;
        
        // Inject into crystal
        self.crystal.inject_decision(delta_angle, adjusted_amplitude, urgency);
    }
    
    /// Tick with realm-specific physics
    pub fn tick(&mut self) {
        // Apply realm-specific damping
        for phonon in &mut self.crystal.phonons {
            phonon.amplitude *= 1.0 - self.tuning.damping;
            
            // Apply dispersion
            let freq = phonon.frequency;
            let speed = self.tuning.dispersion.linear 
                + self.tuning.dispersion.quadratic * freq * freq;
            
            // Clamp to cutoff
            let effective_speed = if freq < self.tuning.dispersion.cutoff {
                speed * self.tuning.propagation_speed
            } else {
                0.0 // Above cutoff, no propagation
            };
            
            // Apply anisotropic propagation
            phonon.wave_vector[0] *= self.tuning.anisotropy[0] * effective_speed;
            phonon.wave_vector[1] *= self.tuning.anisotropy[1] * effective_speed;
            phonon.wave_vector[2] *= self.tuning.anisotropy[2] * effective_speed;
        }
        
        // Standard crystal tick
        self.crystal.tick();
        
        // Check for cross-realm coupling
        self.check_cross_realm_coupling();
    }
    
    fn check_cross_realm_coupling(&mut self) {
        // Phonons near boundary can couple to other realms
        for phonon in &self.crystal.phonons {
            if phonon.amplitude > 0.1 {
                for (other_realm_idx, &coupling) in self.tuning.realm_coupling.iter().enumerate() {
                    if coupling > 0.0 && other_realm_idx != self.tuning.realm as usize {
                        let other_realm = Realm::all()[other_realm_idx];
                        let coupled_phonon = Phonon {
                            amplitude: phonon.amplitude * coupling,
                            ..phonon.clone()
                        };
                        self.cross_realm_buffer.push((other_realm, coupled_phonon));
                    }
                }
            }
        }
    }
    
    /// Drain cross-realm phonons for injection into other realm crystals
    pub fn drain_cross_realm(&mut self) -> Vec<(Realm, Phonon)> {
        std::mem::take(&mut self.cross_realm_buffer)
    }
}
```

---

## 3. Unified Machine Kinematics

### 3.1 Motion Space Definition

Unified X, Y, Z motion for all physical systems:

```rust
/// 3D position with 6-decimal precision
/// RFC-9303 §3
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 3D velocity
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 3D acceleration
#[derive(Debug, Clone, Copy, Default)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Orientation (Euler angles or quaternion)
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Euler { roll: f64, pitch: f64, yaw: f64 },
    Quaternion { w: f64, x: f64, y: f64, z: f64 },
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Euler { roll: 0.0, pitch: 0.0, yaw: 0.0 }
    }
}

/// Complete kinematic state
#[derive(Debug, Clone, Default)]
pub struct KinematicState {
    /// Position in world frame
    pub position: Position,
    
    /// Velocity in world frame
    pub velocity: Velocity,
    
    /// Acceleration in world frame
    pub acceleration: Acceleration,
    
    /// Orientation
    pub orientation: Orientation,
    
    /// Angular velocity (rad/s)
    pub angular_velocity: Velocity,
    
    /// Timestamp (microseconds)
    pub timestamp_us: u64,
    
    /// Reference frame ID
    pub frame_id: u32,
}
```

### 3.2 Motion Commands

```rust
/// Motion command types
#[derive(Debug, Clone)]
pub enum MotionCommand {
    /// Move to absolute position
    MoveTo {
        target: Position,
        speed: f64,
        acceleration: f64,
    },
    
    /// Move relative to current position
    MoveBy {
        delta: Position,
        speed: f64,
    },
    
    /// Follow velocity vector
    SetVelocity {
        velocity: Velocity,
        duration_ms: Option<u64>,
    },
    
    /// Follow trajectory (list of waypoints)
    FollowPath {
        waypoints: Vec<Position>,
        speeds: Vec<f64>,
    },
    
    /// Rotate to orientation
    RotateTo {
        orientation: Orientation,
        angular_speed: f64,
    },
    
    /// Stop all motion
    Stop {
        deceleration: f64,
    },
    
    /// Hold position (active station-keeping)
    Hold {
        position: Position,
        tolerance: f64,
    },
}

/// Motion command with realm context
#[derive(Debug, Clone)]
pub struct RealmMotionCommand {
    /// The motion command
    pub command: MotionCommand,
    
    /// Which realm this motion occurs in
    pub realm: Realm,
    
    /// Associated delta angle
    pub delta_angle: DeltaAngle,
    
    /// Priority (0.0 - 1.0)
    pub priority: f64,
    
    /// Command ID
    pub id: u64,
    
    /// Timestamp
    pub timestamp_us: u64,
}
```

### 3.3 Unified Motion Controller

```rust
/// Unified motion controller for machines across realms
pub struct UnifiedMotionController {
    /// Current kinematic state
    pub state: KinematicState,
    
    /// Active realm
    pub realm: Realm,
    
    /// Motion constraints
    pub constraints: MotionConstraints,
    
    /// Command queue
    pub command_queue: VecDeque<RealmMotionCommand>,
    
    /// Currently executing command
    pub active_command: Option<RealmMotionCommand>,
    
    /// Motion history (for prediction)
    pub history: VecDeque<KinematicState>,
    
    /// PID controllers for each axis
    pub pid_x: PidController,
    pub pid_y: PidController,
    pub pid_z: PidController,
}

#[derive(Debug, Clone)]
pub struct MotionConstraints {
    /// Maximum speed (units/sec)
    pub max_speed: f64,
    
    /// Maximum acceleration (units/sec²)
    pub max_acceleration: f64,
    
    /// Maximum angular velocity (rad/sec)
    pub max_angular_velocity: f64,
    
    /// Position bounds
    pub bounds_min: Position,
    pub bounds_max: Position,
    
    /// Realm-specific constraints
    pub realm_constraints: RealmConstraints,
}

#[derive(Debug, Clone)]
pub enum RealmConstraints {
    /// Cyber: hop limits, latency bounds
    Cyber { max_hops: u32, max_latency_ms: u64 },
    
    /// Kinetic: mass, force limits
    Kinetic { mass_kg: f64, max_force_n: f64 },
    
    /// Orbital: delta-v budget, orbital elements
    Orbital { delta_v_budget: f64, inclination_limit: f64 },
    
    /// Maritime: draft, sea state limits
    Maritime { max_draft_m: f64, max_sea_state: u8 },
    
    /// None/default
    Default,
}

#[derive(Debug, Clone)]
pub struct PidController {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub integral: f64,
    pub last_error: f64,
    pub output_limit: f64,
}

impl UnifiedMotionController {
    pub fn new(realm: Realm) -> Self {
        Self {
            state: KinematicState::default(),
            realm,
            constraints: MotionConstraints::default_for_realm(realm),
            command_queue: VecDeque::new(),
            active_command: None,
            history: VecDeque::with_capacity(1000),
            pid_x: PidController::default(),
            pid_y: PidController::default(),
            pid_z: PidController::default(),
        }
    }
    
    /// Queue a motion command
    pub fn queue_command(&mut self, cmd: RealmMotionCommand) {
        self.command_queue.push_back(cmd);
    }
    
    /// Update controller (called at fixed rate)
    pub fn update(&mut self, dt: f64) {
        // Save history
        self.history.push_back(self.state.clone());
        if self.history.len() > 1000 {
            self.history.pop_front();
        }
        
        // Get current command
        if self.active_command.is_none() && !self.command_queue.is_empty() {
            self.active_command = self.command_queue.pop_front();
        }
        
        // Execute active command
        if let Some(ref cmd) = self.active_command.clone() {
            let complete = self.execute_command(&cmd, dt);
            if complete {
                self.active_command = None;
            }
        }
        
        // Update timestamp
        self.state.timestamp_us += (dt * 1_000_000.0) as u64;
    }
    
    fn execute_command(&mut self, cmd: &RealmMotionCommand, dt: f64) -> bool {
        match &cmd.command {
            MotionCommand::MoveTo { target, speed, acceleration } => {
                self.move_to(target, *speed, *acceleration, dt)
            }
            MotionCommand::MoveBy { delta, speed } => {
                let target = Position {
                    x: self.state.position.x + delta.x,
                    y: self.state.position.y + delta.y,
                    z: self.state.position.z + delta.z,
                };
                self.move_to(&target, *speed, self.constraints.max_acceleration, dt)
            }
            MotionCommand::SetVelocity { velocity, .. } => {
                self.set_velocity(velocity, dt);
                false // Velocity commands don't "complete"
            }
            MotionCommand::Stop { deceleration } => {
                self.stop(*deceleration, dt)
            }
            MotionCommand::Hold { position, tolerance } => {
                self.hold_position(position, *tolerance, dt)
            }
            _ => false,
        }
    }
    
    fn move_to(&mut self, target: &Position, max_speed: f64, max_accel: f64, dt: f64) -> bool {
        let dx = target.x - self.state.position.x;
        let dy = target.y - self.state.position.y;
        let dz = target.z - self.state.position.z;
        
        let distance = (dx*dx + dy*dy + dz*dz).sqrt();
        
        if distance < 0.001 {
            return true; // Arrived
        }
        
        // Normalize direction
        let nx = dx / distance;
        let ny = dy / distance;
        let nz = dz / distance;
        
        // Calculate desired speed (trapezoidal profile)
        let current_speed = (
            self.state.velocity.x.powi(2) + 
            self.state.velocity.y.powi(2) + 
            self.state.velocity.z.powi(2)
        ).sqrt();
        
        let stopping_distance = current_speed.powi(2) / (2.0 * max_accel);
        
        let target_speed = if distance < stopping_distance {
            // Decelerate
            (current_speed - max_accel * dt).max(0.0)
        } else {
            // Accelerate up to max
            (current_speed + max_accel * dt).min(max_speed)
        };
        
        // Apply velocity
        self.state.velocity.x = nx * target_speed;
        self.state.velocity.y = ny * target_speed;
        self.state.velocity.z = nz * target_speed;
        
        // Update position
        self.state.position.x += self.state.velocity.x * dt;
        self.state.position.y += self.state.velocity.y * dt;
        self.state.position.z += self.state.velocity.z * dt;
        
        false // Not yet arrived
    }
    
    fn set_velocity(&mut self, velocity: &Velocity, dt: f64) {
        self.state.velocity = *velocity;
        self.state.position.x += velocity.x * dt;
        self.state.position.y += velocity.y * dt;
        self.state.position.z += velocity.z * dt;
    }
    
    fn stop(&mut self, deceleration: f64, dt: f64) -> bool {
        let speed = (
            self.state.velocity.x.powi(2) + 
            self.state.velocity.y.powi(2) + 
            self.state.velocity.z.powi(2)
        ).sqrt();
        
        if speed < 0.001 {
            self.state.velocity = Velocity::default();
            return true;
        }
        
        let factor = (speed - deceleration * dt).max(0.0) / speed;
        self.state.velocity.x *= factor;
        self.state.velocity.y *= factor;
        self.state.velocity.z *= factor;
        
        self.state.position.x += self.state.velocity.x * dt;
        self.state.position.y += self.state.velocity.y * dt;
        self.state.position.z += self.state.velocity.z * dt;
        
        false
    }
    
    fn hold_position(&mut self, target: &Position, tolerance: f64, dt: f64) -> bool {
        let error_x = target.x - self.state.position.x;
        let error_y = target.y - self.state.position.y;
        let error_z = target.z - self.state.position.z;
        
        // Apply PID control
        let cmd_x = self.pid_x.update(error_x, dt);
        let cmd_y = self.pid_y.update(error_y, dt);
        let cmd_z = self.pid_z.update(error_z, dt);
        
        self.state.velocity.x = cmd_x;
        self.state.velocity.y = cmd_y;
        self.state.velocity.z = cmd_z;
        
        self.state.position.x += self.state.velocity.x * dt;
        self.state.position.y += self.state.velocity.y * dt;
        self.state.position.z += self.state.velocity.z * dt;
        
        false // Hold commands never complete
    }
}

impl PidController {
    pub fn update(&mut self, error: f64, dt: f64) -> f64 {
        self.integral += error * dt;
        let derivative = (error - self.last_error) / dt;
        self.last_error = error;
        
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        output.clamp(-self.output_limit, self.output_limit)
    }
}

impl Default for PidController {
    fn default() -> Self {
        Self {
            kp: 1.0,
            ki: 0.1,
            kd: 0.05,
            integral: 0.0,
            last_error: 0.0,
            output_limit: 10.0,
        }
    }
}
```

---

## 4. Cross-Realm Motion

### 4.1 Realm Translation

Motion in one realm can trigger effects in another:

```rust
/// Cross-realm motion translator
pub struct RealmTranslator {
    /// Crystals for each realm
    pub crystals: [TunedCrystal; 9],
    
    /// Motion controllers per realm
    pub controllers: HashMap<Realm, UnifiedMotionController>,
}

impl RealmTranslator {
    pub fn new() -> Self {
        Self {
            crystals: [
                TunedCrystal::new(Realm::Aether),
                TunedCrystal::new(Realm::Cyber),
                TunedCrystal::new(Realm::Kinetic),
                TunedCrystal::new(Realm::Cognitive),
                TunedCrystal::new(Realm::Orbital),
                TunedCrystal::new(Realm::Maritime),
                TunedCrystal::new(Realm::Subterranean),
                TunedCrystal::new(Realm::Spectrum),
                TunedCrystal::new(Realm::Temporal),
            ],
            controllers: HashMap::new(),
        }
    }
    
    /// Tick all realm crystals and propagate cross-realm effects
    pub fn tick(&mut self) {
        // Tick each crystal
        for crystal in &mut self.crystals {
            crystal.tick();
        }
        
        // Collect cross-realm phonons
        let mut cross_realm_phonons: Vec<(Realm, Phonon)> = Vec::new();
        for crystal in &mut self.crystals {
            cross_realm_phonons.extend(crystal.drain_cross_realm());
        }
        
        // Inject into target realms
        for (target_realm, phonon) in cross_realm_phonons {
            self.crystals[target_realm as usize].crystal.phonons.push(phonon);
        }
    }
    
    /// Translate motion command across realms
    pub fn translate_motion(
        &self,
        cmd: &RealmMotionCommand,
        to_realm: Realm,
    ) -> Option<RealmMotionCommand> {
        let from_tuning = RealmTuning::for_realm(cmd.realm);
        let to_tuning = RealmTuning::for_realm(to_realm);
        
        // Scale motion based on realm physics
        let speed_ratio = to_tuning.propagation_speed / from_tuning.propagation_speed;
        
        let translated_command = match &cmd.command {
            MotionCommand::MoveTo { target, speed, acceleration } => {
                MotionCommand::MoveTo {
                    target: *target, // Position translation depends on realm semantics
                    speed: speed * speed_ratio,
                    acceleration: acceleration * speed_ratio,
                }
            }
            _ => return None, // Not all commands translate
        };
        
        Some(RealmMotionCommand {
            command: translated_command,
            realm: to_realm,
            delta_angle: cmd.delta_angle,
            priority: cmd.priority * from_tuning.realm_coupling[to_realm as usize],
            id: cmd.id,
            timestamp_us: cmd.timestamp_us,
        })
    }
}
```

---

## 5. Delta Angle ↔ Position Mapping

### 5.1 Trivariate to XYZ Translation

```rust
impl Position {
    /// Create from delta angle (normalized 0-1 to position space)
    pub fn from_delta_angle(delta: &DeltaAngle, scale: f64) -> Self {
        Self {
            x: delta.x * scale,
            y: delta.y * scale,
            z: delta.z * scale,
        }
    }
    
    /// Convert to delta angle (position to normalized 0-1)
    pub fn to_delta_angle(&self, scale: f64) -> DeltaAngle {
        DeltaAngle::new(
            (self.x / scale).clamp(0.0, 1.0),
            (self.y / scale).clamp(0.0, 1.0),
            (self.z / scale).clamp(0.0, 1.0),
        )
    }
}

impl DeltaAngle {
    /// Apply to position as directional offset
    pub fn apply_to_position(&self, pos: &Position, magnitude: f64) -> Position {
        Position {
            x: pos.x + self.x * magnitude,
            y: pos.y + self.y * magnitude,
            z: pos.z + self.z * magnitude,
        }
    }
}
```

---

## 6. Unicode Allocation

| Range | Symbol | Component | Description |
|-------|--------|-----------|-------------|
| U+E750 | 🝐 | REALM-0 | Aether realm |
| U+E751 | 🝑 | REALM-1 | Cyber realm |
| U+E752 | 🝒 | REALM-2 | Kinetic realm |
| U+E753 | 🝓 | REALM-3 | Cognitive realm |
| U+E754 | 🝔 | REALM-4 | Orbital realm |
| U+E755 | 🝕 | REALM-5 | Maritime realm |
| U+E756 | 🝖 | REALM-6 | Subterranean realm |
| U+E757 | 🝗 | REALM-7 | Spectrum realm |
| U+E758 | 🝘 | REALM-8 | Temporal realm |
| U+E760 | 🝠 | MOVE | Motion command |
| U+E761 | 🝡 | STOP | Stop command |
| U+E762 | 🝢 | HOLD | Hold position |
| U+E763 | 🝣 | PATH | Follow path |

---

## 7. Implementation Requirements

### 7.1 MUST Requirements

1. All realms MUST have unique tuning parameters
2. Crystal propagation MUST respect realm-specific physics
3. Motion commands MUST use 6-decimal precision
4. Cross-realm coupling MUST be bidirectional

### 7.2 SHOULD Requirements

1. Controllers SHOULD implement all motion command types
2. Realm translation SHOULD preserve command semantics
3. PID parameters SHOULD be tunable per realm

---

## 8. References

- RFC-9301: Thyristor, Crystal, and Ring Bus
- RFC-9302: Nonagon Analytic Node

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2025-12-06 | Initial draft |

---

*End of RFC-9303*
