#!/usr/bin/env python3
"""
ABE Node Interview Generator
Generates RFC-9025 compliant node interviews for 164 CTAS tasks
Designed for ABE (Automated Business Environment) high-GPU batch processing

ABE Integration:
- Exports prompts for ABE parallel execution on high-GPU machines
- Integrates with ABE QA system for validation
- Uploads results to Supabase node_interviews table
- Supports both batch export and individual generation

Usage:
    python generate_node_interviews.py --export-tasks    # Export tasks from Supabase
    python generate_node_interviews.py --export-prompts  # Export ABE-ready prompt files
    python generate_node_interviews.py --generate        # Generate using local LLM/API
    python generate_node_interviews.py --import-results  # Import ABE batch results
    python generate_node_interviews.py --upload          # Upload to Supabase
"""

import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional
import argparse

# Supabase connection
SUPABASE_URL = os.environ.get("SUPABASE_URL", "")
SUPABASE_KEY = os.environ.get("SUPABASE_SERVICE_KEY", "")

# Output directories
OUTPUT_DIR = Path(__file__).parent / "output"
TASKS_FILE = OUTPUT_DIR / "ctas_tasks.json"
ABE_PROMPTS_DIR = OUTPUT_DIR / "abe_prompts"  # Individual prompt files for ABE
ABE_RESULTS_DIR = OUTPUT_DIR / "abe_results"   # ABE batch results
INTERVIEWS_FILE = OUTPUT_DIR / "node_interviews.json"

# ABE System Integration
ABE_SYSTEMS_PATH = Path("/Users/cp5337/Developer/ABE-organized-systems")
ABE_QA_SYSTEM = ABE_SYSTEMS_PATH / "abe-qa-system"

# RFC-9025 Voice Template
VOICE_TEMPLATE = """I am {task_name}. I am {role_description}.

I {primary_action} using {specific_tools}. I {secondary_action} through {methods}.

You have seen me in {apt_example_1} where {apt_detail_1}. You have seen me in {apt_example_2} where {apt_detail_2}. I was {historical_role} in {famous_incidents}.

My indicators are {network_indicators}. {behavioral_indicators}. {temporal_patterns}. I try to {evasion_technique}, but if you're {detection_method}, you'll see {detection_signature}.

My success means {success_outcome}. My failure means {failure_outcome}. I feed {downstream_tasks} and enable {dependent_tasks}. Without me, {consequence_of_absence}."""

# System prompt for node interview generation
SYSTEM_PROMPT = """You are an expert threat intelligence analyst and red team operator creating node interviews for the CTAS (Cognitive Threat Analysis System) v7.3.1.

Your task is to generate a first-person adversary narrative for a specific attack task. The node SPEAKS IN FIRST PERSON as if it were the adversary capability itself.

CRITICAL REQUIREMENTS:
1. The voice must be SPECIFIC - reference actual tools, techniques, and historical incidents
2. Include real MITRE ATT&CK technique IDs (e.g., T1595.001, T1190)
3. Include real MITRE D3FEND countermeasure IDs (e.g., D3-NTA, D3-SYSM)
4. Reference real APT campaigns and incidents (e.g., APT29/SolarWinds, APT28/DNC)
5. Specify actual Kali tools, OSINT sources, and commercial detection products
6. Detection indicators must be technically accurate and actionable
7. The narrative must serve BOTH adversary emulation (2n perspective) AND defense (1n perspective)

HD4 PHASE CONTEXT:
- Hunt: Reconnaissance, intelligence gathering, target selection
- Detect: Active exploitation, initial access, establishing presence
- Disable: Persistence, privilege escalation, credential access
- Disrupt: Lateral movement, command and control, data staging
- Dominate: Exfiltration, impact, cover tracks, achieve objectives

OUTPUT FORMAT: Return valid JSON matching the schema exactly."""

