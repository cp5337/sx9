//! Plasma eBPF Common Library
//!
//! Unified hash → Unicode → eBPF encoding for SDT protocol.
//!
//! ## Trivariate Hash System
//!
//! The trivariate hash consists of three components:
//!
//! 1. **SCH (Semantic Content Hash)** - 64 bits
//!    - Domain mask (16 bits): Cyber/Geo/Space/Maritime
//!    - Execution mask (16 bits): HD4 phase + context
//!    - N-V-N-N structure (16 bits): Noun-Verb-Noun-Noun semantic
//!    - Delta angle (16 bits): Cognitive state delta
//!
//! 2. **CUID (Cognitive Unique Identifier)** - 128 bits
//!    - 16 slots × 8 bits each
//!    - Slots 10-11: Delta angle (tick-aligned)
//!    - Slots 12-13: Entropy sample
//!
//! 3. **UUID** - 128 bits (standard UUIDv7)
//!
//! ## Semantic Hash Features
//!
//! The SCH encodes semantic meaning through:
//! - **Thalmic filtering**: Suppression runes for noise/legacy/overlap
//! - **Priority runes**: U+E800 range for priority levels
//! - **Confidence runes**: U+E900 range for confidence scores
//! - **Domain runes**: Which of the 4 worlds (cyber/geo/space/maritime)
//! - **Agent routing**: Which agent should handle this hash
//!
//! ## Encoding Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                    HASH → UNICODE → eBPF PIPELINE                        │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │   Trivariate Hash ──► Unicode Runes ──► SDT Frame ──► eBPF Map          │
//! │   [SCH][CUID][UUID]   U+E000-U+F8FF    EtherType     BPF_MAP_TYPE_HASH  │
//! │                       (Private Use)    0xSD77                            │
//! │                                                                          │
//! │   ┌──────────────────────────────────────────────────────────────────┐  │
//! │   │ SCH (64 bits) = Domain(16) + Execution(16) + N-V-N-N(16) + Δθ(16)│  │
//! │   │                                                                   │  │
//! │   │ Semantic encoding via Murmur3:                                   │  │
//! │   │   domain_mask = murmur3(domain_text) & 0xFFFF                    │  │
//! │   │   exec_mask = murmur3(hd4_phase + context) & 0xFFFF              │  │
//! │   │   nvnn = murmur3(noun1 + verb + noun2 + noun3) & 0xFFFF          │  │
//! │   │   delta = current_delta_angle                                     │  │
//! │   │                                                                   │  │
//! │   │ Encoded as 4 Unicode runes in Private Use Area:                  │  │
//! │   │   U+E000 + (Domain >> 4)                                         │  │
//! │   │   U+E100 + (Execution >> 4)                                      │  │
//! │   │   U+E200 + (NVNN >> 4)                                           │  │
//! │   │   U+E300 + (DeltaAngle >> 4)                                     │  │
//! │   └──────────────────────────────────────────────────────────────────┘  │
//! │                                                                          │
//! │   ┌──────────────────────────────────────────────────────────────────┐  │
//! │   │ CUID (128 bits) = 16 slots × 8 bits each                         │  │
//! │   │                                                                   │  │
//! │   │ Slots 10-11 = Delta angle (tick-aligned)                         │  │
//! │   │ Encoded as 8 Unicode runes:                                      │  │
//! │   │   U+E400 + slots[0:1]                                            │  │
//! │   │   U+E500 + slots[2:3]                                            │  │
//! │   │   ...                                                            │  │
//! │   │   U+EB00 + slots[14:15]                                          │  │
//! │   └──────────────────────────────────────────────────────────────────┘  │
//! │                                                                          │
//! │   ┌──────────────────────────────────────────────────────────────────┐  │
//! │   │ eBPF Map Key = 8 bytes                                           │  │
//! │   │                                                                   │  │
//! │   │ [Rune0:2][Rune1:2][Rune2:2][Rune3:2]                             │  │
//! │   │                                                                   │  │
//! │   │ Directly indexable, cache-line aligned                           │  │
//! │   └──────────────────────────────────────────────────────────────────┘  │
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

#![no_std]

// ============================================================================
// UNICODE PRIVATE USE AREA ALLOCATION
// ============================================================================

/// Unicode Private Use Area base (U+E000)
pub const PUA_BASE: u32 = 0xE000;

/// Rune ranges for different hash components
pub mod runes {
    use super::PUA_BASE;
    
    // ========================================================================
    // SCH (Semantic Content Hash) Runes - U+E000 to U+E3FF
    // ========================================================================
    
    /// Domain mask runes (U+E000 - U+E0FF)
    /// Encodes which of the 4 worlds: Cyber, Geo, Space, Maritime
    pub const DOMAIN_BASE: u32 = PUA_BASE + 0x000;
    
    /// Execution mask runes (U+E100 - U+E1FF)
    /// Encodes HD4 phase: Hunt, Detect, Disrupt, Disable, Dominate
    pub const EXECUTION_BASE: u32 = PUA_BASE + 0x100;
    
    /// N-V-N-N structure runes (U+E200 - U+E2FF)
    /// Semantic structure: Noun-Verb-Noun-Noun
    pub const NVNN_BASE: u32 = PUA_BASE + 0x200;
    
    /// Delta angle runes (U+E300 - U+E3FF)
    /// Cognitive state delta (0-360°)
    pub const DELTA_ANGLE_BASE: u32 = PUA_BASE + 0x300;
    
    // ========================================================================
    // CUID (Cognitive Unique Identifier) Runes - U+E400 to U+EBFF
    // ========================================================================
    
    /// CUID slot runes (U+E400 - U+EBFF)
    /// 8 rune ranges for 16 CUID slots (2 slots per rune)
    pub const CUID_BASE: u32 = PUA_BASE + 0x400;
    
    // ========================================================================
    // Thalmic Filter Runes - U+E800 to U+E9FF (Semantic Suppression)
    // ========================================================================
    
    /// Priority runes (U+E800 - U+E87F)
    /// 0x00 = lowest, 0x7F = highest
    pub const PRIORITY_BASE: u32 = 0xE800;
    
    /// Confidence runes (U+E880 - U+E8FF)
    /// 0x00 = 0%, 0x7F = 100%
    pub const CONFIDENCE_BASE: u32 = 0xE880;
    
    /// Suppression runes (U+E900 - U+E97F)
    /// Thalmic filter suppression codes
    pub const SUPPRESSION_BASE: u32 = 0xE900;
    
    /// Agent routing runes (U+E980 - U+E9FF)
    /// Which agent should handle this hash
    pub const AGENT_ROUTE_BASE: u32 = 0xE980;
    
    // ========================================================================
    // Suppression Codes (within U+E900 range)
    // ========================================================================
    
    /// No suppression
    pub const SUPPRESS_NONE: u32 = SUPPRESSION_BASE + 0x00;
    /// Noise - filter out
    pub const SUPPRESS_NOISE: u32 = SUPPRESSION_BASE + 0x01;
    /// Legacy - deprecated content
    pub const SUPPRESS_LEGACY: u32 = SUPPRESSION_BASE + 0x02;
    /// Overlap - duplicate of existing
    pub const SUPPRESS_OVERLAP: u32 = SUPPRESSION_BASE + 0x03;
    /// Redundant - already processed
    pub const SUPPRESS_REDUNDANT: u32 = SUPPRESSION_BASE + 0x04;
    /// Low confidence - below threshold
    pub const SUPPRESS_LOW_CONF: u32 = SUPPRESSION_BASE + 0x05;
    
