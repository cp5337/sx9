//! PTCC Primitives Module
//!
//! RFC-9100: PTCC (Primary Tactical Cognitive Compute) Primitives
//!
//! 32 Universal Primitives mapped to Unicode D1 Class (U+E400-E41F)
//! These form the atomic operations for the CTAS-7 cognitive engine.

use serde::{Deserialize, Serialize};

/// Unicode Private Use Area base offset for PTCC primitives
const PUA_BASE_OFFSET: u32 = 0xE400;

/// PTCC Primitive Opcodes (RFC-9100)
///
/// 32 atomic operations that form the basis of all CTAS-7 cognitive operations.
/// Each maps to a unique Unicode codepoint in the D1 Class range (U+E400-E41F).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Primitive {
    // CRUD Operations (0x00-0x03)
    /// Create new entity or resource
    Create = 0x00,
    /// Read/query existing entity
    Read = 0x01,
    /// Update existing entity
    Update = 0x02,
    /// Delete entity (requires approval per RFC-9003)
    Delete = 0x03,

    // Data Operations (0x04-0x07)
    /// Transform data format
    Transform = 0x04,
    /// Validate data integrity
    Validate = 0x05,
    /// Compress data payload
    Compress = 0x06,
    /// Encrypt data (AES-GCM)
    Encrypt = 0x07,

    // Network Operations (0x08-0x0B)
    /// Route message to destination
    Route = 0x08,
    /// Broadcast to all nodes
    Broadcast = 0x09,
    /// Subscribe to event stream
    Subscribe = 0x0A,
    /// Publish event to stream
    Publish = 0x0B,

    // Analysis Operations (0x0C-0x0F)
    /// Analyze pattern in data
    Analyze = 0x0C,
    /// Correlate multiple sources
    Correlate = 0x0D,
    /// Score threat level
    Score = 0x0E,
    /// Predict future state
    Predict = 0x0F,

    // Cognitive Operations (0x10-0x13) - OODA Loop
    /// Observe phase - gather data
    Observe = 0x10,
    /// Orient phase - analyze context
    Orient = 0x11,
    /// Decide phase - select action
    Decide = 0x12,
    /// Act phase - execute action
    Act = 0x13,

    // Control Operations (0x14-0x17)
    /// Lock resource (exclusive access)
    Lock = 0x14,
    /// Unlock resource
    Unlock = 0x15,
    /// Spawn new process/agent
    Spawn = 0x16,
    /// Terminate process/agent
    Terminate = 0x17,

    // State Operations (0x18-0x1B)
    /// Checkpoint current state
    Checkpoint = 0x18,
    /// Restore from checkpoint
    Restore = 0x19,
    /// Synchronize distributed state
    Sync = 0x1A,
    /// Merge conflicting states
    Merge = 0x1B,

    // Meta Operations (0x1C-0x1F)
    /// Measure performance
    Measure = 0x1C,
    /// Log event to audit trail
    Log = 0x1D,
    /// Alert human operator
    Alert = 0x1E,
    /// No operation (heartbeat)
    Noop = 0x1F,
}

impl Primitive {
    /// Convert primitive to Unicode route character
    ///
    /// Maps to Private Use Area D1 Class (U+E400-E41F)
    pub fn to_unicode_route(&self) -> char {
        char::from_u32(PUA_BASE_OFFSET + *self as u32).unwrap_or('\u{FFFD}')
    }

    /// Get Unicode codepoint as hex string
    pub fn to_unicode_hex(&self) -> String {
        format!("U+{:04X}", PUA_BASE_OFFSET + *self as u32)
    }