def get_user_prompt(task: dict) -> str:
    """Generate the user prompt for a specific task.

    Full RFC-9025 + GLAF compliant schema including:
    - Core semantic fields (voice, purpose, ownership)
    - TTL task labels (mandatory/desirable/optional)
    - MITRE mappings (ATT&CK, D3FEND)
    - Dual perspective (1n defender, 2n adversary)
    - GLAF algorithm inputs (risk dimensions, behavioral sequence)
    - Graph relationship fields
    """
    # Note: primitive_type dropped - 60% are "Unclassified" (no analytical value)
    # Focus on hd4_phase and category which provide meaningful tactical context
    return f"""Generate a node interview for this CTAS task:

TASK DETAILS:
- task_id: {task['task_id']}
- task_seq: {task['task_seq']}
- task_name: {task['task_name']}
- category: {task['category']}
- hd4_phase: {task['hd4_phase']}
- description: {task['description']}
- predecessors: {json.dumps(task.get('predecessors') or [])}
- successors: {json.dumps(task.get('successors') or [])}

Return a JSON object with this EXACT structure (RFC-9025 + GLAF compliant):
{{
    "task_id": "{task['task_id']}",
    "task_seq": {task['task_seq']},
    "hd4_phase": "{task.get('hd4_phase', 'Hunt')}",
    "category": "{task.get('category', '')}",

    "voice": "I am {task['task_name']}. I am [role]. I [action] using [tools]... [full first-person narrative 3-5 paragraphs]",
    "purpose": "[What this task exists to accomplish - one sentence]",
    "ownership": "[What this task owns in the attack/defense chain - one sentence]",

    "perspective_1n": "[Defender view: What am I seeing? How do I detect/prevent this?]",
    "perspective_2n": "[Adversary view: Why do I need this? What does success look like?]",

    "needs_required": ["[Capability 1]", "[Capability 2]", "[Capability 3]"],
    "needs_optional": ["[Nice-to-have 1]", "[Nice-to-have 2]"],

    "counters_detection": ["[How defenders detect this - specific]", "[Detection method 2]"],
    "counters_prevention": ["[How to prevent this - specific]", "[Prevention method 2]"],

    "outcomes_success": ["[What success looks like]", "[Success indicator 2]"],
    "outcomes_failure": ["[What failure looks like]", "[Failure indicator 2]"],

    "mitre_tactics": ["TA0043"],
    "mitre_techniques": ["T1595.001", "T1595.002"],
    "d3fend_countermeasures": ["D3-NTA", "D3-SYSM"],

    "toolchain": {{
        "kali": [{{"tool": "nmap", "use": "Port scanning", "flags": "-sS -sV"}}],
        "osint": [{{"tool": "Shodan", "use": "Passive recon", "api": true}}],
        "commercial": [{{"tool": "Wazuh", "rule_category": "network_scan"}}]
    }},

    "indicators": {{
        "network": ["[Specific network indicator 1]"],
        "behavioral": ["[Behavioral pattern 1]"],
        "temporal": ["[Timing pattern 1]"]
    }},

    "eei": {{
        "detection": {{
            "priority": "high",
            "questions": ["What IP ranges are involved?", "What is the attack pattern?"],
            "collection_methods": ["Network flow logs", "IDS alerts"]
        }},
        "execution": {{
            "priority": "medium",
            "questions": ["What is the target?", "What tools are needed?"],
            "collection_methods": ["Active scanning", "OSINT"]
        }}
    }},

    "prerequisites": {json.dumps(task.get('predecessors') or [])},
    "enables": {json.dumps(task.get('successors') or [])},
    "combined_with": [],
    "related_tasks": ["[task_id of commonly combined task]"],

    "apt_examples": [{{"apt": "APT29", "campaign": "SolarWinds", "year": 2020, "note": "[Specific use]"}}],
    "case_studies": ["[Famous incident where this was used]"],

    "time_of_value": {{
        "collection_window": "hours to days",
        "actionable_window": "days to weeks",
        "decay_rate": "medium",
        "persistence_condition": "[When intel remains valid]",
        "refresh_trigger": "[What triggers re-collection]"
    }},

    "task_label": "[mandatory|desirable|optional]",
    "task_label_rationale": "[Why this classification]",
    "is_key_indicator": true,
    "is_interdiction_point": false,

    "target_sectors": ["[Critical infrastructure: Finance, Energy, Healthcare, etc.]"],
    "actor_types": ["[Actor type: Criminal, State-Sponsored, Hacktivist, Insider]"],
    "risk_dimensions": {{
        "likelihood": 0.7,
        "impact": 0.5,
        "detectability": 0.8,
        "reversibility": 0.3
    }},
    "behavioral_sequence": ["[Observable action 1]", "[Observable action 2]", "[Observable action 3]"],
    "keywords": ["[searchable term 1]", "[searchable term 2]"],
    "synonyms": ["[alternative name 1]", "[alternative name 2]"]
}}

IMPORTANT:
- The voice field must be a compelling first-person narrative (3-5 paragraphs)
- All MITRE IDs must be real and accurate for this task type
- Tool names must be real tools that would be used for this task
- APT examples should be historically accurate
- Be SPECIFIC, not generic
- perspective_1n and perspective_2n should be distinct viewpoints

TASK LABEL GUIDANCE:
- mandatory: Absolutely and logically necessary to carry out the attack
- desirable: Adds significant value (improves OPSEC, enhances effectiveness, boosts probability of success)
- optional: Reflects operational preferences (delivery method, detonation type, etc.)
- is_key_indicator: Activity indicating significant probability of developing plot - merits investigation
- is_interdiction_point: Point where law enforcement can intervene (often lesser crimes like fraud/theft)

RISK DIMENSIONS GUIDANCE (for GLAF teth.entropy):
- likelihood: Probability this task will be attempted (0.0-1.0)
- impact: Severity if successful (0.0-1.0)
- detectability: How hard to detect (0.0=easy, 1.0=very hard)
- reversibility: How hard to undo damage (0.0=easy, 1.0=permanent)

BEHAVIORAL SEQUENCE GUIDANCE (for GLAF lstar.learn):
- List 3-5 observable actions in temporal order
- These are what sensors/logs would see, not internal adversary steps"""


