# CTAS Tasks Integration Plan
## Supabase ACID + Slot Graph + Hourglass-Bernoulli

**Date:** December 2025  
**Status:** Integration Plan  
**Goal:** Migrate CTAS tasks to Supabase (ACID), integrate with Slot Graph hash/unicode routing, align with Hourglass-Bernoulli architecture

---

## Overview

**CTAS Tasks Hierarchy:**
```
Task > Skill > Tool Chain > Tool
```

**Execution Paths:**
- Playbooks (DSL)
- Cognitive response (GNN, ANN, LLM)
- Operator action
- Agents

**Goal:** Force multiplication, multi-tasking, automation

---

## 1. CTAS Tasks Schema (Supabase ACID)

### 1.1 Tasks Table

```sql
-- ============================================================================
-- RFC-9005 UNIFIED SCHEMA — CTAS Tasks Table
-- ============================================================================
-- Compliance: RFC-9001, RFC-9002, RFC-9003, RFC-9005, RFC-9026

CREATE TABLE ctas_tasks (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash Identity
    trivariate_hash TEXT NOT NULL,          -- Format: SCH-CUID-UUID
    sch_hash TEXT,                          -- Murmur3-128 (24 chars)
    cuid TEXT,                              -- Base96 (16 chars)
    sx9_uuid TEXT,                          -- Immutable lineage anchor
    
    -- RFC-9002: Unicode Addressing
    unicode_address TEXT,                   -- U+E000-EFFF
    unicode_class CHAR(1),                  -- A-H class designation
    
    -- Task Identity
    task_id TEXT NOT NULL UNIQUE,           -- e.g., "uuid-001-001-001"
    task_name TEXT NOT NULL,
    task_description TEXT,
    
    -- RFC-9026: Hourglass-Bernoulli Classification
    primitive_type TEXT NOT NULL CHECK (primitive_type IN (
        'Concept', 'Actor', 'Object', 'Event', 'Attribute', 'Unclassified'
    )),
    ptcc_primitive_id INTEGER,              -- 0-31 (32 Universal Primitives)
    ptcc_primitive_name TEXT,               -- e.g., "CREATE", "READ", "UPDATE"
    
    -- HD4 Phase (RFC-9020)
    hd4_phase TEXT NOT NULL CHECK (hd4_phase IN (
        'Hunt', 'Detect', 'Disrupt', 'Disable', 'Dominate'
    )),
    
    -- Domain Classification
    domain TEXT CHECK (domain IN (
        'Cyber', 'Geographical', 'Space', 'Maritime', 'Fusion'
    )),
    
    -- Form Type (1n/2n)
    form_type TEXT CHECK (form_type IN ('1n', '2n')),
    
    -- Skill Mapping
    required_skills JSONB DEFAULT '[]',      -- Array of skill IDs
    skill_categories JSONB DEFAULT '[]',    -- e.g., ["reconnaissance", "exploitation"]
    
    -- Tool Chain Mapping
    tool_chain_id TEXT,                     -- Reference to tool chain
    tool_chain_sequence INTEGER,            -- Order in chain
    
    -- Tool Mapping
    tools JSONB DEFAULT '[]',               -- Array of tool IDs/names
    
    -- RFC-9003: Operation Classification
    operation_class TEXT CHECK (operation_class IN (
        'intelligence', 'defensive', 'offensive', 'administrative'
    )),
    escalation_tier INTEGER DEFAULT 1 CHECK (escalation_tier BETWEEN 1 AND 7),
    
    -- RFC-9026: Hourglass-Bernoulli
    ideation_zone BOOLEAN DEFAULT false,    -- Wide: LLM planning
    bernoulli_zone BOOLEAN DEFAULT false,   -- Narrow: Deterministic execution
    management_zone BOOLEAN DEFAULT false,   -- Wide: Analysis/reporting
    
    -- Execution Paths
    execution_paths JSONB DEFAULT '{
        "playbooks": [],
        "cognitive": [],
        "operator": false,
        "agents": []
    }',
    
    -- Slot Graph Integration
    slot_graph_node_id TEXT,                -- Slot Graph node reference
    hash_routing_enabled BOOLEAN DEFAULT true,
    unicode_routing_enabled BOOLEAN DEFAULT true,
    
    -- Protection Suite Lineage
    protection_suite_id TEXT,                -- Lineage to protection suite
    protection_suite_version TEXT,
    
    -- State Management
    current_state TEXT DEFAULT 'draft' CHECK (current_state IN (
        'draft', 'active', 'deprecated', 'archived'
    )),
    
    -- RFC Compliance Tracking
    rfc_version TEXT DEFAULT '9001-9002-9003-9005-9026',
    schema_version TEXT DEFAULT '1.1',
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance Indexes
CREATE UNIQUE INDEX idx_tasks_task_id ON ctas_tasks (task_id);
CREATE UNIQUE INDEX idx_tasks_trivariate ON ctas_tasks (trivariate_hash);
CREATE INDEX idx_tasks_sch ON ctas_tasks (sch_hash);
CREATE INDEX idx_tasks_cuid ON ctas_tasks (cuid);
CREATE INDEX idx_tasks_unicode ON ctas_tasks (unicode_address);
CREATE INDEX idx_tasks_primitive ON ctas_tasks (ptcc_primitive_id);
CREATE INDEX idx_tasks_hd4_phase ON ctas_tasks (hd4_phase);
CREATE INDEX idx_tasks_domain ON ctas_tasks (domain);
CREATE INDEX idx_tasks_skills ON ctas_tasks USING GIN (required_skills);
CREATE INDEX idx_tasks_tools ON ctas_tasks USING GIN (tools);
CREATE INDEX idx_tasks_slot_graph ON ctas_tasks (slot_graph_node_id);
CREATE INDEX idx_tasks_protection ON ctas_tasks (protection_suite_id);
```

