# SX9-RFC Registry — Canonical Specification Index

**Version:** 2.0
**Status:** Active
**Last Updated:** December 4, 2025
**Maintainer:** CTAS Core Engineering Group
**Canonical Path:** `/Users/cp5337/Developer/sx9/01-rfc/`

---

## IMPORTANT: RFC Governance

**Before creating a new RFC:**

1. Check this registry for the next available number in your series
2. Update the `NEXT_AVAILABLE` field when you claim a number
3. Add your RFC to this registry BEFORE writing the document

This prevents numbering conflicts across sessions.

---

## Series Overview

| Series          | Range     | Purpose                                | Next Available |
| --------------- | --------- | -------------------------------------- | -------------- |
| **Core**        | 9000-9009 | Foundation primitives, crypto, routing | **FULL**       |
| **Pipeline**    | 9010-9019 | Ingestion, embedding, training         | 9014           |
| **Cognitive**   | 9020-9029 | AI inference, doctrine, scoring        | 9027           |
| **Integration** | 9100-9149 | System integration, crates             | 9106           |
| **Application** | 9150-9199 | End-user features (GIS, Notebook)      | 9152           |
| **Platform**    | 9200-9299 | SX9 Dev Center, deployment             | 9201           |

---

## Core Series (9000-9009) — FULL

| RFC      | Title                              | Status | Location                                                   |
| -------- | ---------------------------------- | ------ | ---------------------------------------------------------- |
| RFC-9000 | Synaptix9 Agnostic Core & Ontology | Final  | `9000-core/RFC-9000-Agnostic-Core.md`                      |
| RFC-9001 | Trivariate Hashing Standard        | Final  | `9000-core/RFC-9001-Trivariate-Hashing.md`                 |
| RFC-9002 | Unicode Operational Routing System | Final  | `9000-core/RFC-9002-Unicode-Routing.md`                    |
| RFC-9003 | Operation Classifier & Escalation  | Final  | `9000-core/RFC-9003-Operation-Classifier.md`               |
| RFC-9004 | Deterministic Routing Architecture | Draft  | `9000-core/RFC-9004-Deterministic-Routing.md`              |
| RFC-9005 | Unified Schema Specification       | Final  | `9000-core/RFC-9005-Unified-Schema.md`                     |
| RFC-9006 | Secure Transport Profiles          | Draft  | `9000-core/RFC-9006-Secure-Transport-Profiles.md`          |
| RFC-9007 | Obfuscation & Biometric Honeypot   | Draft  | `9000-core/RFC-9007-Obfuscation-Biometric-Honeypot.md`     |
| RFC-9008 | Ephemeral Engagement Rooms         | Draft  | `9000-core/RFC-9008-Ephemeral-Engagement-Rooms.md`         |
| RFC-9009 | Quantum Cryptographic Architecture | Draft  | `9000-core/RFC-9009-Quantum-Cryptographic-Architecture.md` |

**NEXT_AVAILABLE: NONE (series full, use 9100+ for extensions)**

---

## Pipeline Series (9010-9019)

| RFC        | Title                             | Status | Location                                                     |
| ---------- | --------------------------------- | ------ | ------------------------------------------------------------ |
| RFC-9010   | Enterprise Information Extraction | Draft  | `9010-pipeline/RFC-9010-Enterprise-Extraction.md`            |
| RFC-9011   | Threat Content Ingestion          | Draft  | `9010-pipeline/RFC-9011-Threat-Ingestion.md`                 |
| RFC-9011-A | Canonical Ingestion Pipeline      | Draft  | `9010-pipeline/RFC-9011-A-Ingestion-Pipeline.md`             |
| RFC-9011-B | YAML Validation & DSL Conversion  | Draft  | `9010-pipeline/RFC-9011-B-YAML-Validation-DSL-Conversion.md` |
| RFC-9012   | Embeddings & GNN Training Fabric  | Draft  | `9010-pipeline/RFC-9012-GNN-Embeddings.md`                   |
| RFC-9013   | Sensory Substrate                 | Draft  | `9010-pipeline/RFC-9013-Sensory-Substrate.md`                |

**NEXT_AVAILABLE: 9014**

---

## Cognitive Series (9020-9029)

