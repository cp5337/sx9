// SERVICE_ADAPTER_IO_WRAPPER
// service adapt io wrapper

/// Canonical Pattern Block (read-only)
/// Constraints: deterministic, no IO, no logging, no config, single responsibility.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterPlan {
    pub reads_env: bool,
    pub writes_logs: bool,
    pub does_io: bool,
}

/// Canonical adapter: declare side effects explicitly (so patterns remain pure).
pub fn describe_adapter_plan(reads_env: bool, writes_logs: bool, does_io: bool) -> AdapterPlan {
    AdapterPlan { reads_env, writes_logs, does_io }
}
