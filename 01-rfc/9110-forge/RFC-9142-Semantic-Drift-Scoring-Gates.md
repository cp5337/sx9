# RFC-9142: Semantic Drift Scoring & Governance Gates

**Status:** CANONICAL
**Author:** Charles E. Payne / Claude
**Date:** 2025-12-24
**Supersedes:** N/A
**Integrates:** RFC-9140, RFC-9141, RFC-9025, RFC-9050, RFC-9100

---

## Abstract

RFC-9142 defines the semantic drift detection, scoring, and governance gate system used by FORGE to ensure long-lived alignment between original intent and emergent system behavior. This document formalizes how intent is represented, how deviation is measured, and how corrective action is triggered without conflating structure with meaning.

Semantic drift is treated as a first-class operational signal, not a code smell.

---

## 1. Purpose

Structural correctness does not guarantee semantic fidelity.

RFC-9142 answers a single operational question:

> "Is the system still doing what we said it should do?"

This RFC defines:

- How intent is encoded
- How observed behavior is interpreted
- How drift is measured
- How governance gates respond

---

## 2. Definitions

### 2.1 Intent

Intent is the declared purpose and constraint set of a capability, captured through:

- Node interviews (what / why)
- Design-phase narratives
- Canonical pattern anchors (N-V-N-N)

**Constraint:** Intent is immutable once a capability enters Build.

### 2.2 Observation

Observation is the actual behavior of the system, derived from:

- Code structure
- Dependency graphs
- Runtime signals (where available)
- Static QA outputs

Observation is continuously updated.

### 2.3 Semantic Drift

Semantic drift is the measurable divergence between intent and observation.

- Drift does not imply failure
- Drift implies risk

---

## 3. Semantic Graph Model

Two independent graphs are maintained.

### 3.1 Intent Graph

**Constructed from:**
- Node interviews
- Lifecycle narratives
- Canonical pattern declarations

**Nodes represent:**
- Roles
- Responsibilities
- Constraints
- Expected interactions

**Edges represent:**
- Intended dependency
- Allowed influence
- Prohibited coupling

### 3.2 Observed Graph

**Constructed from:**
- AST-derived structure
- Call graphs
- Data flow
- Service boundaries

**Nodes represent:**
- Actual components
- Functions
- Services

**Edges represent:**
- Real dependency
- Data movement
- Control influence

---

## 4. Drift Vectors

Drift is not binary. It is directional.

The following drift vectors are evaluated independently:

| Vector | Description |
|--------|-------------|
| Role Drift | A component assumes responsibilities not declared in intent |
| Constraint Drift | Originally stated limits (bounds, validation, isolation) erode or disappear |
| Coupling Drift | Dependencies increase beyond modeled expectations |
| Authority Drift | Decision-making migrates to unintended layers |
| Pattern Drift | Canonical N-V-N-N patterns are structurally present but behaviorally violated |

---

## 5. Drift Scoring

Each vector produces a normalized score:

| Score | Interpretation |
|-------|----------------|
| 0.0 | No drift |
| 0.3 | Minor deviation |
| 0.6 | Significant misalignment |
| 0.8+ | Governance concern |

Scores are non-aggregated by default. Aggregation is a governance decision, not an algorithmic one.

---

## 6. Δ-Angle Interpretation

Drift vectors are mapped to Δ-angle semantics:

| Angle Range | Response |
|-------------|----------|
| 0°–15° | Acceptable evolution |
| 15°–45° | Monitor |
| 45°–90° | Investigate |
| 90°+ | Escalate |

Δ-angle is directional, measuring drift toward:

- Complexity
- Centralization
- Opacity

---

## 7. Governance Gates

Semantic QA drives governance gates, not build gates.

### 7.1 Gate Levels

| Level | Action |
|-------|--------|
| Observe | Log only |
| Warn | Annotate PR / Issue |
| Gate | Require human acknowledgment |
| Escalate | Block release pending review |

### 7.2 Trigger Conditions

Governance gates may trigger on:

- Sustained drift over time
- Sudden high-magnitude drift
- Drift in safety-critical roles
- Drift across multiple vectors

---

## 8. Relationship to Static QA

Static QA answers:
> "Is this valid?"

Semantic QA answers:
> "Is this faithful?"

They must never override each other.

A system may be:

| Structural State | Semantic State | Status |
|------------------|----------------|--------|
| Perfect | Wrong | Unacceptable |
| Unsafe | Aligned | Unacceptable |

Both conditions are unacceptable in production.

---

## 9. Human-in-the-Loop Doctrine

Semantic drift resolution is always human-adjudicated.

**The system:**
- Detects
- Scores
- Explains

**Humans:**
- Decide
- Correct
- Accept risk

This preserves accountability.

---

## 10. Invariants

1. Intent is immutable post-Design
2. Drift is directional, not binary
3. Semantic QA never auto-approves
4. Governance gates are auditable
5. Patterns define meaning, not aesthetics
6. Drift without visibility is failure

---

## 11. References

- RFC-9140 — FORGE Unified Architecture
- RFC-9141 — FORGE Assembly Line & Dual-Heartbeat QA
- RFC-9025 — Node Interview Schema
- RFC-9050 — QA Two-Heartbeat System
- RFC-9100 — PTCC Primitives

---

**Document Status:** CANONICAL
