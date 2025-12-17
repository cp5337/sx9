#!/usr/bin/env python3
"""
Neo4j Interview Loader
======================

Loads node interviews and crate interviews to Neo4j graph database.

Ports:
  - Neo4j Browser: 7474
  - Neo4j Bolt: 7687
"""

import json
import argparse
from pathlib import Path
from datetime import datetime

try:
    from neo4j import GraphDatabase
    HAS_NEO4J = True
except ImportError:
    HAS_NEO4J = False
    print("WARNING: neo4j driver not installed. Run: pip install neo4j")


OUTPUT_DIR = Path(__file__).parent / "output"


class Neo4jLoader:
    """Load interviews to Neo4j graph database."""

    def __init__(self, uri: str = "bolt://localhost:7687", user: str = "neo4j", password: str = "ctas7_graph"):
        if not HAS_NEO4J:
            raise RuntimeError("neo4j driver not installed")
        self.driver = GraphDatabase.driver(uri, auth=(user, password))

    def close(self):
        self.driver.close()

    def clear_interviews(self):
        """Clear all existing interview nodes."""
        with self.driver.session() as session:
            result = session.run("MATCH (i:Interview) DETACH DELETE i RETURN count(*) as deleted")
            count = result.single()["deleted"]
            print(f"Cleared {count} existing Interview nodes")

    def load_node_interviews(self, file_path: Path) -> int:
        """Load node interviews from JSON file."""
        with open(file_path) as f:
            interviews = json.load(f)

        count = 0
        with self.driver.session() as session:
            for interview in interviews:
                task_id = interview.get("task_id", "unknown")

                # Extract fields safely
                voice = interview.get("voice", "")[:5000]  # Truncate long content
                purpose = interview.get("purpose", "")
                mitre_techniques = interview.get("mitre_techniques", [])
                if isinstance(mitre_techniques, str):
                    mitre_techniques = [mitre_techniques]
                d3fend = interview.get("d3fend_countermeasures", [])
                if isinstance(d3fend, str):
                    d3fend = [d3fend]
                indicators = interview.get("indicators", [])
                if isinstance(indicators, str):
                    indicators = [indicators]

                # Create Interview node
                session.run("""
                    CREATE (i:Interview:Node {
                        task_id: $task_id,
                        voice: $voice,
                        purpose: $purpose,
                        mitre_techniques: $mitre_techniques,
                        d3fend_countermeasures: $d3fend,
                        indicators: $indicators,
                        created_at: datetime()
                    })
                """, task_id=task_id, voice=voice, purpose=purpose,
                     mitre_techniques=mitre_techniques, d3fend=d3fend,
                     indicators=indicators)
                count += 1

                if count % 20 == 0:
                    print(f"  Loaded {count} node interviews...")

        print(f"Loaded {count} node interviews to Neo4j")
        return count

    def load_crate_interviews(self, directory: Path) -> int:
        """Load crate interviews from directory of JSON files."""
        count = 0

        with self.driver.session() as session:
            for file_path in directory.glob("*.json"):
                if file_path.name == "generation_summary.json":
                    continue

                try:
                    with open(file_path) as f:
                        interview = json.load(f)
                except json.JSONDecodeError:
                    print(f"  Skip (invalid JSON): {file_path.name}")
                    continue

                crate_id = interview.get("crate_id", file_path.stem)
                crate_name = interview.get("crate_name", "")
                voice = interview.get("voice", "")[:5000]
                purpose = interview.get("purpose", "")
                capabilities = interview.get("capabilities", [])
                if isinstance(capabilities, str):
                    capabilities = [capabilities]
                design_patterns = interview.get("design_patterns", [])
                if isinstance(design_patterns, str):
                    design_patterns = [design_patterns]
                integration_points = interview.get("integration_points", [])
                if isinstance(integration_points, str):
                    integration_points = [integration_points]

                # Foundation blocks
                foundation_uses = []
                foundation_provides = []
                foundation = interview.get("foundation_blocks", {})
                if isinstance(foundation, dict):
                    foundation_uses = foundation.get("uses", [])
                    foundation_provides = foundation.get("provides", [])

                session.run("""
                    CREATE (c:Interview:Crate {
                        crate_id: $crate_id,
                        crate_name: $crate_name,
                        voice: $voice,
                        purpose: $purpose,
                        capabilities: $capabilities,
                        design_patterns: $design_patterns,
                        integration_points: $integration_points,
                        foundation_uses: $foundation_uses,
                        foundation_provides: $foundation_provides,
                        created_at: datetime()
                    })
                """, crate_id=crate_id, crate_name=crate_name, voice=voice,
                     purpose=purpose, capabilities=capabilities,
                     design_patterns=design_patterns, integration_points=integration_points,
                     foundation_uses=foundation_uses, foundation_provides=foundation_provides)
                count += 1

        print(f"Loaded {count} crate interviews to Neo4j")
        return count

    def create_indexes(self):
        """Create indexes for efficient querying."""
        with self.driver.session() as session:
            # Interview indexes
            session.run("CREATE INDEX interview_task_id IF NOT EXISTS FOR (i:Interview) ON (i.task_id)")
            session.run("CREATE INDEX interview_crate_id IF NOT EXISTS FOR (i:Interview) ON (i.crate_id)")
            print("Created indexes")

    def link_to_techniques(self):
        """Link interviews to existing Technique nodes if they exist."""
        with self.driver.session() as session:
            result = session.run("""
                MATCH (i:Interview:Node)
                WHERE size(i.mitre_techniques) > 0
                UNWIND i.mitre_techniques as tech_id
                MERGE (t:Technique {id: tech_id})
                MERGE (i)-[:COVERS]->(t)
                RETURN count(*) as links_created
            """)
            count = result.single()["links_created"]
            print(f"Created {count} Interview->Technique links")

    def link_crates_to_foundation(self):
        """Link crate interviews to foundation crates."""
        with self.driver.session() as session:
            # Link crates that use foundation blocks
            result = session.run("""
                MATCH (c:Interview:Crate)
                WHERE size(c.foundation_uses) > 0
                UNWIND c.foundation_uses as block
                MERGE (f:FoundationBlock {name: block})
                MERGE (c)-[:USES_FOUNDATION]->(f)
                RETURN count(*) as links_created
            """)
            count = result.single()["links_created"]
            print(f"Created {count} Crate->Foundation links")

    def get_stats(self) -> dict:
        """Get database statistics."""
        with self.driver.session() as session:
            result = session.run("""
                MATCH (n:Interview:Node) WITH count(n) as node_interviews
                MATCH (c:Interview:Crate) WITH node_interviews, count(c) as crate_interviews
                MATCH (t:Technique) WITH node_interviews, crate_interviews, count(t) as techniques
                MATCH ()-[r:COVERS]->() WITH node_interviews, crate_interviews, techniques, count(r) as covers_links
                RETURN node_interviews, crate_interviews, techniques, covers_links
            """)
            record = result.single()
            return dict(record) if record else {}


