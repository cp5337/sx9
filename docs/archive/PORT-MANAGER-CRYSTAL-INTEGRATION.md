# Port Manager + Crystal/Thyristor Integration

**Status:** Design Specification  
**Date:** December 2025  
**Integration:** `ctas7-real-port-manager` + `sx9-atlas-bus` (Crystal/Thyristor)

---

## Overview

Port allocation should be **gated by crystal resonance and SDT state**. Every port allocation request must pass through the crystal/thyristor system before being granted.

```
Port Request → Crystal Resonance → SDT Gate → Port Allocation Decision
```

---

## Architecture

### Current State

**Port Manager (`ctas7-real-port-manager`):**
- Allocates ports based on availability
- No gate control
- No resonance checking

**Crystal/Thyristor (`sx9-atlas-bus`):**
- Controls message flow (commands live/die)
- Not integrated with port allocation

### Integrated State

**Port allocation is now physics-based:**
1. Port request arrives with service metadata
2. Crystal resonance evaluates request (entropy, delta_angle, service hash)
3. SDT gate decides: Off (reject) | Primed (wait) | Conducting (allocate) | Latched (permanent)
4. Port allocation proceeds only if gate is Conducting or Latched

---

## Integration Points

### 1. Port Allocation Request → Crystal Resonance

```rust
// Port Manager with Crystal Integration
use sx9_atlas_bus::{PlasmaState, Polycrystal, ThyristorConfig, SdtState};

pub struct CrystalGatedPortManager {
    port_manager: PortManager,
    plasma_state: Arc<PlasmaState>,
    polycrystal: Arc<Polycrystal>,
    thyristor_config: ThyristorConfig,
    /// Enable/disable crystal/thyristor gating (for dev work)
    crystal_gating_enabled: AtomicBool,
    /// Enable/disable thyristor gating (can disable independently)
    thyristor_gating_enabled: AtomicBool,
}

impl CrystalGatedPortManager {
    /// Allocate port with crystal resonance gate
    pub async fn allocate_port_gated(
        &mut self,
        port: u16,
        service_name: &str,
        service_type: ServiceType,
        service_hash: u64,  // Trivariate hash of service
    ) -> Result<PortAllocation, PortManagerError> {
        // 1. Build payload from service metadata
        let payload = self.build_allocation_payload(service_name, service_type, service_hash);
        
        // 2. Crystal/Thyristor gating (can be disabled for dev)
        if self.crystal_gating_enabled.load(Ordering::Acquire) {
            // Crystal resonance check
            let ring_strength = self.polycrystal.resonate_all(
                &payload,
                self.plasma_state.delta_angle_raw(),
                self.plasma_state.entropy_raw(),
            );
            
            // Thyristor gating (can be disabled independently)
            if self.thyristor_gating_enabled.load(Ordering::Acquire) {
                // Update plasma state and check SDT gate
                let gate_allows = self.plasma_state.resonate_poly(
                    &self.polycrystal,
                    &payload,
                    self.get_tick(),
                    &self.thyristor_config,
                );
                
                // Gate decision
                match self.plasma_state.sdt_state() {
                    SdtState::Off | SdtState::Primed => {
                        // Gate blocked - reject allocation
                        return Err(PortManagerError::CrystalRejected {
                            ring_strength: ring_strength.final_strength,
                            sdt_state: self.plasma_state.sdt_state(),
                            reason: "Crystal did not ring, SDT gate blocked".to_string(),
                        });
                    }
                    SdtState::Conducting | SdtState::Latched => {
                        // Gate open - proceed with allocation
                    }
                }
            } else {
                // Thyristor disabled - only check crystal resonance (log only)
                tracing::debug!("Thyristor gating disabled, crystal resonance: {:.2}", ring_strength.final_strength);
            }
        } else {
            // Crystal gating disabled - bypass all checks (dev mode)
            tracing::warn!("⚠️  Crystal/Thyristor gating DISABLED - dev mode active");
        }
        
        // 5. Allocate port (original logic)
        self.port_manager.allocate_port(port, service_name, service_type).await
    }
    
    fn build_allocation_payload(
        &self,
        service_name: &str,
        service_type: &ServiceType,
        service_hash: u64,
    ) -> Vec<u8> {
        // Combine service metadata into payload for crystal resonance
        let mut payload = Vec::new();
        payload.extend_from_slice(service_name.as_bytes());
        payload.extend_from_slice(&service_type.to_string().as_bytes());
        payload.extend_from_slice(&service_hash.to_be_bytes());
        payload
    }
}
```

### 2. Mirror Port Allocation Based on Ring Strength

