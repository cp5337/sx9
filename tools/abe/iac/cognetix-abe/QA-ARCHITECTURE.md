# SX9 Quality Assurance Architecture

**Version:** 1.0.0
**Date:** 2025-11-27
**Status:** CANONICAL
**RFC:** RFC-9103 (IAC Adaptive Infrastructure)

---

## Executive Summary

This document consolidates all QA systems under the **ABE IAC** umbrella. The fragmentation problem is solved by establishing a clear hierarchy with Lightning QA as the canonical entry point.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    SX9 QA ARCHITECTURE (CANONICAL)                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LAYER 1: ENTRY POINT                                                    │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │  ABE Lightning QA Engine (Port 18109)                              │ │
│  │  04-abe-iac/abe-qa-system/lightning-qa-engine/                     │ │
│  │  • GPU-accelerated script analysis                                 │ │
│  │  • PR automation                                                   │ │
│  │  • Entry point for all QA operations                               │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                              │                                           │
│                              ▼                                           │
│  LAYER 2: SPECIALIZED ANALYZERS                                          │
│  ┌──────────────────────┐  ┌──────────────────────┐  ┌────────────────┐ │
│  │ ctas7-phd-analyzer   │  │ Zencoder Expert QA   │  │ Claude Meta    │ │
│  │ (Rust/WASM)          │  │ (Code Review)        │  │ (AI Agents)    │ │
│  │ ctas7-qa-analyzer/   │  │ zencoder-expert-qa/  │  │ claude-meta-   │ │
│  │                      │  │                      │  │ agents/        │ │
│  │ • PTCC Entropy       │  │ • Deep code review   │  │ • AI-driven    │ │
│  │ • Tesla 200 LOC      │  │ • Expert patterns    │  │   validation   │ │
│  │ • NVNN compliance    │  │ • Security analysis  │  │ • Prompt QA    │ │
│  │ • Certification      │  │                      │  │                │ │
│  └──────────────────────┘  └──────────────────────┘  └────────────────┘ │
│                              │                                           │
│                              ▼                                           │
│  LAYER 3: RESULTS & REPORTING                                            │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │  Unified Results Store                                             │ │
│  │  qa-analysis-results/  (CANONICAL)                                 │ │
│  │  • JSON analysis reports                                           │ │
│  │  • TOML quality metrics                                            │ │
│  │  • XSD schema generation                                           │ │
│  │  • Strip reports (synopsis + quality)                              │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                              │                                           │
│                              ▼                                           │
│  LAYER 4: INTEGRATION                                                    │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │  Statistical CDN (Port 18108) ← Metrics aggregation                │ │
│  │  Linear Integration ← Issue/PR creation                            │ │
│  │  IAC Manifolds ← Infrastructure triggers                           │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Canonical Directories

### ACTIVE (Use These)

| Directory | Purpose | Port |
|-----------|---------|------|
| `04-abe-iac/abe-qa-system/lightning-qa-engine/` | Lightning QA daemon | 18109 |
| `04-abe-iac/abe-qa-system/zencoder-expert-qa/` | Expert code review | — |
| `04-abe-iac/abe-qa-system/claude-meta-agents/` | AI agent QA | — |
| `04-abe-iac/abe-qa-system/linear-integration/` | Linear PR automation | — |
| `04-abe-iac/abe-qa-system/iac-manifolds/` | Terraform triggers | — |
| `ctas7-qa-analyzer/` | Rust PHD analyzer | — |
| `qa-analysis-results/` | Unified results store | — |

### NOT QA SYSTEMS (Corrected Naming)

| Directory | Actual Purpose | Status |
|-----------|----------------|--------|
| `ctas7-ops-main-platform/DSL-orchestration/` | Crate grouping, service orchestration, port management | ACTIVE (renamed from XSD-QA-5) |

> **Note:** "DSL-QA-5" was a legacy name. It's actually an **orchestration system** for crate grouping and service management, not quality assurance. Renamed to `DSL-orchestration`.

### DEPRECATED (Do Not Use)

| Directory | Reason | Action |
|-----------|--------|--------|
| `qa-results-*` (timestamped) | Scattered dumps | ✅ Consolidated |
| `qa-logs/` | Old logs | Archive |
| `qa-results/` | Ambiguous | ✅ Removed |
| `qa-results-local/` | Local testing | Keep for dev only |

---

## Component Details

### 1. Lightning QA Engine (LAYER 1)

**Location:** `04-abe-iac/abe-qa-system/lightning-qa-engine/`
**Port:** 18109
**Language:** Python (FastAPI)

```python
# Entry points
POST /analyze/crate/{crate_name}  # Trigger analysis
GET  /results/{crate_name}        # Fetch results
GET  /health                      # Health check
POST /register                    # Register with Port Manager
```

**Capabilities:**
- GPU-accelerated analysis (CUDA if available)
- Non-invasive script-only analysis
- PR candidate generation
- Integration with Statistical CDN

