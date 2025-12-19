# SX9 Enterprise Knowledge Architecture
## RFC Access Layer + Solutions Development System

**Purpose:** Make enterprise knowledge (RFCs, code, patterns) accessible to:
1. **AI Agents** - During development/planning sessions
2. **SX9 Clients** - Solutions delivery and implementation
3. **Engineering Funnel** - Code → Product pipeline

---

## The Problem

```
Current State:
├── 50+ RFCs scattered across repos
├── Extracted JSON data (scholarly refs, test harnesses)
├── Knowledge graph (Neo4j/Cypher ready)
├── Millions of lines of code
├── Multiple IDEs (Antigravity, Cursor, VS Code, Claude Code)
└── No unified access layer
```

**Result:** Every agent session starts cold. Context is lost. Knowledge is duplicated.

---

## The Solution: Layered Knowledge Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SX9 ENTERPRISE KNOWLEDGE LAYER                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ACCESS INTERFACES                                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  MCP Server │  │  REST API   │  │  CLI Tool   │  │ Prompt Forge│        │
│  │  (Agents)   │  │  (Clients)  │  │  (Eng)      │  │  (Planning) │        │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘        │
│         │                │                │                │                │
│         └────────────────┴────────────────┴────────────────┘                │
│                                   │                                          │
│                                   ▼                                          │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                     KNOWLEDGE SERVICE LAYER                            │  │
│  │                                                                        │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │  │
│  │  │ RFC Service │  │Code Service │  │Pattern Svc  │  │Solution Svc │   │  │
│  │  │             │  │             │  │             │  │             │   │  │
│  │  │ - search    │  │ - crate map │  │ - templates │  │ - client    │   │  │
│  │  │ - get       │  │ - deps      │  │ - examples  │  │ - delivery  │   │  │
│  │  │ - deps      │  │ - quality   │  │ - anti-pat  │  │ - tracking  │   │  │
│  │  │ - comply    │  │ - funnel    │  │ - best-prac │  │ - success   │   │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │  │
│  │                                                                        │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                   │                                          │
│         ┌─────────────────────────┼─────────────────────────┐               │
│         ▼                         ▼                         ▼               │
│  ┌─────────────┐          ┌─────────────┐          ┌─────────────┐         │
│  │   SLEDIS    │          │  SUPABASE   │          │  CHROMADB   │         │
│  │   (Cache)   │          │  (Store)    │          │  (Vector)   │         │
│  │             │          │             │          │             │         │
│  │ - Hot RFCs  │          │ - RFC text  │          │ - Embeddings│         │
│  │ - Session   │          │ - Metadata  │          │ - Semantic  │         │
│  │ - Prompts   │          │ - Graph     │          │ - Similar   │         │
│  └─────────────┘          └─────────────┘          └─────────────┘         │
│                                   │                                          │
│                                   ▼                                          │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                    CANONICAL SOURCE OF TRUTH                           │  │
│  │                                                                        │  │
│  │  /Users/cp5337/Developer/sx9/01-rfc/                                   │  │
│  │  ├── 9000-core/           (Foundational specs)                         │  │
│  │  ├── 9100-integration/    (System integration)                         │  │
│  │  ├── 9300-cognitive/      (AI/ML patterns)                             │  │
│  │  ├── 9400-application/    (User-facing apps)                           │  │
│  │  ├── 9500-platform/       (Platform services)                          │  │
│  │  └── 9800-operational/    (Ops/execution)                              │  │
│  │                                                                        │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Component 1: RFC Registry (Source of Truth)

