# SYNAPTIX9 Forge Workbench - Complete System Prompt for Bolt

## Overview

Build a **unified data workbench** that combines:
- **Forge Canvas** - n8n-style visual workflow builder
- **Graph Browser** - Neo4j-style graph visualization
- **Database Studio** - Supabase-style table views for ALL databases
- **Model Viewer** - GNN/ANN architecture visualization

This is the command center for SYNAPTIX9 data operations.

---

## Tech Stack

```
Framework:     React 18 + TypeScript 5.5
Build:         Vite 5.4
Styling:       Tailwind CSS (dark theme ONLY)
Graph:         D3.js force simulation
Workflow:      React Flow (for n8n-style canvas)
Tables:        TanStack Table v8
Icons:         Lucide React
State:         Zustand
Databases:     
  - Supabase (PostgreSQL)
  - SurrealDB (graph + document)
  - Sled (embedded KV)
  - Redis/Dragonfly (cache)
```

---

## Design System

```css
/* Cyberpunk/Tactical Dark Theme */
:root {
  --bg-void: #050508;
  --bg-primary: #0a0a0f;
  --bg-secondary: #12121a;
  --bg-tertiary: #1a1a24;
  --bg-elevated: #22222e;
  
  --border-subtle: #1a1a24;
  --border-default: #2a2a3a;
  --border-bright: #3a3a4a;
  
  --text-primary: #e8e8ec;
  --text-secondary: #888898;
  --text-muted: #555566;
  
  /* Accent Colors */
  --cyan: #00ffff;
  --magenta: #ff00ff;
  --amber: #ffbf00;
  --lime: #00ff88;
  --coral: #ff6b6b;
  --violet: #8b5cf6;
  --sky: #38bdf8;
  
  /* Database Brand Colors */
  --supabase: #3ecf8e;
  --surrealdb: #ff00a0;
  --sled: #ff6b35;
  --redis: #dc382d;
  --neo4j: #008cc1;
  --n8n: #ea4b71;
  
  /* Glow Effects */
  --glow-cyan: 0 0 20px rgba(0, 255, 255, 0.3);
  --glow-magenta: 0 0 20px rgba(255, 0, 255, 0.3);
}

/* Fonts */
--font-mono: 'JetBrains Mono', 'Fira Code', monospace;
--font-sans: 'Inter', system-ui, sans-serif;
```

---

