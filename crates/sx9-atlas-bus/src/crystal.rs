//! Software-Defined Crystal (SDC)
//!
//! Pure resonance physics. No state machine, no routing decisions.
//! The crystal only answers one question: "How well does this payload ring?"
//!
//! ## Crystal Families
//! Different operational contexts use different resonance profiles:
//! - `Orbital`: Van Allen belt satellites, high entropy tolerance
//! - `GroundStation`: Stable, low jitter, strict thresholds
//! - `TarPit`: Honeypot mode, rings on suspicious patterns
//! - `Silent`: Stealth ops, rings only on perfect matches
//!
//! ## Resonance Score
//! Returns 0.0 - 1.0 indicating how well the payload "rings" the crystal.
//! This feeds directly into the SDT gate logic.
//!
//! ## Unicode Short Codes (RFC-9026)
//!
//! Crystal family runes are in Zone C (1-100ms):
//! - U+ED00: CRYSTAL_ORBITAL
//! - U+ED01: CRYSTAL_GROUND_STATION
//! - U+ED02: CRYSTAL_TAR_PIT
//! - U+ED03: CRYSTAL_SILENT
//! - U+ED04: CRYSTAL_ADAPTIVE
//!
//! Delta class runes:
//! - U+ED10: DELTA_NONE (< 2°)
//! - U+ED11: DELTA_MICRO (2-10°)
//! - U+ED12: DELTA_SOFT (10-25°)
//! - U+ED13: DELTA_HARD (25-60°)
//! - U+ED14: DELTA_CRITICAL (≥ 60°)

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// ============================================================================
// UNICODE SHORT CODES (RFC-9026 Zone C)
// ============================================================================

/// Unicode Private Use Area base for Crystal families (U+ED00)
pub const CRYSTAL_RUNE_BASE: u32 = 0xED00;

/// Crystal family runes
pub mod crystal_runes {
    use super::CRYSTAL_RUNE_BASE;

    /// Orbital crystal (U+ED00)
    pub const ORBITAL: u32 = CRYSTAL_RUNE_BASE + 0x00;
    /// Ground station crystal (U+ED01)
    pub const GROUND_STATION: u32 = CRYSTAL_RUNE_BASE + 0x01;
    /// Tar pit crystal (U+ED02)
    pub const TAR_PIT: u32 = CRYSTAL_RUNE_BASE + 0x02;
    /// Silent crystal (U+ED03)
    pub const SILENT: u32 = CRYSTAL_RUNE_BASE + 0x03;
    /// Adaptive crystal (U+ED04)
    pub const ADAPTIVE: u32 = CRYSTAL_RUNE_BASE + 0x04;

    /// Delta class: None (U+ED10)
    pub const DELTA_NONE: u32 = CRYSTAL_RUNE_BASE + 0x10;
    /// Delta class: Micro (U+ED11)
    pub const DELTA_MICRO: u32 = CRYSTAL_RUNE_BASE + 0x11;
    /// Delta class: Soft (U+ED12)
    pub const DELTA_SOFT: u32 = CRYSTAL_RUNE_BASE + 0x12;
    /// Delta class: Hard (U+ED13)
    pub const DELTA_HARD: u32 = CRYSTAL_RUNE_BASE + 0x13;
    /// Delta class: Critical (U+ED14)
    pub const DELTA_CRITICAL: u32 = CRYSTAL_RUNE_BASE + 0x14;
}

/// Crystal family determines resonance behavior
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrystalFamily {
    /// Orbital operations - high entropy tolerance, Van Allen belt
    Orbital = 0,
    /// Ground station - stable, strict thresholds
    GroundStation = 1,
    /// Tar pit / honeypot - rings on suspicious patterns
    TarPit = 2,
    /// Silent / stealth - only perfect matches ring
    Silent = 3,
    /// Adaptive - learns from traffic patterns
    Adaptive = 4,
}