```rust
impl CrystalGatedPortManager {
    /// Allocate mirror ports based on crystal resonance strength
    pub fn get_mirror_count(&self, ring_strength: f32) -> usize {
        match ring_strength {
            r if r >= 0.98 => 3,  // Perfect ring → 3 mirrors (latched)
            r if r >= 0.90 => 2,  // Strong ring → 2 mirrors
            r if r >= 0.75 => 1,  // Weak ring → 1 mirror
            _ => 0,               // No ring → no mirrors
        }
    }
    
    /// Allocate port with automatic mirror allocation
    pub async fn allocate_port_with_mirrors(
        &mut self,
        port: u16,
        service_name: &str,
        service_type: ServiceType,
        service_hash: u64,
    ) -> Result<PortAllocation, PortManagerError> {
        // ... crystal resonance check ...
        
        let allocation = self.port_manager.allocate_port(port, service_name, service_type).await?;
        
        // Allocate mirrors based on ring strength
        let mirror_count = self.get_mirror_count(ring_strength.final_strength);
        let mut mirror_ports = Vec::new();
        
        for i in 0..mirror_count {
            // Find available port in same block
            let mirror_port = self.find_available_mirror_port(port, i).await?;
            mirror_ports.push(mirror_port);
        }
        
        // Update allocation with mirrors
        Ok(PortAllocation {
            mirror_ports,
            ..allocation
        })
    }
}
```

### 3. Port Deception Based on Crystal Family

```rust
impl CrystalGatedPortManager {
    /// Allocate deception port (honeypot) using TarPit crystal
    pub async fn allocate_deception_port(
        &mut self,
        service_name: &str,
    ) -> Result<PortAllocation, PortManagerError> {
        // Use TarPit crystal for deception (rings on anomalies)
        let tar_pit_crystal = Crystal::new(CrystalFamily::TarPit);
        
        // Build payload that should trigger TarPit (suspicious pattern)
        let payload = self.build_suspicious_payload(service_name);
        
        // Check if TarPit rings (inverted logic - rings on bad)
        let ring_strength = tar_pit_crystal.resonate_payload(
            &payload,
            self.plasma_state.delta_angle_raw(),
        );
        
        if ring_strength > 0.5 {
            // TarPit detected anomaly - allocate deception port
            let deception_port = self.find_deception_port().await?;
            return self.port_manager.allocate_port(
                deception_port,
                &format!("decoy-{}", service_name),
                ServiceType::CyberOps,
            ).await;
        }
        
        Err(PortManagerError::DeceptionNotTriggered)
    }
}
```

### 4. Port Release → SDT Anode Drop

```rust
impl CrystalGatedPortManager {
    /// Release port with SDT anode drop check
    pub async fn release_port_gated(
        &mut self,
        port: u16,
    ) -> Result<(), PortManagerError> {
        // Check if SDT is latched
        if self.plasma_state.is_latched() {
            // Latched ports require explicit reset
            return Err(PortManagerError::PortLatched(port));
        }
        
        // Check for anode drop (entropy drought)
        if self.plasma_state.entropy_raw() < self.thyristor_config.entropy_drought {
            // Anode drop - force release
            self.plasma_state.reset();
        }
        
        // Release port
        self.port_manager.release_port(port).await
    }
}
```

---

## SDT Gate States → Port Allocation Behavior

| SDT State | Port Allocation Behavior | Mirror Allocation |
|-----------|-------------------------|-------------------|
| **Off** | ❌ Reject all requests | 0 mirrors |
| **Primed** | ⏳ Wait for crystal ring | 0 mirrors |
| **Conducting** | ✅ Allocate port | 1-2 mirrors (based on ring strength) |
| **Latched** | ✅ Allocate port (permanent) | 3 mirrors (maximum redundancy) |

---

## Crystal Families → Port Block Selection

| Crystal Family | Port Block | Use Case |
|----------------|------------|----------|
| **Orbital** | 18120-18139 | High entropy tolerance, satellite services |
| **GroundStation** | 18140-18159 | Stable, corporate services (CDN) |
| **TarPit** | 18150-18152 | Deception/honeypot ports |
| **Silent** | 18160-18179 | Stealth ops, neural mesh |
| **Adaptive** | Dynamic | Learns from traffic patterns |

---

## Port Allocation Flow

