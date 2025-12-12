//! PLASMA Delta Operator Module
//!
//! Implements Delta as a noise gating operator for toolchain escalation.
//! Provides delta measurement, gating, and diagnostics.

pub mod delta_gate;
pub mod delta_operator;
pub mod delta_tuner;
pub mod escalation_integration;

pub use delta_gate::{DeltaGate, DeltaGateConfig, GateMode, GatedPayload};
pub use delta_operator::{DeltaMeasurement, DeltaOperator};
pub use delta_tuner::{DeltaLogEntry, DeltaStatistics, DeltaTuner};
pub use escalation_integration::{
    EscalationContext, EscalationGateResult, EscalationIntegration, EscalationTier,
};

#[cfg(test)]
#[cfg(feature = "delta-tuner")]
mod tests;
