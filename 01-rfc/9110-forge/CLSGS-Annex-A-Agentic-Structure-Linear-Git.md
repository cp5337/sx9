# CLSGS ANNEX A

## Agentic Structure, Linear Task Integration, and Git Universe Alignment

---

**Status:** NORMATIVE
**Binding:** RFC-9141, RFC-9142
**Scope:** Extension to CLSGS only
**Authority:** SDC Software Engineering Manual

---

## A.1 PURPOSE

This Annex defines how the Commentary, Lint, and Semantic Governance Specification (CLSGS) operates end-to-end across:

- Agentic folder structures
- Linear task forms
- The Git universe

This Annex introduces no new primitives, QA modes, workflows, or architectures. It formalizes how existing doctrine manifests in operational substrates.

---

## A.2 AGENTIC FOLDER STRUCTURES

Agentic folder structures exist to preserve **behavioral responsibility boundaries** across human, AI, and hybrid contributors.

Folders are not organizational artifacts. They are **semantic containers** whose structure must remain stable under drift analysis.

### A.2.1 Canonical Agentic Structure

Agentic folders MUST be organized by **declared responsibility**, not by implementation detail.

A canonical agentic structure SHALL reflect:

| Element | Requirement |
|---------|-------------|
| Top-level folder | Single dominant ROLE |
| Subfolder | Bounded ACTION scope |
| Ownership | Explicit CONSTRAINT ownership |
| Domain | Stable OBJECT domain |

Folder naming MUST correspond to the same semantic vocabulary used in N-V-N-N annotations.

### A.2.2 Prohibited Structures

The following are explicitly prohibited:

- Stub or placeholder folders
- "misc", "utils", "helpers", or catch-all directories
- Agent folders differentiated only by tool or model name
- Structural duplication introduced solely for ownership separation

Folders that exist without a clear behavioral declaration are treated as **semantic opacity**.

### A.2.3 Lint and Semantic Interaction

**Static QA evaluates:**

- Structural correspondence between folder scope and annotation density
- Absence of artificial subdivision to evade CLSGS interval bands

**Semantic QA evaluates:**

- Whether code within a folder violates its declared behavioral scope
- Whether responsibility has migrated without declaration

Folder boundaries therefore act as **coarse-grain semantic anchors** for drift detection.

---

## A.3 LINEAR TASK FORM INTEGRATION

Linear is treated as an **intent ingress and trace surface**, not a workflow engine.

### A.3.1 Intent Mapping

Each Linear task represents a unit of declared intent.

That intent MUST map to:

- One or more behavioral regions in code
- Corresponding N-V-N-N annotations
- A bounded agentic scope

Tasks do not map to files. Tasks map to **declared behavior**.

### A.3.2 QA Signal Mapping

| QA Type | Maps To | Implication |
|---------|---------|-------------|
| Static QA failure | Task validity | Task cannot progress |
| Semantic QA drift | Task risk | Annotations attach to originating task |
| Governance gate | Task constraint | Surfaces as task-level constraint |
| Accepted risk | Task visibility | Remains visible and attributable |

Linear is therefore a **projection surface**, not an authority layer.

### A.3.3 Prohibited Interpretations

The following are explicitly prohibited:

- Treating task closure as proof of intent fulfillment
- Encoding workflow state as semantic approval
- Allowing task movement to suppress QA signals

Intent fidelity is preserved by QA doctrine, not task status.

---

## A.4 GIT UNIVERSE ALIGNMENT

Git is treated as a **semantic history carrier**, not merely a version control system.

### A.4.1 Behavioral Lineage

N-V-N-N annotations function as **semantic lineage markers**.

Across:

- Commits
- Branches
- Rebases
- Cherry-picks
- Refactors

Behavioral declarations MUST persist or be explicitly revised.

Loss of annotation is treated as **semantic regression**, even if code compiles.

### A.4.2 Drift Across History

Semantic QA evaluates drift longitudinally by:

- Comparing historical annotations to current behavior
- Detecting gradual responsibility expansion
- Identifying constraint erosion across commits

Drift may be acceptable. Undeclared drift is not.

### A.4.3 Pull Requests and Review

Pull requests are evaluation boundaries, not approval mechanisms.

| Gate | Function |
|------|----------|
| Static QA | Establishes structural admissibility |
| Semantic QA | Establishes risk visibility |

Approval does not equal alignment. Merge does not equal intent preservation.

---

## A.5 GOVERNANCE INTERACTION

This Annex introduces no new governance authority.

It clarifies that:

- Agentic structure violations
- Task-to-behavior mismatches
- Annotation loss or falsification

All constitute **semantic drift signals** eligible for escalation under RFC-9142.

---

## A.6 INVARIANTS

1. Structure exists to preserve meaning
2. Tasks express intent, not truth
3. Git carries semantic history
4. Comments bind behavior across systems
5. Drift without visibility is failure

---

## A.7 FINAL STATUS

This Annex is:

- Fully bound to RFC-9141 and RFC-9142
- Non-invasive to existing doctrine
- Auditor-safe and machine-legible
- Ready for SDC seal inclusion

---

**END OF ANNEX**