### Canonical Location
```
/Users/cp5337/Developer/sx9/01-rfc/
├── RFC-INDEX.yaml           # Machine-readable index
├── RFC-REGISTRY.md          # Human-readable overview
│
├── 9000-core/               # Foundation Layer
│   ├── RFC-9000-Agnostic-Core.md
│   ├── RFC-9001-Trivariate-Hashing.md
│   ├── RFC-9002-Unicode-Routing.md
│   ├── RFC-9003-Operation-Classifier.md
│   ├── RFC-9004-Deterministic-Routing.md
│   ├── RFC-9005-Unified-Schema.md
│   ├── RFC-9006-Secure-Transport-Profiles.md
│   ├── RFC-9007-Obfuscation-Biometric-Honeypot.md
│   ├── RFC-9008-Ephemeral-Engagement-Rooms.md
│   ├── RFC-9009-Quantum-Cryptographic-Architecture.md
│   ├── RFC-9020-HD4-Framework.md
│   ├── RFC-9116-APECS-Legion-Bridge-ECS.md
│   └── RFC-93XX-*.md        # Crystal/Nonagon specs
│
├── 9100-integration/        # Integration Layer
│   ├── RFC-9030-Unified-Linear-Agent-Infrastructure.md
│   ├── RFC-9100-Dual-Trivariate-PTCC-Integration.md
│   ├── RFC-9101-Smart-Crate-System.md
│   ├── RFC-9102-Executable-Document-Framework.md
│   ├── RFC-9105-SPIRES-Extraction.md
│   ├── RFC-9107-Unified-Agent-Infrastructure.md
│   ├── RFC-9108-Thalmic-Filter-Model-Registry.md
│   ├── RFC-9109-Plasma-Defender.md
│   ├── RFC-9110-SX9-Lisp-Interpreter.md
│   ├── RFC-9112-Deterministic-Prompt-Engineering.md
│   ├── RFC-9113-TOML-Executable-Document-Specification.md
│   ├── RFC-9114-SX9-Gateway-Neural-Retrofit.md
│   └── RFC-9117-Tool-Response-Block.md
│
├── 9300-cognitive/          # Cognitive/AI Layer
│   ├── RFC-9023-GLAF-Matroid-Convergence.md
│   ├── RFC-9024-H2-Convergence-Contract.md
│   └── RFC-9025-Node-Interview-Schema.md
│
├── 9400-application/        # Application Layer
│   ├── RFC-9150-GIS-UI.md
│   ├── RFC-9151-Patrolmans-Notebook.md
│   └── RFC-9304B-SX9-Workbench.md
│
├── 9500-platform/           # Platform Layer
│   └── RFC-9200-Data-Analytics-Workbench.md
│
├── 9800-operational/        # Operational Layer
│   ├── RFC-9130-L2-NATS-Kali-Execution-Platform.md
│   ├── RFC-9131-Dynamic-Resource-Escalation.md
│   └── RFC-9876-Layer-Two-Unicode-Orchestration.md
│
└── _extracted/              # Pre-processed data
    ├── extractions/         # JSON structured data
    ├── scholarly_refs/      # Academic references
    ├── test_harnesses/      # Generated tests
    └── knowledge_graph/     # Neo4j/Cypher data
```

### RFC Index Schema (RFC-INDEX.yaml)
```yaml
version: "1.0"
last_updated: "2025-12-18"
canonical_path: "/Users/cp5337/Developer/sx9/01-rfc"

categories:
  core:
    range: "9000-9099"
    description: "Foundational specifications"
    rfcs:
      - id: "RFC-9000"
        title: "Agnostic Core"
        status: "active"
        dependencies: []
        implementations: ["sx9-foundation-core"]
        tags: ["architecture", "foundation"]
        
      - id: "RFC-9001"
        title: "Trivariate Hashing"
        status: "active"
        dependencies: ["RFC-9000"]
        implementations: ["sx9-hash-engine"]
        tags: ["cryptography", "hashing", "blake3"]
        
      - id: "RFC-9002"
        title: "Unicode Routing"
        status: "active"
        dependencies: ["RFC-9001"]
        implementations: ["sx9-router"]
        tags: ["routing", "unicode", "glaf"]
        
      # ... more entries

  integration:
    range: "9100-9199"
    description: "System integration specifications"
    rfcs:
      - id: "RFC-9107"
        title: "Unified Agent Infrastructure"
        status: "active"
        dependencies: ["RFC-9000", "RFC-9030"]
        implementations: ["sx9-agent-harness"]
        tags: ["agents", "llm", "personas"]
        
      - id: "RFC-9112"
        title: "Deterministic Prompt Engineering"
        status: "active"
        dependencies: ["RFC-9107"]
        implementations: ["sx9-prompt-forge"]
        tags: ["prompts", "deterministic", "engineering"]

# Cross-reference matrix
dependency_graph:
  RFC-9001: ["RFC-9000"]
  RFC-9002: ["RFC-9001"]
  RFC-9003: ["RFC-9002"]
  RFC-9004: ["RFC-9003"]
  RFC-9005: ["RFC-9000", "RFC-9001", "RFC-9002", "RFC-9003", "RFC-9004"]
  # ... builds full DAG
```

---

## Component 2: RFC MCP Server

**Purpose:** Make RFCs queryable by any AI agent via MCP protocol.

### Tools Exposed
```typescript
// sx9-rfc-mcp-server/src/tools.ts

interface RFCTools {
  // Search RFCs
  rfc_search: {
    query: string;           // Semantic search query
    category?: string;       // Filter by category
    tags?: string[];         // Filter by tags
    limit?: number;          // Max results
  } => RFCSearchResult[];
  
  // Get specific RFC
  rfc_get: {
    id: string;              // RFC-9001, RFC-9112, etc.
    include_extracted?: boolean;  // Include JSON extraction
    include_deps?: boolean;  // Include dependencies
  } => RFCDocument;
  
  // List RFCs by category
  rfc_list: {
    category: string;        // core, integration, cognitive, etc.
    status?: string;         // active, deprecated, draft
  } => RFCSummary[];
  
  // Get dependency graph
  rfc_dependencies: {
    id: string;              // Starting RFC
    direction?: "up" | "down" | "both";  // Traverse direction
    depth?: number;          // Max depth
  } => DependencyGraph;
  
  // Check implementation compliance
  rfc_compliance: {
    crate_path: string;      // Path to Rust crate
    rfc_id: string;          // RFC to check against
  } => ComplianceReport;
  
  // Get related code
  rfc_implementations: {
    id: string;              // RFC ID
  } => Implementation[];
}
```

