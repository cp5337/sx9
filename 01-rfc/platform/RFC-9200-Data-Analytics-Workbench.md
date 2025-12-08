# RFC-9200: SYNAPTIX9 Data Analytics Workbench v0.1

**Status:** Draft  
**Author:** SX9 Architecture Team  
**Created:** 2024-12-04  
**Updated:** 2024-12-06  
**Target:** Q1 2025  

---

## Abstract

This RFC defines the **SYNAPTIX9 Data Analytics Workbench (DAW)**—the UI surface layer for the SX9 cognitive engine. The workbench is not a new system; it exposes the existing SX9 infrastructure (sx9-atlas-bus, Trivariate Hashing, Polycrystal resonance, SDT gate control, multi-database substrate) through a unified visual interface.

**Key Insight**: Everything below the UI already exists. The DAW is the "glass" that lets operators see and interact with the engine.

---

## 1. Introduction

### 1.1 What Already Exists

The SX9 system has a complete cognitive backend:

| Layer | Component | Status |
|-------|-----------|--------|
| **Storage** | Supabase, SurrealDB, Sled, Sledis, NATS | ✅ Running |
| **Identity** | Trivariate Hash (3×64-bit Base96) | ✅ Implemented |
| **Resonance** | Polycrystal, Crystal Families | ✅ Implemented |
| **Control** | SDT Gate, PlasmaState | ✅ Implemented |
| **IPC** | sx9-atlas-bus (sub-10ns) | ✅ Implemented |
| **Workflow** | Forge Engine | ✅ Implemented |
| **UI** | Unified Workbench | ❌ **THIS RFC** |

### 1.2 What This RFC Adds

The DAW provides the missing UI layer:

1. **Visual Graph Canvas** - Exposes SurrealDB graph + GLAF correlation
2. **Horizon Tabs** - Real-time database status from NATS health subjects
3. **Forge Workflow Canvas** - Visual sx9-atlas-bus workflow state
4. **Query/CLI Mode** - Direct database + LLM interaction
5. **Fusion Nodes** - Cross-database correlation via Trivariate Hash matching

### 1.3 Scope

**In Scope (v0.1):**
- GLAF Graph Browser with visual query builder
- Forge Workflow Canvas
- Database Studio (Supabase, SurrealDB, Sled, Sledis)
- Model Viewer (GNN/ANN architecture)
- Vector Search interface

**Out of Scope (Future):**
- OSINT integration (v0.2)
- Figma MCP connector (v0.3)
- Office 365 integration (v0.3)
- Google Cloud product integration (v0.3)
- Full Rust backend rewrite (v1.0)

---

## 2. Architecture

### 2.1 System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     SX9 DATA ANALYTICS WORKBENCH                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        FRONTEND (React + TypeScript)                 │   │
│  │                                                                      │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ │   │
│  │  │  GLAF    │ │  Forge   │ │ Database │ │  Model   │ │  Vector  │ │   │
│  │  │  Graph   │ │  Canvas  │ │  Studio  │ │  Viewer  │ │  Search  │ │   │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ │   │
│  │       │            │            │            │            │        │   │
│  │  ┌────┴────────────┴────────────┴────────────┴────────────┴────┐  │   │
│  │  │                    UNIFIED STATE (Zustand)                   │  │   │
│  │  └──────────────────────────────┬───────────────────────────────┘  │   │
│  └─────────────────────────────────┼───────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────┼───────────────────────────────────┐   │
│  │                    ADAPTER LAYER (TypeScript)                        │   │
│  │                                                                      │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ │   │
│  │  │ Supabase │ │ SurrealDB│ │   Sled   │ │  Sledis  │ │  Conda   │ │   │
│  │  │ Adapter  │ │ Adapter  │ │ Adapter  │ │ Adapter  │ │  Bridge  │ │   │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ │   │
│  └───────┼────────────┼────────────┼────────────┼────────────┼──────────┘   │
│          │            │            │            │            │              │
│          ▼            ▼            ▼            ▼            ▼              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         BACKEND SERVICES                             │   │
│  │                                                                      │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ │   │
│  │  │ Supabase │ │ SurrealDB│ │   Sled   │ │  Sledis  │ │  Conda   │ │   │
│  │  │ :18000   │ │ :18010   │ │ :18400   │ │ :18401   │ │  :18800  │ │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘ │   │
│  │                                                                      │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐              │   │
│  │  │   NATS   │ │  Forge   │ │  Hashing │ │ Trivar   │              │   │
│  │  │ :18020   │ │ :18350   │ │ :18105   │ │ :18106   │              │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    ATLAS DAEMON (Rust - Legion ECS)                  │   │
│  │                                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │ Graph Engine │  │ Hash Engine  │  │ Tick Sync    │              │   │
│  │  │ (Legion+SIMD)│  │ (murmur3)    │  │ (250ns)      │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  │                           ▲                                         │   │
│  │                    Ring Buffer (SPSC)                               │   │
│  │                           ▼                                         │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │ apecs World  │  │ Query Engine │  │ Change Track │              │   │
│  │  │ (Async I/O)  │  │ (sqlparser)  │  │ (Built-in)   │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Component Responsibilities