| RFC      | Title                                                | Status    | Location                                                           |
| -------- | ---------------------------------------------------- | --------- | ------------------------------------------------------------------ |
| RFC-9020 | HD4 Framework (Hunt-Detect-Disable-Disrupt-Dominate) | Final     | `9000-core/RFC-9020-HD4-Framework.md`                              |
| RFC-9021 | Graph Convergence Theory                             | Canonical | `9000-core/RFC-9021-Graph-Convergence-Theory.md`                   |
| RFC-9022 | OODA Vertical Escalation                             | Draft     | `9000-core/RFC-9022-OODA-Vertical-Escalation.md`                   |
| RFC-9023 | Security Framework Integration Map                   | Draft     | `9100-integration/RFC-9023-Security-Framework-Integration-Map.md`  |
| RFC-9024 | Neurological Foundation                              | Canonical | `9000-core/RFC-9024-Neurological-Foundation.md`                    |
| RFC-9025 | Unified Interview Schema                             | Draft     | `9000-core/RFC-9025-Unified-Interview-Schema.md`                   |
| RFC-9026 | Hourglass-Bernoulli Cognitive Architecture           | Canonical | `9000-core/RFC-9026-Hourglass-Bernoulli-Cognitive-Architecture.md` |

**NEXT_AVAILABLE: 9027**

**Note:** Session reference versions available in `cognitive-session-reference/` for comparison.

---

## Integration Series (9100-9149)

| RFC      | Title                               | Status      | Location                                                              |
| -------- | ----------------------------------- | ----------- | --------------------------------------------------------------------- |
| RFC-9100 | Dual-Trivariate PTCC Integration    | Draft       | `9100-integration/RFC-9100-Dual-Trivariate-PTCC-Integration.md`       |
| RFC-9101 | Smart Crate System v7.3.1+          | Production  | `9100-integration/RFC-9101-Smart-Crate-System.md`                     |
| RFC-9102 | Executable Document Framework       | POC         | `9100-integration/RFC-9102-Executable-Document-Framework.md`          |
| RFC-9103 | IAC Adaptive Infrastructure         | Planned     | —                                                                     |
| RFC-9104 | CTE Cognitive Execution Framework   | Planned     | —                                                                     |
| RFC-9105 | SPIRES Extraction                   | Draft       | `9100-integration/RFC-9105-SPIRES-Extraction.md`                      |
| RFC-9030 | Unified Linear Agent Infrastructure | Draft       | `9100-integration/RFC-9030-Unified-Linear-Agent-Infrastructure.md`    |
| RFC-9106 | sx9-conda Python Execution Layer    | Draft       | `../ctas7-shipyard-system/docs/architecture/RFC-9106-sx9-conda.md`    |
| RFC-9107 | Unified Agent Infrastructure        | Draft       | `9100-integration/RFC-9107-Unified-Agent-Infrastructure.md`           |
| RFC-9108 | Thalmic Filter Model Registry       | Draft       | `9100-integration/RFC-9108-Thalmic-Filter-Model-Registry.md`          |
| RFC-9109 | CX9 Custom Kali ISO                 | Planned     | —                                                                     |
| RFC-9110 | SX9 Lisp Interpreter                | Implemented | `9100-integration/RFC-9110-SX9-Lisp-Interpreter.md`                   |
| RFC-9111 | Zero-License Data Fabric            | Planned     | —                                                                     |
| RFC-9112 | Deterministic Prompt Engineering    | Canonical   | `9100-integration/RFC-9112-Deterministic-Prompt-Engineering.md`       |
| RFC-9113 | TOML Executable Document Spec       | Canonical   | `9100-integration/RFC-9113-TOML-Executable-Document-Specification.md` |
| RFC-9114 | SX9 Gateway Neural Retrofit         | Draft       | `9100-integration/RFC-9114-SX9-Gateway-Neural-Retrofit.md`            |
| RFC-9115 | SX9 Frontend Adapter Standard       | Draft       | `9100-integration/RFC-9115-Frontend-Adapter-Standard.md`              |

**NEXT_AVAILABLE: 9116**

---

## Operational Series (9800-9899) — L2 NATS Kali Platform

| RFC      | Title                           | Status    | Location                                                       |
| -------- | ------------------------------- | --------- | -------------------------------------------------------------- |
| RFC-9876 | Layer-Two Unicode Orchestration | Canonical | `9800-operational/RFC-9876-Layer-Two-Unicode-Orchestration.md` |
| RFC-9130 | L2 NATS Kali Execution Platform | Canonical | `9800-operational/RFC-9130-L2-NATS-Kali-Execution-Platform.md` |
| RFC-9131 | Dynamic Resource Escalation     | Draft     | `9800-operational/RFC-9131-Dynamic-Resource-Escalation.md`     |

