# Synaptix9 Core - Function Reference

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           SX9Orchestrator                               │
│                     (Central Coordination Hub)                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────┐          ┌─────────────────────┐              │
│  │ ScriptExecution     │          │ LegionExecution     │              │
│  │ Coordinator         │          │ Engine              │              │
│  │                     │          │                     │              │
│  │ • DAG execution     │          │ • Task execution    │              │
│  │ • Plan management   │          │ • Schema watching   │              │
│  │ • Script routing    │          │ • 1n/2n forms       │              │
│  └──────────┬──────────┘          └──────────┬──────────┘              │
│             │                                │                          │
│  ┌──────────┴──────────┐          ┌──────────┴──────────┐              │
│  │ SlotGraphQuery      │          │ HashingEngine       │              │
│  │ Engine              │          │ Connector           │              │
│  │                     │          │                     │              │
│  │ • Dijkstra routing  │          │ • Murmur3 hashing   │              │
│  │ • BFS pathfinding   │          │ • Batch processing  │              │
│  │ • Network analysis  │          │ • Compression       │              │
│  └─────────────────────┘          └─────────────────────┘              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 1. SX9Orchestrator

**Purpose:** Central nervous system that coordinates all subsystems, manages connections, and provides unified system status.

### Constructor & Lifecycle

| Method | Description |
|--------|-------------|
| `constructor()` | Creates instances of all subsystems (ScriptCoordinator, LegionEngine, DatabaseMux, HashingEngine) and initializes 14 connection targets |
| `initialize(): Promise<boolean>` | Boots the entire system - initializes script coordinator, hashing engine, updates connection statuses. Call this first. |
| `shutdown(): Promise<void>` | Graceful shutdown - stops hashing engine, resets all connections to disconnected |

### Connection Management

| Method | Description |
|--------|-------------|
| `startSystemConnection(): Promise<boolean>` | Executes HD4 connection plan followed by Legion coordination plan. Returns true if all succeed. |
| `testConnection(targetId: string): Promise<boolean>` | Tests a specific connection target by ID. For databases: HTTP check. For APIs: health endpoint. For frontend/backend: dependency check. |
| `getConnectionTargets(): ConnectionTarget[]` | Returns all 14 connection targets with their current status |
| `getConnectionPlan(): CoordinationPlan[]` | Returns all loaded coordination plans from ScriptCoordinator |

### System Status

| Method | Description |
|--------|-------------|
| `getSystemStatus(): Promise<SX9SystemStatus>` | Returns comprehensive status: overall health, component states, connectivity booleans, statistics |
| `updateConnectionStatuses(): Promise<void>` | (Private) Refreshes all connection statuses from subsystems |
| `updateFrontendStatuses(): void` | (Private) Propagates dependency status to frontend components |

### Hashing Shortcuts

| Method | Description |
|--------|-------------|
| `getHashingEngine(): HashingEngineConnector` | Direct access to hashing engine instance |
| `hashThreatIntelligence(indicators: string[]): Promise<Record<string, string>>` | Hash threat indicators with high priority |
| `hashDocuments(documents: Array<{id, content}>): Promise<Array<{id, hash, compressed}>>` | Hash documents for USIM system |
| `hashLegionTasks(tasks: Array<{id, script, world}>): Promise<Array<{taskId, scriptHash, worldHash}>>` | Hash Legion task scripts and world context |
| `getHashingPerformance(): Promise<{requestsPerSecond, avgProcessingTime, compressionEfficiency, uptime}>` | Get hashing engine metrics |

### Connection Targets (14 Total)

```
Databases (4):
  - surrealdb         → http://localhost:11451
  - supabase          → https://lgdatoqcajaqhtbyzfef.supabase.co
  - sled              → (local KV store)
  - legion-slot-graph → (graph database)

APIs (1):
  - hashing-engine    → http://localhost:18105

Frontend (5):
  - dashboard-stats        [depends: legion-slot-graph, database-mux]
  - hd4-task-views         [depends: legion-slot-graph, database-mux]
  - view-switching         [depends: hd4-task-views]
  - database-console       [depends: database-mux]
  - legion-visualization   [depends: legion-slot-graph]

Backend (3):
  - database-mux           [depends: surrealdb, supabase, sled]
  - legion-engine          [depends: database-mux, legion-slot-graph]
  - script-coordinator     [depends: legion-engine, database-mux]
```

