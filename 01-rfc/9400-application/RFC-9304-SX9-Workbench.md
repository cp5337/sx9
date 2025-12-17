# RFC-9304: SYNAPTIX9 Workbench

**Version:** 0.1.0  
**Status:** DRAFT  
**Date:** 2025-12-06  
**Author:** CTAS Architecture Team  
**Depends-On:** RFC-9302 (Nonagon), RFC-9303 (Realms/Kinematics)  

---

## Abstract

This RFC defines the **SYNAPTIX9 Workbench** — a unified multi-workspace platform for:

- **Data Analytics** — Graph visualization with Nonagon fusion nodes
- **Tool Chains** — TETH-integrated tool orchestration  
- **Agents** — Autonomous agent management and monitoring
- **Workflows** — Rust-native workflow engine (Forge)

The workbench implements the SX9 architectural constant (9) across all layers:
- **9 Realms** as operational contexts
- **9-sided Nonagon** as fusion node geometry
- **9 workspace modes** for different operational needs

---

## 1. Architectural Foundation

### 1.1 SX9 Constant Expression

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SYNAPTIX9 WORKBENCH ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                              SX9 CONSTANT (9)                               │
│                                    │                                        │
│         ┌──────────────────────────┼──────────────────────────┐            │
│         │                          │                          │            │
│         ▼                          ▼                          ▼            │
│   ┌───────────┐            ┌───────────┐            ┌───────────┐          │
│   │  NONAGON  │            │   NINE    │            │  NINE     │          │
│   │   NODE    │            │  REALMS   │            │ WORKSPACES│          │
│   │ (RFC-9302)│            │(RFC-9303) │            │ (RFC-9304)│          │
│   └─────┬─────┘            └─────┬─────┘            └─────┬─────┘          │
│         │                        │                        │                │
│         │    ┌───────────────────┼───────────────────┐    │                │
│         │    │                   │                   │    │                │
│         ▼    ▼                   ▼                   ▼    ▼                │
│   ┌─────────────────────────────────────────────────────────────┐          │
│   │                      WORKBENCH UI                           │          │
│   │                                                             │          │
│   │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │          │
│   │  │  GRAPH  │ │  FORGE  │ │  QUERY  │ │ AGENTS  │  ...      │          │
│   │  └─────────┘ └─────────┘ └─────────┘ └─────────┘           │          │
│   │                                                             │          │
│   └─────────────────────────────────────────────────────────────┘          │
│                                    │                                        │
│                                    ▼                                        │
│   ┌─────────────────────────────────────────────────────────────┐          │
│   │                    MULTI-DATABASE LAYER                     │          │
│   │  [Supabase] [SurrealDB] [Sled] [Sledis] [NATS] [Fusion]    │          │
│   └─────────────────────────────────────────────────────────────┘          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Nine Workspaces

| Index | Workspace | Realm Alignment | Primary Function |
|-------|-----------|-----------------|------------------|
| 0 | **COMMAND** | Aether | C2 dashboard, mission control |
| 1 | **GRAPH** | Cyber | Network/entity graph visualization |
| 2 | **FORGE** | Kinetic | Workflow orchestration |
| 3 | **INTEL** | Cognitive | Intelligence analysis, reports |
| 4 | **ORBITAL** | Orbital | Space asset tracking |
| 5 | **MARITIME** | Maritime | Naval/underwater ops |
| 6 | **TUNNEL** | Subterranean | Infrastructure/underground |
| 7 | **SPECTRUM** | Spectrum | EMS/RF analysis |
| 8 | **TIMELINE** | Temporal | Temporal analysis, scheduling |

---

## 2. UI Layout Specification

