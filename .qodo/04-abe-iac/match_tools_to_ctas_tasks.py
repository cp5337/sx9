#!/usr/bin/env python3
"""
Match Tools and Tool Chains to CTAS Tasks using Gemini
Generate Cypher queries for Neo4j import

WORKFLOW (RFC-9001/9002 Compliant):
1. Load tools (Kali, etc.) - already has dual-trivariate hashes from yaml_dsl_pipeline
2. Load CTAS tasks from CSV (NOT ATL tasks - CTAS uses uuid- format)
3. Load PTCC configurations (already hashed and SPIRES-processed)
4. Use Gemini to match tools/tool chains to CTAS tasks and PTCCs
5. Generate Cypher queries for Neo4j import

This script expects:
- Tools have been processed through yaml_dsl_pipeline (dual-trivariate hashes)
- SPIRES ontology has been generated (spires_ontology_extractor.py)
- All data is in the output directories with hashes included
"""

import os
import sys
import json
import csv
import pandas as pd
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from datetime import datetime

# TOML writing
try:
    import tomllib  # Python 3.11+
    HAS_TOML = True
except ImportError:
    try:
        import tomli  # For reading
        HAS_TOML = True
    except ImportError:
        HAS_TOML = False

# For writing TOML, we'll use a simple manual approach or toml library
try:
    import toml
    HAS_TOML_WRITE = True
except ImportError:
    try:
        import tomli_w
        HAS_TOML_WRITE = True
    except ImportError:
        HAS_TOML_WRITE = False
        print("⚠️  TOML writer not installed. TOML output will use JSON format. Run: pip install toml or tomli-w")

# Gemini API
try:
    import google.generativeai as genai
    HAS_GEMINI = True
except ImportError:
    HAS_GEMINI = False
    print("⚠️  google-generativeai not installed. Run: pip install google-generativeai")

# Neo4j driver
try:
    from neo4j import GraphDatabase
    HAS_NEO4J = True
except ImportError:
    HAS_NEO4J = False
    print("⚠️  neo4j driver not installed. Run: pip install neo4j")

# ============================================================================
# CONFIGURATION
# ============================================================================

# Paths
# Get the actual script directory and resolve BASE_DIR correctly
try:
    _script_dir = Path(__file__).parent.resolve()
except NameError:
    # Fallback if __file__ not available
    _script_dir = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac")

BASE_DIR = _script_dir.parent.resolve()  # Go up from 04-abe-iac to ctas-7-shipyard-staging

# CTAS tasks CSV - use absolute path (confirmed to exist)
CTAS_TASKS_CSV = BASE_DIR / "ctas_tasks_with_primitive_type.csv" / "ctas_tasks_with_primitive_type.csv"
THREAT_OUTPUT_DIR = BASE_DIR / "04-abe-iac" / "node-interview-generator" / "output" / "threat_content"
CYPHER_OUTPUT_DIR = BASE_DIR / "ctas-glaf" / "import"
CYPHER_OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# Neo4j connection
NEO4J_URI = os.getenv("NEO4J_URI", "bolt://localhost:7687")
NEO4J_USER = os.getenv("NEO4J_USER", "neo4j")
NEO4J_PASSWORD = os.getenv("NEO4J_PASSWORD", "Protected1")

# Gemini API
GEMINI_API_KEY = os.getenv("GEMINI_API_KEY")
if not GEMINI_API_KEY:
    print("⚠️  GEMINI_API_KEY not set. Will use mock matching.")
    print("   Set it with: export GEMINI_API_KEY=your_key")

# ============================================================================
# DATA LOADING
# ============================================================================

def load_ctas_tasks(csv_path: Path) -> List[Dict]:
    """Load CTAS tasks from CSV (NOT ATL - CTAS uses uuid- format)."""
    print(f"\n📋 Loading CTAS tasks from: {csv_path}")
    
    # Try to find the file if path doesn't exist
    if not csv_path.exists():
        # Try alternative locations
        alternatives = [
            Path(__file__).parent.parent.parent / "ctas_tasks_with_primitive_type.csv" / "ctas_tasks_with_primitive_type.csv",
            Path(__file__).parent.parent.parent / "ctas_tasks_with_primitive_type.csv",
            Path(__file__).parent.parent.parent.parent / "ctas7-command-center" / "ctas_tasks_with_primitive_type.csv",
        ]
        for alt in alternatives:
            if alt.exists() and alt.is_file():
                csv_path = alt
                print(f"  ✅ Found CSV at: {csv_path}")
                break
        else:
            raise FileNotFoundError(f"CTAS tasks CSV not found. Tried: {csv_path} and alternatives")
    
    tasks = []
    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            # CSV has empty first column, so hash_id might be in first data column
            # Try multiple ways to get the task ID
            task_id = (
                row.get('hash_id', '').strip() or 
                row.get('task_id', '').strip() or
                list(row.values())[0].strip() if row else ''  # First column value
            )
            # Skip empty rows
            if not task_id:
                continue
            # Verify this is CTAS (not ATL) - CTAS uses uuid- format
            if not task_id.startswith('uuid-'):
                # Skip header row or non-CTAS tasks
                continue
            
            tasks.append({
                'hash_id': task_id,
                'task_name': row.get('task_name', ''),
                'description': row.get('description', ''),
                'category': row.get('category', ''),
                'hd4_phase': row.get('hd4_phase', ''),
                'primitive_type': row.get('primitive_type', ''),
                'predecessors': row.get('predecessors', '').split(';') if row.get('predecessors') else [],
                'successors': row.get('successors', '').split(';') if row.get('successors') else [],
                'p': float(row.get('p', 0)) if row.get('p') else None,
                't': float(row.get('t', 0)) if row.get('t') else None,
                'h': float(row.get('h', 0)) if row.get('h') else None,
                'task_seq': int(row.get('task_seq', 0)) if row.get('task_seq') else 0,
            })
    
    print(f"  ✅ Loaded {len(tasks)} CTAS tasks (uuid- format verified)")
    return tasks