    /// Create primitive from opcode value
    pub fn from_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            0x00 => Some(Primitive::Create),
            0x01 => Some(Primitive::Read),
            0x02 => Some(Primitive::Update),
            0x03 => Some(Primitive::Delete),
            0x04 => Some(Primitive::Transform),
            0x05 => Some(Primitive::Validate),
            0x06 => Some(Primitive::Compress),
            0x07 => Some(Primitive::Encrypt),
            0x08 => Some(Primitive::Route),
            0x09 => Some(Primitive::Broadcast),
            0x0A => Some(Primitive::Subscribe),
            0x0B => Some(Primitive::Publish),
            0x0C => Some(Primitive::Analyze),
            0x0D => Some(Primitive::Correlate),
            0x0E => Some(Primitive::Score),
            0x0F => Some(Primitive::Predict),
            0x10 => Some(Primitive::Observe),
            0x11 => Some(Primitive::Orient),
            0x12 => Some(Primitive::Decide),
            0x13 => Some(Primitive::Act),
            0x14 => Some(Primitive::Lock),
            0x15 => Some(Primitive::Unlock),
            0x16 => Some(Primitive::Spawn),
            0x17 => Some(Primitive::Terminate),
            0x18 => Some(Primitive::Checkpoint),
            0x19 => Some(Primitive::Restore),
            0x1A => Some(Primitive::Sync),
            0x1B => Some(Primitive::Merge),
            0x1C => Some(Primitive::Measure),
            0x1D => Some(Primitive::Log),
            0x1E => Some(Primitive::Alert),
            0x1F => Some(Primitive::Noop),
            _ => None,
        }
    }

    /// Create primitive from Unicode character
    pub fn from_unicode(c: char) -> Option<Self> {
        let code = c as u32;
        if code >= PUA_BASE_OFFSET && code <= PUA_BASE_OFFSET + 0x1F {
            Self::from_opcode((code - PUA_BASE_OFFSET) as u8)
        } else {
            None
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Primitive::Create => "CREATE",
            Primitive::Read => "READ",
            Primitive::Update => "UPDATE",
            Primitive::Delete => "DELETE",
            Primitive::Transform => "TRANSFORM",
            Primitive::Validate => "VALIDATE",
            Primitive::Compress => "COMPRESS",
            Primitive::Encrypt => "ENCRYPT",
            Primitive::Route => "ROUTE",
            Primitive::Broadcast => "BROADCAST",
            Primitive::Subscribe => "SUBSCRIBE",
            Primitive::Publish => "PUBLISH",
            Primitive::Analyze => "ANALYZE",
            Primitive::Correlate => "CORRELATE",
            Primitive::Score => "SCORE",
            Primitive::Predict => "PREDICT",
            Primitive::Observe => "OBSERVE",
            Primitive::Orient => "ORIENT",
            Primitive::Decide => "DECIDE",
            Primitive::Act => "ACT",
            Primitive::Lock => "LOCK",
            Primitive::Unlock => "UNLOCK",
            Primitive::Spawn => "SPAWN",
            Primitive::Terminate => "TERMINATE",
            Primitive::Checkpoint => "CHECKPOINT",
            Primitive::Restore => "RESTORE",
            Primitive::Sync => "SYNC",
            Primitive::Merge => "MERGE",
            Primitive::Measure => "MEASURE",
            Primitive::Log => "LOG",
            Primitive::Alert => "ALERT",
            Primitive::Noop => "NOOP",
        }
    }

    /// Get category for this primitive
    pub fn category(&self) -> PrimitiveCategory {
        match *self as u8 >> 2 {
            0 => PrimitiveCategory::Crud,
            1 => PrimitiveCategory::Data,
            2 => PrimitiveCategory::Network,
            3 => PrimitiveCategory::Analysis,
            4 => PrimitiveCategory::Cognitive,
            5 => PrimitiveCategory::Control,
            6 => PrimitiveCategory::State,
            7 => PrimitiveCategory::Meta,
            _ => PrimitiveCategory::Meta,
        }
    }

    /// Check if this primitive requires approval (RFC-9003)
    pub fn requires_approval(&self) -> bool {
        matches!(
            self,
            Primitive::Delete | Primitive::Lock | Primitive::Terminate | Primitive::Alert
        )
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.to_unicode_hex())
    }
}

/// Primitive categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveCategory {
    /// CRUD operations (Create, Read, Update, Delete)
    Crud,
    /// Data transformation operations
    Data,
    /// Network/messaging operations
    Network,
    /// Analysis and intelligence operations
    Analysis,
    /// OODA cognitive loop operations
    Cognitive,
    /// Control and lifecycle operations
    Control,
    /// State management operations
    State,
    /// Meta/administrative operations
    Meta,
}

impl PrimitiveCategory {
    /// Get all primitives in this category
    pub fn primitives(&self) -> &'static [Primitive] {
        match self {
            PrimitiveCategory::Crud => &[
                Primitive::Create,
                Primitive::Read,
                Primitive::Update,
                Primitive::Delete,
            ],
            PrimitiveCategory::Data => &[
                Primitive::Transform,
                Primitive::Validate,
                Primitive::Compress,
                Primitive::Encrypt,
            ],
            PrimitiveCategory::Network => &[
                Primitive::Route,
                Primitive::Broadcast,
                Primitive::Subscribe,
                Primitive::Publish,
            ],
            PrimitiveCategory::Analysis => &[
                Primitive::Analyze,
                Primitive::Correlate,
                Primitive::Score,
                Primitive::Predict,
            ],
            PrimitiveCategory::Cognitive => &[
                Primitive::Observe,
                Primitive::Orient,
                Primitive::Decide,
                Primitive::Act,
            ],
            PrimitiveCategory::Control => &[
                Primitive::Lock,
                Primitive::Unlock,
                Primitive::Spawn,
                Primitive::Terminate,
            ],
            PrimitiveCategory::State => &[
                Primitive::Checkpoint,
                Primitive::Restore,
                Primitive::Sync,
                Primitive::Merge,
            ],
            PrimitiveCategory::Meta => &[
                Primitive::Measure,
                Primitive::Log,
                Primitive::Alert,
                Primitive::Noop,
            ],
        }
    }
}