    // ========================================================================
    // Domain Codes (within U+E000 range)
    // ========================================================================
    
    /// Cyber domain
    pub const DOMAIN_CYBER: u32 = DOMAIN_BASE + 0x10;
    /// Geographical domain
    pub const DOMAIN_GEO: u32 = DOMAIN_BASE + 0x20;
    /// Space domain
    pub const DOMAIN_SPACE: u32 = DOMAIN_BASE + 0x30;
    /// Maritime domain
    pub const DOMAIN_MARITIME: u32 = DOMAIN_BASE + 0x40;
    /// Fusion (multiple domains)
    pub const DOMAIN_FUSION: u32 = DOMAIN_BASE + 0x50;
    
    // ========================================================================
    // HD4 Phase Codes (within U+E100 range)
    // ========================================================================
    
    /// Hunt phase
    pub const HD4_HUNT: u32 = EXECUTION_BASE + 0x10;
    /// Detect phase
    pub const HD4_DETECT: u32 = EXECUTION_BASE + 0x20;
    /// Disrupt phase
    pub const HD4_DISRUPT: u32 = EXECUTION_BASE + 0x30;
    /// Disable phase
    pub const HD4_DISABLE: u32 = EXECUTION_BASE + 0x40;
    /// Dominate phase
    pub const HD4_DOMINATE: u32 = EXECUTION_BASE + 0x50;
    
    // ========================================================================
    // SDT / Crystal / Tool Runes - U+EC00 to U+EFFF
    // ========================================================================
    
    /// SDT state runes (U+EC00 - U+ECFF)
    pub const SDT_STATE_BASE: u32 = PUA_BASE + 0xC00;
    
    /// Crystal family runes (U+ED00 - U+EDFF)
    pub const CRYSTAL_BASE: u32 = PUA_BASE + 0xD00;
    
    /// Tool trigger runes (U+EE00 - U+EEFF)
    pub const TOOL_TRIGGER_BASE: u32 = PUA_BASE + 0xE00;
    
    /// Tool response runes (U+EF00 - U+EFFF)
    pub const TOOL_RESPONSE_BASE: u32 = PUA_BASE + 0xF00;
    
    /// Completion byte (U+F8FF - Apple's private use)
    pub const COMPLETION: u32 = 0xF8FF;
}

// ============================================================================
// THALMIC ANNOTATION (Semantic Filtering)
// ============================================================================

/// Thalmic annotation for semantic filtering
///
/// The thalamic filter decides what content passes through to processing.
/// Named after the thalamus - the brain's relay station that filters sensory input.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThalmicAnnotation {
    /// Priority level (0-127)
    pub priority: u8,
    /// Confidence score (0-127 → 0-100%)
    pub confidence: u8,
    /// Suppression code
    pub suppression: SuppressionCode,
    /// Target agent ID
    pub agent_route: u8,
}

/// Suppression codes for thalmic filtering
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuppressionCode {
    /// No suppression - pass through
    None = 0,
    /// Noise - filter out
    Noise = 1,
    /// Legacy - deprecated content
    Legacy = 2,
    /// Overlap - duplicate of existing
    Overlap = 3,
    /// Redundant - already processed
    Redundant = 4,
    /// Low confidence - below threshold
    LowConfidence = 5,
}

impl ThalmicAnnotation {
    /// Create new annotation
    pub const fn new(priority: u8, confidence: u8) -> Self {
        Self {
            priority: priority & 0x7F,
            confidence: confidence & 0x7F,
            suppression: SuppressionCode::None,
            agent_route: 0,
        }
    }
    
    /// Check if content should be suppressed
    pub fn should_suppress(&self, threshold: u8) -> bool {
        self.suppression != SuppressionCode::None || self.confidence < threshold
    }
    
    /// Encode to Unicode runes (4 runes)
    pub fn to_runes(&self) -> [u32; 4] {
        [
            runes::PRIORITY_BASE + (self.priority as u32),
            runes::CONFIDENCE_BASE + (self.confidence as u32),
            runes::SUPPRESSION_BASE + (self.suppression as u32),
            runes::AGENT_ROUTE_BASE + (self.agent_route as u32),
        ]
    }
    
    /// Decode from Unicode runes
    pub fn from_runes(runes_arr: [u32; 4]) -> Self {
        Self {
            priority: (runes_arr[0] - runes::PRIORITY_BASE) as u8 & 0x7F,
            confidence: (runes_arr[1] - runes::CONFIDENCE_BASE) as u8 & 0x7F,
            suppression: match (runes_arr[2] - runes::SUPPRESSION_BASE) as u8 {
                0 => SuppressionCode::None,
                1 => SuppressionCode::Noise,
                2 => SuppressionCode::Legacy,
                3 => SuppressionCode::Overlap,
                4 => SuppressionCode::Redundant,
                5 => SuppressionCode::LowConfidence,
                _ => SuppressionCode::None,
            },
            agent_route: (runes_arr[3] - runes::AGENT_ROUTE_BASE) as u8,
        }
    }
}

// ============================================================================
// DOMAIN ENCODING
// ============================================================================

/// Operational domain (4 worlds)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    /// Cyber domain
    Cyber = 0x10,
    /// Geographical domain
    Geo = 0x20,
    /// Space domain
    Space = 0x30,
    /// Maritime domain
    Maritime = 0x40,
    /// Fusion (multiple domains)
    Fusion = 0x50,
}

impl Domain {
    /// Convert to Unicode rune
    pub fn to_rune(self) -> u32 {
        runes::DOMAIN_BASE + (self as u32)
    }
    
    /// Parse from domain text
    pub fn from_text(text: &[u8]) -> Self {
        let hash = murmur3_32(text, 0xD0AA1A) & 0xFF;
        match hash % 5 {
            0 => Domain::Cyber,
            1 => Domain::Geo,
            2 => Domain::Space,
            3 => Domain::Maritime,
            _ => Domain::Fusion,
        }
    }
}

// ============================================================================
// HD4 PHASE ENCODING
// ============================================================================

/// HD4 operational phase
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hd4Phase {
    /// Hunt - reconnaissance
    Hunt = 0x10,
    /// Detect - identification
    Detect = 0x20,
    /// Disrupt - interference
    Disrupt = 0x30,
    /// Disable - neutralization
    Disable = 0x40,
    /// Dominate - control
    Dominate = 0x50,
}

impl Hd4Phase {
    /// Convert to Unicode rune
    pub fn to_rune(self) -> u32 {
        runes::EXECUTION_BASE + (self as u32)
    }
    
    /// Parse from phase text
    pub fn from_text(text: &[u8]) -> Self {
        let hash = murmur3_32(text, 0x4D4D4) & 0xFF;
        match hash % 5 {
            0 => Hd4Phase::Hunt,
            1 => Hd4Phase::Detect,
            2 => Hd4Phase::Disrupt,
            3 => Hd4Phase::Disable,
            _ => Hd4Phase::Dominate,
        }
    }
}