def load_tools_from_fetcher(output_dir: Path) -> Dict:
    """
    Load tools from ALL threat intelligence sources.
    Tools should already have dual-trivariate hashes from yaml_dsl_pipeline.
    
    Sources:
    - MITRE ATT&CK (techniques, software)
    - Atomic Red Team (tests)
    - Nuclei (templates)
    - Sigma (rules)
    - Caldera (abilities)
    - Nmap (scripts)
    - LOLBAS, GTFOBins, etc.
    - Kali Tools
    """
    print(f"\n🔧 Loading ALL tools from threat intelligence sources...")
    print(f"   Source directory: {output_dir}")
    
    tools_data = {}
    
    # ========== 1. Load from DSL pipeline output (tools with hashes) ==========
    dsl_output_dir = output_dir.parent / "task_graph"
    if dsl_output_dir.exists():
        tool_graph_file = dsl_output_dir / "threat_tools_graph.json"
        if tool_graph_file.exists():
            with open(tool_graph_file, 'r') as f:
                tool_graph = json.load(f)
                # Extract ALL tools with their hashes
                for tool_id, tool_data in tool_graph.get('tools', {}).items():
                    tool_source = tool_data.get('source', 'unknown')
                    tools_data.setdefault(tool_source, {})[tool_id] = {
                        'name': tool_data.get('name', tool_id),
                        'description': tool_data.get('description', ''),
                        'type': tool_data.get('type', ''),
                        'trivariate_primary': tool_data.get('trivariate', ''),
                        'trivariate_secondary': tool_data.get('trivariate_secondary', ''),
                        'dual_hash': tool_data.get('dual_hash', {}),
                        'unicode_op': tool_data.get('unicode_op', ''),
                        'source': tool_source,
                    }
            print(f"  ✅ Loaded {sum(len(v) for v in tools_data.values())} tools with hashes from DSL pipeline")
    
    # ========== 2. Load from threat_content_fetcher raw output ==========
    # Load ALL tool sources (not just Kali)
    tools_file = output_dir / "threat_content_summary.json"
    if tools_file.exists():
        with open(tools_file, 'r') as f:
            data = json.load(f)
            
            # Load ALL tool sources
            tool_sources = {
                'kali_tools': data.get('kali_tools', {}),
                'atomic_tests': data.get('atomic_tests', {}),
                'nuclei_templates': data.get('nuclei_templates', {}),
                'sigma_rules': data.get('sigma_rules', {}),
                'caldera_abilities': data.get('caldera_abilities', {}),
                'nmap_scripts': data.get('nmap_scripts', {}),
                'lolbas': data.get('lolbas', {}),
                'gtfobins': data.get('gtfobins', {}),
                'yara_rules': data.get('yara_rules', {}),
                'wazuh_rules': data.get('wazuh_rules', {}),
            }
            
            for source_name, source_data in tool_sources.items():
                if source_data and isinstance(source_data, dict):
                    for tool_id, tool_info in source_data.items():
                        if tool_id not in tools_data.get(source_name, {}):
                            tools_data.setdefault(source_name, {})[tool_id] = {
                                'name': tool_info.get('name', tool_id) if isinstance(tool_info, dict) else str(tool_id),
                                'description': tool_info.get('description', '') if isinstance(tool_info, dict) else '',
                                'type': source_name,
                                'source': source_name,
                            }
            
            total_raw = sum(len(v) for v in tool_sources.values() if isinstance(v, dict))
            if total_raw > 0:
                print(f"  ✅ Loaded {total_raw} raw tools from threat_content_fetcher (run yaml_dsl_pipeline for hashes)")
    
    # ========== 3. Load from MITRE ATT&CK (techniques as tools) ==========
    mitre_file = output_dir / "mitre_attack.json"
    if mitre_file.exists():
        try:
            with open(mitre_file, 'r') as f:
                mitre_data = json.load(f)
                # Extract techniques and software as tools
                for obj in mitre_data.get('objects', []):
                    obj_type = obj.get('type', '')
                    if obj_type in ['attack-pattern', 'tool', 'malware']:
                        obj_id = obj.get('id', '')
                        if obj_id:
                            tools_data.setdefault('mitre_techniques', {})[obj_id] = {
                                'name': obj.get('name', obj_id),
                                'description': ' '.join(obj.get('description', '').split()[:20]) if obj.get('description') else '',
                                'type': obj_type,
                                'source': 'mitre_attack',
                                'external_id': obj.get('external_references', [{}])[0].get('external_id', '') if obj.get('external_references') else '',
                            }
            print(f"  ✅ Loaded {len(tools_data.get('mitre_techniques', {}))} MITRE techniques/software")
        except Exception as e:
            print(f"  ⚠️  Error loading MITRE: {e}")
    
    # ========== 4. Load Atomic Red Team tests ==========
    atomic_dir = output_dir / "atomic-red-team"
    if atomic_dir.exists():
        atomic_count = 0
        for yaml_file in atomic_dir.rglob("*.yaml"):
            try:
                import yaml
                with open(yaml_file, 'r') as f:
                    atomic_data = yaml.safe_load(f)
                    if atomic_data and isinstance(atomic_data, dict):
                        atomic_id = atomic_data.get('attack_technique', {}).get('technique_id', '') or yaml_file.stem
                        if atomic_id:
                            tools_data.setdefault('atomic_tests', {})[atomic_id] = {
                                'name': atomic_data.get('display_name', atomic_id),
                                'description': atomic_data.get('description', ''),
                                'type': 'atomic_test',
                                'source': 'atomic_red_team',
                                'technique_id': atomic_data.get('attack_technique', {}).get('technique_id', ''),
                            }
                            atomic_count += 1
            except:
                pass
        if atomic_count > 0:
            print(f"  ✅ Loaded {atomic_count} Atomic Red Team tests")
    
    # ========== 5. Load Nuclei templates ==========
    nuclei_dir = output_dir / "nuclei-templates"
    if nuclei_dir.exists():
        nuclei_count = 0
        for yaml_file in nuclei_dir.rglob("*.yaml"):
            try:
                import yaml
                with open(yaml_file, 'r') as f:
                    nuclei_data = yaml.safe_load(f)
                    if nuclei_data and isinstance(nuclei_data, dict):
                        template_id = nuclei_data.get('id', '') or yaml_file.stem
                        if template_id:
                            tools_data.setdefault('nuclei_templates', {})[template_id] = {
                                'name': nuclei_data.get('info', {}).get('name', template_id),
                                'description': nuclei_data.get('info', {}).get('description', ''),
                                'type': 'nuclei_template',
                                'source': 'nuclei',
                                'severity': nuclei_data.get('info', {}).get('severity', ''),
                            }
                            nuclei_count += 1
            except:
                pass
        if nuclei_count > 0:
            print(f"  ✅ Loaded {nuclei_count} Nuclei templates")
    
    # ========== 6. Load Sigma rules ==========
    sigma_dir = output_dir / "sigma"
    if sigma_dir.exists():
        sigma_count = 0
        for yaml_file in sigma_dir.rglob("*.yaml"):
            try:
                import yaml
                with open(yaml_file, 'r') as f:
                    sigma_data = yaml.safe_load(f)
                    if sigma_data and isinstance(sigma_data, dict):
                        rule_id = sigma_data.get('id', '') or yaml_file.stem
                        if rule_id:
                            tools_data.setdefault('sigma_rules', {})[rule_id] = {
                                'name': sigma_data.get('title', rule_id),
                                'description': sigma_data.get('description', ''),
                                'type': 'sigma_rule',
                                'source': 'sigma',
                                'level': sigma_data.get('level', ''),
                            }
                            sigma_count += 1
            except:
                pass
        if sigma_count > 0:
            print(f"  ✅ Loaded {sigma_count} Sigma rules")
    
    # ========== 7. Load Caldera abilities ==========
    caldera_dir = output_dir / "caldera"
    if caldera_dir.exists():
        caldera_count = 0
        for yaml_file in caldera_dir.rglob("*.yaml"):
            try:
                import yaml
                with open(yaml_file, 'r') as f:
                    caldera_data = yaml.safe_load(f)
                    if caldera_data and isinstance(caldera_data, dict):
                        ability_id = caldera_data.get('id', '') or yaml_file.stem
                        if ability_id:
                            tools_data.setdefault('caldera_abilities', {})[ability_id] = {
                                'name': caldera_data.get('name', ability_id),
                                'description': caldera_data.get('description', ''),
                                'type': 'caldera_ability',
                                'source': 'caldera',
                                'tactic': caldera_data.get('tactic', ''),
                            }
                            caldera_count += 1
            except:
                pass
        if caldera_count > 0:
            print(f"  ✅ Loaded {caldera_count} Caldera abilities")
    
    # ========== 8. Load ExploitDB exploits ==========
    exploitdb_dir = output_dir / "exploitdb"
    if exploitdb_dir.exists():
        exploitdb_count = 0
        # Look for exploit index or process exploit files
        exploitdb_index = output_dir / "exploitdb_index.json"
        if exploitdb_index.exists():
            try:
                with open(exploitdb_index, 'r') as f:
                    exploitdb_data = json.load(f)
                    for exploit in exploitdb_data.get('exploits', []):
                        exploit_id = exploit.get('id', '') or exploit.get('file', '')
                        if exploit_id:
                            tools_data.setdefault('exploitdb', {})[exploit_id] = {
                                'name': exploit.get('description', exploit_id),
                                'description': exploit.get('description', ''),
                                'type': 'exploit',
                                'source': 'exploitdb',
                                'platform': exploit.get('platform', ''),
                                'cve': exploit.get('cve', ''),
                            }
                            exploitdb_count += 1
                print(f"  ✅ Loaded {exploitdb_count} ExploitDB exploits from index")
            except Exception as e:
                print(f"  ⚠️  Error loading ExploitDB index: {e}")
        else:
            # Try to process exploit files directly
            exploits_subdir = exploitdb_dir / "exploits"
            if exploits_subdir.exists():
                # Count exploit files
                exploit_files = list(exploits_subdir.rglob("*.txt")) + list(exploits_subdir.rglob("*.py"))
                if exploit_files:
                    print(f"  ⚠️  Found {len(exploit_files)} ExploitDB files (no index - run exploit_arsenal_extractor.py)")
    
    # ========== 9. Load PTCC tools (already hashed and SPIRES-processed) ==========
    ptcc_file = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-ptcc-teth-database/abe_results/abe_ptcc_results.json")
    if not ptcc_file.exists():
        ptcc_file = output_dir.parent.parent / "ctas7-ptcc-teth-database" / "abe_results" / "abe_ptcc_results.json"
    
    if ptcc_file.exists():
        with open(ptcc_file, 'r') as f:
            ptcc_data = json.load(f)
            for config in ptcc_data.get('recovered_configurations', []) + ptcc_data.get('generated_configurations', []):
                tool = config.get('tool', '')
                if tool:
                    tools_data.setdefault('ptcc_tools', {})[tool] = {
                        **config,
                        'ptcc_id': config.get('ptcc_id', ''),
                        'operator': config.get('operator', ''),
                        'skill_level': config.get('skill_level', 0.0),
                        'hd4_phase': config.get('recommended_hd4_phase', ''),
                        'source': 'ptcc',
                    }
        print(f"  ✅ Loaded {len(tools_data.get('ptcc_tools', {}))} PTCC tools")
    
    # Summary
    total_tools = sum(len(v) for v in tools_data.values())
    print(f"\n  ✅ Total tools loaded: {total_tools}")
    print(f"     Sources: {', '.join(tools_data.keys())}")
    
    return tools_data

