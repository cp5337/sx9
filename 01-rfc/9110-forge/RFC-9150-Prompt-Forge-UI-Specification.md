---
id: RFC-9150
title: Prompt Forge UI Specification
status: Draft
created: 2025-12-26
supersedes: null
binding_rfcs:
  - RFC-9141 (Assembly Line & Dual-Heartbeat QA)
  - RFC-9142 (Semantic Drift Scoring & Gates)
  - CLSGS Annex A (N-V-N-N Behavioral Scope)
---

# RFC-9150: Prompt Forge UI Specification

## Abstract

This RFC defines the user interface specification for Prompt Forge, the primary prompt assembly workbench in the SX9 system. It establishes information architecture, component structure, and interaction patterns aligned with RFC-9141's "prompts are assembled, not authored" principle.

## Problem Statement

The current PromptForgeScreen implementation (1785 lines) exhibits critical architectural failures:

| Category | Count | Impact |
|----------|-------|--------|
| Severe | 7 | Fundamental IA inversion, dead code |
| Moderate | 5 | Props drilling, no validation |
| Minor | 10+ | Style drift, hardcoded values |

### Root Causes

1. **Inverted Information Architecture**: YAML output displayed before mission input
2. **Wrong Mental Model**: Treats prompt creation as form-filling, not assembly
3. **Dead Functionality**: ~50% of UI elements are stubs or cosmetic
4. **Monolithic Structure**: 26-prop component drilling, no state management
5. **No RFC Integration**: QA gates, semantic drift scoring not connected

## Design Principles

### P1: Assembly Line Model (RFC-9141)

```
Intent → Variable Selection → Prompt Assembly → Artifact Production
                ↓                    ↓                   ↓
        Parameter Binding     QA Gate Check      Output Generation
```

The UI MUST follow this flow left-to-right, top-to-bottom.

### P2: Dual-Heartbeat QA (RFC-9141 §4)

Two concurrent validation streams:
- **Sync Heartbeat**: Real-time linting during assembly
- **Async Heartbeat**: Background QA gate execution

### P3: Semantic Drift Visibility (RFC-9142)

Drift vectors MUST be visible at all times:
- Role Drift (RD)
- Constraint Drift (CD)
- Coupling Drift (CpD)
- Authority Drift (AD)
- Pattern Drift (PD)

### P4: Progressive Disclosure

Show complexity only when needed. Default view is minimal.

## Information Architecture

### Three-Rail Layout

```
┌─────────────────────────────────────────────────────────────────────────┐
│                            HEADER BAR                                    │
│  [Forge Logo]  Mission: ____________  Agent: [Forge ▼]  [Run] [Export]  │
├─────────────┬───────────────────────────────────────┬───────────────────┤
│             │                                       │                   │
│   INTENT    │           ASSEMBLY CANVAS             │    CONTEXT        │
│   RAIL      │                                       │    RAIL           │
│   (240px)   │           (flex-grow)                 │    (320px)        │
│             │                                       │                   │
│  ┌────────┐ │  ┌─────────────────────────────────┐  │  ┌─────────────┐  │
│  │Mission │ │  │                                 │  │  │ Drift Score │  │
│  │Context │ │  │     Variable Binding Zone      │  │  │   Gauge     │  │
│  └────────┘ │  │                                 │  │  └─────────────┘  │
│             │  └─────────────────────────────────┘  │                   │
│  ┌────────┐ │                                       │  ┌─────────────┐  │
│  │Variable│ │  ┌─────────────────────────────────┐  │  │ QA Gates    │  │
│  │Palette │ │  │                                 │  │  │ Status      │  │
│  └────────┘ │  │     Template Editor             │  │  └─────────────┘  │
│             │  │     (Monaco)                    │  │                   │
│  ┌────────┐ │  │                                 │  │  ┌─────────────┐  │
│  │Template│ │  └─────────────────────────────────┘  │  │ Agent       │  │
│  │Library │ │                                       │  │ Capabilities│  │
│  └────────┘ │  ┌─────────────────────────────────┐  │  └─────────────┘  │
│             │  │     Preview / Output            │  │                   │
│  ┌────────┐ │  │                                 │  │  ┌─────────────┐  │
│  │History │ │  └─────────────────────────────────┘  │  │ RFC Refs    │  │
│  └────────┘ │                                       │  └─────────────┘  │
│             │                                       │                   │
├─────────────┴───────────────────────────────────────┴───────────────────┤
│                            STATUS BAR                                    │
│  [Static ✓] [Arch ✓] [Pattern ⏳] [Semantic ○]  Tokens: 1,234  Cost: $0.02│
└─────────────────────────────────────────────────────────────────────────┘
```