/// Tactical Instruction combining primitive with operands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalInstruction {
    /// The primitive operation
    pub primitive: Primitive,
    /// Source operand (hash reference)
    pub source: Option<String>,
    /// Destination operand (hash reference)
    pub destination: Option<String>,
    /// Additional parameters
    pub params: Vec<String>,
    /// Execution priority (0-255, higher = more urgent)
    pub priority: u8,
    /// Requires human approval
    pub approval_required: bool,
}

impl TacticalInstruction {
    /// Create new instruction
    pub fn new(primitive: Primitive) -> Self {
        Self {
            approval_required: primitive.requires_approval(),
            primitive,
            source: None,
            destination: None,
            params: Vec::new(),
            priority: 128, // Default medium priority
        }
    }

    /// Set source operand
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set destination operand
    pub fn with_destination(mut self, dest: impl Into<String>) -> Self {
        self.destination = Some(dest.into());
        self
    }

    /// Add parameter
    pub fn with_param(mut self, param: impl Into<String>) -> Self {
        self.params.push(param.into());
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Encode to Unicode instruction string
    pub fn to_unicode(&self) -> String {
        let mut result = String::new();
        result.push(self.primitive.to_unicode_route());
        // Encode priority in next byte
        result.push(char::from_u32(0xE420 + self.priority as u32 / 16).unwrap_or('\u{E420}'));
        result
    }
}

/// All 32 primitives as array (for iteration)
pub const ALL_PRIMITIVES: [Primitive; 32] = [
    Primitive::Create,
    Primitive::Read,
    Primitive::Update,
    Primitive::Delete,
    Primitive::Transform,
    Primitive::Validate,
    Primitive::Compress,
    Primitive::Encrypt,
    Primitive::Route,
    Primitive::Broadcast,
    Primitive::Subscribe,
    Primitive::Publish,
    Primitive::Analyze,
    Primitive::Correlate,
    Primitive::Score,
    Primitive::Predict,
    Primitive::Observe,
    Primitive::Orient,
    Primitive::Decide,
    Primitive::Act,
    Primitive::Lock,
    Primitive::Unlock,
    Primitive::Spawn,
    Primitive::Terminate,
    Primitive::Checkpoint,
    Primitive::Restore,
    Primitive::Sync,
    Primitive::Merge,
    Primitive::Measure,
    Primitive::Log,
    Primitive::Alert,
    Primitive::Noop,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_unicode_mapping() {
        assert_eq!(Primitive::Create.to_unicode_route(), '\u{E400}');
        assert_eq!(Primitive::Read.to_unicode_route(), '\u{E401}');
        assert_eq!(Primitive::Noop.to_unicode_route(), '\u{E41F}');
    }

    #[test]
    fn test_primitive_from_opcode() {
        assert_eq!(Primitive::from_opcode(0x00), Some(Primitive::Create));
        assert_eq!(Primitive::from_opcode(0x10), Some(Primitive::Observe));
        assert_eq!(Primitive::from_opcode(0x20), None);
    }

    #[test]
    fn test_primitive_from_unicode() {
        assert_eq!(Primitive::from_unicode('\u{E400}'), Some(Primitive::Create));
        assert_eq!(Primitive::from_unicode('\u{E413}'), Some(Primitive::Act));
        assert_eq!(Primitive::from_unicode('A'), None);
    }

    #[test]
    fn test_approval_requirements() {
        assert!(Primitive::Delete.requires_approval());
        assert!(Primitive::Lock.requires_approval());
        assert!(!Primitive::Read.requires_approval());
        assert!(!Primitive::Analyze.requires_approval());
    }

    #[test]
    fn test_all_primitives_count() {
        assert_eq!(ALL_PRIMITIVES.len(), 32);
    }

    #[test]
    fn test_category_assignment() {
        assert_eq!(Primitive::Create.category(), PrimitiveCategory::Crud);
        assert_eq!(Primitive::Observe.category(), PrimitiveCategory::Cognitive);
        assert_eq!(Primitive::Noop.category(), PrimitiveCategory::Meta);
    }
}