### 2.1 Primary Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SYNAPTIX9 WORKBENCH                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ GLYPH RAIL (42px) ──────────────────────────────────────────────────┐   │
│  │ ◀ │ Navigation Glyphs (10px vertical)                                │   │
│  │   │ ┌───┐                                                            │   │
│  │   │ │ ⌘ │ Command (Realm 0)                                          │   │
│  │   │ │ ⬡ │ Graph (Realm 1)                                            │   │
│  │   │ │ ⚡ │ Forge (Realm 2)                                            │   │
│  │   │ │ 🧠 │ Intel (Realm 3)                                            │   │
│  │   │ │ 🛰 │ Orbital (Realm 4)                                          │   │
│  │   │ │ ⚓ │ Maritime (Realm 5)                                         │   │
│  │   │ │ ⛏ │ Tunnel (Realm 6)                                           │   │
│  │   │ │ 📡 │ Spectrum (Realm 7)                                         │   │
│  │   │ │ ⏱ │ Timeline (Realm 8)                                         │   │
│  │   │ ├───┤                                                            │   │
│  │   │ │ 🔗 │ Connections                                                │   │
│  │   │ │ ⚙️ │ Settings                                                   │   │
│  │   │ └───┘                                                            │   │
│  └───┴──────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─ HORIZON TABS (Database Context) ────────────────────────────────────┐   │
│  │ [●Supabase] [●SurrealDB] [●Sled] [○Sledis] [●NATS] [◐Fusion]        │   │
│  │  #3ecf8e     #ff00a0     #ff6b35  #666      #4222ff  #00ffff         │   │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─ REALM INDICATOR ────────────────────────────────────────────────────┐   │
│  │ ▣ REALM 1: CYBER │ Crystal: propagation=5.0 damping=0.01            │   │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─ MAIN CANVAS ────────────────────────────────────────────────────────┐   │
│  │                                                                       │   │
│  │                    << WORKSPACE-SPECIFIC CONTENT >>                  │   │
│  │                                                                       │   │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─ STATUS BAR ─────────────────────────────────────────────────────────┐   │
│  │ ● 5 DBs │ ⚡ 3 workflows │ 🤖 7 agents │ 📊 1.2k nodes │ Δ(0.5,0.7,0.3)│  │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Glyph Rail

```rust
/// Glyph rail navigation item
#[derive(Debug, Clone)]
pub struct GlyphItem {
    /// Workspace index (0-8 for realms, 9+ for system)
    pub index: u8,
    
    /// Unicode glyph
    pub glyph: char,
    
    /// Label (shown when expanded)
    pub label: String,
    
    /// Associated realm (if workspace)
    pub realm: Option<Realm>,
    
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    
    /// Badge count (notifications)
    pub badge: Option<u32>,
}

impl GlyphItem {
    pub fn workspaces() -> Vec<Self> {
        vec![
            Self { index: 0, glyph: '⌘', label: "Command".into(), realm: Some(Realm::Aether), shortcut: Some("⌘1".into()), badge: None },
            Self { index: 1, glyph: '⬡', label: "Graph".into(), realm: Some(Realm::Cyber), shortcut: Some("⌘2".into()), badge: None },
            Self { index: 2, glyph: '⚡', label: "Forge".into(), realm: Some(Realm::Kinetic), shortcut: Some("⌘3".into()), badge: None },
            Self { index: 3, glyph: '🧠', label: "Intel".into(), realm: Some(Realm::Cognitive), shortcut: Some("⌘4".into()), badge: None },
            Self { index: 4, glyph: '🛰', label: "Orbital".into(), realm: Some(Realm::Orbital), shortcut: Some("⌘5".into()), badge: None },
            Self { index: 5, glyph: '⚓', label: "Maritime".into(), realm: Some(Realm::Maritime), shortcut: Some("⌘6".into()), badge: None },
            Self { index: 6, glyph: '⛏', label: "Tunnel".into(), realm: Some(Realm::Subterranean), shortcut: Some("⌘7".into()), badge: None },
            Self { index: 7, glyph: '📡', label: "Spectrum".into(), realm: Some(Realm::Spectrum), shortcut: Some("⌘8".into()), badge: None },
            Self { index: 8, glyph: '⏱', label: "Timeline".into(), realm: Some(Realm::Temporal), shortcut: Some("⌘9".into()), badge: None },
        ]
    }
    
    pub fn system() -> Vec<Self> {
        vec![
            Self { index: 10, glyph: '🔗', label: "Connections".into(), realm: None, shortcut: Some("⌘,".into()), badge: None },
            Self { index: 11, glyph: '⚙', label: "Settings".into(), realm: None, shortcut: Some("⌘.".into()), badge: None },
        ]
    }
}
```

---

## 3. Graph Workspace (Nonagon Integration)

### 3.1 Node Shape System

| Label | Shape | Sides | Color | Description |
|-------|-------|-------|-------|-------------|
| **Fusion** | Nonagon | 9 | #00ffff | Cross-database entity |
| Agent | Hexagon | 6 | #00ffff | Autonomous agent |
| Slot | Octagon | 8 | #ff00ff | Execution slot |
| Tool | Diamond | 4 | #ffbf00 | TETH tool |
| Hash | Heptagon | 7 | #00ff88 | Trivariate hash |
| Workflow | Pentagon | 5 | #ea4b71 | Forge workflow |
| Realm | Nonagon | 9 | realm.color | Realm container |

### 3.2 Fusion Node (Nonagon)