### 2. PHD Analyzer (LAYER 2 - Rust)

**Location:** `ctas7-qa-analyzer/`
**Binaries:**
- `ctas7-phd-analyzer` - Main analyzer
- `ctas7_certify` - Certification checks
- `strip_reporter` - Report generation
- `line_by_line_analyzer` - Detailed analysis
- `clone_checker` - Duplicate detection

**Metrics:**
- PTCC Entropy Score (target: >0.7)
- Tesla 200 LOC compliance
- NVNN comment density
- Complexity scoring
- Security indicators

### 3. Zencoder Expert QA (LAYER 2)

**Location:** `04-abe-iac/abe-qa-system/zencoder-expert-qa/`
**Language:** Python

Expert-level code review with pattern matching for:
- Security vulnerabilities
- Performance anti-patterns
- Architectural violations

### 4. Claude Meta Agents (LAYER 2)

**Location:** `04-abe-iac/abe-qa-system/claude-meta-agents/`
**Language:** Python

AI-driven validation using Claude for:
- Prompt quality assessment
- Code explanation validation
- Documentation completeness

---

## Results Schema

**Location:** `qa-analysis-results/`

```
qa-analysis-results/
├── {crate-name}.json              # Full analysis JSON
├── {crate-name}-analysis.json     # Detailed metrics
├── strip-reports/
│   ├── {crate-name}_synopsis.md   # Human-readable summary
│   ├── {crate-name}_quality.toml  # Machine-readable metrics
│   └── {crate-name}_quality.dsl   # DSL schema definition
└── lightning-qa-results/          # From Lightning QA daemon
    └── {crate-name}_analysis.json
```

### Quality Metrics Schema

```toml
[quality]
ptcc_entropy = 0.75
tesla_compliance = 0.85
nvnn_density = 0.92
complexity_score = 45.3
loc_count = 1847
file_count = 23

[security]
critical = 0
high = 2
medium = 5
low = 12

[recommendation]
grade = "B+"
pr_candidates = 3
```

---

## Integration Points

### Port Manager Registration

```json
{
  "service_name": "lightning-qa-engine",
  "port": 18109,
  "layer": "quality_assurance",
  "capabilities": ["gpu_analysis", "script_only", "pr_automation"],
  "health_endpoint": "/health"
}
```

### Statistical CDN Reporting

```json
{
  "service": "lightning-qa-engine",
  "layer": 1,
  "result": {
    "crate_name": "ctas7-foundation-core",
    "overall_grade": "A-",
    "analysis_time_seconds": 2.5
  }
}
```

### Linear Integration

PR candidates from QA analysis are automatically:
1. Created as Linear issues
2. Prioritized by severity
3. Assigned to relevant team members

---

## Migration Plan

### Phase 1: Consolidate Results (Immediate)
```bash
# Move scattered results to canonical location
mv qa-results/* qa-analysis-results/
mv qa-results-*/*.json qa-analysis-results/archive/
```

### Phase 2: Rename Misnamed Systems (DONE)
```bash
# XSD-QA-5 was NOT a QA system - it's orchestration
# Renamed to DSL-orchestration
mv ctas7-ops-main-platform/XSD-QA-5 ctas7-ops-main-platform/DSL-orchestration
```

### Phase 3: Update References (This Week)
- Update all scripts to use `qa-analysis-results/`
- Update Lightning QA results path
- Update PHD analyzer output path

---

## Usage Examples

### Run Full QA Pipeline

```bash
# 1. Start Lightning QA daemon
cd 04-abe-iac/abe-qa-system/lightning-qa-engine
python src/lightning_qa_daemon.py

# 2. Trigger analysis
curl -X POST http://localhost:18109/analyze/crate/ctas7-foundation-core

# 3. Fetch results
curl http://localhost:18109/results/ctas7-foundation-core
```

### Run PHD Analyzer Directly

```bash
cd ctas7-qa-analyzer
cargo run --bin ctas7-phd-analyzer -- --target ../ctas7-foundation-core
```

### Generate Certification Report

```bash
cargo run --bin ctas7_certify -- --crate ctas7-foundation-core --output qa-analysis-results/
```

---

## NVNN Comment Standard

All QA code must follow NVNN discipline (1 comment per 20 lines):

```rust
// Analyzer calculates entropy score via PTCC algorithm
// System validates LOC count against Tesla threshold
// Reporter generates synopsis from quality metrics
```

---

## Related RFCs

- **RFC-9103:** IAC Adaptive Infrastructure (defines QA layer)
- **RFC-9101:** Smart Crate System (quality thresholds)
- **RFC-9200:** SX9 Development Center (ABE QA integration)

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-11-27 | Created canonical QA architecture | CTAS Engineering |
| 2025-11-27 | Defined 4-layer hierarchy | CTAS Engineering |
| 2025-11-27 | Marked deprecated directories | CTAS Engineering |
