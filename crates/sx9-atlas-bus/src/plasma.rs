//! Plasma state integration
//!
//! Tracks plasma field state and SDT gate status within the bus.
//!
//! ## Architecture
//!
//! The plasma system has two distinct layers:
//!
//! 1. **Crystal (Software-Defined Crystal)**: Pure resonance physics.
//!    - Takes entropy + delta_angle + hash → returns ring_strength (0.0-1.0)
//!    - Family-specific (Orbital, GroundStation, TarPit, Silent)
//!    - Stateless except for accumulators
//!
//! 2. **Thyristor (Software-Defined Thyristor)**: Gate control.
//!    - Takes ring_strength → decides Off/Primed/Conducting/Latched
//!    - Controls command lifetime, supersession, lineage kill
//!    - Implements holding current and anode drop logic
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    PLASMA STATE MACHINE                          │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                  │
//! │   Payload ──┬──► Crystal ──► ring_strength ──► Thyristor        │
//! │             │      │                              │              │
//! │   Entropy ──┤      │                              ▼              │
//! │             │      │                         SDT Gate            │
//! │   Delta ────┘      │                         Off/Primed/         │
//! │                    │                         Conducting/Latched  │
//! │                    │                              │              │
//! │                    └──────────────────────────────┼──► Command   │
//! │                                                   │    Lives/Dies│
//! │                                                   │              │
//! │   Anode Drop ─────────────────────────────────────┘              │
//! │   (entropy drought, holding current < threshold)                 │
//! │                                                                  │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicBool, Ordering};
use crate::crystal::{Crystal, CrystalFamily, DeltaClass, Polycrystal, PolycrystalResult};

/// SDT (Software-Defined Thyristor) states
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdtState {
    /// Gate is off, no flow allowed
    Off = 0,
    /// Gate is primed, waiting for trigger
    Primed = 1,
    /// Gate is conducting, flow allowed
    Conducting = 2,
    /// Gate is latched, requires explicit reset
    Latched = 3,
}

impl From<u8> for SdtState {
    fn from(v: u8) -> Self {
        match v {
            0 => SdtState::Off,
            1 => SdtState::Primed,
            2 => SdtState::Conducting,
            3 => SdtState::Latched,
            _ => SdtState::Off,
        }
    }
}

/// Thyristor configuration
#[derive(Debug, Clone, Copy)]
pub struct ThyristorConfig {
    /// Gate threshold - minimum ring_strength to fire (Primed → Conducting)
    pub gate_thresh: f32,
    /// Holding threshold - below this, conducting → off (anode drop)
    pub holding_thresh: f32,
    /// Perfect threshold - above this, auto-latch
    pub perfect_thresh: f32,
    /// Entropy drought threshold - if entropy drops below, consider anode drop
    pub entropy_drought: u32,
}

impl Default for ThyristorConfig {
    fn default() -> Self {
        Self {
            gate_thresh: 0.50,
            holding_thresh: 0.35,
            perfect_thresh: 0.98,
            entropy_drought: 1000,
        }
    }
}

impl ThyristorConfig {
    /// Strict config for critical operations
    pub const STRICT: Self = Self {
        gate_thresh: 0.75,
        holding_thresh: 0.50,
        perfect_thresh: 0.995,
        entropy_drought: 5000,
    };
    
    /// Permissive config for development
    pub const PERMISSIVE: Self = Self {
        gate_thresh: 0.30,
        holding_thresh: 0.20,
        perfect_thresh: 0.90,
        entropy_drought: 100,
    };
}

/// Plasma field state
///
/// Embedded in the bus for real-time state tracking without additional IPC.
/// Integrates the Crystal (resonance) and Thyristor (gate control) layers.
#[repr(C, align(64))]
pub struct PlasmaState {
    /// Current delta angle (fixed point, 0-65535 maps to 0-360°)
    delta_angle: AtomicU16,
    
    /// Current entropy value
    entropy: AtomicU32,
    
    /// Whether plasma is in excited state (crystal ringing)
    excited: AtomicBool,
    
    /// SDT gate state
    sdt_state: AtomicU8,
    