```rust
/// Fusion node - cross-database entity correlation
/// Rendered as Nonagon (9 sides) per RFC-9302
#[derive(Debug, Clone)]
pub struct FusionNode {
    /// Node ID (SX9-UUID)
    pub id: Uuid,
    
    /// Trivariate hash
    pub trivariate_hash: String,
    
    /// Nonagon analytic data
    pub nonagon: NonagonNode,
    
    /// Source database links
    pub sources: Vec<FusionSource>,
    
    /// Fusion confidence (0.0 - 1.0)
    pub fusion_score: f64,
    
    /// How fusion was detected
    pub fusion_method: FusionMethod,
    
    /// Visual properties
    pub position: Position,
    pub size: f64,
    pub color: String,
    
    /// Crystal tuning for this node's realm
    pub realm_tuning: RealmTuning,
}

#[derive(Debug, Clone)]
pub struct FusionSource {
    pub database: DatabaseType,
    pub table_or_collection: String,
    pub record_id: String,
    pub last_sync: u64,
    pub sync_status: SyncStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum FusionMethod {
    /// SCH hash match
    Hash,
    /// LLM semantic similarity
    Semantic,
    /// User manually linked
    Manual,
    /// ML model detected
    MachineLearning,
    /// Graph structure similarity
    GraphStructure,
    /// Temporal correlation
    TemporalCorrelation,
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseType {
    Supabase,
    SurrealDB,
    Sled,
    Sledis,
    NATS,
}
```

### 3.3 Graph Canvas Integration

```typescript
// React component for graph canvas
interface GraphCanvasProps {
  nodes: GraphNode[];
  edges: GraphEdge[];
  realm: Realm;
  crystalTuning: RealmTuning;
  onNodeSelect: (node: GraphNode) => void;
  onFusionDetect: (nodes: GraphNode[]) => void;
}

// Node rendering based on shape
function renderNode(node: GraphNode, ctx: CanvasRenderingContext2D) {
  switch (node.shape) {
    case 'nonagon':
      renderNonagon(node, ctx);  // 9-sided fusion node
      break;
    case 'hexagon':
      renderPolygon(node, ctx, 6);  // Agent
      break;
    case 'octagon':
      renderPolygon(node, ctx, 8);  // Slot
      break;
    case 'heptagon':
      renderPolygon(node, ctx, 7);  // Hash
      break;
    case 'pentagon':
      renderPolygon(node, ctx, 5);  // Workflow
      break;
    case 'diamond':
      renderPolygon(node, ctx, 4, 45);  // Tool (rotated square)
      break;
  }
}

// Nonagon rendering with realm segments
function renderNonagon(node: FusionNode, ctx: CanvasRenderingContext2D) {
  const { x, y } = node.position;
  const size = node.size;
  
  // Draw 9 segments, colored by source database presence
  for (let i = 0; i < 9; i++) {
    const startAngle = (i * 40 - 90) * Math.PI / 180;
    const endAngle = ((i + 1) * 40 - 90) * Math.PI / 180;
    
    ctx.beginPath();
    ctx.moveTo(x, y);
    ctx.arc(x, y, size / 2, startAngle, endAngle);
    ctx.closePath();
    
    // Color segment based on source presence
    const source = node.sources[i % node.sources.length];
    ctx.fillStyle = source ? getDatabaseColor(source.database) : '#333';
    ctx.fill();
    ctx.strokeStyle = '#00ffff';
    ctx.stroke();
  }
  
  // Center label
  ctx.fillStyle = '#fff';
  ctx.textAlign = 'center';
  ctx.fillText(node.label, x, y);
}
```

---

## 4. Forge Workspace (Workflow Engine)

### 4.1 Rust-Native Workflow Engine

