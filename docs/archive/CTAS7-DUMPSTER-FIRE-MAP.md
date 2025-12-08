# 🔥 CTAS7/SYNAPTIX9 Nested Dumpster Fire Map

**Generated:** December 4, 2025  
**Purpose:** Understand the chaos before consolidating

---

## 🎯 THE CANONICAL TARGETS (Use These)

| Directory | Port | Status | Description |
|-----------|------|--------|-------------|
| `ctas7-command-center-canonical/` | 25175 | ✅ WORKING | **THE** canonical UI - use this |
| `synaptix9-workflow-system/` | 18350 | ✅ NEW | Clean target for workflow system |
| `ctas-6-6-mono/` | - | ✅ STABLE | Rust monorepo - core crates |

---

## 🗺️ TOP-LEVEL DIRECTORY MAP

```
/Users/cp5337/Developer/
│
├── 🟢 CANONICAL / PRODUCTION
│   ├── ctas7-command-center-canonical/     ← THE canonical UI (port 25175)
│   ├── synaptix9-workflow-system/          ← New workflow system target
│   └── ctas-6-6-mono/                      ← Rust monorepo (stable)
│
├── 🟡 STAGING / DEVELOPMENT  
│   └── ctas-7-shipyard-staging/            ← 130+ nested projects (THE DUMPSTER)
│       ├── ctas7-command-center/           ← DUPLICATE! Has SYNAPTIX9 UI
│       ├── ctas7-ops-main-platform/        ← DSL orchestration
│       ├── ctas7-exploit-arsenal/          ← 32K lines Rust (tools/tasks)
│       ├── ctas7-dioxus-docs-system/       ← Forge Rust integration
│       └── ... 126 more projects ...
│
├── 🟠 NON-CANONICAL / EXPERIMENTAL
│   ├── ctas7-command-center/               ← DUPLICATE! Different from staging
│   ├── sx9-development-center/             ← Another copy of canonical?
│   └── ctas7-retrograde/                   ← Legacy Rust workspace
│
├── 🔴 RECYCLE / DEPRECATED
│   └── ctas7-recycle/
│       ├── rcx-ctas7-broken-deprecated/    ← Old broken UI with N8NWorkflows
│       └── ... 20+ archived projects ...
│
└── 📦 ARCHIVES
    ├── ctas7-zip-archive/                  ← Compressed backups
    └── ABE-organized-systems/              ← Document organization
```

---

## 🔥 THE NESTED DUPLICATION PROBLEM

### `ctas7-command-center` exists in **4 PLACES**:

| Location | Has SYNAPTIX9? | Has Forge? | Status |
|----------|---------------|------------|--------|
| `/Developer/ctas7-command-center-canonical/` | ❌ No | ❌ No | **CANONICAL** |
| `/Developer/ctas7-command-center/` | ❌ No | ❌ No | Non-canonical |
| `/Developer/ctas-7-shipyard-staging/ctas7-command-center/` | ✅ YES | ✅ YES | **Has the good stuff!** |
| `/Developer/ctas7-retrograde/crates/ctas7-command-center/` | ❌ No | ❌ No | Legacy Rust |

### The Problem:
- **SYNAPTIX9 UI** (UniversalTopologyDesigner) is in **staging**, not canonical
- **Forge integration** is in **staging**, not canonical
- Easy to edit wrong copy and lose work
- No clear source of truth

---

## 📍 WHERE THINGS ACTUALLY LIVE

### Workflow System Components

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| **Forge Backend** | `synaptix9-workflow-system/tools/forge-backend/` | 200 | ✅ Canonical |
| **UniversalTopologyDesigner** | `ctas-7-shipyard-staging/ctas7-command-center/src/components/SYNAPTIX9/` | 2373 | ⚠️ NOT in canonical |
| **ForgeIntegrationService** | `ctas-7-shipyard-staging/ctas7-command-center/src/services/` | 251 | ⚠️ NOT in canonical |
| **SYNAPTIX9Context** | `ctas-7-shipyard-staging/ctas7-command-center/src/contexts/` | ? | ⚠️ NOT in canonical |