// ============================================================================
// SCH (Semantic Content Hash) - 64 bits
// ============================================================================

/// SCH hash components
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SchHash {
    /// Domain mask (16 bits)
    /// Encodes the operational domain (cyber, geo, space, maritime)
    pub domain: u16,
    
    /// Execution mask (16 bits)
    /// Encodes HD4 phase and execution context
    pub execution: u16,
    
    /// N-V-N-N structure (16 bits)
    /// Noun-Verb-Noun-Noun semantic encoding
    pub nvnn: u16,
    
    /// Delta angle (16 bits)
    /// Current cognitive state delta (0-65535 → 0-360°)
    pub delta_angle: u16,
}

impl SchHash {
    /// Create new SCH hash
    pub const fn new(domain: u16, execution: u16, nvnn: u16, delta_angle: u16) -> Self {
        Self { domain, execution, nvnn, delta_angle }
    }
    
    /// Build SCH from semantic components
    ///
    /// # Arguments
    /// - `domain_text`: Text describing the domain (e.g., "cyber", "space")
    /// - `phase_text`: HD4 phase text (e.g., "hunt", "detect")
    /// - `noun1`: First noun in N-V-N-N structure
    /// - `verb`: Verb in N-V-N-N structure
    /// - `noun2`: Second noun
    /// - `noun3`: Third noun
    /// - `delta_angle`: Current cognitive state delta
    pub fn from_semantic(
        domain_text: &[u8],
        phase_text: &[u8],
        noun1: &[u8],
        verb: &[u8],
        noun2: &[u8],
        noun3: &[u8],
        delta_angle: u16,
    ) -> Self {
        // Hash domain text to get domain mask
        let domain = murmur3_32(domain_text, 0xD0AA1A) as u16;
        
        // Hash phase text to get execution mask
        let execution = murmur3_32(phase_text, 0xFA5E5) as u16;
        
        // Hash N-V-N-N structure
        let mut nvnn_data = [0u8; 64];
        let mut offset = 0;
        
        for &b in noun1.iter().take(16) {
            nvnn_data[offset] = b;
            offset += 1;
        }
        for &b in verb.iter().take(16) {
            nvnn_data[offset] = b;
            offset += 1;
        }
        for &b in noun2.iter().take(16) {
            nvnn_data[offset] = b;
            offset += 1;
        }
        for &b in noun3.iter().take(16) {
            nvnn_data[offset] = b;
            offset += 1;
        }
        
        let nvnn = murmur3_32(&nvnn_data[..offset], 0xABBA) as u16;
        
        Self { domain, execution, nvnn, delta_angle }
    }
    
    /// Build SCH with thalmic annotation
    pub fn with_thalmic(
        domain: Domain,
        phase: Hd4Phase,
        nvnn_text: &[u8],
        delta_angle: u16,
        annotation: &ThalmicAnnotation,
    ) -> (Self, [u32; 4]) {
        let domain_mask = (domain as u16) << 8 | (annotation.priority as u16);
        let execution_mask = (phase as u16) << 8 | (annotation.confidence as u16);
        let nvnn = murmur3_32(nvnn_text, 0xABBA) as u16;
        
        let sch = Self {
            domain: domain_mask,
            execution: execution_mask,
            nvnn,
            delta_angle,
        };
        
        let thalmic_runes = annotation.to_runes();
        
        (sch, thalmic_runes)
    }
    
    /// Encode to 4 Unicode runes
    pub fn to_runes(&self) -> [u32; 4] {
        [
            runes::DOMAIN_BASE + (self.domain >> 4) as u32,
            runes::EXECUTION_BASE + (self.execution >> 4) as u32,
            runes::NVNN_BASE + (self.nvnn >> 4) as u32,
            runes::DELTA_ANGLE_BASE + (self.delta_angle >> 4) as u32,
        ]
    }
    
    /// Decode from 4 Unicode runes
    pub fn from_runes(runes: [u32; 4]) -> Self {
        Self {
            domain: ((runes[0] - runes::DOMAIN_BASE) << 4) as u16,
            execution: ((runes[1] - runes::EXECUTION_BASE) << 4) as u16,
            nvnn: ((runes[2] - runes::NVNN_BASE) << 4) as u16,
            delta_angle: ((runes[3] - runes::DELTA_ANGLE_BASE) << 4) as u16,
        }
    }
    
    /// Convert to eBPF map key (8 bytes)
    pub fn to_ebpf_key(&self) -> [u8; 8] {
        let runes = self.to_runes();
        [
            (runes[0] & 0xFF) as u8,
            ((runes[0] >> 8) & 0xFF) as u8,
            (runes[1] & 0xFF) as u8,
            ((runes[1] >> 8) & 0xFF) as u8,
            (runes[2] & 0xFF) as u8,
            ((runes[2] >> 8) & 0xFF) as u8,
            (runes[3] & 0xFF) as u8,
            ((runes[3] >> 8) & 0xFF) as u8,
        ]
    }
    
    /// Convert to raw 64-bit value
    pub fn to_u64(&self) -> u64 {
        ((self.domain as u64) << 48) |
        ((self.execution as u64) << 32) |
        ((self.nvnn as u64) << 16) |
        (self.delta_angle as u64)
    }
    
    /// Convert to bytes (8 bytes)
    pub fn to_bytes(&self) -> [u8; 8] {
        [
            (self.domain >> 8) as u8,
            (self.domain & 0xFF) as u8,
            (self.execution >> 8) as u8,
            (self.execution & 0xFF) as u8,
            (self.nvnn >> 8) as u8,
            (self.nvnn & 0xFF) as u8,
            (self.delta_angle >> 8) as u8,
            (self.delta_angle & 0xFF) as u8,
        ]
    }
}

// ============================================================================
// CUID (Cognitive Unique Identifier) - 128 bits
// ============================================================================

/// CUID slot assignments
pub mod cuid_slots {
    /// Slot 0-1: Agent ID
    pub const AGENT_ID: (usize, usize) = (0, 1);
    
    /// Slot 2-3: Task ID
    pub const TASK_ID: (usize, usize) = (2, 3);
    
    /// Slot 4-5: Sequence number
    pub const SEQUENCE: (usize, usize) = (4, 5);
    
    /// Slot 6-7: Timestamp (high bits)
    pub const TIMESTAMP_HI: (usize, usize) = (6, 7);
    
    /// Slot 8-9: Timestamp (low bits)
    pub const TIMESTAMP_LO: (usize, usize) = (8, 9);
    
    /// Slot 10-11: Delta angle (TICK-ALIGNED)
    pub const DELTA_ANGLE: (usize, usize) = (10, 11);
    
    /// Slot 12-13: Entropy sample
    pub const ENTROPY: (usize, usize) = (12, 13);
    
    /// Slot 14-15: Checksum
    pub const CHECKSUM: (usize, usize) = (14, 15);
}

/// CUID hash (128 bits = 16 slots)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CuidHash {
    /// 16 slots of 8 bits each
    pub slots: [u8; 16],
}

impl CuidHash {
    /// Create new CUID with all zeros
    pub const fn new() -> Self {
        Self { slots: [0; 16] }
    }
    