## MASTER LAYOUT

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ ┌─────┐  SYNAPTIX9 Forge Workbench                    [🔍] [⚙️] [👤]       │
│ │ ≡   │  ────────────────────────────────────────────────────────────────  │
│ └─────┘  [Forge] [Graph] [Supabase] [Surreal] [Sled] [Models] [Vectors]    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                        << ACTIVE WORKSPACE VIEW >>                           │
│                                                                              │
│    (Content changes based on selected tab - see sections below)              │
│                                                                              │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│ [Status Bar: Connected ● Supabase ● SurrealDB ● Sled | 3 workflows active]  │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## TAB 1: FORGE CANVAS (n8n-Style Workflow Builder)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ FORGE CANVAS                                          [▶ Run] [💾] [📤]    │
├─────────┬───────────────────────────────────────────────────────┬───────────┤
│ TOOLBOX │                                                       │ INSPECTOR │
│         │              WORKFLOW CANVAS                          │           │
│ ┌─────┐ │                                                       │ ┌───────┐ │
│ │Trigg│ │    ┌─────────┐      ┌─────────┐      ┌─────────┐    │ │ Node  │ │
│ │gers │ │    │ Webhook │─────▶│ Filter  │─────▶│ Supabase│    │ │ Props │ │
│ └─────┘ │    │ Trigger │      │ Thalmic │      │ Insert  │    │ │       │ │
│ ┌─────┐ │    └─────────┘      └─────────┘      └─────────┘    │ │ name: │ │
│ │Data │ │         │                                  │         │ │ type: │ │
│ │bases│ │         │           ┌─────────┐           │         │ │ config│ │
│ └─────┘ │         └──────────▶│ Trivar  │◀──────────┘         │ └───────┘ │
│ ┌─────┐ │                     │ Hash    │                      │           │
│ │Trans│ │                     └─────────┘                      │ ┌───────┐ │
│ │forms│ │                          │                           │ │ I/O   │ │
│ └─────┘ │                          ▼                           │ │ Schema│ │
│ ┌─────┐ │                     ┌─────────┐                      │ │       │ │
│ │Tools│ │                     │ NATS    │                      │ │ in:   │ │
│ │     │ │                     │ Publish │                      │ │ out:  │ │
│ └─────┘ │                     └─────────┘                      │ └───────┘ │
│ ┌─────┐ │                                                       │           │
│ │AI/ML│ │  [Grid: ···] [Snap: ON] [Zoom: 100%]                │ [Test]    │
│ └─────┘ │                                                       │ [Delete]  │
└─────────┴───────────────────────────────────────────────────────┴───────────┘
```

### Forge Node Categories

```typescript
const FORGE_NODE_CATEGORIES = {
  triggers: [
    { id: 'webhook', name: 'Webhook', icon: 'Webhook', color: '#00ffff' },
    { id: 'schedule', name: 'Schedule', icon: 'Clock', color: '#00ffff' },
    { id: 'nats-sub', name: 'NATS Subscribe', icon: 'Radio', color: '#00ffff' },
    { id: 'file-watch', name: 'File Watch', icon: 'Eye', color: '#00ffff' },
    { id: 'db-trigger', name: 'DB Trigger', icon: 'Database', color: '#00ffff' },
  ],
  
  databases: [
    { id: 'supabase', name: 'Supabase', icon: 'Database', color: '#3ecf8e' },
    { id: 'surrealdb', name: 'SurrealDB', icon: 'Share2', color: '#ff00a0' },
    { id: 'sled', name: 'Sled KV', icon: 'HardDrive', color: '#ff6b35' },
    { id: 'redis', name: 'Redis', icon: 'Zap', color: '#dc382d' },
    { id: 'vector-db', name: 'Vector DB', icon: 'Boxes', color: '#8b5cf6' },
  ],
  
  transforms: [
    { id: 'thalmic', name: 'Thalmic Filter', icon: 'Filter', color: '#ff00ff' },
    { id: 'trivariate', name: 'Trivariate Hash', icon: 'Hash', color: '#00ff88' },
    { id: 'json-transform', name: 'JSON Transform', icon: 'FileJson', color: '#ffbf00' },
    { id: 'code', name: 'Code Block', icon: 'Code', color: '#ffbf00' },
    { id: 'split', name: 'Split', icon: 'GitBranch', color: '#ffbf00' },
    { id: 'merge', name: 'Merge', icon: 'GitMerge', color: '#ffbf00' },
    { id: 'aggregate', name: 'Aggregate', icon: 'Layers', color: '#ffbf00' },
  ],
  
  tools: [
    { id: 'nmap', name: 'Nmap', icon: 'Radar', color: '#ff6b6b' },
    { id: 'nuclei', name: 'Nuclei', icon: 'Bug', color: '#ff6b6b' },
    { id: 'masscan', name: 'Masscan', icon: 'Scan', color: '#ff6b6b' },
    { id: 'http', name: 'HTTP Request', icon: 'Globe', color: '#38bdf8' },
    { id: 'ssh', name: 'SSH', icon: 'Terminal', color: '#38bdf8' },
    { id: 'shell', name: 'Shell Command', icon: 'TerminalSquare', color: '#38bdf8' },
  ],
  
  ai_ml: [
    { id: 'llm-prompt', name: 'LLM Prompt', icon: 'Brain', color: '#8b5cf6' },
    { id: 'embedding', name: 'Embedding', icon: 'Sparkles', color: '#8b5cf6' },
    { id: 'classifier', name: 'Classifier', icon: 'Tags', color: '#8b5cf6' },
    { id: 'ann-inference', name: 'ANN Inference', icon: 'Network', color: '#8b5cf6' },
    { id: 'gnn-inference', name: 'GNN Inference', icon: 'Share2', color: '#8b5cf6' },
  ],
  
  outputs: [
    { id: 'nats-pub', name: 'NATS Publish', icon: 'Send', color: '#00ff88' },
    { id: 'webhook-out', name: 'Webhook Out', icon: 'ExternalLink', color: '#00ff88' },
    { id: 'email', name: 'Email', icon: 'Mail', color: '#00ff88' },
    { id: 'slack', name: 'Slack', icon: 'MessageSquare', color: '#00ff88' },
    { id: 'file-write', name: 'File Write', icon: 'FileOutput', color: '#00ff88' },
  ],
  
  control: [
    { id: 'if', name: 'If/Else', icon: 'GitBranch', color: '#888898' },
    { id: 'switch', name: 'Switch', icon: 'Route', color: '#888898' },
    { id: 'loop', name: 'Loop', icon: 'Repeat', color: '#888898' },
    { id: 'wait', name: 'Wait', icon: 'Timer', color: '#888898' },
    { id: 'error', name: 'Error Handler', icon: 'AlertTriangle', color: '#888898' },
  ],
};
```

### Forge Canvas Implementation (React Flow)

```typescript
import ReactFlow, {
  Node,
  Edge,
  Controls,
  Background,
  MiniMap,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
} from 'reactflow';

