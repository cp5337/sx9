# RFC-9150 Critical Analysis: Current State Assessment

## Executive Summary

Analysis of `sx9-forge/src/screens/PromptForgeScreen.tsx` (1785 lines) reveals systemic architectural failures requiring complete refactoring per RFC-9150.

## Severity Classification

### SEVERE (Blocking - Must Fix)

| ID | Problem | Current State | RFC-9150 Solution |
|----|---------|---------------|-------------------|
| S1 | **Inverted Information Architecture** | YAML output panel (lines 1200-1400) displayed before mission input | Three-rail layout: Intent -> Assembly -> Context |
| S2 | **Disconnected Navigation** | 6 icons in NavIconsColumn, 10 tabs in TabContent, no correlation | Single Intent rail with 4 logical sections |
| S3 | **50%+ Dead Functionality** | 22 stub/fake features identified (see list below) | Remove all non-functional elements |
| S4 | **Wrong Mental Model** | Form-based input gathering | Assembly line per RFC-9141 |
| S5 | **Cosmetic Right Rail** | Hardcoded agent stats, fake drift scores | Real-time data from sx9-harness |
| S6 | **No Semantic Drift Visualization** | Drift scores mentioned but not computed | DriftGauge component with 5 vectors |
| S7 | **No Real QA Integration** | QA gates are UI stubs only | Direct sx9-harness gate execution |

### MODERATE (High Priority)

| ID | Problem | Current State | RFC-9150 Solution |
|----|---------|---------------|-------------------|
| M1 | **Props Drilling Antipattern** | LeftContent receives 26 props | Zustand store, max 3 prop levels |
| M2 | **No YAML Validation** | Raw text display only | Schema-based validation |
| M3 | **No Error States** | Missing loading/error/empty states | Defined states for all async operations |
| M4 | **Accessibility Failures** | No ARIA labels, poor keyboard nav | WCAG 2.1 AA compliance |
| M5 | **No Responsive Design** | Fixed layout only | Breakpoint-based collapsible rails |

### MINOR (Should Fix)

| ID | Problem | Location |
|----|---------|----------|
| m1 | Hardcoded agent list | Line 89-120 |
| m2 | Inline styles mixed with Tailwind | Throughout |
| m3 | Magic numbers | Lines 450, 720, 890 |
| m4 | Duplicate color definitions | Lines 200-250 |
| m5 | Dead imports | Lines 1-30 |
| m6 | Unused state variables | Lines 150-180 |
| m7 | Hardcoded strings (no i18n) | Throughout |
| m8 | Missing loading spinners | Tab transitions |
| m9 | No confirmation dialogs | Destructive actions |
| m10 | Console.log statements | Lines 400, 650, 1100 |

## Dead Functionality Inventory

Components/features that exist in UI but do nothing:

```
1.  NavIconsColumn icons (6) - visual only, no handlers
2.  "Run Harness" button - logs to console
3.  "Export YAML" button - stub
4.  "Save Template" button - stub
5.  Agent capability badges - hardcoded display
6.  Drift score display - hardcoded "0.42"
7.  Token counter - always shows "1,234"
8.  Cost estimate - always shows "$0.02"
9.  History panel - empty list
10. Template library - empty list
11. Variable palette - no drag-drop
12. QA gate indicators - no backend calls
13. Semantic analysis tab - "Coming Soon" text
14. Pattern library tab - empty
15. Agent roster tab - partial implementation
16. Mission context tab - fields don't save
17. Right rail agent stats - fake data
18. Right rail recent prompts - fake data
19. Settings modal - opens but doesn't persist
20. Keyboard shortcuts - documented but not wired
21. Search functionality - input exists, no search
22. Filter dropdowns - UI only, no filtering
```

## Code Quality Issues

### Monolith Structure
```
PromptForgeScreen.tsx: 1785 lines
├── 45 useState hooks
├── 12 useEffect hooks
├── 26-prop component (LeftContent)
├── 8 inline component definitions
└── 0 extracted hooks
```

### State Management Chaos
```typescript
// Current: 45+ useState scattered across component
const [tab, setTab] = useState('mission');
const [harnessMode, setHarnessMode] = useState('full');
const [agent, setAgent] = useState('forge');
const [model, setModel] = useState('claude-3.5-sonnet');
// ... 41 more

// RFC-9150: Single Zustand store
const useForgeStore = create<ForgeState>((set) => ({
  mission: defaultMission,
  agent: defaultAgent,
  // ... consolidated state
}));
```