| Component | Responsibility | Technology |
|-----------|---------------|------------|
| GLAF Graph | Graph visualization, query building | D3.js, React |
| Forge Canvas | Workflow automation | React Flow |
| Database Studio | Table views, CRUD operations | TanStack Table |
| Model Viewer | Neural network visualization | D3.js, SVG |
| Vector Search | Embedding search, t-SNE viz | Custom |
| Adapter Layer | Database abstraction | TypeScript |
| ATLAS Daemon | Hot path: graph traversal, hashing | Legion ECS, SIMD |
| apecs World | Cold path: async I/O, change tracking | apecs, Tokio |

---

## 3. GLAF Graph Browser

### 3.1 Visual Query Builder

The Visual Query Builder (VQB) is the primary differentiator from Neo4j Browser. Users construct queries through a form-based interface that generates Cypher/SQL/SurrealQL behind the scenes.

#### 3.1.1 Query Model

```typescript
interface VisualQuery {
  // What to find
  find: 'nodes' | 'relationships' | 'paths';
  
  // Starting conditions
  startConditions: Condition[];
  
  // Relationship traversal
  hops: Hop[];
  
  // Result options
  options: {
    limit: number;
    orderBy?: string;
    direction?: 'asc' | 'desc';
  };
}

interface Condition {
  field: string;           // 'label', 'name', 'status', etc.
  operator: Operator;      // 'equals', 'contains', 'greater_than', etc.
  value: string | number;
}

type Operator = 
  | 'equals' 
  | 'not_equals' 
  | 'contains' 
  | 'starts_with' 
  | 'ends_with'
  | 'greater_than' 
  | 'less_than' 
  | 'between'
  | 'in' 
  | 'is_null' 
  | 'is_not_null';

interface Hop {
  relationship: {
    type?: string;         // Optional: filter by relationship type
    direction: 'outgoing' | 'incoming' | 'any';
    minHops?: number;      // For variable-length paths
    maxHops?: number;
  };
  targetConditions: Condition[];
}
```

#### 3.1.2 Query Generation