def create_abe_prompt_file(task: dict) -> dict:
    """Create an ABE-ready prompt structure for a single task."""
    return {
        "task_id": task['task_id'],
        "task_seq": task['task_seq'],
        "task_name": task['task_name'],
        "system_prompt": SYSTEM_PROMPT,
        "user_prompt": get_user_prompt(task),
        "model_config": {
            "model": "claude-sonnet-4-20250514",  # or local LLM
            "max_tokens": 4096,
            "temperature": 0.7
        },
        "expected_output_schema": "node_interview_rfc9025",
        "hd4_phase": task.get('hd4_phase', 'Unknown'),
        "category": task.get('category', 'Unknown')
    }


def export_abe_prompts(tasks: list):
    """Export individual prompt files for ABE parallel processing."""
    ABE_PROMPTS_DIR.mkdir(parents=True, exist_ok=True)

    manifest = {
        "generated_at": datetime.now().isoformat(),
        "total_tasks": len(tasks),
        "prompt_files": [],
        "hd4_distribution": {},
        "category_distribution": {}
    }

    for task in tasks:
        prompt_data = create_abe_prompt_file(task)

        # Save individual prompt file
        filename = f"prompt_{task['task_id']}.json"
        filepath = ABE_PROMPTS_DIR / filename

        with open(filepath, 'w') as f:
            json.dump(prompt_data, f, indent=2, default=str)

        manifest["prompt_files"].append({
            "file": filename,
            "task_id": task['task_id'],
            "task_name": task['task_name'],
            "hd4_phase": task.get('hd4_phase', 'Unknown')
        })

        # Track distributions
        phase = task.get('hd4_phase', 'Unknown')
        manifest["hd4_distribution"][phase] = manifest["hd4_distribution"].get(phase, 0) + 1

        category = task.get('category', 'Unknown')
        manifest["category_distribution"][category] = manifest["category_distribution"].get(category, 0) + 1

    # Save manifest
    manifest_file = ABE_PROMPTS_DIR / "manifest.json"
    with open(manifest_file, 'w') as f:
        json.dump(manifest, f, indent=2)

    print(f"Exported {len(tasks)} ABE prompt files to {ABE_PROMPTS_DIR}")
    print(f"Manifest: {manifest_file}")
    print(f"\nHD4 Distribution:")
    for phase, count in sorted(manifest["hd4_distribution"].items()):
        print(f"  {phase}: {count}")

    # Create ABE run script
    run_script = ABE_PROMPTS_DIR / "run_abe_batch.sh"
    with open(run_script, 'w') as f:
        f.write("""#!/bin/bash
# ABE Node Interview Batch Processing Script
# Run on high-GPU machine

set -e

PROMPT_DIR="$(dirname "$0")"
RESULTS_DIR="${PROMPT_DIR}/../abe_results"
mkdir -p "$RESULTS_DIR"

echo "🚀 ABE Node Interview Batch Processing"
echo "📂 Prompts: $PROMPT_DIR"
echo "📊 Results: $RESULTS_DIR"

# Process all prompt files in parallel
for prompt_file in "$PROMPT_DIR"/prompt_*.json; do
    task_id=$(basename "$prompt_file" .json | sed 's/prompt_//')
    echo "Processing: $task_id"

    # Extract prompts and call LLM
    # This is where ABE would invoke the model
    # python3 abe_llm_caller.py "$prompt_file" > "$RESULTS_DIR/result_${task_id}.json" &
done

wait
echo "✅ Batch processing complete"
""")

    print(f"\nRun script: {run_script}")
    print("Execute on high-GPU machine with: bash run_abe_batch.sh")


