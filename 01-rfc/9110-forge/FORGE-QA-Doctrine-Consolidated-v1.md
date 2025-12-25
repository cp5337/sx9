# FORGE QA Doctrine: Assembly Line & Semantic Drift Detection

**Consolidated Reference Document**
**Version:** 1.0
**Date:** 2025-12-24
**Sources:** RFC-9141, RFC-9142, OrbStack Canonical QA Bundle v1.1
**Status:** For further refinement

---

## Document Purpose

This document consolidates the FORGE assembly line architecture, dual-heartbeat QA doctrine, semantic drift detection, and governance gate specifications into a single reference. It is intended for technical review and refinement prior to final canonization.

---

# Part I: FORGE Assembly Line Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         FORGE ASSEMBLY LINE                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌──────────┐    ┌──────────────────┐    ┌────────────────┐    ┌────────┐ │
│   │  INTENT  │───▶│ VARIABLE SELECT  │───▶│ PROMPT ASSEMBLY│───▶│ARTIFACT│ │
│   │  INGRESS │    │  (Deterministic) │    │  (Narrative)   │    │PRODUCE │ │
│   └──────────┘    └──────────────────┘    └────────────────┘    └────────┘ │
│        │                   │                      │                  │      │
│        ▼                   ▼                      ▼                  ▼      │
│   ┌─────────┐         ┌─────────┐           ┌─────────┐        ┌─────────┐ │
│   │ Linear  │         │Lifecycle│           │   SDC   │        │  Code   │ │
│   │ Slack   │         │  Phase  │           │Narrative│        │  Tests  │ │
│   │ NATS    │         │Task Type│           │Templates│        │ Schemas │ │
│   │ Voice   │         │  PTCC   │           │Patterns │        │  Docs   │ │
│   └─────────┘         │ N-V-N-N │           └─────────┘        └─────────┘ │
│                       └─────────┘                                           │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Core Principle

FORGE is not an LLM interface. It is a software manufacturing line.

- 90% of prompt content derives from system-selected variables
- Human input supplies intent, not structure
- Outputs are reproducible, inspectable, and attributable
- Quality failures are caught early, deterministically, and explainably

## Assembly Stages

### Stage 1: Intent Ingress

Intent enters via:
- Linear issues
- Slack mentions
- NATS requests
- Voice transcription

Intent is unstructured by design.

### Stage 2: Variable Selection

The system resolves intent into bounded variables:

| Category | Purpose |
|----------|---------|
| Lifecycle phase | Design, Model, Build, Validate, Operate |
| Task type | Factory Task (15) or Core Task (39) |
| PTCC primitive envelope | 32 primitives, implicitly encoded |
| Canonical pattern anchors | N-V-N-N headers |
| Target scope | Language, crate, subsystem |
| Quality gates | Static, Semantic, Pattern |

**Invariant:** Variable selection must be deterministic and replayable.

### Stage 3: Prompt Assembly

Prompts are assembled from:
- Canonical narrative paragraphs (SDC lifecycle prose)
- Node interview templates
- Crate interview templates
- Selected primitives (implicitly encoded)
- Pattern constraints

**Invariant:** Humans do not author prompts. They approve or adjust variables.

### Stage 4: Artifact Production

Every artifact is:
- Traceable to intent
- Linked to variables
- Subject to QA heartbeats

---

# Part II: Dual-Heartbeat QA Doctrine

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        DUAL-HEARTBEAT QA SYSTEM                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                    HEARTBEAT A: STATIC QA                            │   │
│   │                       (Cold Truth)                                   │   │
│   │                                                                      │   │
│   │   read_only: true    network_mode: none    DETERMINISTIC            │   │
│   │                                                                      │   │
│   │   ┌──────────┐   ┌────────────┐   ┌───────────┐                     │   │
│   │   │ sx9-ast  │   │ sx9-mccabe │   │ sx9-rules │                     │   │
│   │   │   AST    │   │ Complexity │   │  Static   │                     │   │
│   │   │Extract   │   │  Analysis  │   │   Rules   │                     │   │
│   │   └──────────┘   └────────────┘   └───────────┘                     │   │
│   │                          │                                           │   │
│   │                          ▼                                           │   │
│   │                    /work/static (immutable)                          │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                 │                                            │
│                                 ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                   HEARTBEAT B: SEMANTIC QA                           │   │
│   │                      (Warm Annotation)                               │   │
│   │                                                                      │   │
│   │   INTERPRETIVE    CONTEXT-AWARE    GRAPH-DRIVEN    ADVISORY         │   │
│   │                                                                      │   │
│   │   ┌──────────────┐   ┌─────────────────────┐   ┌─────────────────┐  │   │
│   │   │ sx9-semantic │   │SEMANTIC_INTENT_AGENT│   │  Neo4j Graphs   │  │   │
│   │   │   Engine     │   │   Intent Matching   │   │ Intent/Observed │  │   │
│   │   └──────────────┘   └─────────────────────┘   └─────────────────┘  │   │
│   │                                                                      │   │
│   │   Outputs: Intent scores, Drift annotations, Deviation warnings     │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Non-Negotiable Constraints

