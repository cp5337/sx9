# RFC-9141: FORGE Assembly Line & Dual-Heartbeat QA Doctrine

**Status:** CANONICAL
**Author:** Charles E. Payne / Claude
**Date:** 2025-12-24
**Supersedes:** N/A
**Integrates:** RFC-9140, RFC-9100, RFC-9025, RFC-9050, RFC-9060, RFC-9112

---

## Abstract

RFC-9141 defines the FORGE assembly line as a deterministic, variable-driven production system that transforms human intent into traceable, testable, and auditable software artifacts. Central to this architecture is a dual-heartbeat quality assurance model that separates structural correctness from semantic intent alignment.

This document establishes how prompts are assembled, not written, and how quality is enforced through independent, non-overlapping verification planes.

---

## 1. Purpose and Scope

FORGE is not an LLM interface. It is a software manufacturing line.

The assembly line ensures that:

- 90% of prompt content derives from system-selected variables
- Human input supplies intent, not structure
- Outputs are reproducible, inspectable, and attributable
- Quality failures are caught early, deterministically, and explainably

This RFC applies to:

- All FORGE prompt generation
- All QA enforcement
- All canonical code generation paths

---

## 2. Assembly Line Overview

The FORGE assembly line consists of four irreversible stages:

```
Intent → Variable Selection → Prompt Assembly → Artifact Production
```

At no point does a human free-write a production prompt.

### 2.1 Intent Ingress

Intent enters the system via:

- Linear issues
- Slack mentions
- NATS requests
- Voice transcription

Intent is unstructured by design.

### 2.2 Variable Selection

The system resolves intent into a bounded set of variables:

| Variable Category | Examples |
|-------------------|----------|
| Lifecycle phase | Design, Model, Build, Validate, Operate |
| Task type | Factory Task, Core Task |
| PTCC primitive envelope | Encoded implicitly |
| Canonical pattern anchors | N-V-N-N headers |
| Target scope | Language, crate, subsystem |
| Quality gates | Static, Semantic, Pattern |

**Invariant:** Variable selection must be deterministic and replayable.

### 2.3 Prompt Assembly

Prompts are assembled from:

- Canonical narrative paragraphs (SDC lifecycle prose)
- Node interview templates
- Crate interview templates
- Selected primitives (implicitly encoded)
- Pattern constraints

Humans do not author prompts. They approve or adjust variables.

### 2.4 Artifact Production

Artifacts include:

- Source code
- Tests
- Schemas
- Documentation
- Configurations

Every artifact is:

- Traceable to intent
- Linked to variables
- Subject to QA heartbeats

---

## 3. Dual-Heartbeat QA Doctrine

Quality assurance in FORGE is enforced through two independent heartbeats.

**Non-negotiable constraints:**

- They must never merge
- They must never override each other
- They must never share failure semantics

---

## 4. Heartbeat A: Static QA (Cold Truth)

### 4.1 Purpose

Static QA answers a single question:

> "Is this structurally correct?"

It evaluates what exists, not what was intended.

### 4.2 Properties

| Property | Value |
|----------|-------|
| Deterministic | Yes |
| Read-only | Yes |
| Network isolated | Yes |
| Negotiable | No |

If Static QA fails, the pipeline stops.

### 4.3 Components

| Component | Function |
|-----------|----------|
| `sx9-ast` | AST extraction |
| `sx9-mccabe` | Cyclomatic complexity analysis |
| `sx9-rules` | Static rule enforcement |

**Execution mode:**
```yaml
read_only: true
network_mode: none
```

### 4.4 Outputs

Static QA produces:

- Structural facts
- Complexity scores
- Rule violations

Outputs are written to `/work/static`.

These outputs are immutable inputs to Semantic QA.

---

## 5. Heartbeat B: Semantic QA (Warm Annotation)

### 5.1 Purpose

Semantic QA answers a different question:

> "Is this still what we meant to build?"

It evaluates intent alignment, not correctness.

### 5.2 Properties

| Property | Value |
|----------|-------|
| Interpretive | Yes |
| Context-aware | Yes |
| Graph-driven | Yes |
| Advisory | Yes (never silent) |

Semantic QA may warn, gate, or escalate—but never auto-pass.

### 5.3 Components

| Component | Function |
|-----------|----------|
| `sx9-semantic` | Semantic analysis engine |
| `SEMANTIC_INTENT_AGENT` | Intent matching agent |
| Neo4j graphs | Intent and observation storage |

### 5.4 Inputs

Semantic QA consumes:

- Static QA outputs
- Repository content
- Canonical pattern snippets (N-V-N-N)
- Node and crate interviews

### 5.5 Outputs

Semantic QA produces:

- Intent alignment scores
- Drift annotations
- Pattern deviation warnings
- Remediation suggestions

---

## 6. Canonical Pattern Snippets

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

## 7. Intent vs Drift Detection

Two graphs are maintained:

### Intent Graph

- **Source:** Node interviews
- **Encodes:** Roles, constraints, expectations

### Observed Graph

- **Source:** Code and structure
- **Encodes:** Dependencies, behavior

Drift is detected via:

- Role inversion
- Constraint erosion
- Coupling growth
- Responsibility bleed

This produces Δ-angle signals used by governance gates.

---

## 8. Relationship to PTCC Primitives

PTCC primitives are never exposed in QA output.

They are:

- Encoded implicitly in prose
- Reconstructed by machines
- Used for traceability and replay

**Invariant:** Humans read narratives. Machines read primitives.

---

## 9. Governance and Escalation

| Condition | Response |
|-----------|----------|
| Static QA failure | Hard stop |
| Semantic QA drift | Warn or gate |
| Repeated drift | Governance escalation |

All actions are logged and attributable.

---

## 10. Invariants

1. Prompts are assembled, not authored
2. Variable selection precedes generation
3. Static QA is absolute
4. Semantic QA is advisory but visible
5. Canonical patterns anchor meaning
6. Intent must remain traceable post-deployment

---

## 11. References

- RFC-9140 — FORGE Unified Architecture
- RFC-9100 — PTCC Primitives
- RFC-9025 — Node Interview Schema
- RFC-9050 — QA Two-Heartbeat System
- RFC-9060 — Agent Memory Architecture
- RFC-9112 — Deterministic Prompt Engineering

---

**Document Status:** CANONICAL
**Next:** RFC-9142 (Semantic Drift Scoring & Gates)
