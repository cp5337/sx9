#!/usr/bin/env python3
"""
ATL-OSINT GLAF Exporter
Exports Awesome OSINT to dedicated GLAF graph layer for actionable OSINT operations.
Triggers via WASM modules or shell scripts.
"""

import json
import re
import hashlib
import uuid
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict

OUTPUT_DIR = Path(__file__).parent / "output"
GLAF_DIR = OUTPUT_DIR / "glaf"
OSINT_README = OUTPUT_DIR / "threat_content" / "awesome-osint" / "README.md"


def generate_sch(content: str) -> str:
    """Generate 16-char Semantic Convergent Hash."""
    h = hashlib.sha256(content.encode()).hexdigest()[:16]
    return h


def generate_cuid() -> str:
    """Generate 16-char Contextual Unique ID."""
    ts = hex(int(datetime.now().timestamp() * 1000))[2:][:10]
    rand = uuid.uuid4().hex[:6]
    return f"{ts}{rand}"


def generate_trivariate(content: str) -> str:
    """Generate full trivariate hash: SCH-CUID-UUID."""
    sch = generate_sch(content)
    cuid = generate_cuid()
    uid = str(uuid.uuid4())
    return f"triv:{sch}_{cuid}_{uid}"


@dataclass
class OSINTTool:
    """OSINT tool/resource with GLAF attributes."""
    name: str
    url: str
    description: str
    category: str
    subcategory: Optional[str] = None

    # GLAF attributes
    triv_hash: str = ""
    genome: str = ""
    hd4: str = "Hunt"  # OSINT is primarily Hunt phase
    risk: float = 0.0  # Safe tools

    # Action triggers
    action_type: str = "web"  # web, api, cli, wasm
    action_script: Optional[str] = None
    action_wasm: Optional[str] = None
    api_endpoint: Optional[str] = None

    # MITRE mappings
    mitre_techniques: List[str] = None
    d3fend_techniques: List[str] = None

    def __post_init__(self):
        self.mitre_techniques = self.mitre_techniques or []
        self.d3fend_techniques = self.d3fend_techniques or []
        if not self.triv_hash:
            self.triv_hash = generate_trivariate(f"{self.name}:{self.url}")
        if not self.genome:
            # 48-char genome fingerprint
            self.genome = hashlib.sha384(f"{self.name}:{self.category}:{self.url}".encode()).hexdigest()[:48]


@dataclass
class OSINTCategory:
    """OSINT category node."""
    name: str
    slug: str
    tool_count: int = 0
    triv_hash: str = ""
    genome: str = ""
    hd4: str = "Hunt"

    def __post_init__(self):
        if not self.triv_hash:
            self.triv_hash = generate_trivariate(f"category:{self.slug}")
        if not self.genome:
            self.genome = hashlib.sha384(f"osint_category:{self.slug}".encode()).hexdigest()[:48]


# OSINT category to MITRE ATT&CK technique mappings
CATEGORY_MITRE_MAP = {
    "general-search": ["T1593", "T1596"],  # Search Open Websites/Domains
    "google-dorks": ["T1593.002"],  # Search Engines
    "social-media": ["T1593.001"],  # Social Media
    "username-check": ["T1589.001"],  # Gather Victim Identity
    "email-search": ["T1589.002"],  # Email Addresses
    "phone-number": ["T1589.002"],  # Email Addresses (close)
    "domain-and-ip": ["T1596.001", "T1596.002"],  # DNS/Passive DNS, WHOIS
    "threat-intelligence": ["T1588"],  # Obtain Capabilities
    "dark-web": ["T1593"],  # Search websites
    "data-breach": ["T1589"],  # Gather Victim Identity Info
    "osint": ["T1591", "T1592", "T1593"],  # Gather org/host/website info
    "geospatial": ["T1591.001"],  # Physical locations
    "image-search": ["T1592.004"],  # Client configs (metadata)
    "video-search": ["T1592.004"],
    "vulnerability": ["T1588.006"],  # Vulnerabilities
    "people": ["T1589.001", "T1589.002", "T1589.003"],  # Credentials, names, emails
    "company": ["T1591.001", "T1591.002", "T1591.004"],  # Org info
    "code-search": ["T1593.003"],  # Code repositories
    "forensics": ["T1005"],  # Data from local system
    "network": ["T1590"],  # Gather victim network info
}

