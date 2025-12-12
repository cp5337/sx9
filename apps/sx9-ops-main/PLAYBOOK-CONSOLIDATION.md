# Playbook/Orchestration System Consolidation

**Version:** 1.0.0
**Date:** 2025-11-27
**Status:** ACTION REQUIRED
**Issue:** "Double Wide with Car on Blocks"

---

## The Problem

Multiple competing playbook/orchestration systems have evolved independently, creating confusion about which is canonical.

```
                    THE CURRENT SITUATION
                    =====================

   "Like a double wide with a car up on blocks in the front yard"

   ┌─────────────────────────────────────────────────────────────┐
   │                                                              │
   │   ctas7-ops-main-platform/                                   │
   │   ├── DSL-orchestration/playbooks/      ← Smart Crate DSL   │
   │   │   ├── dsl-crate-grouping-system.dsl                      │
   │   │   └── run-crate-grouping-playbook.sh                     │
   │   └── playbooks/                        ← DUPLICATE!         │
   │       ├── crate-interview-playbook.toml                      │
   │       ├── crate-interview-playbook.xsd                       │
   │       ├── lisp-rdf-integration-playbook.toml                 │
   │       ├── lisp-rdf-integration-playbook.xsd                  │
   │       └── network-recon-playbook.toml                        │
   │                                                              │
   │   smart-crate-system/                                        │
   │   └── ctas7-smart-crate-orchestrator/                        │
   │       └── src/playbook_orchestrator.rs  ← Multi-Modal Exec   │
   │                                                              │
   │   ctas7-foundation-daemon/                                   │
   │   └── src/dsl/                                               │
   │       ├── playbook_executor.rs          ← Unicode Playbooks  │
   │       └── playbook_unicode.rs                                │
   │                                                              │
   │   ROOT LEVEL:                                                │
   │   ├── execute_xsd_playbook.sh           ← Loose script       │
   │   ├── orchestrate_all_services.sh       ← Loose script       │
   │   └── simple_orchestrator.sh            ← Loose script       │
   │                                                              │
   └─────────────────────────────────────────────────────────────┘
```

---

## Inventory of Playbook Systems

### 1. DSL-orchestration (CANONICAL for Smart Crate)

**Location:** `ctas7-ops-main-platform/DSL-orchestration/`
**Purpose:** Smart Crate playbook foundation (RFC-9101)
**Status:** CANONICAL for crate grouping

Files:
- `dsl-crate-grouping-system.dsl` - Master crate classification schema
- `dsl-port-integration.rs` - Port manager extension
- `ai-cli-integration.ts` - AI-CLI React component
- `playbooks/run-crate-grouping-playbook.sh`

### 2. Smart Crate Orchestrator (CANONICAL for Multi-Modal)

**Location:** `smart-crate-system/ctas7-smart-crate-orchestrator/`
**Purpose:** Multi-modal playbook execution (7 execution modes)
**Status:** CANONICAL for playbook execution engine

Features (from playbook_orchestrator.rs):
- XSD/XML Structure execution
- LISP Expression evaluation
- Trivariate Hash content-addressing
- RDF Triple graph traversal
- USIM Registry routing
- Unicode Assembly primitives
- Voice Commands (ElevenLabs + Zoe)

### 3. Foundation Daemon DSL (CANONICAL for Unicode Playbooks)

**Location:** `ctas7-foundation-daemon/src/dsl/`
**Purpose:** Unicode playbook execution with Neural Mux
**Status:** CANONICAL for Foundation layer execution

Files:
- `playbook_executor.rs` - Executes Unicode playbooks
- `playbook_unicode.rs` - Unicode playbook definitions

### 4. Loose Playbooks (DUPLICATE - MERGE INTO DSL-orchestration)

**Location:** `ctas7-ops-main-platform/playbooks/`
**Purpose:** Additional playbook definitions
**Status:** DUPLICATE - Should merge into DSL-orchestration

Files to migrate:
- `crate-interview-playbook.toml` → DSL-orchestration/playbooks/
- `crate-interview-playbook.xsd` → DSL-orchestration/playbooks/
- `lisp-rdf-integration-playbook.toml` → DSL-orchestration/playbooks/
- `lisp-rdf-integration-playbook.xsd` → DSL-orchestration/playbooks/
- `network-recon-playbook.toml` → DSL-orchestration/playbooks/

### 5. Root Level Scripts (DEPRECATE)

**Location:** Repository root
**Purpose:** Ad-hoc orchestration
**Status:** DEPRECATE - Functionality covered by canonical systems