```rust
/// Forge workflow definition
#[derive(Debug, Clone)]
pub struct Workflow {
    /// Workflow ID
    pub id: Uuid,
    
    /// Human-readable name
    pub name: String,
    
    /// Workflow nodes
    pub nodes: Vec<WorkflowNode>,
    
    /// Connections between nodes
    pub edges: Vec<WorkflowEdge>,
    
    /// Associated realm
    pub realm: Realm,
    
    /// Crystal tuning for execution timing
    pub crystal_tuning: RealmTuning,
    
    /// Execution state
    pub state: WorkflowState,
    
    /// Delta angle for workflow position
    pub delta_angle: DeltaAngle,
}

#[derive(Debug, Clone)]
pub struct WorkflowNode {
    pub id: Uuid,
    pub node_type: WorkflowNodeType,
    pub config: serde_json::Value,
    pub position: (f64, f64),
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum WorkflowNodeType {
    // Triggers
    Webhook { path: String, method: String },
    Schedule { cron: String },
    NatsSubscribe { subject: String },
    RealmGate { realm: Realm, threshold: f64 },
    
    // Database operations
    SupabaseQuery { query: String },
    SurrealQuery { query: String },
    SledGet { key_pattern: String },
    SledisCommand { command: String },
    
    // Transforms
    Filter { expression: String },
    Map { transform: String },
    TrivariteHash,
    NonagonFusion { fusion_method: FusionMethod },
    
    // AI/ML
    LlmPrompt { model: String, prompt_template: String },
    Embedding { model: String },
    Classify { model: String, labels: Vec<String> },
    
    // Crystal operations
    CrystalInject { realm: Realm },
    CrystalPropagate,
    RealmTranslate { from: Realm, to: Realm },
    
    // Motion (for kinetic realm)
    MotionCommand { command_type: String },
    KinematicUpdate,
    
    // Outputs
    NatsPublish { subject: String },
    HttpRequest { url: String, method: String },
    Alert { channel: String },
    
    // Code
    WasmModule { module_path: String, function: String },
    RustClosure { code: String },
}

#[derive(Debug, Clone)]
pub struct WorkflowEdge {
    pub from_node: Uuid,
    pub from_output: String,
    pub to_node: Uuid,
    pub to_input: String,
    pub transform: Option<String>,
}
```

### 4.2 Workflow Execution Engine

```rust
/// Rust-native workflow executor
pub struct ForgeExecutor {
    /// Active workflows
    pub workflows: HashMap<Uuid, Workflow>,
    
    /// Execution queue
    pub queue: VecDeque<ExecutionTask>,
    
    /// Crystal lattices per realm
    pub crystals: [TunedCrystal; 9],
    
    /// Motion controllers
    pub motion_controllers: HashMap<Uuid, UnifiedMotionController>,
    
    /// Database connections
    pub databases: DatabasePool,
    
    /// NATS connection
    pub nats: async_nats::Client,
    
    /// Metrics
    pub metrics: ExecutionMetrics,
}

impl ForgeExecutor {
    /// Execute a single workflow node
    pub async fn execute_node(
        &mut self,
        workflow_id: Uuid,
        node_id: Uuid,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, ForgeError> {
        let workflow = self.workflows.get(&workflow_id)
            .ok_or(ForgeError::WorkflowNotFound)?;
        
        let node = workflow.nodes.iter()
            .find(|n| n.id == node_id)
            .ok_or(ForgeError::NodeNotFound)?;
        
        let start = std::time::Instant::now();
        
        let result = match &node.node_type {
            WorkflowNodeType::SupabaseQuery { query } => {
                self.databases.supabase_query(query, &input).await?
            }
            WorkflowNodeType::SurrealQuery { query } => {
                self.databases.surreal_query(query, &input).await?
            }
            WorkflowNodeType::TrivariteHash => {
                self.compute_trivariate_hash(&input)?
            }
            WorkflowNodeType::NonagonFusion { fusion_method } => {
                self.detect_fusion(&input, *fusion_method).await?
            }
            WorkflowNodeType::CrystalInject { realm } => {
                self.inject_crystal(*realm, &input)?
            }
            WorkflowNodeType::MotionCommand { command_type } => {
                self.execute_motion(command_type, &input).await?
            }
            WorkflowNodeType::LlmPrompt { model, prompt_template } => {
                self.llm_completion(model, prompt_template, &input).await?
            }
            // ... other node types
            _ => input.clone(),
        };
        
        // Record metrics
        self.metrics.record_execution(
            workflow_id,
            node_id,
            start.elapsed(),
        );
        
        Ok(result)
    }
    
    /// Inject decision into realm crystal
    fn inject_crystal(
        &mut self,
        realm: Realm,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, ForgeError> {
        let delta_angle = DeltaAngle::from_json(input)?;
        let amplitude = input["amplitude"].as_f64().unwrap_or(1.0);
        let urgency = input["urgency"].as_f64().unwrap_or(0.5);
        
        self.crystals[realm as usize].inject(delta_angle, amplitude, urgency);
        
        Ok(json!({
            "status": "injected",
            "realm": realm as u8,
            "delta_angle": delta_angle.format(),
        }))
    }
}
```

---

## 5. Query Workspace (3-Pane Interface)

