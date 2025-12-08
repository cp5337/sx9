# GLAF Graph Browser - v0 Prompt

## Paste Into v0.dev

---

Build a **Neo4j-style graph database browser** with these exact specifications:

## Tech Stack
- Next.js 14 App Router
- TypeScript
- Tailwind CSS (dark theme only)
- D3.js for force-directed graph
- Lucide React icons
- shadcn/ui components

## Color Palette (REQUIRED)
```css
--bg-void: #050508
--bg-primary: #0a0a0f
--bg-secondary: #12121a
--bg-tertiary: #1a1a24
--cyan: #00ffff
--magenta: #ff00ff
--amber: #ffbf00
--lime: #00ff88
```

## Layout

```
┌────────────────────────────────────────────────────────────────────┐
│ GLAF Graph Browser                              [Layout▼] [Export] │
├────────────────────────────────────────────────────────────────────┤
│ ┌────────────────────────────────────────────────────────────────┐ │
│ │ Find [label▼] [equals▼] [________] [+Filter]        [Search]  │ │
│ └────────────────────────────────────────────────────────────────┘ │
├────────┬─────────────────────────────────────────────┬─────────────┤
│ FILTER │              GRAPH CANVAS                   │   DETAILS   │
│        │                                             │             │
│ Labels │     ⬡ ════════════ ⬢                       │ ┌─────────┐ │
│ ☑Agent │     Agent         Slot                     │ │  Agent  │ │
│ ☑Slot  │         ╲                                  │ │─────────│ │
│ ☐Tool  │          ╲   ◇                             │ │name:    │ │
│        │           ╲ Tool                           │ │ Alpha   │ │
│ Types  │                                             │ │         │ │
│ ☑ALLOC │                                             │ │status:  │ │
│ ☐EXEC  │                                             │ │ ACTIVE  │ │
│        │  [Force] [Tree] [Radial]      Zoom: 100%   │ └─────────┘ │
└────────┴─────────────────────────────────────────────┴─────────────┘
```

## Node Shapes (CRITICAL)

Each entity type has a unique shape:

| Label | Shape | Color | Icon |
|-------|-------|-------|------|
| Agent | Hexagon (6 sides) | #00ffff | Bot |
| Slot | Octagon (8 sides) | #ff00ff | Grid3x3 |
| Tool | Diamond | #ffbf00 | Wrench |
| Hash | Nonagon (9 sides) | #00ff88 | Hash |
| Database | Cylinder | #888888 | Database |
| Service | Rounded rect | #448888 | Server |

## Relationship Styles

| Type | Color | Style |
|------|-------|-------|
| ALLOCATED_TO | #00ffff | Solid, animated |
| EXECUTES | #ff00ff | Solid thick |
| DEPENDS_ON | #ffbf00 | Dashed |
| HASHES | #00ff88 | Thin dashed |

## Required Features

1. **D3 Force Simulation**
   - Nodes attract/repel naturally
   - Drag to move and pin nodes
   - Double-click to unpin
   - Scroll to zoom, drag canvas to pan

2. **Visual Query Builder**
   - Dropdowns: label, operator (equals/contains/greater), value
   - Add multiple filter conditions
   - Generate query on Search click

3. **Left Panel - Filters**
   - Checkbox list of node labels with counts
   - Checkbox list of relationship types
   - Click to filter visible graph

4. **Right Panel - Details**
   - Shows selected node properties
   - Lists relationships (incoming/outgoing counts)
   - Edit/Delete buttons

5. **Canvas Interactions**
   - Click node → select, show in right panel
   - Double-click node → expand connected nodes
   - Right-click → context menu (Expand, Hide, Delete)

## Sample Data (use this for demo)

```typescript
const nodes = [
  { id: '1', label: 'Agent', properties: { name: 'Alpha', status: 'ACTIVE', priority: 3 } },
  { id: '2', label: 'Agent', properties: { name: 'Beta', status: 'IDLE', priority: 1 } },
  { id: '3', label: 'Slot', properties: { index: 0, status: 'READY' } },
  { id: '4', label: 'Slot', properties: { index: 1, status: 'EXECUTING' } },
  { id: '5', label: 'Tool', properties: { name: 'nmap', category: 'recon' } },
  { id: '6', label: 'Tool', properties: { name: 'nuclei', category: 'vuln' } },
  { id: '7', label: 'Hash', properties: { canonical: 'triv:7F3A_CUID_UUID', type: 'trivariate' } },
];

const relationships = [
  { id: 'r1', type: 'ALLOCATED_TO', source: '1', target: '3' },
  { id: 'r2', type: 'ALLOCATED_TO', source: '2', target: '4' },
  { id: 'r3', type: 'EXECUTES', source: '1', target: '5' },
  { id: 'r4', type: 'EXECUTES', source: '2', target: '6' },
  { id: 'r5', type: 'HASHES', source: '1', target: '7' },
];
```

## Visual Style Requirements

- **NO light mode** - dark only
- **Sharp edges** on panels (no rounded corners on containers)
- **Glow effects** on selected nodes (box-shadow with accent color)
- **Grid background** on canvas (subtle, like graph paper)
- **Monospace font** for data values (JetBrains Mono or Fira Code)
- **Animated edges** - dashed lines should animate flow direction

## Component Structure

```
components/
├── graph-canvas.tsx      # D3 force simulation
├── node-renderer.tsx     # SVG shapes per label
├── edge-renderer.tsx     # Relationship lines
├── filter-panel.tsx      # Left sidebar
├── detail-panel.tsx      # Right sidebar  
├── query-builder.tsx     # Visual filter bar
└── shapes/
    ├── hexagon.tsx
    ├── octagon.tsx
    ├── nonagon.tsx
    └── diamond.tsx
```

## Make It Feel Like

- A tactical command system
- Neo4j Browser but cyberpunk
- Something from a sci-fi movie control room

Build the complete working app with all interactions functional.

---

## After v0 Generates

Once you have the code, we'll:
1. Add Supabase/SurrealDB adapters
2. Connect to sx9-atlas-bus for real-time updates
3. Add trivariate hash decoding
4. Integrate with NATS for live subscriptions



