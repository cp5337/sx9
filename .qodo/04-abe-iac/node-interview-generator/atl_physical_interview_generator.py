#!/usr/bin/env python3
"""
ATL-Physical Interview Generator
================================

Generates RFC-9025 compliant node interviews for ATL-Physical adversary tasks.
Interviews are stored in the ATL-Physical Neo4j container alongside task nodes.

Physical Domain Context:
- IED attack phases (procurement, assembly, emplacement, detonation)
- Interdiction points for Left-of-Bang intervention
- Key indicators with mundanity scores
- HD4 phase mapping (HUNT through DOMINATE)

Container: neo4j-atl-physical
Ports: 7475 (browser), 7688 (bolt)

Usage:
    python atl_physical_interview_generator.py --export-prompts  # Export for ABE batch
    python atl_physical_interview_generator.py --generate        # Generate with API
    python atl_physical_interview_generator.py --load-to-neo4j   # Store in Neo4j
    python atl_physical_interview_generator.py --stats           # Show statistics
"""

import json
import os
import argparse
import logging
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Any

try:
    from neo4j import GraphDatabase
    HAS_NEO4J = True
except ImportError:
    HAS_NEO4J = False

try:
    import google.generativeai as genai
    HAS_GENAI = True
except ImportError:
    HAS_GENAI = False

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# ATL-Physical Neo4j connection
ATL_NEO4J_URI = "bolt://localhost:7688"
ATL_NEO4J_USER = "neo4j"
ATL_NEO4J_PASSWORD = "atl_physical_graph"

# Output paths
OUTPUT_DIR = Path(__file__).parent / "output"
ATL_INTERVIEWS_DIR = OUTPUT_DIR / "atl_physical_interviews"
ATL_PROMPTS_DIR = OUTPUT_DIR / "atl_physical_prompts"

# RFC-9025 Physical Domain Voice Template
PHYSICAL_VOICE_TEMPLATE = """I am {task_name}. I am {role_description}.

I {primary_action} using {methods_and_materials}. I {secondary_action} through {operational_security}.

You have seen me in {incident_1} where {incident_detail_1}. You have seen me in {incident_2} where {incident_detail_2}. I was {historical_context} in {famous_attacks}.

My indicators are {observable_indicators}. {behavioral_patterns}. {temporal_signatures}. I try to {concealment_method}, but if you're {detection_approach}, you'll see {detection_signature}.

My success means {success_outcome}. My failure means {failure_outcome}. I feed {downstream_tasks} and enable {dependent_phases}. Without me, {consequence_of_absence}.

{interdiction_guidance}"""

# System prompt for physical domain interviews
SYSTEM_PROMPT = """You are an expert counter-IED analyst and physical security specialist creating node interviews for the ATL-Physical (Adversary Task List - Physical Domain) system.

Your task is to generate a first-person adversary narrative for a specific physical attack task. The node SPEAKS IN FIRST PERSON as if it were the adversary capability itself.

CRITICAL REQUIREMENTS:
1. The voice must be SPECIFIC - reference actual materials, methods, and historical incidents
2. Include real TTPs from IED, active shooter, and physical intrusion domains
3. Reference real incidents (Oklahoma City, Boston Marathon, Mumbai, etc.)
4. Specify actual detection methods (K-9, X-ray, behavioral analysis, CCTV)
5. Detection indicators must be technically accurate and actionable
6. Include "Left of Bang" interdiction opportunities where applicable
7. The narrative must serve BOTH adversary emulation AND physical security defense

HD4 PHASE CONTEXT (Physical Domain):
- HUNT: Surveillance, reconnaissance, target selection, vulnerability assessment
- DETECT: Material acquisition, financing, logistics, communication
- DISABLE: Assembly, testing, rehearsal, staging
- DISRUPT: Movement to target, emplacement, final preparations
- DOMINATE: Execution, escape, secondary attacks, post-incident

PHYSICAL DOMAIN SPECIFICS:
- Mundanity Score: How "normal" the activity appears (0.0 = obviously suspicious, 1.0 = completely mundane)
- Interdiction Points: Where intervention can disrupt the attack chain
- Key Indicators: Observable behaviors that distinguish hostile from benign
- Node Form: 1n (single actor), 2n (cell), 3n (network)

OUTPUT FORMAT: Return valid JSON matching the schema exactly."""


