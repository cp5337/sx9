#!/usr/bin/env python3
"""
Neo4j Threat Content Loader
============================

Loads threat intelligence data (techniques, actors, rules, tools) to Neo4j.
RFC-9011, RFC-9011-A, RFC-9023

Ports:
  - Neo4j Browser: 7474
  - Neo4j Bolt: 7687

Usage:
    python neo4j_threat_loader.py --all
    python neo4j_threat_loader.py --cypher output/cypher/threat_graph.cypher
    python neo4j_threat_loader.py --techniques output/threat_content/mitre_attack.json
"""

import json
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
    print("WARNING: neo4j driver not installed. Run: pip install neo4j")

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

OUTPUT_DIR = Path(__file__).parent / "output"


class Neo4jThreatLoader:
    """Load threat intelligence to Neo4j graph database."""

    def __init__(
        self,
        uri: str = "bolt://localhost:7687",
        user: str = "neo4j",
        password: str = "ctas7_graph"
    ):
        if not HAS_NEO4J:
            raise RuntimeError("neo4j driver not installed")
        self.driver = GraphDatabase.driver(uri, auth=(user, password))
        logger.info(f"Connected to Neo4j at {uri}")

    def close(self):
        self.driver.close()

    def create_threat_indexes(self):
        """Create indexes for threat data querying."""
        with self.driver.session() as session:
            indexes = [
                "CREATE INDEX technique_id IF NOT EXISTS FOR (t:Technique) ON (t.id)",
                "CREATE INDEX actor_id IF NOT EXISTS FOR (a:ThreatActor) ON (a.id)",
                "CREATE INDEX rule_id IF NOT EXISTS FOR (r:DetectionRule) ON (r.id)",
                "CREATE INDEX tool_name IF NOT EXISTS FOR (tool:OffensiveTool) ON (tool.package_name)",
                "CREATE INDEX mitigation_id IF NOT EXISTS FOR (m:Mitigation) ON (m.d3fend_id)",
            ]
            for idx in indexes:
                try:
                    session.run(idx)
                except Exception as e:
                    logger.warning(f"Index creation: {e}")
            logger.info("Created threat data indexes")

    def load_techniques(self, file_path: Path) -> int:
        """Load MITRE ATT&CK techniques from JSON."""
        if not file_path.exists():
            logger.warning(f"Techniques file not found: {file_path}")
            return 0

        with open(file_path) as f:
            data = json.load(f)

        techniques = data if isinstance(data, list) else data.get("techniques", [])
        count = 0

        with self.driver.session() as session:
            for tech in techniques:
                tech_id = tech.get("technique_id") or tech.get("id", "")
                name = tech.get("name", "")[:500]
                description = tech.get("description", "")[:2000]
                tactics = tech.get("tactic", []) or tech.get("tactics", [])
                if isinstance(tactics, str):
                    tactics = [tactics]
                platforms = tech.get("platforms", [])
                if isinstance(platforms, str):
                    platforms = [platforms]

                # MERGE to avoid duplicates
                session.run("""
                    MERGE (t:Technique {id: $id})
                    SET t.name = $name,
                        t.description = $description,
                        t.tactics = $tactics,
                        t.platforms = $platforms,
                        t.updated_at = datetime()
                """, id=tech_id, name=name, description=description,
                     tactics=tactics, platforms=platforms)
                count += 1

                # Link to tactics
                for tactic in tactics:
                    session.run("""
                        MERGE (tac:Tactic {name: $tactic})
                        WITH tac
                        MATCH (t:Technique {id: $tech_id})
                        MERGE (t)-[:BELONGS_TO]->(tac)
                    """, tactic=tactic, tech_id=tech_id)

                if count % 50 == 0:
                    logger.info(f"  Loaded {count} techniques...")

        logger.info(f"Loaded {count} techniques to Neo4j")
        return count

    def load_actors(self, file_path: Path) -> int:
        """Load threat actors/groups from JSON."""
        if not file_path.exists():
            logger.warning(f"Actors file not found: {file_path}")
            return 0

        with open(file_path) as f:
            data = json.load(f)

        actors = data if isinstance(data, list) else data.get("groups", [])
        count = 0

        with self.driver.session() as session:
            for actor in actors:
                actor_id = actor.get("actor_id") or actor.get("id", "")
                name = actor.get("name", "")[:200]
                aliases = actor.get("aliases", [])
                if isinstance(aliases, str):
                    aliases = [aliases]
                description = actor.get("description", "")[:2000]
                techniques_used = actor.get("techniques_used", []) or actor.get("techniques", [])
                if isinstance(techniques_used, str):
                    techniques_used = [techniques_used]

                session.run("""
                    MERGE (a:ThreatActor {id: $id})
                    SET a.name = $name,
                        a.aliases = $aliases,
                        a.description = $description,
                        a.updated_at = datetime()
                """, id=actor_id, name=name, aliases=aliases, description=description)

                # Link to techniques
                for tech_id in techniques_used:
                    if isinstance(tech_id, str) and tech_id.startswith("T"):
                        session.run("""
                            MATCH (a:ThreatActor {id: $actor_id})
                            MERGE (t:Technique {id: $tech_id})
                            MERGE (a)-[:USES]->(t)
                        """, actor_id=actor_id, tech_id=tech_id)

                count += 1

        logger.info(f"Loaded {count} threat actors to Neo4j")
        return count

    def load_detection_rules(self, directory: Path) -> int:
        """Load Sigma/YARA rules from directory."""
        if not directory.exists():
            logger.warning(f"Rules directory not found: {directory}")
            return 0

        count = 0
        with self.driver.session() as session:
            for rule_file in directory.glob("**/*.yml"):
                try:
                    import yaml
                    with open(rule_file) as f:
                        rule = yaml.safe_load(f)
                except Exception as e:
                    logger.debug(f"Skip {rule_file.name}: {e}")
                    continue

                if not rule:
                    continue

                rule_id = rule.get("id", rule_file.stem)
                title = rule.get("title", "")[:500]
                status = rule.get("status", "experimental")
                level = rule.get("level", "medium")
                description = rule.get("description", "")[:2000]
                tags = rule.get("tags", [])
                if isinstance(tags, str):
                    tags = [tags]

                # Extract MITRE references from tags
                mitre_refs = [t.split(".")[-1] for t in tags if "attack.t" in t.lower()]

                session.run("""
                    MERGE (r:DetectionRule {id: $id})
                    SET r.title = $title,
                        r.status = $status,
                        r.level = $level,
                        r.description = $description,
                        r.tags = $tags,
                        r.rule_type = 'sigma',
                        r.updated_at = datetime()
                """, id=rule_id, title=title, status=status,
                     level=level, description=description, tags=tags)

                # Link to techniques
                for tech_ref in mitre_refs:
                    tech_id = tech_ref.upper()
                    if not tech_id.startswith("T"):
                        tech_id = "T" + tech_id
                    session.run("""
                        MATCH (r:DetectionRule {id: $rule_id})
                        MERGE (t:Technique {id: $tech_id})
                        MERGE (r)-[:DETECTS]->(t)
                    """, rule_id=rule_id, tech_id=tech_id)

                count += 1

        logger.info(f"Loaded {count} detection rules to Neo4j")
        return count

    def load_tools(self, file_path: Path) -> int:
        """Load offensive tools from JSON."""
        if not file_path.exists():
            logger.warning(f"Tools file not found: {file_path}")
            return 0

        with open(file_path) as f:
            data = json.load(f)

        tools = data if isinstance(data, list) else data.get("tools", [])
        count = 0

        with self.driver.session() as session:
            for tool in tools:
                package_name = tool.get("package_name") or tool.get("name", "")
                display_name = tool.get("display_name", package_name)[:200]
                categories = tool.get("categories", [])
                if isinstance(categories, str):
                    categories = [categories]
                homepage = tool.get("homepage", "")
                mitre_techniques = tool.get("mitre_techniques", [])
                if isinstance(mitre_techniques, str):
                    mitre_techniques = [mitre_techniques]

                session.run("""
                    MERGE (tool:OffensiveTool {package_name: $package_name})
                    SET tool.display_name = $display_name,
                        tool.categories = $categories,
                        tool.homepage = $homepage,
                        tool.updated_at = datetime()
                """, package_name=package_name, display_name=display_name,
                     categories=categories, homepage=homepage)

                # Link to techniques
                for tech_id in mitre_techniques:
                    if isinstance(tech_id, str) and tech_id.startswith("T"):
                        session.run("""
                            MATCH (tool:OffensiveTool {package_name: $package_name})
                            MERGE (t:Technique {id: $tech_id})
                            MERGE (tool)-[:IMPLEMENTS]->(t)
                        """, package_name=package_name, tech_id=tech_id)

                count += 1

        logger.info(f"Loaded {count} offensive tools to Neo4j")
        return count

    def load_cypher_file(self, file_path: Path) -> int:
        """Load raw Cypher statements from file."""
        if not file_path.exists():
            logger.warning(f"Cypher file not found: {file_path}")
            return 0

        content = file_path.read_text()
        statements = [s.strip() for s in content.split(";") if s.strip()]

        count = 0
        errors = 0
        with self.driver.session() as session:
            for stmt in statements:
                if not stmt or stmt.startswith("//"):
                    continue
                try:
                    session.run(stmt)
                    count += 1
                    if count % 100 == 0:
                        logger.info(f"  Executed {count} Cypher statements...")
                except Exception as e:
                    errors += 1
                    if errors <= 5:
                        logger.warning(f"Cypher error: {e}")

        logger.info(f"Executed {count} Cypher statements ({errors} errors)")
        return count

    def load_vectors(self, file_path: Path) -> int:
        """Load vector embeddings and attach to nodes."""
        if not file_path.exists():
            logger.warning(f"Vectors file not found: {file_path}")
            return 0

        with open(file_path) as f:
            vectors = json.load(f)

        count = 0
        with self.driver.session() as session:
            for item in vectors:
                entity_id = item.get("id", "")
                entity_type = item.get("type", "Technique")
                vector = item.get("vector", [])

                if not entity_id or not vector:
                    continue

                # Pad to 768 dimensions if needed (RFC-9012)
                if len(vector) < 768:
                    vector = vector + [0.0] * (768 - len(vector))
                elif len(vector) > 768:
                    vector = vector[:768]

                session.run(f"""
                    MATCH (n:{entity_type} {{id: $id}})
                    SET n.embedding = $vector
                """, id=entity_id, vector=vector)
                count += 1

        logger.info(f"Attached {count} vector embeddings")
        return count

    def get_stats(self) -> Dict[str, int]:
        """Get threat data statistics."""
        with self.driver.session() as session:
            result = session.run("""
                MATCH (t:Technique) WITH count(t) as techniques
                MATCH (a:ThreatActor) WITH techniques, count(a) as actors
                MATCH (r:DetectionRule) WITH techniques, actors, count(r) as rules
                MATCH (tool:OffensiveTool) WITH techniques, actors, rules, count(tool) as tools
                MATCH ()-[uses:USES]->() WITH techniques, actors, rules, tools, count(uses) as uses_links
                MATCH ()-[det:DETECTS]->() WITH techniques, actors, rules, tools, uses_links, count(det) as detects_links
                RETURN techniques, actors, rules, tools, uses_links, detects_links
            """)
            record = result.single()
            return dict(record) if record else {}