    /// Set delta angle in slots 10-11 (tick-aligned)
    pub fn set_delta_angle(&mut self, delta: u16) {
        self.slots[10] = (delta >> 8) as u8;
        self.slots[11] = (delta & 0xFF) as u8;
    }
    
    /// Get delta angle from slots 10-11
    pub fn get_delta_angle(&self) -> u16 {
        ((self.slots[10] as u16) << 8) | (self.slots[11] as u16)
    }
    
    /// Encode to 8 Unicode runes (2 slots per rune)
    pub fn to_runes(&self) -> [u32; 8] {
        [
            runes::CUID_BASE + 0x000 + ((self.slots[0] as u32) << 8) + (self.slots[1] as u32),
            runes::CUID_BASE + 0x100 + ((self.slots[2] as u32) << 8) + (self.slots[3] as u32),
            runes::CUID_BASE + 0x200 + ((self.slots[4] as u32) << 8) + (self.slots[5] as u32),
            runes::CUID_BASE + 0x300 + ((self.slots[6] as u32) << 8) + (self.slots[7] as u32),
            runes::CUID_BASE + 0x400 + ((self.slots[8] as u32) << 8) + (self.slots[9] as u32),
            runes::CUID_BASE + 0x500 + ((self.slots[10] as u32) << 8) + (self.slots[11] as u32),
            runes::CUID_BASE + 0x600 + ((self.slots[12] as u32) << 8) + (self.slots[13] as u32),
            runes::CUID_BASE + 0x700 + ((self.slots[14] as u32) << 8) + (self.slots[15] as u32),
        ]
    }
    
    /// Convert to eBPF map key (16 bytes)
    pub fn to_ebpf_key(&self) -> [u8; 16] {
        self.slots
    }
    
    /// Extract 64-bit "essence" from 128-bit CUID
    ///
    /// Extracts the most important slots:
    /// - Slots 0-1: Agent ID (16 bits)
    /// - Slots 4-5: Sequence (16 bits)
    /// - Slots 10-11: Delta angle (16 bits) ← CRITICAL
    /// - Slots 12-13: Entropy (16 bits)
    ///
    /// This is the minimum viable CUID for compact operations.
    pub fn extract_64(&self) -> u64 {
        let agent_id = ((self.slots[0] as u64) << 8) | (self.slots[1] as u64);
        let sequence = ((self.slots[4] as u64) << 8) | (self.slots[5] as u64);
        let delta = ((self.slots[10] as u64) << 8) | (self.slots[11] as u64);
        let entropy = ((self.slots[12] as u64) << 8) | (self.slots[13] as u64);
        
        (agent_id << 48) | (sequence << 32) | (delta << 16) | entropy
    }
    
    /// Set agent ID in slots 0-1
    pub fn set_agent_id(&mut self, id: u16) {
        self.slots[0] = (id >> 8) as u8;
        self.slots[1] = (id & 0xFF) as u8;
    }
    
    /// Set sequence in slots 4-5
    pub fn set_sequence(&mut self, seq: u16) {
        self.slots[4] = (seq >> 8) as u8;
        self.slots[5] = (seq & 0xFF) as u8;
    }
    
    /// Set entropy in slots 12-13
    pub fn set_entropy(&mut self, entropy: u16) {
        self.slots[12] = (entropy >> 8) as u8;
        self.slots[13] = (entropy & 0xFF) as u8;
    }
}

// ============================================================================
// SDT FRAME ENCODING
// ============================================================================

/// SDT frame header (18 bytes, aligned for eBPF)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SdtHeader {
    /// Protocol version (0x0001)
    pub version: u16,
    
    /// SDT state (Off=0, Primed=1, Conducting=2, Latched=3)
    pub state: u16,
    
    /// Delta angle (fixed point, 0.001° resolution)
    pub delta_angle: u32,
    
    /// Entropy value
    pub entropy: u32,
    
    /// Identity hash (truncated Murmur3)
    pub hash: u32,
    
    /// Payload type
    pub payload_type: u16,
}

impl SdtHeader {
    /// SDT state as Unicode rune
    pub fn state_rune(&self) -> u32 {
        runes::SDT_STATE_BASE + (self.state as u32)
    }
    
    /// Convert to eBPF map key
    pub fn to_ebpf_key(&self) -> [u8; 8] {
        // Use hash + delta_angle as key
        [
            (self.hash & 0xFF) as u8,
            ((self.hash >> 8) & 0xFF) as u8,
            ((self.hash >> 16) & 0xFF) as u8,
            ((self.hash >> 24) & 0xFF) as u8,
            (self.delta_angle & 0xFF) as u8,
            ((self.delta_angle >> 8) & 0xFF) as u8,
            ((self.delta_angle >> 16) & 0xFF) as u8,
            ((self.delta_angle >> 24) & 0xFF) as u8,
        ]
    }
}

// ============================================================================
// TOOL TRIGGERS (Unicode → eBPF)
// ============================================================================

/// Tool trigger encoding
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolTrigger {
    // nmap (0x10-0x1F)
    NmapSynScan = 0x10,
    NmapUdpScan = 0x11,
    NmapVersionDetect = 0x12,
    NmapOsFingerprint = 0x13,
    NmapScriptScan = 0x14,
    
    // masscan (0x20-0x2F)
    MasscanTcpScan = 0x20,
    MasscanUdpScan = 0x21,
    MasscanBannerGrab = 0x22,
    
    // nuclei (0x30-0x3F)
    NucleiTemplateScan = 0x30,
    NucleiCveScan = 0x31,
    NucleiCustomScan = 0x32,
    
    // sqlmap (0x40-0x4F)
    SqlmapDetect = 0x40,
    SqlmapExploit = 0x41,
    SqlmapDump = 0x42,
    
    // hydra (0x50-0x5F)
    HydraSsh = 0x50,
    HydraFtp = 0x51,
    HydraHttp = 0x52,
    HydraSmb = 0x53,
    
    // metasploit (0x60-0x6F)
    MsfExploit = 0x60,
    MsfPayload = 0x61,
    MsfPost = 0x62,
    MsfAuxiliary = 0x63,
    
    // responder (0x70-0x7F)
    ResponderLlmnr = 0x70,
    ResponderNbtns = 0x71,
    ResponderMdns = 0x72,
    
    // impacket (0x80-0x8F)
    ImpacketSmb = 0x80,
    ImpacketWmi = 0x81,
    ImpacketDce = 0x82,
    ImpacketKerberos = 0x83,
    
    // bloodhound (0x90-0x9F)
    BloodhoundCollect = 0x90,
    BloodhoundAnalyze = 0x91,
    
    // crackmapexec (0xA0-0xAF)
    CmeSmb = 0xA0,
    CmeWinrm = 0xA1,
    CmeSsh = 0xA2,
    CmeMssql = 0xA3,
}

impl ToolTrigger {
    /// Convert to Unicode rune
    pub fn to_rune(self) -> u32 {
        runes::TOOL_TRIGGER_BASE + (self as u32)
    }
    