```typescript
function generateCypher(query: VisualQuery): string {
  const { find, startConditions, hops, options } = query;
  
  // Build MATCH clause
  let cypher = 'MATCH ';
  
  // Start node
  const startLabel = startConditions.find(c => c.field === 'label')?.value;
  const startProps = startConditions
    .filter(c => c.field !== 'label')
    .map(c => `${c.field}: '${c.value}'`)
    .join(', ');
  
  cypher += `(n${startLabel ? ':' + startLabel : ''}${startProps ? ' {' + startProps + '}' : ''})`;
  
  // Add hops
  hops.forEach((hop, i) => {
    const dir = hop.relationship.direction;
    const arrow = dir === 'outgoing' ? '->' : dir === 'incoming' ? '<-' : '-';
    const relType = hop.relationship.type ? ':' + hop.relationship.type : '';
    
    const targetLabel = hop.targetConditions.find(c => c.field === 'label')?.value;
    
    cypher += `-[r${i}${relType}]${arrow}(m${i}${targetLabel ? ':' + targetLabel : ''})`;
  });
  
  // WHERE clause for complex conditions
  const whereConditions: string[] = [];
  startConditions
    .filter(c => c.field !== 'label' && c.operator !== 'equals')
    .forEach(c => {
      whereConditions.push(conditionToCypher('n', c));
    });
  
  if (whereConditions.length > 0) {
    cypher += '\nWHERE ' + whereConditions.join(' AND ');
  }
  
  // RETURN clause
  cypher += '\nRETURN n';
  hops.forEach((_, i) => {
    cypher += `, r${i}, m${i}`;
  });
  
  // Options
  if (options.orderBy) {
    cypher += `\nORDER BY n.${options.orderBy} ${options.direction || 'ASC'}`;
  }
  if (options.limit) {
    cypher += `\nLIMIT ${options.limit}`;
  }
  
  return cypher;
}

function conditionToCypher(alias: string, condition: Condition): string {
  const { field, operator, value } = condition;
  
  switch (operator) {
    case 'equals': return `${alias}.${field} = '${value}'`;
    case 'not_equals': return `${alias}.${field} <> '${value}'`;
    case 'contains': return `${alias}.${field} CONTAINS '${value}'`;
    case 'starts_with': return `${alias}.${field} STARTS WITH '${value}'`;
    case 'greater_than': return `${alias}.${field} > ${value}`;
    case 'less_than': return `${alias}.${field} < ${value}`;
    case 'is_null': return `${alias}.${field} IS NULL`;
    case 'is_not_null': return `${alias}.${field} IS NOT NULL`;
    default: return `${alias}.${field} = '${value}'`;
  }
}
```

#### 3.1.3 Multi-Backend Translation

The VQB generates queries for multiple backends:

```typescript
interface QueryTranslator {
  toCypher(query: VisualQuery): string;      // Neo4j, Memgraph
  toSurrealQL(query: VisualQuery): string;   // SurrealDB
  toSQL(query: VisualQuery): string;         // PostgreSQL (with graph schema)
  toGremlin(query: VisualQuery): string;     // TinkerPop (future)
}

// SurrealDB translation example
function toSurrealQL(query: VisualQuery): string {
  const { startConditions, hops } = query;
  
  const startLabel = startConditions.find(c => c.field === 'label')?.value?.toLowerCase();
  
  let surql = `SELECT * FROM ${startLabel || 'node'}`;
  
  // Add WHERE conditions
  const conditions = startConditions
    .filter(c => c.field !== 'label')
    .map(c => `${c.field} ${operatorToSurQL(c.operator)} '${c.value}'`);
  
  if (conditions.length > 0) {
    surql += ' WHERE ' + conditions.join(' AND ');
  }
  
  // Add graph traversal
  if (hops.length > 0) {
    const hop = hops[0];
    const relType = hop.relationship.type?.toLowerCase() || '*';
    surql = `SELECT *, ->${relType}->* AS connected FROM ${startLabel}`;
  }
  
  return surql;
}
```

### 3.2 Node Visualization

#### 3.2.1 Shape Definitions

Each entity type has a distinct shape for instant recognition:

```typescript
const NODE_SHAPES: Record<string, NodeShapeConfig> = {
  // GLAF Core Entities
  Agent: {
    shape: 'hexagon',
    vertices: 6,
    color: '#00ffff',
    icon: 'Bot',
    description: 'Autonomous agent in the GLAF system',
  },
  Slot: {
    shape: 'octagon',
    vertices: 8,
    color: '#ff00ff',
    icon: 'Grid3x3',
    description: 'Execution slot for agent allocation',
  },
  Tool: {
    shape: 'diamond',
    vertices: 4,
    rotation: 45,
    color: '#ffbf00',
    icon: 'Wrench',
    description: 'Security or utility tool',
  },
  TrivariteHash: {
    shape: 'nonagon',
    vertices: 9,  // 9 for SYNAPTIX9
    color: '#00ff88',
    icon: 'Hash',
    description: 'Trivariate hash identifier',
  },
  
  // Infrastructure
  GroundStation: {
    shape: 'hexagon',
    vertices: 6,
    color: '#38bdf8',
    icon: 'Radio',
    description: 'Satellite ground station',
  },
  Satellite: {
    shape: 'circle',
    color: '#8b5cf6',
    icon: 'Satellite',
    description: 'Orbital satellite',
  },
  
  // Data Entities
  Table: {
    shape: 'rectangle',
    color: '#3ecf8e',
    icon: 'Table',
    description: 'Database table',
  },
  Collection: {
    shape: 'rounded-rect',
    borderRadius: 8,
    color: '#ff00a0',
    icon: 'Folder',
    description: 'Document collection',
  },
  
  // ML Entities
  Model: {
    shape: 'pentagon',
    vertices: 5,
    color: '#ff6b6b',
    icon: 'Brain',
    description: 'Machine learning model',
  },
  
  // Workflow
  Workflow: {
    shape: 'parallelogram',
    skew: 15,
    color: '#ea4b71',
    icon: 'Workflow',
    description: 'Automated workflow',
  },
};
```

#### 3.2.2 SVG Shape Generators

```typescript
function generatePolygonPath(vertices: number, size: number, rotation: number = 0): string {
  const points: string[] = [];
  const angleStep = (2 * Math.PI) / vertices;
  const startAngle = rotation * (Math.PI / 180) - Math.PI / 2;
  
  for (let i = 0; i < vertices; i++) {
    const angle = startAngle + i * angleStep;
    const x = size / 2 + (size / 2) * Math.cos(angle);
    const y = size / 2 + (size / 2) * Math.sin(angle);
    points.push(`${x},${y}`);
  }
  
  return `M ${points.join(' L ')} Z`;
}

function generateDiamondPath(size: number): string {
  const half = size / 2;
  return `M ${half} 0 L ${size} ${half} L ${half} ${size} L 0 ${half} Z`;
}

function generateParallelogramPath(size: number, skew: number): string {
  const offset = size * Math.tan(skew * Math.PI / 180);
  return `M ${offset} 0 L ${size} 0 L ${size - offset} ${size} L 0 ${size} Z`;
}
```

### 3.3 Graph Interactions

| Action | Behavior |
|--------|----------|
| Click node | Select, show details in right panel |
| Double-click node | Expand connected nodes |
| Right-click node | Context menu (Expand, Hide, Edit, Delete, Copy) |
| Drag node | Move and pin position |
| Shift+click | Multi-select |
| Scroll | Zoom |
| Drag canvas | Pan |
| Double-click canvas | Reset view |

---

## 4. Database Adapters

### 4.1 Adapter Interface

```typescript
interface DatabaseAdapter {
  // Identity
  readonly id: string;
  readonly name: string;
  readonly icon: string;
  readonly color: string;
  
  // Connection
  connect(config: ConnectionConfig): Promise<void>;
  disconnect(): Promise<void>;
  isConnected(): boolean;
  testConnection(): Promise<boolean>;
  
  // Schema Discovery
  getTables(): Promise<TableInfo[]>;
  getTableSchema(table: string): Promise<ColumnInfo[]>;
  getRelationships(): Promise<RelationshipInfo[]>;
  
  // CRUD Operations
  query<T>(sql: string): Promise<T[]>;
  insert(table: string, data: Record<string, any>): Promise<any>;
  update(table: string, id: string, data: Record<string, any>): Promise<any>;
  delete(table: string, id: string): Promise<void>;
  
  // Graph Operations (optional)
  getGraphData?(): Promise<GraphData>;
  executeGraphQuery?(query: string): Promise<GraphResult>;
  
  // Real-time (optional)
  subscribe?(table: string, callback: (change: Change) => void): Subscription;
}
```