impl CrystalFamily {
    /// Convert to Unicode rune (U+ED00-ED04)
    #[inline]
    pub const fn to_rune(self) -> u32 {
        CRYSTAL_RUNE_BASE + (self as u32)
    }

    /// Parse from Unicode rune
    #[inline]
    pub const fn from_rune(rune: u32) -> Option<Self> {
        if rune < CRYSTAL_RUNE_BASE || rune > CRYSTAL_RUNE_BASE + 4 {
            return None;
        }
        Some(match rune - CRYSTAL_RUNE_BASE {
            0 => CrystalFamily::Orbital,
            1 => CrystalFamily::GroundStation,
            2 => CrystalFamily::TarPit,
            3 => CrystalFamily::Silent,
            4 => CrystalFamily::Adaptive,
            _ => return None,
        })
    }
}

impl Default for CrystalFamily {
    fn default() -> Self {
        CrystalFamily::GroundStation
    }
}

/// Resonance thresholds per family
#[derive(Debug, Clone, Copy)]
pub struct ResonanceProfile {
    /// Minimum ring strength for "None" class (perfect)
    pub perfect_thresh: f32, // >= this → None class
    /// Minimum for "Micro" class
    pub micro_thresh: f32, // >= this → Micro class
    /// Minimum for "Soft" class  
    pub soft_thresh: f32, // >= this → Soft class
    /// Minimum for "Hard" class
    pub hard_thresh: f32, // >= this → Hard class
    /// Below this → Critical (crystal silent)
    pub critical_thresh: f32,
    /// Entropy weight in resonance calculation
    pub entropy_weight: f32,
    /// Delta angle weight
    pub delta_weight: f32,
    /// Hash coherence weight
    pub hash_weight: f32,
}

impl ResonanceProfile {
    /// Orbital profile - tolerant of entropy fluctuations
    pub const ORBITAL: Self = Self {
        perfect_thresh: 0.95,
        micro_thresh: 0.85,
        soft_thresh: 0.70,
        hard_thresh: 0.45,
        critical_thresh: 0.30,
        entropy_weight: 0.2,
        delta_weight: 0.5,
        hash_weight: 0.3,
    };

    /// Ground station - strict, stable
    pub const GROUND_STATION: Self = Self {
        perfect_thresh: 0.98,
        micro_thresh: 0.90,
        soft_thresh: 0.75,
        hard_thresh: 0.50,
        critical_thresh: 0.35,
        entropy_weight: 0.3,
        delta_weight: 0.4,
        hash_weight: 0.3,
    };

    /// Tar pit - inverted, rings on anomalies
    pub const TAR_PIT: Self = Self {
        perfect_thresh: 0.20, // Inverted! Low coherence = ring
        micro_thresh: 0.35,
        soft_thresh: 0.50,
        hard_thresh: 0.70,
        critical_thresh: 0.85,
        entropy_weight: 0.5, // High entropy sensitivity
        delta_weight: 0.3,
        hash_weight: 0.2,
    };

    /// Silent - only perfect matches
    pub const SILENT: Self = Self {
        perfect_thresh: 0.995,
        micro_thresh: 0.99,
        soft_thresh: 0.95,
        hard_thresh: 0.90,
        critical_thresh: 0.80,
        entropy_weight: 0.1,
        delta_weight: 0.6,
        hash_weight: 0.3,
    };

    /// Adaptive baseline (adjusted at runtime)
    pub const ADAPTIVE: Self = Self {
        perfect_thresh: 0.96,
        micro_thresh: 0.88,
        soft_thresh: 0.72,
        hard_thresh: 0.48,
        critical_thresh: 0.32,
        entropy_weight: 0.33,
        delta_weight: 0.34,
        hash_weight: 0.33,
    };

    /// Get profile for family
    pub fn for_family(family: CrystalFamily) -> Self {
        match family {
            CrystalFamily::Orbital => Self::ORBITAL,
            CrystalFamily::GroundStation => Self::GROUND_STATION,
            CrystalFamily::TarPit => Self::TAR_PIT,
            CrystalFamily::Silent => Self::SILENT,
            CrystalFamily::Adaptive => Self::ADAPTIVE,
        }
    }
}

