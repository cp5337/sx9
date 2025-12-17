#!/usr/bin/env python3
"""
Neo4j ATL-Physical Loader
=========================

Loads ATL-Physical (physical threat domain) data to a SEPARATE Neo4j container.
All nodes use :ATLPhysical label for future merging with main operational graph.

Container: neo4j-atl-physical
Ports: 7475 (browser), 7688 (bolt)
Auth: neo4j/atl_physical_graph

RFC-9011-A: Physical domain threat modeling
TTL Source: Terrorist IED Task List decomposition

Usage:
    python neo4j_atl_physical_loader.py --load
    python neo4j_atl_physical_loader.py --stats
"""

import yaml
import argparse
import logging
from pathlib import Path
from typing import Dict, List, Optional, Any
from datetime import datetime

try:
    from neo4j import GraphDatabase
    HAS_NEO4J = True
except ImportError:
    HAS_NEO4J = False
    print("WARNING: neo4j driver not installed. Run: pip install neo4j")

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# ATL-Physical data path (3 parent levels from this file)
ATL_PHYSICAL_PATH = Path(__file__).parent.parent.parent / "ctas-dir" / "20-atl" / "physical" / "data" / "atl_physical_ied.yaml"

# ATL-Physical container connection
ATL_PHYSICAL_URI = "bolt://localhost:7688"
ATL_PHYSICAL_USER = "neo4j"
ATL_PHYSICAL_PASSWORD = "atl_physical_graph"