### Rail Specifications

#### Intent Rail (Left, 240px fixed)

Purpose: Define what the prompt should accomplish.

| Section | Content | State |
|---------|---------|-------|
| Mission Context | Mission name, behavioral scope (N-V-N-N) | Required |
| Variable Palette | Available variables for binding | Interactive |
| Template Library | Saved prompt templates | Collapsible |
| History | Recent prompt assemblies | Collapsible |

#### Assembly Canvas (Center, flex-grow)

Purpose: Construct the prompt through variable binding and template editing.

| Zone | Function | Interaction |
|------|----------|-------------|
| Variable Binding | Drag-drop variables into slots | Drag-and-drop |
| Template Editor | Monaco editor with prompt syntax | Direct editing |
| Preview | Live-rendered prompt output | Read-only |

#### Context Rail (Right, 320px fixed)

Purpose: Real-time feedback and reference information.

| Section | Content | Update Frequency |
|---------|---------|------------------|
| Drift Score Gauge | Composite drift score (0-100) | 500ms debounce |
| QA Gates Status | Four gate indicators | Per-gate completion |
| Agent Capabilities | Selected agent's skills | On agent change |
| RFC References | Relevant RFCs for context | Static |

## Component Architecture

### State Management (Zustand)

```typescript
interface ForgeState {
  // Intent
  mission: MissionContext;
  agent: AgentSelection;
  behavioralScope: NvnnScope;

  // Assembly
  template: PromptTemplate;
  variables: BoundVariable[];

  // Validation
  driftScores: DriftVector[];
  qaGates: QaGateStatus[];

  // Output
  renderedPrompt: string;
  artifacts: Artifact[];

  // Actions
  bindVariable: (variable: Variable, slot: Slot) => void;
  updateTemplate: (content: string) => void;
  runQaGate: (gate: QaGate) => Promise<GateResult>;
  assemblePrompt: () => Promise<AssembledPrompt>;
}
```

### Component Hierarchy

```
<ForgeScreen>
├── <Header>
│   ├── <MissionInput />
│   ├── <AgentSelector />
│   └── <ActionButtons />
├── <ThreeRailLayout>
│   ├── <IntentRail>
│   │   ├── <MissionContext />
│   │   ├── <VariablePalette />
│   │   ├── <TemplateLibrary />
│   │   └── <HistoryPanel />
│   ├── <AssemblyCanvas>
│   │   ├── <VariableBindingZone />
│   │   ├── <TemplateEditor />
│   │   └── <PreviewPane />
│   └── <ContextRail>
│       ├── <DriftGauge />
│       ├── <QaGatePanel />
│       ├── <AgentCapabilities />
│       └── <RfcReferences />
└── <StatusBar>
    ├── <GateIndicators />
    ├── <TokenCounter />
    └── <CostEstimate />
```

### File Structure

```
sx9-forge/src/
├── screens/
│   └── PromptForgeScreen.tsx      # Main screen (< 100 lines)
├── features/
│   └── forge/
│       ├── store.ts               # Zustand state
│       ├── hooks/
│       │   ├── useForge.ts        # Main hook
│       │   ├── useQaGates.ts      # QA integration
│       │   └── useDriftScoring.ts # Drift calculation
│       ├── components/
│       │   ├── Header.tsx
│       │   ├── IntentRail/
│       │   │   ├── index.tsx
│       │   │   ├── MissionContext.tsx
│       │   │   ├── VariablePalette.tsx
│       │   │   ├── TemplateLibrary.tsx
│       │   │   └── HistoryPanel.tsx
│       │   ├── AssemblyCanvas/
│       │   │   ├── index.tsx
│       │   │   ├── VariableBindingZone.tsx
│       │   │   ├── TemplateEditor.tsx
│       │   │   └── PreviewPane.tsx
│       │   ├── ContextRail/
│       │   │   ├── index.tsx
│       │   │   ├── DriftGauge.tsx
│       │   │   ├── QaGatePanel.tsx
│       │   │   ├── AgentCapabilities.tsx
│       │   │   └── RfcReferences.tsx
│       │   └── StatusBar.tsx
│       └── types.ts
└── lib/
    ├── api/
    │   ├── forge.ts               # Backend API calls
    │   └── qa.ts                  # QA gate API
    └── utils/
        ├── promptRenderer.ts      # Variable interpolation
        └── driftCalculator.ts     # Drift vector math
```