interface ForgeNode extends Node {
  data: {
    category: string;
    nodeType: string;
    config: Record<string, any>;
    inputs: PortDefinition[];
    outputs: PortDefinition[];
  };
}

// Custom node component with proper styling
const ForgeNodeComponent = ({ data, selected }) => (
  <div className={`
    forge-node 
    bg-bg-secondary border border-border-default rounded-lg
    ${selected ? 'ring-2 ring-cyan shadow-glow-cyan' : ''}
  `}>
    <div className="node-header flex items-center gap-2 px-3 py-2 border-b border-border-subtle">
      <Icon name={data.icon} className="w-4 h-4" style={{ color: data.color }} />
      <span className="text-sm font-medium">{data.label}</span>
    </div>
    <div className="node-body p-3">
      {/* Input/output handles */}
    </div>
  </div>
);
```

### n8n Import/Export

```typescript
// Support importing actual n8n workflow JSON
interface N8NWorkflow {
  name: string;
  nodes: N8NNode[];
  connections: N8NConnections;
  settings: N8NSettings;
}

// Convert n8n format to Forge format
function importN8NWorkflow(n8n: N8NWorkflow): ForgeWorkflow {
  // Map n8n node types to Forge equivalents
  const nodeMapping: Record<string, string> = {
    'n8n-nodes-base.webhook': 'webhook',
    'n8n-nodes-base.httpRequest': 'http',
    'n8n-nodes-base.postgres': 'supabase',
    'n8n-nodes-base.if': 'if',
    // ... more mappings
  };
  // ... conversion logic
}

// Export Forge workflow as n8n-compatible JSON
function exportToN8N(forge: ForgeWorkflow): N8NWorkflow {
  // ... conversion logic
}
```

---

## TAB 2: GRAPH BROWSER (Neo4j-Style)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ GRAPH BROWSER                                    [Layout ▼] [Export ▼]      │
├─────────────────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ MATCH (n:Agent)-[r:ALLOCATED_TO]->(s:Slot) RETURN n, r, s    [▶ Run]   │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
├─────────┬───────────────────────────────────────────────────────┬───────────┤
│ FILTERS │                 GRAPH CANVAS                          │  DETAILS  │
│         │                                                       │           │
│ Labels  │        ┌───┐                    ┌───┐                │ ┌───────┐ │
│ ☑ Agent │        │ A │════════════════════│ S │                │ │ Agent │ │
│ ☑ Slot  │        └───┘                    └───┘                │ │ ───── │ │
│ ☐ Tool  │          ║                        ║                  │ │ name: │ │
│ ☐ Hash  │          ║      ┌───┐            ║                  │ │ Alpha │ │
│         │          ╚══════│ T │════════════╝                  │ │       │ │
│ Types   │                 └───┘                                │ │ state:│ │
│ ☑ ALLOC │                                                       │ │ IDLE  │ │
│ ☐ EXEC  │                                                       │ │       │ │
│ ☐ CONV  │                                                       │ │ Rels: │ │
│         │                                                       │ │ → 3   │ │
│ Source  │                                                       │ │ ← 1   │ │
│ ○ All   │  [Force] [Hierarchy] [Radial] [Grid]                │ └───────┘ │
│ ● Supa  │                                                       │           │
│ ○ Surr  │                                                       │ [Expand]  │
│ ○ Sled  │                                                       │ [Hide]    │
└─────────┴───────────────────────────────────────────────────────┴───────────┘
```

### Node Shapes (SVG Definitions)

