# SX9 Harness — Doctrine + Performance Test Bundle

> Enforces: SDC §6–§11, CLSGS, Annex A.2/A.3/A.4
> Authority: RFC-9141, RFC-9142

## What this is
Test suite validating CLSGS Annex implementation in `sx9-harness`:
- **doctrine_unit.rs** - Unit tests for N-V-N-N types, drift signals, governance gates
- **doctrine_smoke.rs** - End-to-end tests (Linear → QA → Lineage → Governance)
- **perf_smoke.rs** - Performance smoke tests with external budget enforcement
- **benches/perf.rs** - Criterion benchmarks for micro-performance

## Test Coverage

### Annex A.2 (Agentic Folder Structures)
- BehavioralScope N-V-N-N field validation
- Annotation string generation/parsing
- Scope validity checks

### Annex A.3 (Linear Task Integration)
- IntentAnchor mapping to behavioral regions
- LinearQaSignal static/semantic QA outcomes
- Task progression gating on QA failure

### Annex A.4 (Git Universe Alignment)
- LineageTracker annotation parsing
- Annotation loss detection
- Scope expansion detection
- PR boundary (approval ≠ alignment)

### RFC-9142 (Governance)
- DriftSignal structure and vectors
- GovernanceGate escalation levels
- NATS subject namespace validation

## Run Tests

### Unit + Smoke
```bash
cargo test -p sx9-harness
```

### Benchmarks
```bash
cargo bench -p sx9-harness
```

### Perf Smoke (budget tripwire)
```bash
export SX9_PERF_BUDGETS_PATH=./crates/sx9-harness/perf-budgets.example.json
cargo test -p sx9-harness --test perf_smoke -- --nocapture
```

## Cargo Dependencies
Ensure these are in `[dev-dependencies]`:
```toml
criterion = "0.5"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
```

## Performance Budgets
The `perf-budgets.example.json` file defines threshold values:
- `nvnn_parse_mean_ns` - N-V-N-N parsing budget
- `lineage_scan_per_marker_mean_ns` - Per-marker lineage analysis
- `drift_eval_mean_ns` - Drift evaluation budget
- `nats_subject_build_mean_ns` - Subject string construction
- `linear_gate_eval_mean_ns` - Task gating decision time

Adjust values based on target hardware and requirements.
