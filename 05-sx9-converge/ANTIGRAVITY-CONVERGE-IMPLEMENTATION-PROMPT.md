ANTIGRAVITY IMPLEMENTATION PROMPT
================================

PROJECT: CONVERGE (RFC-93XX FAMILY)
ROLE: Antigravity – Spec-Driven Systems Engineer

OBJECTIVE
---------
Implement the CONVERGE convergence-detection sensor in strict compliance with
RFC-9101, RFC-9021, RFC-9304, and RFC-93X1–93X4.

This is a PHASE-0 implementation:
- Fix failing tests
- Lock interfaces and invariants
- Add early-step tests
- DO NOT implement full matroid/HMM math yet

KEY INVARIANTS
--------------
- Graph is a sensor, not storage
- Deterministic execution
- No raw-event persistence
- Geometry may affect confidence only
- Selection is set-based, not event-based

DELIVERABLES
------------
1. Root-cause analysis of 15 failing tests:
   - Executor binding (4)
   - Parser advanced patterns (2)
   - Workflow validator (3)
   - Algorithm edge cases (6)

2. Code fixes with regression tests

3. CONVERGE crates:
   - converge (sensor)
   - converge-geometry (helper)
   - converge-selection (deterministic core)

4. Smart Crate manifests (RFC-9101 canonical)

5. Early-step tests enforcing:
   - determinism
   - no-storage invariant
   - windowing correctness
   - geometry boundary enforcement
   - selection ordering stability

6. Math plug-in boundaries for future:
   - HMM (Viterbi, FB, BW)
   - Matroid rank engines

TECHNICAL APPROACH
------------------
- Fix failing tests first (minimal patches)
- Implement stable types and traits
- Use placeholder math with correct semantics
- Ensure interfaces will not change when real math lands

PHASE 0 VS PHASE 1
------------------
PHASE 0 (NOW):
- Interfaces
- Deterministic selection
- Tests
- Integration

PHASE 1 (LATER):
- Full matroid rank
- HMM state inference
- Monte Carlo stability analysis

COMPLETION CRITERIA
-------------------
- All tests pass
- New tests added and passing
- Crates compile
- Manifests canonical
- No invariant violations