def export_tasks_for_batch():
    """Export all tasks from Supabase for ABE processing."""
    try:
        from supabase import create_client

        client = create_client(SUPABASE_URL, SUPABASE_KEY)
        response = client.table("ctas_tasks").select("*").order("task_seq").execute()
        tasks = response.data

        OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

        # Save raw tasks
        with open(TASKS_FILE, 'w') as f:
            json.dump(tasks, f, indent=2, default=str)
        print(f"Exported {len(tasks)} tasks to {TASKS_FILE}")

        return tasks

    except ImportError:
        print("Error: supabase-py not installed. Run: pip install supabase")
        sys.exit(1)


def export_tasks_for_local():
    """Export tasks as JSON for local processing without Supabase."""
    # Fetch from Supabase or use cached file
    if TASKS_FILE.exists():
        with open(TASKS_FILE, 'r') as f:
            tasks = json.load(f)
        print(f"Loaded {len(tasks)} tasks from cache")
        return tasks
    else:
        print("No cached tasks. Run with --export-tasks first or ensure Supabase connection.")
        return []


def generate_single_interview(task: dict, api_key: str = None, provider: str = "vertex") -> Optional[dict]:
    """Generate a single node interview using LLM API.

    Args:
        task: Task data from ctas_tasks table
        api_key: API key for the provider (not needed for vertex)
        provider: "vertex" (uses gcloud auth), "gemini" (needs API key), or "anthropic"
    """

    try:
        if provider == "vertex":
            # Use Vertex AI with gcloud auth - no API key needed
            import vertexai
            from vertexai.generative_models import GenerativeModel, GenerationConfig

            vertexai.init(project="ctas-7", location="us-central1")
            model = GenerativeModel(
                model_name="gemini-2.5-flash-preview-05-20",
                system_instruction=SYSTEM_PROMPT
            )

            response = model.generate_content(
                get_user_prompt(task),
                generation_config=GenerationConfig(
                    max_output_tokens=4096,
                    temperature=0.7
                )
            )
            content = response.text

        elif provider == "gemini":
            import google.generativeai as genai

            genai.configure(api_key=api_key)
            model = genai.GenerativeModel(
                model_name="gemini-2.5-flash-preview-05-20",
                system_instruction=SYSTEM_PROMPT
            )

            response = model.generate_content(
                get_user_prompt(task),
                generation_config=genai.types.GenerationConfig(
                    max_output_tokens=4096,
                    temperature=0.7
                )
            )
            content = response.text

        else:  # anthropic
            import anthropic

            client = anthropic.Anthropic(api_key=api_key)
            response = client.messages.create(
                model="claude-sonnet-4-20250514",
                max_tokens=4096,
                system=SYSTEM_PROMPT,
                messages=[
                    {"role": "user", "content": get_user_prompt(task)}
                ]
            )
            content = response.content[0].text

        # Try to extract JSON from response
        if "```json" in content:
            json_str = content.split("```json")[1].split("```")[0]
        elif "```" in content:
            json_str = content.split("```")[1].split("```")[0]
        else:
            json_str = content

        return json.loads(json_str.strip())

    except Exception as e:
        print(f"Error generating interview for {task['task_id']}: {e}")
        return None


