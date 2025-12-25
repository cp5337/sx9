# RFC-9144: HUD Canonical ACK & PR Governance

**Status:** NORMATIVE
**Binding:** RFC-9141, RFC-9142, CLSGS
**Scope:** FORGE Assembly Line, Decentralized Operations
**Authority:** SDC Software Engineering Manual

---

## 1. PURPOSE

This RFC defines the canonical acknowledgment (ACK) structure for the Full HUD and establishes PR governance modes for decentralized operations.

---

## X. AUTOMATED PR ENVELOPE AND PR MODES

All construction runs SHALL generate an automated PR envelope. The envelope exists to provide a deterministic diff container and lineage anchor. PR creation is mechanical and mandatory; PR approval is policy-driven and optional.

### X.1 Principle

**The pull is the act of legitimacy. Approval is a coordination convenience.**

In decentralized or asynchronous operations, PR approval may be delayed or unavailable. The system SHALL remain forward-capable under such conditions without weakening lineage, traceability, or gating.

### X.2 PR Modes (Governance Posture)

PR modes describe governance posture, not correctness.

- **SD — Solo Development (default in decentralized operations)**
  - PR is auto-created as an envelope for symmetry and lineage
  - Review is not required to proceed
  - Merge is not expected and may be deferred indefinitely
  - SD is self-governed execution, not reduced governance

- **AD — Assisted / Agentic Development**
  - PR is auto-created and labeled for agentic or long-run execution
  - Review is optional unless repository policy escalates
  - Used to preserve symmetry and auditability for non-human execution

- **TD — Team Development**
  - PR is auto-created and governed by required reviewers and checks
  - Merge is gated by policy enforcement

### X.3 Default Inversion Rule

Default PR mode SHALL be configurable at the repository or workspace policy layer:

- In solo / decentralized posture, default mode SHALL be **SD**
- In collaborative / synchronized posture, default mode SHALL be **TD**

This inversion is policy-only and SHALL NOT require schema or doctrine changes.

### X.4 Non-Blocking Requirement (Decentralized Operations)

In decentralized operations, the absence of reviewers SHALL NOT block progression when PR mode is SD or AD. The system SHALL preserve:

- Branch isolation
- PR envelope existence
- Full HUD ACK visibility
- Linear intent anchoring
- Trace and WIP emission

### X.5 HUD Rendering Rule

The Full HUD SHALL render PR envelope state as factual posture:

- `PR Mode: SD | AD | TD`
- `Review: optional | required`
- `Envelope: present`

No alignment indicator shall degrade solely due to SD/AD posture.

---

## INVARIANTS

1. The pull is the act of legitimacy
2. PR envelope is mandatory; approval is policy
3. SD/AD/TD describe posture, not correctness
4. Decentralized ops remain forward-capable
5. HUD renders mode without alignment degradation

---

**END OF RFC**
