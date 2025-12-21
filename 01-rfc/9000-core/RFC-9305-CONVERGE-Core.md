# RFC-9305
## CONVERGE Core Specification
### Distributed Action-Set Convergence Detection

Status: Draft (Standards Track)

1. Problem
Distributed coordination evades single-event analytics.

2. Solution
Detect convergence via deterministic action-set selection and rank thresholds.

3. ActionEvent
Immutable, ephemeral events: time, geography, source, action_type, confidence, novelty.

4. Windowing
Sliding bounded windows; no persistence.

5. Output
ConvergeSignal: rank, rank_delta, spans, confidence, explanation.

6. Invariants
Deterministic, no storage, set-based reasoning only.