### 4.2 Sledis Adapter

Sledis is a Redis-compatible interface over Sled, providing:
- RESP protocol compatibility
- Persistence via Sled's embedded storage
- RFC-compliant operations

```typescript
class SledisAdapter implements DatabaseAdapter {
  readonly id = 'sledis';
  readonly name = 'Sledis';
  readonly icon = 'Database';
  readonly color = '#ff9500';
  
  private client: Redis;  // Uses ioredis
  
  async connect(config: ConnectionConfig): Promise<void> {
    this.client = new Redis({
      host: config.host,
      port: config.port || 18401,
      password: config.credentials?.password,
    });
  }
  
  async query<T>(command: string): Promise<T[]> {
    // Parse Redis command
    const [cmd, ...args] = command.split(' ');
    const result = await this.client.call(cmd, ...args);
    return Array.isArray(result) ? result : [result];
  }
  
  // Key-Value operations
  async get(key: string): Promise<string | null> {
    return this.client.get(key);
  }
  
  async set(key: string, value: string, options?: SetOptions): Promise<void> {
    if (options?.ex) {
      await this.client.setex(key, options.ex, value);
    } else {
      await this.client.set(key, value);
    }
  }
  
  async keys(pattern: string): Promise<string[]> {
    return this.client.keys(pattern);
  }
  
  // Hash operations
  async hget(key: string, field: string): Promise<string | null> {
    return this.client.hget(key, field);
  }
  
  async hgetall(key: string): Promise<Record<string, string>> {
    return this.client.hgetall(key);
  }
  
  async hset(key: string, field: string, value: string): Promise<void> {
    await this.client.hset(key, field, value);
  }
  
  // Sorted set operations (for graph edges)
  async zadd(key: string, score: number, member: string): Promise<void> {
    await this.client.zadd(key, score, member);
  }
  
  async zrange(key: string, start: number, stop: number): Promise<string[]> {
    return this.client.zrange(key, start, stop);
  }
  
  // Graph representation in Sledis
  async getGraphData(): Promise<GraphData> {
    // Nodes stored as hashes: node:{id} -> {label, properties...}
    // Edges stored as sorted sets: edges:{from_id} -> [to_id:type, ...]
    
    const nodeKeys = await this.keys('node:*');
    const nodes = await Promise.all(
      nodeKeys.map(async key => {
        const data = await this.hgetall(key);
        return {
          id: key.replace('node:', ''),
          label: data.label,
          properties: JSON.parse(data.properties || '{}'),
        };
      })
    );
    
    const edgeKeys = await this.keys('edges:*');
    const relationships: GraphRelationship[] = [];
    
    for (const key of edgeKeys) {
      const fromId = key.replace('edges:', '');
      const edges = await this.zrange(key, 0, -1);
      
      for (const edge of edges) {
        const [toId, type] = edge.split(':');
        relationships.push({
          id: `${fromId}-${type}-${toId}`,
          type,
          sourceId: fromId,
          targetId: toId,
          properties: {},
        });
      }
    }
    
    return { nodes, relationships };
  }
}
```

---

## 5. Forge Workflow Engine

### 5.1 Workflow Definition

```typescript
interface Workflow {
  id: string;
  name: string;
  description?: string;
  nodes: WorkflowNode[];
  edges: WorkflowEdge[];
  settings: WorkflowSettings;
  createdAt: Date;
  updatedAt: Date;
}

interface WorkflowNode {
  id: string;
  type: string;           // 'webhook', 'supabase', 'filter', etc.
  position: { x: number; y: number };
  data: {
    label: string;
    config: Record<string, any>;
  };
}

interface WorkflowEdge {
  id: string;
  source: string;         // Node ID
  sourceHandle?: string;  // Output port
  target: string;         // Node ID
  targetHandle?: string;  // Input port
}

interface WorkflowSettings {
  timezone: string;
  errorHandling: 'stop' | 'continue' | 'retry';
  maxRetries: number;
  retryDelay: number;
}
```