### 1.2 Skills Table

```sql
CREATE TABLE ctas_skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT NOT NULL,
    sch_hash TEXT,
    cuid TEXT,
    
    -- Skill Identity
    skill_id TEXT NOT NULL UNIQUE,          -- e.g., "skill-recon-001"
    skill_name TEXT NOT NULL,
    skill_category TEXT NOT NULL,           -- e.g., "reconnaissance", "exploitation"
    
    -- PTCC Primitives
    ptcc_primitives JSONB DEFAULT '[]',     -- Array of primitive IDs
    
    -- Tool Mapping
    tools JSONB DEFAULT '[]',               -- Array of tool IDs/names
    
    -- HD4 Phase Applicability
    hd4_phases JSONB DEFAULT '[]',         -- Array of applicable phases
    
    -- State
    current_state TEXT DEFAULT 'active',
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_skills_skill_id ON ctas_skills (skill_id);
CREATE INDEX idx_skills_category ON ctas_skills (skill_category);
CREATE INDEX idx_skills_primitives ON ctas_skills USING GIN (ptcc_primitives);
```

### 1.3 Tool Chains Table

```sql
CREATE TABLE ctas_tool_chains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT NOT NULL,
    
    -- Tool Chain Identity
    tool_chain_id TEXT NOT NULL UNIQUE,
    tool_chain_name TEXT NOT NULL,
    tool_chain_description TEXT,
    
    -- Chain Sequence
    chain_sequence JSONB NOT NULL,          -- Ordered array of skill IDs
    
    -- HD4 Phase
    hd4_phase TEXT,
    
    -- Execution Path
    execution_path TEXT CHECK (execution_path IN (
        'playbook', 'cognitive', 'operator', 'agent'
    )),
    
    -- State
    current_state TEXT DEFAULT 'active',
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_tool_chains_chain_id ON ctas_tool_chains (tool_chain_id);
CREATE INDEX idx_tool_chains_hd4 ON ctas_tool_chains (hd4_phase);
```

---

## 2. Slot Graph Integration

### 2.1 Hash/Unicode Routing

**No data traverses graphs** - only hashes or unicode runes.

**Routing Flow:**
```
Task (trivariate hash)
    │
    ▼
Slot Graph (hash routing, no data)
    │
    ▼
Skill (unicode rune trigger)
    │
    ▼
Tool Chain (unicode rune sequence)
    │
    ▼
Tool (unicode rune execution)
```

### 2.2 Unicode Rune Mapping

**Task → Skill → Tool Chain → Tool:**

| Level | Unicode Range | Encoding |
|-------|---------------|----------|
| Task | U+E000-E0FF | Domain + HD4 phase |
| Skill | U+E100-E1FF | Skill category + PTCC primitive |
| Tool Chain | U+E200-E2FF | Chain sequence |
| Tool | U+EE00-EEFF | Tool trigger (nmap, masscan, etc.) |

### 2.3 Slot Graph Node Structure

```typescript
interface TaskSlotGraphNode {
    node_id: string;                    // Slot Graph node ID
    trivariate_hash: string;           // Task trivariate hash
    unicode_address: string;            // U+E000-E0FF
    skill_edges: string[];              // Connected skill node IDs
    tool_chain_edges: string[];         // Connected tool chain node IDs
    tool_edges: string[];               // Connected tool node IDs
    hash_routing: boolean;              // Enable hash routing
    unicode_routing: boolean;           // Enable unicode routing
}
```