**NEXT_AVAILABLE: 9132**

### Operational Series Overview

The **L2 NATS Kali Execution Platform** is a complete microsecond-latency security tool execution system:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    MICROSECOND KALI EXECUTION STACK                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  RFC-9112: Deterministic Prompt Engineering                                 │
│      │     (PromptScript DSL, Hermetic Execution, NATS Fabric)              │
│      │                                                                      │
│      ▼                                                                      │
│  RFC-9876: Layer-Two Unicode Orchestration                                  │
│      │     (XDP/eBPF, U+E000-U+F8FF triggers, L2 frames)                    │
│      │                                                                      │
│      ▼                                                                      │
│  RFC-9130: L2 NATS Kali Execution Platform                                  │
│      │     (CTAS-7 Server, Skills Matrix, PTCC, TETH, < 50μs)               │
│      │                                                                      │
│      ▼                                                                      │
│  RFC-9001: Trivariate Hashing (Murmur3-64, Base96)                          │
│  RFC-9100: PTCC Primitives (32 ops, U+E400-E41F)                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**

- **< 50μs latency** — Bernoulli zone compliant
- **Hermetic execution** — No shell, no files, no logs
- **NATS fabric** — All communication via NATS pub/sub and KV
- **CTAS-7 brain** — Skills Matrix, PTCC configs, TETH entropy
- **L2 triggers** — Unicode private-use sequences via XDP/eBPF

---

## Application Series (9150-9199)

| RFC      | Title                                | Status | Location                                           |
| -------- | ------------------------------------ | ------ | -------------------------------------------------- |
| RFC-9150 | GIS UI Specification                 | Draft  | `9400-application/RFC-9150-GIS-UI.md`              |
| RFC-9151 | Patrolman's Notebook Evidence System | Draft  | `9400-application/RFC-9151-Patrolmans-Notebook.md` |

**NEXT_AVAILABLE: 9152**

---

## Platform Series (9200-9299)

| RFC      | Title                  | Status | Location                                              |
| -------- | ---------------------- | ------ | ----------------------------------------------------- |
| RFC-9200 | SX9 Development Center | Draft  | `9100-integration/RFC-9200-SX9-Development-Center.md` |

**NEXT_AVAILABLE: 9201**

---

## Core Graph & Analytics Series (9300-9399)

| RFC      | Title                           | Status    | Location                                                |
| -------- | ------------------------------- | --------- | ------------------------------------------------------- |
| RFC-9300 | HD4 Canonical Specification     | Canonical | `9000-core/RFC-9300-HD4-Canonical-Specification.md`     |
| RFC-9301 | Thyristor Crystal RingBus       | Draft     | `9000-core/RFC-9301-Thyristor-Crystal-RingBus.md`       |
| RFC-9302 | Nonagon Analytic Node           | Validated | `9000-core/RFC-9302-Nonagon-Analytic-Node.md`           |
| RFC-9303 | Crystal Realms Kinematics       | Draft     | `9000-core/RFC-9303-Crystal-Realms-Kinematics.md`       |
| RFC-9304 | GLAF Graph Engine Specification | Draft     | `9000-core/RFC-9304-GLAF-Graph-Engine-Specification.md` |

**NEXT_AVAILABLE: 9305**

---

## Directory Structure

RFCs are organized by series with numbered folders for proper sorting:

```
01-rfc/
│   ├── RFC-9020-HD4-Framework.md
│   ├── RFC-9021-Graph-Convergence-Theory.md
│   ├── RFC-9022-OODA-Vertical-Escalation.md
│   ├── RFC-9024-Neurological-Foundation.md
│   ├── RFC-9025-Unified-Interview-Schema.md
│   └── RFC-9026-Hourglass-Bernoulli-Cognitive-Architecture.md  # NEW - Canonical IP
├── pipeline/                             # 9010-9019 Data Pipeline
│   ├── RFC-9010-Enterprise-Extraction.md
│   ├── RFC-9011-Threat-Ingestion.md
│   ├── RFC-9011-A-Ingestion-Pipeline.md           # NEW
│   ├── RFC-9011-B-YAML-Validation-DSL-Conversion.md # NEW
│   ├── RFC-9012-GNN-Embeddings.md
│   └── RFC-9013-Sensory-Substrate.md
├── integration/                          # 9100-9149 Applied Specs
│   ├── RFC-9023-Security-Framework-Integration-Map.md
│   ├── RFC-9100-Dual-Trivariate-PTCC-Integration.md
│   ├── RFC-9101-Smart-Crate-System.md
│   ├── RFC-9102-Executable-Document-Framework.md
│   ├── RFC-9105-SPIRES-Extraction.md              # NEW
│   └── RFC-9200-SX9-Development-Center.md
├── application/                          # 9150-9199 End-User Features
│   ├── RFC-9150-GIS-UI.md                         # MOVED from 9006
│   └── RFC-9151-Patrolmans-Notebook.md            # MOVED from 9007
├── platform/                             # 9200-9299 Platform
│   └── RFC-9200-Data-Analytics-Workbench.md
├── cognitive-session-reference/          # Session exports for review
│   └── (alternative versions from Claude Desktop sessions)
└── archive/                              # Deprecated/superseded
```

