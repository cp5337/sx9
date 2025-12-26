---
id: RFC-9010
title: Universal Rail System
status: Draft
created: 2025-12-26
scope: foundation
applies_to:
  - sx9-forge
  - sx9-ops-main
  - sx9-workbench
  - all future SX9 UIs
---

# RFC-9010: Universal Rail System

## Abstract

Defines a consistent three-rail layout system for all SX9 applications. This foundation RFC establishes proportions, behaviors, and interaction patterns that ensure visual consistency and usability across the platform.

## Problem Statement

Current SX9 applications have inconsistent layouts:
- **sx9-forge**: Cramped left rail, oversized center, inverted IA
- **sx9-ops-main**: Context-dependent sidebars with no standard widths
- **No shared components**: Each app implements its own layout primitives

This fragmentation causes:
1. Inconsistent user experience across apps
2. Duplicated layout code
3. Accessibility gaps
4. Responsive design failures

## Design Goals

1. **Universality**: Works for any SX9 application context
2. **Flexibility**: Configurable for different content needs
3. **Consistency**: Same mental model everywhere
4. **Accessibility**: Keyboard navigable, screen reader friendly
5. **Responsive**: Graceful degradation on smaller screens

## The Three-Rail Model

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                              HEADER BAR (56px)                                │
├────────────┬─────────────────────────────────────────────────┬───────────────┤
│            │                                                 │               │
│   ACTION   │                                                 │   CONTEXT     │
│   RAIL     │              PRIMARY CANVAS                     │   RAIL        │
│            │                                                 │               │
│  (Intent)  │              (Work Area)                        │  (Feedback)   │
│            │                                                 │               │
│   240px    │               flex-grow                         │    320px      │
│   fixed    │                                                 │    fixed      │
│            │                                                 │               │
├────────────┴─────────────────────────────────────────────────┴───────────────┤
│                              STATUS BAR (32px)                                │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Rail Specifications

### Header Bar (56px fixed height)

Purpose: Application identity, global navigation, primary actions.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ [Logo] [App Name]     [Context Title]     [Search]  [Actions]  [User/Menu]  │
└──────────────────────────────────────────────────────────────────────────────┘
   Zone A (200px)         Zone B (flex)        Zone C (400px max)
```

| Zone | Content | Alignment |
|------|---------|-----------|
| A | Logo, app name, back button | Left |
| B | Context-specific title, breadcrumbs | Center |
| C | Search, primary actions, user menu | Right |

### Action Rail (Left, 240px default)

Purpose: Define intent, select tools, navigate sections.

**Width States:**
| State | Width | Trigger |
|-------|-------|---------|
| Expanded | 240px | Default on desktop |
| Collapsed | 56px | User toggle or laptop breakpoint |
| Hidden | 0px | Tablet/mobile or user preference |

**Content Hierarchy:**
```
┌─────────────────────────┐
│ PRIMARY TABS            │ ← Section navigation (max 6)
├─────────────────────────┤
│                         │
│ SECTION CONTENT         │ ← Context-specific tools/forms
│                         │
│ - Grouped by function   │
│ - Scrollable if needed  │
│                         │
├─────────────────────────┤
│ SETTINGS / HELP         │ ← Always at bottom (pinned)
└─────────────────────────┘
```

**Tab Design:**
- Maximum 6 primary tabs
- Icons + labels when expanded
- Icons only when collapsed
- Active state clearly indicated
- Keyboard: Arrow keys navigate, Enter selects

### Primary Canvas (Center, flex-grow)

Purpose: Main work area where the primary task is performed.

**Characteristics:**
- Takes all remaining horizontal space
- Minimum width: 480px (below this, rails collapse)
- Can be subdivided into zones (horizontal or vertical)
- Scrolls independently from rails

**Common Subdivisions:**
```
Vertical Split (Editor + Preview):
┌─────────────────────────────────────────┐
│                                         │
│              EDITOR (60%)               │
│                                         │
├─────────────────────────────────────────┤
│              PREVIEW (40%)              │
└─────────────────────────────────────────┘