# D3FEND mappings for defensive posture
CATEGORY_D3FEND_MAP = {
    "threat-intelligence": ["D3-TI", "D3-IAA"],  # Threat Intelligence, Intel Analysis
    "vulnerability": ["D3-VA"],  # Vulnerability Analysis
    "domain-and-ip": ["D3-NTA"],  # Network Traffic Analysis
    "data-breach": ["D3-CDM"],  # Credential Monitoring
    "forensics": ["D3-FA", "D3-DA"],  # File/Data Analysis
    "network": ["D3-NTA", "D3-PM"],  # Network Traffic, Protocol Metadata
}

# Action type inference based on tool characteristics
def infer_action_type(name: str, url: str, description: str) -> Tuple[str, Optional[str]]:
    """Infer action type and script/wasm path."""
    name_lower = name.lower()
    desc_lower = description.lower()

    # CLI tools (can be scripted)
    cli_tools = ["nmap", "shodan", "amass", "subfinder", "theharvester", "recon-ng",
                 "maltego", "spiderfoot", "theHarvester", "dnsenum", "dnsrecon",
                 "whois", "dig", "host", "traceroute", "curl", "wget"]

    for tool in cli_tools:
        if tool in name_lower:
            return "cli", f"scripts/osint/{name_lower.replace(' ', '_')}.sh"

    # API-based tools
    if "api" in desc_lower or ".io" in url or "api." in url:
        return "api", None

    # WASM candidates (browser-based that could be wrapped)
    wasm_candidates = ["search", "lookup", "check", "scan", "analyze"]
    for candidate in wasm_candidates:
        if candidate in desc_lower:
            return "wasm", f"wasm/osint/{name_lower.replace(' ', '_')}.wasm"

    # Default to web
    return "web", None