class ATLPhysicalNeo4jLoader:
    """
    Load ATL-Physical threat data to dedicated Neo4j container.

    All nodes carry :ATLPhysical label for:
    1. Clear domain separation from cyber threat data
    2. Future MERGE capability into operational graph
    3. Training data isolation (invisible operationally)
    """

    def __init__(
        self,
        uri: str = ATL_PHYSICAL_URI,
        user: str = ATL_PHYSICAL_USER,
        password: str = ATL_PHYSICAL_PASSWORD
    ):
        if not HAS_NEO4J:
            raise RuntimeError("neo4j driver not installed")
        self.driver = GraphDatabase.driver(uri, auth=(user, password))
        logger.info(f"Connected to ATL-Physical Neo4j at {uri}")

    def close(self):
        self.driver.close()

    def create_atl_indexes(self):
        """Create indexes for ATL-Physical data querying."""
        with self.driver.session() as session:
            indexes = [
                # Primary ATL indexes with :ATLPhysical label
                "CREATE INDEX atl_task_id IF NOT EXISTS FOR (t:ATLPhysical:AdversaryTask) ON (t.task_id)",
                "CREATE INDEX atl_phase IF NOT EXISTS FOR (t:ATLPhysical:AdversaryTask) ON (t.phase)",
                "CREATE INDEX atl_modality IF NOT EXISTS FOR (t:ATLPhysical:AdversaryTask) ON (t.modality)",
                "CREATE INDEX atl_classification IF NOT EXISTS FOR (t:ATLPhysical:AdversaryTask) ON (t.classification)",
                "CREATE INDEX atl_interdiction IF NOT EXISTS FOR (t:ATLPhysical:AdversaryTask) ON (t.is_interdiction_point)",
                "CREATE INDEX atl_indicator IF NOT EXISTS FOR (t:ATLPhysical:AdversaryTask) ON (t.is_key_indicator)",
                # HD4 phase index
                "CREATE INDEX atl_hd4 IF NOT EXISTS FOR (p:ATLPhysical:HD4Phase) ON (p.phase_name)",
            ]
            for idx in indexes:
                try:
                    session.run(idx)
                except Exception as e:
                    logger.warning(f"Index creation: {e}")
            logger.info("Created ATL-Physical indexes")

    def load_atl_physical(self) -> Dict[str, int]:
        """Load ATL-Physical tasks and relationships to Neo4j."""
        if not ATL_PHYSICAL_PATH.exists():
            logger.error(f"ATL-Physical file not found: {ATL_PHYSICAL_PATH}")
            return {"tasks": 0, "relationships": 0, "hd4_links": 0}

        with open(ATL_PHYSICAL_PATH, 'r', encoding='utf-8') as f:
            data = yaml.safe_load(f)

        stats = {"tasks": 0, "relationships": 0, "hd4_links": 0}

        with self.driver.session() as session:
            # Create HD4 phase nodes
            hd4_phases = ["HUNT", "DETECT", "DISABLE", "DISRUPT", "DOMINATE"]
            for phase in hd4_phases:
                session.run("""
                    MERGE (p:ATLPhysical:HD4Phase {phase_name: $phase})
                    SET p.domain = 'physical',
                        p.created_at = datetime()
                """, phase=phase)

            # Create modality nodes
            modalities = set()
            for task in data.get('tasks', []):
                mods = task.get('modality', [])
                if isinstance(mods, list):
                    modalities.update(mods)
                elif mods:
                    modalities.add(mods)

            for mod in modalities:
                session.run("""
                    MERGE (m:ATLPhysical:ThreatModality {name: $name})
                    SET m.domain = 'physical'
                """, name=mod)

            # Load tasks with :ATLPhysical label
            for task in data.get('tasks', []):
                task_id = task.get('task_id', '')
                title = task.get('title', '')[:500]
                description = task.get('description', '')[:2000]
                phase = task.get('phase', 0)
                classification = task.get('classification', 'OPTIONAL')
                hd4_phases_list = task.get('hd4_phases', [])
                modality = task.get('modality', [])
                if isinstance(modality, list) and modality:
                    primary_modality = modality[0]
                elif isinstance(modality, str):
                    primary_modality = modality
                else:
                    primary_modality = 'IED'

                is_interdiction = task.get('is_interdiction_point', False)
                is_indicator = task.get('is_key_indicator', False)
                mundanity = task.get('mundanity_score', 0.5)
                node_form = task.get('node_form', '1n')
                parent_task = task.get('parent_task')

                # MERGE task with ATLPhysical label
                session.run("""
                    MERGE (t:ATLPhysical:AdversaryTask {task_id: $task_id})
                    SET t.title = $title,
                        t.description = $description,
                        t.phase = $phase,
                        t.classification = $classification,
                        t.modality = $modality,
                        t.is_interdiction_point = $is_interdiction,
                        t.is_key_indicator = $is_indicator,
                        t.mundanity_score = $mundanity,
                        t.node_form = $node_form,
                        t.domain = 'physical',
                        t.source = 'TTL',
                        t.updated_at = datetime()
                """, task_id=task_id, title=title, description=description,
                     phase=phase, classification=classification,
                     modality=primary_modality, is_interdiction=is_interdiction,
                     is_indicator=is_indicator, mundanity=mundanity,
                     node_form=node_form)
                stats["tasks"] += 1

                # Link to parent task
                if parent_task:
                    session.run("""
                        MATCH (child:ATLPhysical:AdversaryTask {task_id: $child_id})
                        MERGE (parent:ATLPhysical:AdversaryTask {task_id: $parent_id})
                        MERGE (child)-[:SUBTASK_OF]->(parent)
                    """, child_id=task_id, parent_id=parent_task)

                # Link to HD4 phases
                for hd4 in hd4_phases_list:
                    if hd4 in hd4_phases:
                        session.run("""
                            MATCH (t:ATLPhysical:AdversaryTask {task_id: $task_id})
                            MATCH (p:ATLPhysical:HD4Phase {phase_name: $phase})
                            MERGE (t)-[:MAPS_TO_HD4]->(p)
                        """, task_id=task_id, phase=hd4)
                        stats["hd4_links"] += 1

                # Link to modality
                session.run("""
                    MATCH (t:ATLPhysical:AdversaryTask {task_id: $task_id})
                    MERGE (m:ATLPhysical:ThreatModality {name: $modality})
                    MERGE (t)-[:HAS_MODALITY]->(m)
                """, task_id=task_id, modality=primary_modality)

                if stats["tasks"] % 20 == 0:
                    logger.info(f"  Loaded {stats['tasks']} tasks...")

            # Load relationships
            for rel in data.get('relationships', []):
                source = rel.get('source', '')
                target_title = rel.get('target_title', '')
                target_page = rel.get('target_page', 0)
                rel_type = rel.get('type', 'RELATED_TO')

                # Find target by title match or create placeholder
                session.run("""
                    MATCH (s:ATLPhysical:AdversaryTask {task_id: $source})
                    MERGE (t:ATLPhysical:AdversaryTask {title: $target_title})
                    ON CREATE SET t.task_id = 'REF-' + $source + '-' + toString($target_page),
                                  t.domain = 'physical',
                                  t.source = 'TTL-reference'
                    MERGE (s)-[:RELATED_TO {type: $rel_type, target_page: $target_page}]->(t)
                """, source=source, target_title=target_title,
                     target_page=target_page, rel_type=rel_type)
                stats["relationships"] += 1

        logger.info(f"Loaded ATL-Physical: {stats['tasks']} tasks, "
                   f"{stats['relationships']} relationships, {stats['hd4_links']} HD4 links")
        return stats

    def get_stats(self) -> Dict[str, Any]:
        """Get ATL-Physical graph statistics."""
        with self.driver.session() as session:
            result = session.run("""
                MATCH (t:ATLPhysical:AdversaryTask) WITH count(t) as tasks
                MATCH (m:ATLPhysical:ThreatModality) WITH tasks, count(m) as modalities
                MATCH (p:ATLPhysical:HD4Phase) WITH tasks, modalities, count(p) as hd4_phases
                MATCH ()-[r:SUBTASK_OF]->() WITH tasks, modalities, hd4_phases, count(r) as subtask_links
                MATCH ()-[h:MAPS_TO_HD4]->() WITH tasks, modalities, hd4_phases, subtask_links, count(h) as hd4_links
                MATCH ()-[rel:RELATED_TO]->() WITH tasks, modalities, hd4_phases, subtask_links, hd4_links, count(rel) as related_links
                MATCH (t:ATLPhysical:AdversaryTask) WHERE t.is_interdiction_point = true
                WITH tasks, modalities, hd4_phases, subtask_links, hd4_links, related_links, count(t) as interdiction_points
                MATCH (t:ATLPhysical:AdversaryTask) WHERE t.is_key_indicator = true
                RETURN tasks, modalities, hd4_phases, subtask_links, hd4_links, related_links, interdiction_points, count(t) as key_indicators
            """)
            record = result.single()
            return dict(record) if record else {}

    def get_interdiction_points(self) -> List[Dict]:
        """Get all interdiction points for tactical planning."""
        with self.driver.session() as session:
            result = session.run("""
                MATCH (t:ATLPhysical:AdversaryTask)
                WHERE t.is_interdiction_point = true
                OPTIONAL MATCH (t)-[:MAPS_TO_HD4]->(p:HD4Phase)
                RETURN t.task_id as task_id, t.title as title, t.phase as phase,
                       t.modality as modality, collect(p.phase_name) as hd4_phases
                ORDER BY t.phase
            """)
            return [dict(r) for r in result]

    def get_key_indicators(self) -> List[Dict]:
        """Get all key indicators for detection."""
        with self.driver.session() as session:
            result = session.run("""
                MATCH (t:ATLPhysical:AdversaryTask)
                WHERE t.is_key_indicator = true
                OPTIONAL MATCH (t)-[:MAPS_TO_HD4]->(p:HD4Phase)
                RETURN t.task_id as task_id, t.title as title, t.phase as phase,
                       t.mundanity_score as mundanity, collect(p.phase_name) as hd4_phases
                ORDER BY t.mundanity_score DESC
            """)
            return [dict(r) for r in result]