def load_tool_chains_from_fetcher(output_dir: Path) -> Dict:
    """Load tool chains from threat_content_fetcher output."""
    print(f"\n🔗 Loading tool chains from: {output_dir}")
    
    # Tool chains are derived from PTCC configs
    ptcc_file = output_dir.parent.parent / "ctas7-ptcc-teth-database" / "abe_results" / "abe_ptcc_results.json"
    tool_chains = {}
    
    if ptcc_file.exists():
        with open(ptcc_file, 'r') as f:
            ptcc_data = json.load(f)
            
            # Group by operator, phase, region (as done in threat_content_fetcher)
            configs = ptcc_data.get('recovered_configurations', []) + ptcc_data.get('generated_configurations', [])
            
            # Operator-based chains
            operator_chains = {}
            for config in configs:
                operator = config.get('operator', 'unknown')
                if operator not in operator_chains:
                    operator_chains[operator] = []
                operator_chains[operator].append(config)
            
            for operator, configs_list in operator_chains.items():
                chain_id = f"operator_chain_{operator.replace(' ', '_')}"
                tool_chains[chain_id] = {
                    'id': chain_id,
                    'name': f"Operator Chain: {operator}",
                    'type': 'operator_based',
                    'tools': [c.get('tool', '') for c in configs_list],
                    'hd4_phases': list(set(c.get('recommended_hd4_phase', 'Unknown') for c in configs_list)),
                }
    
    print(f"  ✅ Loaded {len(tool_chains)} tool chains")
    return tool_chains

