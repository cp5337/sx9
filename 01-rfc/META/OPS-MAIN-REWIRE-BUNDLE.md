# SX9 Ops-Main Rewire Bundle

**Recovered:** 2025-12-24  
**Status:** From conversation history  
**Source:** ATLASMonitor session (2025-12-23)

---

## Overview

Comprehensive route decluttering and Kill Chain implementation for `sx9-ops-main`.

---

## Current Route Inventory

### HD4 Core (5 routes) ✓
- `/hunt`
- `/detect`
- `/disable`
- `/disrupt`
- `/dominate`

### Infrastructure/Ops (6 routes)
- `/dvm`
- `/containers`
- `/database`
- `/setup-scripts`
- `/vkali`
- `/plasma`

### Intel/Analysis (5 routes)
- `/sectors`
- `/raptor`
- `/shodan`
- `/exploit-db`
- `/info-streams`

### Visualization (4 routes)
- `/map`
- `/map-test`
- `/graph`
- `/cognigraph`

### Utility (5 routes)
- `/tasks`
- `/quick-scripts`
- `/documentation`
- `/cli`
- `/settings`

---

## Routes to Delete

### Stubs (3 routes) - Just `<div>` placeholders
- `/critical-infrastructure` (keep for future - CA infrastructure dataset)
- `/kill-chain` → **REPLACED** with KillChain component
- `/kill-chain/:phase` → **REPLACED** with KillChain component

### Dev/Demo Routes (5 routes) - Candidates for removal
- `/demo-report` - DemoDataReportPage
- `/gallery` - Gallery (keep if useful)
- `/component-showcase` - **DUPLICATE** (points to Gallery)
- `/shared-components` - SharedComponentsDemo
- `/firefly` - FireflyPage (check status)

---

## Kill Chain Component

Mapping Lockheed Martin Cyber Kill Chain → HD4:

```tsx
// pages/KillChain/index.tsx

const phases = [
  { phase: 'Reconnaissance', hd4: 'hunt', path: '/hunt' },
  { phase: 'Weaponization', hd4: 'hunt', path: '/hunt' },
  { phase: 'Delivery', hd4: 'disrupt', path: '/disrupt' },
  { phase: 'Exploitation', hd4: 'disable', path: '/disable' },
  { phase: 'Installation', hd4: 'disable', path: '/disable' },
  { phase: 'C2', hd4: 'detect', path: '/detect' },
  { phase: 'Actions on Objectives', hd4: 'dominate', path: '/dominate' },
];

export const KillChain = () => {
  return (
    <div className="kill-chain-container">
      <h1>Kill Chain → HD4 Bridge</h1>
      <p>
        This view maps the traditional 7-phase Lockheed Martin Cyber Kill Chain
        to CTAS HD4 operational phases for operators trained on the legacy model.
      </p>
      
      <div className="phase-grid">
        {phases.map((p, i) => (
          <Link key={i} to={p.path} className={`phase-card hd4-${p.hd4}`}>
            <span className="phase-number">{i + 1}</span>
            <span className="phase-name">{p.phase}</span>
            <span className="hd4-mapping">→ {p.hd4.toUpperCase()}</span>
          </Link>
        ))}
      </div>
    </div>
  );
};

export default KillChain;
```

**Philosophy:** Kill chain is a UX bridge for operators trained on the 7-phase model. They get familiar terminology while operating in CTAS.

---

## Operational Context Architecture

Three orthogonal axes on the same graph:

| Axis | Values | Question |
|------|--------|----------|
| **Perspective** | Counter / Emulate | Am I stopping this or doing this? |
| **Framework** | HD4 / Kill Chain | How do I think about phases? |
| **Domain** | Cyber / Kinetic / Cognitive | What space am I operating in? |

---

## Frontend Component Status

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| **ATLASMonitor.tsx** | `/components/glaf/` | - | Reads static Supabase, needs live daemon wiring |
| **vKali** | `/components/` | ~707 | Tool execution, Class A Unicode routing |
| **KaliToolsIntegration** | `/components/` | ~483 | **DUPLICATE** - needs consolidation with vKali |
| **Cognigraph** | `/components/` | - | Visualization |
| **Gallery** | `/pages/` | - | Tool chain distribution |
| **KillChain** | `/pages/KillChain/` | - | Maps 7 Lockheed phases → HD4 |

---

## Backend Wiring Targets

| Service | Port | Frontend Status |
|---------|------|-----------------|
| Port Manager | 18103 | ✅ Can query |
| Gateway | 18120 | WebSocket client needed |
| ATLAS Daemon | 18106 | ATLASMonitor reads Supabase, not live |
| GLAF | 18050 | glaf_client.rs exists |
| Prompt Forge | 3001 | ✅ Working |

---

## Unicode Execution Classes

| Class | Range | Purpose |
|-------|-------|---------|
| A | U+E000-E0FF | Direct tool execution (Kali) |
| D | U+E400-E4FF | PTCC primitives |
| E | U+E700-E7FF | UI elements |

---

## Known Gaps

1. **ATLASMonitor** → polling Supabase instead of live daemon (18106)
2. **vKali + KaliToolsIntegration** → duplicate execution paths, need merge
3. **Gateway WebSocket client** → TypeScript client not wired
4. **Atomic Clipboard** → depends on port 18123, needs graceful degradation

---

## Declutter Summary

| Action | Routes | Status |
|--------|--------|--------|
| Keep | 25-27 | Core HD4, Infrastructure, Intel, Viz, Utility |
| Implement | 2 | Kill Chain mapping |
| Delete | 4-6 | Dev/demo cruft, duplicates |
| Future | 1 | Critical Infrastructure |

---

**Document Status:** RECOVERED  
**Recovery Date:** 2025-12-24
