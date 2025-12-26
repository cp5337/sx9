// SYSTEM_WRITE_IDEMPOTENT_RECORD
// system write record idempotent

/// Canonical Pattern Block (read-only)
/// Constraints: deterministic, no IO, no logging, no config, single responsibility.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordKey(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordValue(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteDecision {
    NoOpSameValue,
    UpsertNewValue,
}

pub fn decide_idempotent_write(existing: Option<&RecordValue>, proposed: &RecordValue) -> WriteDecision {
    match existing {
        Some(v) if v == proposed => WriteDecision::NoOpSameValue,
        _ => WriteDecision::UpsertNewValue,
    }
}