class ATLOSINTExporter:
    """Export Awesome OSINT to GLAF format."""

    def __init__(self):
        self.tools: List[OSINTTool] = []
        self.categories: Dict[str, OSINTCategory] = {}
        self.current_category = None
        self.current_subcategory = None

    def parse_readme(self, filepath: Path) -> int:
        """Parse Awesome OSINT README.md."""
        print(f"Parsing {filepath}...")

        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        lines = content.split('\n')

        # Regex patterns
        category_pattern = re.compile(r'^## \[↑\].*?\) (.+)$')
        subcategory_pattern = re.compile(r'^### (.+)$')
        tool_pattern = re.compile(r'^\* \[([^\]]+)\]\(([^)]+)\)\s*-?\s*(.*)$')

        for line in lines:
            # Check for category header
            cat_match = category_pattern.match(line)
            if cat_match:
                cat_name = cat_match.group(1).strip()
                slug = cat_name.lower().replace(' ', '-').replace('/', '-')
                self.current_category = slug
                self.current_subcategory = None

                if slug not in self.categories:
                    self.categories[slug] = OSINTCategory(
                        name=cat_name,
                        slug=slug
                    )
                continue

            # Check for subcategory
            sub_match = subcategory_pattern.match(line)
            if sub_match and self.current_category:
                self.current_subcategory = sub_match.group(1).strip()
                continue

            # Check for tool entry
            tool_match = tool_pattern.match(line)
            if tool_match and self.current_category:
                name = tool_match.group(1).strip()
                url = tool_match.group(2).strip()
                desc = tool_match.group(3).strip() if tool_match.group(3) else ""

                # Infer action type
                action_type, action_path = infer_action_type(name, url, desc)

                # Get MITRE/D3FEND mappings
                mitre = CATEGORY_MITRE_MAP.get(self.current_category, [])
                d3fend = CATEGORY_D3FEND_MAP.get(self.current_category, [])

                tool = OSINTTool(
                    name=name,
                    url=url,
                    description=desc,
                    category=self.current_category,
                    subcategory=self.current_subcategory,
                    action_type=action_type,
                    mitre_techniques=mitre,
                    d3fend_techniques=d3fend,
                )

                if action_type == "cli":
                    tool.action_script = action_path
                elif action_type == "wasm":
                    tool.action_wasm = action_path
                elif action_type == "api":
                    tool.api_endpoint = url

                self.tools.append(tool)
                self.categories[self.current_category].tool_count += 1

        print(f"  Parsed {len(self.tools)} tools in {len(self.categories)} categories")
        return len(self.tools)

    def export_cypher(self, filepath: Path):
        """Export to Cypher++ for GLAF."""
        print(f"Exporting Cypher to {filepath}...")

        with open(filepath, 'w') as f:
            # Header
            f.write("// ATL-OSINT Graph Layer\n")
            f.write("// Generated: {}\n".format(datetime.now().isoformat()))
            f.write("// Tools: {}, Categories: {}\n\n".format(len(self.tools), len(self.categories)))

            # Schema constraints
            f.write("// === SCHEMA ===\n")
            f.write("CREATE CONSTRAINT osint_tool_triv IF NOT EXISTS FOR (t:OSINTTool) REQUIRE t.triv_hash IS UNIQUE;\n")
            f.write("CREATE CONSTRAINT osint_cat_triv IF NOT EXISTS FOR (c:OSINTCategory) REQUIRE c.triv_hash IS UNIQUE;\n")
            f.write("CREATE INDEX osint_tool_name IF NOT EXISTS FOR (t:OSINTTool) ON (t.name);\n")
            f.write("CREATE INDEX osint_tool_action IF NOT EXISTS FOR (t:OSINTTool) ON (t.action_type);\n\n")

            # Root node for ATL-OSINT layer
            root_triv = generate_trivariate("atl-osint-root")
            root_genome = hashlib.sha384(b"atl-osint-root-layer").hexdigest()[:48]
            f.write("// === ATL-OSINT ROOT ===\n")
            f.write(f'CREATE (root:ATLLayer:OSINTRoot {{\n')
            f.write(f'  triv_hash: "{root_triv}",\n')
            f.write(f'  genome: "{root_genome}",\n')
            f.write(f'  name: "ATL-OSINT",\n')
            f.write(f'  description: "Attack Threat Library - OSINT Operations",\n')
            f.write(f'  hd4: "Hunt",\n')
            f.write(f'  tool_count: {len(self.tools)},\n')
            f.write(f'  category_count: {len(self.categories)},\n')
            f.write(f'  layer_type: "actionable",\n')
            f.write(f'  trigger_types: ["web", "api", "cli", "wasm"]\n')
            f.write(f'}});\n\n')

            # Category nodes
            f.write("// === CATEGORIES ===\n")
            for cat in self.categories.values():
                f.write(f'CREATE (c_{cat.slug.replace("-", "_")}:OSINTCategory {{\n')
                f.write(f'  triv_hash: "{cat.triv_hash}",\n')
                f.write(f'  genome: "{cat.genome}",\n')
                f.write(f'  name: "{cat.name}",\n')
                f.write(f'  slug: "{cat.slug}",\n')
                f.write(f'  hd4: "{cat.hd4}",\n')
                f.write(f'  tool_count: {cat.tool_count}\n')
                f.write(f'}});\n')
            f.write('\n')

            # Category -> Root relationships
            f.write("// === CATEGORY RELATIONSHIPS ===\n")
            for cat in self.categories.values():
                f.write(f'MATCH (root:OSINTRoot), (c:OSINTCategory {{slug: "{cat.slug}"}})\n')
                f.write(f'CREATE (root)-[:HAS_CATEGORY]->(c);\n')
            f.write('\n')

            # Tool nodes
            f.write("// === TOOLS ===\n")
            for i, tool in enumerate(self.tools):
                # Escape quotes in strings
                name_esc = tool.name.replace('"', '\\"').replace("'", "\\'")
                desc_esc = tool.description.replace('"', '\\"').replace("'", "\\'")[:500]
                url_esc = tool.url.replace('"', '\\"')

                f.write(f'CREATE (t{i}:OSINTTool {{\n')
                f.write(f'  triv_hash: "{tool.triv_hash}",\n')
                f.write(f'  genome: "{tool.genome}",\n')
                f.write(f'  name: "{name_esc}",\n')
                f.write(f'  url: "{url_esc}",\n')
                f.write(f'  description: "{desc_esc}",\n')
                f.write(f'  category: "{tool.category}",\n')
                if tool.subcategory:
                    f.write(f'  subcategory: "{tool.subcategory}",\n')
                f.write(f'  hd4: "{tool.hd4}",\n')
                f.write(f'  risk: {tool.risk},\n')
                f.write(f'  action_type: "{tool.action_type}",\n')
                if tool.action_script:
                    f.write(f'  action_script: "{tool.action_script}",\n')
                if tool.action_wasm:
                    f.write(f'  action_wasm: "{tool.action_wasm}",\n')
                if tool.api_endpoint:
                    f.write(f'  api_endpoint: "{tool.api_endpoint}",\n')
                if tool.mitre_techniques:
                    f.write(f'  mitre_techniques: {json.dumps(tool.mitre_techniques)},\n')
                if tool.d3fend_techniques:
                    f.write(f'  d3fend_techniques: {json.dumps(tool.d3fend_techniques)},\n')
                f.write(f'  actionable: true\n')
                f.write(f'}});\n')
            f.write('\n')

            # Tool -> Category relationships
            f.write("// === TOOL RELATIONSHIPS ===\n")
            for i, tool in enumerate(self.tools):
                f.write(f'MATCH (t:OSINTTool {{triv_hash: "{tool.triv_hash}"}}), ')
                f.write(f'(c:OSINTCategory {{slug: "{tool.category}"}})\n')
                f.write(f'CREATE (c)-[:HAS_TOOL]->(t);\n')

        print(f"  Wrote {len(self.tools)} tool nodes + {len(self.categories)} categories")

    def export_json(self, filepath: Path):
        """Export to JSON for programmatic access."""
        print(f"Exporting JSON to {filepath}...")

        data = {
            "layer": "ATL-OSINT",
            "version": "1.0.0",
            "generated": datetime.now().isoformat(),
            "stats": {
                "total_tools": len(self.tools),
                "total_categories": len(self.categories),
                "action_types": {
                    "web": len([t for t in self.tools if t.action_type == "web"]),
                    "api": len([t for t in self.tools if t.action_type == "api"]),
                    "cli": len([t for t in self.tools if t.action_type == "cli"]),
                    "wasm": len([t for t in self.tools if t.action_type == "wasm"]),
                }
            },
            "categories": {cat.slug: asdict(cat) for cat in self.categories.values()},
            "tools": [asdict(t) for t in self.tools],
        }

        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)

        print(f"  Wrote {len(self.tools)} tools to JSON")

    def export_action_manifest(self, filepath: Path):
        """Export action manifest for WASM/script triggers."""
        print(f"Exporting action manifest to {filepath}...")

        actions = {
            "layer": "ATL-OSINT",
            "actions": []
        }

        for tool in self.tools:
            action = {
                "id": tool.triv_hash.split(':')[1][:32],
                "name": tool.name,
                "type": tool.action_type,
                "category": tool.category,
            }

            if tool.action_type == "cli":
                action["trigger"] = {
                    "method": "shell",
                    "script": tool.action_script,
                    "args_template": "{target}"
                }
            elif tool.action_type == "api":
                action["trigger"] = {
                    "method": "http",
                    "endpoint": tool.api_endpoint or tool.url,
                    "method_type": "GET"
                }
            elif tool.action_type == "wasm":
                action["trigger"] = {
                    "method": "wasm",
                    "module": tool.action_wasm,
                    "function": "execute"
                }
            else:  # web
                action["trigger"] = {
                    "method": "browser",
                    "url": tool.url
                }

            actions["actions"].append(action)

        with open(filepath, 'w') as f:
            json.dump(actions, f, indent=2)

        print(f"  Wrote {len(actions['actions'])} action triggers")