---

## 2. ScriptExecutionCoordinator

**Purpose:** Executes coordination plans with dependency-aware ordering. Manages script execution lifecycle.

### Lifecycle

| Method | Description |
|--------|-------------|
| `initialize(): Promise<void>` | Initializes LegionEngine, DatabaseMux, loads predefined coordination plans |
| `loadCoordinationPlans(): Promise<void>` | (Private) Creates HD4 and Legion coordination plans |

### Plan Execution

| Method | Description |
|--------|-------------|
| `executeCoordinationPlan(planId: string): Promise<boolean>` | Executes a plan by ID. Calculates dependency order, runs scripts sequentially. |
| `calculateExecutionOrder(executions: ScriptExecution[]): ScriptExecution[]` | (Private) Topological sort - returns scripts in dependency-safe order |
| `executeScript(execution: ScriptExecution): Promise<boolean>` | Runs a single script, tracks timing, updates status |

### Built-in Scripts

| Script Name | What It Does |
|-------------|--------------|
| `connectDashboardToRealData` | Fetches Legion task count, checks DB status |
| `connectHD4TasksToLegion` | Queries tasks by HD4 phase (Hunt/Detect/Disrupt/Disable/Dominate) |
| `connectViewSwitching` | Sets up GIS/Grid/Graph view routing |
| `connectDatabaseMuxGlobally` | Verifies all database connections are active |
| `mapAdversaryTasks` | Scans all 4 worlds for 1n (adversary) form tasks |
| `mapCounterAdversaryTasks` | Scans all 4 worlds for 2n (counter-adversary) form tasks |
| `coordinateTaskPairs` | Links 1n/2n task pairs for coordination |

### Status & Monitoring

| Method | Description |
|--------|-------------|
| `getCoordinationPlan(planId: string): CoordinationPlan` | Get a specific plan by ID |
| `getAllCoordinationPlans(): CoordinationPlan[]` | Get all loaded plans |
| `getRunningExecutions(): ScriptExecution[]` | Get currently executing scripts |
| `getSystemStatus(): Promise<{legionEngine, databaseMux, runningExecutions, activePlans}>` | Subsystem status summary |

### Predefined Plans

**HD4 Connection Plan** (`hd4-connection-plan`):
```
connect-dashboard-stats → connect-hd4-tasks → connect-view-switching
                    ↘                      ↗
                      connect-database-mux
```

**Legion Coordination Plan** (`legion-coordination-plan`):
```
map-adversary-tasks → map-counter-tasks → coordinate-task-pairs
```

---

## 3. LegionExecutionEngine

**Purpose:** Executes Legion tasks across 4 worlds (Cyber, Geographical, Space, Maritime) with 1n/2n adversary form coordination.

### Lifecycle

| Method | Description |
|--------|-------------|
| `initializeEngine(): Promise<void>` | Initializes DB connections, loads crates, initializes node capabilities |
| `loadAvailableCrates(): Promise<void>` | Queries `comprehensive_crate_interviews` for capability crates |
| `initializeNodeCapabilities(): Promise<void>` | Queries `node_interviews` for 1n/2n form mappings |

### Task Execution

| Method | Description |
|--------|-------------|
| `executeTask(task: LegionTask): Promise<boolean>` | Full task execution: builds context, runs coordination script, updates status |
| `buildExecutionContext(task): Promise<ExecutionContext>` | (Private) Assembles crates, DB connections, node capabilities for a task |
| `executeCoordinationScript(task, context): Promise<boolean>` | (Private) Validates context and runs task logic |

### Context Building (Private)

| Method | Description |
|--------|-------------|
| `getCratesForTask(task): Promise<Record<string, any>>` | Queries crates matching task's world/phase/form |
| `getDatabaseConnectionsForTask(task): Promise<Record<string, any>>` | Gets DB connections specified in task.databases |
| `getNodeCapabilitiesForTask(task): Promise<Record<string, any>>` | Queries node capabilities matching task parameters |

### Task Queries

