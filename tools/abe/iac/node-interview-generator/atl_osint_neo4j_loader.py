#!/usr/bin/env python3
"""
Load ATL-OSINT into Neo4j for visualization.
Creates a dedicated graph view with action triggers as properties.
"""

import json
from pathlib import Path

try:
    from neo4j import GraphDatabase
    HAS_NEO4J = True
except ImportError:
    HAS_NEO4J = False
    print("Installing neo4j driver...")
    import subprocess
    subprocess.run(["pip3", "install", "neo4j"], capture_output=True)
    from neo4j import GraphDatabase

GLAF_DIR = Path(__file__).parent / "output" / "glaf"
NEO4J_URI = "bolt://localhost:7687"  # neo4j-test instance
NEO4J_USER = "neo4j"
NEO4J_PASS = "testpassword123"


def load_atl_osint():
    """Load ATL-OSINT layer into Neo4j."""

    # Load JSON data
    json_file = GLAF_DIR / "atl_osint_layer.json"
    print(f"Loading {json_file}...")

    with open(json_file, 'r') as f:
        data = json.load(f)

    tools = data["tools"]
    categories = data["categories"]

    print(f"  Tools: {len(tools)}")
    print(f"  Categories: {len(categories)}")

    # Connect to Neo4j
    print(f"\nConnecting to Neo4j at {NEO4J_URI}...")
    driver = GraphDatabase.driver(NEO4J_URI, auth=(NEO4J_USER, NEO4J_PASS))

    with driver.session() as session:
        # Clear existing ATL-OSINT data
        print("Clearing existing ATL-OSINT nodes...")
        session.run("MATCH (n:OSINTTool) DETACH DELETE n")
        session.run("MATCH (n:OSINTCategory) DETACH DELETE n")
        session.run("MATCH (n:ATLLayer {name: 'ATL-OSINT'}) DETACH DELETE n")

        # Create constraints
        print("Creating constraints...")
        try:
            session.run("CREATE CONSTRAINT osint_tool_triv IF NOT EXISTS FOR (t:OSINTTool) REQUIRE t.triv_hash IS UNIQUE")
            session.run("CREATE CONSTRAINT osint_cat_slug IF NOT EXISTS FOR (c:OSINTCategory) REQUIRE c.slug IS UNIQUE")
        except Exception as e:
            print(f"  (constraints may already exist: {e})")

        # Create root node
        print("Creating ATL-OSINT root node...")
        session.run("""
            CREATE (root:ATLLayer {
                name: 'ATL-OSINT',
                description: 'Attack Threat Library - OSINT Operations',
                tool_count: $tool_count,
                category_count: $cat_count,
                layer_type: 'actionable',
                hd4: 'Hunt'
            })
        """, tool_count=len(tools), cat_count=len(categories))

        # Create category nodes
        print("Creating category nodes...")
        for slug, cat in categories.items():
            session.run("""
                CREATE (c:OSINTCategory {
                    name: $name,
                    slug: $slug,
                    triv_hash: $triv_hash,
                    genome: $genome,
                    hd4: $hd4,
                    tool_count: $tool_count
                })
            """, **cat)

        # Link categories to root
        print("Linking categories to root...")
        session.run("""
            MATCH (root:ATLLayer {name: 'ATL-OSINT'}), (c:OSINTCategory)
            CREATE (root)-[:HAS_CATEGORY]->(c)
        """)

        # Create tool nodes in batches
        print("Creating tool nodes...")
        batch_size = 100
        for i in range(0, len(tools), batch_size):
            batch = tools[i:i+batch_size]
            for tool in batch:
                # Flatten for Neo4j (lists need special handling)
                props = {
                    "name": tool["name"],
                    "url": tool["url"],
                    "description": tool["description"][:500] if tool["description"] else "",
                    "category": tool["category"],
                    "subcategory": tool.get("subcategory") or "",
                    "triv_hash": tool["triv_hash"],
                    "genome": tool["genome"],
                    "hd4": tool["hd4"],
                    "risk": tool["risk"],
                    "action_type": tool["action_type"],
                    "action_script": tool.get("action_script") or "",
                    "action_wasm": tool.get("action_wasm") or "",
                    "api_endpoint": tool.get("api_endpoint") or "",
                    "mitre_techniques": tool.get("mitre_techniques") or [],
                    "d3fend_techniques": tool.get("d3fend_techniques") or [],
                    "actionable": True
                }

                session.run("""
                    CREATE (t:OSINTTool {
                        name: $name,
                        url: $url,
                        description: $description,
                        category: $category,
                        subcategory: $subcategory,
                        triv_hash: $triv_hash,
                        genome: $genome,
                        hd4: $hd4,
                        risk: $risk,
                        action_type: $action_type,
                        action_script: $action_script,
                        action_wasm: $action_wasm,
                        api_endpoint: $api_endpoint,
                        mitre_techniques: $mitre_techniques,
                        d3fend_techniques: $d3fend_techniques,
                        actionable: $actionable
                    })
                """, **props)

            print(f"  Created {min(i+batch_size, len(tools))}/{len(tools)} tools")

        # Link tools to categories
        print("Linking tools to categories...")
        session.run("""
            MATCH (t:OSINTTool), (c:OSINTCategory)
            WHERE t.category = c.slug
            CREATE (c)-[:HAS_TOOL]->(t)
        """)

        # Create inter-tool relationships based on action type
        print("Creating action-type relationships...")
        session.run("""
            MATCH (t1:OSINTTool), (t2:OSINTTool)
            WHERE t1.action_type = t2.action_type
            AND t1.category = t2.category
            AND id(t1) < id(t2)
            CREATE (t1)-[:SAME_ACTION_TYPE]->(t2)
        """)

        # Verify
        result = session.run("MATCH (t:OSINTTool) RETURN count(t) as count")
        count = result.single()["count"]
        print(f"\nVerification: {count} OSINTTool nodes in Neo4j")

        result = session.run("MATCH (c:OSINTCategory) RETURN count(c) as count")
        cat_count = result.single()["count"]
        print(f"Verification: {cat_count} OSINTCategory nodes in Neo4j")

    driver.close()

    print("\n" + "=" * 60)
    print("ATL-OSINT loaded to Neo4j")
    print("=" * 60)
    print(f"  Neo4j Browser: http://localhost:7474")
    print(f"  Query: MATCH (n:ATLLayer)-[*1..2]-(m) RETURN n, m LIMIT 100")
    print(f"  Full view: MATCH (n:OSINTTool) RETURN n LIMIT 500")


if __name__ == "__main__":
    load_atl_osint()