def get_physical_user_prompt(task: Dict) -> str:
    """Generate user prompt for ATL-Physical task interview."""
    return f"""Generate a node interview for this ATL-Physical adversary task:

TASK DETAILS:
- task_id: {task.get('task_id', '')}
- title: {task.get('title', '')}
- description: {task.get('description', '')}
- phase: {task.get('phase', 0)}
- classification: {task.get('classification', 'OPTIONAL')}
- modality: {task.get('modality', 'IED')}
- hd4_phases: {json.dumps(task.get('hd4_phases', []))}
- is_interdiction_point: {task.get('is_interdiction_point', False)}
- is_key_indicator: {task.get('is_key_indicator', False)}
- mundanity_score: {task.get('mundanity_score', 0.5)}
- node_form: {task.get('node_form', '1n')}
- parent_task: {task.get('parent_task', '')}
- related_tasks: {json.dumps(task.get('related_tasks', []))}

Generate a JSON response with this EXACT structure:
{{
    "task_id": "{task.get('task_id', '')}",
    "domain": "physical",
    "modality": "{task.get('modality', 'IED')}",
    "voice": "<first-person narrative following the template>",
    "purpose": "<what this task accomplishes in the attack chain>",
    "ownership": {{
        "actor_types": ["<lone wolf|cell|network|state-sponsored>"],
        "skill_level": "<low|medium|high|expert>",
        "resource_requirements": "<minimal|moderate|substantial|extensive>"
    }},
    "ttl_classification": "{task.get('classification', 'OPTIONAL')}",
    "phase_in_chain": {task.get('phase', 0)},
    "hd4_mapping": {{
        "primary_phase": "<HUNT|DETECT|DISABLE|DISRUPT|DOMINATE>",
        "secondary_phases": ["<other applicable phases>"]
    }},
    "indicators": {{
        "observable": ["<what can be seen/detected>"],
        "behavioral": ["<suspicious behaviors>"],
        "temporal": ["<timing patterns>"],
        "material": ["<physical evidence>"]
    }},
    "detection_methods": {{
        "technical": ["<sensors, cameras, screening>"],
        "human": ["<behavioral analysis, informants, tips>"],
        "procedural": ["<background checks, verification>"]
    }},
    "interdiction": {{
        "is_interdiction_point": {str(task.get('is_interdiction_point', False)).lower()},
        "intervention_methods": ["<how to disrupt at this point>"],
        "window_of_opportunity": "<how long intervention is possible>",
        "consequences_of_miss": "<what happens if not interdicted>"
    }},
    "mundanity_analysis": {{
        "score": {task.get('mundanity_score', 0.5)},
        "cover_activities": ["<legitimate activities this resembles>"],
        "distinguishing_factors": ["<what separates hostile from benign>"]
    }},
    "historical_examples": {{
        "incidents": ["<real attacks where this task was observed>"],
        "lessons_learned": ["<what these incidents taught us>"]
    }},
    "dependencies": {{
        "requires": ["<prerequisite tasks>"],
        "enables": ["<downstream tasks>"],
        "alternatives": ["<substitute methods>"]
    }},
    "countermeasures": {{
        "preventive": ["<stop before it happens>"],
        "detective": ["<identify when it happens>"],
        "responsive": ["<react after detection>"]
    }}
}}"""


