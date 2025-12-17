//! RFC-9003 Operational Classification & Escalation Semantics
//!
//! Defines the taxonomy for classifying Nodes, Crates, and Operations within the
//! CTAS-7.3.1 ecosystem. This provides the "Labeling" layer for the OODA loop.

use serde::{Deserialize, Serialize};

/// Operational Class (RFC-9003 Section 2.1)
/// Determines the primary mission of a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationalClass {
    /// Intelligence gathering and analysis (Passive)
    Intelligence,
    /// Defensive actions and protection (Active/Reactive)
    Defensive,
    /// Offensive capabilities and disruption (Active/Proactive)
    Offensive,
    /// Administrative and orchestration functions (Support)
    Administrative,
    /// Logistics and resource management (Support)
    Logistics,
}

impl OperationalClass {
    pub fn is_kinetic(&self) -> bool {
        matches!(self, Self::Defensive | Self::Offensive)
    }

    pub fn to_code(&self) -> &'static str {
        match self {
            Self::Intelligence => "INT",
            Self::Defensive => "DEF",
            Self::Offensive => "OFF",
            Self::Administrative => "ADM",
            Self::Logistics => "LOG",
        }
    }
}

/// Escalation Tier (RFC-9003 Section 3.2)
/// Defines the execution environment complexity and privilege level.
/// Maps loosely to `trivariate_hash::ExecEnv`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EscalationTier {
    /// Tier 0: WebAssembly Microkernel (Sandboxed, ephemeral)
    Wasm = 0,
    /// Tier 1: Microkernel Task (Managed, limited scope)
    Microkernel = 1,
    /// Tier 2: Kernel Crate (System level, highly privileged)
    KernelCrate = 2,
    /// Tier 3: Multi-Crate Ensemble (Coordinated group)
    MultiCrate = 3,
    /// Tier 4: Container (Docker/OrbStack, full OS isolation)
    Container = 4,
    /// Tier 5: Firefly / Orbital (Exotic runtimes)
    Firefly = 5,
    /// Tier 6: Orb (Planetary scale)
    Orb = 6,
}

impl EscalationTier {
    pub fn requires_containerization(&self) -> bool {
        matches!(self, Self::Container | Self::Firefly | Self::Orb)
    }
}

/// Node Classification Frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationFrame {
    pub op_class: OperationalClass,
    pub tier: EscalationTier,
    pub clearance_level: String, // e.g. "TS//SI"
    pub mission_code: String,
}

impl Default for ClassificationFrame {
    fn default() -> Self {
        Self {
            op_class: OperationalClass::Administrative,
            tier: EscalationTier::Wasm,
            clearance_level: "UNCLASSIFIED".to_string(),
            mission_code: "GENERAL".to_string(),
        }
    }
}
