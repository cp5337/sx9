//! PLASMA Delta Operator Module
//!
//! Implements Delta as a noise gating operator for toolchain escalation.
//! Provides delta measurement, gating, and diagnostics.

pub mod delta_operator;
pub mod delta_gate;
pub mod delta_tuner;
pub mod escalation_integration;

pub use delta_operator::{DeltaOperator, DeltaMeasurement};
pub use delta_gate::{DeltaGate, DeltaGateConfig, GateMode, GatedPayload};
pub use delta_tuner::{DeltaTuner, DeltaLogEntry, DeltaStatistics};
pub use escalation_integration::{
    EscalationTier, EscalationIntegration, EscalationContext, EscalationGateResult,
};

#[cfg(test)]
#[cfg(feature = "delta-tuner")]
mod tests;