class ATLPhysicalInterviewGenerator:
    """Generate and manage ATL-Physical node interviews."""

    def __init__(
        self,
        neo4j_uri: str = ATL_NEO4J_URI,
        neo4j_user: str = ATL_NEO4J_USER,
        neo4j_password: str = ATL_NEO4J_PASSWORD
    ):
        self.neo4j_uri = neo4j_uri
        self.neo4j_user = neo4j_user
        self.neo4j_password = neo4j_password
        self.driver = None

        # Ensure output directories exist
        ATL_INTERVIEWS_DIR.mkdir(parents=True, exist_ok=True)
        ATL_PROMPTS_DIR.mkdir(parents=True, exist_ok=True)

    def connect_neo4j(self):
        """Connect to ATL-Physical Neo4j container."""
        if not HAS_NEO4J:
            raise RuntimeError("neo4j driver not installed")
        self.driver = GraphDatabase.driver(
            self.neo4j_uri,
            auth=(self.neo4j_user, self.neo4j_password)
        )
        logger.info(f"Connected to ATL-Physical Neo4j at {self.neo4j_uri}")

    def close(self):
        """Close Neo4j connection."""
        if self.driver:
            self.driver.close()

    def get_tasks_from_neo4j(self) -> List[Dict]:
        """Fetch all ATL-Physical tasks from Neo4j."""
        if not self.driver:
            self.connect_neo4j()

        tasks = []
        with self.driver.session() as session:
            result = session.run("""
                MATCH (t:ATLPhysical:AdversaryTask)
                OPTIONAL MATCH (t)-[:MAPS_TO_HD4]->(p:HD4Phase)
                OPTIONAL MATCH (t)-[:SUBTASK_OF]->(parent:AdversaryTask)
                OPTIONAL MATCH (t)-[:RELATED_TO]->(related:AdversaryTask)
                WITH t,
                     collect(DISTINCT p.phase_name) as hd4_phases,
                     parent.task_id as parent_task,
                     collect(DISTINCT related.task_id) as related_tasks
                RETURN t.task_id as task_id,
                       t.title as title,
                       t.description as description,
                       t.phase as phase,
                       t.classification as classification,
                       t.modality as modality,
                       t.is_interdiction_point as is_interdiction_point,
                       t.is_key_indicator as is_key_indicator,
                       t.mundanity_score as mundanity_score,
                       t.node_form as node_form,
                       hd4_phases,
                       parent_task,
                       related_tasks
                ORDER BY t.phase, t.task_id
            """)
            for record in result:
                tasks.append(dict(record))

        logger.info(f"Fetched {len(tasks)} ATL-Physical tasks from Neo4j")
        return tasks

    def export_prompts(self) -> int:
        """Export ABE-ready prompt files for batch processing."""
        tasks = self.get_tasks_from_neo4j()
        count = 0

        for task in tasks:
            task_id = task.get('task_id', 'unknown')
            safe_id = task_id.replace('.', '_').replace('/', '_')

            prompt_data = {
                "task_id": task_id,
                "system_prompt": SYSTEM_PROMPT,
                "user_prompt": get_physical_user_prompt(task),
                "task_metadata": task
            }

            prompt_file = ATL_PROMPTS_DIR / f"{safe_id}_prompt.json"
            with open(prompt_file, 'w') as f:
                json.dump(prompt_data, f, indent=2, default=str)
            count += 1

        logger.info(f"Exported {count} ATL-Physical prompts to {ATL_PROMPTS_DIR}")
        return count

    def _load_api_key(self) -> Optional[str]:
        """Load API key from credentials vault or environment."""
        # Try vault first (same pattern as crate_interview_generator.py)
        vault_path = Path(__file__).parent.parent / "credentials-vault" / "command-center-credentials.json"
        if vault_path.exists():
            try:
                with open(vault_path, 'r') as f:
                    vault = json.load(f)
                    for key_name, entries in vault.get("credentials", {}).items():
                        if key_name == "GEMINI_API_KEY" and entries:
                            best = max(entries, key=lambda x: x.get("confidence", 0))
                            api_key = best.get("value")
                            if api_key:
                                logger.info("Loaded API key from credentials vault")
                                return api_key
            except Exception as e:
                logger.warning(f"Failed to load vault: {e}")

        # Fallback to environment
        api_key = os.environ.get("GOOGLE_API_KEY") or os.environ.get("GEMINI_API_KEY")
        if api_key:
            logger.info("Loaded API key from environment")
        return api_key

    def generate_interview(self, task: Dict) -> Optional[Dict]:
        """Generate interview using Gemini API."""
        if not HAS_GENAI:
            logger.warning("google-generativeai not installed")
            return None

        api_key = self._load_api_key()
        if not api_key:
            logger.error("No API key found in vault or environment")
            return None

        genai.configure(api_key=api_key)
        model = genai.GenerativeModel('gemini-2.0-flash-exp')

        try:
            response = model.generate_content(
                SYSTEM_PROMPT + "\n\n" + get_physical_user_prompt(task),
                generation_config=genai.GenerationConfig(
                    response_mime_type="application/json",
                    temperature=0.7
                )
            )

            interview = json.loads(response.text)
            interview['generated_at'] = datetime.now().isoformat()
            interview['generator'] = 'gemini-1.5-flash'
            return interview

        except Exception as e:
            logger.error(f"Generation failed for {task.get('task_id')}: {e}")
            return None

    def generate_all_interviews(self, limit: int = None) -> int:
        """Generate interviews for all tasks."""
        tasks = self.get_tasks_from_neo4j()
        if limit:
            tasks = tasks[:limit]

        count = 0
        for task in tasks:
            task_id = task.get('task_id', 'unknown')
            safe_id = task_id.replace('.', '_').replace('/', '_')

            output_file = ATL_INTERVIEWS_DIR / f"{safe_id}_interview.json"

            # Skip if already generated
            if output_file.exists():
                logger.info(f"Skipping {task_id} - already exists")
                continue

            interview = self.generate_interview(task)
            if interview:
                with open(output_file, 'w') as f:
                    json.dump(interview, f, indent=2)
                count += 1
                logger.info(f"Generated interview for {task_id}")

            # Rate limiting
            import time
            time.sleep(0.5)

        logger.info(f"Generated {count} new interviews")
        return count

    def load_interviews_to_neo4j(self) -> int:
        """Load generated interviews back to Neo4j as Interview nodes."""
        if not self.driver:
            self.connect_neo4j()

        count = 0
        interview_files = list(ATL_INTERVIEWS_DIR.glob("*_interview.json"))

        with self.driver.session() as session:
            # Create Interview node index
            try:
                session.run("""
                    CREATE INDEX atl_interview_task IF NOT EXISTS
                    FOR (i:ATLPhysical:Interview) ON (i.task_id)
                """)
            except:
                pass

            for interview_file in interview_files:
                try:
                    with open(interview_file) as f:
                        interview = json.load(f)

                    task_id = interview.get('task_id', '')
                    if not task_id:
                        continue

                    # Store interview as node linked to task
                    session.run("""
                        MATCH (t:ATLPhysical:AdversaryTask {task_id: $task_id})
                        MERGE (i:ATLPhysical:Interview {task_id: $task_id})
                        SET i.voice = $voice,
                            i.purpose = $purpose,
                            i.domain = 'physical',
                            i.modality = $modality,
                            i.full_interview = $full_interview,
                            i.generated_at = $generated_at,
                            i.generator = $generator,
                            i.updated_at = datetime()
                        MERGE (t)-[:HAS_INTERVIEW]->(i)
                    """,
                        task_id=task_id,
                        voice=interview.get('voice', '')[:5000],
                        purpose=interview.get('purpose', '')[:2000],
                        modality=interview.get('modality', 'IED'),
                        full_interview=json.dumps(interview)[:10000],
                        generated_at=interview.get('generated_at', ''),
                        generator=interview.get('generator', 'unknown')
                    )

                    # Store indicators as separate nodes for graph queries
                    indicators = interview.get('indicators', {})
                    for indicator_type, indicator_list in indicators.items():
                        if isinstance(indicator_list, list):
                            for indicator in indicator_list:
                                if indicator:
                                    session.run("""
                                        MATCH (i:ATLPhysical:Interview {task_id: $task_id})
                                        MERGE (ind:ATLPhysical:Indicator {
                                            text: $text,
                                            type: $type
                                        })
                                        MERGE (i)-[:HAS_INDICATOR]->(ind)
                                    """, task_id=task_id, text=indicator[:500], type=indicator_type)

                    count += 1
                    if count % 20 == 0:
                        logger.info(f"Loaded {count} interviews...")

                except Exception as e:
                    logger.warning(f"Failed to load {interview_file.name}: {e}")

        logger.info(f"Loaded {count} interviews to ATL-Physical Neo4j")
        return count

    def get_stats(self) -> Dict[str, Any]:
        """Get interview statistics from Neo4j."""
        if not self.driver:
            self.connect_neo4j()

        with self.driver.session() as session:
            result = session.run("""
                MATCH (t:ATLPhysical:AdversaryTask) WITH count(t) as total_tasks
                OPTIONAL MATCH (i:ATLPhysical:Interview) WITH total_tasks, count(i) as interviews
                OPTIONAL MATCH ()-[r:HAS_INTERVIEW]->() WITH total_tasks, interviews, count(r) as linked
                OPTIONAL MATCH (ind:ATLPhysical:Indicator)
                RETURN total_tasks, interviews, linked, count(ind) as indicators
            """)
            record = result.single()

            # Count local files
            prompt_count = len(list(ATL_PROMPTS_DIR.glob("*_prompt.json")))
            interview_count = len(list(ATL_INTERVIEWS_DIR.glob("*_interview.json")))

            return {
                "neo4j_tasks": record["total_tasks"] if record else 0,
                "neo4j_interviews": record["interviews"] if record else 0,
                "neo4j_linked": record["linked"] if record else 0,
                "neo4j_indicators": record["indicators"] if record else 0,
                "local_prompts": prompt_count,
                "local_interviews": interview_count
            }