```typescript
const NODE_SHAPES = {
  // GLAF Entities
  Agent: { shape: 'hexagon', color: '#00ffff', icon: 'Bot' },
  Slot: { shape: 'octagon', color: '#ff00ff', icon: 'Grid3x3' },
  Tool: { shape: 'diamond', color: '#ffbf00', icon: 'Wrench' },
  TrivariteHash: { shape: 'nonagon', color: '#00ff88', icon: 'Hash' },
  
  // Infrastructure
  GroundStation: { shape: 'hexagon', color: '#38bdf8', icon: 'Radio' },
  Satellite: { shape: 'circle', color: '#8b5cf6', icon: 'Satellite' },
  Beam: { shape: 'triangle', color: '#ffbf00', icon: 'Zap' },
  
  // ML Models
  ANN: { shape: 'rectangle', color: '#ff6b6b', icon: 'Network' },
  GNN: { shape: 'hexagon', color: '#3ecf8e', icon: 'Share2' },
  Layer: { shape: 'rounded-rect', color: '#888898', icon: 'Layers' },
  
  // Workflows
  Workflow: { shape: 'parallelogram', color: '#ea4b71', icon: 'Workflow' },
  Step: { shape: 'rectangle', color: '#888898', icon: 'Square' },
  
  // Default
  default: { shape: 'circle', color: '#555566', icon: 'Circle' },
};

// SVG path generators for each shape
const shapePaths = {
  hexagon: (size: number) => {
    const a = size / 2;
    const b = a * Math.sqrt(3) / 2;
    return `M ${a} 0 L ${a*2} ${b} L ${a*2} ${b*2} L ${a} ${b*3} L 0 ${b*2} L 0 ${b} Z`;
  },
  octagon: (size: number) => {
    const s = size / (1 + Math.SQRT2);
    return `M ${s} 0 L ${size-s} 0 L ${size} ${s} L ${size} ${size-s} 
            L ${size-s} ${size} L ${s} ${size} L 0 ${size-s} L 0 ${s} Z`;
  },
  nonagon: (size: number) => {
    // 9-sided polygon for SYNAPTIX9 branding
    const points = [];
    for (let i = 0; i < 9; i++) {
      const angle = (i * 2 * Math.PI / 9) - Math.PI / 2;
      points.push(`${size/2 + size/2 * Math.cos(angle)},${size/2 + size/2 * Math.sin(angle)}`);
    }
    return `M ${points.join(' L ')} Z`;
  },
  diamond: (size: number) => `M ${size/2} 0 L ${size} ${size/2} L ${size/2} ${size} L 0 ${size/2} Z`,
  // ... more shapes
};
```

### Query Language Support

```typescript
// Support multiple query languages
type QueryLanguage = 'cypher' | 'surql' | 'sql' | 'graphql';

interface QueryParser {
  parse(query: string): ParsedQuery;
  toSQL(parsed: ParsedQuery): string;
  toSurQL(parsed: ParsedQuery): string;
}

// Example queries by language
const QUERY_EXAMPLES = {
  cypher: [
    'MATCH (n:Agent) RETURN n',
    'MATCH (a:Agent)-[r:ALLOCATED_TO]->(s:Slot) WHERE s.status = "READY" RETURN a, r, s',
    'MATCH path = shortestPath((a:GroundStation)-[*]-(s:Satellite)) RETURN path',
  ],
  surql: [
    'SELECT * FROM agent',
    'SELECT * FROM agent WHERE state = "EXECUTING"',
    'SELECT ->allocated_to->slot FROM agent:alpha',
  ],
  sql: [
    'SELECT * FROM nodes WHERE label = \'Agent\'',
    'SELECT n.*, r.*, s.* FROM nodes n JOIN relationships r ON n.id = r.source_node_id JOIN nodes s ON r.target_node_id = s.id',
  ],
};
```

---

## TAB 3-6: DATABASE STUDIO VIEWS

