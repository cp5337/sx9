# DSL-Orchestration: Smart Crate Playbook Foundation

**Version:** 1.0.0
**Date:** 2025-11-27
**Status:** ACTIVE - Core Infrastructure
**RFC:** RFC-9101 (Smart Crate System), RFC-9102 (Executable Document Framework)

---

## Overview

This directory is the **playbook foundation** for the Smart Crate System and crate orchestration. It provides:

- DSL-based crate grouping and classification
- Port management integration
- Service orchestration
- AI-CLI integration for operational commands

> **Historical Note:** Previously named "XSD-QA-5" which was misleading - this is NOT a QA system. It's the orchestration layer that enables Smart Crate automation.

---

## Architecture Role

```
┌─────────────────────────────────────────────────────────────────────┐
│                    SX9 ORCHESTRATION LAYER                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  DSL-orchestration/ (YOU ARE HERE)                                  │
│  ├── Playbooks (DSL definitions)                                    │
│  │   └── Crate grouping schemas                                     │
│  ├── Port Integration                                               │
│  │   └── Extends ctas-port-manager                                  │
│  └── AI-CLI Integration                                             │
│      └── Operational commands                                       │
│                              │                                       │
│                              ▼                                       │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  Smart Crate System (RFC-9101)                                │  │
│  │  • Crate provisioning                                         │  │
│  │  • Dependency management                                      │  │
│  │  • Quality thresholds                                         │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                              │                                       │
│                              ▼                                       │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  Executable Document Framework (RFC-9102)                     │  │
│  │  • Living specifications                                      │  │
│  │  • Automated artifact generation                              │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Directory Structure

```
DSL-orchestration/
├── README.md                          # This file
├── dsl-crate-grouping-system.dsl      # Master crate grouping schema
├── dsl-port-integration.rs            # Port manager extension (Rust)
├── ai-cli-integration.ts              # AI-CLI React component
├── frontend-integration-system.ts     # Frontend orchestration
├── group-operations-executor.sh       # Batch execution scripts
├── live-integration.sh                # Live system integration
└── playbooks/
    ├── dsl-crate-grouping-system.dsl  # Crate classification playbook
    └── run-crate-grouping-playbook.sh # Execution script
```

---

## Components

### 1. DSL Crate Grouping Schema

**File:** `dsl-crate-grouping-system.dsl`

Defines crate groups with criteria-based classification:

```xml
<crateGroup groupId="foundation" groupName="Foundation" groupType="core_infrastructure">
  <criteria>
    <criterion>core_infrastructure</criterion>
    <criterion>base_functionality</criterion>
  </criteria>
  <crates>
    <crate>ctas-core</crate>
    <crate>ctas-port-manager</crate>
    ...
  </crates>
</crateGroup>
```

**Groups Defined:**
- Foundation (core infrastructure)
- Intelligence (analysis/ML)
- Operations (workflows)
- Network (communication)
- Data (storage/processing)
- Security (crypto/auth)

### 2. Port Integration (Rust)

**File:** `dsl-port-integration.rs`

Extends the ctas-port-manager for orchestration:

```rust
pub enum ServiceType {
    GroupOperations,
    CrateInterview,
    LispRdfIntegration,
    DSLOrchestration,  // Was XSDOrchestration
    OperationalIntelligence,
    FrontendIntegration,
    DatabaseIntegration,
    AICLI,
}
```

### 3. AI-CLI Integration

**File:** `ai-cli-integration.ts`

React component for operational commands:

```typescript
export enum AICLICategory {
  PortManagement = 'port_management',
  OperationalIntelligence = 'operational_intelligence',
  GroupOperations = 'group_operations',
  CrateInterview = 'crate_interview',
  LispRdfIntegration = 'lisp_rdf_integration',
  SystemHealth = 'system_health',
}
```

---

## Integration with Smart Crate System

This DSL orchestration layer feeds into RFC-9101 Smart Crate System:

1. **Classification** - DSL defines which crates belong to which groups
2. **Provisioning** - Smart Crate uses groups to determine dependencies
3. **Quality Gates** - Groups have quality thresholds (via QA system)
4. **Automation** - Playbooks enable automated crate management

---

## Usage

### Run Crate Grouping

```bash
cd playbooks
./run-crate-grouping-playbook.sh
```

### Execute Group Operations

```bash
./group-operations-executor.sh --group foundation --action validate
```

### Live Integration

```bash
./live-integration.sh --mode production
```

---

## Related Systems

| System | Relationship |
|--------|--------------|
| **ctas-port-manager** | Extended by dsl-port-integration.rs |
| **Smart Crate (RFC-9101)** | Consumes crate groupings |
| **Executable Docs (RFC-9102)** | Playbooks become living specs |
| **Lightning QA** | Quality gates per group |

---

## NOT a QA System

This directory is for **orchestration**, not quality assurance:

- **QA Systems:** `04-abe-iac/abe-qa-system/`, `ctas7-qa-analyzer/`
- **Orchestration:** This directory (`DSL-orchestration/`)

The "QA-5" in the old name was a version/iteration marker, not indicating quality assurance.

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-11-27 | Renamed from XSD-QA-5 to DSL-orchestration | CTAS Engineering |
| 2025-11-27 | Renamed xsd-* files to dsl-* | CTAS Engineering |
| 2025-11-27 | Created this README | CTAS Engineering |