    /// Last trigger tick
    last_trigger_tick: AtomicU64,
    
    /// Trigger count (for rate limiting)
    trigger_count: AtomicU32,
    
    /// Last ring strength (f32 as bits)
    last_ring_strength: AtomicU32,
    
    /// Supersession count (lineages killed)
    supersession_count: AtomicU32,
    
    /// Padding to cache line
    _pad: [u8; 24],
}

impl PlasmaState {
    /// Create new plasma state
    pub const fn new() -> Self {
        Self {
            delta_angle: AtomicU16::new(0),
            entropy: AtomicU32::new(0),
            excited: AtomicBool::new(false),
            sdt_state: AtomicU8::new(SdtState::Off as u8),
            last_trigger_tick: AtomicU64::new(0),
            trigger_count: AtomicU32::new(0),
            last_ring_strength: AtomicU32::new(0),
            supersession_count: AtomicU32::new(0),
            _pad: [0; 24],
        }
    }
    
    // ========================================================================
    // Crystal + Thyristor Integration (THE CORE)
    // ========================================================================
    
    /// Resonate payload through single crystal and update thyristor state
    ///
    /// This is THE function that decides if a command lives or dies.
    ///
    /// # Returns
    /// - `true` if command should proceed (crystal rang, thyristor conducting/latched)
    /// - `false` if command should be killed (crystal silent, thyristor blocked)
    #[inline]
    pub fn resonate(
        &self,
        crystal: &Crystal,
        payload: &[u8],
        tick: u64,
        config: &ThyristorConfig,
    ) -> bool {
        let delta = self.delta_angle_raw();
        
        // Crystal resonance
        let ring_strength = crystal.resonate_payload(payload, delta);
        self.last_ring_strength.store(ring_strength.to_bits(), Ordering::Release);
        
        // Update excited state
        let is_ringing = ring_strength >= config.holding_thresh;
        self.excited.store(is_ringing, Ordering::Release);
        
        // Thyristor state machine
        let current_sdt = self.sdt_state();
        let new_sdt = self.compute_sdt_transition(
            current_sdt,
            ring_strength,
            config,
        );
        
        // Apply state transition
        if new_sdt != current_sdt {
            self.sdt_state.store(new_sdt as u8, Ordering::Release);
            
            if new_sdt == SdtState::Conducting || new_sdt == SdtState::Latched {
                self.last_trigger_tick.store(tick, Ordering::Release);
                self.trigger_count.fetch_add(1, Ordering::AcqRel);
            }
        }
        
        // Command lives if thyristor is conducting or latched
        matches!(new_sdt, SdtState::Conducting | SdtState::Latched)
    }
    
    /// Resonate payload through POLYCRYSTAL array and update thyristor state
    ///
    /// Multiple crystals vote in parallel. Different families, different weights.
    /// Like doping a semiconductor with multiple impurities.
    ///
    /// # Returns
    /// - `true` if command should proceed (vote passed, thyristor conducting/latched)
    /// - `false` if command should be killed (vote failed, thyristor blocked)
    #[inline]
    pub fn resonate_poly(
        &self,
        polycrystal: &Polycrystal,
        payload: &[u8],
        tick: u64,
        config: &ThyristorConfig,
    ) -> (bool, PolycrystalResult) {
        let delta = self.delta_angle_raw();
        
        // Polycrystal resonance - all crystals vote
        let result = polycrystal.resonate_payload(payload, delta);
        
        // Store weighted ring strength
        self.last_ring_strength.store(result.ring_strength.to_bits(), Ordering::Release);
        
        // Excited if vote passed
        self.excited.store(result.passed, Ordering::Release);
        
        // Thyristor state machine uses the final ring_strength
        let current_sdt = self.sdt_state();
        let new_sdt = self.compute_sdt_transition(
            current_sdt,
            result.ring_strength,
            config,
        );
        
        // Apply state transition
        if new_sdt != current_sdt {
            self.sdt_state.store(new_sdt as u8, Ordering::Release);
            
            if new_sdt == SdtState::Conducting || new_sdt == SdtState::Latched {
                self.last_trigger_tick.store(tick, Ordering::Release);
                self.trigger_count.fetch_add(1, Ordering::AcqRel);
            }
        }
        
        // Command lives if vote passed AND thyristor is conducting/latched
        let passed = result.passed && matches!(new_sdt, SdtState::Conducting | SdtState::Latched);
        (passed, result)
    }
    