### Unified Database Navigator

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ DATABASE: SUPABASE                              [+ New Table] [SQL Editor]  │
├─────────────────────────────────────────────────────────────────────────────┤
│ ┌─────────┬─────────────────────────────────────────────────────────────────┤
│ │ TABLES  │  TABLE: ground_stations                    [Filter] [+ Row]    │
│ │         │ ┌────────┬──────────────┬──────────┬──────────┬──────────────┐ │
│ │ 📋 nodes│ │ id     │ name         │ latitude │ longitude│ status       │ │
│ │ 📋 rels │ ├────────┼──────────────┼──────────┼──────────┼──────────────┤ │
│ │ 📋 ground│ │ gs-001 │ Phoenix GS   │ 33.4484  │ -112.074 │ ACTIVE       │ │
│ │ 📋 sats │ │ gs-002 │ Miami GS     │ 25.7617  │ -80.1918 │ ACTIVE       │ │
│ │ 📋 beams│ │ gs-003 │ Seattle GS   │ 47.6062  │ -122.332 │ MAINTENANCE  │ │
│ │         │ │ gs-004 │ Denver GS    │ 39.7392  │ -104.990 │ ACTIVE       │ │
│ │ VIEWS   │ │ ...    │ ...          │ ...      │ ...      │ ...          │ │
│ │ 👁 active│ └────────┴──────────────┴──────────┴──────────┴──────────────┘ │
│ │ 👁 stats│                                                                  │
│ │         │  Showing 1-50 of 257 rows                    [< Prev] [Next >]  │
│ │ FUNCS   │ ──────────────────────────────────────────────────────────────  │
│ │ ƒ geo   │                                                                  │
│ │ ƒ hash  │  ROW DETAIL (gs-001)                                   [Edit]  │
│ │         │  ┌─────────────────────────────────────────────────────────┐   │
│ │         │  │ id: gs-001                                              │   │
│ │         │  │ name: Phoenix GS                                        │   │
│ │         │  │ latitude: 33.4484                                       │   │
│ │         │  │ longitude: -112.0740                                    │   │
│ │         │  │ tier: 1                                                 │   │
│ │         │  │ capacity_gbps: 100                                      │   │
│ │         │  │ weather_score: 0.92                                     │   │
│ │         │  │ last_contact: 2024-12-04T10:23:45Z                      │   │
│ │         │  └─────────────────────────────────────────────────────────┘   │
│ └─────────┴─────────────────────────────────────────────────────────────────┘
└─────────────────────────────────────────────────────────────────────────────┘
```

### Database-Specific Features

```typescript
interface DatabaseAdapter {
  id: string;
  name: string;
  icon: string;
  color: string;
  
  // Connection
  connect(config: ConnectionConfig): Promise<void>;
  disconnect(): Promise<void>;
  testConnection(): Promise<boolean>;
  
  // Schema
  getTables(): Promise<TableInfo[]>;
  getTableSchema(table: string): Promise<ColumnInfo[]>;
  getRelationships(): Promise<ForeignKey[]>;
  
  // CRUD
  query(sql: string): Promise<QueryResult>;
  insert(table: string, data: Record<string, any>): Promise<any>;
  update(table: string, id: string, data: Record<string, any>): Promise<any>;
  delete(table: string, id: string): Promise<void>;
  
  // Native features
  getNativeUI?(): React.ComponentType;
  getGraphData?(): Promise<GraphData>;
}

// Implementations
const DATABASE_ADAPTERS: Record<string, DatabaseAdapter> = {
  supabase: {
    id: 'supabase',
    name: 'Supabase',
    icon: 'Database',
    color: '#3ecf8e',
    // Uses @supabase/supabase-js
    // Real-time subscriptions
    // Row-level security
    // Storage integration
  },
  
  surrealdb: {
    id: 'surrealdb',
    name: 'SurrealDB',
    icon: 'Share2',
    color: '#ff00a0',
    // Uses surrealdb.js
    // Native graph queries
    // Record links
    // Live queries
  },
  
  sled: {
    id: 'sled',
    name: 'Sled KV',
    icon: 'HardDrive',
    color: '#ff6b35',
    // HTTP API to Rust backend
    // Key-value operations
    // Atomic transactions
    // Prefix scans
  },
  
  redis: {
    id: 'redis',
    name: 'Redis/Dragonfly',
    icon: 'Zap',
    color: '#dc382d',
    // Uses ioredis
    // Pub/sub
    // Streams
    // Graph module (RedisGraph)
  },
};
```

### SurrealDB Native Graph View

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ DATABASE: SURREALDB                                    [SurrealQL Editor]   │
├─────────────────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ SELECT * FROM ground_station, satellite, ->gs_to_sat_link              │ │
│ │ WHERE ground_station.status = 'ACTIVE'                    [▶ Execute]  │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
├─────────┬───────────────────────────────────────────────────────────────────┤
│ SCHEMA  │  RESULT: Graph View                              [Table] [JSON]  │
│         │                                                                   │
│ 📊 Tables│        ┌──────────┐                                             │
│ ground_ │        │ Phoenix  │                                             │
│ satellite│        │ GS       │═══════════╗                                 │
│ gs_link │        └──────────┘           ║                                 │
│ sat_link│             ║                  ║                                 │
│         │             ║            ┌─────╨─────┐                           │
│ 🔗 Edges │             ║            │ Satellite │                           │
│ gs_to_sat│             ║            │ Alpha     │                           │
│ sat_to_sat            ║            └───────────┘                           │
│         │        ┌────╨─────┐           ║                                 │
│ ƒ Funcs │        │ Miami    │═══════════╝                                 │
│ geo::dist│        │ GS       │                                             │
│ math::   │        └──────────┘                                             │
│         │                                                                   │
│         │  [Auto-layout] [Expand All] [Collapse]                           │
└─────────┴───────────────────────────────────────────────────────────────────┘
```

