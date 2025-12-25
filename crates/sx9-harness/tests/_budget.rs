// Enforces: SDC §6–§11, CLSGS, Annex A.2/A.3/A.4
// Authority: RFC-9141, RFC-9142

#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct PerfBudgets {
    /// Max allowed mean parse time for N-V-N-N (ns)
    pub nvnn_parse_mean_ns: u64,
    /// Max allowed mean lineage scan per marker (ns)
    pub lineage_scan_per_marker_mean_ns: u64,
    /// Max allowed mean drift eval time (ns)
    pub drift_eval_mean_ns: u64,
    /// Max allowed mean subject build time (ns)
    pub nats_subject_build_mean_ns: u64,
    /// Max allowed mean task gating decision time (ns)
    pub linear_gate_eval_mean_ns: u64,
}

fn budgets_path_from_env() -> Option<PathBuf> {
    env::var("SX9_PERF_BUDGETS_PATH").ok().map(PathBuf::from)
}

pub fn load_perf_budgets() -> Result<PerfBudgets> {
    let path = budgets_path_from_env().ok_or_else(|| {
        anyhow!(
            "SX9_PERF_BUDGETS_PATH is not set. Provide a JSON budgets file to run perf tests."
        )
    })?;

    let bytes = fs::read(&path).with_context(|| format!("Failed to read budgets file: {:?}", path))?;
    let budgets: PerfBudgets = serde_json::from_slice(&bytes)
        .with_context(|| format!("Failed to parse budgets JSON: {:?}", path))?;

    Ok(budgets)
}