---

## 3. Hourglass-Bernoulli Integration

### 3.1 Zone Classification

**Per RFC-9026:**

| Zone | Task Phase | LLM Usage | Latency | Cost |
|------|------------|-----------|---------|------|
| **Ideation** (Wide) | Planning, design | ✅ Large LLMs | Hours-Days | $$$$ |
| **Bernoulli** (Narrow) | Execution | ❌ NO LLMs | <50μs | $0.0000001 |
| **Management** (Wide) | Analysis, reporting | ✅ Large + Small LLMs | Seconds-Hours | $0.001-0.01 |

### 3.2 Task Zone Mapping

```sql
-- Update tasks with zone classification
UPDATE ctas_tasks
SET 
    ideation_zone = CASE 
        WHEN hd4_phase = 'Hunt' THEN true  -- Planning phase
        ELSE false
    END,
    bernoulli_zone = CASE
        WHEN hd4_phase IN ('Detect', 'Disrupt', 'Disable') THEN true  -- Execution
        ELSE false
    END,
    management_zone = CASE
        WHEN hd4_phase = 'Dominate' THEN true  -- Analysis/reporting
        ELSE false
    END;
```

### 3.3 32 Universal Primitives Mapping

**Per RFC-9026 §3.2:**

| Primitive Type | Count | PTCC Category | Example Primitives |
|----------------|-------|---------------|-------------------|
| Concept | 10 | Cognitive/Control | BRANCH, LOOP, CALL, RETURN |
| Actor | 12 | Coordination | COORDINATE, SYNCHRONIZE, SIGNAL, WAIT |
| Object | 28 | Data/Network | TRANSFORM, VALIDATE, CONNECT, ROUTE, FILTER |
| Event | 12 | State/Security | CHECKPOINT, SAVE, AUTHENTICATE, AUTHORIZE |
| Attribute | 7 | Security | ENCRYPT, DECRYPT, LOCK, UNLOCK |
| Unclassified | 97 | CRUD/Resource | CREATE, READ, UPDATE, DELETE, ALLOCATE |

---

## 4. Migration Plan

### 4.1 Phase 1: Schema Deployment

```bash
# 1. Deploy unified schema to Supabase
supabase db push

# 2. Verify table creation
supabase db diff
```

### 4.2 Phase 2: CSV Import

```python
# Import script: ctas_tasks_import.py
import csv
import psycopg2
from ctas7_foundation_core import TrivariateHashEngine

def import_ctas_tasks(csv_path: str, db_conn):
    hash_engine = TrivariateHashEngine()
    
    with open(csv_path, 'r') as f:
        reader = csv.DictReader(f)
        
        for row in reader:
            # Generate trivariate hash
            triv_hash = hash_engine.generate_for_task(
                task_id=row['task_id'],
                task_name=row['task_name'],
                hd4_phase=row['hd4_phase'],
                primitive_type=row['primitive_type']
            )
            
            # Generate unicode address
            unicode_addr = hash_engine.to_unicode_rune(triv_hash.sch)
            
            # Insert into Supabase
            db_conn.execute("""
                INSERT INTO ctas_tasks (
                    task_id, task_name, task_description,
                    primitive_type, hd4_phase, domain, form_type,
                    trivariate_hash, sch_hash, cuid, sx9_uuid,
                    unicode_address, unicode_class,
                    ptcc_primitive_id, ptcc_primitive_name,
                    required_skills, tools,
                    hash_routing_enabled, unicode_routing_enabled,
                    protection_suite_id, protection_suite_version
                ) VALUES (
                    %s, %s, %s, %s, %s, %s, %s,
                    %s, %s, %s, %s, %s, %s,
                    %s, %s, %s, %s,
                    true, true,
                    %s, %s
                )
            """, (
                row['task_id'], row['task_name'], row.get('description'),
                row['primitive_type'], row['hd4_phase'], row.get('domain'), row.get('form_type'),
                triv_hash.full, triv_hash.sch, triv_hash.cuid, triv_hash.uuid,
                unicode_addr, 'A',  # Class A: Core Components
                row.get('ptcc_primitive_id'), row.get('ptcc_primitive_name'),
                json.dumps(row.get('required_skills', [])), json.dumps(row.get('tools', [])),
                row.get('protection_suite_id'), row.get('protection_suite_version')
            ))
```

### 4.3 Phase 3: Slot Graph Integration