    /// Compute SDT state transition based on ring strength
    #[inline]
    fn compute_sdt_transition(
        &self,
        current: SdtState,
        ring_strength: f32,
        config: &ThyristorConfig,
    ) -> SdtState {
        match (current, ring_strength) {
            // Perfect ring → latch forever
            (_, r) if r >= config.perfect_thresh => SdtState::Latched,
            
            // Already latched → check for anode drop
            (SdtState::Latched, r) if r < config.holding_thresh => {
                // Anode drop - unlatch
                SdtState::Off
            }
            (SdtState::Latched, _) => SdtState::Latched, // Stay latched
            
            // Off or Primed → check gate threshold
            (SdtState::Off | SdtState::Primed, r) if r >= config.gate_thresh => {
                SdtState::Conducting
            }
            (SdtState::Off, _) => SdtState::Off,
            (SdtState::Primed, _) => SdtState::Primed,
            
            // Conducting → check holding current
            (SdtState::Conducting, r) if r < config.holding_thresh => {
                // Holding current lost
                SdtState::Off
            }
            (SdtState::Conducting, _) => SdtState::Conducting,
        }
    }
    
    /// Check for anode drop (entropy drought)
    ///
    /// Call this periodically to kill commands during entropy drought.
    #[inline]
    pub fn check_anode_drop(&self, config: &ThyristorConfig) -> bool {
        let entropy = self.entropy();
        let current = self.sdt_state();
        
        if entropy < config.entropy_drought && current == SdtState::Latched {
            self.sdt_state.store(SdtState::Off as u8, Ordering::Release);
            true
        } else {
            false
        }
    }
    
    /// Supersede a lineage (kill all commands with matching lineage)
    ///
    /// Called when delta class is Critical.
    #[inline]
    pub fn supersede(&self) {
        self.supersession_count.fetch_add(1, Ordering::AcqRel);
        self.sdt_state.store(SdtState::Off as u8, Ordering::Release);
        self.excited.store(false, Ordering::Release);
    }
    
    /// Get last ring strength
    #[inline]
    pub fn last_ring_strength(&self) -> f32 {
        f32::from_bits(self.last_ring_strength.load(Ordering::Acquire))
    }
    
    /// Get supersession count
    #[inline]
    pub fn supersession_count(&self) -> u32 {
        self.supersession_count.load(Ordering::Acquire)
    }
    
    /// Get delta class from current ring strength
    #[inline]
    pub fn current_delta_class(&self, crystal: &Crystal) -> DeltaClass {
        crystal.delta_class(self.last_ring_strength())
    }
    
    // ========================================================================
    // Delta Angle
    // ========================================================================
    
    /// Get current delta angle in degrees (0.0 - 360.0)
    #[inline]
    pub fn delta_angle(&self) -> f32 {
        let raw = self.delta_angle.load(Ordering::Acquire);
        (raw as f32 / 65535.0) * 360.0
    }
    
    /// Get raw delta angle value
    #[inline]
    pub fn delta_angle_raw(&self) -> u16 {
        self.delta_angle.load(Ordering::Acquire)
    }
    
    /// Set delta angle in degrees
    #[inline]
    pub fn set_delta_angle(&self, degrees: f32) {
        let raw = ((degrees / 360.0) * 65535.0) as u16;
        self.delta_angle.store(raw, Ordering::Release);
    }
    
    /// Set raw delta angle
    #[inline]
    pub fn set_delta_angle_raw(&self, raw: u16) {
        self.delta_angle.store(raw, Ordering::Release);
    }
    