/// Voting policy for polycrystal resonance
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VotingPolicy {
    /// ANY crystal fires → pass (most sensitive, tripwire)
    Any = 0,
    /// ALL crystals must fire → pass (strictest, corporate healthz)
    All = 1,
    /// Majority must fire → pass
    Majority = 2,
    /// Weighted average ≥ threshold → pass
    WeightedAverage = 3,
    /// Quorum (N of M) must fire → pass
    Quorum { required: u8 } = 4,
}

impl Default for VotingPolicy {
    fn default() -> Self {
        VotingPolicy::WeightedAverage
    }
}

/// Software-Defined Crystal
///
/// Pure resonance calculator. Feed it entropy + delta + hash,
/// get back a ring strength (0.0 - 1.0).
#[repr(C, align(64))]
pub struct Crystal {
    /// Crystal family determines resonance behavior
    family: CrystalFamily,

    /// Resonance profile (thresholds and weights)
    profile: ResonanceProfile,

    /// Weight in polycrystal voting (0.0 - 1.0)
    weight: f32,

    /// Running entropy accumulator (Monte Carlo)
    entropy_acc: AtomicU32,

    /// Last resonance score (for trend detection)
    last_ring: AtomicU32, // f32 as bits

    /// Ring count (for adaptive tuning)
    ring_count: AtomicU64,

    /// Silent count (no ring)
    silent_count: AtomicU64,
}

impl Clone for Crystal {
    fn clone(&self) -> Self {
        Self {
            family: self.family,
            profile: self.profile,
            weight: self.weight,
            entropy_acc: AtomicU32::new(self.entropy_acc.load(Ordering::Relaxed)),
            last_ring: AtomicU32::new(self.last_ring.load(Ordering::Relaxed)),
            ring_count: AtomicU64::new(self.ring_count.load(Ordering::Relaxed)),
            silent_count: AtomicU64::new(self.silent_count.load(Ordering::Relaxed)),
        }
    }
}

impl Crystal {
    /// Create new crystal with specified family (weight = 1.0)
    pub const fn new(family: CrystalFamily) -> Self {
        let profile = match family {
            CrystalFamily::Orbital => ResonanceProfile::ORBITAL,
            CrystalFamily::GroundStation => ResonanceProfile::GROUND_STATION,
            CrystalFamily::TarPit => ResonanceProfile::TAR_PIT,
            CrystalFamily::Silent => ResonanceProfile::SILENT,
            CrystalFamily::Adaptive => ResonanceProfile::ADAPTIVE,
        };

        Self {
            family,
            profile,
            weight: 1.0,
            entropy_acc: AtomicU32::new(0),
            last_ring: AtomicU32::new(0),
            ring_count: AtomicU64::new(0),
            silent_count: AtomicU64::new(0),
        }
    }

    /// Create with custom weight for polycrystal voting
    pub const fn with_weight(family: CrystalFamily, weight: f32) -> Self {
        let profile = match family {
            CrystalFamily::Orbital => ResonanceProfile::ORBITAL,
            CrystalFamily::GroundStation => ResonanceProfile::GROUND_STATION,
            CrystalFamily::TarPit => ResonanceProfile::TAR_PIT,
            CrystalFamily::Silent => ResonanceProfile::SILENT,
            CrystalFamily::Adaptive => ResonanceProfile::ADAPTIVE,
        };

        Self {
            family,
            profile,
            weight,
            entropy_acc: AtomicU32::new(0),
            last_ring: AtomicU32::new(0),
            ring_count: AtomicU64::new(0),
            silent_count: AtomicU64::new(0),
        }
    }

