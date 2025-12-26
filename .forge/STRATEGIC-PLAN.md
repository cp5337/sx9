# SX9 Strategic Plan

**Created:** 2025-12-26
**Status:** ACTIVE - Blocking Issues Identified

---

## Chief Friction Points

### 1. Harness/Agent Mesh Not Automated
**Problem:** Built linear-agent and slack integration but only have random task pile. Nothing automated.
**Root Cause:** Infrastructure recovery has been VFR direct - reactive, not planned.
**Action Required:**
- [ ] Adjudicate old Linear tasks (close stale, archive noise)
- [ ] Start fresh with clean backlog
- [ ] Wire sx9-linear-agent to actually run autonomously
- [ ] Connect harness agents to Linear via MCP

### 2. UI Weak for Forge
**Problem:** Forge UI incomplete, GLAF partway. Agents broke → firefighting day.
**Root Cause:** No solid component pipeline to Vercel/Canva/Figma.
**Action Required:**
- [ ] Fix agent registry issues (found broken today)
- [ ] Set up Figma → Code Connect pipeline
- [ ] Deploy to Vercel for preview
- [ ] Establish UI component creation workflow

### 3. Critical Queue Blocked by Context Loss
**Problem:** Major math/tools/orbital/OSINT work cannot execute when context lapses.
**Root Cause:** MEMORY.md insufficient, development-center in separate repo, LLM doesn't retain session state.
**Action Required:**
- [ ] Consolidate critical repos or document cross-repo locations
- [ ] Expand MEMORY.md with execution state
- [ ] Use Linear as source of truth (not LLM memory)
- [ ] Big items need human checkpoints, not unattended runs

### 4. Linear Initiative/Project Structure
**Problem:** Structure exists but not being used for deliberate planning.

```
LINEAR INITIATIVES
├── CTAS (Threat/Security Ops)
│   ├── Tools (335 Kali tools, 5,065 matches)
│   ├── Plasma (Defender, ECS)
│   ├── OSINT (6,474 incidents, 32,327 USIMs)
│   └── GLAF/CONVERGE (Graph + Matroid selection)
│
├── DEV (Development)
│   ├── FORGE (Prompt Forge UI)
│   ├── Gallery (Component showcase)
│   ├── GLAF DEV (Graph dev tools)
│   ├── ops-main DEV (App development)
│   └── Orbital UI (Cesium integration)
│
└── ORBITAL (Space Domain)
    ├── Simulator (sx9-orbital-simulator)
    ├── Ground Stations (257 LaserLight FSO)
    └── Weather Engine (3 providers)
```

---

## Blocking Issues (Fix Before Big Runs)

| Issue | Impact | Fix |
|-------|--------|-----|
| development-center in separate repo | Context loss | Document in MEMORY, consider symlink or submodule |
| Linear tasks stale | Can't plan | Adjudicate: close/archive/keep |
| Agent registry broken | No automation | Fix sx9-harness agent loading |
| No Figma pipeline | Manual UI work | Wire Figma MCP + Code Connect |
| Forge not deployed | Can't share/review | Deploy to Vercel |

---

## Execution Order (Proposed)

### Phase 0: Stabilize (DO THIS FIRST)
1. Adjudicate Linear backlog - 30min human task
2. Document all cross-repo locations in MEMORY.md
3. Verify agent registry loads correctly

### Phase 1: Automate
1. Wire sx9-linear-agent to run on schedule
2. Connect Slack notifications
3. Set up Forge → Vercel deploy

### Phase 2: UI Pipeline
1. Wire Figma MCP server
2. Establish component creation workflow
3. Push Gallery to production

### Phase 3: Execute Big Items
Only after Phase 0-2:
- Tool runs with aligned data
- Orbital math implementation
- OSINT platform integration
- CONVERGE matroid algorithms

---

## Cross-Repo Locations (DO NOT LOSE)

| Component | Location | Notes |
|-----------|----------|-------|
| SmartCrateControl | `/Users/cp5337/Developer/sx9-development-center/src/components/SmartCrateControl.tsx` | 950-line dashboard |
| TacticalHUD | `/Users/cp5337/Developer/sx9-development-center/src/components/TacticalHUD.tsx` | Tactical display |
| Main workspace | `/Users/cp5337/Developer/sx9` | Primary repo |
| Shipyard staging | `/Users/cp5337/Developer/ctas-7-shipyard-staging` | Stub dirs, not built |

---

## Decision Points (Need Human Input)

1. **Linear Cleanup:** Should old tasks be archived or closed? Who decides relevance?
2. **development-center:** Merge into sx9 monorepo or keep separate?
3. **Unattended Runs:** What's the max duration before human checkpoint?
4. **Vercel Deploy:** What domain? synaptix9.com? sx9.dev?

---

## Today's Wins (Don't Lose)

- [x] Converge crates renamed to sx9-* prefix
- [x] Orbital simulator type conflicts fixed
- [x] Gateway wired to QA heartbeat (`/qa/heartbeat`)
- [x] MEMORY.md updated with architecture
- [x] Found SmartCrateControl in development-center
- [x] Documented app/task domain mapping