def main():
    parser = argparse.ArgumentParser(description="Neo4j Interview Loader")
    parser.add_argument("--uri", default="bolt://localhost:7687", help="Neo4j URI")
    parser.add_argument("--user", default="neo4j", help="Neo4j user")
    parser.add_argument("--password", default="ctas7_graph", help="Neo4j password")
    parser.add_argument("--clear", action="store_true", help="Clear existing interviews first")
    parser.add_argument("--node-interviews", type=str, help="Path to node interviews JSON")
    parser.add_argument("--crate-interviews", type=str, help="Path to crate interviews directory")
    parser.add_argument("--all", action="store_true", help="Load all interviews from default paths")
    parser.add_argument("--stats", action="store_true", help="Show database statistics")
    args = parser.parse_args()

    if not HAS_NEO4J:
        print("ERROR: Install neo4j driver: pip install neo4j")
        return

    loader = Neo4jLoader(uri=args.uri, user=args.user, password=args.password)

    try:
        if args.stats:
            stats = loader.get_stats()
            print(f"\nNeo4j Statistics:")
            print(f"  Node Interviews: {stats.get('node_interviews', 0)}")
            print(f"  Crate Interviews: {stats.get('crate_interviews', 0)}")
            print(f"  Techniques: {stats.get('techniques', 0)}")
            print(f"  COVERS Links: {stats.get('covers_links', 0)}")
            return

        if args.clear:
            loader.clear_interviews()

        loader.create_indexes()

        if args.all or args.node_interviews:
            node_path = Path(args.node_interviews) if args.node_interviews else OUTPUT_DIR / "interviews_for_upload.json"
            if node_path.exists():
                loader.load_node_interviews(node_path)
                loader.link_to_techniques()
            else:
                print(f"Node interviews not found: {node_path}")

        if args.all or args.crate_interviews:
            crate_dir = Path(args.crate_interviews) if args.crate_interviews else OUTPUT_DIR / "crate_interviews"
            if crate_dir.exists():
                loader.load_crate_interviews(crate_dir)
                loader.link_crates_to_foundation()
            else:
                print(f"Crate interviews directory not found: {crate_dir}")

        # Show final stats
        stats = loader.get_stats()
        print(f"\nFinal Statistics:")
        print(f"  Node Interviews: {stats.get('node_interviews', 0)}")
        print(f"  Crate Interviews: {stats.get('crate_interviews', 0)}")
        print(f"  Techniques: {stats.get('techniques', 0)}")
        print(f"  COVERS Links: {stats.get('covers_links', 0)}")

    finally:
        loader.close()


if __name__ == "__main__":
    main()