def import_abe_results():
    """Import batch results from ABE processing."""
    ABE_RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    if not ABE_RESULTS_DIR.exists():
        print(f"ABE results directory not found: {ABE_RESULTS_DIR}")
        return []

    interviews = []
    errors = []

    # Process individual result files from ABE
    for result_file in ABE_RESULTS_DIR.glob("result_*.json"):
        try:
            with open(result_file, 'r') as f:
                result = json.load(f)

            # Handle different result formats
            if isinstance(result, dict):
                if "content" in result:
                    # Extract from LLM response format
                    content = result["content"]
                    if isinstance(content, list) and len(content) > 0:
                        content = content[0].get("text", "")
                elif "interview" in result:
                    interviews.append(result["interview"])
                    continue
                else:
                    # Direct interview data
                    interviews.append(result)
                    continue

                # Parse JSON from LLM text response
                if isinstance(content, str):
                    if "```json" in content:
                        json_str = content.split("```json")[1].split("```")[0]
                    elif "```" in content:
                        json_str = content.split("```")[1].split("```")[0]
                    else:
                        json_str = content

                    interview = json.loads(json_str.strip())
                    interviews.append(interview)

        except json.JSONDecodeError as e:
            errors.append({"file": result_file.name, "error": f"JSON parse error: {e}"})
        except Exception as e:
            errors.append({"file": result_file.name, "error": str(e)})

    # Save combined interviews
    with open(INTERVIEWS_FILE, 'w') as f:
        json.dump(interviews, f, indent=2)

    print(f"Imported {len(interviews)} interviews from ABE results")
    print(f"Saved to: {INTERVIEWS_FILE}")

    if errors:
        print(f"\nErrors: {len(errors)}")
        for e in errors[:10]:
            print(f"  - {e['file']}: {e['error']}")

    return interviews


def upload_to_supabase():
    """Upload generated interviews to Supabase."""
    if not INTERVIEWS_FILE.exists():
        print(f"Interviews file not found: {INTERVIEWS_FILE}")
        return

    try:
        from supabase import create_client

        client = create_client(SUPABASE_URL, SUPABASE_KEY)

        with open(INTERVIEWS_FILE, 'r') as f:
            interviews = json.load(f)

        # Batch upsert
        for interview in interviews:
            try:
                client.table("node_interviews").upsert(interview).execute()
                print(f"Uploaded: {interview['task_id']}")
            except Exception as e:
                print(f"Error uploading {interview['task_id']}: {e}")

        print(f"Uploaded {len(interviews)} interviews to Supabase")

    except ImportError:
        print("Error: supabase-py not installed")