def main():
    print("=" * 60)
    print("ATL-OSINT GLAF Exporter")
    print("=" * 60)

    GLAF_DIR.mkdir(parents=True, exist_ok=True)

    exporter = ATLOSINTExporter()

    # Parse Awesome OSINT
    if OSINT_README.exists():
        count = exporter.parse_readme(OSINT_README)
    else:
        print(f"ERROR: {OSINT_README} not found")
        return

    # Export to GLAF formats
    exporter.export_cypher(GLAF_DIR / "atl_osint_graph.cypher")
    exporter.export_json(GLAF_DIR / "atl_osint_layer.json")
    exporter.export_action_manifest(GLAF_DIR / "atl_osint_actions.json")

    # Summary
    print("\n" + "=" * 60)
    print("ATL-OSINT Export Complete")
    print("=" * 60)
    print(f"  Tools:      {len(exporter.tools)}")
    print(f"  Categories: {len(exporter.categories)}")
    print(f"  Action Types:")
    for atype in ["web", "api", "cli", "wasm"]:
        count = len([t for t in exporter.tools if t.action_type == atype])
        print(f"    - {atype}: {count}")
    print(f"\nOutputs:")
    print(f"  - {GLAF_DIR / 'atl_osint_graph.cypher'}")
    print(f"  - {GLAF_DIR / 'atl_osint_layer.json'}")
    print(f"  - {GLAF_DIR / 'atl_osint_actions.json'}")


if __name__ == "__main__":
    main()