    /// Get delta angle class based on supersession thresholds
    ///
    /// Returns:
    /// - 0: None (< 2°)
    /// - 1: Micro (2-10°)
    /// - 2: Soft (10-25°)
    /// - 3: Hard (25-60°)
    /// - 4: Critical (≥ 60°)
    #[inline]
    pub fn delta_class(&self) -> u8 {
        let degrees = self.delta_angle();
        match degrees {
            d if d < 2.0 => 0,
            d if d < 10.0 => 1,
            d if d < 25.0 => 2,
            d if d < 60.0 => 3,
            _ => 4,
        }
    }
    
    // ========================================================================
    // Entropy
    // ========================================================================
    
    /// Get current entropy
    #[inline]
    pub fn entropy(&self) -> u32 {
        self.entropy.load(Ordering::Acquire)
    }
    
    /// Set entropy
    #[inline]
    pub fn set_entropy(&self, entropy: u32) {
        self.entropy.store(entropy, Ordering::Release);
    }
    
    /// Add to entropy (saturating)
    #[inline]
    pub fn add_entropy(&self, delta: u32) {
        self.entropy.fetch_add(delta, Ordering::AcqRel);
    }
    
    // ========================================================================
    // Excited State
    // ========================================================================
    
    /// Check if plasma is excited
    #[inline]
    pub fn is_excited(&self) -> bool {
        self.excited.load(Ordering::Acquire)
    }
    
    /// Set excited state
    #[inline]
    pub fn set_excited(&self, excited: bool) {
        self.excited.store(excited, Ordering::Release);
    }
    
    /// Excite the plasma
    #[inline]
    pub fn excite(&self) {
        self.excited.store(true, Ordering::Release);
    }
    
    /// De-excite the plasma
    #[inline]
    pub fn relax(&self) {
        self.excited.store(false, Ordering::Release);
    }
    
    // ========================================================================
    // SDT Gate
    // ========================================================================
    
    /// Get SDT state
    #[inline]
    pub fn sdt_state(&self) -> SdtState {
        SdtState::from(self.sdt_state.load(Ordering::Acquire))
    }
    
    /// Set SDT state
    #[inline]
    pub fn set_sdt_state(&self, state: SdtState) {
        self.sdt_state.store(state as u8, Ordering::Release);
    }
    
    /// Check if SDT is conducting (allowing flow)
    #[inline]
    pub fn is_conducting(&self) -> bool {
        matches!(self.sdt_state(), SdtState::Conducting)
    }
    
    /// Check if SDT is latched (requires reset)
    #[inline]
    pub fn is_latched(&self) -> bool {
        matches!(self.sdt_state(), SdtState::Latched)
    }
    
    /// Prime the SDT gate
    #[inline]
    pub fn prime(&self) {
        self.set_sdt_state(SdtState::Primed);
    }
    
    /// Trigger the SDT gate (primed → conducting)
    ///
    /// Returns true if trigger was successful
    #[inline]
    pub fn trigger(&self, tick: u64) -> bool {
        let current = self.sdt_state.load(Ordering::Acquire);
        
        if current == SdtState::Primed as u8 {
            self.sdt_state.store(SdtState::Conducting as u8, Ordering::Release);
            self.last_trigger_tick.store(tick, Ordering::Release);
            self.trigger_count.fetch_add(1, Ordering::AcqRel);
            true
        } else {
            false
        }
    }
    
    /// Latch the SDT gate (conducting → latched)
    #[inline]
    pub fn latch(&self) {
        let current = self.sdt_state.load(Ordering::Acquire);
        if current == SdtState::Conducting as u8 {
            self.sdt_state.store(SdtState::Latched as u8, Ordering::Release);
        }
    }
    
    /// Reset the SDT gate (any → off)
    #[inline]
    pub fn reset(&self) {
        self.set_sdt_state(SdtState::Off);
    }
    
    /// Get last trigger tick
    #[inline]
    pub fn last_trigger_tick(&self) -> u64 {
        self.last_trigger_tick.load(Ordering::Acquire)
    }
    
    /// Get trigger count
    #[inline]
    pub fn trigger_count(&self) -> u32 {
        self.trigger_count.load(Ordering::Acquire)
    }
    
    // ========================================================================
    // Combined Operations
    // ========================================================================
    