### Core Orchestration (Migrated to SX9)

| Component | Old Location | New Location | Status |
|-----------|--------------|--------------|--------|
| CTASOrchestrator → SX9Orchestrator | `ctas7-command-center/` | `synaptix9-workflow-system/packages/core/` | ✅ Migrated |
| LegionExecutionEngine | same | same | ✅ Migrated |
| SlotGraphQueryEngine | same | same | ✅ Migrated |
| ScriptExecutionCoordinator | same | same | ✅ Migrated |
| HashingEngineConnector | same | same | ✅ Migrated |

### Rust Crates (32K+ lines in staging)

| Crate | Location | Lines | Purpose |
|-------|----------|-------|---------|
| ctas7-exploit-arsenal | `ctas-7-shipyard-staging/` | 32,000+ | Task/tool graph execution |
| ctas7-dioxus-docs-system | `ctas-7-shipyard-staging/` | 1500+ | Forge Rust integration |
| ctas7-foundation-core | `ctas-7-shipyard-staging/` | ? | Core foundation |
| ctas7-slotgraph-engine | `ctas-7-shipyard-staging/` | ? | Graph engine |

---

## 🚨 DANGER ZONES

### Files That Look Similar But Aren't:

```
# These are ALL DIFFERENT:
/Developer/ctas7-command-center/src/App.tsx
/Developer/ctas7-command-center-canonical/src/App.tsx
/Developer/ctas-7-shipyard-staging/ctas7-command-center/src/App.tsx

# These services exist in different places:
/Developer/ctas7-command-center/src/services/CTASOrchestrator.ts
/Developer/ctas7-command-center-canonical/src/services/CTASOrchestrator.ts
# But ForgeIntegrationService ONLY exists in staging!
```

### Why We Keep Losing Canonical:
1. **Same directory names** in multiple parent folders
2. **IDE opens wrong one** based on recent files
3. **No visual distinction** between canonical/staging
4. **Staging has more features** so we work there
5. **Forget to merge back** to canonical

---

## 🎯 CONSOLIDATION PLAN

### Phase 1: Stabilize Canonical (NOW)
- [x] Verify canonical runs (port 25175) ✅
- [ ] Document what's missing from canonical
- [ ] Create migration checklist

### Phase 2: Migrate Missing Components
- [ ] Move SYNAPTIX9 UI to canonical or synaptix9-workflow-system
- [ ] Move ForgeIntegrationService
- [ ] Move SYNAPTIX9Context
- [ ] Update imports

### Phase 3: Clean Up Duplicates
- [ ] Archive staging command-center
- [ ] Remove non-canonical top-level copy
- [ ] Update all references

### Phase 4: Prevent Future Chaos
- [ ] Add NonCanonicalBanner to ALL non-canonical copies
- [ ] Create symlinks or aliases
- [ ] Document in README

---

## 📊 BY THE NUMBERS

| Category | Count |
|----------|-------|
| Total CTAS/Synaptix directories | 200+ |
| Directories in staging | 130+ |
| Duplicate command-centers | 4 |
| Recycled/deprecated projects | 20+ |
| Rust crates in staging | 50+ |
| Rust crates in retrograde | 25+ |
| Rust crates in mono | 15+ |

---

## 🔧 QUICK REFERENCE

### Start Canonical:
```bash
cd /Users/cp5337/Developer/ctas7-command-center-canonical
npm run dev  # Port 25175
```

### Start Forge Backend:
```bash
cd /Users/cp5337/Developer/synaptix9-workflow-system/tools/forge-backend
npm run dev  # Port 18350
```

### Find a Component:
```bash
# Search all locations
find /Users/cp5337/Developer -name "ComponentName*" 2>/dev/null | grep -v node_modules
```

---

**Remember:** When in doubt, check `/Developer/ctas7-command-center-canonical/` first!




