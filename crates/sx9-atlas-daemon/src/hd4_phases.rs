//! HD4 Phases and Vertical Levels
//!
//! RFC-9020: HD4 Phase Definitions
//! Hunt -> Detect -> Disable -> Disrupt -> Dominate

use serde::{Deserialize, Serialize};

/// HD4 Operational Phases (RFC-9020)
///
/// The five phases of threat response:
/// - Hunt: Proactive search for precursor activities
/// - Detect: Identify indicators and initial convergence
/// - Disable: Remove capability (requires approval per RFC-9003)
/// - Disrupt: Break adversary tempo
/// - Dominate: Control battlespace (terminal goal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HD4Phase {
    /// Q Hunt - Phase 1: Proactive Search
    Hunt,
    /// O Detect - Phase 2: Identify Indicators
    Detect,
    /// D Disable - Phase 3: Remove Capability (Strict Approval Required)
    Disable,
    /// D Disrupt - Phase 4: Break Tempo
    Disrupt,
    /// D Dominate - Phase 5: Control Battlespace
    Dominate,
}

impl HD4Phase {
    /// Get next phase in HD4 progression
    pub fn next(&self) -> Self {
        match self {
            HD4Phase::Hunt => HD4Phase::Detect,
            HD4Phase::Detect => HD4Phase::Disable,
            HD4Phase::Disable => HD4Phase::Disrupt,
            HD4Phase::Disrupt => HD4Phase::Dominate,
            HD4Phase::Dominate => HD4Phase::Dominate, // Terminal
        }
    }

    /// Check if this phase requires strict approval (RFC-9003)
    pub fn requires_approval(&self) -> bool {
        matches!(
            self,
            HD4Phase::Disable | HD4Phase::Disrupt | HD4Phase::Dominate
        )
    }

    /// Get phase index (0-4)
    pub fn index(&self) -> u8 {
        match self {
            HD4Phase::Hunt => 0,
            HD4Phase::Detect => 1,
            HD4Phase::Disable => 2,
            HD4Phase::Disrupt => 3,
            HD4Phase::Dominate => 4,
        }
    }

    /// Create from index
    pub fn from_index(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(HD4Phase::Hunt),
            1 => Some(HD4Phase::Detect),
            2 => Some(HD4Phase::Disable),
            3 => Some(HD4Phase::Disrupt),
            4 => Some(HD4Phase::Dominate),
            _ => None,
        }
    }

    /// Get phase code (Q, O, D, D, D)
    pub fn code(&self) -> char {
        match self {
            HD4Phase::Hunt => 'Q',
            HD4Phase::Detect => 'O',
            HD4Phase::Disable | HD4Phase::Disrupt | HD4Phase::Dominate => 'D',
        }
    }
}

impl std::fmt::Display for HD4Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Vertical Escalation Levels (RFC-9003)
///
/// Determines the scope and authority level:
/// - Tactical: Local execution, minimal approval
/// - Operational: Regional coordination
/// - Strategic: Enterprise-wide impact
/// - National: Multi-organization coordination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalLevel {
    /// Local execution level
    Tactical,
    /// Regional coordination level
    Operational,
    /// Enterprise-wide level
    Strategic,
    /// Multi-organization level
    National,
}

impl VerticalLevel {
    /// Get next escalation level
    pub fn escalate(&self) -> Self {
        match self {
            VerticalLevel::Tactical => VerticalLevel::Operational,
            VerticalLevel::Operational => VerticalLevel::Strategic,
            VerticalLevel::Strategic => VerticalLevel::National,
            VerticalLevel::National => VerticalLevel::National, // Maximum
        }
    }

    /// Get de-escalation level
    pub fn deescalate(&self) -> Self {
        match self {
            VerticalLevel::National => VerticalLevel::Strategic,
            VerticalLevel::Strategic => VerticalLevel::Operational,
            VerticalLevel::Operational => VerticalLevel::Tactical,
            VerticalLevel::Tactical => VerticalLevel::Tactical, // Minimum
        }
    }

    /// Get level index (0-3)
    pub fn index(&self) -> u8 {
        match self {
            VerticalLevel::Tactical => 0,
            VerticalLevel::Operational => 1,
            VerticalLevel::Strategic => 2,
            VerticalLevel::National => 3,
        }
    }

    /// Get approval threshold multiplier
    pub fn approval_threshold(&self) -> f64 {
        match self {
            VerticalLevel::Tactical => 0.75,
            VerticalLevel::Operational => 0.85,
            VerticalLevel::Strategic => 0.95,
            VerticalLevel::National => 0.99,
        }
    }
}

impl std::fmt::Display for VerticalLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hd4_progression() {
        assert_eq!(HD4Phase::Hunt.next(), HD4Phase::Detect);
        assert_eq!(HD4Phase::Detect.next(), HD4Phase::Disable);
        assert_eq!(HD4Phase::Dominate.next(), HD4Phase::Dominate);
    }

    #[test]
    fn test_hd4_approval() {
        assert!(!HD4Phase::Hunt.requires_approval());
        assert!(!HD4Phase::Detect.requires_approval());
        assert!(HD4Phase::Disable.requires_approval());
    }

    #[test]
    fn test_vertical_escalation() {
        assert_eq!(
            VerticalLevel::Tactical.escalate(),
            VerticalLevel::Operational
        );
        assert_eq!(VerticalLevel::National.escalate(), VerticalLevel::National);
    }
}
