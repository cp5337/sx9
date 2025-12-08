# SlotGraph Cypher Query Reference for Node Interviews

## Overview

This document provides Cypher query patterns for working with the `node_interviews` data in SlotGraph. The 164 CTAS tasks form a directed graph with:
- **Nodes**: Task interviews with first-person adversary voice
- **Edges**: Prerequisites, enables, combined_with relationships
- **Layers**: HD4 phases (Hunt → Detect → Disrupt → Disable → Dominate)

---

## 1. Node Creation Queries

### Create Interview Node
```cypher
CREATE (n:NodeInterview {
    task_id: $task_id,
    task_seq: $task_seq,
    task_label: $task_label,
    hd4_phase: $hd4_phase,
    is_key_indicator: $is_key_indicator,
    is_interdiction_point: $is_interdiction_point,
    voice: $voice,
    purpose: $purpose,
    ownership: $ownership,
    mitre_tactics: $mitre_tactics,
    mitre_techniques: $mitre_techniques,
    h1_operational: $h1_operational,
    h2_semantic: $h2_semantic,
    unicode_visual: $unicode_visual
})
RETURN n
```

### Batch Create from JSON
```cypher
UNWIND $interviews AS interview
CREATE (n:NodeInterview)
SET n = interview
RETURN count(n) as created
```

---

## 2. Edge Creation Queries

### Create Prerequisites Edge
```cypher
MATCH (source:NodeInterview {task_id: $source_task_id})
MATCH (target:NodeInterview {task_id: $target_task_id})
CREATE (source)-[:REQUIRES {
    relationship_type: 'prerequisite',
    weight: 1.0
}]->(target)
```

### Create Enables Edge
```cypher
MATCH (source:NodeInterview {task_id: $source_task_id})
MATCH (target:NodeInterview {task_id: $target_task_id})
CREATE (source)-[:ENABLES {
    relationship_type: 'enables',
    weight: 1.0
}]->(target)
```

### Create Combined_With Edge (bidirectional)
```cypher
MATCH (a:NodeInterview {task_id: $task_id_a})
MATCH (b:NodeInterview {task_id: $task_id_b})
CREATE (a)-[:COMBINED_WITH {weight: 0.8}]->(b)
CREATE (b)-[:COMBINED_WITH {weight: 0.8}]->(a)
```

### Build All Edges from Interview Data
```cypher
// Prerequisites (incoming)
MATCH (n:NodeInterview)
WHERE n.prerequisites IS NOT NULL
UNWIND n.prerequisites AS prereq_id
MATCH (prereq:NodeInterview {task_id: prereq_id})
MERGE (prereq)-[:ENABLES]->(n)

// Enables (outgoing)
MATCH (n:NodeInterview)
WHERE n.enables IS NOT NULL
UNWIND n.enables AS enables_id
MATCH (enabled:NodeInterview {task_id: enables_id})
MERGE (n)-[:ENABLES]->(enabled)

// Related tasks
MATCH (n:NodeInterview)
WHERE n.related_tasks IS NOT NULL
UNWIND n.related_tasks AS related_id
MATCH (related:NodeInterview {task_id: related_id})
MERGE (n)-[:RELATED_TO]->(related)
```

---

## 3. HD4 Phase Queries

### Get All Tasks by Phase
```cypher
MATCH (n:NodeInterview)
WHERE n.hd4_phase = $phase
RETURN n.task_id, n.task_seq, n.voice, n.task_label
ORDER BY n.task_seq
```

### Phase Distribution
```cypher
MATCH (n:NodeInterview)
RETURN n.hd4_phase as phase, count(n) as count
ORDER BY CASE n.hd4_phase
    WHEN 'Hunt' THEN 1
    WHEN 'Detect' THEN 2
    WHEN 'Disrupt' THEN 3
    WHEN 'Disable' THEN 4
    WHEN 'Dominate' THEN 5
END
```

### Cross-Phase Attack Paths
```cypher
MATCH path = (hunt:NodeInterview {hd4_phase: 'Hunt'})
             -[:ENABLES*1..5]->
             (dominate:NodeInterview {hd4_phase: 'Dominate'})
RETURN path, length(path) as hops
ORDER BY hops ASC
LIMIT 10
```

---

## 4. Task Label Queries (TTL Classification)