### 5.2 Node Types

```typescript
const WORKFLOW_NODE_TYPES = {
  // Triggers
  webhook: {
    category: 'trigger',
    inputs: [],
    outputs: ['data'],
    config: {
      path: { type: 'string', required: true },
      method: { type: 'enum', values: ['GET', 'POST', 'PUT', 'DELETE'] },
      authentication: { type: 'enum', values: ['none', 'basic', 'bearer', 'api_key'] },
    },
  },
  schedule: {
    category: 'trigger',
    inputs: [],
    outputs: ['trigger'],
    config: {
      cron: { type: 'string', required: true },
      timezone: { type: 'string', default: 'UTC' },
    },
  },
  nats_subscribe: {
    category: 'trigger',
    inputs: [],
    outputs: ['message'],
    config: {
      subject: { type: 'string', required: true },
      queue: { type: 'string' },
    },
  },
  
  // Databases
  supabase: {
    category: 'database',
    inputs: ['trigger'],
    outputs: ['data', 'error'],
    config: {
      operation: { type: 'enum', values: ['select', 'insert', 'update', 'delete', 'rpc'] },
      table: { type: 'string', required: true },
      filters: { type: 'json' },
    },
  },
  surrealdb: {
    category: 'database',
    inputs: ['trigger'],
    outputs: ['data', 'error'],
    config: {
      query: { type: 'string', required: true },
      variables: { type: 'json' },
    },
  },
  sled: {
    category: 'database',
    inputs: ['trigger'],
    outputs: ['data', 'error'],
    config: {
      operation: { type: 'enum', values: ['get', 'set', 'delete', 'scan'] },
      key: { type: 'string' },
      prefix: { type: 'string' },
    },
  },
  sledis: {
    category: 'database',
    inputs: ['trigger'],
    outputs: ['data', 'error'],
    config: {
      command: { type: 'string', required: true },  // Redis command
    },
  },
  
  // Transforms
  filter: {
    category: 'transform',
    inputs: ['data'],
    outputs: ['passed', 'rejected'],
    config: {
      conditions: { type: 'json', required: true },
      mode: { type: 'enum', values: ['all', 'any'] },
    },
  },
  map: {
    category: 'transform',
    inputs: ['data'],
    outputs: ['data'],
    config: {
      expression: { type: 'string', required: true },
    },
  },
  code: {
    category: 'transform',
    inputs: ['data'],
    outputs: ['data', 'error'],
    config: {
      language: { type: 'enum', values: ['javascript', 'python'] },
      code: { type: 'code', required: true },
    },
  },
  trivariate_hash: {
    category: 'transform',
    inputs: ['data'],
    outputs: ['hash'],
    config: {
      fields: { type: 'array', items: 'string' },
      algorithm: { type: 'enum', values: ['murmur3', 'xxhash', 'blake3'] },
    },
  },
  thalmic_filter: {
    category: 'transform',
    inputs: ['data'],
    outputs: ['passed', 'suppressed'],
    config: {
      threshold: { type: 'number', default: 0.67 },
      suppressionRules: { type: 'json' },
    },
  },
  
  // AI/ML
  conda: {
    category: 'ai',
    inputs: ['data'],
    outputs: ['result', 'error'],
    config: {
      environment: { type: 'enum', values: ['sx9-base', 'sx9-ml', 'sx9-geo', 'sx9-astro', 'sx9-graph'] },
      code: { type: 'code', language: 'python', required: true },
    },
  },
  llm_prompt: {
    category: 'ai',
    inputs: ['data'],
    outputs: ['response', 'error'],
    config: {
      model: { type: 'string', default: 'gpt-4' },
      prompt: { type: 'template', required: true },
      temperature: { type: 'number', default: 0.7 },
    },
  },
  embedding: {
    category: 'ai',
    inputs: ['text'],
    outputs: ['vector'],
    config: {
      model: { type: 'string', default: 'text-embedding-3-small' },
    },
  },
  
  // Outputs
  nats_publish: {
    category: 'output',
    inputs: ['data'],
    outputs: [],
    config: {
      subject: { type: 'string', required: true },
    },
  },
  http_request: {
    category: 'output',
    inputs: ['data'],
    outputs: ['response', 'error'],
    config: {
      url: { type: 'string', required: true },
      method: { type: 'enum', values: ['GET', 'POST', 'PUT', 'DELETE'] },
      headers: { type: 'json' },
      body: { type: 'template' },
    },
  },
};
```