### 5.1 Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              QUERY WORKSPACE                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ RESULTS CANVAS (60% height) ────────────────────────────────────────┐   │
│  │                                                                       │   │
│  │   Query results as graph or table                                    │   │
│  │                                                                       │   │
│  │   ⬡ ──── ⬢ ──── ◇        │ id │ name  │ status │ realm │            │   │
│  │   │             │        ├────┼───────┼────────┼───────┤            │   │
│  │   └──── ⬡ ─────┘        │ 1  │ Alpha │ ACTIVE │ CYBER │            │   │
│  │                          │ 2  │ Beta  │ IDLE   │ ORBIT │            │   │
│  │   [Graph View]           [Table View]          [JSON View]           │   │
│  │                                                                       │   │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─ PANE 1 (33%) ──────┐ ┌─ PANE 2 (33%) ──────┐ ┌─ PANE 3 (33%) ──────┐   │
│  │ ┌─ SQL/Query ─────┐ │ │ ┌─ LLM Prompt ────┐ │ │ ┌─ CLI/REPL ──────┐ │   │
│  │ │                 │ │ │ │                 │ │ │ │                 │ │   │
│  │ │ SELECT *        │ │ │ │ Find all fusion │ │ │ │ > .realm cyber  │ │   │
│  │ │ FROM agents     │ │ │ │ nodes in CYBER  │ │ │ │ > .query agents │ │   │
│  │ │ WHERE realm     │ │ │ │ realm with      │ │ │ │ > .crystal      │ │   │
│  │ │   = 'CYBER'     │ │ │ │ score > 0.9     │ │ │ │   inject 0.5    │ │   │
│  │ │ AND fusion_     │ │ │ │ and suggest     │ │ │ │ > .motion       │ │   │
│  │ │   score > 0.9   │ │ │ │ correlations    │ │ │ │   stop          │ │   │
│  │ │                 │ │ │ │                 │ │ │ │                 │ │   │
│  │ └─────────────────┘ │ │ └─────────────────┘ │ │ └─────────────────┘ │   │
│  │ [▶ Run] [💾 Save]   │ │ [▶ Ask] [🔄 Refine] │ │ [History ▼]        │   │
│  │ Target: [SurrealDB▼]│ │ Model: [Claude▼]   │ │ Realm: [CYBER▼]   │   │
│  └─────────────────────┘ └─────────────────────┘ └─────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 CLI Commands

```rust
/// CLI command parser for Query workspace REPL
#[derive(Debug, Clone)]
pub enum CliCommand {
    // Realm commands
    RealmSet { realm: Realm },
    RealmStatus,
    RealmList,
    
    // Database commands
    DbSelect { database: DatabaseType },
    DbQuery { query: String },
    DbStatus,
    
    // Crystal commands
    CrystalInject { amplitude: f64, delta_angle: Option<DeltaAngle> },
    CrystalStatus,
    CrystalTick { count: u32 },
    
    // Motion commands
    MotionMoveTo { x: f64, y: f64, z: f64 },
    MotionStop,
    MotionStatus,
    
    // Nonagon/Fusion commands
    FusionDetect { threshold: f64 },
    FusionList,
    FusionInspect { id: String },
    
    // Graph commands
    GraphQuery { cypher: String },
    GraphVisualize,
    
    // Workflow commands
    WorkflowRun { name: String },
    WorkflowList,
    WorkflowStatus { id: String },
    
    // Agent commands
    AgentList,
    AgentSpawn { agent_type: String },
    AgentKill { id: String },
    
    // System commands
    Help,
    Clear,
    Exit,
}

impl CliCommand {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some(".realm") => match parts.get(1) {
                Some(realm_str) => Ok(Self::RealmSet { 
                    realm: Realm::from_str(realm_str)? 
                }),
                None => Ok(Self::RealmStatus),
            },
            Some(".db") => match parts.get(1) {
                Some("query") => Ok(Self::DbQuery { 
                    query: parts[2..].join(" ") 
                }),
                Some(db) => Ok(Self::DbSelect { 
                    database: DatabaseType::from_str(db)? 
                }),
                None => Ok(Self::DbStatus),
            },
            Some(".crystal") => match parts.get(1) {
                Some("inject") => Ok(Self::CrystalInject {
                    amplitude: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                    delta_angle: None,
                }),
                Some("tick") => Ok(Self::CrystalTick {
                    count: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1),
                }),
                _ => Ok(Self::CrystalStatus),
            },
            Some(".motion") => match parts.get(1) {
                Some("stop") => Ok(Self::MotionStop),
                Some("goto") => Ok(Self::MotionMoveTo {
                    x: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    y: parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    z: parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                }),
                _ => Ok(Self::MotionStatus),
            },
            Some(".fusion") => match parts.get(1) {
                Some("detect") => Ok(Self::FusionDetect {
                    threshold: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.8),
                }),
                Some("inspect") => Ok(Self::FusionInspect {
                    id: parts.get(2).unwrap_or(&"").to_string(),
                }),
                _ => Ok(Self::FusionList),
            },
            Some(".agent") => match parts.get(1) {
                Some("spawn") => Ok(Self::AgentSpawn {
                    agent_type: parts.get(2).unwrap_or(&"default").to_string(),
                }),
                Some("kill") => Ok(Self::AgentKill {
                    id: parts.get(2).unwrap_or(&"").to_string(),
                }),
                _ => Ok(Self::AgentList),
            },
            Some(".workflow") | Some(".wf") => match parts.get(1) {
                Some("run") => Ok(Self::WorkflowRun {
                    name: parts.get(2).unwrap_or(&"").to_string(),
                }),
                Some("status") => Ok(Self::WorkflowStatus {
                    id: parts.get(2).unwrap_or(&"").to_string(),
                }),
                _ => Ok(Self::WorkflowList),
            },
            Some(".help") | Some("?") => Ok(Self::Help),
            Some(".clear") => Ok(Self::Clear),
            Some(".exit") | Some(".quit") => Ok(Self::Exit),
            _ => Err(ParseError::UnknownCommand),
        }
    }
}
```