---

## TAB 7: MODEL VIEWER (GNN/ANN Architecture)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ MODEL VIEWER                                    [Load Model] [Export ONNX]  │
├─────────────────────────────────────────────────────────────────────────────┤
│ MODEL: weather_prediction_gnn                              [Train] [Infer]  │
├─────────┬───────────────────────────────────────────────────────┬───────────┤
│ LAYERS  │              ARCHITECTURE VIEW                        │ LAYER     │
│         │                                                       │ DETAIL    │
│ 📥 Input│    ┌─────────────────────────────────────────┐       │           │
│   257   │    │           INPUT LAYER (257)             │       │ GraphConv │
│         │    │  ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○  │       │ ───────── │
│ 🔄 GConv│    └─────────────────────────────────────────┘       │ in: 257   │
│   128   │                      │                                │ out: 128  │
│         │                      ▼                                │ aggr: mean│
│ 🔄 GConv│    ┌─────────────────────────────────────────┐       │ act: ReLU │
│   64    │    │         GRAPH CONV (128)                │       │           │
│         │    │     ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○            │       │ params:   │
│ 🧮 Dense│    └─────────────────────────────────────────┘       │ 33,024    │
│   32    │                      │                                │           │
│         │                      ▼                                │ gradients:│
│ 📤 Output   ┌─────────────────────────────────────────┐       │ ████░░░░  │
│   12    │    │         GRAPH CONV (64)                 │       │           │
│         │    │        ○ ○ ○ ○ ○ ○ ○ ○                  │       │           │
│ METRICS │    └─────────────────────────────────────────┘       │           │
│ ─────── │                      │                                │           │
│ Loss:   │                      ▼                                │           │
│ 0.0234  │    ┌─────────────────────────────────────────┐       │           │
│         │    │           DENSE (32)                    │       │           │
│ Acc:    │    │            ○ ○ ○ ○ ○                    │       │           │
│ 94.2%   │    └─────────────────────────────────────────┘       │           │
│         │                      │                                │           │
│ Epoch:  │                      ▼                                │           │
│ 45/100  │    ┌─────────────────────────────────────────┐       │           │
│         │    │           OUTPUT (12)                   │       │           │
│         │    │              ○ ○ ○                      │       │           │
│         │    └─────────────────────────────────────────┘       │           │
└─────────┴───────────────────────────────────────────────────────┴───────────┘
```

### Model Definition Schema

```typescript
interface NeuralModel {
  id: string;
  name: string;
  type: 'ann' | 'gnn' | 'cnn' | 'transformer';
  layers: Layer[];
  optimizer: OptimizerConfig;
  loss: string;
  metrics: string[];
}

interface Layer {
  id: string;
  type: string;
  params: Record<string, any>;
  inputShape: number[];
  outputShape: number[];
}