# ============================================================================
# GEMINI MATCHING
# ============================================================================

def match_tool_to_task_gemini(tool: Dict, task: Dict, model) -> Tuple[float, str]:
    """
    Use Gemini to match a tool to a CTAS task.
    Returns (confidence_score, reasoning).
    """
    prompt = f"""You are a threat intelligence analyst matching security tools to operational tasks.

CTAS Task:
- ID: {task['hash_id']}
- Name: {task['task_name']}
- Description: {task['description']}
- Category: {task['category']}
- HD4 Phase: {task['hd4_phase']}
- Primitive Type: {task['primitive_type']}

Tool:
- Name: {tool.get('name', 'Unknown')}
- Type: {tool.get('type', 'Unknown')}
- Description: {tool.get('description', 'No description')}

Analyze if this tool is relevant for executing or supporting this CTAS task.
Return a JSON object with:
{{
  "confidence": 0.0-1.0,
  "reasoning": "Brief explanation",
  "relevance": "high|medium|low"
}}
"""
    
    try:
        response = model.generate_content(prompt)
        result_text = response.text.strip()
        
        # Extract JSON from response
        if '```json' in result_text:
            result_text = result_text.split('```json')[1].split('```')[0].strip()
        elif '```' in result_text:
            result_text = result_text.split('```')[1].split('```')[0].strip()
        
        result = json.loads(result_text)
        confidence = float(result.get('confidence', 0.0))
        reasoning = result.get('reasoning', '')
        
        return confidence, reasoning
    except Exception as e:
        print(f"  ⚠️  Gemini error: {e}")
        return 0.0, f"Error: {e}"