### MCP Server Configuration
```json
{
  "mcpServers": {
    "sx9-rfc": {
      "command": "node",
      "args": ["/Users/cp5337/Developer/sx9/tools/mcp-servers/rfc-server/index.js"],
      "env": {
        "RFC_INDEX_PATH": "/Users/cp5337/Developer/sx9/01-rfc/RFC-INDEX.yaml",
        "CHROMA_HOST": "localhost:8000",
        "SLEDIS_HOST": "localhost:6379"
      }
    }
  }
}
```

---

## Component 3: Prompt Forge Integration

### RFC-Aware Prompt Generation

When generating prompts, Prompt Forge should:

1. **Auto-inject relevant RFCs** based on task type
2. **Enforce compliance** with RFC constraints
3. **Reference implementations** from RFC metadata

```yaml
# Example: Prompt Forge output with RFC context
mission:
  type: "BUILD"
  persona: "FORGE"
  harness: "build"
  
  # AUTO-INJECTED based on task analysis
  rfc_context:
    primary:
      - id: "RFC-9112"
        title: "Deterministic Prompt Engineering"
        relevance: "Direct - building prompt system"
        
    supporting:
      - id: "RFC-9107"
        title: "Unified Agent Infrastructure"
        relevance: "Agent harness patterns"
        
      - id: "RFC-9101"
        title: "Smart Crate System"
        relevance: "Module organization"
        
    constraints_from_rfcs:
      - "Max 300 lines per module (RFC-9101 §4.2)"
      - "Use NVNN comments for state (RFC-9112 §3.1)"
      - "Persona must map to voice_id (RFC-9107 §7)"
      
  context_loading:
    sequence:
      - tool: "rfc_get"
        params: { id: "RFC-9112", include_extracted: true }
      - tool: "rfc_implementations"
        params: { id: "RFC-9112" }
      - tool: "linear:list_issues"
        params: { labels: ["rfc:9112"] }
```

### Prompt Type → RFC Mapping
```yaml
prompt_type_rfc_mapping:
  BUILD:
    primary: ["RFC-9101"]  # Smart Crate System
    supporting: ["RFC-9000", "RFC-9112"]
    
  SECURITY_AUDIT:
    primary: ["RFC-9006", "RFC-9007", "RFC-9109"]
    supporting: ["RFC-9023"]
    
  THREAT_EMULATION:
    primary: ["RFC-9020", "RFC-9130"]  # HD4, L2 Execution
    supporting: ["RFC-9876", "RFC-9011"]
    
  RFC_IMPLEMENTATION:
    primary: ["${target_rfc}"]
    supporting: ["RFC-9101", "RFC-9112"]
    
  INTEGRATION:
    primary: ["RFC-9100", "RFC-9114"]
    supporting: ["RFC-9005"]
    
  PLANNING:
    primary: ["RFC-9200"]
    supporting: ["RFC-9112", "RFC-9107"]
```

---

## Component 4: Solutions Delivery System

### For SX9 Clients