```
┌─────────────────────────────────────────────────────────────────┐
│              CRYSTAL-GATED PORT ALLOCATION                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Port Request Arrives                                         │
│     └─ Service: "sx9-gateway"                                   │
│     └─ Type: CDN                                                │
│     └─ Hash: 0xDEADBEEFCAFEBABE                                │
│                                                                  │
│  2. Build Payload                                                │
│     └─ Combine: service_name + type + hash                     │
│                                                                  │
│  3. Crystal Resonance                                            │
│     └─ Polycrystal: GroundStation(0.8) + Adaptive(0.2)         │
│     └─ Ring Strength: 0.92                                      │
│     └─ Delta Class: Micro                                       │
│                                                                  │
│  4. SDT Gate Check                                               │
│     └─ Current: Primed                                          │
│     └─ Ring: 0.92 ≥ gate_thresh (0.50)                         │
│     └─ Transition: Primed → Conducting                          │
│                                                                  │
│  5. Port Allocation                                              │
│     └─ Primary: 18140 (CDN block)                               │
│     └─ Mirrors: 18141, 18142 (ring strength 0.92 → 2 mirrors) │
│                                                                  │
│  6. Result                                                       │
│     └─ ✅ Port allocated with 2 mirrors                        │
│     └─ SDT State: Conducting                                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum PortManagerError {
    // ... existing errors ...
    
    #[error("Crystal rejected allocation: ring_strength={ring_strength:.2}, sdt_state={sdt_state:?}, reason={reason}")]
    CrystalRejected {
        ring_strength: f32,
        sdt_state: SdtState,
        reason: String,
    },
    
    #[error("Port {0} is latched and cannot be released without reset")]
    PortLatched(u16),
    
    #[error("Deception port not triggered (TarPit did not ring)")]
    DeceptionNotTriggered,
    
    #[error("SDT gate blocked: state={0:?}")]
    SdtGateBlocked(SdtState),
}
```

---

## Dev Toggle Implementation

```rust
impl CrystalGatedPortManager {
    /// Enable/disable crystal gating (for dev work)
    pub fn set_crystal_gating(&self, enabled: bool) {
        self.crystal_gating_enabled.store(enabled, Ordering::Release);
        if !enabled {
            tracing::warn!("⚠️  Crystal gating DISABLED - dev mode active");
        }
    }
    
    /// Enable/disable thyristor gating independently
    pub fn set_thyristor_gating(&self, enabled: bool) {
        self.thyristor_gating_enabled.store(enabled, Ordering::Release);
        if !enabled {
            tracing::warn!("⚠️  Thyristor gating DISABLED - dev mode active");
        }
    }
    
    /// Check if crystal gating is enabled
    pub fn is_crystal_gating_enabled(&self) -> bool {
        self.crystal_gating_enabled.load(Ordering::Acquire)
    }
    
    /// Check if thyristor gating is enabled
    pub fn is_thyristor_gating_enabled(&self) -> bool {
        self.thyristor_gating_enabled.load(Ordering::Acquire)
    }
}
```

**Environment Variable Control:**
```rust
// In initialization
let crystal_enabled = std::env::var("PLASMA_CRYSTAL_GATING")
    .unwrap_or_else(|_| "true".to_string())
    .parse::<bool>()
    .unwrap_or(true);

let thyristor_enabled = std::env::var("PLASMA_THYRISTOR_GATING")
    .unwrap_or_else(|_| "true".to_string())
    .parse::<bool>()
    .unwrap_or(true);

manager.set_crystal_gating(crystal_enabled);
manager.set_thyristor_gating(thyristor_enabled);
```

**Usage:**
```bash
# Disable both for dev work
PLASMA_CRYSTAL_GATING=false PLASMA_THYRISTOR_GATING=false ./gateway

# Disable only thyristor (crystal still active)
PLASMA_THYRISTOR_GATING=false ./gateway

# Production (both enabled by default)
./gateway
```

## Integration Checklist

- [x] Add dev toggle to crystal/thyristor gating
- [ ] Add `sx9-atlas-bus` dependency to `ctas7-real-port-manager/Cargo.toml`
- [ ] Create `CrystalGatedPortManager` struct
- [ ] Implement `allocate_port_gated()` with crystal resonance
- [ ] Implement mirror allocation based on ring strength
- [ ] Add deception port allocation using TarPit crystal
- [ ] Implement port release with anode drop check
- [ ] Add error types for crystal rejection
- [ ] Update port allocation API to include service hash
- [ ] Add crystal family → port block mapping
- [ ] Update gateway task graph (TASK-004, TASK-005, TASK-006)

---

## Benefits

1. **Physics-Based Security:** Port allocation requires crystal resonance
2. **Automatic Mirroring:** Ring strength determines mirror count
3. **Deception Support:** TarPit crystal for honeypot ports
4. **State-Aware:** SDT gate controls allocation lifecycle
5. **Unified Control:** Same crystal/thyristor system for messages AND ports

---

**The crystal is the quartz. The thyristor is the switch it triggers. Port allocation is now part of the same physics.**