---

## 6. Agent Workspace

### 6.1 Agent Management Interface

```rust
/// Agent in the SYNAPTIX9 system
#[derive(Debug, Clone)]
pub struct Agent {
    /// Agent ID
    pub id: Uuid,
    
    /// Human-readable name
    pub name: String,
    
    /// Agent type/class
    pub agent_type: AgentType,
    
    /// Current operational realm
    pub realm: Realm,
    
    /// Kinematic state (for physical agents)
    pub kinematic_state: Option<KinematicState>,
    
    /// Nonagon analysis state
    pub nonagon: NonagonNode,
    
    /// Associated workflows
    pub workflows: Vec<Uuid>,
    
    /// Current status
    pub status: AgentStatus,
    
    /// Delta angle position
    pub delta_angle: DeltaAngle,
    
    /// Last heartbeat
    pub last_heartbeat: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum AgentType {
    /// Software agent (cyber realm)
    Software,
    /// Drone/UAV (kinetic/orbital)
    Drone,
    /// Ground robot (kinetic)
    Robot,
    /// Maritime vessel (maritime)
    Vessel,
    /// Satellite (orbital)
    Satellite,
    /// Sensor node (spectrum)
    Sensor,
    /// Human operator (cognitive)
    Human,
}

#[derive(Debug, Clone, Copy)]
pub enum AgentStatus {
    Idle,
    Active,
    Executing,
    Error,
    Offline,
}
```

### 6.2 Agent Canvas

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             AGENT WORKSPACE                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ AGENT LIST ─────────────────────────────────────────────────────────┐   │
│  │ ● Alpha-1  │ Software │ CYBER    │ ACTIVE   │ Δ(0.5, 0.7, 0.3)      │   │
│  │ ● Beta-2   │ Drone    │ ORBITAL  │ ACTIVE   │ Δ(0.2, 0.4, 0.8)      │   │
│  │ ○ Gamma-3  │ Robot    │ KINETIC  │ IDLE     │ Δ(0.8, 0.1, 0.5)      │   │
│  │ ◐ Delta-4  │ Vessel   │ MARITIME │ EXECUTING│ Δ(0.3, 0.9, 0.2)      │   │
│  │ ✕ Echo-5   │ Sensor   │ SPECTRUM │ ERROR    │ Δ(0.6, 0.5, 0.7)      │   │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─ AGENT DETAIL ───────────────────────────────────────────────────────┐   │
│  │                                                                       │   │
│  │  Agent: Alpha-1                     Status: ● ACTIVE                 │   │
│  │  Type: Software                     Realm: CYBER                     │   │
│  │  Created: 2025-12-06 08:00:00      Uptime: 4h 32m                   │   │
│  │                                                                       │   │
│  │  ┌─ NONAGON ANALYSIS ──────────────────────────────────────────────┐ │   │
│  │  │                                                                  │ │   │
│  │  │         A₀(0.8)                                                 │ │   │
│  │  │        /       \                                                │ │   │
│  │  │    A₈(0.6)    A₁(0.7)                                          │ │   │
│  │  │      |           |                                              │ │   │
│  │  │    A₇(0.5)    A₂(0.9)    Center: 0.72                          │ │   │
│  │  │      |           |       Coverage: 89%                          │ │   │
│  │  │    A₆(0.7)    A₃(0.8)    Balance: 0.85                         │ │   │
│  │  │        \       /                                                │ │   │
│  │  │         A₅(0.6)                                                 │ │   │
│  │  │            |                                                     │ │   │
│  │  │         A₄(0.7)                                                 │ │   │
│  │  │                                                                  │ │   │
│  │  └──────────────────────────────────────────────────────────────────┘ │   │
│  │                                                                       │   │
│  │  [▶ Execute] [⏸ Pause] [⏹ Stop] [🔄 Restart] [🗑 Terminate]          │   │
│  │                                                                       │   │
│  └───────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Horizon Tabs (Database Context)

