# RFC-9130: Unified Forge Pipeline Integration

**Status:** DRAFT  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-24  
**Supersedes:** Individual component specs  
**Integrates:** RFC-9112, RFC-9120, RFC-9121, RFC-9122, RFC-9127

---

## Abstract

RFC-9130 unifies the Forge pipeline into a single cohesive system. It integrates:
- **Prompt Forge v4** (RFC-9120) - Plain language → canonical prompts
- **Lightning QA** (RFC-9121) - Deterministic quality grading
- **Git Workflow** (RFC-9122) - Branch/PR/Slack gates
- **Architecture Compliance** (RFC-9127) - ECS/TCR enforcement
- **Canonical Pattern Discovery** - Pattern matching sidecar

**Core Principle:** One pipeline, deterministic flow, no manual intervention between gates.

---

## 1. Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        UNIFIED FORGE PIPELINE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Stage 1: PROMPT FORGE (RFC-9120)                                          │
│  ├─ Plain Language Input                                                   │
│  ├─ Thalmic Filter (clarity ≥ 0.7)                                         │
│  ├─ Pattern Resolution                                                     │
│  └─ Canonical Prompt Output                                                │
│              │                                                              │
│              ▼                                                              │
│  Stage 2: CODE GENERATION                                                   │
│  ├─ Factory Agent (Claude Sonnet)                                          │
│  ├─ N-V-N-N Grammar Enforcement                                            │
│  └─ Crate Scaffold Generation                                              │
│              │                                                              │
│              ▼                                                              │
│  Stage 3a: ARCHITECTURE CHECK (RFC-9127)                                    │
│  ├─ Legion ECS Compliance                                                  │
│  ├─ TCR (Type-Crate Registry) Validation                                   │
│  ├─ Rune/Slot Type Enforcement                                             │
│  └─ Forbidden Pattern Detection (Bevy, etc.)                               │
│              │                                                              │
│              ▼                                                              │
│  Stage 3b: LIGHTNING QA (RFC-9121)                                         │
│  ├─ AST Parsing (syn for Rust)                                             │
│  ├─ Metrics Calculation (complexity, cohesion)                             │
│  ├─ TETH Anti-Pattern Detection                                            │
│  └─ Grade Assignment (A-F)                                                 │
│              │                                                              │
│              ▼                                                              │
│  Stage 4: GIT WORKFLOW (RFC-9122)                                          │
│  ├─ Branch: factory/SX9-*                                                  │
│  ├─ PR Creation with QA Report                                             │
│  ├─ Slack Decision Gate                                                    │
│  └─ Merge to develop → main                                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Quality Gates

| Stage | Gate | Pass Criteria |
|-------|------|---------------|
| **Forge Output** | Thalmic Score | ≥ 0.7 clarity |
| **Architecture** | Layer Compliance | 0 violations |
| **Architecture** | TCR Compliance | 0 violations |
| **Architecture** | Rune/Slot Types | 100% correct |
| **Lightning QA** | Grade | ≥ B (70+) |
| **Git Push** | CI Tests | All pass |
| **PR Review** | Slack Approval | ✓ Approved |

---

## 3. Rejection Actions

| Violation | Action | Retry Allowed |
|-----------|--------|---------------|
| Bevy import | REJECT | Yes, after fix |
| Wrong layer usage | REFACTOR | Yes, with directive |
| Rune as String | REJECT | Yes, after fix |
| Grade F | HALT | No, manual review |
| CI failure | BLOCK | Yes, after fix |

---

## 4. Docker Compose Deployment

```yaml
version: '3.8'

services:
  forge-static-qa:
    image: sx9/static-qa:latest
    volumes:
      - ./crates:/workspace:ro
      - ./work:/work
    environment:
      - QA_CONFIG=/work/forge.config.toml

  forge-arch-check:
    image: sx9/arch-check:latest
    volumes:
      - ./crates:/workspace:ro
      - ./work:/work
    depends_on:
      - forge-static-qa

  forge-pattern-agent:
    image: sx9/pattern-agent:latest
    volumes:
      - ./sx9-canonical:/canonical:ro
      - ./work:/work
    depends_on:
      - forge-arch-check

  forge-aggregator:
    image: sx9/qa-aggregate:latest
    volumes:
      - ./work:/work
    depends_on:
      - forge-pattern-agent
```

