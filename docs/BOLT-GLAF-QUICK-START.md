# GLAF Browser - Quick Start Prompt for Bolt

## Phase 1: Copy This First

```
Build a Neo4j-style graph database browser called "GLAF Browser" with:

Tech: React 18 + TypeScript + Vite + Tailwind CSS + D3.js + Supabase

DARK THEME ONLY - cyberpunk aesthetic:
- Background: #0a0a0f
- Panels: #12121a with #1a1a24 borders
- Primary: cyan #00ffff
- Secondary: magenta #ff00ff  
- Accent: amber #ffbf00
- Text: #e0e0e0
- Monospace font for data (JetBrains Mono)

Layout (3-column with command bar):
┌────────────────────────────────────────────────────┐
│ [Query Bar: MATCH (n:Agent) RETURN n] [▶ Run]      │
├────────┬──────────────────────────────┬────────────┤
│ LEFT   │      GRAPH CANVAS            │   RIGHT    │
│ RAIL   │   (D3 force simulation)      │   RAIL     │
│        │                              │            │
│ Labels │    ┌───┐      ┌───┐         │  DETAIL    │
│ • Agent│    │ A │──────│ S │         │  PANEL     │
│ • Slot │    └───┘      └───┘         │            │
│ • Tool │                              │  Props     │
│        │                              │  Rels      │
│ Types  │                              │  Actions   │
│ • ALLOC│                              │            │
├────────┴──────────────────────────────┴────────────┤
│ [Results Table/Stream]                              │
└────────────────────────────────────────────────────┘

Node shapes by label:
- Agent: hexagon, cyan
- Slot: octagon, magenta
- Tool: diamond, amber
- Hash: nonagon (9-sided), green

Interactions:
- Click node → show details in right panel
- Double-click node → expand relationships
- Drag node → move and pin
- Right-click → context menu

Start with hardcoded demo data, then connect Supabase.
```

---

## Phase 2: Add After Basic Layout Works

```
Add these features to the GLAF Browser:

1. Query Parser - translate Cypher-like syntax to PostgreSQL:
   "MATCH (n:Agent) RETURN n" → SELECT * FROM nodes WHERE label = 'Agent'
   "MATCH (a)-[r:ALLOCATED_TO]->(b) RETURN a,r,b" → JOIN query

2. Real-time updates via Supabase subscriptions:
   - New nodes fade in with glow
   - Updated nodes pulse
   - Deleted nodes fade out

3. Keyboard shortcuts:
   - Ctrl+Enter: run query
   - Escape: clear selection
   - F: fit to view
   - Delete: remove selected

4. Export buttons:
   - PNG screenshot
   - JSON graph data
   - CSV tables
```

---

## Phase 3: Advanced Features

```
Add advanced GLAF Browser features:

1. Multiple layout algorithms:
   - Force-directed (default)
   - Hierarchical (top-down)
   - Radial (circular)
   - Grid (matrix)

2. Relationship animations:
   - Dashed lines for DEPENDS_ON
   - Animated flow for ALLOCATED_TO
   - Pulsing for TRIGGERS

3. Search/filter panel:
   - Full-text search across properties
   - Filter by label checkboxes
   - Filter by relationship type
   - Date range filters

4. Query history with favorites:
   - Save queries with names
   - Recent queries list
   - One-click re-run

5. Property editor in detail panel:
   - Inline edit values
   - Add/remove properties
   - Type validation
```

---

## Database Schema (Provide to Bolt)

```sql
-- Core tables for GLAF Browser

CREATE TABLE nodes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  label TEXT NOT NULL,
  properties JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE relationships (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  type TEXT NOT NULL,
  source_node_id UUID REFERENCES nodes(id) ON DELETE CASCADE,
  target_node_id UUID REFERENCES nodes(id) ON DELETE CASCADE,
  properties JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE graph_queries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT,
  query TEXT NOT NULL,
  is_favorite BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed data
INSERT INTO nodes (label, properties) VALUES
  ('Agent', '{"name": "Alpha", "priority": 3, "state": "IDLE"}'),
  ('Agent', '{"name": "Beta", "priority": 2, "state": "EXECUTING"}'),
  ('Slot', '{"index": 0, "status": "READY"}'),
  ('Slot', '{"index": 1, "status": "BUSY"}'),
  ('Tool', '{"name": "nmap", "category": "recon"}');
```

---

## Key D3 Force Simulation Code Pattern

```typescript
// Give this to Bolt as a reference

import * as d3 from 'd3';

interface Node extends d3.SimulationNodeDatum {
  id: string;
  label: string;
  properties: Record<string, any>;
}

interface Link extends d3.SimulationLinkDatum<Node> {
  id: string;
  type: string;
}

const simulation = d3.forceSimulation<Node>(nodes)
  .force('link', d3.forceLink<Node, Link>(links)
    .id(d => d.id)
    .distance(100))
  .force('charge', d3.forceManyBody().strength(-300))
  .force('center', d3.forceCenter(width / 2, height / 2))
  .force('collision', d3.forceCollide().radius(40));

simulation.on('tick', () => {
  // Update node positions
  // Update link positions
});
```

---

## Color Palette (Copy Exactly)

```css
:root {
  --bg-primary: #0a0a0f;
  --bg-secondary: #12121a;
  --bg-tertiary: #1a1a24;
  --border: #2a2a3a;
  --text-primary: #e0e0e0;
  --text-secondary: #888888;
  --cyan: #00ffff;
  --magenta: #ff00ff;
  --amber: #ffbf00;
  --green: #00ff88;
  --red: #ff4444;
  --blue: #4488ff;
}
```

---

## Tips for Bolt

1. **Start simple** - Get the 3-column layout working first
2. **Hardcode data** - Don't connect Supabase until UI works
3. **D3 in useEffect** - Initialize simulation in useEffect with cleanup
4. **SVG for < 500 nodes** - Canvas/WebGL for larger graphs
5. **Zustand for state** - Don't use Redux, too complex
6. **No light mode** - Skip theme toggle entirely

---