### 7.1 Database Connection Status

```rust
/// Database connection state
#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    /// Database type
    pub db_type: DatabaseType,
    
    /// Display name
    pub name: String,
    
    /// Connection status
    pub status: ConnectionStatus,
    
    /// Brand color
    pub color: String,
    
    /// Connection details
    pub host: String,
    pub port: u16,
    
    /// Metrics
    pub latency_ms: f64,
    pub last_ping: u64,
    pub record_count: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Syncing,
    Error,
}

impl DatabaseConnection {
    pub fn defaults() -> Vec<Self> {
        vec![
            Self {
                db_type: DatabaseType::Supabase,
                name: "Supabase".into(),
                status: ConnectionStatus::Connected,
                color: "#3ecf8e".into(),
                host: "db.xxxxx.supabase.co".into(),
                port: 5432,
                latency_ms: 23.0,
                last_ping: 0,
                record_count: 0,
            },
            Self {
                db_type: DatabaseType::SurrealDB,
                name: "SurrealDB".into(),
                status: ConnectionStatus::Connected,
                color: "#ff00a0".into(),
                host: "localhost".into(),
                port: 18019,
                latency_ms: 1.0,
                last_ping: 0,
                record_count: 0,
            },
            Self {
                db_type: DatabaseType::Sled,
                name: "Sled".into(),
                status: ConnectionStatus::Connected,
                color: "#ff6b35".into(),
                host: "/var/sx9/sled".into(),
                port: 0,
                latency_ms: 0.1,
                last_ping: 0,
                record_count: 0,
            },
            Self {
                db_type: DatabaseType::Sledis,
                name: "Sledis".into(),
                status: ConnectionStatus::Disconnected,
                color: "#ff9500".into(),
                host: "localhost".into(),
                port: 18401,
                latency_ms: 0.0,
                last_ping: 0,
                record_count: 0,
            },
            Self {
                db_type: DatabaseType::NATS,
                name: "NATS".into(),
                status: ConnectionStatus::Syncing,
                color: "#4222ff".into(),
                host: "localhost".into(),
                port: 18020,
                latency_ms: 0.5,
                last_ping: 0,
                record_count: 0,
            },
        ]
    }
}
```

### 7.2 Fusion Tab (Virtual Database)

The **Fusion** tab is a virtual database representing cross-database correlations:

```rust
/// Fusion "database" - virtual view of correlated entities
pub struct FusionDatabase {
    /// All fusion nodes
    pub nodes: Vec<FusionNode>,
    
    /// Cross-database relationships
    pub relationships: Vec<FusionRelationship>,
    
    /// Fusion detection settings
    pub detection_config: FusionConfig,
    
    /// Status
    pub status: ConnectionStatus,
    
    /// Color (cyan)
    pub color: String,
}

impl FusionDatabase {
    /// Query fusion nodes
    pub fn query(&self, filter: FusionFilter) -> Vec<&FusionNode> {
        self.nodes.iter()
            .filter(|n| filter.matches(n))
            .collect()
    }
    
    /// Detect new fusions across databases
    pub async fn detect_fusions(
        &mut self,
        databases: &DatabasePool,
        method: FusionMethod,
        threshold: f64,
    ) -> Vec<FusionNode> {
        // Implementation depends on method
        match method {
            FusionMethod::Hash => self.detect_by_hash(databases).await,
            FusionMethod::Semantic => self.detect_by_semantic(databases, threshold).await,
            FusionMethod::GraphStructure => self.detect_by_structure(databases).await,
            FusionMethod::TemporalCorrelation => self.detect_by_temporal(databases).await,
            _ => Vec::new(),
        }
    }
}
```

---

## 8. Tech Stack

### 8.1 Frontend

```
Next.js 14 (App Router)
├── TypeScript 5.5
├── Tailwind CSS (dark theme only)
├── D3.js (graph visualization)
├── React Flow (workflow canvas)
├── shadcn/ui (components)
├── Zustand (state management)
└── Monaco Editor (code panes)
```

### 8.2 Backend Integration