| Method | Description |
|--------|-------------|
| `getTaskStatus(taskId: string): Promise<LegionTask \| null>` | Get task by ID |
| `getAllRunningTasks(): Promise<LegionTask[]>` | Get all tasks with status='running' |
| `getTasksByWorld(world: string): Promise<LegionTask[]>` | Filter tasks by world (Cyber/Geographical/Space/Maritime) |
| `getTasksByHD4Phase(phase: string): Promise<LegionTask[]>` | Filter tasks by HD4 phase |

### 1n/2n Coordination

| Method | Description |
|--------|-------------|
| `coordinateAdversaryCounterOps(adversaryTaskId, counterTaskId): Promise<boolean>` | Links a 1n (adversary) task with its 2n (counter-adversary) counterpart |

### Schema Watching (MCP Integration)

| Method | Description |
|--------|-------------|
| `handleSchemaUpdateEvent(event: SchemaUpdateEvent): void` | Process incoming schema update from MCP bus |
| `addSchemaUpdateListener(listener): void` | Subscribe to schema changes |
| `removeSchemaUpdateListener(listener): void` | Unsubscribe from schema changes |
| `getSchemaStatus(): {current_hash, last_update, listeners_count, is_watching}` | Current schema state |

### LegionTask Interface

```typescript
interface LegionTask {
  id: string;
  name: string;
  world: 'Cyber' | 'Geographical' | 'Space' | 'Maritime';
  form: '1n' | '2n';  // Adversary vs Counter-adversary
  hd4Phase: 'Hunt' | 'Detect' | 'Disrupt' | 'Disable' | 'Dominate';
  crates: string[];
  databases: string[];
  nodes: string[];
  executionScript: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
}
```

---

## 4. SlotGraphQueryEngine

**Purpose:** Graph database query engine with Dijkstra routing, BFS pathfinding, and network analysis. Dual-database: SurrealDB primary, Supabase fallback.

### Initialization

| Method | Description |
|--------|-------------|
| `constructor()` | Auto-initializes SurrealDB connection |
| `initializeSurrealDB(): Promise<void>` | (Private) Signs into SurrealDB, stores auth token |
| `querySurreal<T>(query, vars?): Promise<T>` | (Private) Execute raw SurrealQL query |

### Route Finding

| Method | Description |
|--------|-------------|
| `findAllRoutes(source, dest, maxHops=5): Promise<Route[]>` | Find ALL possible routes via graph traversal. Falls back to BFS on Supabase. |
| `findAllRoutesSupabase(source, dest, maxHops): Promise<Route[]>` | (Private) BFS implementation using adjacency list |
| `findOptimalRoute(source, dest, constraints): Promise<Route \| null>` | **Dijkstra's algorithm** with constraint filtering (latency, QKD, reliability, bandwidth) |
| `findOptimalRouteSupabase(source, dest, constraints): Promise<Route \| null>` | (Private) Dijkstra fallback on Supabase data |

### RouteConstraints Interface

```typescript
interface RouteConstraints {
  maxLatency?: number;      // Max total latency in ms
  requireQKD?: boolean;     // Require quantum key distribution
  minReliability?: number;  // Minimum link reliability (0-1)
  minBandwidth?: number;    // Minimum bandwidth in Gbps
}
```

### Network Analysis

| Method | Description |
|--------|-------------|
| `findBottlenecks(threshold=0.8): Promise<NetworkLink[]>` | Find links with utilization above threshold |
| `simulateFailure(nodeIds: string[]): Promise<NetworkImpact>` | Simulate node failures, calculate impact on routes |
| `analyzeTrafficPatterns(timeRange): Promise<Pattern[]>` | Analyze traffic patterns over time period |

### NetworkImpact Interface

```typescript
interface NetworkImpact {
  failedNodes: string[];
  totalRoutes: number;
  routesAffected: number;
  alternateRoutesAvailable: number;  // 0-1 ratio
  avgLatencyIncrease: number;
  networkHealthScore: number;        // 0-1
  criticalPaths: string[][];
}
```

### General Queries

| Method | Description |
|--------|-------------|
| `queryHFTNetwork(query: SlotGraphQuery): Promise<SlotGraphResult>` | Generic query with filters, pagination |

### Helper Methods (Private)