```typescript
// Create Slot Graph nodes for tasks
async function createTaskSlotGraphNodes(db: SupabaseClient) {
    const { data: tasks } = await db
        .from('ctas_tasks')
        .select('*')
        .eq('hash_routing_enabled', true);
    
    for (const task of tasks) {
        // Create Slot Graph node (hash only, no data)
        await slotGraphClient.createNode({
            node_id: `task-${task.task_id}`,
            trivariate_hash: task.trivariate_hash,
            unicode_address: task.unicode_address,
            node_type: 'Task',
            // No data payload - only hash/unicode
        });
    }
}
```

### 4.4 Phase 4: Protection Suite Lineage

```sql
-- Link tasks to protection suite
UPDATE ctas_tasks
SET 
    protection_suite_id = 'protection-suite-v1.0',
    protection_suite_version = '1.0'
WHERE current_state = 'active';

-- Create lineage relationship
INSERT INTO relationships (
    source_entity_id,
    target_entity_id,
    relationship_type
)
SELECT 
    t.id,
    ps.id,
    'escalates_to'
FROM ctas_tasks t
CROSS JOIN entities ps
WHERE ps.entity_type = 'protection_suite'
AND t.protection_suite_id = ps.name;
```

---

## 5. Execution Path Integration

### 5.1 Playbook Execution

```yaml
# playbook: task-execution.yml
name: task-execution
trigger: gateway.request.task
escalate:
  - component: ctas7-cognitive-execution-tool
    condition: task.execution_path == "playbook"
    activate: true
  - component: ctas7-agent-dispatch
    condition: task.execution_path == "agent"
    activate: true
```

### 5.2 Cognitive Response

```rust
// Cognitive response via GNN/ANN/LLM
async fn execute_cognitive_response(
    task: &CtasTask,
    context: &ExecutionContext
) -> Result<ExecutionResult> {
    match task.execution_paths.cognitive.as_str() {
        "gnn" => {
            // GNN graph analysis
            gnn_fabric.analyze_task_graph(&task.trivariate_hash).await?
        }
        "ann" => {
            // ANN pattern recognition
            ann_engine.classify_task(&task.trivariate_hash).await?
        }
        "llm" => {
            // LLM semantic analysis
            llm_engine.generate_analysis(&task.task_description).await?
        }
        _ => Err(Error::InvalidExecutionPath)
    }
}
```

### 5.3 Operator Action

```typescript
// Operator force multiplication
interface OperatorAction {
    task_id: string;
    operator_id: string;
    action_type: 'approve' | 'reject' | 'modify' | 'escalate';
    force_multiply: boolean;  // Enable multi-tasking
    automation_level: number; // 0-100%
}
```

### 5.4 Agent Execution

```rust
// Agent dispatch via NATS
async fn dispatch_to_agent(
    task: &CtasTask,
    agent_id: &str
) -> Result<()> {
    nats_client.publish(
        &format!("sx9.escalate.agent.{}", agent_id),
        &serde_json::json!({
            "task_id": task.task_id,
            "trivariate_hash": task.trivariate_hash,
            "unicode_address": task.unicode_address,
            "execution_path": "agent"
        })
    ).await?;
    
    Ok(())
}
```

---

## 6. Performance Targets

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Task lookup (Supabase) | <10ms p99 | ACID query latency |
| Slot Graph routing | <1ms | Hash/unicode routing |
| Tool execution | <50μs | Bernoulli zone latency |
| Playbook execution | <100ms | DSL parsing + routing |
| Cognitive response | <5s | GNN/ANN/LLM analysis |
| Agent dispatch | <10ms | NATS publish latency |

---

## 7. Success Criteria

1. ✅ All 165 CTAS tasks migrated to Supabase (ACID)
2. ✅ Trivariate hash generated for each task
3. ✅ Unicode address assigned per RFC-9002
4. ✅ Slot Graph nodes created (hash/unicode only, no data)
5. ✅ Protection suite lineage established
6. ✅ Hourglass-Bernoulli zone classification applied
7. ✅ 32 Universal Primitives mapped
8. ✅ Execution paths configured (playbook/cognitive/operator/agent)
9. ✅ Performance targets met

---

## 8. Next Steps

1. **Deploy schema** to Supabase
2. **Import CSV** using migration script
3. **Create Slot Graph nodes** (hash/unicode routing)
4. **Link protection suite** lineage
5. **Test execution paths** (playbook/cognitive/operator/agent)
6. **Verify performance** targets

**This integration is speedy** because:
- Slot Graph routes hashes/unicode only (no data traversal)
- Supabase provides ACID guarantees
- Hourglass-Bernoulli ensures deterministic execution
- Protection suite lineage is maintained

---

**Status:** Ready for implementation



