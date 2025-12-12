//! Threat Reaction Module - Recognize-Formulate-React Architecture
//!
//! Implements the three-phase threat response system:
//! 1. RECOGNIZE: Threat detection and correlation
//! 2. FORMULATE: Response planning and playbook generation
//! 3. REACT: Execution via escalation continuum

pub mod escalation_planner;
pub mod formulate;
pub mod glaf_correlation;
pub mod interdiction_analyzer;
pub mod pattern_discovery;
pub mod react;
pub mod recognize;

pub use escalation_planner::*;
pub use formulate::*;
pub use glaf_correlation::*;
pub use interdiction_analyzer::*;
pub use pattern_discovery::*;
pub use react::*;
pub use recognize::*;