1. Heartbeats must never merge
2. Heartbeats must never override each other
3. Heartbeats must never share failure semantics

## Heartbeat A: Static QA (Cold Truth)

**Question answered:** "Is this structurally correct?"

| Property | Value |
|----------|-------|
| Deterministic | Yes |
| Read-only | Yes |
| Network isolated | Yes |
| Negotiable | No |

**If Static QA fails, the pipeline stops.**

### Components

| Component | Function |
|-----------|----------|
| `sx9-ast` | AST extraction |
| `sx9-mccabe` | Cyclomatic complexity |
| `sx9-rules` | Static rule enforcement |

### Execution Mode

```yaml
read_only: true
network_mode: none
```

### Outputs

Written to `/work/static` (immutable):
- Structural facts
- Complexity scores
- Rule violations

## Heartbeat B: Semantic QA (Warm Annotation)

**Question answered:** "Is this still what we meant to build?"

| Property | Value |
|----------|-------|
| Interpretive | Yes |
| Context-aware | Yes |
| Graph-driven | Yes |
| Advisory | Yes (never silent) |

**Semantic QA may warn, gate, or escalate—but never auto-pass.**

### Components

| Component | Function |
|-----------|----------|
| `sx9-semantic` | Semantic analysis engine |
| `SEMANTIC_INTENT_AGENT` | Intent matching agent |
| Neo4j graphs | Intent and observation storage |

### Inputs

- Static QA outputs
- Repository content
- Canonical pattern snippets (N-V-N-N)
- Node and crate interviews

### Outputs

- Intent alignment scores
- Drift annotations
- Pattern deviation warnings
- Remediation suggestions

---

# Part III: Semantic Drift Detection

## Conceptual Model

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SEMANTIC DRIFT MODEL                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌─────────────────────────┐         ┌─────────────────────────┐           │
│   │      INTENT GRAPH       │         │     OBSERVED GRAPH      │           │
│   │                         │         │                         │           │
│   │  Source:                │         │  Source:                │           │
│   │  - Node interviews      │         │  - AST structure        │           │
│   │  - Lifecycle narratives │         │  - Call graphs          │           │
│   │  - Pattern declarations │         │  - Data flow            │           │
│   │                         │         │  - Service boundaries   │           │
│   │  Encodes:               │         │                         │           │
│   │  - Roles                │         │  Encodes:               │           │
│   │  - Responsibilities     │         │  - Actual components    │           │
│   │  - Constraints          │         │  - Functions            │           │
│   │  - Expected interactions│         │  - Services             │           │
│   │                         │         │  - Real dependencies    │           │
│   └───────────┬─────────────┘         └───────────┬─────────────┘           │
│               │                                   │                          │
│               └──────────────┬────────────────────┘                          │
│                              │                                               │
│                              ▼                                               │
│                    ┌─────────────────┐                                       │
│                    │  DRIFT VECTORS  │                                       │
│                    │                 │                                       │
│                    │  - Role         │                                       │
│                    │  - Constraint   │                                       │
│                    │  - Coupling     │                                       │
│                    │  - Authority    │                                       │
│                    │  - Pattern      │                                       │
│                    └────────┬────────┘                                       │
│                             │                                                │
│                             ▼                                                │
│                    ┌─────────────────┐                                       │
│                    │   Δ-ANGLE       │                                       │
│                    │   SCORING       │                                       │
│                    └────────┬────────┘                                       │
│                             │                                                │
│                             ▼                                                │
│                    ┌─────────────────┐                                       │
│                    │ GOVERNANCE GATE │                                       │
│                    └─────────────────┘                                       │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Drift Vectors

Drift is not binary. It is directional.

| Vector | Description |
|--------|-------------|
| Role Drift | Component assumes undeclared responsibilities |
| Constraint Drift | Stated limits erode or disappear |
| Coupling Drift | Dependencies exceed expectations |
| Authority Drift | Decision-making migrates unexpectedly |
| Pattern Drift | N-V-N-N patterns structurally present but behaviorally violated |

## Drift Scoring

| Score | Interpretation |
|-------|----------------|
| 0.0 | No drift |
| 0.3 | Minor deviation |
| 0.6 | Significant misalignment |
| 0.8+ | Governance concern |

Scores are non-aggregated by default. Aggregation is a governance decision.

## Δ-Angle Interpretation

| Angle Range | Response |
|-------------|----------|
| 0°–15° | Acceptable evolution |
| 15°–45° | Monitor |
| 45°–90° | Investigate |
| 90°+ | Escalate |

Δ-angle measures drift toward:
- Complexity
- Centralization
- Opacity