def main():
    parser = argparse.ArgumentParser(description="Neo4j Threat Content Loader")
    parser.add_argument("--uri", default="bolt://localhost:7687", help="Neo4j URI")
    parser.add_argument("--user", default="neo4j", help="Neo4j user")
    parser.add_argument("--password", default="ctas7_graph", help="Neo4j password")
    parser.add_argument("--techniques", type=str, help="Path to techniques JSON")
    parser.add_argument("--actors", type=str, help="Path to actors JSON")
    parser.add_argument("--rules", type=str, help="Path to rules directory")
    parser.add_argument("--tools", type=str, help="Path to tools JSON")
    parser.add_argument("--cypher", type=str, help="Path to Cypher file")
    parser.add_argument("--vectors", type=str, help="Path to vectors JSON")
    parser.add_argument("--all", action="store_true", help="Load all from default paths")
    parser.add_argument("--stats", action="store_true", help="Show statistics only")
    args = parser.parse_args()

    if not HAS_NEO4J:
        print("ERROR: Install neo4j driver: pip install neo4j")
        return

    loader = Neo4jThreatLoader(uri=args.uri, user=args.user, password=args.password)

    try:
        if args.stats:
            stats = loader.get_stats()
            print("\nNeo4j Threat Data Statistics:")
            print(f"  Techniques: {stats.get('techniques', 0)}")
            print(f"  Threat Actors: {stats.get('actors', 0)}")
            print(f"  Detection Rules: {stats.get('rules', 0)}")
            print(f"  Offensive Tools: {stats.get('tools', 0)}")
            print(f"  USES Links: {stats.get('uses_links', 0)}")
            print(f"  DETECTS Links: {stats.get('detects_links', 0)}")
            return

        loader.create_threat_indexes()

        # Load from specific paths or defaults
        if args.all:
            threat_dir = OUTPUT_DIR / "threat_content"
            loader.load_techniques(threat_dir / "mitre_attack.json")
            loader.load_actors(threat_dir / "mitre_groups.json")
            loader.load_detection_rules(threat_dir / "sigma_rules")
            loader.load_tools(threat_dir / "kali_tools_inventory.json")

            # Load generated Cypher if exists
            cypher_file = OUTPUT_DIR / "cypher" / "threat_graph.cypher"
            if cypher_file.exists():
                loader.load_cypher_file(cypher_file)

            # Load vectors if exists
            vectors_file = OUTPUT_DIR / "vectors" / "threat_vectors.json"
            if vectors_file.exists():
                loader.load_vectors(vectors_file)
        else:
            if args.techniques:
                loader.load_techniques(Path(args.techniques))
            if args.actors:
                loader.load_actors(Path(args.actors))
            if args.rules:
                loader.load_detection_rules(Path(args.rules))
            if args.tools:
                loader.load_tools(Path(args.tools))
            if args.cypher:
                loader.load_cypher_file(Path(args.cypher))
            if args.vectors:
                loader.load_vectors(Path(args.vectors))

        # Show final stats
        stats = loader.get_stats()
        print("\nFinal Statistics:")
        print(f"  Techniques: {stats.get('techniques', 0)}")
        print(f"  Threat Actors: {stats.get('actors', 0)}")
        print(f"  Detection Rules: {stats.get('rules', 0)}")
        print(f"  Offensive Tools: {stats.get('tools', 0)}")
        print(f"  USES Links: {stats.get('uses_links', 0)}")
        print(f"  DETECTS Links: {stats.get('detects_links', 0)}")

    finally:
        loader.close()


if __name__ == "__main__":
    main()