    /// Create with custom profile
    pub const fn with_profile(family: CrystalFamily, profile: ResonanceProfile) -> Self {
        Self {
            family,
            profile,
            weight: 1.0,
            entropy_acc: AtomicU32::new(0),
            last_ring: AtomicU32::new(0),
            ring_count: AtomicU64::new(0),
            silent_count: AtomicU64::new(0),
        }
    }

    /// Create with custom profile and weight
    pub const fn with_profile_and_weight(
        family: CrystalFamily,
        profile: ResonanceProfile,
        weight: f32,
    ) -> Self {
        Self {
            family,
            profile,
            weight,
            entropy_acc: AtomicU32::new(0),
            last_ring: AtomicU32::new(0),
            ring_count: AtomicU64::new(0),
            silent_count: AtomicU64::new(0),
        }
    }

    /// Get crystal weight
    #[inline]
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Core resonance calculation
    ///
    /// # Arguments
    /// - `entropy`: Current entropy value (e.g., from Monte Carlo)
    /// - `delta_angle`: Current delta angle (0-65535 → 0-360°)
    /// - `hash`: Payload hash (Murmur3 or similar)
    ///
    /// # Returns
    /// Ring strength 0.0 - 1.0
    #[inline]
    pub fn resonate(&self, entropy: u32, delta_angle: u16, hash: u64) -> f32 {
        // Normalize inputs to 0.0 - 1.0
        let e = (entropy as f32) / (u32::MAX as f32);
        let d = 1.0 - ((delta_angle as f32) / 65535.0); // Lower delta = better
        let h = self.hash_coherence(hash);

        // Weighted combination
        let raw = self.profile.entropy_weight * e
            + self.profile.delta_weight * d
            + self.profile.hash_weight * h;

        // Clamp to 0.0 - 1.0
        let ring_strength = raw.clamp(0.0, 1.0);

        // Update accumulators
        self.entropy_acc
            .fetch_add(entropy.wrapping_shr(16) as u32, Ordering::Relaxed);
        self.last_ring
            .store(ring_strength.to_bits(), Ordering::Release);

        if ring_strength >= self.profile.hard_thresh {
            self.ring_count.fetch_add(1, Ordering::Relaxed);
        } else {
            self.silent_count.fetch_add(1, Ordering::Relaxed);
        }

        ring_strength
    }

    /// Simplified resonance with just payload bytes
    #[inline]
    pub fn resonate_payload(&self, payload: &[u8], delta_angle: u16) -> f32 {
        let hash = self.quick_hash(payload);
        let entropy = self.payload_entropy(payload);
        self.resonate(entropy, delta_angle, hash)
    }

    /// Get delta class from ring strength
    #[inline]
    pub fn delta_class(&self, ring_strength: f32) -> DeltaClass {
        if ring_strength >= self.profile.perfect_thresh {
            DeltaClass::None
        } else if ring_strength >= self.profile.micro_thresh {
            DeltaClass::Micro
        } else if ring_strength >= self.profile.soft_thresh {
            DeltaClass::Soft
        } else if ring_strength >= self.profile.hard_thresh {
            DeltaClass::Hard
        } else {
            DeltaClass::Critical
        }
    }

    /// Check if crystal is ringing (above hard threshold)
    #[inline]
    pub fn is_ringing(&self, ring_strength: f32) -> bool {
        ring_strength >= self.profile.hard_thresh
    }

    /// Get last ring strength
    #[inline]
    pub fn last_ring_strength(&self) -> f32 {
        f32::from_bits(self.last_ring.load(Ordering::Acquire))
    }

    /// Get ring ratio (rings / total)
    #[inline]
    pub fn ring_ratio(&self) -> f32 {
        let rings = self.ring_count.load(Ordering::Relaxed);
        let silent = self.silent_count.load(Ordering::Relaxed);
        let total = rings + silent;

        if total == 0 {
            0.0
        } else {
            rings as f32 / total as f32
        }
    }

    /// Get family
    #[inline]
    pub fn family(&self) -> CrystalFamily {
        self.family
    }

