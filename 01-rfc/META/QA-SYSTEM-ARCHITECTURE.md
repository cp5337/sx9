# SX9 QA System Architecture

**Recovered:** 2025-12-24  
**Status:** From conversation history  
**Source:** Multiple sessions on QA infrastructure

---

## Overview

Three-tier QA execution with SARIF output integration:

| Tier | Tool | Trigger | Latency | What It Catches |
|------|------|---------|---------|-----------------|
| **WASM** | `sx9-qa-quick.wasm` | Pre-commit hook | <100ms | LOC/complexity violations, pattern smells |
| **Local** | Serena + `sx9-phd-analyzer` | IDE save, RECON tick | 1-5s | Semantic issues, dead code, type misuse |
| **Container** | OrbStack `ctas7-qa-analyzer` | Hourly/on-demand | 30-300s | Full clippy pedantic, `cargo audit`, cross-crate |

---

## Three-Tier Execution Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        QA EXECUTION MODES                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐  │
│  │   WASM QUICK     │    │   LOCAL RUST     │    │   ORBSTACK       │  │
│  │   (Pre-commit)   │    │   (IDE Live)     │    │   (Deep Sweep)   │  │
│  ├──────────────────┤    ├──────────────────┤    ├──────────────────┤  │
│  │ • AST metrics    │    │ • Serena LSP     │    │ • Full clippy    │  │
│  │ • Pattern match  │    │ • Incremental    │    │ • Multi-crate    │  │
│  │ • <100ms target  │    │ • Semantic refs  │    │ • Security audit │  │
│  │ • No network     │    │ • Type-aware     │    │ • Scheduled/burst│  │
│  └────────┬─────────┘    └────────┬─────────┘    └────────┬─────────┘  │
│           │                       │                       │             │
│           └───────────────────────┼───────────────────────┘             │
│                                   ▼                                     │
│                    ┌──────────────────────────┐                         │
│                    │    RECON AGGREGATOR      │                         │
│                    │    (Folder State + QA)   │                         │
│                    └──────────────────────────┘                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Serena Integration

Serena provides semantic analysis beyond clippy:

```rust
// sx9-qa-core/src/serena_bridge.rs
pub struct SerenaAnalysis {
    pub dead_code: Vec<DeadSymbol>,
    pub unused_deps: Vec<String>,
    pub complexity_hotspots: Vec<ComplexityReport>,
    pub semantic_warnings: Vec<SemanticIssue>,
    pub refactor_risks: Vec<RefactorRisk>,
}

impl SerenaAnalysis {
    pub async fn analyze(workspace: &Path) -> Result<Self, SerenaError> {
        // Uses rust-analyzer protocol under the hood
        // Much deeper than clippy alone
    }
}
```

**What Serena Provides:**
- **Semantic understanding** - knows what a function *does*, not just syntax
- **Cross-file analysis** - tracks usage across modules
- **Refactor safety** - detects breaking changes before they happen
- **Intent matching** - can compare implementation against RFC spec

---

## Agent Guardrails

QA findings become **hard constraints** on agent behavior:

```rust
pub struct AgentGuardrails {
    // From QA analysis
    pub max_function_complexity: u32,  // Enforced by WASM pre-commit
    pub forbidden_patterns: Vec<Pattern>,  // Regex/AST patterns
    pub required_tests_for: Vec<PathMatcher>,  // Must have test coverage
    
    // From Serena
    pub no_modify_stable: Vec<PathBuf>,  // Semantic stability zones
    pub breaking_change_requires_rfc: bool,
}

impl AgentGuardrails {
    pub fn validate_diff(&self, diff: &Diff) -> ValidationResult {
        // Called before any agent-generated code is committed
    }
}
```

---

## Clippy Configuration

```yaml
# qa-config.yml
clippy:
  # Quick mode (IDE, pre-commit) - essential warnings only
  quick:
    deny: [clippy::correctness]
    warn: [clippy::suspicious]
    allow: [clippy::pedantic, clippy::nursery]
    timeout: 30s
    
  # Full mode (Orbstack sweep) - everything
  full:
    deny: [clippy::correctness, clippy::complexity]
    warn: [clippy::pedantic, clippy::nursery, clippy::cargo]
    timeout: 300s
    
  # Skip expensive checks when unchanged
  incremental: true
  cache_results: true
```

---

## OrbStack Container Architecture

### QA Analyzer Container

```yaml
# orbstack/compose.yml
services:
  qa-analyzer:
    image: ctas7-phd-analyzer:latest
    container_name: ctas7-qa-analyzer
    volumes:
      - /Users/cp5337/Developer/sx9:/workspace:ro
      - ./qa-results:/output
    environment:
      - SARIF_OUTPUT=true
      - CLIPPY_PEDANTIC=true
      - MULTI_CRATE=true
    command: ["sweep", "--workspace", "/workspace", "--format", "sarif"]
```

### Full Stack Compose

```yaml
services:
  # Kali Linux - Tool execution
  kali-executor:
    image: kalilinux/kali-rolling
    container_name: ctas-kali
    networks:
      - ctas-network
    volumes:
      - ./data/shared:/workspace
      - ./tools:/tools
    cap_add:
      - NET_ADMIN
      - NET_RAW

  # Purple Team - Analysis container
  purple-analyst:
    image: python:3.11-slim
    container_name: ctas-purple
    networks:
      - ctas-network
    volumes:
      - ./data/shared:/workspace
      - ./scripts:/scripts

  # CTAS Orchestrator
  ctas-orchestrator:
    build:
      context: ./docker/orchestrator
      dockerfile: Dockerfile
    container_name: ctas-orchestrator
    networks:
      - ctas-network
    volumes:
      - ./data/shared:/workspace
      - /var/run/docker.sock:/var/run/docker.sock
    depends_on:
      - kali-executor
      - purple-analyst

networks:
  ctas-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16
```

---

## Health/Heartbeat Systems

### 1. Foundation Health Network (Primary)

**Location:** `crates/sx9-foundation-core/src/health_network.rs`  
**Size:** 453 lines

Features:
- UDP multicast for efficient network communication
- Global health state tracking across all orchestrators
- Hash integrity verification
- TOML export for reporting
- Critical path routing

### 2. Plasma Defender Health (Security)

**Location:** `crates/sx9-plasma-defender/src/health.rs`  
**Size:** 30 lines (simplified)

---

## File Structure

```
sx9-qa-system/
├── crates/
│   ├── sx9-qa-core/          # Shared types, grading logic
│   ├── sx9-qa-wasm/          # WASM quick analyzer (pre-commit)
│   ├── sx9-qa-serena/        # Serena LSP bridge
│   └── sx9-phd-analyzer/     # Full AST analyzer
├── orbstack/
│   ├── compose.yml           # Multi-crate container analysis
│   ├── qa-config.yml         # Thresholds, patterns
│   └── Dockerfile
└── scripts/
    └── install-hooks.sh      # Git hooks for WASM pre-commit
```

---

## SARIF Output

All QA tools output SARIF format for:
- IDE integration
- CI/CD pipelines
- Security scanning aggregation
- Cross-tool correlation

---

## SmartCrate QA Integration

SmartCrate becomes center touch point for quality through foundation crates and gateway:

1. **Health Network** - Rust Foundation Core (453 lines)
2. **Plasma Defender** - Security layer (30 lines)
3. **Gateway/Ports** - Zero trust coordination

---

**Document Status:** RECOVERED  
**Recovery Date:** 2025-12-24