```
┌─────────────────────────────────────────────────────────────────┐
│                 SX9 SOLUTIONS DELIVERY PIPELINE                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. REQUIREMENTS INTAKE                                          │
│     ┌─────────────────────────────────────────────────────────┐ │
│     │ Client Need → Map to RFC Categories → Gap Analysis      │ │
│     │                                                         │ │
│     │ "Need threat detection" → RFC-9020 (HD4), RFC-9011      │ │
│     │ "Need secure comms"    → RFC-9006, RFC-9008             │ │
│     │ "Need GIS dashboard"   → RFC-9150, RFC-9151             │ │
│     └─────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              ▼                                   │
│  2. SOLUTION DESIGN                                              │
│     ┌─────────────────────────────────────────────────────────┐ │
│     │ RFC-Based Architecture → Component Selection            │ │
│     │                                                         │ │
│     │ Auto-generate:                                          │ │
│     │ - Architecture diagram (from RFC deps)                  │ │
│     │ - Component list (from RFC implementations)             │ │
│     │ - Integration points (from RFC-9100)                    │ │
│     └─────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              ▼                                   │
│  3. IMPLEMENTATION                                               │
│     ┌─────────────────────────────────────────────────────────┐ │
│     │ Prompt Forge → Agent Harness → Linear Tracking          │ │
│     │                                                         │ │
│     │ Each task:                                              │ │
│     │ - Injected with relevant RFCs                           │ │
│     │ - Compliance-checked against specs                      │ │
│     │ - Tracked in Linear with RFC labels                     │ │
│     └─────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              ▼                                   │
│  4. DELIVERY                                                     │
│     ┌─────────────────────────────────────────────────────────┐ │
│     │ Solution Package → Deployment → Success Metrics         │ │
│     │                                                         │ │
│     │ Includes:                                               │ │
│     │ - Deployed components                                   │ │
│     │ - RFC compliance report                                 │ │
│     │ - Client-specific documentation                         │ │
│     │ - Success criteria tracking                             │ │
│     └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component 5: Engineering Product Funnel

### Code → Product Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│              ENGINEERING PRODUCT FUNNEL                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  STAGE 1: CODE INVENTORY                                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Input: Millions of lines across repos                       ││
│  │                                                             ││
│  │ Actions:                                                    ││
│  │ - Scan all crates/modules                                   ││
│  │ - Map to RFC implementations                                ││
│  │ - Identify orphan code (no RFC coverage)                    ││
│  │ - Quality scoring (tests, docs, compliance)                 ││
│  │                                                             ││
│  │ Output: Code Registry with RFC linkage                      ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                              ▼                                   │
│  STAGE 2: PRODUCT MAPPING                                        │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Input: Code Registry + RFC Index                            ││
│  │                                                             ││
│  │ Actions:                                                    ││
│  │ - Group by product vertical                                 ││
│  │ - Identify reusable components                              ││
│  │ - Gap analysis (needed but missing)                         ││
│  │ - Priority scoring                                          ││
│  │                                                             ││
│  │ Products:                                                   ││
│  │ - CTAS (Threat Analysis)    → RFC-9020, 9011, 9130          ││
│  │ - GIS Platform              → RFC-9150, 9151                ││
│  │ - Agent Harness (SX9 Ops)   → RFC-9107, 9112                ││
│  │ - Secure Comms              → RFC-9006, 9008, 9009          ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                              ▼                                   │
│  STAGE 3: PRODUCTIZATION                                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Input: Product Mapping                                      ││
│  │                                                             ││
│  │ Actions:                                                    ││
│  │ - Package as deployable units                               ││
│  │ - Generate documentation                                    ││
│  │ - Create demo environments                                  ││
│  │ - Build pricing models                                      ││
│  │                                                             ││
│  │ Output: Shippable Products                                  ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                              ▼                                   │
│  STAGE 4: MARKET DELIVERY                                        │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ - SDVOSB contract vehicles                                  ││
│  │ - Direct enterprise sales                                   ││
│  │ - Partner integrations                                      ││
│  │ - SaaS offerings                                            ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

---

## Implementation Priority

### Phase 1: Foundation (This Week)
1. **Consolidate RFC location** → Single canonical path
2. **Create RFC-INDEX.yaml** → Machine-readable registry
3. **Build RFC MCP Server** → Basic search/get/list

### Phase 2: Integration (Next Week)
4. **Prompt Forge RFC injection** → Auto-context loading
5. **Sledis cache layer** → Hot RFC access
6. **Linear RFC labels** → Track work by RFC

### Phase 3: Scale (Following Weeks)
7. **ChromaDB embeddings** → Semantic search
8. **Code registry** → Map crates to RFCs
9. **Solutions templates** → Client delivery packages

---

## Key Files to Create

| File | Purpose |
|------|---------|
| `/sx9/01-rfc/RFC-INDEX.yaml` | Machine-readable RFC registry |
| `/sx9/tools/mcp-servers/rfc-server/` | RFC MCP server for agents |
| `/sx9/.sx9/rfc-cache.json` | Sledis-backed hot cache |
| `/sx9/tools/rfc-scanner/` | Code → RFC compliance checker |

---

## Integration with Current Work

This RFC Access Layer becomes **Context Source #1** in the Prompt Forge:

```yaml
context_loading:
  sequence:
    1_rfc_context:
      tool: "sx9-rfc:rfc_search"
      params: { query: "${task_description}", limit: 3 }
      
    2_memory:
      tool: "conversation_search"
      params: { query: "${task_keywords}" }
      
    3_linear:
      tool: "linear:list_issues"
      params: { labels: ["rfc:${primary_rfc}"] }
      
    4_drive:
      tool: "google_drive_search"
      params: { query: "${project_name} architecture" }
      
    5_filesystem:
      tool: "filesystem:search_files"
      params: { pattern: "${rfc_implementation}" }
```

**Every agent session starts RFC-informed.**