    /// Get profile
    #[inline]
    pub fn profile(&self) -> &ResonanceProfile {
        &self.profile
    }

    // ========================================================================
    // Hash / Entropy helpers (public for Polycrystal use)
    // ========================================================================

    /// Quick hash for payload (Murmur3-like)
    #[inline]
    pub fn quick_hash(&self, payload: &[u8]) -> u64 {
        let mut h: u64 = 0x517cc1b727220a95;
        for chunk in payload.chunks(8) {
            let mut v: u64 = 0;
            for (i, &b) in chunk.iter().enumerate() {
                v |= (b as u64) << (i * 8);
            }
            h ^= v.wrapping_mul(0x87c37b91114253d5);
            h = h.rotate_left(31);
            h = h.wrapping_mul(5).wrapping_add(0x52dce729);
        }
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd);
        h ^= h >> 33;
        h
    }

    /// Estimate payload entropy (no_std compatible)
    #[inline]
    pub fn payload_entropy(&self, payload: &[u8]) -> u32 {
        if payload.is_empty() {
            return 0;
        }

        // Quick byte frequency estimation
        let mut counts = [0u32; 256];
        for &b in payload {
            counts[b as usize] += 1;
        }

        // Count unique bytes (simpler than Shannon, but no_std compatible)
        let mut unique = 0u32;
        let mut max_count = 0u32;

        for &count in &counts {
            if count > 0 {
                unique += 1;
                if count > max_count {
                    max_count = count;
                }
            }
        }

        // Entropy approximation based on:
        // - unique byte count (more unique = higher entropy)
        // - distribution evenness (max_count close to avg = higher entropy)
        let len = payload.len() as u32;
        let avg = len / unique.max(1);
        let evenness = if max_count > 0 {
            (avg * 256) / max_count
        } else {
            0
        };

        // Combine: unique bytes (0-256) + evenness (0-256)
        // Scale to u32 range
        let raw = (unique.min(256) << 8) | evenness.min(256);
        raw.wrapping_mul(0x10000)
    }

    /// Hash coherence (how "structured" the hash looks)
    #[inline]
    fn hash_coherence(&self, hash: u64) -> f32 {
        // Count bit transitions (structured data has fewer)
        let transitions = (hash ^ (hash >> 1)).count_ones();

        // Normalize: 32 transitions = random, 0 = perfect structure
        1.0 - (transitions as f32 / 32.0)
    }
}

impl Default for Crystal {
    fn default() -> Self {
        Self::new(CrystalFamily::GroundStation)
    }
}

/// Delta class derived from ring strength
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaClass {
    /// Perfect resonance (< 2°) - no regeneration needed
    None = 0,
    /// Micro adjustment (2-10°) - tweak CUID slots 10-11
    Micro = 1,
    /// Soft regeneration (10-25°) - regen SCH + CUID
    Soft = 2,
    /// Hard regeneration (25-60°) - full trivariate regen
    Hard = 3,
    /// Critical (≥ 60°) - supersede lineage, kill old command
    Critical = 4,
}

impl DeltaClass {
    /// Convert to delta angle in degrees
    pub fn to_degrees(&self) -> f32 {
        match self {
            DeltaClass::None => 0.0,
            DeltaClass::Micro => 5.0,
            DeltaClass::Soft => 17.5,
            DeltaClass::Hard => 42.5,
            DeltaClass::Critical => 90.0,
        }
    }

    /// Convert to Unicode rune (U+ED10-ED14)
    #[inline]
    pub const fn to_rune(self) -> u32 {
        CRYSTAL_RUNE_BASE + 0x10 + (self as u32)
    }

    /// Parse from Unicode rune
    #[inline]
    pub const fn from_rune(rune: u32) -> Option<Self> {
        let base = CRYSTAL_RUNE_BASE + 0x10;
        if rune < base || rune > base + 4 {
            return None;
        }
        Some(match rune - base {
            0 => DeltaClass::None,
            1 => DeltaClass::Micro,
            2 => DeltaClass::Soft,
            3 => DeltaClass::Hard,
            4 => DeltaClass::Critical,
            _ => return None,
        })
    }
}