def match_tools_to_tasks_and_ptccs(tools: Dict, tasks: List[Dict], ptcc_configs: Dict, use_gemini: bool = True) -> List[Dict]:
    """
    Match tools to CTAS tasks AND PTCC configurations.
    Tools should already have dual-trivariate hashes from yaml_dsl_pipeline.
    PTCCs should already be SPIRES-processed.
    """
    print(f"\n🔍 Matching {sum(len(v) for v in tools.values())} tools to:")
    print(f"   - {len(tasks)} CTAS tasks")
    print(f"   - {len(ptcc_configs)} PTCC configurations")
    
    matches = []
    
    # Initialize Gemini if available
    model = None
    if use_gemini and HAS_GEMINI and GEMINI_API_KEY:
        genai.configure(api_key=GEMINI_API_KEY)
        model = genai.GenerativeModel("gemini-2.0-flash-exp")
        print("  ✅ Using Gemini for intelligent matching")
    else:
        print("  ⚠️  Using simple keyword matching (Gemini not available)")
    
    # Match each tool to tasks
    for tool_source, tool_dict in tools.items():
        for tool_id, tool_data in tool_dict.items():
            if not isinstance(tool_data, dict):
                continue
            
            tool_name = tool_data.get('name', tool_id)
            
            # For each task, calculate match
            for task in tasks:
                if use_gemini and model:
                    confidence, reasoning = match_tool_to_task_gemini(tool_data, task, model)
                else:
                    # Simple keyword matching fallback
                    confidence = 0.0
                    reasoning = "Keyword matching"
                    
                    # Check if tool name or description matches task keywords
                    task_keywords = set(task['task_name'].lower().split() + task['description'].lower().split()[:10])
                    tool_keywords = set(tool_name.lower().split())
                    
                    if task_keywords & tool_keywords:
                        confidence = 0.5
                
                # Only keep matches above threshold
                if confidence > 0.3:
                    matches.append({
                        'tool_id': tool_id,
                        'tool_name': tool_name,
                        'tool_source': tool_source,
                        'tool_trivariate': tool_data.get('trivariate_primary', ''),
                        'tool_unicode': tool_data.get('unicode_op', ''),
                        'task_hash_id': task['hash_id'],
                        'task_name': task['task_name'],
                        'hd4_phase': task['hd4_phase'],
                        'confidence': confidence,
                        'reasoning': reasoning,
                        'match_type': 'task',
                    })
            
            # Also match to PTCC configurations
            for ptcc_id, ptcc_config in ptcc_configs.items():
                if use_gemini and model:
                    # Match tool to PTCC
                    ptcc_prompt = f"""Match tool to PTCC configuration:

Tool: {tool_name} ({tool_data.get('description', '')})
PTCC: Operator={ptcc_config.get('operator', '')}, Tool={ptcc_config.get('tool', '')}, Skill={ptcc_config.get('skill_level', 0)}

Return JSON: {{"confidence": 0.0-1.0, "reasoning": "explanation"}}
"""
                    try:
                        response = model.generate_content(ptcc_prompt)
                        result_text = response.text.strip()
                        if '```json' in result_text:
                            result_text = result_text.split('```json')[1].split('```')[0].strip()
                        result = json.loads(result_text)
                        ptcc_confidence = float(result.get('confidence', 0.0))
                    except:
                        ptcc_confidence = 0.0
                else:
                    # Simple matching: check if tool name matches PTCC tool
                    ptcc_confidence = 0.8 if tool_name.lower() == ptcc_config.get('tool', '').lower() else 0.0
                
                if ptcc_confidence > 0.3:
                    matches.append({
                        'tool_id': tool_id,
                        'tool_name': tool_name,
                        'tool_source': tool_source,
                        'tool_trivariate': tool_data.get('trivariate_primary', ''),
                        'ptcc_id': ptcc_id,
                        'ptcc_operator': ptcc_config.get('operator', ''),
                        'ptcc_skill_level': ptcc_config.get('skill_level', 0.0),
                        'confidence': ptcc_confidence,
                        'reasoning': f"PTCC match: {ptcc_config.get('tool', '')}",
                        'match_type': 'ptcc',
                    })
    
    print(f"  ✅ Found {len(matches)} tool matches:")
    task_matches = [m for m in matches if m['match_type'] == 'task']
    ptcc_matches = [m for m in matches if m['match_type'] == 'ptcc']
    print(f"     - {len(task_matches)} tool-to-task matches")
    print(f"     - {len(ptcc_matches)} tool-to-PTCC matches")
    return matches

def match_tool_chains_to_tasks(tool_chains: Dict, tasks: List[Dict], use_gemini: bool = True) -> List[Dict]:
    """Match tool chains to CTAS tasks."""
    print(f"\n🔗 Matching {len(tool_chains)} tool chains to {len(tasks)} CTAS tasks...")
    
    matches = []
    
    # Initialize Gemini if available
    model = None
    if use_gemini and HAS_GEMINI and GEMINI_API_KEY:
        genai.configure(api_key=GEMINI_API_KEY)
        model = genai.GenerativeModel("gemini-2.0-flash-exp")
        print("  ✅ Using Gemini for intelligent matching")
    
    for chain_id, chain_data in tool_chains.items():
        chain_tools = chain_data.get('tools', [])
        chain_hd4_phases = chain_data.get('hd4_phases', [])
        
        # Match to tasks by HD4 phase and tool overlap
        for task in tasks:
            # Check HD4 phase match
            if task['hd4_phase'] in chain_hd4_phases:
                confidence = 0.6  # Base confidence for phase match
                
                # Check if any tools in chain match task keywords
                task_keywords = set(task['task_name'].lower().split() + task['description'].lower().split()[:10])
                for tool in chain_tools:
                    if tool.lower() in task_keywords:
                        confidence = 0.8
                        break
                
                matches.append({
                    'chain_id': chain_id,
                    'chain_name': chain_data.get('name', chain_id),
                    'chain_type': chain_data.get('type', 'unknown'),
                    'task_hash_id': task['hash_id'],
                    'task_name': task['task_name'],
                    'hd4_phase': task['hd4_phase'],
                    'confidence': confidence,
                    'tools': chain_tools,
                })
    
    print(f"  ✅ Found {len(matches)} tool-chain-to-task matches")
    return matches

# ============================================================================
# CYPHER GENERATION
# ============================================================================