def export_slotgraph():
    """Export interviews to SlotGraph format (Cypher + SurrealQL)."""
    if not INTERVIEWS_FILE.exists():
        print(f"Interviews file not found: {INTERVIEWS_FILE}")
        return

    with open(INTERVIEWS_FILE, 'r') as f:
        interviews = json.load(f)

    slotgraph_dir = OUTPUT_DIR / "slotgraph"
    slotgraph_dir.mkdir(parents=True, exist_ok=True)

    # Generate Cypher statements
    cypher_nodes = []
    cypher_edges = []

    for interview in interviews:
        task_id = interview.get('task_id', '')

        # Node creation
        node_props = {
            'task_id': task_id,
            'task_seq': interview.get('task_seq'),
            'hd4_phase': interview.get('hd4_phase', ''),
            'task_label': interview.get('task_label', 'optional'),
            'is_key_indicator': interview.get('is_key_indicator', False),
            'is_interdiction_point': interview.get('is_interdiction_point', False),
            'voice': interview.get('voice', '')[:500],  # Truncate for graph
            'purpose': interview.get('purpose', ''),
            'ownership': interview.get('ownership', ''),
            'mitre_tactics': interview.get('mitre_tactics', []),
            'mitre_techniques': interview.get('mitre_techniques', []),
        }
        cypher_nodes.append(f"CREATE (n:NodeInterview {json.dumps(node_props)})")

        # Edge creation from prerequisites
        for prereq in interview.get('prerequisites', []) or []:
            cypher_edges.append(
                f"MATCH (a:NodeInterview {{task_id: '{prereq}'}}), "
                f"(b:NodeInterview {{task_id: '{task_id}'}}) "
                f"CREATE (a)-[:ENABLES]->(b)"
            )

        # Edge creation from enables
        for enables in interview.get('enables', []) or []:
            cypher_edges.append(
                f"MATCH (a:NodeInterview {{task_id: '{task_id}'}}), "
                f"(b:NodeInterview {{task_id: '{enables}'}}) "
                f"CREATE (a)-[:ENABLES]->(b)"
            )

        # Edge creation from related_tasks
        for related in interview.get('related_tasks', []) or []:
            cypher_edges.append(
                f"MATCH (a:NodeInterview {{task_id: '{task_id}'}}), "
                f"(b:NodeInterview {{task_id: '{related}'}}) "
                f"CREATE (a)-[:RELATED_TO]->(b)"
            )

    # Write Cypher file
    cypher_file = slotgraph_dir / "node_interviews.cypher"
    with open(cypher_file, 'w') as f:
        f.write("// SlotGraph Node Interview Import - Cypher\n")
        f.write(f"// Generated: {datetime.now().isoformat()}\n")
        f.write(f"// Total Nodes: {len(interviews)}\n\n")
        f.write("// === NODES ===\n")
        f.write(";\n".join(cypher_nodes))
        f.write(";\n\n// === EDGES ===\n")
        f.write(";\n".join(cypher_edges))
        f.write(";\n")

    # Generate SurrealQL statements
    surql_file = slotgraph_dir / "node_interviews.surql"
    with open(surql_file, 'w') as f:
        f.write("-- SlotGraph Node Interview Import - SurrealQL\n")
        f.write(f"-- Generated: {datetime.now().isoformat()}\n")
        f.write(f"-- Total Nodes: {len(interviews)}\n\n")

        for interview in interviews:
            task_id = interview.get('task_id', '').replace('-', '_')
            f.write(f"CREATE node_interview:{task_id} SET\n")
            f.write(f"    task_id = '{interview.get('task_id', '')}',\n")
            f.write(f"    task_seq = {interview.get('task_seq', 0)},\n")
            f.write(f"    hd4_phase = '{interview.get('hd4_phase', '')}',\n")
            f.write(f"    task_label = '{interview.get('task_label', 'optional')}',\n")
            f.write(f"    is_key_indicator = {str(interview.get('is_key_indicator', False)).lower()},\n")
            f.write(f"    is_interdiction_point = {str(interview.get('is_interdiction_point', False)).lower()},\n")
            f.write(f"    purpose = '{(interview.get('purpose', '') or '').replace(chr(39), chr(39)+chr(39))}',\n")
            f.write(f"    ownership = '{(interview.get('ownership', '') or '').replace(chr(39), chr(39)+chr(39))}';\n\n")

        f.write("\n-- === EDGES ===\n")
        for interview in interviews:
            src_id = interview.get('task_id', '').replace('-', '_')
            for enables in interview.get('enables', []) or []:
                tgt_id = enables.replace('-', '_')
                f.write(f"RELATE node_interview:{src_id}->enables->node_interview:{tgt_id} SET weight = 1.0;\n")

    print(f"Exported SlotGraph files to {slotgraph_dir}")
    print(f"  - Cypher: {cypher_file}")
    print(f"  - SurrealQL: {surql_file}")