---

## 6. Integration Roadmap

### 6.1 v0.1 (Current)

| Integration | Status | Notes |
|-------------|--------|-------|
| Supabase | ✅ Ready | Full adapter |
| SurrealDB | ✅ Ready | Full adapter |
| Sled | ✅ Ready | HTTP API |
| Sledis | ✅ Ready | RESP protocol |
| NATS | ✅ Ready | Pub/sub |
| Conda | ✅ Ready | Python bridge |

### 6.2 v0.2 (Q2 2025)

| Integration | Status | Notes |
|-------------|--------|-------|
| OSINT Tools | 🔄 Planned | Shodan, VirusTotal, etc. |
| Neo4j | 🔄 Planned | Direct Bolt protocol |
| Memgraph | 🔄 Planned | Cypher compatible |
| ClickHouse | 🔄 Planned | Analytics |

### 6.3 v0.3 (Q3 2025)

| Integration | Status | Notes |
|-------------|--------|-------|
| Google Cloud | 🔄 Planned | BigQuery, GCS, Vertex AI |
| Office 365 | 🔄 Planned | Graph API, SharePoint |
| Figma | 🔄 Planned | MCP connector |
| Notion | 🔄 Planned | API integration |

### 6.4 v1.0 (Q4 2025)

| Integration | Status | Notes |
|-------------|--------|-------|
| Full Rust Backend | 🔄 Planned | Replace TypeScript adapters |
| WASM Query Engine | 🔄 Planned | Client-side query execution |
| P2P Sync | 🔄 Planned | Decentralized data |

---

## 7. Security Considerations

### 7.1 Authentication

- API key per database connection
- OAuth2 for cloud integrations
- Biometric verification for sensitive operations (Plasma)

### 7.2 Authorization

- Row-level security (RLS) passthrough for Supabase
- Namespace isolation for SurrealDB
- Key prefix restrictions for Sled/Sledis

### 7.3 Data Protection

- TLS for all connections
- Encryption at rest (database-dependent)
- Audit logging to NATS

---

## 8. Performance Requirements

| Metric | Target |
|--------|--------|
| Graph render (1K nodes) | < 100ms |
| Graph render (10K nodes) | < 1s (WebGL) |
| Query execution | < 500ms (p95) |
| Real-time update latency | < 100ms |
| Workflow step execution | < 200ms |

---

## 9. References

- RFC-9005: Unified Graph Schema
- RFC-9112: Deterministic Prompt Engineering
- **SX9-UNIFIED-HASH-SPEC.md**: Trivariate Hash, SDT, Crystal, eBPF Pipeline
- **SDT-PROTOCOL-SPEC.md**: Software-Defined Thyristor Layer 2 Protocol
- Neo4j Browser: https://github.com/neo4j/neo4j-browser
- React Flow: https://reactflow.dev/
- D3.js Force Simulation: https://d3js.org/d3-force

---

## 10. Appendix

### A. Port Allocation

See `PORTS-CDN-CONDA-SPEC.md` for complete port matrix.

### B. Database Schema

See RFC-9005 for unified graph schema.

### C. Conda Environments

See `PORTS-CDN-CONDA-SPEC.md` for environment definitions.

---

**END OF RFC-9200**