def generate_toml_output(
    tool_matches: List[Dict],
    chain_matches: List[Dict],
    tasks: List[Dict],
    ptcc_configs: Dict,
    output_dir: Path
) -> Optional[Path]:
    """Generate TOML output for all matches."""
    if not HAS_TOML_WRITE:
        print(f"\n⚠️  Skipping TOML output (TOML writer not installed)")
        return None
    
    print(f"\n📝 Generating TOML output...")
    
    toml_data = {
        'metadata': {
            'generated_at': datetime.now().isoformat(),
            'total_tools': len(set(m['tool_id'] for m in tool_matches)),
            'total_tasks': len(tasks),
            'total_ptccs': len(ptcc_configs),
            'total_matches': len(tool_matches),
            'total_chains': len(chain_matches),
        },
        'tools': {},
        'tasks': {},
        'ptccs': {},
        'tool_chains': {},
        'matches': {
            'tool_to_task': [m for m in tool_matches if m.get('match_type') == 'task'],
            'tool_to_ptcc': [m for m in tool_matches if m.get('match_type') == 'ptcc'],
            'chain_to_task': chain_matches,
        }
    }
    
    # Add tools
    for match in tool_matches:
        tool_id = match['tool_id']
        if tool_id not in toml_data['tools']:
            toml_data['tools'][tool_id] = {
                'name': match['tool_name'],
                'source': match['tool_source'],
                'trivariate_primary': match.get('tool_trivariate', ''),
                'unicode_op': match.get('tool_unicode', ''),
            }
    
    # Add tasks
    for task in tasks:
        toml_data['tasks'][task['hash_id']] = {
            'name': task['task_name'],
            'description': task['description'][:200],
            'category': task['category'],
            'hd4_phase': task['hd4_phase'],
            'primitive_type': task['primitive_type'],
        }
    
    # Add PTCCs
    for ptcc_id, ptcc in ptcc_configs.items():
        toml_data['ptccs'][str(ptcc_id)] = {
            'operator': ptcc.get('operator', ''),
            'tool': ptcc.get('tool', ''),
            'skill_level': ptcc.get('skill_level', 0.0),
            'hd4_phase': ptcc.get('recommended_hd4_phase', ''),
        }
    
    # Add tool chains
    for match in chain_matches:
        toml_data['tool_chains'][match['chain_id']] = {
            'name': match['chain_name'],
            'type': match['chain_type'],
            'tools': match['tools'],
        }
    
    toml_file = output_dir / "tools_tasks_matching.toml"
    
    # Write TOML using available library
    if 'toml' in sys.modules:
        with open(toml_file, 'w') as f:
            toml.dump(toml_data, f)
    elif 'tomli_w' in sys.modules:
        import tomli_w
        with open(toml_file, 'wb') as f:
            tomli_w.dump(toml_data, f)
    else:
        # Fallback: write as JSON with .toml extension (not ideal but functional)
        with open(toml_file, 'w') as f:
            json.dump(toml_data, f, indent=2)
        print(f"  ⚠️  TOML writer not available, wrote JSON format to .toml file")
    
    print(f"  ✅ Generated TOML: {toml_file}")
    return toml_file

def generate_json_output(
    tool_matches: List[Dict],
    chain_matches: List[Dict],
    tasks: List[Dict],
    ptcc_configs: Dict,
    output_dir: Path
) -> Path:
    """Generate JSON output for all matches."""
    print(f"\n📝 Generating JSON output...")
    
    json_data = {
        'metadata': {
            'generated_at': datetime.now().isoformat(),
            'total_tools': len(set(m['tool_id'] for m in tool_matches)),
            'total_tasks': len(tasks),
            'total_ptccs': len(ptcc_configs),
            'total_matches': len(tool_matches),
            'total_chains': len(chain_matches),
        },
        'tools': {m['tool_id']: {
            'name': m['tool_name'],
            'source': m['tool_source'],
            'trivariate_primary': m.get('tool_trivariate', ''),
            'unicode_op': m.get('tool_unicode', ''),
        } for m in tool_matches},
        'tasks': {t['hash_id']: t for t in tasks},
        'ptccs': ptcc_configs,
        'tool_chains': {m['chain_id']: {
            'name': m['chain_name'],
            'type': m['chain_type'],
            'tools': m['tools'],
        } for m in chain_matches},
        'matches': {
            'tool_to_task': [m for m in tool_matches if m.get('match_type') == 'task'],
            'tool_to_ptcc': [m for m in tool_matches if m.get('match_type') == 'ptcc'],
            'chain_to_task': chain_matches,
        }
    }
    
    json_file = output_dir / "tools_tasks_matching.json"
    with open(json_file, 'w') as f:
        json.dump(json_data, f, indent=2)
    
    print(f"  ✅ Generated JSON: {json_file}")
    return json_file