### Find All Mandatory Tasks
```cypher
MATCH (n:NodeInterview {task_label: 'mandatory'})
RETURN n.task_id, n.task_seq, n.hd4_phase, n.purpose
ORDER BY n.task_seq
```

### Key Indicators (Investigation Triggers)
```cypher
MATCH (n:NodeInterview {is_key_indicator: true})
RETURN n.task_id, n.hd4_phase, n.voice, n.indicators
ORDER BY n.hd4_phase, n.task_seq
```

### Interdiction Points (Intervention Opportunities)
```cypher
MATCH (n:NodeInterview {is_interdiction_point: true})
RETURN n.task_id, n.hd4_phase, n.purpose, n.counters_prevention
ORDER BY n.hd4_phase, n.task_seq
```

### Mandatory Tasks That Enable Key Indicators
```cypher
MATCH (mandatory:NodeInterview {task_label: 'mandatory'})
      -[:ENABLES]->
      (indicator:NodeInterview {is_key_indicator: true})
RETURN mandatory.task_id as mandatory_task,
       indicator.task_id as key_indicator,
       mandatory.hd4_phase as phase
```

---

## 5. MITRE ATT&CK Queries

### Find Tasks by Technique
```cypher
MATCH (n:NodeInterview)
WHERE $technique IN n.mitre_techniques
RETURN n.task_id, n.hd4_phase, n.voice
```

### Technique Coverage by Phase
```cypher
MATCH (n:NodeInterview)
UNWIND n.mitre_techniques AS technique
RETURN n.hd4_phase as phase,
       technique,
       count(*) as task_count
ORDER BY phase, task_count DESC
```

### Attack Chain by Tactic Sequence
```cypher
MATCH path = (recon:NodeInterview)-[:ENABLES*1..6]->(impact:NodeInterview)
WHERE 'TA0043' IN recon.mitre_tactics  // Reconnaissance
  AND 'TA0040' IN impact.mitre_tactics // Impact
RETURN [n IN nodes(path) | n.task_id] as attack_chain,
       [n IN nodes(path) | n.mitre_tactics[0]] as tactics
LIMIT 5
```

---

## 6. Convergence Detection Queries

### High-Risk Convergence (Multiple Indicators Active)
```cypher
MATCH (n:NodeInterview {is_key_indicator: true})
WHERE n.h1_operational IS NOT NULL
WITH n, n.h1_operational as h1
MATCH (related:NodeInterview)-[:RELATED_TO|ENABLES*1..2]-(n)
WHERE related.is_key_indicator = true
RETURN n.task_id as primary,
       collect(DISTINCT related.task_id) as converging_indicators,
       count(related) as convergence_score
ORDER BY convergence_score DESC
```

### Threat Cluster Detection
```cypher
// Find densely connected task clusters
CALL gds.louvain.stream('node_interview_graph')
YIELD nodeId, communityId
MATCH (n:NodeInterview) WHERE id(n) = nodeId
RETURN communityId as cluster,
       collect(n.task_id) as tasks,
       collect(DISTINCT n.hd4_phase) as phases
ORDER BY size(tasks) DESC
```

### Active Threat Path (with indicators firing)
```cypher
// Given active indicators, find likely attack progression
MATCH (active:NodeInterview)
WHERE active.task_id IN $active_task_ids
MATCH path = (active)-[:ENABLES*1..4]->(next:NodeInterview)
WHERE NOT next.task_id IN $active_task_ids
RETURN next.task_id as predicted_next,
       next.hd4_phase as phase,
       next.task_label as priority,
       length(path) as distance
ORDER BY distance,
         CASE next.task_label
           WHEN 'mandatory' THEN 1
           WHEN 'desirable' THEN 2
           ELSE 3
         END
```

---

## 7. Toolchain Queries

### Find Tasks Using Specific Tool
```cypher
MATCH (n:NodeInterview)
WHERE any(tool IN n.toolchain.kali WHERE tool.tool = $tool_name)
RETURN n.task_id, n.hd4_phase, n.toolchain.kali
```

### Kali Tool Distribution
```cypher
MATCH (n:NodeInterview)
WHERE n.toolchain.kali IS NOT NULL
UNWIND n.toolchain.kali AS kali_tool
RETURN kali_tool.tool as tool,
       count(*) as usage_count,
       collect(DISTINCT n.hd4_phase) as phases
ORDER BY usage_count DESC
LIMIT 20
```

---

## 8. EEI (Essential Elements of Information) Queries