## Data Types

### MissionContext

```typescript
interface MissionContext {
  id: string;
  name: string;
  description: string;
  behavioralScope: NvnnScope;
  constraints: Constraint[];
  createdAt: Date;
  updatedAt: Date;
}

interface NvnnScope {
  role: NvnnRole;       // FACTORY, ANALYZER, ORCHESTRATOR, etc.
  action: string;       // generate, analyze, validate, etc.
  constraint: string;   // rust_crate, algorithm, etc.
  object: string;       // source_code, computation, etc.
}

type NvnnRole =
  | 'FACTORY'
  | 'ANALYZER'
  | 'ORCHESTRATOR'
  | 'MONITOR'
  | 'VALIDATOR';
```

### PromptTemplate

```typescript
interface PromptTemplate {
  id: string;
  name: string;
  content: string;
  slots: TemplateSlot[];
  metadata: TemplateMetadata;
}

interface TemplateSlot {
  id: string;
  name: string;
  type: SlotType;
  required: boolean;
  defaultValue?: string;
  validation?: SlotValidation;
}

type SlotType =
  | 'text'
  | 'code'
  | 'file'
  | 'agent'
  | 'rfc'
  | 'variable';
```

### DriftVector (RFC-9142)

```typescript
interface DriftVector {
  type: DriftType;
  score: number;        // 0.0 - 1.0
  threshold: number;    // Gate threshold
  status: DriftStatus;
  details: string;
}

type DriftType =
  | 'role'        // RD
  | 'constraint'  // CD
  | 'coupling'    // CpD
  | 'authority'   // AD
  | 'pattern';    // PD

type DriftStatus =
  | 'observe'     // score < 0.3
  | 'warn'        // 0.3 <= score < 0.6
  | 'gate'        // 0.6 <= score < 0.8
  | 'escalate';   // score >= 0.8
```

### QaGateStatus

```typescript
interface QaGateStatus {
  gate: QaGate;
  status: GateStatus;
  result?: GateResult;
  duration?: number;
  timestamp?: Date;
}

type QaGate = 'static' | 'arch' | 'pattern' | 'semantic';

type GateStatus =
  | 'pending'     // Not yet run
  | 'running'     // In progress
  | 'passed'      // Green
  | 'warned'      // Yellow (advisory)
  | 'failed'      // Red (blocking)
  | 'skipped';    // Gray (not applicable)
```

## Interaction Patterns

### 1. Mission Definition Flow

```
User enters mission name
       ↓
System suggests N-V-N-N scope based on keywords
       ↓
User confirms or modifies behavioral scope
       ↓
System loads relevant templates and variables
```

### 2. Variable Binding Flow

```
User drags variable from palette
       ↓
System highlights compatible slots in template
       ↓
User drops variable into slot
       ↓
System validates binding, updates preview
       ↓
Drift scores recalculate (debounced 500ms)
```

### 3. QA Gate Execution Flow

```
User clicks [Run] or reaches checkpoint
       ↓
Static gate runs first (blocking)
       ↓
If passed: Arch gate runs
       ↓
If passed: Pattern gate runs
       ↓
If passed: Semantic gate runs
       ↓
All passed: Enable artifact production
```

### 4. Template Editing Flow

```
User types in Monaco editor
       ↓
Syntax highlighting updates (immediate)
       ↓
Preview updates (debounced 300ms)
       ↓
Drift scores update (debounced 500ms)
       ↓
Sync heartbeat validates (debounced 1000ms)
```

## API Integration

### Backend Endpoints

```typescript
// Forge Backend (Port 18350)
const API = {
  // Templates
  listTemplates: () => GET('/api/templates'),
  getTemplate: (id: string) => GET(`/api/templates/${id}`),
  saveTemplate: (data: TemplateCreate) => POST('/api/templates', data),

  // Variables
  listVariables: (scope: NvnnScope) => GET('/api/variables', { scope }),

  // Assembly
  renderPrompt: (data: RenderRequest) => POST('/api/render', data),

  // QA Gates
  runGate: (gate: QaGate, content: string) => POST(`/api/qa/${gate}`, { content }),

  // Agents
  listAgents: () => GET('/api/agents'),
  getAgentCapabilities: (id: string) => GET(`/api/agents/${id}/capabilities`),
};
```