---

# Part IV: Governance Gates

## Gate Levels

| Level | Action |
|-------|--------|
| Observe | Log only |
| Warn | Annotate PR / Issue |
| Gate | Require human acknowledgment |
| Escalate | Block release pending review |

## Trigger Conditions

Gates may trigger on:
- Sustained drift over time
- Sudden high-magnitude drift
- Drift in safety-critical roles
- Drift across multiple vectors

## Escalation Matrix

| Condition | Response |
|-----------|----------|
| Static QA failure | Hard stop |
| Semantic QA drift | Warn or gate |
| Repeated drift | Governance escalation |

## Human-in-the-Loop Doctrine

Semantic drift resolution is always human-adjudicated.

**System responsibilities:**
- Detect
- Score
- Explain

**Human responsibilities:**
- Decide
- Correct
- Accept risk

---

# Part V: Canonical Pattern Snippets

Canonical pattern snippets act as semantic anchors, not templates.

| Snippet | Domain |
|---------|--------|
| `USER_VALIDATE_INPUT_SECURITY.rs` | Validation |
| `SYSTEM_WRITE_IDEMPOTENT_RECORD.rs` | Persistence |
| `WORKER_PROCESS_TASK_BOUNDED.rs` | Concurrency |
| `SERVICE_ROTATE_TOKEN_SECURE.rs` | Security |
| `RESOURCE_CLOSE_GRACEFUL.rs` | Lifecycle |
| `SERVICE_ADAPTER_IO_WRAPPER.rs` | Design |

Each snippet defines:
- Role
- Action
- Constraint
- Object

Semantic QA evaluates behavioral fidelity, not pattern names.

---

# Part VI: OrbStack Deployment Architecture

## Stack Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         ORBSTACK QA DEPLOYMENT                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  ORBSTACK A: STATIC QA (Cold Truth)                                  │   │
│   │  Container: sx9/static-qa:latest                                     │   │
│   │  Mode: read_only: true, network_mode: none                           │   │
│   │  Commands: sx9-ast, sx9-mccabe, sx9-rules                            │   │
│   │  Output: /work/static                                                │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                 │                                            │
│                                 ▼ (immutable inputs)                         │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  ORBSTACK B: SEMANTIC QA (Warm Annotation)                           │   │
│   │  Container: sx9/semantic:latest                                      │   │
│   │  Agent: SEMANTIC_INTENT_AGENT                                        │   │
│   │  Inputs: /work/static, /repo                                         │   │
│   │  Output: /work/semantic                                              │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                 │                                            │
│                                 ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  ORBSTACK C: DISCOVERY (Hot Intelligence)                            │   │
│   │  Agent: CANONICAL_DISCOVERY_AGENT                                    │   │
│   │  Agent: GLAF_ANNOTATION_AGENT                                        │   │
│   │  Output: pattern.matches.json, graph.delta.json                      │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                 │                                            │
│                                 ▼                                            │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  NEO4J: GLAF Graph Storage                                           │   │
│   │  Schema: LoadSet → File → Function → CanonicalPattern → Finding      │   │
│   │  Port: 7474 (HTTP), 7687 (Bolt)                                      │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Execution Order

```bash
# 1. Put target repo at ./work/repo
# 2. Run stacks in order:
./orbstack-static/run.sh      # Cold truth
./orbstack-semantic/run.sh    # Warm annotation
./orbstack-discovery/run.sh   # Hot intelligence
```

---

# Part VII: Invariants

## RFC-9141 Invariants

1. Prompts are assembled, not authored
2. Variable selection precedes generation
3. Static QA is absolute
4. Semantic QA is advisory but visible
5. Canonical patterns anchor meaning
6. Intent must remain traceable post-deployment

## RFC-9142 Invariants

1. Intent is immutable post-Design
2. Drift is directional, not binary
3. Semantic QA never auto-approves
4. Governance gates are auditable
5. Patterns define meaning, not aesthetics
6. Drift without visibility is failure

---

# Part VIII: PTCC Primitive Integration

PTCC primitives are never exposed in QA output.

They are:
- Encoded implicitly in prose
- Reconstructed by machines
- Used for traceability and replay

**Invariant:** Humans read narratives. Machines read primitives.

---

# Part IX: References

| RFC | Title |
|-----|-------|
| RFC-9140 | FORGE Unified Architecture |
| RFC-9141 | FORGE Assembly Line & Dual-Heartbeat QA |
| RFC-9142 | Semantic Drift Scoring & Governance Gates |
| RFC-9025 | Node Interview Schema |
| RFC-9050 | QA Two-Heartbeat System |
| RFC-9060 | Agent Memory Architecture |
| RFC-9100 | PTCC Primitives |
| RFC-9112 | Deterministic Prompt Engineering |

---

**Document Status:** For Further Refinement
**Generated:** 2025-12-24