def generate_cypher_queries(
    tool_matches: List[Dict],
    chain_matches: List[Dict],
    tasks: List[Dict],
    ptcc_configs: Dict,
    output_dir: Path
) -> str:
    """Generate Cypher queries for Neo4j import."""
    print(f"\n📝 Generating Cypher queries...")
    
    cypher_queries = []
    
    # Create Tool nodes
    cypher_queries.append("// ============================================================================")
    cypher_queries.append("// TOOLS")
    cypher_queries.append("// ============================================================================")
    
    unique_tools = {}
    for match in tool_matches:
        tool_id = match['tool_id']
        if tool_id not in unique_tools:
            unique_tools[tool_id] = {
                'id': tool_id,
                'name': match['tool_name'],
                'source': match['tool_source'],
            }
    
    for tool_id, tool_data in unique_tools.items():
        # Get trivariate hash if available
        tool_match = next((m for m in tool_matches if m['tool_id'] == tool_id), None)
        trivariate = tool_match.get('tool_trivariate', '') if tool_match else ''
        unicode_op = tool_match.get('tool_unicode', '') if tool_match else ''
        
        cypher_queries.append(
            f"MERGE (t:Tool {{id: '{tool_id}'}})\n"
            f"SET t.name = '{tool_data['name'].replace("'", "\\'")}',\n"
            f"    t.source = '{tool_data['source']}',\n"
            f"    t.trivariate_primary = '{trivariate}',\n"
            f"    t.unicode_op = '{unicode_op}'"
        )
    
    # Create ToolChain nodes
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// TOOL CHAINS")
    cypher_queries.append("// ============================================================================")
    
    unique_chains = {}
    for match in chain_matches:
        chain_id = match['chain_id']
        if chain_id not in unique_chains:
            unique_chains[chain_id] = {
                'id': chain_id,
                'name': match['chain_name'],
                'type': match['chain_type'],
            }
    
    for chain_id, chain_data in unique_chains.items():
        cypher_queries.append(
            f"MERGE (tc:ToolChain {{id: '{chain_id}'}})\n"
            f"SET tc.name = '{chain_data['name'].replace("'", "\\'")}',\n"
            f"    tc.type = '{chain_data['type']}'"
        )
    
    # Create CTAS Task nodes (if not already in Neo4j)
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// CTAS TASKS")
    cypher_queries.append("// ============================================================================")
    
    unique_tasks = {t['hash_id']: t for t in tasks}
    for task_id, task in unique_tasks.items():
        cypher_queries.append(
            f"MERGE (task:CTASTask {{hash_id: '{task_id}'}})\n"
            f"SET task.name = '{task['task_name'].replace("'", "\\'")}',\n"
            f"    task.description = '{task['description'].replace("'", "\\'")[:200]}',\n"
            f"    task.category = '{task['category']}',\n"
            f"    task.hd4_phase = '{task['hd4_phase']}',\n"
            f"    task.primitive_type = '{task['primitive_type']}'"
        )
    
    # Create PTCC nodes
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// PTCC CONFIGURATIONS")
    cypher_queries.append("// ============================================================================")
    
    for ptcc_id, ptcc in ptcc_configs.items():
        cypher_queries.append(
            f"MERGE (ptcc:PTCC {{id: '{ptcc_id}'}})\n"
            f"SET ptcc.operator = '{ptcc.get('operator', '').replace("'", "\\'")}',\n"
            f"    ptcc.tool = '{ptcc.get('tool', '').replace("'", "\\'")}',\n"
            f"    ptcc.skill_level = {ptcc.get('skill_level', 0.0)},\n"
            f"    ptcc.hd4_phase = '{ptcc.get('recommended_hd4_phase', '')}'"
        )
    
    # Create Tool -> Task relationships
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// TOOL -> TASK RELATIONSHIPS")
    cypher_queries.append("// ============================================================================")
    
    for match in tool_matches:
        if match.get('match_type') == 'task':
            cypher_queries.append(
                f"MATCH (t:Tool {{id: '{match['tool_id']}'}})\n"
                f"MATCH (task:CTASTask {{hash_id: '{match['task_hash_id']}'}})\n"
                f"MERGE (t)-[r:SUPPORTS_TASK {{confidence: {match['confidence']:.2f}, reasoning: '{match['reasoning'].replace("'", "\\'")[:100]}'}}]->(task)"
            )
    
    # Create Tool -> PTCC relationships
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// TOOL -> PTCC RELATIONSHIPS")
    cypher_queries.append("// ============================================================================")
    
    for match in tool_matches:
        if match.get('match_type') == 'ptcc':
            cypher_queries.append(
                f"MATCH (t:Tool {{id: '{match['tool_id']}'}})\n"
                f"MATCH (ptcc:PTCC {{id: '{match['ptcc_id']}'}})\n"
                f"MERGE (t)-[r:MATCHES_PTCC {{confidence: {match['confidence']:.2f}, reasoning: '{match['reasoning'].replace("'", "\\'")[:100]}'}}]->(ptcc)"
            )
    
    # Create ToolChain -> Task relationships
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// TOOL CHAIN -> TASK RELATIONSHIPS")
    cypher_queries.append("// ============================================================================")
    
    for match in chain_matches:
        tools_str = "', '".join(match['tools'][:5])  # Limit to 5 tools
        cypher_queries.append(
            f"MATCH (tc:ToolChain {{id: '{match['chain_id']}'}})\n"
            f"MATCH (task:CTASTask {{hash_id: '{match['task_hash_id']}'}})\n"
            f"MERGE (tc)-[r:SUPPORTS_TASK {{confidence: {match['confidence']:.2f}, tools: ['{tools_str}']}}]->(task)"
        )
    
    # Create Tool -> ToolChain relationships
    cypher_queries.append("\n// ============================================================================")
    cypher_queries.append("// TOOL -> TOOL CHAIN RELATIONSHIPS")
    cypher_queries.append("// ============================================================================")
    
    for match in chain_matches:
        for tool in match['tools']:
            cypher_queries.append(
                f"MATCH (t:Tool {{name: '{tool.replace("'", "\\'")}'}})\n"
                f"MATCH (tc:ToolChain {{id: '{match['chain_id']}'}})\n"
                f"MERGE (t)-[:PART_OF_CHAIN]->(tc)"
            )
    
    cypher_content = "\n\n".join(cypher_queries)
    
    # Save to file
    cypher_file = output_dir / "tools_tasks_matching.cypher"
    with open(cypher_file, 'w') as f:
        f.write(cypher_content)
    
    print(f"  ✅ Generated Cypher queries: {cypher_file}")
    print(f"     - {len(unique_tools)} tools")
    print(f"     - {len(unique_chains)} tool chains")
    print(f"     - {len(unique_tasks)} CTAS tasks")
    print(f"     - {len(tool_matches)} tool-to-task relationships")
    print(f"     - {len(chain_matches)} chain-to-task relationships")
    
    return cypher_content

# ============================================================================
# NEO4J IMPORT
# ============================================================================