### WebSocket Events

```typescript
// Real-time updates via WebSocket
interface ForgeSocketEvents {
  // From server
  'qa:gate:started': { gate: QaGate };
  'qa:gate:completed': { gate: QaGate; result: GateResult };
  'drift:updated': { vectors: DriftVector[] };
  'agent:status': { agentId: string; status: AgentStatus };

  // From client
  'subscribe:session': { sessionId: string };
  'unsubscribe:session': { sessionId: string };
}
```

## Accessibility Requirements

### WCAG 2.1 AA Compliance

| Criterion | Requirement | Implementation |
|-----------|-------------|----------------|
| 1.4.3 | Contrast ratio >= 4.5:1 | Use design tokens |
| 2.1.1 | Keyboard accessible | Tab order, focus management |
| 2.4.7 | Focus visible | Ring outline on all interactive |
| 4.1.2 | Name, role, value | ARIA labels on all controls |

### Keyboard Navigation

```
Tab         → Move focus forward through controls
Shift+Tab   → Move focus backward
Enter       → Activate focused control
Escape      → Close modals/popovers
Ctrl+S      → Save template
Ctrl+Enter  → Run QA gates
Ctrl+Shift+P → Open command palette
```

## Responsive Behavior

### Breakpoints

| Breakpoint | Width | Layout |
|------------|-------|--------|
| Desktop | >= 1280px | Three-rail |
| Laptop | 1024-1279px | Collapsible rails |
| Tablet | 768-1023px | Stacked rails |
| Mobile | < 768px | Not supported (warning) |

### Collapsible Rails

At laptop breakpoint:
- Intent rail collapses to icon-only (48px)
- Context rail collapses to icon-only (48px)
- Click expands to overlay panel

## Performance Budgets

| Metric | Budget | Measurement |
|--------|--------|-------------|
| First Contentful Paint | < 1.5s | Lighthouse |
| Time to Interactive | < 3.0s | Lighthouse |
| Largest Contentful Paint | < 2.5s | Lighthouse |
| Total Bundle Size | < 500KB | gzipped |
| Monaco Editor Chunk | < 200KB | gzipped, lazy |

## Migration Strategy

### Phase 1: State Extraction (Week 1)
1. Create Zustand store with current state shape
2. Add store provider to existing component
3. Migrate useState calls to store

### Phase 2: Component Extraction (Week 2-3)
1. Extract Header component
2. Extract IntentRail components
3. Extract AssemblyCanvas components
4. Extract ContextRail components
5. Extract StatusBar component

### Phase 3: API Integration (Week 4)
1. Replace mock data with real API calls
2. Add WebSocket connection for real-time updates
3. Integrate with sx9-harness QA gates

### Phase 4: Polish (Week 5)
1. Add keyboard navigation
2. Add accessibility labels
3. Add responsive behavior
4. Performance optimization

## Test Strategy

### Unit Tests
- Each component has isolated tests
- Store actions have pure function tests
- Utility functions have comprehensive tests

### Integration Tests
- Rail interactions work together
- API calls succeed and update state
- WebSocket events update UI

### E2E Tests (Playwright)
- Full assembly flow works
- QA gates execute and block/allow
- Export produces valid output

## Success Criteria

1. **Component Count**: Main screen < 100 lines
2. **Props Depth**: Maximum 3 levels of prop drilling
3. **State Coupling**: All state in Zustand store
4. **Feature Coverage**: 100% of RFC-9141 assembly line
5. **QA Integration**: Real sx9-harness gate execution
6. **Drift Visibility**: All 5 drift vectors displayed
7. **Accessibility**: WCAG 2.1 AA compliant
8. **Performance**: All budgets met

## References

- RFC-9141: FORGE Assembly Line & Dual-Heartbeat QA Doctrine
- RFC-9142: Semantic Drift Scoring & Gates
- CLSGS Annex A: N-V-N-N Behavioral Scope
- RFC-9120: Prompt Forge v4
- RFC-9140: FORGE Unified Architecture

---

**Status**: Draft
**Author**: SX9 Architecture Team
**Review Required**: UI/UX, Engineering Lead
