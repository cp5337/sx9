# SX9 RFC Registry

**Recovery Date:** 2025-12-24  
**Source:** Claude conversation history extraction

---

## Core RFCs (9000 Series)

| RFC | Title | Status | Dependencies |
|-----|-------|--------|--------------|
| RFC-9000 | Agnostic Core & Ontology | Final | — |
| RFC-9001 | Trivariate Hashing Standard | Final | RFC-9000 |
| RFC-9002 | Unicode Operational Routing | Final | RFC-9001 |
| RFC-9003 | Operation Classifier (PTCC) | Final | RFC-9002 |
| RFC-9004 | Deterministic Routing | Final | RFC-9003 |
| RFC-9005 | Unified Schema Specification | Final | RFC-9000, RFC-9001 |
| RFC-9006 | Smart Memory Mesh | Draft | RFC-9001, RFC-9005 |

---

## Pipeline RFCs (9010 Series)

| RFC | Title | Status | Dependencies |
|-----|-------|--------|--------------|
| RFC-9020 | NATS Message Infrastructure | Draft | RFC-9000, RFC-9003 |
| RFC-9021 | Trivariate Message Routing | Draft | RFC-9001, RFC-9020 |
| RFC-9025 | Interview Schema | Final | RFC-9000, RFC-9005 |

---

## Integration RFCs (9100 Series)

| RFC | Title | Status | Dependencies |
|-----|-------|--------|--------------|
| RFC-9100 | Dual-Trivariate PTCC (32 Primitives) | Final | RFC-9001, RFC-9003 |
| RFC-9101 | Smart Crate System | Final | RFC-9000, RFC-9005 |
| RFC-9102 | Executable Document Framework | Active | RFC-9112, RFC-9113 |
| RFC-9105 | SPIRES Extraction | Active | RFC-9001, RFC-9011 |
| RFC-9107 | Unified Agent Infrastructure | Active | RFC-9000, RFC-9112 |
| RFC-9108 | Thalmic Filter Model Registry | Active | RFC-9107, RFC-9021 |
| RFC-9109 | Plasma Defender | Active | RFC-9000 |
| RFC-9112 | Deterministic Prompt Engineering | Active | RFC-9100 |

---

## Forge RFCs (9110 Series)

| RFC | Title | Status | Dependencies |
|-----|-------|--------|--------------|
| RFC-9112 | Deterministic Prompt Engineering | Active | RFC-9100 |
| RFC-9116 | Dev Forge Architecture | Active | RFC-9112 |
| RFC-9120 | Prompt Forge v4 | Active | RFC-9112 |
| RFC-9121 | Lightning QA | Active | RFC-9120 |
| RFC-9122 | Git/Linear/Slack Workflow | Active | RFC-9121 |
| RFC-9123 | Gold Disk Disaster Recovery | Draft | RFC-9122 |
| RFC-9124 | Cloud Reference Architecture | Draft | RFC-9123 |
| RFC-9127 | Architecture Compliance | Active | RFC-9121 |
| RFC-9130 | Unified Forge Pipeline | Draft | All 9110+ series |

---

## Application RFCs (9150+)

| RFC | Title | Status | Dependencies |
|-----|-------|--------|--------------|
| RFC-9150 | GIS UI Layer | Active | RFC-9109 |
| RFC-9151 | Patrolman's Notebook | Active | RFC-9150, RFC-9005 |
| RFC-9200 | SX9 Development Center | Active | RFC-9000, RFC-9101 |

---

## Operational RFCs (9800+)

| RFC | Title | Status | Dependencies |
|-----|-------|--------|--------------|
| RFC-9130 | L2 NATS Kali Execution | Active | RFC-9020, RFC-9100, RFC-9876 |
| RFC-9131 | Dynamic Resource Escalation | Active | RFC-9130, RFC-9020 |
| RFC-9876 | Layer-Two Unicode Orchestration | Active | RFC-9002, RFC-9004 |

---

## Agent Harness Versions

| Version | Format | Status | Key Features |
|---------|--------|--------|--------------|
| v2 | YAML | Deprecated | 5 harness modes |
| v3 | React/JSX | Deprecated | HARNESSES + PERSONAS + Linear |
| v4 | YAML | Deprecated | SX9-PROMPT v4.0 format |
| **v5** | tar.gz | **Current** | Full bundle with gates, harness/, integrations/ |

---

## Recovered in This Bundle

| Category | Files | Status |
|----------|-------|--------|
| **RFC-9112** | Deterministic Prompt Engineering | ✅ Recovered |
| **RFC-9120** | Prompt Forge v4 | ✅ Recovered |
| **QA Architecture** | Three-tier (WASM/Local/OrbStack) | ✅ Recovered |
| **OrbStack Compose** | Database stack | ✅ Recovered |
| **OPS-MAIN Rewire** | Kill Chain + Route declutter | ✅ Recovered |
| **REGISTRY** | This file | ✅ Created |

---

## What Needs Further Recovery

### High Priority
1. **RFC-9000** - Agnostic Core full text
2. **RFC-9001** - Trivariate Hashing complete spec
3. **RFC-9100** - PTCC 32 primitives full document
4. **RFC-9121** - Lightning QA grading formulas
5. **RFC-9130** - Unified Forge Pipeline complete spec

### Medium Priority
1. **Harness types.rs** - Rust port reference
2. **RFC-9002** - Unicode Routing details
3. **RFC-9003** - Operation Classifier logic

---

## Bundle Contents

```
rfc-recovery-3/
├── REGISTRY.md                              # This file
├── 9000-core/
│   └── (needs recovery from bundle upload)
├── 9100-integration/
│   └── RFC-9112-Deterministic-Prompt-Engineering.md
├── 9110-forge/
│   └── RFC-9120-Prompt-Forge-v4.md
├── harness/
│   └── (needs recovery from bundle upload)
├── forge-pipeline/
│   └── (needs recovery from bundle upload)
└── META/
    ├── OPS-MAIN-REWIRE-BUNDLE.md
    ├── QA-SYSTEM-ARCHITECTURE.md
    └── ORBSTACK-DOCKER-COMPOSE.md
```

---

**Document Status:** RECOVERED  
**Recovery Date:** 2025-12-24