---

## Migration Log (Nov 28, 2025)

| Action  | From                         | To                                          | Reason                                |
| ------- | ---------------------------- | ------------------------------------------- | ------------------------------------- |
| MOVE    | RFC-9006-GIS-UI              | RFC-9150-GIS-UI                             | Application-level, not core primitive |
| MOVE    | RFC-9007-Patrolmans-Notebook | RFC-9151-Patrolmans-Notebook                | Application-level, not core primitive |
| IMPORT  | Session RFC-9006             | RFC-9006-Secure-Transport-Profiles          | Security layer belongs in core        |
| IMPORT  | Session RFC-9007             | RFC-9007-Obfuscation-Biometric-Honeypot     | Security layer belongs in core        |
| IMPORT  | Session RFC-9008             | RFC-9008-Ephemeral-Engagement-Rooms         | Security layer belongs in core        |
| IMPORT  | Session RFC-9009             | RFC-9009-Quantum-Cryptographic-Architecture | Security layer belongs in core        |
| IMPORT  | Session RFC-9011-A           | RFC-9011-A-Ingestion-Pipeline               | Sub-RFC to 9011                       |
| IMPORT  | Session RFC-9011-B           | RFC-9011-B-YAML-Validation-DSL-Conversion   | Sub-RFC to 9011                       |
| IMPORT  | Session RFC-9105             | RFC-9105-SPIRES-Extraction                  | Integration spec                      |
| ARCHIVE | Session RFC-9020-9025        | cognitive-session-reference/                | Existing versions more complete       |

---

## Change Log

| Date       | Change                                                                               | Author           |
| ---------- | ------------------------------------------------------------------------------------ | ---------------- |
| 2025-12-04 | **RFC-9026** Hourglass-Bernoulli Cognitive Architecture (Canonical IP documentation) | CTAS Engineering |
| 2025-12-03 | **RFC-9131** Dynamic Resource Escalation (Force Multiplier, entropy-based scaling)   | CTAS Engineering |
| 2025-12-03 | **RFC-9130** L2 NATS Kali Execution Platform (microsecond Kali with CTAS brain)      | CTAS Engineering |
| 2025-12-03 | **RFC-9876** Updated to v1.1 (RFC-9001 compliance, NATS integration, hermetic)       | CTAS Engineering |
| 2025-12-03 | **Operational Series** Created for L2 NATS Kali Platform (9800-9899)                 | CTAS Engineering |
| 2025-12-01 | **RFC-9110** Added SX9 Lisp Interpreter (HFT Unicode bytecode, escalation, OODA)     | CTAS Engineering |
| 2025-12-01 | **RFC-9107** Added Unified Agent Infrastructure (dual-role agents, ABE QA, voice)    | CTAS Engineering |
| 2025-11-29 | **RFC-9106** Added sx9-conda Python Execution Layer (NATS, gRPC, Port Manager)       | CTAS Engineering |
| 2025-11-28 | **RFC-9011-A §9** Added ATL-Physical Domain Integration (physical threat pipeline)   | CTAS Engineering |
| 2025-11-28 | **RFC-9025 §8** Added ATL-Physical Domain Interviews (physical interview schema)     | CTAS Engineering |
| 2025-11-28 | **v2.0** Registry restructure - added Application series, NEXT_AVAILABLE tracking    | CTAS Engineering |
| 2025-11-28 | Imported session RFCs, resolved numbering conflicts                                  | CTAS Engineering |
| 2025-11-27 | Added RFC-9006 GIS UI, RFC-9007 Patrolman's Notebook                                 | CTAS Engineering |
| 2025-11-26 | Created canonical structure                                                          | CTAS Engineering |

---

**Status:** This registry is the SINGLE SOURCE OF TRUTH for RFC numbering.