Horizontal Split (List + Detail):
┌──────────────────┬──────────────────────┐
│                  │                      │
│   LIST (35%)     │     DETAIL (65%)     │
│                  │                      │
└──────────────────┴──────────────────────┘
```

### Context Rail (Right, 320px default)

Purpose: Provide feedback, metadata, and supplementary information.

**Width States:**
| State | Width | Trigger |
|-------|-------|---------|
| Expanded | 320px | Default on desktop |
| Collapsed | 56px | User toggle or laptop breakpoint |
| Hidden | 0px | Tablet/mobile or user preference |

**Content Hierarchy:**
```
┌─────────────────────────┐
│ CONTEXT TABS            │ ← Max 4 tabs
├─────────────────────────┤
│                         │
│ PRIMARY FEEDBACK        │ ← Most important info (always visible)
│ (Status, Scores, etc.)  │
│                         │
├─────────────────────────┤
│                         │
│ SECONDARY INFO          │ ← Supporting details (scrollable)
│ (Metadata, History)     │
│                         │
├─────────────────────────┤
│ ACTIONS                 │ ← Context-specific actions (pinned)
└─────────────────────────┘
```

### Status Bar (32px fixed height)

Purpose: System-wide status, background processes, quick metrics.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ [Indicators]          [Status Message]          [Metrics]  [Connection]     │
└──────────────────────────────────────────────────────────────────────────────┘
   Zone A (300px)         Zone B (flex)              Zone C (300px)
```

| Zone | Content | Examples |
|------|---------|----------|
| A | State indicators | QA gates, process status |
| B | Status messages | "Saved", "Processing...", errors |
| C | Metrics, connectivity | Token count, cost, online/offline |

## Responsive Behavior

### Breakpoints

| Name | Width | Behavior |
|------|-------|----------|
| Desktop | >= 1440px | Full three-rail layout |
| Laptop | 1024-1439px | Rails collapsible, default collapsed |
| Tablet | 768-1023px | Rails as overlays |
| Mobile | < 768px | Rails as full-screen modals |

### Collapse Behavior

```
Desktop (>= 1440px):
[Action Rail 240px] [Canvas flex] [Context Rail 320px]

Laptop (1024-1439px):
[56px] [Canvas flex] [56px]  ← Collapsed by default, expand on click

Tablet (768-1023px):
[Canvas 100%] + [Overlay rails on demand]

Mobile (< 768px):
[Canvas 100%] + [Full-screen modal rails]
```

### Rail Toggle Controls

```
┌─────────┐                                               ┌─────────┐
│ ◀ │ ▶   │ ← Toggle in header or rail edge              │  ◀ │ ▶  │
└─────────┘                                               └─────────┘
  Action                                                     Context
```

## Component API

### ThreeRailLayout

```typescript
interface ThreeRailLayoutProps {
  // Header
  header?: React.ReactNode;
  headerHeight?: number; // default: 56

  // Action Rail (Left)
  actionRail?: React.ReactNode;
  actionRailWidth?: number; // default: 240
  actionRailCollapsedWidth?: number; // default: 56
  actionRailDefaultCollapsed?: boolean;

  // Primary Canvas (Center)
  children: React.ReactNode;
  canvasMinWidth?: number; // default: 480

  // Context Rail (Right)
  contextRail?: React.ReactNode;
  contextRailWidth?: number; // default: 320
  contextRailCollapsedWidth?: number; // default: 56
  contextRailDefaultCollapsed?: boolean;

  // Status Bar
  statusBar?: React.ReactNode;
  statusBarHeight?: number; // default: 32

  // Behavior
  breakpoints?: BreakpointConfig;
  persistState?: boolean; // Remember collapsed state
  storageKey?: string; // LocalStorage key for state
}

interface BreakpointConfig {
  desktop: number; // default: 1440
  laptop: number;  // default: 1024
  tablet: number;  // default: 768
}
```

### Rail Components

```typescript
// Action Rail
interface ActionRailProps {
  tabs: RailTab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  children: React.ReactNode; // Tab content
  footer?: React.ReactNode; // Settings/help section
}

// Context Rail
interface ContextRailProps {
  tabs?: RailTab[]; // Optional - can be single-content
  activeTab?: string;
  onTabChange?: (tabId: string) => void;
  children: React.ReactNode;
  actions?: React.ReactNode; // Bottom action buttons
}

// Tab definition
interface RailTab {
  id: string;
  label: string;
  icon: React.ReactNode;
  badge?: string | number; // Notification badge
}
```

### Hooks

