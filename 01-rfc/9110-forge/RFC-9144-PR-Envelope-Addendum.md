─────────────────────────────────────────────────────────────────────────────────────────────────────────
1) RFC-9144 Addendum Text (Drop-in Sections)

Paste these into RFC-9144 (recommended placement: after the PR automation section / or immediately after the Branch-first ACK language).

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

2) Linear Entry (Copy/Paste)

Title: Lock PR Envelope + SD/AD/TD Modes (Decentralized Ops Safe)
Labels: governance, hud, rfc-9144, sdc, clsgs
Type: Decision / RFC Amendment

Body:

Decision: All construction runs auto-generate a PR envelope for symmetry and lineage.
PR approval is policy-driven and optional; the pull is the act of legitimacy.

PR modes:
- SD (Solo Development): default for decentralized operations; no required review to proceed
- AD (Assisted/Agentic Development): agentic / long-run posture; review optional unless escalated
- TD (Team Development): review + checks required; merge gated

Default inversion:
- Solo/decentralized posture defaults SD
- Team/synchronized posture defaults TD
This inversion is policy-only and does not change doctrine/schema.

HUD must render PR Mode + review posture as factual state and may not degrade alignment solely due to SD/AD.

3) Git Commit Patch (Copy/Paste)
File to add/update

docs/rfc/RFC-9144-HUD-Canonical-ACK.md (append the addendum above)

Commit message
rfc(9144): define PR envelope + SD/AD/TD modes for decentralized ops

- All runs auto-generate PR envelope for symmetry/lineage
- Pull establishes legitimacy; approval is policy convenience
- SD/AD/TD modes define posture without affecting correctness
- Default inversion SD↔TD is policy-only
- HUD renders PR mode without degrading alignment


If Claude is doing the mechanics, this is all he needs:

paste addendum into RFC-9144

create Linear decision entry

commit with message above