    /// Parse from Unicode rune
    pub fn from_rune(rune: u32) -> Option<Self> {
        let code = (rune - runes::TOOL_TRIGGER_BASE) as u8;
        
        // Validate range
        match code {
            0x10..=0x14 => Some(unsafe { core::mem::transmute(code) }),
            0x20..=0x22 => Some(unsafe { core::mem::transmute(code) }),
            0x30..=0x32 => Some(unsafe { core::mem::transmute(code) }),
            0x40..=0x42 => Some(unsafe { core::mem::transmute(code) }),
            0x50..=0x53 => Some(unsafe { core::mem::transmute(code) }),
            0x60..=0x63 => Some(unsafe { core::mem::transmute(code) }),
            0x70..=0x72 => Some(unsafe { core::mem::transmute(code) }),
            0x80..=0x83 => Some(unsafe { core::mem::transmute(code) }),
            0x90..=0x91 => Some(unsafe { core::mem::transmute(code) }),
            0xA0..=0xA3 => Some(unsafe { core::mem::transmute(code) }),
            _ => None,
        }
    }
    
    /// Convert to eBPF program index
    pub fn to_ebpf_index(self) -> u8 {
        self as u8
    }
}

// ============================================================================
// eBPF MAP KEY GENERATION
// ============================================================================

/// Generate eBPF map key from trivariate hash
///
/// The key is structured for efficient BPF map lookup:
/// - First 4 bytes: SCH hash (domain + execution)
/// - Next 4 bytes: CUID delta angle slots + entropy
pub fn trivariate_to_ebpf_key(sch: &SchHash, cuid: &CuidHash) -> [u8; 8] {
    [
        // SCH: domain + execution (4 bytes)
        (sch.domain >> 8) as u8,
        (sch.domain & 0xFF) as u8,
        (sch.execution >> 8) as u8,
        (sch.execution & 0xFF) as u8,
        // CUID: delta angle slots 10-11 + entropy slots 12-13
        cuid.slots[10],
        cuid.slots[11],
        cuid.slots[12],
        cuid.slots[13],
    ]
}

/// Generate extended eBPF map key (16 bytes)
pub fn trivariate_to_ebpf_key_extended(sch: &SchHash, cuid: &CuidHash) -> [u8; 16] {
    let mut key = [0u8; 16];
    
    // SCH (8 bytes)
    let sch_key = sch.to_ebpf_key();
    key[0..8].copy_from_slice(&sch_key);
    
    // CUID delta + entropy (8 bytes)
    key[8..16].copy_from_slice(&cuid.slots[8..16]);
    
    key
}

// ============================================================================
// TRIVARIATE HASH (SCH + CUID + UUID)
// ============================================================================

/// Full trivariate hash
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrivariateHash {
    /// Semantic Content Hash (64 bits)
    pub sch: SchHash,
    /// Cognitive Unique Identifier (128 bits)
    pub cuid: CuidHash,
    /// UUID (128 bits) - stored as two u64s
    pub uuid_hi: u64,
    pub uuid_lo: u64,
    /// Thalmic annotation
    pub thalmic: ThalmicAnnotation,
}

impl TrivariateHash {
    /// Create new trivariate hash
    pub fn new(sch: SchHash, cuid: CuidHash, uuid_hi: u64, uuid_lo: u64) -> Self {
        Self {
            sch,
            cuid,
            uuid_hi,
            uuid_lo,
            thalmic: ThalmicAnnotation::new(64, 100), // Default: medium priority, high confidence
        }
    }
    
    /// Create with thalmic annotation
    pub fn with_thalmic(
        sch: SchHash,
        cuid: CuidHash,
        uuid_hi: u64,
        uuid_lo: u64,
        thalmic: ThalmicAnnotation,
    ) -> Self {
        Self { sch, cuid, uuid_hi, uuid_lo, thalmic }
    }
    
    /// Encode to Unicode runes (SCH + CUID + Thalmic = 4 + 8 + 4 = 16 runes)
    pub fn to_runes(&self) -> [u32; 16] {
        let sch_runes = self.sch.to_runes();
        let cuid_runes = self.cuid.to_runes();
        let thalmic_runes = self.thalmic.to_runes();
        
        [
            sch_runes[0], sch_runes[1], sch_runes[2], sch_runes[3],
            cuid_runes[0], cuid_runes[1], cuid_runes[2], cuid_runes[3],
            cuid_runes[4], cuid_runes[5], cuid_runes[6], cuid_runes[7],
            thalmic_runes[0], thalmic_runes[1], thalmic_runes[2], thalmic_runes[3],
        ]
    }
    
    /// Convert to eBPF map key (16 bytes)
    ///
    /// Key structure:
    /// - Bytes 0-3: SCH domain + execution
    /// - Bytes 4-7: SCH nvnn + delta
    /// - Bytes 8-11: CUID delta slots + entropy
    /// - Bytes 12-15: Thalmic annotation
    pub fn to_ebpf_key(&self) -> [u8; 16] {
        let mut key = [0u8; 16];
        
        // SCH (8 bytes)
        key[0] = (self.sch.domain >> 8) as u8;
        key[1] = (self.sch.domain & 0xFF) as u8;
        key[2] = (self.sch.execution >> 8) as u8;
        key[3] = (self.sch.execution & 0xFF) as u8;
        key[4] = (self.sch.nvnn >> 8) as u8;
        key[5] = (self.sch.nvnn & 0xFF) as u8;
        key[6] = (self.sch.delta_angle >> 8) as u8;
        key[7] = (self.sch.delta_angle & 0xFF) as u8;
        
        // CUID delta + entropy (4 bytes)
        key[8] = self.cuid.slots[10];
        key[9] = self.cuid.slots[11];
        key[10] = self.cuid.slots[12];
        key[11] = self.cuid.slots[13];
        
        // Thalmic (4 bytes)
        key[12] = self.thalmic.priority;
        key[13] = self.thalmic.confidence;
        key[14] = self.thalmic.suppression as u8;
        key[15] = self.thalmic.agent_route;
        
        key
    }
    
    /// Check if this hash should be suppressed
    pub fn should_suppress(&self, confidence_threshold: u8) -> bool {
        self.thalmic.should_suppress(confidence_threshold)
    }
    
    /// Get delta class based on delta angle
    pub fn delta_class(&self) -> DeltaClass {
        let degrees = (self.sch.delta_angle as f32 / 65535.0) * 360.0;
        match degrees {
            d if d < 2.0 => DeltaClass::None,
            d if d < 10.0 => DeltaClass::Micro,
            d if d < 25.0 => DeltaClass::Soft,
            d if d < 60.0 => DeltaClass::Hard,
            _ => DeltaClass::Critical,
        }
    }
}

/// Delta class for supersession logic
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeltaClass {
    /// < 2° - no regeneration needed
    None = 0,
    /// 2-10° - tweak CUID slots 10-11
    Micro = 1,
    /// 10-25° - regen SCH + CUID
    Soft = 2,
    /// 25-60° - full trivariate regen
    Hard = 3,
    /// ≥ 60° - supersede lineage
    Critical = 4,
}

// ============================================================================
// BASE96 ENCODING (Trivariate Canonical Format)
// ============================================================================

/// Base96 alphabet (96 printable ASCII characters)
///
/// ASCII 0x21-0x7E (94 chars) + 2 extras = 96
/// We use all printable ASCII except space, plus tab and newline mapped
pub const BASE96_ALPHABET: &[u8; 96] = &[
    // 0-9 (10)
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    // A-Z (26)
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
    b'U', b'V', b'W', b'X', b'Y', b'Z',
    // a-z (26)
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
    b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
    b'u', b'v', b'w', b'x', b'y', b'z',
    // Special (34)
    b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*',
    b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>',
    b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|',
    b'}', b'~', 0x7F, 0x80, // Use high bytes for last 2
];

