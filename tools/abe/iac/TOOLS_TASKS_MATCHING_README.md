# Tools & Tool Chains to CTAS Tasks & PTCC Matching

## Overview

This script matches security tools and tool chains to CTAS tasks AND PTCC configurations using Gemini AI, then generates Cypher queries for Neo4j import.

**Workflow (RFC-9001/9002 Compliant):**
1. **Dual-Trivariate Hashing** - Tools processed through `yaml_dsl_pipeline.py` (primary + secondary hashes)
2. **SPIRES Ontology** - All data processed through `spires_ontology_extractor.py`
3. **Matching** - Gemini matches Kali tools to CTAS tasks AND PTCC configurations

**Important:** This matches to **CTAS tasks** (uuid- format), NOT ATL tasks (hierarchical 1.3.2 format).

## Current Neo4j Status

```
Technique nodes: 1088 (MITRE ATT&CK)
Platform nodes: 26
Tactic nodes: 19
CTAS tasks: 0 (will be populated)
Tools: 0 (will be populated)
```

## Script: `match_tools_to_ctas_tasks.py`

### What It Does

1. **Loads Tools (with Dual-Trivariate Hashes)**
   - From `yaml_dsl_pipeline.py` output (tools already hashed)
   - Includes primary + secondary trivariate hashes (RFC-9001)
   - Includes Unicode operations (RFC-9002)
   - Falls back to raw tools if hashes not available

2. **Loads CTAS Tasks** from CSV
   - Verifies uuid- format (not ATL's 1.3.2 format)
   - Loads task metadata (name, description, HD4 phase, primitive type)

3. **Loads PTCC Configurations (SPIRES-processed)**
   - From `ctas7-ptcc-teth-database/abe_results/`
   - Already processed through SPIRES ontology extraction
   - Includes operator, skill level, HD4 phase mappings

4. **Gemini Matching**
   - Matches tools to **CTAS tasks** (by task description, HD4 phase)
   - Matches tools to **PTCC configurations** (by operator, tool, skill level)
   - Uses Gemini 2.0 Flash for intelligent matching
   - Returns confidence scores and reasoning

5. **Generates Cypher Queries**
   - Creates Tool nodes (with trivariate hashes)
   - Creates ToolChain nodes
   - Creates CTASTask nodes
   - Creates PTCC nodes
   - Creates relationships (SUPPORTS_TASK, MATCHES_PTCC, PART_OF_CHAIN)

6. **Imports to Neo4j**
   - Executes Cypher queries
   - Populates graph database with hashed, SPIRES-processed data

### Usage

**Prerequisites:**
1. Run `threat_content_fetcher.py --all` to download tools
2. Run `yaml_dsl_pipeline.py` to generate dual-trivariate hashes
3. Run `spires_ontology_extractor.py` to process through SPIRES

**Then run matching:**
```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac

# Set Gemini API key
export GEMINI_API_KEY=your_key_here

# Run matching (expects hashed, SPIRES-processed data)
python match_tools_to_ctas_tasks.py
```

### Output

The script generates **three output formats**:

1. **TOML file:** `ctas-glaf/import/tools_tasks_matching.toml`
   - Human-readable format
   - Contains all tools, tasks, PTCCs, chains, and matches
   - Can be used for configuration and review

2. **JSON file:** `ctas-glaf/import/tools_tasks_matching.json`
   - Machine-readable format
   - Same data as TOML, structured for programmatic access
   - Includes metadata, matches, and relationships

3. **Cypher file:** `ctas-glaf/import/tools_tasks_matching.cypher`
   - Neo4j import queries
   - Creates nodes (Tool, CTASTask, PTCC, ToolChain)
   - Creates relationships (SUPPORTS_TASK, MATCHES_PTCC, PART_OF_CHAIN)
   - Automatically executed if Neo4j driver available

**Note:** PLASMA rule generation is deferred until after reviewing matches. PLASMA rules require:
- Rule IDs, levels, descriptions
- 1NF indicators (regex, countermeasures)
- 2NF evasion tactics
- Nine-sided dual-trivariate hashes
- Active response configurations

We'll generate PLASMA TOML rules from the matches in a follow-up step.

### Requirements

```bash
pip install google-generativeai neo4j pandas
```

### Environment Variables

- `GEMINI_API_KEY` - Required for intelligent matching
- `NEO4J_URI` - Default: `bolt://localhost:7687`
- `NEO4J_USER` - Default: `neo4j`
- `NEO4J_PASSWORD` - Default: `ctas7_graph`

## CTAS vs ATL Tasks

### CTAS Tasks (This Script)
- Format: `uuid-000-000-001`
- Source: `ctas_tasks_with_primitive_type.csv`
- Domain: Cyber threat analysis
- Structure: hash_id, task_name, description, hd4_phase, primitive_type

### ATL Tasks (NOT This Script)
- Format: `1.3.2`, `3.4.5.1` (hierarchical)
- Source: ATL Physical domain
- Domain: Physical threat analysis
- Structure: task_id, title, phase, modality, classification

## Matching Logic

1. **Tool-to-CTAS Task Matching**
   - Gemini analyzes tool capabilities vs task requirements
   - Uses tool's dual-trivariate hash for routing context
   - Confidence score: 0.0-1.0
   - Only matches > 0.3 confidence are kept

2. **Tool-to-PTCC Matching**
   - Matches tools to PTCC configurations (already SPIRES-processed)
   - Checks operator, tool name, skill level alignment
   - Uses PTCC's recommended HD4 phase
   - Confidence score: 0.0-1.0

3. **Tool Chain-to-Task Matching**
   - Matches by HD4 phase alignment
   - Checks tool overlap with task keywords
   - Base confidence: 0.6 (phase match), 0.8 (tool overlap)

4. **Fallback**
   - If Gemini unavailable, uses simple keyword matching
   - Lower accuracy but still functional

## Cypher Query Structure

```cypher
// Tools
MERGE (t:Tool {id: 'tool_id'})
SET t.name = 'Tool Name', t.source = 'kali_tools'

// Tool Chains
MERGE (tc:ToolChain {id: 'chain_id'})
SET tc.name = 'Chain Name', tc.type = 'operator_based'

// CTAS Tasks
MERGE (task:CTASTask {hash_id: 'uuid-000-000-001'})
SET task.name = 'Task Name', task.hd4_phase = 'Hunt'

// Relationships
MATCH (t:Tool {id: 'tool_id'})
MATCH (task:CTASTask {hash_id: 'uuid-000-000-001'})
MERGE (t)-[r:SUPPORTS_TASK {confidence: 0.85}]->(task)
```

## Next Steps

1. Run the script to populate Neo4j
2. Verify matches in Neo4j Browser (http://localhost:7474)
3. Query relationships:
   ```cypher
   MATCH (t:Tool)-[r:SUPPORTS_TASK]->(task:CTASTask)
   WHERE r.confidence > 0.7
   RETURN t.name, task.name, r.confidence
   ORDER BY r.confidence DESC
   LIMIT 20
   ```