### Detection Questions by Phase
```cypher
MATCH (n:NodeInterview)
WHERE n.eei.detection IS NOT NULL
RETURN n.hd4_phase as phase,
       n.task_id,
       n.eei.detection.questions as detection_questions,
       n.eei.detection.priority as priority
ORDER BY phase, priority DESC
```

### High Priority Collection Requirements
```cypher
MATCH (n:NodeInterview)
WHERE n.eei.detection.priority = 'high'
   OR n.eei.execution.priority = 'high'
RETURN n.task_id,
       n.eei.detection.collection_methods as detection_sources,
       n.eei.execution.collection_methods as execution_sources
```

---

## 9. Graph Statistics

### Overall Graph Metrics
```cypher
MATCH (n:NodeInterview)
OPTIONAL MATCH (n)-[r]->()
RETURN count(DISTINCT n) as total_nodes,
       count(r) as total_edges,
       avg(size(n.prerequisites)) as avg_prerequisites,
       avg(size(n.enables)) as avg_enables
```

### Nodes by Degree (Most Connected)
```cypher
MATCH (n:NodeInterview)
OPTIONAL MATCH (n)-[out]->()
OPTIONAL MATCH ()-[in]->(n)
WITH n, count(DISTINCT out) as out_degree, count(DISTINCT in) as in_degree
RETURN n.task_id,
       n.hd4_phase,
       in_degree + out_degree as total_degree,
       in_degree,
       out_degree
ORDER BY total_degree DESC
LIMIT 20
```

### Isolated Nodes (No Connections)
```cypher
MATCH (n:NodeInterview)
WHERE NOT (n)-[]-()
RETURN n.task_id, n.hd4_phase, n.task_label
```

---

## 10. SurrealDB Equivalent Queries

### Create Node (SurrealQL)
```surql
CREATE node_interview SET
    task_id = $task_id,
    task_seq = $task_seq,
    hd4_phase = $hd4_phase,
    voice = $voice,
    task_label = $task_label,
    is_key_indicator = $is_key_indicator,
    is_interdiction_point = $is_interdiction_point;
```

### Create Edge (SurrealQL)
```surql
RELATE node_interview:$source->enables->node_interview:$target
    SET weight = 1.0, relationship_type = 'prerequisite';
```

### Phase Query (SurrealQL)
```surql
SELECT * FROM node_interview
WHERE hd4_phase = 'Hunt'
ORDER BY task_seq;
```

### Graph Traversal (SurrealQL)
```surql
SELECT ->enables->node_interview.* as downstream
FROM node_interview
WHERE task_id = $task_id;
```

### Key Indicators with Relationships (SurrealQL)
```surql
SELECT
    task_id,
    hd4_phase,
    ->enables->node_interview.task_id as enables,
    <-enables<-node_interview.task_id as enabled_by
FROM node_interview
WHERE is_key_indicator = true;
```

---

## Index Recommendations

```cypher
// Neo4j Indexes
CREATE INDEX node_interview_task_id FOR (n:NodeInterview) ON (n.task_id);
CREATE INDEX node_interview_phase FOR (n:NodeInterview) ON (n.hd4_phase);
CREATE INDEX node_interview_label FOR (n:NodeInterview) ON (n.task_label);
CREATE INDEX node_interview_indicator FOR (n:NodeInterview) ON (n.is_key_indicator);
CREATE INDEX node_interview_interdiction FOR (n:NodeInterview) ON (n.is_interdiction_point);

// Full-text for voice search
CREATE FULLTEXT INDEX node_interview_voice FOR (n:NodeInterview) ON EACH [n.voice];

// GIN index for MITRE arrays (PostgreSQL equivalent in Supabase)
CREATE INDEX idx_mitre_techniques ON node_interviews USING GIN(mitre_techniques);
```

---

## Usage with ABE Generator

After running the ABE batch:
```bash
# 1. Export tasks from Supabase
python generate_node_interviews.py --export-tasks

# 2. Generate ABE prompts
python generate_node_interviews.py --export-prompts

# 3. Run ABE batch on GPU machine
bash output/abe_prompts/run_abe_batch.sh

# 4. Import results
python generate_node_interviews.py --import-results

# 5. Upload to Supabase
python generate_node_interviews.py --upload

# 6. Export to SlotGraph (Neo4j/SurrealDB)
python generate_node_interviews.py --export-slotgraph
```