/// Encode bytes to Base96 string
///
/// Returns array of encoded bytes (no heap allocation)
pub fn base96_encode(data: &[u8], output: &mut [u8]) -> usize {
    if data.is_empty() || output.is_empty() {
        return 0;
    }
    
    let mut out_idx = 0;
    let mut accumulator: u64 = 0;
    let mut bits: u32 = 0;
    
    for &byte in data {
        accumulator = (accumulator << 8) | (byte as u64);
        bits += 8;
        
        // Extract Base96 characters (log2(96) ≈ 6.58 bits per char)
        while bits >= 7 && out_idx < output.len() {
            bits -= 7;
            let idx = ((accumulator >> bits) & 0x7F) as usize;
            // Map 0-127 to 0-95
            let mapped = if idx < 96 { idx } else { idx - 32 };
            output[out_idx] = BASE96_ALPHABET[mapped % 96];
            out_idx += 1;
        }
    }
    
    // Handle remaining bits
    if bits > 0 && out_idx < output.len() {
        let idx = ((accumulator << (7 - bits)) & 0x7F) as usize;
        let mapped = if idx < 96 { idx } else { idx - 32 };
        output[out_idx] = BASE96_ALPHABET[mapped % 96];
        out_idx += 1;
    }
    
    out_idx
}

/// Decode Base96 string to bytes
pub fn base96_decode(encoded: &[u8], output: &mut [u8]) -> usize {
    if encoded.is_empty() || output.is_empty() {
        return 0;
    }
    
    let mut out_idx = 0;
    let mut accumulator: u64 = 0;
    let mut bits: u32 = 0;
    
    for &ch in encoded {
        // Find index in alphabet
        let idx = BASE96_ALPHABET.iter().position(|&c| c == ch);
        if let Some(val) = idx {
            accumulator = (accumulator << 7) | (val as u64);
            bits += 7;
            
            while bits >= 8 && out_idx < output.len() {
                bits -= 8;
                output[out_idx] = ((accumulator >> bits) & 0xFF) as u8;
                out_idx += 1;
            }
        }
    }
    
    out_idx
}

// ============================================================================
// BASE64 ENCODING (Minimum/Fallback)
// ============================================================================

/// Standard Base64 alphabet
pub const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode bytes to Base64
pub fn base64_encode(data: &[u8], output: &mut [u8]) -> usize {
    if data.is_empty() || output.is_empty() {
        return 0;
    }
    
    let mut out_idx = 0;
    
    for chunk in data.chunks(3) {
        if out_idx + 4 > output.len() {
            break;
        }
        
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        
        let triple = (b0 << 16) | (b1 << 8) | b2;
        
        output[out_idx] = BASE64_ALPHABET[((triple >> 18) & 0x3F) as usize];
        output[out_idx + 1] = BASE64_ALPHABET[((triple >> 12) & 0x3F) as usize];
        
        if chunk.len() > 1 {
            output[out_idx + 2] = BASE64_ALPHABET[((triple >> 6) & 0x3F) as usize];
        } else {
            output[out_idx + 2] = b'=';
        }
        
        if chunk.len() > 2 {
            output[out_idx + 3] = BASE64_ALPHABET[(triple & 0x3F) as usize];
        } else {
            output[out_idx + 3] = b'=';
        }
        
        out_idx += 4;
    }
    
    out_idx
}

/// Decode Base64 to bytes
pub fn base64_decode(encoded: &[u8], output: &mut [u8]) -> usize {
    if encoded.is_empty() || output.is_empty() {
        return 0;
    }
    
    let mut out_idx = 0;
    
    for chunk in encoded.chunks(4) {
        if chunk.len() < 4 || out_idx + 3 > output.len() {
            break;
        }
        
        let decode_char = |c: u8| -> u32 {
            match c {
                b'A'..=b'Z' => (c - b'A') as u32,
                b'a'..=b'z' => (c - b'a' + 26) as u32,
                b'0'..=b'9' => (c - b'0' + 52) as u32,
                b'+' => 62,
                b'/' => 63,
                _ => 0,
            }
        };
        
        let a = decode_char(chunk[0]);
        let b = decode_char(chunk[1]);
        let c = decode_char(chunk[2]);
        let d = decode_char(chunk[3]);
        
        let triple = (a << 18) | (b << 12) | (c << 6) | d;
        
        output[out_idx] = ((triple >> 16) & 0xFF) as u8;
        out_idx += 1;
        
        if chunk[2] != b'=' && out_idx < output.len() {
            output[out_idx] = ((triple >> 8) & 0xFF) as u8;
            out_idx += 1;
        }
        
        if chunk[3] != b'=' && out_idx < output.len() {
            output[out_idx] = (triple & 0xFF) as u8;
            out_idx += 1;
        }
    }
    
    out_idx
}

// ============================================================================
// TRIVARIATE CANONICAL FORMAT (Base96)
// ============================================================================

/// Trivariate canonical format: `triv:[SCH]_[CUID]_[UUID]`
///
/// ## Encoding
/// - **Base96**: Full fidelity canonical encoding
/// - **Minimum 64-bit**: Compact form extracted from 128-bit CUID
///
/// ## Structure
/// - SCH: 64 bits → 10 Base96 chars
/// - CUID: 128 bits → 20 Base96 chars (or 64-bit compact → 10 chars)
/// - UUID: 128 bits → 20 Base96 chars
///
/// ## Compact 64-bit Extraction
/// From 128-bit CUID, extract:
/// - Slots 0-1: Agent ID (16 bits)
/// - Slots 4-5: Sequence (16 bits)
/// - Slots 10-11: Delta angle (16 bits) ← CRITICAL
/// - Slots 12-13: Entropy (16 bits)
///
/// This gives a 64-bit "essence" that preserves the most important state.
pub struct TrivariateCanonical {
    /// Full canonical string buffer (Base96)
    buffer: [u8; 64],
    /// Length of valid data
    len: usize,
}

impl TrivariateCanonical {
    /// Create FULL canonical format from trivariate hash (Base96)
    ///
    /// Format: `triv:[SCH:10]_[CUID:20]_[UUID:20]` = 55 chars
    pub fn from_trivariate(triv: &TrivariateHash) -> Self {
        let mut buffer = [0u8; 64];
        let mut pos = 0;
        
        // Prefix: "triv:"
        buffer[0..5].copy_from_slice(b"triv:");
        pos = 5;
        
        // SCH in Base96 (64 bits → 10 chars)
        let sch_bytes = triv.sch.to_bytes();
        let sch_len = base96_encode(&sch_bytes, &mut buffer[pos..pos+12]);
        pos += sch_len;
        
        // Separator
        buffer[pos] = b'_';
        pos += 1;
        
        // CUID in Base96 (128 bits → 20 chars)
        let cuid_len = base96_encode(&triv.cuid.slots, &mut buffer[pos..pos+24]);
        pos += cuid_len;
        
        // Separator
        buffer[pos] = b'_';
        pos += 1;
        
        // UUID in Base96 (128 bits → 20 chars)
        let uuid_bytes = [
            (triv.uuid_hi >> 56) as u8, (triv.uuid_hi >> 48) as u8,
            (triv.uuid_hi >> 40) as u8, (triv.uuid_hi >> 32) as u8,
            (triv.uuid_hi >> 24) as u8, (triv.uuid_hi >> 16) as u8,
            (triv.uuid_hi >> 8) as u8, triv.uuid_hi as u8,
            (triv.uuid_lo >> 56) as u8, (triv.uuid_lo >> 48) as u8,
            (triv.uuid_lo >> 40) as u8, (triv.uuid_lo >> 32) as u8,
            (triv.uuid_lo >> 24) as u8, (triv.uuid_lo >> 16) as u8,
            (triv.uuid_lo >> 8) as u8, triv.uuid_lo as u8,
        ];
        let uuid_len = base96_encode(&uuid_bytes, &mut buffer[pos..pos+24]);
        pos += uuid_len;
        
        Self { buffer, len: pos }
    }
    