```
sx9-atlas-bus (Rust IPC)
├── NATS JetStream (messaging)
├── Supabase (PostgreSQL)
├── SurrealDB (multi-model)
├── Sled (embedded KV)
├── Sledis (Redis-compatible)
└── Crystal/Kinematics (RFC-9303)
```

### 8.3 Component Structure

```
app/
├── layout.tsx                 # Main layout with glyph rail
├── page.tsx                   # Default to Command workspace
├── command/page.tsx           # Realm 0: C2 dashboard
├── graph/page.tsx             # Realm 1: Graph browser
├── forge/page.tsx             # Realm 2: Workflow canvas
├── intel/page.tsx             # Realm 3: Intelligence analysis
├── orbital/page.tsx           # Realm 4: Space assets
├── maritime/page.tsx          # Realm 5: Naval ops
├── tunnel/page.tsx            # Realm 6: Underground
├── spectrum/page.tsx          # Realm 7: EMS analysis
├── timeline/page.tsx          # Realm 8: Temporal view
├── connections/page.tsx       # Database connections
└── settings/page.tsx          # Settings

components/
├── layout/
│   ├── glyph-rail.tsx
│   ├── horizon-tabs.tsx
│   ├── realm-indicator.tsx
│   └── status-bar.tsx
├── graph/
│   ├── graph-canvas.tsx
│   ├── fusion-node.tsx        # Nonagon renderer
│   └── shapes/
│       ├── nonagon.tsx        # 9-sided
│       ├── octagon.tsx        # 8-sided
│       ├── heptagon.tsx       # 7-sided
│       ├── hexagon.tsx        # 6-sided
│       └── pentagon.tsx       # 5-sided
├── query/
│   ├── sql-pane.tsx
│   ├── llm-pane.tsx
│   ├── cli-pane.tsx
│   └── results-canvas.tsx
├── forge/
│   ├── workflow-canvas.tsx
│   ├── node-palette.tsx
│   └── execution-log.tsx
├── agents/
│   ├── agent-list.tsx
│   ├── agent-detail.tsx
│   └── nonagon-viz.tsx
└── common/
    ├── delta-angle-display.tsx
    ├── realm-badge.tsx
    └── crystal-status.tsx
```

---

## 9. Unicode Allocation

| Range | Symbol | Component | Description |
|-------|--------|-----------|-------------|
| U+E770 | 🝰 | WS-CMD | Command workspace |
| U+E771 | 🝱 | WS-GRAPH | Graph workspace |
| U+E772 | 🝲 | WS-FORGE | Forge workspace |
| U+E773 | 🝳 | WS-INTEL | Intel workspace |
| U+E774 | 🝴 | WS-ORBIT | Orbital workspace |
| U+E775 | 🝵 | WS-MARIT | Maritime workspace |
| U+E776 | 🝶 | WS-TUNNEL | Tunnel workspace |
| U+E777 | 🝷 | WS-SPEC | Spectrum workspace |
| U+E778 | 🝸 | WS-TIME | Timeline workspace |
| U+E780 | 🞀 | DB-SUPA | Supabase connected |
| U+E781 | 🞁 | DB-SURR | SurrealDB connected |
| U+E782 | 🞂 | DB-SLED | Sled connected |
| U+E783 | 🞃 | DB-SLEDIS | Sledis connected |
| U+E784 | 🞄 | DB-NATS | NATS connected |
| U+E785 | 🞅 | DB-FUSION | Fusion active |

---

## 10. Implementation Requirements

### 10.1 MUST Requirements

1. Workbench MUST support all 9 realm-aligned workspaces
2. Graph workspace MUST render Nonagon fusion nodes correctly
3. Forge workflows MUST execute via Rust-native engine
4. Query workspace MUST provide 3-pane interface
5. All workspaces MUST display current realm and crystal tuning
6. Delta angles MUST be visible in status bar

### 10.2 SHOULD Requirements

1. Glyph rail SHOULD collapse to 42px
2. Horizon tabs SHOULD show real-time connection status
3. Fusion detection SHOULD support multiple methods
4. CLI commands SHOULD auto-complete

### 10.3 MAY Requirements

1. Workspaces MAY be customized per user
2. Additional database types MAY be added
3. Custom workflow nodes MAY be defined via WASM

---

## 11. References

- RFC-9302: Nonagon Analytic Node
- RFC-9303: Crystal Realm Tunings & Unified Kinematics
- RFC-9301: Thyristor, Crystal, and Ring Bus
- RFC-9001: Trivariate Hashing System

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2025-12-06 | Initial draft |

---

*End of RFC-9304*