// GNN-specific for ground station network
interface GNNModel extends NeuralModel {
  type: 'gnn';
  nodeFeatures: string[];  // e.g., ['latitude', 'longitude', 'capacity', 'weather_score']
  edgeFeatures: string[];  // e.g., ['distance', 'bandwidth', 'latency']
  aggregation: 'mean' | 'sum' | 'max';
}
```

---

## TAB 8: VECTOR DATABASE VIEW

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ VECTOR DB                                         [+ Collection] [Search]   │
├─────────────────────────────────────────────────────────────────────────────┤
│ ┌─────────┬─────────────────────────────────────────────────────────────────┤
│ │COLLECT- │  COLLECTION: threat_embeddings              [Dim: 1536]        │
│ │IONS     │ ┌───────────────────────────────────────────────────────────┐  │
│ │         │ │ 🔍 Search: "lateral movement techniques"        [Search]  │  │
│ │ 📦 threat│ └───────────────────────────────────────────────────────────┘  │
│ │ 📦 tools │                                                                │
│ │ 📦 docs  │  RESULTS (similarity > 0.85)                                  │
│ │ 📦 code  │ ┌────────┬─────────────────────────────────┬────────────────┐ │
│ │         │ │ Score  │ Content                         │ Metadata       │ │
│ │ STATS   │ ├────────┼─────────────────────────────────┼────────────────┤ │
│ │ ─────── │ │ 0.94   │ Pass-the-Hash attack vector... │ MITRE: T1550   │ │
│ │ Vectors:│ │ 0.91   │ Kerberoasting technique for... │ MITRE: T1558   │ │
│ │ 45,230  │ │ 0.89   │ SMB relay attacks enable...    │ MITRE: T1557   │ │
│ │         │ │ 0.87   │ WMI lateral movement using...  │ MITRE: T1047   │ │
│ │ Dim:    │ └────────┴─────────────────────────────────┴────────────────┘ │
│ │ 1536    │                                                                │
│ │         │  EMBEDDING VISUALIZATION (t-SNE)                              │
│ │ Index:  │ ┌───────────────────────────────────────────────────────────┐ │
│ │ HNSW    │ │    ·  · ·                    · ··                         │ │
│ │         │ │  ·· · ·  ·                 ·  · ·  ·                      │ │
│ │         │ │   · ·· ·                    ·· ·                          │ │
│ │         │ │              ★ query                                      │ │
│ │         │ │        · ·                      · · ·                     │ │
│ │         │ │       ·  ·  ·                  · ·                        │ │
│ │         │ └───────────────────────────────────────────────────────────┘ │
└─────────┴───────────────────────────────────────────────────────────────────┘
```

---

## GLOBAL FEATURES

### Connection Manager

```typescript
interface ConnectionConfig {
  id: string;
  name: string;
  type: 'supabase' | 'surrealdb' | 'sled' | 'redis' | 'postgres' | 'vector';
  host: string;
  port: number;
  database?: string;
  credentials: {
    username?: string;
    password?: string;
    apiKey?: string;
    token?: string;
  };
  ssl: boolean;
  poolSize?: number;
}

// Store connections in localStorage/IndexedDB
const connectionStore = {
  connections: ConnectionConfig[];
  activeConnections: Map<string, DatabaseAdapter>;
};
```

### Universal Search (Cmd+K)

```
┌─────────────────────────────────────────────────────────────────┐
│ 🔍 Search everything...                                         │
├─────────────────────────────────────────────────────────────────┤
│ RECENT                                                          │
│   📋 ground_stations table                                      │
│   🔄 weather_prediction workflow                                │
│   📊 Agent-Alpha node                                           │
│                                                                 │
│ TABLES                                                          │
│   📋 ground_stations (Supabase)                                 │
│   📋 satellites (Supabase)                                      │
│   📋 ground_station (SurrealDB)                                 │
│                                                                 │
│ WORKFLOWS                                                       │
│   🔄 Threat Detection Pipeline                                  │
│   🔄 Weather Analysis                                           │
│                                                                 │
│ NODES                                                           │
│   📊 Agent-Alpha                                                │
│   📊 Slot-7                                                     │
└─────────────────────────────────────────────────────────────────┘
```

### Status Bar

```typescript
interface StatusBarProps {
  connections: {
    database: string;
    status: 'connected' | 'disconnected' | 'error';
    latency?: number;
  }[];
  activeWorkflows: number;
  lastSync: Date;
  user?: string;
}

// Status bar component
<StatusBar>
  <ConnectionStatus db="Supabase" status="connected" latency={23} />
  <ConnectionStatus db="SurrealDB" status="connected" latency={45} />
  <ConnectionStatus db="Sled" status="disconnected" />
  <Separator />
  <span>3 workflows active</span>
  <Separator />
  <span>Last sync: 2 min ago</span>
</StatusBar>
```

---

## KEYBOARD SHORTCUTS

| Shortcut | Action |
|----------|--------|
| Cmd+K | Universal search |
| Cmd+1-8 | Switch tabs |
| Cmd+Enter | Execute query/run workflow |
| Cmd+S | Save current item |
| Cmd+N | New (context-aware) |
| Cmd+D | Duplicate selected |
| Cmd+Backspace | Delete selected |
| Cmd+Z / Cmd+Shift+Z | Undo/Redo |
| Cmd+/ | Toggle comment (in editors) |
| Escape | Close modal/deselect |
| F | Fit to view (graph/canvas) |
| Space | Play/pause (workflow) |