```typescript
// Access rail state from any child component
function useRailState(): {
  actionRailCollapsed: boolean;
  contextRailCollapsed: boolean;
  toggleActionRail: () => void;
  toggleContextRail: () => void;
  breakpoint: 'desktop' | 'laptop' | 'tablet' | 'mobile';
}

// Access canvas dimensions
function useCanvasDimensions(): {
  width: number;
  height: number;
  availableWidth: number; // Minus rail widths
}
```

## Application Mappings

### sx9-forge (Prompt Forge)

```
Action Rail:
  - Tabs: Mission, Variables, Templates, History
  - Content: Context-specific forms/lists
  - Footer: Settings, Keyboard Shortcuts

Primary Canvas:
  - Variable Binding Zone (25%)
  - Template Editor (50%)
  - Preview Pane (25%)

Context Rail:
  - Tabs: QA, Intel, Tools, Refs
  - Primary: Drift Gauge, Gate Status
  - Secondary: Agent capabilities, RFC links
```

### sx9-ops-main (Operations)

```
Action Rail:
  - Tabs: HD4 phases (Hunt, Detect, Disable, Disrupt, Dominate)
  - Content: Phase-specific tools
  - Footer: Settings, Documentation

Primary Canvas:
  - Varies by phase (Map, Graph, List views)

Context Rail:
  - Tabs: Intel, Threats, Tasks, Alerts
  - Primary: Active threat indicators
  - Secondary: Related entities, history
```

### sx9-workbench (Internal Tools)

```
Action Rail:
  - Tabs: Dashboard, RFCs, Agents, Linear
  - Content: Navigation lists
  - Footer: Offline status, Sync

Primary Canvas:
  - Content viewer/editor

Context Rail:
  - Tabs: Status, Logs
  - Primary: System health
  - Secondary: Recent activity
```

## Accessibility Requirements

### Keyboard Navigation

| Key | Action |
|-----|--------|
| `[` | Toggle action rail |
| `]` | Toggle context rail |
| `Tab` | Move through rail tabs |
| `Enter` | Select tab |
| `Escape` | Close expanded overlay (tablet/mobile) |
| `Ctrl+1-6` | Jump to action rail tab 1-6 |
| `Ctrl+Shift+1-4` | Jump to context rail tab 1-4 |

### ARIA Landmarks

```html
<header role="banner">...</header>
<nav role="navigation" aria-label="Action rail">...</nav>
<main role="main">...</main>
<aside role="complementary" aria-label="Context rail">...</aside>
<footer role="contentinfo">...</footer>
```

### Focus Management

- Focus trap when rail is overlay mode
- Return focus to trigger when rail closes
- Visible focus indicators (2px ring)
- Skip links for rail bypass

## Implementation Plan

### Phase 1: Core Components
```
packages/ui/src/layout/
├── ThreeRailLayout.tsx
├── ActionRail.tsx
├── ContextRail.tsx
├── Header.tsx
├── StatusBar.tsx
├── RailTab.tsx
└── hooks/
    ├── useRailState.ts
    └── useCanvasDimensions.ts
```

### Phase 2: App Integration
1. Create layout wrapper for sx9-forge
2. Migrate ops-main to use shared layout
3. Build sx9-workbench using layout from start

### Phase 3: Design Tokens
```typescript
// packages/ui/src/tokens/layout.ts
export const layoutTokens = {
  rail: {
    action: {
      width: 240,
      collapsedWidth: 56,
      minTabs: 2,
      maxTabs: 6,
    },
    context: {
      width: 320,
      collapsedWidth: 56,
      minTabs: 1,
      maxTabs: 4,
    },
  },
  header: {
    height: 56,
  },
  statusBar: {
    height: 32,
  },
  canvas: {
    minWidth: 480,
  },
  breakpoints: {
    desktop: 1440,
    laptop: 1024,
    tablet: 768,
  },
};
```

## Success Criteria

1. **Consistency**: All three apps use same layout component
2. **Proportions**: Fixed rails, flex canvas - no cramping
3. **Responsive**: Graceful collapse at each breakpoint
4. **Accessible**: Full keyboard navigation, ARIA landmarks
5. **Performant**: No layout shift, smooth transitions
6. **Documented**: Storybook examples for all variants

## References

- RFC-9150: Prompt Forge UI Specification
- Material Design 3: Navigation rail
- Apple HIG: Sidebars
- Atlassian Design System: Page layouts

---

**Status**: Draft
**Author**: SX9 Architecture Team
**Review Required**: UI/UX, Frontend Engineering