def main():
    parser = argparse.ArgumentParser(description="ABE Node Interview Generator")
    parser.add_argument("--export-tasks", action="store_true", help="Export tasks from Supabase")
    parser.add_argument("--export-prompts", action="store_true", help="Export ABE-ready prompt files")
    parser.add_argument("--generate", action="store_true", help="Generate interviews using local LLM/API")
    parser.add_argument("--import-results", action="store_true", help="Import ABE batch results")
    parser.add_argument("--upload", action="store_true", help="Upload to Supabase node_interviews table")
    parser.add_argument("--export-slotgraph", action="store_true", help="Export to SlotGraph (Cypher + SurrealQL)")
    parser.add_argument("--task-id", type=str, help="Generate single task by ID")
    parser.add_argument("--limit", type=int, default=164, help="Limit number of tasks")
    parser.add_argument("--hd4-phase", type=str, help="Filter by HD4 phase (Hunt/Detect/Disable/Disrupt/Dominate)")

    args = parser.parse_args()

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    print("=" * 60)
    print("ABE Node Interview Generator - RFC-9025 Compliant")
    print("=" * 60)

    if args.export_tasks:
        # Step 1: Export tasks from Supabase
        print("\n📥 Exporting tasks from Supabase...")
        tasks = export_tasks_for_batch()
        print(f"✅ Exported {len(tasks)} tasks")

    elif args.export_prompts:
        # Step 2: Generate ABE prompt files
        print("\n📝 Generating ABE prompt files...")
        tasks = export_tasks_for_local()

        if args.hd4_phase:
            tasks = [t for t in tasks if t.get('hd4_phase', '').lower() == args.hd4_phase.lower()]
            print(f"Filtered to {len(tasks)} tasks in {args.hd4_phase} phase")

        tasks = tasks[:args.limit]
        export_abe_prompts(tasks)

    elif args.import_results:
        # Step 3: Import ABE results
        print("\n📊 Importing ABE batch results...")
        interviews = import_abe_results()
        print(f"✅ Imported {len(interviews)} interviews")

    elif args.upload:
        # Step 4: Upload to Supabase
        print("\n☁️  Uploading to Supabase...")
        upload_to_supabase()

    elif args.export_slotgraph:
        # Step 5: Export to SlotGraph
        print("\n🔗 Exporting to SlotGraph format...")
        export_slotgraph()

    elif args.generate:
        # Alternative: Direct generation using API
        # Default to Gemini (uses credentials vault)
        provider = os.environ.get("LLM_PROVIDER", "gemini").lower()
        api_key = None

        # Load from credentials vault
        vault_path = Path(__file__).parent.parent / "credentials-vault" / "command-center-credentials.json"
        vault_keys = {}
        if vault_path.exists():
            with open(vault_path, 'r') as f:
                vault = json.load(f)
                for key_name, entries in vault.get("credentials", {}).items():
                    if entries and isinstance(entries, list):
                        best = max(entries, key=lambda x: x.get("confidence", 0))
                        vault_keys[key_name] = best.get("value")

        if provider == "gemini":
            api_key = os.environ.get("GOOGLE_API_KEY") or os.environ.get("GEMINI_API_KEY") or vault_keys.get("GEMINI_API_KEY")
            if not api_key:
                print("Error: GEMINI_API_KEY not found in env or credentials vault")
                sys.exit(1)
            print("\n🤖 Generating interviews using Gemini API...")
        else:
            api_key = os.environ.get("ANTHROPIC_API_KEY")
            if not api_key:
                print("Error: ANTHROPIC_API_KEY not set")
                sys.exit(1)
            print("\n🤖 Generating interviews using Claude Sonnet...")

        tasks = export_tasks_for_local()
        if args.task_id:
            tasks = [t for t in tasks if t['task_id'] == args.task_id]

        if args.hd4_phase:
            tasks = [t for t in tasks if t.get('hd4_phase', '').lower() == args.hd4_phase.lower()]

        tasks = tasks[:args.limit]
        interviews = []

        for i, task in enumerate(tasks):
            print(f"Generating {i+1}/{len(tasks)}: {task['task_name']}")
            interview = generate_single_interview(task, api_key, provider=provider)
            if interview:
                interviews.append(interview)

        with open(INTERVIEWS_FILE, 'w') as f:
            json.dump(interviews, f, indent=2)

        print(f"\n✅ Generated {len(interviews)} interviews")
        print(f"📁 Saved to: {INTERVIEWS_FILE}")

    else:
        parser.print_help()
        print("\n" + "=" * 60)
        print("ABE Workflow:")
        print("=" * 60)
        print("1. python generate_node_interviews.py --export-tasks")
        print("   → Fetches 164 tasks from Supabase ctas_tasks table")
        print("")
        print("2. python generate_node_interviews.py --export-prompts")
        print("   → Creates individual prompt files for ABE batch processing")
        print("")
        print("3. [Run ABE batch on high-GPU machine]")
        print("   → bash output/abe_prompts/run_abe_batch.sh")
        print("")
        print("4. python generate_node_interviews.py --import-results")
        print("   → Imports results from output/abe_results/")
        print("")
        print("5. python generate_node_interviews.py --upload")
        print("   → Uploads interviews to Supabase node_interviews table")
        print("=" * 60)


if __name__ == "__main__":
    main()