def main():
    parser = argparse.ArgumentParser(description="Neo4j ATL-Physical Loader")
    parser.add_argument("--uri", default=ATL_PHYSICAL_URI, help="Neo4j URI")
    parser.add_argument("--user", default=ATL_PHYSICAL_USER, help="Neo4j user")
    parser.add_argument("--password", default=ATL_PHYSICAL_PASSWORD, help="Neo4j password")
    parser.add_argument("--load", action="store_true", help="Load ATL-Physical data")
    parser.add_argument("--stats", action="store_true", help="Show statistics only")
    parser.add_argument("--interdiction", action="store_true", help="Show interdiction points")
    parser.add_argument("--indicators", action="store_true", help="Show key indicators")
    args = parser.parse_args()

    if not HAS_NEO4J:
        print("ERROR: Install neo4j driver: pip install neo4j")
        return

    loader = ATLPhysicalNeo4jLoader(uri=args.uri, user=args.user, password=args.password)

    try:
        if args.stats:
            stats = loader.get_stats()
            print("\n╔══════════════════════════════════════════════════════════╗")
            print("║           ATL-Physical Neo4j Statistics                  ║")
            print("╠══════════════════════════════════════════════════════════╣")
            print(f"║  Adversary Tasks:      {stats.get('tasks', 0):>6}                          ║")
            print(f"║  Threat Modalities:    {stats.get('modalities', 0):>6}                          ║")
            print(f"║  HD4 Phases:           {stats.get('hd4_phases', 0):>6}                          ║")
            print(f"║  Subtask Links:        {stats.get('subtask_links', 0):>6}                          ║")
            print(f"║  HD4 Links:            {stats.get('hd4_links', 0):>6}                          ║")
            print(f"║  Related Links:        {stats.get('related_links', 0):>6}                          ║")
            print(f"║  Interdiction Points:  {stats.get('interdiction_points', 0):>6}                          ║")
            print(f"║  Key Indicators:       {stats.get('key_indicators', 0):>6}                          ║")
            print("╚══════════════════════════════════════════════════════════╝")
            return

        if args.interdiction:
            points = loader.get_interdiction_points()
            print(f"\n=== ATL-Physical Interdiction Points ({len(points)}) ===")
            for p in points:
                print(f"  [{p['phase']}] {p['task_id']}: {p['title'][:60]}")
                print(f"       HD4: {', '.join(p['hd4_phases'])}")
            return

        if args.indicators:
            indicators = loader.get_key_indicators()
            print(f"\n=== ATL-Physical Key Indicators ({len(indicators)}) ===")
            for i in indicators:
                print(f"  [{i['phase']}] {i['task_id']}: {i['title'][:60]}")
                print(f"       Mundanity: {i['mundanity']:.2f}, HD4: {', '.join(i['hd4_phases'])}")
            return

        if args.load:
            loader.create_atl_indexes()
            stats = loader.load_atl_physical()
            print("\n╔══════════════════════════════════════════════════════════╗")
            print("║           ATL-Physical Load Complete                     ║")
            print("╠══════════════════════════════════════════════════════════╣")
            print(f"║  Tasks Loaded:         {stats['tasks']:>6}                          ║")
            print(f"║  Relationships:        {stats['relationships']:>6}                          ║")
            print(f"║  HD4 Links:            {stats['hd4_links']:>6}                          ║")
            print("╠══════════════════════════════════════════════════════════╣")
            print("║  Container: neo4j-atl-physical                           ║")
            print("║  Browser:   http://localhost:7475                        ║")
            print("║  Bolt:      bolt://localhost:7688                        ║")
            print("║  Label:     :ATLPhysical (for future merge)              ║")
            print("╚══════════════════════════════════════════════════════════╝")
        else:
            print("Use --load to load data, --stats for statistics")
            print("Use --interdiction or --indicators for tactical queries")

    finally:
        loader.close()


if __name__ == "__main__":
    main()