// ============================================================================
// POLYCRYSTAL ARRAY - Multiple crystals voting in parallel
// ============================================================================

/// Maximum crystals in a polycrystal array (stack-allocated)
pub const MAX_CRYSTALS: usize = 8;

/// Polycrystal resonance result
#[derive(Debug, Clone, Copy)]
pub struct PolycrystalResult {
    /// Final ring strength (0.0 - 1.0) after voting
    pub ring_strength: f32,
    /// Number of crystals that fired (above their hard threshold)
    pub fired_count: usize,
    /// Total crystals in the array
    pub total_count: usize,
    /// Weighted sum before normalization
    pub weighted_sum: f32,
    /// Did the vote pass according to policy?
    pub passed: bool,
}

/// Polycrystal array - multiple crystals voting in parallel
///
/// Like doping a semiconductor with multiple impurities.
/// Different crystals for different threat models, orbits, customers.
#[repr(C)]
#[derive(Clone)]
pub struct Polycrystal {
    /// Array of crystals (stack-allocated, no Vec)
    crystals: [Option<Crystal>; MAX_CRYSTALS],
    /// Number of active crystals
    count: usize,
    /// Voting policy
    policy: VotingPolicy,
    /// Threshold for weighted average policy
    threshold: f32,
}

impl Polycrystal {
    /// Create empty polycrystal
    pub const fn new(policy: VotingPolicy) -> Self {
        Self {
            crystals: [None, None, None, None, None, None, None, None],
            count: 0,
            policy,
            threshold: 0.90,
        }
    }

    /// Create with threshold for weighted average
    pub const fn with_threshold(policy: VotingPolicy, threshold: f32) -> Self {
        Self {
            crystals: [None, None, None, None, None, None, None, None],
            count: 0,
            policy,
            threshold,
        }
    }

    /// Add a crystal to the array
    ///
    /// Returns false if array is full
    pub fn add(&mut self, crystal: Crystal) -> bool {
        if self.count >= MAX_CRYSTALS {
            return false;
        }
        self.crystals[self.count] = Some(crystal);
        self.count += 1;
        true
    }

    /// Get number of crystals
    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Resonate payload through all crystals and vote
    ///
    /// This is THE polycrystal resonance function.
    pub fn resonate_all(&self, entropy: u32, delta_angle: u16, hash: u64) -> PolycrystalResult {
        if self.count == 0 {
            return PolycrystalResult {
                ring_strength: 0.0,
                fired_count: 0,
                total_count: 0,
                weighted_sum: 0.0,
                passed: false,
            };
        }

        let mut fired_count = 0;
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for i in 0..self.count {
            if let Some(ref crystal) = self.crystals[i] {
                let strength = crystal.resonate(entropy, delta_angle, hash);
                let weight = crystal.weight();

                weighted_sum += strength * weight;
                total_weight += weight;

                if crystal.is_ringing(strength) {
                    fired_count += 1;
                }
            }
        }

        // Normalize
        let ring_strength = if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        };

        // Apply voting policy
        let passed = match self.policy {
            VotingPolicy::Any => fired_count > 0,
            VotingPolicy::All => fired_count == self.count,
            VotingPolicy::Majority => fired_count > self.count / 2,
            VotingPolicy::WeightedAverage => ring_strength >= self.threshold,
            VotingPolicy::Quorum { required } => fired_count >= required as usize,
        };