    /// Update plasma state atomically
    #[inline]
    pub fn update(&self, delta_angle: u16, entropy: u32, excited: bool) {
        self.delta_angle.store(delta_angle, Ordering::Release);
        self.entropy.store(entropy, Ordering::Release);
        self.excited.store(excited, Ordering::Release);
    }
    
    /// Snapshot current state
    #[inline]
    pub fn snapshot(&self) -> PlasmaSnapshot {
        PlasmaSnapshot {
            delta_angle: self.delta_angle_raw(),
            entropy: self.entropy(),
            excited: self.is_excited(),
            sdt_state: self.sdt_state(),
            last_trigger_tick: self.last_trigger_tick(),
            trigger_count: self.trigger_count(),
            last_ring_strength: self.last_ring_strength(),
            supersession_count: self.supersession_count(),
        }
    }
}

impl Default for PlasmaState {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable snapshot of plasma state
#[derive(Debug, Clone, Copy)]
pub struct PlasmaSnapshot {
    pub delta_angle: u16,
    pub entropy: u32,
    pub excited: bool,
    pub sdt_state: SdtState,
    pub last_trigger_tick: u64,
    pub trigger_count: u32,
    pub last_ring_strength: f32,
    pub supersession_count: u32,
}

impl PlasmaSnapshot {
    /// Get delta angle in degrees
    pub fn delta_degrees(&self) -> f32 {
        (self.delta_angle as f32 / 65535.0) * 360.0
    }
    
    /// Check if crystal is ringing
    pub fn is_ringing(&self) -> bool {
        self.excited
    }
    
    /// Check if thyristor is allowing flow
    pub fn is_conducting(&self) -> bool {
        matches!(self.sdt_state, SdtState::Conducting | SdtState::Latched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_delta_angle() {
        let plasma = PlasmaState::new();
        
        plasma.set_delta_angle(45.0);
        let angle = plasma.delta_angle();
        assert!((angle - 45.0).abs() < 0.01);
        
        plasma.set_delta_angle(180.0);
        assert!((plasma.delta_angle() - 180.0).abs() < 0.01);
    }
    
    #[test]
    fn test_delta_class() {
        let plasma = PlasmaState::new();
        
        plasma.set_delta_angle(1.0);
        assert_eq!(plasma.delta_class(), 0); // None
        
        plasma.set_delta_angle(5.0);
        assert_eq!(plasma.delta_class(), 1); // Micro
        
        plasma.set_delta_angle(15.0);
        assert_eq!(plasma.delta_class(), 2); // Soft
        
        plasma.set_delta_angle(40.0);
        assert_eq!(plasma.delta_class(), 3); // Hard
        
        plasma.set_delta_angle(90.0);
        assert_eq!(plasma.delta_class(), 4); // Critical
    }
    
    #[test]
    fn test_sdt_lifecycle() {
        let plasma = PlasmaState::new();
        
        assert_eq!(plasma.sdt_state(), SdtState::Off);
        
        plasma.prime();
        assert_eq!(plasma.sdt_state(), SdtState::Primed);
        
        assert!(plasma.trigger(100));
        assert_eq!(plasma.sdt_state(), SdtState::Conducting);
        assert_eq!(plasma.last_trigger_tick(), 100);
        assert_eq!(plasma.trigger_count(), 1);
        
        plasma.latch();
        assert_eq!(plasma.sdt_state(), SdtState::Latched);
        
        plasma.reset();
        assert_eq!(plasma.sdt_state(), SdtState::Off);
    }
    
    #[test]
    fn test_snapshot() {
        let plasma = PlasmaState::new();
        
        plasma.set_delta_angle(30.0);
        plasma.set_entropy(42);
        plasma.excite();
        plasma.prime();
        plasma.trigger(999);
        
        let snap = plasma.snapshot();
        
        assert!((snap.delta_degrees() - 30.0).abs() < 0.1);
        assert_eq!(snap.entropy, 42);
        assert!(snap.excited);
        assert_eq!(snap.sdt_state, SdtState::Conducting);
        assert_eq!(snap.last_trigger_tick, 999);
    }
}