---

## FILE STRUCTURE

```
src/
├── components/
│   ├── Layout/
│   │   ├── AppShell.tsx
│   │   ├── TabBar.tsx
│   │   ├── StatusBar.tsx
│   │   └── CommandPalette.tsx
│   ├── Forge/
│   │   ├── ForgeCanvas.tsx
│   │   ├── NodeToolbox.tsx
│   │   ├── NodeInspector.tsx
│   │   ├── nodes/
│   │   │   ├── TriggerNode.tsx
│   │   │   ├── DatabaseNode.tsx
│   │   │   ├── TransformNode.tsx
│   │   │   └── ...
│   │   └── N8NImporter.tsx
│   ├── Graph/
│   │   ├── GraphCanvas.tsx
│   │   ├── QueryBar.tsx
│   │   ├── NodeShapes.tsx
│   │   └── DetailPanel.tsx
│   ├── Database/
│   │   ├── DatabaseStudio.tsx
│   │   ├── TableView.tsx
│   │   ├── SQLEditor.tsx
│   │   └── adapters/
│   │       ├── SupabaseAdapter.ts
│   │       ├── SurrealAdapter.ts
│   │       ├── SledAdapter.ts
│   │       └── RedisAdapter.ts
│   ├── Models/
│   │   ├── ModelViewer.tsx
│   │   ├── LayerGraph.tsx
│   │   └── TrainingMetrics.tsx
│   ├── Vectors/
│   │   ├── VectorSearch.tsx
│   │   ├── EmbeddingViz.tsx
│   │   └── CollectionManager.tsx
│   └── ui/
│       └── ... (shadcn components)
├── hooks/
│   ├── useDatabase.ts
│   ├── useGraph.ts
│   ├── useWorkflow.ts
│   └── useKeyboard.ts
├── stores/
│   ├── connectionStore.ts
│   ├── workflowStore.ts
│   └── graphStore.ts
├── lib/
│   ├── adapters/
│   ├── queryParsers/
│   └── exporters/
├── types/
│   └── index.ts
├── App.tsx
└── main.tsx
```

---

## ENVIRONMENT VARIABLES

```env
# Supabase
VITE_SUPABASE_URL=
VITE_SUPABASE_ANON_KEY=

# SurrealDB
VITE_SURREALDB_URL=ws://localhost:8000
VITE_SURREALDB_NS=sx9
VITE_SURREALDB_DB=production

# Sled (via HTTP API)
VITE_SLED_API_URL=http://localhost:18400

# Redis
VITE_REDIS_URL=redis://localhost:6379

# Vector DB
VITE_VECTOR_DB_URL=http://localhost:6333

# App
VITE_APP_NAME=SX9 Forge Workbench
```

---

## BUILD ORDER

1. **Phase 1**: App shell with tab navigation + dark theme
2. **Phase 2**: Database Studio with Supabase table view
3. **Phase 3**: Graph Browser with D3 force simulation
4. **Phase 4**: Forge Canvas with React Flow
5. **Phase 5**: Add SurrealDB adapter + native graph view
6. **Phase 6**: Model Viewer
7. **Phase 7**: Vector search
8. **Phase 8**: Polish + keyboard shortcuts + command palette

---

## START PROMPT FOR BOLT

Copy this to start:

```
Build "SX9 Forge Workbench" - a multi-database workbench combining:
1. n8n-style workflow builder (React Flow)
2. Neo4j-style graph browser (D3.js)
3. Supabase-style table views for multiple databases

Tech: React 18 + TypeScript + Vite + Tailwind + D3 + React Flow + Zustand

DARK THEME ONLY with cyberpunk aesthetic:
- Background: #0a0a0f
- Panels: #12121a
- Borders: #2a2a3a
- Accents: cyan #00ffff, magenta #ff00ff, amber #ffbf00

Start with the app shell:
- Top: Logo + tab bar (Forge, Graph, Supabase, Surreal, Sled, Models, Vectors)
- Main: Content area that changes per tab
- Bottom: Status bar showing database connections

Use monospace font (JetBrains Mono) for code/data, Inter for UI.
Sharp corners on panels, subtle grid background, glow effects on hover.
```

---