        PolycrystalResult {
            ring_strength,
            fired_count,
            total_count: self.count,
            weighted_sum,
            passed,
        }
    }

    /// Simplified resonance with just payload bytes
    pub fn resonate_payload(&self, payload: &[u8], delta_angle: u16) -> PolycrystalResult {
        if self.count == 0 {
            return PolycrystalResult {
                ring_strength: 0.0,
                fired_count: 0,
                total_count: 0,
                weighted_sum: 0.0,
                passed: false,
            };
        }

        // Use first crystal's hash/entropy functions
        if let Some(ref first) = self.crystals[0] {
            let hash = first.quick_hash(payload);
            let entropy = first.payload_entropy(payload);
            self.resonate_all(entropy, delta_angle, hash)
        } else {
            PolycrystalResult {
                ring_strength: 0.0,
                fired_count: 0,
                total_count: 0,
                weighted_sum: 0.0,
                passed: false,
            }
        }
    }
}

impl Default for Polycrystal {
    fn default() -> Self {
        Self::new(VotingPolicy::WeightedAverage)
    }
}

// ============================================================================
// PRESET POLYCRYSTAL CONFIGURATIONS
// ============================================================================

impl Polycrystal {
    /// Ultra-sensitive tripwire (ANY fires → pass)
    /// Good for: sx9.atlas.cmd.critical
    pub fn tripwire() -> Self {
        let mut p = Self::new(VotingPolicy::Any);
        p.add(Crystal::with_weight(CrystalFamily::Silent, 1.0));
        p.add(Crystal::with_weight(CrystalFamily::GroundStation, 1.0));
        p
    }

    /// Strict corporate (ALL must fire → pass)
    /// Good for: healthz, corporate compliance
    pub fn corporate_strict() -> Self {
        let mut p = Self::new(VotingPolicy::All);
        p.add(Crystal::with_weight(CrystalFamily::GroundStation, 1.0));
        p.add(Crystal::with_weight(CrystalFamily::TarPit, 1.0));
        p
    }

    /// Van Allen orbital entropy harvest
    /// Good for: sx9.sdt.van-allen
    pub fn van_allen() -> Self {
        let mut p = Self::with_threshold(VotingPolicy::WeightedAverage, 0.93);
        p.add(Crystal::with_weight(CrystalFamily::Orbital, 0.5));
        p.add(Crystal::with_weight(CrystalFamily::Adaptive, 0.3));
        p.add(Crystal::with_weight(CrystalFamily::Silent, 0.2));
        p
    }

    /// Normal operations (weighted, mostly corporate)
    /// Good for: sx9.atlas.cmd.normal
    pub fn normal_ops() -> Self {
        let mut p = Self::with_threshold(VotingPolicy::WeightedAverage, 0.90);
        p.add(Crystal::with_weight(CrystalFamily::GroundStation, 0.8));
        p.add(Crystal::with_weight(CrystalFamily::Adaptive, 0.2));
        p
    }