def main():
    parser = argparse.ArgumentParser(description="ATL-Physical Interview Generator")
    parser.add_argument("--export-prompts", action="store_true", help="Export ABE-ready prompts")
    parser.add_argument("--generate", action="store_true", help="Generate interviews with API")
    parser.add_argument("--limit", type=int, help="Limit number of interviews to generate")
    parser.add_argument("--load-to-neo4j", action="store_true", help="Load interviews to Neo4j")
    parser.add_argument("--stats", action="store_true", help="Show statistics")
    parser.add_argument("--all", action="store_true", help="Run full pipeline")
    args = parser.parse_args()

    generator = ATLPhysicalInterviewGenerator()

    try:
        if args.stats:
            stats = generator.get_stats()
            print("\n" + "="*60)
            print("       ATL-Physical Interview Statistics")
            print("="*60)
            print(f"  Neo4j Tasks:           {stats['neo4j_tasks']:>6}")
            print(f"  Neo4j Interviews:      {stats['neo4j_interviews']:>6}")
            print(f"  Neo4j Linked:          {stats['neo4j_linked']:>6}")
            print(f"  Neo4j Indicators:      {stats['neo4j_indicators']:>6}")
            print("-"*60)
            print(f"  Local Prompts:         {stats['local_prompts']:>6}")
            print(f"  Local Interviews:      {stats['local_interviews']:>6}")
            print("="*60)
            return

        if args.export_prompts or args.all:
            generator.export_prompts()

        if args.generate or args.all:
            generator.generate_all_interviews(limit=args.limit)

        if args.load_to_neo4j or args.all:
            generator.load_interviews_to_neo4j()

        # Show final stats
        stats = generator.get_stats()
        print(f"\nFinal: {stats['neo4j_interviews']} interviews in Neo4j, "
              f"{stats['local_interviews']} local files")

    finally:
        generator.close()


if __name__ == "__main__":
    main()