---

## 5. Harness v5 Structure

```
forge-unified-v5/
├── forge-pipeline.sh        # Main runner
├── forge.toml               # Configuration
├── canonical/               # N-V-N-N pattern blocks
│   ├── lifecycle.nvnn
│   ├── communication.nvnn
│   └── security.nvnn
├── gates/                   # QA gates (Python)
│   ├── static_gate.py
│   ├── arch_gate.py
│   ├── pattern_gate.py
│   └── aggregator.py
├── schemas/                 # JSON contracts
│   ├── qa-report.schema.json
│   └── crate-manifest.schema.json
├── agents/                  # Prompt specs
│   ├── factory-agent.md
│   └── refactor-agent.md
├── integrations/
│   ├── RFC-9122-GIT-WORKFLOW-LINEAR-SLACK.md
│   └── LINEAR-SLACK-PLAN.md
└── harness/                 # TypeScript harness (for Rust port)
    ├── executor.ts
    ├── actions.ts
    ├── middleware.ts
    ├── validators.ts
    ├── types.ts
    ├── reducer.ts
    ├── selectors.ts
    ├── graphActions.ts
    └── graphCRUD.ts
```

---

## 6. Configuration

### forge.config.toml

```toml
[forge]
version = "4.0"
mode = "supervised"

[thalmic]
min_clarity = 0.7
reject_on_ambiguity = true

[factory]
max_refactor_attempts = 3
temperature = 0.0
persona = "FORGE"

[qa]
min_grade = "B"
parallel_gates = true

[qa.weights]
structure = 0.25
complexity = 0.25
pattern = 0.25
arch = 0.25

[qa.thresholds]
A = 85
B = 70
C = 55
D = 40

[arch]
ecs_backend = "legion"
async_layer = "apecs"
bevy_forbidden = true
rune_type = "u32"
slot_type = "u64"

[pattern]
canonical_registry = "./sx9-canonical"
min_confidence = 0.7
require_nvnn_header = true

[git]
branch_prefix = "factory/SX9-"
require_slack_approval = true
target_branch = "develop"
```

---

## 7. Rust Port Mapping

| TypeScript File | Rust Module | Purpose |
|-----------------|-------------|---------|
| executor.ts | executor.rs | Pipeline orchestration |
| actions.ts | actions.rs | Redux-like actions |
| middleware.ts | middleware.rs | Pre/post hooks |
| validators.ts | validators.rs | Schema validation |
| types.ts | types.rs | Core types |
| reducer.rs | reducer.rs | State management |
| selectors.ts | selectors.rs | State queries |
| graphActions.ts | graph.rs | Graph operations |
| graphCRUD.ts | graph_crud.rs | CRUD for graph |

---

## 8. Integration with Cole's Harness

Cole's proven patterns adopted:

1. **Two-Agent Architecture**
   - Initializer Agent: Creates META issue, plans work
   - Coder Agent: Executes tasks, reports progress

2. **META Issue Handoff**
   - Session state in Linear issue comments
   - Enables resume after interruption

3. **Slack Visibility**
   - Real-time notifications
   - Interactive approval buttons

---

## 9. Success Criteria

| Metric | Target |
|--------|--------|
| Pipeline latency (Stage 1-4) | < 5 minutes |
| False positive rate (pattern match) | < 10% |
| Auto-refactor success rate | > 70% |
| Human intervention rate | < 20% |
| Grade B+ pass rate | > 80% |

---

## 10. RFC Cross-Reference

```
RFC-9112 (Deterministic Prompts)
    └─► N-V-N-N grammar, agent prompts
    
RFC-9120 (Prompt Forge v4)
    └─► Stage 1: Intent capture, Thalmic filter
    
RFC-9121 (Lightning QA)
    └─► Stage 3: Aggregator, grading
    
RFC-9122 (Git Workflow)
    └─► Stage 4: Branch/PR/Slack
    
RFC-9127 (Arch Compliance)
    └─► Stage 3a: ECS/TCR validation
    
RFC-9130 (THIS RFC)
    └─► Unified pipeline, integration spec
```

---

**Document Status:** DRAFT  
**Recovery Date:** 2025-12-24  
**Source:** Claude conversation history