| Method | Description |
|--------|-------------|
| `convertPathsToRoutes(paths, source, dest): Route[]` | Convert SurrealDB paths to Route objects |
| `createRouteFromPath(source, dest, path): Route` | Build Route with calculated latency, bandwidth, QKD status |
| `countConnectedPairs(graph, nodes): number` | Count reachable node pairs |
| `isConnected(graph, start, end): boolean` | BFS connectivity check |
| `findCriticalPaths(graph, nodes): string[][]` | Find articulation points (single points of failure) |

---

## 5. HashingEngineConnector

**Purpose:** Client for containerized Murmur3 hashing service with batch processing, compression, and multiple output formats.

### Lifecycle

| Method | Description |
|--------|-------------|
| `constructor(baseUrl?)` | Set base URL (default: http://localhost:18005) |
| `initialize(): Promise<boolean>` | Check connection, load status, log capabilities |
| `checkConnection(): Promise<boolean>` | HTTP GET /health |
| `loadStatus(): Promise<HashingEngineStatus \| null>` | HTTP GET /status |
| `shutdown(): Promise<void>` | Clear queue, reset state |

### Single Hashing

| Method | Description |
|--------|-------------|
| `hashData(request: HashRequest): Promise<HashResponse>` | Hash single item with specified algorithm/format/compression |

### HashRequest Interface

```typescript
interface HashRequest {
  data: string;
  algorithm?: 'murmur3';  // Currently only murmur3
  format?: 'hex' | 'base64' | 'base96' | 'binary';
  compress?: boolean;
  metadata?: Record<string, any>;
}
```

### Batch Hashing

| Method | Description |
|--------|-------------|
| `batchHash(requests: HashRequest[], priority?): Promise<BatchHashResponse>` | Hash multiple items with priority queue |

### Domain-Specific Hashing

| Method | Description |
|--------|-------------|
| `hashForThreatIntelligence(indicators: string[]): Promise<Record<string, string>>` | Hash threat indicators (HIGH priority, base96, compressed) |
| `hashForDocumentManager(documents: Array<{id, content}>): Promise<Array<{id, hash, compressed}>>` | Hash documents for USIM (NORMAL priority) |
| `hashForLegionTasks(tasks: Array<{id, script, world}>): Promise<Array<{taskId, scriptHash, worldHash}>>` | Hash task scripts AND world context (HIGH priority) |

### Status & Monitoring

| Method | Description |
|--------|-------------|
| `getStatus(): HashingEngineStatus \| null` | Cached status object |
| `isOnline(): boolean` | Connection state |
| `getQueueStatus(): {queuedRequests, queuedItems}` | Pending batch count |
| `getPerformanceMetrics(): Promise<{requestsPerSecond, avgProcessingTime, compressionEfficiency, uptime}>` | Performance stats |

---

## Key Concepts

### HD4 Phase Model

```
Hunt → Detect → Disrupt → Disable → Dominate
  │       │        │         │          │
  │       │        │         │          └── Total control achieved
  │       │        │         └── Target neutralized
  │       │        └── Active interference
  │       └── Threat identified
  └── Reconnaissance/search
```

### 4-World Model

| World | Domain |
|-------|--------|
| Cyber | Digital/network operations |
| Geographical | Physical/terrain operations |
| Space | Orbital/satellite operations |
| Maritime | Naval/ocean operations |

### 1n/2n Form Model

| Form | Role | Description |
|------|------|-------------|
| 1n | Adversary | Offensive/attack tasks |
| 2n | Counter-adversary | Defensive/response tasks |

Tasks are paired: each 1n task has a corresponding 2n task for red team/blue team coordination.

---

## Usage Example

```typescript
import { SX9Orchestrator } from '@synaptix9/core';

async function main() {
  // 1. Create and initialize
  const orchestrator = new SX9Orchestrator();
  await orchestrator.initialize();

  // 2. Connect all systems
  await orchestrator.startSystemConnection();

  // 3. Check status
  const status = await orchestrator.getSystemStatus();
  console.log(`System: ${status.overall}`);
  console.log(`Active tasks: ${status.statistics.activeTasks}`);

  // 4. Hash some data
  const hashes = await orchestrator.hashThreatIntelligence([
    '192.168.1.1',
    'malware.exe',
    'evil-domain.com'
  ]);

  // 5. Shutdown
  await orchestrator.shutdown();
}
```




