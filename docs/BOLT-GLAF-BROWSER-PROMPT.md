# GLAF Graph Browser - Bolt System Prompt

## Copy Everything Below This Line Into Bolt

---

# Build: SYNAPTIX9 GLAF Graph Browser

## Overview
Build a Neo4j-style graph database browser that EXCEEDS Neo4j Browser's capabilities. This is the GLAF (Graph-Lattice Allocation Framework) visualization frontend for a multi-database graph system.

## Tech Stack (REQUIRED)
- **Framework**: React 18 + TypeScript 5.5
- **Build**: Vite 5.4
- **Styling**: Tailwind CSS with custom dark theme (NO light mode)
- **Graph Engine**: D3.js force simulation (NOT force-graph library - we need full control)
- **Icons**: Lucide React
- **State**: Zustand for global state
- **Database**: PostgreSQL via Supabase client (@supabase/supabase-js)

## Design Philosophy
- Dark theme ONLY (like Neo4j Browser dark mode but better)
- Cyberpunk/tactical aesthetic with cyan (#00ffff), magenta (#ff00ff), amber (#ffbf00) accents
- Monospace fonts for data (JetBrains Mono or Fira Code)
- Sans-serif for UI (Inter or system)
- NO rounded corners on panels (sharp edges, tactical look)
- Subtle grid background pattern
- Glow effects on interactive elements

---

## LAYOUT ARCHITECTURE

```
┌────────────────────────────────────────────────────────────────────────────┐
│ COMMAND BAR (Query Input)                                          [▶ Run]│
│ ┌────────────────────────────────────────────────────────────────────────┐│
│ │ MATCH (n:Agent)-[r:ALLOCATED_TO]->(s:Slot) RETURN n, r, s              ││
│ └────────────────────────────────────────────────────────────────────────┘│
├────────┬───────────────────────────────────────────────────────┬──────────┤
│        │                                                       │          │
│  LEFT  │                  GRAPH CANVAS                         │  RIGHT   │
│  RAIL  │                                                       │  RAIL    │
│        │        ┌───┐                    ┌───┐                │          │
│ ┌────┐ │        │ A │════════════════════│ S │                │ ┌──────┐ │
│ │ 📊 │ │        └───┘                    └───┘                │ │DETAIL│ │
│ │ 🔍 │ │          ║                        ║                  │ │      │ │
│ │ ⚙️ │ │          ║      ┌───┐            ║                  │ │ Node │ │
│ │ 📁 │ │          ╚══════│ T │════════════╝                  │ │ Props│ │
│ └────┘ │                 └───┘                                │ │      │ │
│        │                                                       │ │ Rels │ │
│ Labels │                                                       │ │      │ │
│ ───────│                                                       │ └──────┘ │
│ Agent  │                                                       │          │
│ Slot   │                                                       │ Actions  │
│ Tool   │                                                       │ ──────── │
│ Hash   │                                                       │ [Expand] │
│        │                                                       │ [Hide]   │
│ Types  │                                                       │ [Edit]   │
│ ───────│                                                       │ [Delete] │
│ ALLOC  │                                                       │          │
│ EXEC   │                                                       │          │
│ CONV   │                                                       │          │
│        │                                                       │          │
├────────┴───────────────────────────────────────────────────────┴──────────┤
│ RESULTS STREAM / TABLE VIEW                                    [Graph|Table]│
│ ┌──────┬──────────┬──────────┬────────────┬─────────────────────────────┐│
│ │ ID   │ Label    │ Status   │ Properties │ Trivariate Hash              ││
│ ├──────┼──────────┼──────────┼────────────┼─────────────────────────────┤│
│ │ 42   │ Agent    │ EXEC     │ {pri: 3}   │ triv:7F3A...                 ││
│ └──────┴──────────┴──────────┴────────────┴─────────────────────────────┘│
└────────────────────────────────────────────────────────────────────────────┘
```

---

## COMPONENT SPECIFICATIONS

### 1. Command Bar (Query Editor)
```typescript
interface CommandBarProps {
  onExecute: (query: string) => void;
  history: string[];
  favorites: SavedQuery[];
}
```

Features:
- Syntax highlighting for Cypher-like queries
- Auto-complete for labels, relationship types, property keys
- Query history dropdown (last 50 queries)
- Favorites/saved queries panel
- Keyboard shortcuts: Ctrl+Enter to execute, Ctrl+/ to comment
- Multi-line support with line numbers
- Error highlighting with inline messages

### 2. Graph Canvas (D3 Force Simulation)
```typescript
interface GraphCanvasProps {
  nodes: GraphNode[];
  relationships: GraphRelationship[];
  onNodeClick: (node: GraphNode) => void;
  onNodeDoubleClick: (node: GraphNode) => void;
  onRelationshipClick: (rel: GraphRelationship) => void;
  onCanvasClick: () => void;
  selectedNodeId: string | null;
  layout: 'force' | 'hierarchical' | 'radial' | 'grid';
}

interface GraphNode {
  id: string;
  labels: string[];
  properties: Record<string, any>;
  x?: number;
  y?: number;
  fx?: number | null;  // Fixed position
  fy?: number | null;
  // Visual properties
  size: number;
  color: string;
  icon?: string;
  glow?: boolean;
  pulse?: boolean;
}

interface GraphRelationship {
  id: string;
  type: string;
  sourceId: string;
  targetId: string;
  properties: Record<string, any>;
  // Visual properties
  color: string;
  width: number;
  dashed?: boolean;
  animated?: boolean;
}
```

Node Shapes by Label (CRITICAL):
```typescript
const NODE_SHAPES: Record<string, NodeShape> = {
  // GLAF Entities
  'Agent': { shape: 'hexagon', color: '#00ffff', icon: 'Bot' },
  'Slot': { shape: 'octagon', color: '#ff00ff', icon: 'Grid3x3' },
  'Tool': { shape: 'diamond', color: '#ffbf00', icon: 'Wrench' },
  'TrivariteHash': { shape: 'nonagon', color: '#00ff88', icon: 'Hash' },
  
  // ATLAS Entities
  'AtlasNode': { shape: 'circle', color: '#4488ff', icon: 'Brain' },
  'IacManifold': { shape: 'square', color: '#ff4488', icon: 'Layers' },
  'Convergence': { shape: 'triangle', color: '#88ff44', icon: 'Target' },
  
  // Forge Entities
  'ClipboardEntry': { shape: 'rectangle', color: '#ff8844', icon: 'Clipboard' },
  'ThalmicAnnotation': { shape: 'pentagon', color: '#8844ff', icon: 'Filter' },
  'PromptScript': { shape: 'parallelogram', color: '#44ff88', icon: 'Code' },
  'ToolChain': { shape: 'trapezoid', color: '#ff4444', icon: 'Link' },
  
  // Infrastructure
  'Database': { shape: 'cylinder', color: '#888888', icon: 'Database' },
  'Service': { shape: 'rounded-rect', color: '#448888', icon: 'Server' },
  'CDN': { shape: 'cloud', color: '#884488', icon: 'Cloud' },
  
  // Default
  'default': { shape: 'circle', color: '#666666', icon: 'Circle' }
};
```

Relationship Styles by Type:
```typescript
const RELATIONSHIP_STYLES: Record<string, RelStyle> = {
  'ALLOCATED_TO': { color: '#00ffff', width: 2, animated: true },
  'EXECUTES': { color: '#ff00ff', width: 3, dashed: false },
  'CONVERGES_TO': { color: '#00ff88', width: 2, animated: true },
  'DEPENDS_ON': { color: '#ffbf00', width: 1, dashed: true },
  'TRIGGERS': { color: '#ff4444', width: 2, animated: true },
  'ROUTES_TO': { color: '#4488ff', width: 2, dashed: false },
  'HASHES': { color: '#88ff44', width: 1, dashed: true },
  'default': { color: '#444444', width: 1, dashed: false }
};
```

Canvas Interactions:
- **Single click node**: Select, show details in right panel
- **Double click node**: Expand relationships (fetch connected nodes)
- **Right click node**: Context menu (Expand All, Hide, Edit, Delete, Copy ID, Copy Trivariate)
- **Drag node**: Move and pin position
- **Double click canvas**: Unpin all nodes
- **Scroll**: Zoom in/out
- **Click + drag canvas**: Pan
- **Shift + click**: Multi-select nodes
- **Ctrl + click**: Add to selection

### 3. Left Rail (Navigation & Filters)
```typescript
interface LeftRailProps {
  labels: LabelCount[];
  relationshipTypes: TypeCount[];
  onLabelFilter: (labels: string[]) => void;
  onTypeFilter: (types: string[]) => void;
  activeFilters: FilterState;
}

interface LabelCount {
  label: string;
  count: number;
  color: string;
}
```

Sections:
1. **Quick Actions** (icon buttons)
   - New Node
   - New Relationship
   - Clear Canvas
   - Export PNG
   - Export JSON

2. **Labels** (collapsible)
   - Checkbox list with counts
   - Color indicator dots
   - Click to filter graph

3. **Relationship Types** (collapsible)
   - Checkbox list with counts
   - Line style preview
   - Click to filter

4. **Saved Queries** (collapsible)
   - List of favorites
   - Click to load into command bar

5. **History** (collapsible)
   - Recent queries
   - Timestamp + preview

### 4. Right Rail (Detail Panel)
```typescript
interface DetailPanelProps {
  selectedNode: GraphNode | null;
  selectedRelationship: GraphRelationship | null;
  onPropertyEdit: (key: string, value: any) => void;
  onNodeDelete: () => void;
  onExpandRelationships: () => void;
}
```

When Node Selected:
```
┌─────────────────────────────┐
│ 🔷 Agent                    │
│ ID: agent-42                │
├─────────────────────────────┤
│ PROPERTIES                  │
│ ┌─────────┬───────────────┐│
│ │ priority│ 3             ││
│ │ state   │ EXECUTING     ││
│ │ slot_id │ slot-7        ││
│ │ created │ 2024-12-04    ││
│ └─────────┴───────────────┘│
├─────────────────────────────┤
│ TRIVARIATE HASH             │
│ ┌─────────────────────────┐│
│ │ triv:7F3A2B...          ││
│ │ [Copy] [Decode]         ││
│ └─────────────────────────┘│
├─────────────────────────────┤
│ RELATIONSHIPS               │
│ ─→ ALLOCATED_TO (3)        │
│ ←─ EXECUTES (1)            │
│ ─→ CONVERGES_TO (2)        │
├─────────────────────────────┤
│ ACTIONS                     │
│ [Expand All] [Hide]         │
│ [Edit] [Delete]             │
└─────────────────────────────┘
```

### 5. Results Stream (Bottom Panel)
```typescript
interface ResultsStreamProps {
  results: QueryResult[];
  viewMode: 'graph' | 'table' | 'json' | 'ascii';
  onRowClick: (item: any) => void;
}
```

View Modes:
- **Graph**: Render results in canvas (default)
- **Table**: Spreadsheet-style with sortable columns
- **JSON**: Raw JSON with syntax highlighting
- **ASCII**: Text-based graph representation (like Neo4j's ASCII art)

---

## DATABASE SCHEMA (PostgreSQL via Supabase)

```sql
-- Already exists in Bolt database, connect to it:

-- Core Graph
CREATE TABLE nodes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  label TEXT NOT NULL,
  properties JSONB DEFAULT '{}',
  created_by TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE relationships (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  type TEXT NOT NULL,
  source_node_id UUID REFERENCES nodes(id) ON DELETE CASCADE,
  target_node_id UUID REFERENCES nodes(id) ON DELETE CASCADE,
  properties JSONB DEFAULT '{}',
  created_by TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- GLAF Specific
CREATE TABLE glaf_slots (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  slot_index INTEGER NOT NULL UNIQUE,
  status TEXT NOT NULL DEFAULT 'EMPTY',
  assigned_agent_id UUID,
  resource_requirements JSONB,
  convergence_state TEXT,
  last_tick_processed BIGINT,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE glaf_agents (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  agent_type TEXT NOT NULL,
  resource_profile JSONB,
  priority INTEGER DEFAULT 0,
  current_slot_id UUID,
  state TEXT DEFAULT 'IDLE',
  trivariate_hash TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ATLAS Monitoring
CREATE TABLE atlas_nodes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  entity_id TEXT UNIQUE,
  node_status TEXT,
  cognitive_tick_rate_us INTEGER,
  neural_mux_latency_ns INTEGER,
  ticks_processed BIGINT DEFAULT 0,
  l_star_learning_enabled BOOLEAN DEFAULT FALSE,
  voice_orchestration_enabled BOOLEAN DEFAULT FALSE,
  cuda_compute_slots INTEGER DEFAULT 0,
  iac_manifold_capabilities TEXT[]
);

-- Trivariate Hashes
CREATE TABLE trivariate_hashes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  sch_hash TEXT NOT NULL,
  cuid_hash TEXT NOT NULL,
  uuid_hash TEXT NOT NULL,
  canonical_form TEXT GENERATED ALWAYS AS (
    'triv:' || sch_hash || '_' || cuid_hash || '_' || uuid_hash
  ) STORED,
  operation_text TEXT,
  domain_mask INTEGER,
  execution_mask INTEGER,
  delta_angle_class TEXT,
  agent_id UUID,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Query History
CREATE TABLE graph_queries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT,
  query TEXT NOT NULL,
  is_favorite BOOLEAN DEFAULT FALSE,
  execution_count INTEGER DEFAULT 0,
  last_executed_at TIMESTAMPTZ,
  created_by TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## QUERY LANGUAGE (Cypher-like)

Implement a simple query parser that translates to PostgreSQL:

```typescript
// Example queries the system should support:

// Find all agents
"MATCH (n:Agent) RETURN n"
// → SELECT * FROM nodes WHERE label = 'Agent'

// Find agents allocated to slots
"MATCH (a:Agent)-[r:ALLOCATED_TO]->(s:Slot) RETURN a, r, s"
// → Complex JOIN query

// Find by property
"MATCH (n:Agent {state: 'EXECUTING'}) RETURN n"
// → SELECT * FROM nodes WHERE label = 'Agent' AND properties->>'state' = 'EXECUTING'

// Find with WHERE
"MATCH (n:Agent) WHERE n.priority > 2 RETURN n"
// → SELECT * FROM nodes WHERE label = 'Agent' AND (properties->>'priority')::int > 2

// Count by label
"MATCH (n) RETURN labels(n), count(*)"
// → SELECT label, COUNT(*) FROM nodes GROUP BY label

// Shortest path (bonus)
"MATCH path = shortestPath((a:Agent)-[*]-(s:Slot)) RETURN path"
// → Recursive CTE
```

---

## REAL-TIME FEATURES

### WebSocket Subscriptions (Supabase Realtime)
```typescript
// Subscribe to node changes
supabase
  .channel('graph-changes')
  .on('postgres_changes', { 
    event: '*', 
    schema: 'public', 
    table: 'nodes' 
  }, handleNodeChange)
  .on('postgres_changes', { 
    event: '*', 
    schema: 'public', 
    table: 'relationships' 
  }, handleRelChange)
  .subscribe();
```

Visual Feedback:
- New nodes: Fade in with glow animation
- Updated nodes: Pulse effect
- Deleted nodes: Fade out
- New relationships: Draw animation (line extends from source to target)

---

## KEYBOARD SHORTCUTS

| Shortcut | Action |
|----------|--------|
| Ctrl+Enter | Execute query |
| Ctrl+/ | Toggle comment |
| Ctrl+S | Save query to favorites |
| Ctrl+F | Focus search/filter |
| Escape | Clear selection |
| Delete | Delete selected (with confirm) |
| Ctrl+Z | Undo last action |
| Ctrl+Shift+Z | Redo |
| Ctrl+A | Select all visible nodes |
| Ctrl+E | Expand selected node |
| Ctrl+H | Hide selected nodes |
| F | Fit graph to view |
| R | Reset zoom |
| L | Toggle labels visibility |
| G | Toggle grid |

---

## EXPORT CAPABILITIES

1. **PNG Export**: High-res screenshot of canvas
2. **SVG Export**: Vector graphics for editing
3. **JSON Export**: Full graph data
4. **Cypher Export**: Generate CREATE statements
5. **CSV Export**: Nodes and relationships as separate CSVs

---

## PERFORMANCE REQUIREMENTS

- Handle 10,000+ nodes smoothly
- Use WebGL for large graphs (switch from SVG at threshold)
- Virtual scrolling in table view
- Debounced search/filter
- Lazy load node properties
- Canvas-based rendering for 1000+ nodes

---

## ENVIRONMENT VARIABLES

```env
VITE_SUPABASE_URL=your_supabase_url
VITE_SUPABASE_ANON_KEY=your_anon_key
VITE_APP_NAME=GLAF Browser
VITE_DEFAULT_LAYOUT=force
```

---

## FILE STRUCTURE

```
src/
├── components/
│   ├── CommandBar/
│   │   ├── CommandBar.tsx
│   │   ├── QueryEditor.tsx
│   │   ├── AutoComplete.tsx
│   │   └── QueryHistory.tsx
│   ├── GraphCanvas/
│   │   ├── GraphCanvas.tsx
│   │   ├── NodeRenderer.tsx
│   │   ├── RelationshipRenderer.tsx
│   │   ├── ForceSimulation.ts
│   │   └── shapes/
│   │       ├── Hexagon.tsx
│   │       ├── Octagon.tsx
│   │       ├── Nonagon.tsx
│   │       └── ... (all shapes)
│   ├── LeftRail/
│   │   ├── LeftRail.tsx
│   │   ├── LabelFilter.tsx
│   │   ├── TypeFilter.tsx
│   │   └── SavedQueries.tsx
│   ├── RightRail/
│   │   ├── RightRail.tsx
│   │   ├── NodeDetail.tsx
│   │   ├── RelationshipDetail.tsx
│   │   └── PropertyEditor.tsx
│   ├── ResultsStream/
│   │   ├── ResultsStream.tsx
│   │   ├── TableView.tsx
│   │   ├── JsonView.tsx
│   │   └── AsciiView.tsx
│   └── ui/
│       ├── Button.tsx
│       ├── Input.tsx
│       ├── Panel.tsx
│       └── ... (shadcn/ui components)
├── hooks/
│   ├── useGraph.ts
│   ├── useQuery.ts
│   ├── useRealtime.ts
│   └── useKeyboardShortcuts.ts
├── lib/
│   ├── supabase.ts
│   ├── queryParser.ts
│   ├── graphUtils.ts
│   └── exportUtils.ts
├── stores/
│   └── graphStore.ts (Zustand)
├── types/
│   └── graph.ts
├── App.tsx
└── main.tsx
```

---

## INITIAL SEED DATA

Create these on first load if database is empty:

```typescript
const SEED_DATA = {
  nodes: [
    { label: 'Agent', properties: { name: 'Agent-Alpha', priority: 3, state: 'IDLE' } },
    { label: 'Agent', properties: { name: 'Agent-Beta', priority: 2, state: 'EXECUTING' } },
    { label: 'Slot', properties: { slot_index: 0, status: 'READY' } },
    { label: 'Slot', properties: { slot_index: 1, status: 'EXECUTING' } },
    { label: 'Tool', properties: { name: 'nmap', category: 'recon' } },
    { label: 'Tool', properties: { name: 'nuclei', category: 'vuln' } },
    { label: 'TrivariteHash', properties: { canonical: 'triv:7F3A2B_CUID123_UUID456' } },
  ],
  relationships: [
    { type: 'ALLOCATED_TO', source: 0, target: 2 },
    { type: 'ALLOCATED_TO', source: 1, target: 3 },
    { type: 'EXECUTES', source: 0, target: 4 },
    { type: 'EXECUTES', source: 1, target: 5 },
    { type: 'HASHES', source: 0, target: 6 },
  ]
};
```

---

## START BUILDING

Begin with:
1. Set up Vite + React + TypeScript + Tailwind
2. Create the main layout shell (command bar, 3-column layout, results)
3. Implement the D3 force simulation canvas with basic nodes
4. Add node shapes and relationship rendering
5. Connect to Supabase and fetch real data
6. Implement the query parser
7. Add real-time subscriptions
8. Polish interactions and animations

Make it BETTER than Neo4j Browser. Make it feel like a tactical command system.

---