def import_to_neo4j(cypher_file: Path):
    """Import Cypher queries to Neo4j."""
    if not HAS_NEO4J:
        print("  ⚠️  Neo4j driver not available. Skipping import.")
        return
    
    print(f"\n📤 Importing to Neo4j: {NEO4J_URI}")
    
    try:
        driver = GraphDatabase.driver(NEO4J_URI, auth=(NEO4J_USER, NEO4J_PASSWORD))
        
        with open(cypher_file, 'r') as f:
            cypher_content = f.read()
        
        # Split by double newlines (query separators)
        queries = [q.strip() for q in cypher_content.split('\n\n') if q.strip() and not q.strip().startswith('//')]
        
        with driver.session() as session:
            for i, query in enumerate(queries, 1):
                try:
                    session.run(query)
                    if i % 10 == 0:
                        print(f"  ✅ Processed {i}/{len(queries)} queries...")
                except Exception as e:
                    print(f"  ⚠️  Error in query {i}: {e}")
        
        driver.close()
        print(f"  ✅ Import complete: {len(queries)} queries executed")
        
    except Exception as e:
        print(f"  ❌ Neo4j import error: {e}")

# ============================================================================
# MAIN
# ============================================================================

def load_ptcc_configurations() -> Dict:
    """Load ALL PTCC configurations from all PTCC files (SPIRES-processed)."""
    print(f"\n📊 Loading ALL PTCC configurations (SPIRES-processed)...")
    
    # Use absolute path to ensure we find it
    ptcc_db_dir = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-ptcc-teth-database/abe_results")
    if not ptcc_db_dir.exists():
        # Try relative to BASE_DIR
        ptcc_db_dir = BASE_DIR / "ctas7-ptcc-teth-database" / "abe_results"
    
    ptcc_configs = {}
    
    # Load from all PTCC files (same as threat_content_fetcher.py)
    ptcc_files = [
        ptcc_db_dir / "abe_ptcc_results.json",
        ptcc_db_dir / "abe_recovered_ptcc.json",
        ptcc_db_dir / "abe_generated_ptcc.json",
    ]
    
    print(f"  Looking in: {ptcc_db_dir}")
    
    total_loaded = 0
    for ptcc_file in ptcc_files:
        if ptcc_file.exists():
            try:
                with open(ptcc_file, 'r') as f:
                    ptcc_data = json.load(f)
                    
                    # Handle different JSON structures
                    configs = []
                    if isinstance(ptcc_data, dict):
                        configs = (
                            ptcc_data.get('recovered_configurations', []) +
                            ptcc_data.get('generated_configurations', []) +
                            ptcc_data.get('configurations', [])
                        )
                    elif isinstance(ptcc_data, list):
                        configs = ptcc_data
                    
                    for config in configs:
                        if isinstance(config, dict):
                            # Generate unique PTCC ID
                            ptcc_id = (
                                config.get('ptcc_id') or 
                                f"{config.get('operator', 'unknown')}_{config.get('tool', 'unknown')}_{total_loaded}"
                            )
                            ptcc_configs[str(ptcc_id)] = config
                            total_loaded += 1
                    
                print(f"  ✅ Loaded {len(configs)} PTCC configs from {ptcc_file.name}")
            except Exception as e:
                print(f"  ⚠️  Error reading {ptcc_file.name}: {e}")
        else:
            print(f"  ⚠️  PTCC file not found: {ptcc_file.name}")
    
    if total_loaded == 0:
        print(f"  ⚠️  No PTCC configurations loaded (check ctas7-ptcc-teth-database/abe_results/)")
    else:
        print(f"  ✅ Total PTCC configurations loaded: {len(ptcc_configs)}")
    
    return ptcc_configs

def main():
    print("=" * 70)
    print("Match Tools & Tool Chains to CTAS Tasks & PTCCs")
    print("Workflow: Dual-Trivariate Hash → SPIRES → Matching")
    print("=" * 70)
    
    # Load data (tools should already have hashes from yaml_dsl_pipeline)
    tasks = load_ctas_tasks(CTAS_TASKS_CSV)
    tools = load_tools_from_fetcher(THREAT_OUTPUT_DIR)
    tool_chains = load_tool_chains_from_fetcher(THREAT_OUTPUT_DIR)
    ptcc_configs = load_ptcc_configurations()
    
    # Match tools to tasks AND PTCCs
    tool_matches = match_tools_to_tasks_and_ptccs(tools, tasks, ptcc_configs, use_gemini=bool(GEMINI_API_KEY))
    chain_matches = match_tool_chains_to_tasks(tool_chains, tasks, use_gemini=bool(GEMINI_API_KEY))
    
    # Generate TOML, JSON, and Cypher outputs
    toml_file = generate_toml_output(tool_matches, chain_matches, tasks, ptcc_configs, CYPHER_OUTPUT_DIR)
    json_file = generate_json_output(tool_matches, chain_matches, tasks, ptcc_configs, CYPHER_OUTPUT_DIR)
    cypher_content = generate_cypher_queries(tool_matches, chain_matches, tasks, ptcc_configs, CYPHER_OUTPUT_DIR)
    
    # Import to Neo4j
    cypher_file = CYPHER_OUTPUT_DIR / "tools_tasks_matching.cypher"
    if cypher_file.exists():
        import_to_neo4j(cypher_file)
    
    print("\n✅ Complete!")
    if toml_file:
        print(f"   TOML file: {toml_file}")
    print(f"   JSON file: {json_file}")
    print(f"   Cypher file: {cypher_file}")
    task_matches_count = len([m for m in tool_matches if m.get('match_type') == 'task'])
    ptcc_matches_count = len([m for m in tool_matches if m.get('match_type') == 'ptcc'])
    print(f"   Matches: {task_matches_count} tool-to-task, {ptcc_matches_count} tool-to-PTCC, {len(chain_matches)} chains")
    print(f"\n💡 Note: PLASMA rule generation deferred - will generate after reviewing matches")

if __name__ == "__main__":
    main()