    /// Create COMPACT canonical format (64-bit minimum)
    ///
    /// Format: `trc:[SCH:10]_[CUID64:10]` = 24 chars
    ///
    /// Extracts the 64-bit "essence" from CUID:
    /// - Agent ID + Sequence + Delta Angle + Entropy
    pub fn compact(triv: &TrivariateHash) -> Self {
        let mut buffer = [0u8; 64];
        let mut pos = 0;
        
        // Prefix: "trc:" (trivariate compact)
        buffer[0..4].copy_from_slice(b"trc:");
        pos = 4;
        
        // SCH in Base96 (64 bits → 10 chars)
        let sch_bytes = triv.sch.to_bytes();
        let sch_len = base96_encode(&sch_bytes, &mut buffer[pos..pos+12]);
        pos += sch_len;
        
        // Separator
        buffer[pos] = b'_';
        pos += 1;
        
        // CUID 64-bit extract in Base96 (64 bits → 10 chars)
        let cuid64 = triv.cuid.extract_64();
        let cuid64_bytes = cuid64.to_be_bytes();
        let cuid_len = base96_encode(&cuid64_bytes, &mut buffer[pos..pos+12]);
        pos += cuid_len;
        
        Self { buffer, len: pos }
    }
    
    /// Get canonical string as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
    
    /// Get length
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 64-bit compact trivariate (minimum viable hash)
///
/// Used for:
/// - eBPF map keys (8 bytes)
/// - Fast comparison
/// - Network transmission
/// - Cache keys
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Trivariate64 {
    /// Compact 64-bit hash
    pub value: u64,
}

impl Trivariate64 {
    /// Create from full trivariate
    pub fn from_trivariate(triv: &TrivariateHash) -> Self {
        // XOR SCH with CUID extract for maximum entropy
        let sch64 = triv.sch.to_u64();
        let cuid64 = triv.cuid.extract_64();
        
        Self {
            value: sch64 ^ cuid64,
        }
    }
    
    /// Create from SCH only (when CUID not available)
    pub fn from_sch(sch: &SchHash) -> Self {
        Self {
            value: sch.to_u64(),
        }
    }
    
    /// Get as eBPF map key
    pub fn to_ebpf_key(&self) -> [u8; 8] {
        self.value.to_be_bytes()
    }
    
    /// Encode to Base96 (10 chars)
    pub fn to_base96(&self, output: &mut [u8; 12]) -> usize {
        base96_encode(&self.value.to_be_bytes(), output)
    }
}

// ============================================================================
// MURMUR3 HASH (for identity hash)
// ============================================================================

/// Murmur3 32-bit hash
pub fn murmur3_32(data: &[u8], seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;
    
    let mut hash = seed;
    let len = data.len();
    
    // Process 4-byte chunks
    let chunks = len / 4;
    for i in 0..chunks {
        let mut k = u32::from_le_bytes([
            data[i * 4],
            data[i * 4 + 1],
            data[i * 4 + 2],
            data[i * 4 + 3],
        ]);
        
        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);
        
        hash ^= k;
        hash = hash.rotate_left(R2);
        hash = hash.wrapping_mul(M).wrapping_add(N);
    }
    
    // Process remaining bytes
    let remaining = &data[chunks * 4..];
    if !remaining.is_empty() {
        let mut k = 0u32;
        for (i, &byte) in remaining.iter().enumerate() {
            k |= (byte as u32) << (i * 8);
        }
        
        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);
        