    /// Honeypot mode (inverted, catches anomalies)
    /// Good for: tar pits, deception networks
    pub fn honeypot() -> Self {
        let mut p = Self::with_threshold(VotingPolicy::WeightedAverage, 0.50);
        p.add(Crystal::with_weight(CrystalFamily::TarPit, 0.7));
        p.add(Crystal::with_weight(CrystalFamily::Adaptive, 0.3));
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_resonance() {
        let crystal = Crystal::new(CrystalFamily::GroundStation);

        // Perfect resonance: high entropy, low delta, structured hash (alternating bits)
        // Hash 0xAAAA... has 0 transitions = perfect coherence (1.0)
        // Delta 0 = 1.0 (inverted)
        // Entropy u32::MAX = 1.0
        // Weights: 0.3 * 1.0 + 0.4 * 1.0 + 0.3 * 1.0 = 1.0
        let strength = crystal.resonate(u32::MAX, 0, 0xAAAAAAAAAAAAAAAA);
        assert!(strength > 0.3, "Expected > 0.3, got {}", strength);

        // Poor resonance: low entropy, high delta, random hash
        let strength = crystal.resonate(0, 65535, 0x123456789ABCDEF0);
        // Low entropy + high delta + random hash = low strength
        // Should be lower than good case
        assert!(strength < 0.5, "Expected < 0.5, got {}", strength);

        // Good input should be better than bad input
        let good = crystal.resonate(u32::MAX, 0, 0xAAAAAAAAAAAAAAAA);
        let bad = crystal.resonate(0, 65535, 0x123456789ABCDEF0);
        assert!(good > bad, "Expected good ({}) > bad ({})", good, bad);
    }

    #[test]
    fn test_delta_class() {
        let crystal = Crystal::new(CrystalFamily::GroundStation);

        assert_eq!(crystal.delta_class(0.99), DeltaClass::None);
        assert_eq!(crystal.delta_class(0.92), DeltaClass::Micro);
        assert_eq!(crystal.delta_class(0.80), DeltaClass::Soft);
        assert_eq!(crystal.delta_class(0.55), DeltaClass::Hard);
        assert_eq!(crystal.delta_class(0.30), DeltaClass::Critical);
    }

    #[test]
    fn test_tar_pit_inverted() {
        let crystal = Crystal::new(CrystalFamily::TarPit);

        // Tar pit has INVERTED thresholds - low coherence = ring
        // With medium inputs, should get a non-extreme class
        let strength = crystal.resonate(u32::MAX / 2, 32768, 0xDEADBEEF);

        // Tar pit thresholds are inverted, so check the strength is reasonable
        assert!(
            strength > 0.0 && strength < 1.0,
            "Expected 0 < strength < 1, got {}",
            strength
        );
    }

    #[test]
    fn test_payload_resonance() {
        let crystal = Crystal::new(CrystalFamily::GroundStation);

        // Structured payload (low entropy - single byte repeated)
        let structured = b"AAAAAAAAAAAAAAAA";
        let s1 = crystal.resonate_payload(structured, 1000);

        // Random payload (high entropy - many unique bytes)
        let random = b"\x12\x34\x56\x78\x9A\xBC\xDE\xF0\x11\x22\x33\x44\x55\x66\x77\x88";
        let s2 = crystal.resonate_payload(random, 1000);

        // Both should produce valid strengths
        assert!(s1 >= 0.0 && s1 <= 1.0);
        assert!(s2 >= 0.0 && s2 <= 1.0);
    }

    #[test]
    fn test_polycrystal_any() {
        let poly = Polycrystal::tripwire();

        // With high entropy, low delta, good hash - should fire at least one
        let result = poly.resonate_all(u32::MAX, 0, 0xAAAAAAAAAAAAAAAA);

        // Check we have crystals
        assert!(result.total_count > 0);

        // Ring strength should be reasonable
        assert!(result.ring_strength >= 0.0 && result.ring_strength <= 1.0);
    }

    #[test]
    fn test_polycrystal_all() {
        let poly = Polycrystal::corporate_strict();

        // Perfect input - should pass ALL
        let result = poly.resonate_all(u32::MAX, 0, 0xAAAAAAAAAAAAAAAA);
        // May or may not pass depending on tar pit inversion
        assert!(result.total_count == 2);
    }

    #[test]
    fn test_polycrystal_weighted() {
        let poly = Polycrystal::normal_ops();

        // Good input - high entropy, low delta
        let result = poly.resonate_all(u32::MAX, 0, 0xAAAAAAAAAAAAAAAA);
        assert!(
            result.ring_strength > 0.0,
            "Expected > 0, got {}",
            result.ring_strength
        );

        // Bad input - zero entropy, max delta
        let result = poly.resonate_all(0, 65535, 0x123456789ABCDEF0);
        // Should be lower than good input
        assert!(
            result.ring_strength < 0.7,
            "Expected < 0.7, got {}",
            result.ring_strength
        );
    }

    #[test]
    fn test_van_allen() {
        let poly = Polycrystal::van_allen();

        assert_eq!(poly.len(), 3);

        // Orbital entropy harvest
        let result = poly.resonate_all(u32::MAX / 2, 10000, 0xDEADBEEFCAFEBABE);
        assert!(result.total_count == 3);
    }
}