Scripts to deprecate:
- `execute_xsd_playbook.sh` - Calls XSD Environment (port 18102)
- `orchestrate_all_services.sh` - Service orchestration
- `simple_orchestrator.sh` - Simple service coordination

---

## Canonical Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    PLAYBOOK ARCHITECTURE (TARGET STATE)              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  LAYER 1: PLAYBOOK DEFINITIONS                                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  DSL-orchestration/playbooks/  (SINGLE SOURCE OF TRUTH)        │ │
│  │  • Crate grouping (dsl-crate-grouping-system.dsl)              │ │
│  │  • Crate interview (crate-interview-playbook.*)                │ │
│  │  • LISP/RDF integration (lisp-rdf-integration-playbook.*)      │ │
│  │  • Network recon (network-recon-playbook.*)                    │ │
│  │  • Additional playbooks...                                     │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                              │                                       │
│                              ▼                                       │
│  LAYER 2: EXECUTION ENGINES                                          │
│  ┌──────────────────────┐  ┌──────────────────────────────────────┐ │
│  │ Foundation Daemon    │  │ Smart Crate Orchestrator             │ │
│  │ (Unicode Execution)  │  │ (Multi-Modal Execution)              │ │
│  │                      │  │                                      │ │
│  │ • playbook_executor  │  │ • XSD/XML structure                  │ │
│  │ • playbook_unicode   │  │ • LISP expressions                   │ │
│  │ • Neural Mux routing │  │ • Trivariate hash lookup             │ │
│  │                      │  │ • RDF graph traversal                │ │
│  │                      │  │ • USIM registry                      │ │
│  │                      │  │ • Unicode assembly                   │ │
│  │                      │  │ • Voice commands                     │ │
│  └──────────────────────┘  └──────────────────────────────────────┘ │
│                              │                                       │
│                              ▼                                       │
│  LAYER 3: INTEGRATION                                                │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Port Manager (18105) ← Service discovery                      │ │
│  │  Statistical CDN (18108) ← Metrics                             │ │
│  │  XSD Environment (18102) ← Schema validation                   │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Consolidation Plan

### Phase 1: Merge Duplicate Playbooks (Immediate)

```bash
# Move loose playbooks into DSL-orchestration
mv ctas7-ops-main-platform/playbooks/*.toml ctas7-ops-main-platform/DSL-orchestration/playbooks/
mv ctas7-ops-main-platform/playbooks/*.xsd ctas7-ops-main-platform/DSL-orchestration/playbooks/

# Keep the original run script (it's more comprehensive)
# Compare and merge run-crate-grouping-playbook.sh versions

# Remove now-empty directory
rmdir ctas7-ops-main-platform/playbooks/
```

### Phase 2: Archive Root-Level Scripts

```bash
# Create archive directory
mkdir -p scripts/deprecated-orchestration/

# Move deprecated scripts
mv execute_xsd_playbook.sh scripts/deprecated-orchestration/
mv orchestrate_all_services.sh scripts/deprecated-orchestration/
mv simple_orchestrator.sh scripts/deprecated-orchestration/

# Add deprecation notice
echo "DEPRECATED: Use DSL-orchestration or Smart Crate Orchestrator" > scripts/deprecated-orchestration/README.md
```

### Phase 3: Update References

1. Update any scripts that reference the old playbook locations
2. Update DSL-orchestration README to list all canonical playbooks
3. Add cross-references between Foundation Daemon DSL and Smart Crate Orchestrator

---

## Role Clarification

| System | Role | Canonical For |
|--------|------|---------------|
| **DSL-orchestration** | Playbook definitions, crate grouping | Definition storage |
| **Smart Crate Orchestrator** | Multi-modal execution engine | Complex playbook execution |
| **Foundation Daemon DSL** | Unicode playbook execution | Foundation layer primitives |
| **Port Manager** | Service discovery | Runtime coordination |

---

## NOT Duplicates (Intentional Separation)

These are NOT duplicates - they serve different layers:

1. **DSL-orchestration** - DEFINITIONS (the "what")
2. **Smart Crate Orchestrator** - EXECUTION (the "how" - multi-modal)
3. **Foundation Daemon DSL** - LOW-LEVEL (Unicode primitives for Neural Mux)

The orchestrator CONSUMES playbooks from DSL-orchestration and CAN delegate to Foundation Daemon for Unicode execution.

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-11-27 | Created consolidation plan | CTAS Engineering |
| 2025-11-27 | Identified 5 playbook systems | CTAS Engineering |
| 2025-11-27 | Defined canonical architecture | CTAS Engineering |