        hash ^= k;
    }
    
    // Finalization
    hash ^= len as u32;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;
    
    hash
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sch_runes() {
        let sch = SchHash::new(0x0123, 0x0456, 0x0789, 0x0ABC);
        let runes = sch.to_runes();
        
        // Verify runes are in correct ranges (each range is 0x100 = 256)
        // Domain: 0x0123 >> 4 = 0x12 = 18, so rune = 0xE000 + 18 = 0xE012
        assert!(runes[0] >= runes::DOMAIN_BASE && runes[0] < runes::DOMAIN_BASE + 0x100,
            "Domain rune {} not in range", runes[0]);
        assert!(runes[1] >= runes::EXECUTION_BASE && runes[1] < runes::EXECUTION_BASE + 0x100,
            "Execution rune {} not in range", runes[1]);
        assert!(runes[2] >= runes::NVNN_BASE && runes[2] < runes::NVNN_BASE + 0x100,
            "NVNN rune {} not in range", runes[2]);
        assert!(runes[3] >= runes::DELTA_ANGLE_BASE && runes[3] < runes::DELTA_ANGLE_BASE + 0x100,
            "Delta angle rune {} not in range", runes[3]);
    }
    
    #[test]
    fn test_cuid_delta_angle() {
        let mut cuid = CuidHash::new();
        cuid.set_delta_angle(0x1234);
        
        assert_eq!(cuid.get_delta_angle(), 0x1234);
        assert_eq!(cuid.slots[10], 0x12);
        assert_eq!(cuid.slots[11], 0x34);
    }
    
    #[test]
    fn test_tool_trigger_runes() {
        let trigger = ToolTrigger::NmapSynScan;
        let rune = trigger.to_rune();
        
        assert_eq!(rune, runes::TOOL_TRIGGER_BASE + 0x10);
        
        let parsed = ToolTrigger::from_rune(rune).unwrap();
        assert_eq!(parsed, ToolTrigger::NmapSynScan);
    }
    
    #[test]
    fn test_thalmic_annotation() {
        let ann = ThalmicAnnotation::new(64, 100);
        
        assert_eq!(ann.priority, 64);
        assert_eq!(ann.confidence, 100);
        assert!(!ann.should_suppress(50));
        
        let runes = ann.to_runes();
        let decoded = ThalmicAnnotation::from_runes(runes);
        
        assert_eq!(decoded.priority, ann.priority);
        assert_eq!(decoded.confidence, ann.confidence);
    }
    
    #[test]
    fn test_semantic_hash_from_text() {
        let sch = SchHash::from_semantic(
            b"cyber",
            b"hunt",
            b"target",
            b"scan",
            b"port",
            b"service",
            1000,
        );
        
        // Should produce consistent hashes
        let sch2 = SchHash::from_semantic(
            b"cyber",
            b"hunt",
            b"target",
            b"scan",
            b"port",
            b"service",
            1000,
        );
        
        // Copy to avoid packed struct alignment issues
        let d1 = { sch.domain };
        let d2 = { sch2.domain };
        let e1 = { sch.execution };
        let e2 = { sch2.execution };
        let n1 = { sch.nvnn };
        let n2 = { sch2.nvnn };
        
        assert_eq!(d1, d2);
        assert_eq!(e1, e2);
        assert_eq!(n1, n2);
    }
    
    #[test]
    fn test_trivariate_hash() {
        let sch = SchHash::new(0x1234, 0x5678, 0x9ABC, 0xDEF0);
        let mut cuid = CuidHash::new();
        cuid.set_delta_angle(0x1234);
        
        let triv = TrivariateHash::new(sch, cuid, 0xDEADBEEF, 0xCAFEBABE);
        
        let key = triv.to_ebpf_key();
        assert_eq!(key.len(), 16);
        
        // Check SCH bytes
        assert_eq!(key[0], 0x12);
        assert_eq!(key[1], 0x34);
        
        // Check CUID delta
        assert_eq!(key[8], 0x12);
        assert_eq!(key[9], 0x34);
    }
    
    #[test]
    fn test_base96_encode() {
        let data = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        let mut encoded = [0u8; 16];
        
        let enc_len = base96_encode(&data, &mut encoded);
        
        // Should produce non-empty output
        assert!(enc_len > 0);
        
        // All chars should be in alphabet
        for &ch in &encoded[..enc_len] {
            assert!(BASE96_ALPHABET.contains(&ch), "Invalid char: {}", ch);
        }
        
        // Same input = same output
        let mut encoded2 = [0u8; 16];
        let enc_len2 = base96_encode(&data, &mut encoded2);
        assert_eq!(enc_len, enc_len2);
        assert_eq!(&encoded[..enc_len], &encoded2[..enc_len2]);
    }
    
    #[test]
    fn test_base64_roundtrip() {
        let data = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE];
        let mut encoded = [0u8; 16];
        let mut decoded = [0u8; 16];
        
        let enc_len = base64_encode(&data, &mut encoded);
        assert!(enc_len > 0);
        
        let dec_len = base64_decode(&encoded[..enc_len], &mut decoded);
        assert!(dec_len > 0);
        
        // Should roundtrip correctly
        assert_eq!(&decoded[..data.len()], &data);
    }
    
    #[test]
    fn test_cuid_extract_64() {
        let mut cuid = CuidHash::new();
        
        // Set specific slots
        cuid.set_agent_id(0x1234);
        cuid.set_sequence(0x5678);
        cuid.set_delta_angle(0x9ABC);
        cuid.set_entropy(0xDEF0);
        
        let extracted = cuid.extract_64();
        
        // Verify extraction
        assert_eq!((extracted >> 48) & 0xFFFF, 0x1234); // Agent ID
        assert_eq!((extracted >> 32) & 0xFFFF, 0x5678); // Sequence
        assert_eq!((extracted >> 16) & 0xFFFF, 0x9ABC); // Delta
        assert_eq!(extracted & 0xFFFF, 0xDEF0);         // Entropy
    }
    
    #[test]
    fn test_trivariate64() {
        let sch = SchHash::new(0x1234, 0x5678, 0x9ABC, 0xDEF0);
        let mut cuid = CuidHash::new();
        cuid.set_agent_id(0xAAAA);
        cuid.set_sequence(0xBBBB);
        cuid.set_delta_angle(0xCCCC);
        cuid.set_entropy(0xDDDD);
        
        let triv = TrivariateHash::new(sch, cuid, 0, 0);
        let compact = Trivariate64::from_trivariate(&triv);
        
        // Should produce non-zero value
        assert_ne!(compact.value, 0);
        
        // Should be consistent
        let compact2 = Trivariate64::from_trivariate(&triv);
        assert_eq!(compact.value, compact2.value);
        
        // eBPF key should be 8 bytes
        let key = compact.to_ebpf_key();
        assert_eq!(key.len(), 8);
    }
    
    #[test]
    fn test_trivariate_canonical() {
        let sch = SchHash::new(0x1234, 0x5678, 0x9ABC, 0xDEF0);
        let mut cuid = CuidHash::new();
        cuid.set_delta_angle(0x1234);
        
        let triv = TrivariateHash::new(sch, cuid, 0xDEADBEEF, 0xCAFEBABE);
        
        // Full canonical
        let full = TrivariateCanonical::from_trivariate(&triv);
        let full_str = full.as_bytes();
        assert!(full_str.starts_with(b"triv:"));
        assert!(full.len() > 20);
        
        // Compact canonical
        let compact = TrivariateCanonical::compact(&triv);
        let compact_str = compact.as_bytes();
        assert!(compact_str.starts_with(b"trc:"));
        assert!(compact.len() < full.len()); // Compact should be shorter
    }
    
    #[test]
    fn test_delta_class() {
        let sch_none = SchHash::new(0, 0, 0, 0); // 0°
        let sch_micro = SchHash::new(0, 0, 0, 1820); // ~10°
        let sch_soft = SchHash::new(0, 0, 0, 4550); // ~25°
        let sch_hard = SchHash::new(0, 0, 0, 10920); // ~60°
        let sch_critical = SchHash::new(0, 0, 0, 16380); // ~90°
        
        let cuid = CuidHash::new();
        
        let t1 = TrivariateHash::new(sch_none, cuid, 0, 0);
        let t2 = TrivariateHash::new(sch_micro, cuid, 0, 0);
        let t3 = TrivariateHash::new(sch_soft, cuid, 0, 0);
        let t4 = TrivariateHash::new(sch_hard, cuid, 0, 0);
        let t5 = TrivariateHash::new(sch_critical, cuid, 0, 0);
        
        assert_eq!(t1.delta_class(), DeltaClass::None);
        assert_eq!(t2.delta_class(), DeltaClass::Micro);
        assert_eq!(t3.delta_class(), DeltaClass::Soft);
        assert_eq!(t4.delta_class(), DeltaClass::Hard);
        assert_eq!(t5.delta_class(), DeltaClass::Critical);
    }
    
    #[test]
    fn test_murmur3() {
        let hash = murmur3_32(b"test", 0);
        assert_ne!(hash, 0);
        
        // Same input = same hash
        let hash2 = murmur3_32(b"test", 0);
        assert_eq!(hash, hash2);
        
        // Different input = different hash
        let hash3 = murmur3_32(b"test2", 0);
        assert_ne!(hash, hash3);
    }
    
    #[test]
    fn test_ebpf_key() {
        let sch = SchHash::new(0x1234, 0x5678, 0x9ABC, 0xDEF0);
        let mut cuid = CuidHash::new();
        cuid.set_delta_angle(0xABCD);
        cuid.slots[12] = 0xEF;
        cuid.slots[13] = 0x01;
        
        let key = trivariate_to_ebpf_key(&sch, &cuid);
        
        assert_eq!(key.len(), 8);
        assert_eq!(key[0], 0x12); // domain high
        assert_eq!(key[1], 0x34); // domain low
        assert_eq!(key[4], 0xAB); // delta high
        assert_eq!(key[5], 0xCD); // delta low
    }
}