### Component Coupling
```
Current Prop Chain:
PromptForgeScreen
  └── LeftContent (26 props)
       └── TabContent (18 props)
            └── MissionTab (12 props)
                 └── InputField (8 props)

RFC-9150 Max Depth:
PromptForgeScreen
  └── IntentRail (store connection)
       └── MissionContext (direct store access)
```

## Information Architecture Comparison

### Current (Wrong)
```
┌────────────────────────────────────────────────────────────┐
│ [Icons] │ [Random Tabs] │ [YAML Output] │ [Fake Stats]    │
│         │               │               │                  │
│  ????   │  Form Fields  │  Generated    │  Agent: Forge   │
│         │               │  YAML here    │  Tokens: 1234   │
│         │               │               │  Cost: $0.02    │
└────────────────────────────────────────────────────────────┘
     ^           ^              ^               ^
     │           │              │               │
     No          No             OUTPUT          FAKE
     function    flow           BEFORE          DATA
                                INPUT
```

### RFC-9150 (Correct)
```
┌────────────────────────────────────────────────────────────┐
│                        HEADER                               │
│ [Mission] ──────────────────── [Agent] ─── [Run] [Export]  │
├──────────┬────────────────────────────────┬────────────────┤
│ INTENT   │       ASSEMBLY CANVAS          │   CONTEXT      │
│          │                                │                │
│ 1.Mission│   Variable Binding             │ Drift Gauge    │
│ 2.Vars   │   ────────────────             │ QA Gates       │
│ 3.Tmpl   │   Template Editor              │ Agent Skills   │
│ 4.History│   ────────────────             │ RFC Refs       │
│          │   Preview                      │                │
├──────────┴────────────────────────────────┴────────────────┤
│ [Static ✓] [Arch ✓] [Pattern ⏳] [Semantic ○]  Tokens Cost │
└────────────────────────────────────────────────────────────┘
     ^           ^              ^               ^
     │           │              │               │
     DEFINE      ASSEMBLE       PREVIEW         VALIDATE
     INTENT      PROMPT         OUTPUT          QUALITY
```

## Root Cause Analysis

### Why This Happened

1. **Prototype-to-Production**: UI was prototyped, then shipped without refactoring
2. **No Binding Specification**: Built before RFC-9141/9142 existed
3. **Feature Creep**: Features added without architectural review
4. **Missing QA Backend**: UI built before sx9-harness was ready
5. **Solo Development**: No code review on initial implementation

### Why It Persists

1. **Fear of Regression**: 1785 lines feels risky to touch
2. **No Test Coverage**: Zero tests for current implementation
3. **Unclear Ownership**: No designated UI architect
4. **Missing Metrics**: No performance/usage data to prioritize

## Migration Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Feature regression | High | High | Inventory all features before migration |
| User confusion | Medium | Medium | Maintain similar visual language |
| Extended timeline | High | Medium | Phase-based migration per RFC-9150 |
| API incompatibility | Low | High | Define API contract first |
| Performance issues | Low | Medium | Performance budgets in spec |

## Recommended Action

1. **Approve RFC-9150** as binding UI specification
2. **Create feature inventory** of what actually works today
3. **Phase 1 migration** - Extract Zustand store (non-breaking)
4. **Phase 2 migration** - Component extraction (incremental)
5. **Phase 3 migration** - API integration (adds functionality)
6. **Phase 4 migration** - Polish and accessibility

## Metrics for Success

| Metric | Current | Target |
|--------|---------|--------|
| Lines of code | 1785 | < 500 (split across files) |
| Main screen lines | 1785 | < 100 |
| Props depth | 4+ | <= 3 |
| useState hooks | 45 | 0 (use store) |
| Dead features | 22 | 0 |
| Test coverage | 0% | > 80% |
| WCAG compliance | Failed | AA |
| LCP | Unknown | < 2.5s |

---

**Document Status**: Analysis Complete
**Next Steps**: RFC-9150 review and approval
**Related**: RFC-9141, RFC-9142, CLSGS Annex A
