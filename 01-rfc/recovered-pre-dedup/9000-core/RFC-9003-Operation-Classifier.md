# RFC-9003 — Operation Classifier & Escalation Logic

**Version:** 1.0
**Status:** Final
**Date:** November 23, 2025
**Applies To:** Synaptix9, CTAS-7.3.1
**Author:** CTAS Core Engineering Group

## 1. Operation Categories

| Class              | Meaning                   | Restrictions    |
| :----------------- | :------------------------ | :-------------- |
| **Intelligence**   | Collection & discovery    | Always allowed  |
| **Defensive**      | Hardening                 | Always allowed  |
| **Offensive**      | Reactive threat emulation | Strict approval |
| **Administrative** | Metadata ops              | Low priority    |

## 2. Escalation Tiers

1.  **WASM**
2.  **Microkernel**
3.  **Kernel Crate**
4.  **Multi-Crate**
5.  **Container**
6.  **Firefly**
7.  **Orb**

Each tier SHALL require:

- Authentication
- State handoff
- Resource checks
- Delta gate evaluation
