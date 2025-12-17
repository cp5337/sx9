#!/usr/bin/env python3
"""
RFC-9011: Threat Content Vectorization Pipeline

Creates vector embeddings and training data for:
- Phi-3 LoRA fine-tuning (generative threat analysis)
- DistilBERT LoRA fine-tuning (technique classification/NER)
- Neo4j graph database (Cypher queries)
- ChromaDB/FAISS vector store (semantic search)
"""

import json
import os
import hashlib
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass, asdict
import csv

# Vector/ML imports - graceful fallback
try:
    import numpy as np
    HAS_NUMPY = True
except ImportError:
    HAS_NUMPY = False
    print("Warning: numpy not found, some features disabled")

try:
    from sentence_transformers import SentenceTransformer
    HAS_SENTENCE_TRANSFORMERS = True
except ImportError:
    HAS_SENTENCE_TRANSFORMERS = False
    print("Warning: sentence-transformers not found, using hash-based vectors")

try:
    import chromadb
    HAS_CHROMADB = True
except ImportError:
    HAS_CHROMADB = False
    print("Warning: chromadb not found, using JSON fallback")

OUTPUT_DIR = Path(__file__).parent / "output"
VECTOR_DIR = OUTPUT_DIR / "vectors"
TRAINING_DIR = OUTPUT_DIR / "training_data"
CYPHER_DIR = OUTPUT_DIR / "cypher"

# Import ATL-Physical loader (training data only, invisible operationally)
try:
    from leptose_training_prep import load_atl_physical
    HAS_ATL_PHYSICAL = True
except ImportError:
    HAS_ATL_PHYSICAL = False
    print("Note: ATL-Physical loader not available")


@dataclass
class ThreatDocument:
    """Unified threat document for vectorization."""
    id: str
    source: str  # mitre, atomic, sigma, nuclei, lolbas, etc.
    doc_type: str  # technique, test, rule, template, binary
    title: str
    content: str
    metadata: Dict[str, Any]

    # Relationships
    mitre_techniques: List[str] = None
    tactics: List[str] = None
    platforms: List[str] = None

    def __post_init__(self):
        self.mitre_techniques = self.mitre_techniques or []
        self.tactics = self.tactics or []
        self.platforms = self.platforms or []

    def to_embedding_text(self) -> str:
        """Generate text for embedding."""
        parts = [
            f"Title: {self.title}",
            f"Type: {self.doc_type}",
            f"Source: {self.source}",
        ]
        if self.mitre_techniques:
            parts.append(f"MITRE Techniques: {', '.join(self.mitre_techniques)}")
        if self.tactics:
            parts.append(f"Tactics: {', '.join(self.tactics)}")
        if self.platforms:
            parts.append(f"Platforms: {', '.join(self.platforms)}")
        parts.append(f"Content: {self.content[:1000]}")
        return "\n".join(parts)


class ThreatVectorPipeline:
    """Pipeline for vectorizing threat content and generating training data."""

    def __init__(self, model_name: str = "all-MiniLM-L6-v2"):
        VECTOR_DIR.mkdir(parents=True, exist_ok=True)
        TRAINING_DIR.mkdir(parents=True, exist_ok=True)
        CYPHER_DIR.mkdir(parents=True, exist_ok=True)

        self.model_name = model_name
        self.embedding_model = None
        self.documents: List[ThreatDocument] = []
        self.vectors: Dict[str, List[float]] = {}

        # Initialize embedding model if available
        if HAS_SENTENCE_TRANSFORMERS:
            try:
                self.embedding_model = SentenceTransformer(model_name)
                print(f"Loaded embedding model: {model_name}")
            except Exception as e:
                print(f"Warning: Could not load {model_name}: {e}")

        # Initialize ChromaDB if available
        self.chroma_client = None
        self.collection = None
        if HAS_CHROMADB:
            try:
                self.chroma_client = chromadb.PersistentClient(
                    path=str(VECTOR_DIR / "chromadb")
                )
                self.collection = self.chroma_client.get_or_create_collection(
                    name="threat_content",
                    metadata={"hnsw:space": "cosine"}
                )
                print("Initialized ChromaDB collection")
            except Exception as e:
                print(f"Warning: ChromaDB init failed: {e}")

    def load_threat_content(self, threat_dir: Path) -> int:
        """Load all threat content from fetcher output."""
        print("\n" + "=" * 70)
        print("Loading Threat Content for Vectorization")
        print("=" * 70)

        count = 0

        # ========== MITRE ATT&CK Suite (Enterprise, ICS, Mobile) ==========
        print("\n--- MITRE ATT&CK Suite ---")
        mitre_file = threat_dir / "mitre_attack.json"
        if mitre_file.exists():
            count += self._load_mitre_attack(mitre_file, "enterprise")

        # Load MITRE ICS
        mitre_ics_file = threat_dir / "mitre_attack_ics.json"
        if mitre_ics_file.exists():
            count += self._load_mitre_attack(mitre_ics_file, "ics")

        # Load MITRE Mobile
        mitre_mobile_file = threat_dir / "mitre_attack_mobile.json"
        if mitre_mobile_file.exists():
            count += self._load_mitre_attack(mitre_mobile_file, "mobile")

        # Load MITRE index (parsed techniques/groups)
        mitre_index = threat_dir / "mitre_index.json"
        if mitre_index.exists():
            count += self._load_mitre_index(mitre_index)

        # ========== MITRE Defense Suite (D3FEND) ==========
        print("\n--- MITRE Defense Suite ---")
        d3fend_file = threat_dir / "d3fend.json"
        if d3fend_file.exists():
            count += self._load_d3fend(d3fend_file)

        # Load crosswalk for relationships
        crosswalk_file = threat_dir / "crosswalk_index.json"
        if crosswalk_file.exists():
            self._load_crosswalk(crosswalk_file)

        # ========== Cloned Repos ==========
        print("\n--- Cloned Repositories ---")
        for repo_name in ["atomic-red-team", "sigma", "nuclei-templates",
                          "lolbas", "gtfobins", "yara-rules", "awesome-osint",
                          "nmap", "wazuh", "car", "caldera", "loldrivers",
                          "hijacklibs", "wadcoms", "osint-framework", "sherlock"]:
            repo_dir = threat_dir / repo_name
            if repo_dir.exists():
                count += self._load_repo_content(repo_dir, repo_name)

        # ========== Kali Tools (from exploit-arsenal JSON) ==========
        print("\n--- Kali Tools ---")
        kali_file = threat_dir / "kali_tools_inventory.json"
        if kali_file.exists():
            count += self._load_kali_inventory(kali_file)
        else:
            count += self._load_kali_tools()  # Fallback to embedded

        # ========== ExploitDB ==========
        print("\n--- ExploitDB ---")
        exploitdb_file = threat_dir / "exploitdb_index.json"
        if exploitdb_file.exists():
            count += self._load_exploitdb(exploitdb_file)

        print(f"\nTotal documents loaded: {len(self.documents)}")
        return len(self.documents)

    def _load_mitre_attack(self, filepath: Path, domain: str = "enterprise") -> int:
        """Load raw MITRE ATT&CK STIX data (Enterprise, ICS, or Mobile)."""
        with open(filepath, 'r') as f:
            data = json.load(f)

        count = 0
        for obj in data.get("objects", []):
            if obj.get("type") == "attack-pattern":
                refs = obj.get("external_references", [])
                tech_id = None
                for ref in refs:
                    if ref.get("source_name") == "mitre-attack":
                        tech_id = ref.get("external_id")
                        break

                if tech_id:
                    doc = ThreatDocument(
                        id=f"mitre_{domain}_{tech_id}",
                        source=f"mitre_attack_{domain}",
                        doc_type="technique",
                        title=obj.get("name", ""),
                        content=obj.get("description", "")[:2000],
                        metadata={
                            "tech_id": tech_id,
                            "domain": domain,
                            "created": obj.get("created", ""),
                            "modified": obj.get("modified", ""),
                        },
                        mitre_techniques=[tech_id],
                        tactics=[p.get("phase_name") for p in obj.get("kill_chain_phases", [])],
                        platforms=obj.get("x_mitre_platforms", []),
                    )
                    self.documents.append(doc)
                    count += 1

        print(f"  Loaded {count} MITRE ATT&CK {domain.upper()} techniques")
        return count

    def _load_d3fend(self, filepath: Path) -> int:
        """Load MITRE D3FEND defensive countermeasures."""
        with open(filepath, 'r') as f:
            data = json.load(f)

        count = 0
        if "@graph" in data:
            for item in data["@graph"]:
                item_type = item.get("@type", [])
                if isinstance(item_type, str):
                    item_type = [item_type]

                # Look for defensive techniques
                if any("DefensiveTechnique" in str(t) or "d3f:" in str(t) for t in item_type):
                    d3f_id = item.get("@id", "").replace("d3f:", "")
                    label = item.get("rdfs:label", "")
                    definition = item.get("d3f:definition", "") or ""

                    if d3f_id and label:
                        doc = ThreatDocument(
                            id=f"d3fend_{d3f_id}",
                            source="mitre_d3fend",
                            doc_type="countermeasure",
                            title=label,
                            content=definition[:1500] if definition else f"D3FEND defensive technique: {label}",
                            metadata={
                                "d3f_id": d3f_id,
                                "type": item_type,
                            },
                        )
                        self.documents.append(doc)
                        count += 1

        print(f"  Loaded {count} D3FEND countermeasures")
        return count

    def _load_kali_tools(self) -> int:
        """Load Kali tools from embedded inventory."""
        KALI_TOOL_CATEGORIES = {
            "NetworkRecon": ["nmap", "masscan", "netdiscover", "arp-scan", "unicornscan"],
            "WebApplicationTesting": ["nikto", "sqlmap", "gobuster", "dirb", "wfuzz", "burpsuite", "zaproxy"],
            "ExploitationFrameworks": ["metasploit", "armitage", "beef-xss", "social-engineer-toolkit"],
            "PasswordCracking": ["hydra", "hashcat", "john", "medusa", "ncrack", "ophcrack"],
            "WirelessNetworks": ["aircrack-ng", "wifite", "reaver", "fern-wifi-cracker", "kismet"],
            "OSINT": ["theharvester", "recon-ng", "maltego", "spiderfoot", "shodan"],
            "Forensics": ["autopsy", "binwalk", "volatility", "sleuthkit", "foremost"],
            "ReverseEngineering": ["ghidra", "radare2", "gdb", "objdump", "ida-free"],
            "Sniffing": ["wireshark", "tcpdump", "ettercap", "bettercap", "dsniff"],
            "VulnerabilityAnalysis": ["nessus", "openvas", "nikto", "lynis", "wapiti"],
            "PostExploitation": ["empire", "covenant", "sliver", "bloodhound", "mimikatz"],
        }

        count = 0
        for category, tools in KALI_TOOL_CATEGORIES.items():
            for tool in tools:
                doc = ThreatDocument(
                    id=f"kali_{tool}",
                    source="kali_linux",
                    doc_type="tool",
                    title=tool,
                    content=f"Kali Linux tool: {tool}. Category: {category}. Used for {category.replace('_', ' ').lower()} operations.",
                    metadata={
                        "category": category,
                        "deployment": "bare_metal" if category in ["NetworkRecon", "ExploitationFrameworks", "WebApplicationTesting"] else "iso",
                    },
                )
                self.documents.append(doc)
                count += 1

        print(f"  Loaded {count} Kali tools")
        return count

    def _load_kali_inventory(self, filepath: Path) -> int:
        """Load Kali tools from exploit-arsenal JSON inventory."""
        with open(filepath, 'r') as f:
            data = json.load(f)

        count = 0
        tools = data.get("tools", [])
        for tool in tools:
            doc = ThreatDocument(
                id=f"kali_{tool.get('name', 'unknown')}",
                source="kali_linux",
                doc_type="tool",
                title=tool.get("display_name", tool.get("name", "")),
                content=f"Kali Linux tool: {tool.get('name')}. Package: {tool.get('package_name')}. {tool.get('description', '')}. Category: {tool.get('category')}. Deployment: {tool.get('deployment_type')}. Risk: {tool.get('risk_level')}",
                metadata={
                    "package": tool.get("package_name", ""),
                    "category": tool.get("category", ""),
                    "deployment": tool.get("deployment_type", ""),
                    "risk_level": tool.get("risk_level", ""),
                },
                mitre_techniques=tool.get("mitre_techniques", []),
            )
            self.documents.append(doc)
            count += 1

        print(f"  Loaded {count} Kali tools from inventory")
        return count

    def _load_exploitdb(self, filepath: Path) -> int:
        """Load ExploitDB index."""
        with open(filepath, 'r') as f:
            data = json.load(f)

        count = 0
        for exploit in data.get("exploits", []):
            doc = ThreatDocument(
                id=f"edb_{exploit.get('id', 'unknown')}",
                source="exploitdb",
                doc_type="exploit",
                title=f"EDB-{exploit.get('id', '')}",
                content=f"ExploitDB exploit {exploit.get('id')}. Category: {exploit.get('category')}. File: {exploit.get('filename')}. Type: {exploit.get('type', '')}",
                metadata={
                    "category": exploit.get("category", ""),
                    "filename": exploit.get("filename", ""),
                    "exploit_type": exploit.get("type", ""),
                },
            )
            self.documents.append(doc)
            count += 1

        print(f"  Loaded {count} ExploitDB entries")
        return count

    def _load_mitre_index(self, filepath: Path) -> int:
        """Load parsed MITRE index."""
        with open(filepath, 'r') as f:
            data = json.load(f)

        count = 0
        # Groups
        for group_id, group in data.get("groups", {}).items():
            doc = ThreatDocument(
                id=f"mitre_group_{group_id}",
                source="mitre_attack",
                doc_type="group",
                title=group.get("name", group_id),
                content=f"Threat group {group.get('name')}. Aliases: {', '.join(group.get('aliases', []))}",
                metadata=group,
            )
            self.documents.append(doc)
            count += 1

        print(f"  Loaded {count} MITRE groups")
        return count

    def _load_crosswalk(self, filepath: Path):
        """Load crosswalk mappings for enrichment."""
        with open(filepath, 'r') as f:
            self.crosswalk = json.load(f)
        print(f"  Loaded crosswalk mappings")

    def _load_repo_content(self, repo_dir: Path, repo_name: str) -> int:
        """Load content from cloned repositories."""
        count = 0

        if repo_name == "atomic-red-team":
            atomics_dir = repo_dir / "atomics"
            if atomics_dir.exists():
                for tech_dir in atomics_dir.iterdir():
                    if tech_dir.is_dir() and tech_dir.name.startswith("T"):
                        yaml_file = tech_dir / f"{tech_dir.name}.yaml"
                        if yaml_file.exists():
                            count += self._load_atomic_yaml(yaml_file)

        elif repo_name == "sigma":
            rules_dir = repo_dir / "rules"
            if rules_dir.exists():
                for yaml_file in rules_dir.rglob("*.yml"):
                    count += self._load_sigma_rule(yaml_file)

        elif repo_name == "nuclei-templates":
            for yaml_file in repo_dir.rglob("*.yaml"):
                if count < 5000:  # Limit for memory
                    count += self._load_nuclei_template(yaml_file)

        elif repo_name == "lolbas":
            yml_dir = repo_dir / "yml"
            if yml_dir.exists():
                for yaml_file in yml_dir.rglob("*.yml"):
                    count += self._load_lolbas_entry(yaml_file)

        elif repo_name == "gtfobins":
            gtfo_dir = repo_dir / "_gtfobins"
            if gtfo_dir.exists():
                for md_file in gtfo_dir.glob("*.md"):
                    count += self._load_gtfobins_entry(md_file)

        elif repo_name == "awesome-osint":
            readme = repo_dir / "README.md"
            if readme.exists():
                count += self._load_awesome_osint(readme)

        elif repo_name == "yara-rules":
            for yar_file in repo_dir.rglob("*.yar"):
                count += self._load_yara_rule(yar_file)

        elif repo_name == "nmap":
            scripts_dir = repo_dir / "scripts"
            if scripts_dir.exists():
                for nse_file in scripts_dir.glob("*.nse"):
                    count += self._load_nmap_script(nse_file)

        elif repo_name == "wazuh":
            rules_dir = repo_dir / "ruleset" / "rules"
            if rules_dir.exists():
                for xml_file in rules_dir.glob("*.xml"):
                    count += self._load_wazuh_rules(xml_file)

        elif repo_name == "car":
            analytics_dir = repo_dir / "analytics"
            if analytics_dir.exists():
                for yaml_file in analytics_dir.rglob("*.yaml"):
                    count += self._load_car_analytic(yaml_file)

        elif repo_name == "caldera":
            for yaml_file in repo_dir.rglob("*.yml"):
                if "abilities" in str(yaml_file) or "adversaries" in str(yaml_file):
                    count += self._load_caldera_ability(yaml_file)

        elif repo_name == "loldrivers":
            drivers_dir = repo_dir / "yaml" / "drivers"
            if drivers_dir.exists():
                for yaml_file in drivers_dir.glob("*.yaml"):
                    count += self._load_loldriver(yaml_file)

        elif repo_name == "hijacklibs":
            yml_dir = repo_dir / "yml"
            if yml_dir.exists():
                for yaml_file in yml_dir.rglob("*.yml"):
                    count += self._load_hijacklib(yaml_file)

        elif repo_name == "wadcoms":
            wadcoms_dir = repo_dir / "_wadcoms"
            if wadcoms_dir.exists():
                for md_file in wadcoms_dir.glob("*.md"):
                    count += self._load_wadcom(md_file)

        elif repo_name == "osint-framework":
            arf_file = repo_dir / "arf.json"
            if arf_file.exists():
                count += self._load_osint_framework(arf_file)

        elif repo_name == "sherlock":
            data_file = repo_dir / "sherlock" / "resources" / "data.json"
            if data_file.exists():
                count += self._load_sherlock_sites(data_file)

        if count > 0:
            print(f"  Loaded {count} documents from {repo_name}")
        return count

    def _load_atomic_yaml(self, filepath: Path) -> int:
        """Load Atomic Red Team test YAML."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            if not data:
                return 0

            tech_id = data.get("attack_technique", "")
            for i, test in enumerate(data.get("atomic_tests", [])):
                doc = ThreatDocument(
                    id=f"atomic_{tech_id}_{i}",
                    source="atomic_red_team",
                    doc_type="test",
                    title=test.get("name", f"{tech_id} Test {i}"),
                    content=test.get("description", "")[:1000],
                    metadata={
                        "executor": test.get("executor", {}).get("name", ""),
                        "elevation_required": test.get("executor", {}).get("elevation_required", False),
                    },
                    mitre_techniques=[tech_id],
                    platforms=test.get("supported_platforms", []),
                )
                self.documents.append(doc)

            return len(data.get("atomic_tests", []))
        except Exception:
            return 0

    def _load_sigma_rule(self, filepath: Path) -> int:
        """Load Sigma detection rule."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            if not data or not isinstance(data, dict):
                return 0

            # Extract MITRE techniques from tags
            tags = data.get("tags", [])
            techniques = [t.replace("attack.", "").upper()
                         for t in tags if t.startswith("attack.t")]

            doc = ThreatDocument(
                id=f"sigma_{data.get('id', filepath.stem)}",
                source="sigma",
                doc_type="rule",
                title=data.get("title", ""),
                content=f"{data.get('description', '')} Detection: {json.dumps(data.get('detection', {}))}",
                metadata={
                    "status": data.get("status", ""),
                    "level": data.get("level", ""),
                    "logsource": data.get("logsource", {}),
                },
                mitre_techniques=techniques,
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_nuclei_template(self, filepath: Path) -> int:
        """Load Nuclei vulnerability template."""
        import re
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read(3000)

            id_match = re.search(r'^id:\s*(.+)$', content, re.MULTILINE)
            name_match = re.search(r'name:\s*(.+)$', content, re.MULTILINE)
            severity_match = re.search(r'severity:\s*(\w+)', content)

            if not id_match:
                return 0

            template_id = id_match.group(1).strip()

            doc = ThreatDocument(
                id=f"nuclei_{template_id}",
                source="nuclei",
                doc_type="template",
                title=name_match.group(1).strip() if name_match else template_id,
                content=content[:1500],
                metadata={
                    "severity": severity_match.group(1) if severity_match else "unknown",
                    "path": str(filepath.relative_to(filepath.parent.parent.parent)),
                },
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_lolbas_entry(self, filepath: Path) -> int:
        """Load LOLBAS binary entry."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            if not data:
                return 0

            name = data.get("Name", filepath.stem)
            commands = data.get("Commands", [])

            # Extract MITRE IDs from commands
            techniques = []
            for cmd in commands:
                mitre_id = cmd.get("MitreID", "")
                if mitre_id and mitre_id not in techniques:
                    techniques.append(mitre_id)

            doc = ThreatDocument(
                id=f"lolbas_{name.lower()}",
                source="lolbas",
                doc_type="binary",
                title=name,
                content=f"{data.get('Description', '')} Commands: {json.dumps([c.get('Command', '')[:100] for c in commands[:3]])}",
                metadata={
                    "author": data.get("Author", ""),
                    "paths": data.get("Full_Path", [])[:3],
                },
                mitre_techniques=techniques,
                platforms=["windows"],
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_gtfobins_entry(self, filepath: Path) -> int:
        """Load GTFOBins entry."""
        import re
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read()

            name = filepath.stem

            # Extract functions from front matter
            functions = []
            func_match = re.search(r'functions:\s*\n((?:\s+-\s+\w+\n)+)', content)
            if func_match:
                functions = [f.strip().lstrip('- ')
                            for f in func_match.group(1).split('\n') if f.strip()]

            doc = ThreatDocument(
                id=f"gtfobins_{name}",
                source="gtfobins",
                doc_type="binary",
                title=name,
                content=f"Unix binary for: {', '.join(functions)}. {content[200:800]}",
                metadata={
                    "functions": functions,
                },
                platforms=["linux", "macos"],
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_awesome_osint(self, filepath: Path) -> int:
        """Load OSINT tools from awesome-osint README."""
        import re
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read()

            count = 0
            current_category = "General"

            for line in content.split('\n'):
                if line.startswith('## '):
                    current_category = line[3:].strip()

                # Extract tool links
                link_matches = re.findall(r'\[([^\]]+)\]\(([^)]+)\)', line)
                for name, url in link_matches:
                    if url.startswith('http') and 'github.com/jivoi' not in url:
                        doc = ThreatDocument(
                            id=f"osint_{count}_{re.sub(r'[^a-z0-9]', '_', name.lower())}",
                            source="osint",
                            doc_type="tool",
                            title=name,
                            content=f"OSINT tool: {name}. Category: {current_category}. URL: {url}",
                            metadata={
                                "url": url,
                                "category": current_category,
                            },
                        )
                        self.documents.append(doc)
                        count += 1

            return count
        except Exception:
            return 0

    def _load_yara_rule(self, filepath: Path) -> int:
        """Load YARA malware detection rule."""
        import re
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read()

            count = 0
            rule_matches = re.findall(r'rule\s+(\w+)', content)
            category = filepath.parent.name
            file_stem = filepath.stem

            for idx, rule_name in enumerate(rule_matches):
                # Use file stem + rule name + index to ensure uniqueness
                doc = ThreatDocument(
                    id=f"yara_{file_stem}_{rule_name}_{idx}",
                    source="yara",
                    doc_type="rule",
                    title=rule_name,
                    content=f"YARA malware detection rule: {rule_name}. Category: {category}. File: {filepath.name}",
                    metadata={
                        "category": category,
                        "file": str(filepath.name),
                    },
                )
                self.documents.append(doc)
                count += 1

            return count
        except Exception:
            return 0

    def _load_nmap_script(self, filepath: Path) -> int:
        """Load Nmap NSE script."""
        import re
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read(2000)

            desc_match = re.search(r'description\s*=\s*\[\[([^\]]+)\]\]', content, re.DOTALL)
            categories_match = re.search(r'categories\s*=\s*\{([^}]+)\}', content)

            categories = []
            if categories_match:
                categories = [c.strip().strip('"\'') for c in categories_match.group(1).split(",")]

            doc = ThreatDocument(
                id=f"nmap_{filepath.stem}",
                source="nmap",
                doc_type="script",
                title=filepath.stem,
                content=desc_match.group(1)[:500].strip() if desc_match else f"Nmap NSE script: {filepath.stem}",
                metadata={"categories": categories},
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_wazuh_rules(self, filepath: Path) -> int:
        """Load Wazuh SIEM detection rules."""
        import re
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read()

            count = 0
            rule_matches = re.findall(
                r'<rule\s+id="(\d+)"[^>]*>.*?<description>([^<]+)</description>',
                content, re.DOTALL
            )

            for rule_id, description in rule_matches:
                doc = ThreatDocument(
                    id=f"wazuh_{rule_id}",
                    source="wazuh",
                    doc_type="rule",
                    title=f"Wazuh Rule {rule_id}",
                    content=description.strip()[:500],
                    metadata={"rule_id": rule_id, "file": filepath.name},
                )
                self.documents.append(doc)
                count += 1

            return count
        except Exception:
            return 0

    def _load_car_analytic(self, filepath: Path) -> int:
        """Load MITRE CAR analytic."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            if not data or not isinstance(data, dict):
                return 0

            car_id = data.get("id", filepath.stem)
            techniques = [cov.get("technique") for cov in data.get("coverage", []) if cov.get("technique")]

            doc = ThreatDocument(
                id=f"car_{car_id}",
                source="mitre_car",
                doc_type="analytic",
                title=data.get("title", car_id),
                content=data.get("description", "")[:1000],
                metadata={
                    "platforms": data.get("platforms", []),
                    "implementations": len(data.get("implementations", [])),
                },
                mitre_techniques=techniques,
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_caldera_ability(self, filepath: Path) -> int:
        """Load Caldera adversary ability."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            count = 0
            if isinstance(data, list):
                for item in data:
                    if "ability_id" in item:
                        doc = ThreatDocument(
                            id=f"caldera_{item.get('ability_id')}",
                            source="caldera",
                            doc_type="ability",
                            title=item.get("name", item.get("ability_id")),
                            content=item.get("description", "")[:800],
                            metadata={
                                "tactic": item.get("tactic", ""),
                            },
                            mitre_techniques=[item.get("technique_id", "")] if item.get("technique_id") else [],
                        )
                        self.documents.append(doc)
                        count += 1
            return count
        except Exception:
            return 0

    def _load_loldriver(self, filepath: Path) -> int:
        """Load LOLDriver entry."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            if not data:
                return 0

            driver_id = data.get("Id", filepath.stem)
            doc = ThreatDocument(
                id=f"loldriver_{driver_id}",
                source="loldrivers",
                doc_type="driver",
                title=data.get("Name", driver_id),
                content=f"Vulnerable driver: {data.get('Name', '')}. Category: {data.get('Category', '')}",
                metadata={
                    "category": data.get("Category", ""),
                    "verified": data.get("Verified", False),
                },
                platforms=["windows"],
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_hijacklib(self, filepath: Path) -> int:
        """Load HijackLib DLL hijacking entry."""
        import yaml
        try:
            with open(filepath, 'r') as f:
                data = yaml.safe_load(f)

            if not data:
                return 0

            name = data.get("Name", filepath.stem)
            file_stem = filepath.stem
            doc = ThreatDocument(
                id=f"hijacklib_{file_stem}_{name.lower().replace(' ', '_')}",
                source="hijacklibs",
                doc_type="dll_hijack",
                title=name,
                content=f"DLL hijack target: {name}. Vendor: {data.get('Vendor', '')}",
                metadata={
                    "vendor": data.get("Vendor", ""),
                    "locations": data.get("ExpectedLocations", [])[:3],
                },
                platforms=["windows"],
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_wadcom(self, filepath: Path) -> int:
        """Load WADCom Windows/AD cheatsheet entry."""
        try:
            with open(filepath, 'r', errors='ignore') as f:
                content = f.read(1500)

            name = filepath.stem
            doc = ThreatDocument(
                id=f"wadcom_{name}",
                source="wadcoms",
                doc_type="cheatsheet",
                title=name,
                content=f"Windows/AD offensive technique: {name}. {content[200:800]}",
                metadata={},
                platforms=["windows"],
            )
            self.documents.append(doc)
            return 1
        except Exception:
            return 0

    def _load_osint_framework(self, filepath: Path) -> int:
        """Load OSINT Framework structured data."""
        import re
        try:
            with open(filepath, 'r') as f:
                data = json.load(f)

            count = 0

            def parse_node(node, category=""):
                nonlocal count
                if isinstance(node, dict):
                    name = node.get("name", "")
                    url = node.get("url", "")
                    if url and url.startswith("http"):
                        doc = ThreatDocument(
                            id=f"osint_fw_{count}_{re.sub(r'[^a-z0-9]', '_', name.lower())}",
                            source="osint_framework",
                            doc_type="resource",
                            title=name,
                            content=f"OSINT resource: {name}. Category: {category}. URL: {url}",
                            metadata={"url": url, "category": category},
                        )
                        self.documents.append(doc)
                        count += 1

                    for child in node.get("children", []):
                        parse_node(child, name or category)
                elif isinstance(node, list):
                    for item in node:
                        parse_node(item, category)

            parse_node(data)
            return count
        except Exception:
            return 0

    def _load_sherlock_sites(self, filepath: Path) -> int:
        """Load Sherlock username search sites."""
        try:
            with open(filepath, 'r') as f:
                data = json.load(f)

            count = 0
            for site_name, site_data in data.items():
                doc = ThreatDocument(
                    id=f"sherlock_{site_name.lower().replace(' ', '_')}",
                    source="sherlock",
                    doc_type="site",
                    title=site_name,
                    content=f"Username search target: {site_name}. URL pattern: {site_data.get('url', '')}",
                    metadata={"url": site_data.get("url", "")},
                )
                self.documents.append(doc)
                count += 1

            return count
        except Exception:
            return 0

    def generate_embeddings(self) -> int:
        """Generate vector embeddings for all documents."""
        print("\n" + "=" * 70)
        print("Generating Vector Embeddings")
        print("=" * 70)

        if not self.documents:
            print("No documents loaded!")
            return 0

        # Deduplicate documents by ID (keep first occurrence)
        seen_ids = set()
        unique_docs = []
        duplicates = 0
        for doc in self.documents:
            if doc.id not in seen_ids:
                seen_ids.add(doc.id)
                unique_docs.append(doc)
            else:
                duplicates += 1

        if duplicates > 0:
            print(f"Removed {duplicates} duplicate IDs, {len(unique_docs)} unique documents remain")

        texts = [doc.to_embedding_text() for doc in unique_docs]
        ids = [doc.id for doc in unique_docs]

        if self.embedding_model:
            # Use sentence-transformers
            print(f"Encoding {len(texts)} documents with {self.model_name}...")
            embeddings = self.embedding_model.encode(
                texts,
                show_progress_bar=True,
                convert_to_numpy=True
            )

            for i, doc_id in enumerate(ids):
                self.vectors[doc_id] = embeddings[i].tolist()

            # Save to ChromaDB if available
            if self.collection:
                print("Saving to ChromaDB...")
                # Batch insert
                batch_size = 1000
                for i in range(0, len(ids), batch_size):
                    batch_ids = ids[i:i+batch_size]
                    batch_embeddings = [self.vectors[did] for did in batch_ids]
                    batch_docs = [texts[j] for j in range(i, min(i+batch_size, len(texts)))]
                    batch_metadata = [
                        {
                            "source": unique_docs[j].source,
                            "doc_type": unique_docs[j].doc_type,
                            "title": unique_docs[j].title[:200],
                        }
                        for j in range(i, min(i+batch_size, len(unique_docs)))
                    ]

                    self.collection.upsert(
                        ids=batch_ids,
                        embeddings=batch_embeddings,
                        documents=batch_docs,
                        metadatas=batch_metadata,
                    )
                print(f"Saved {len(ids)} vectors to ChromaDB")

        else:
            # Fallback: hash-based pseudo-vectors
            print("Using hash-based vectors (install sentence-transformers for real embeddings)")
            for i, doc_id in enumerate(ids):
                # Create deterministic 384-dim vector from hash
                text_hash = hashlib.sha384(texts[i].encode()).digest()
                vec = [b / 255.0 for b in text_hash]
                self.vectors[doc_id] = vec

        # Save vectors to JSON
        vectors_file = VECTOR_DIR / "threat_vectors.json"
        with open(vectors_file, 'w') as f:
            json.dump({
                "model": self.model_name,
                "count": len(self.vectors),
                "dimension": len(next(iter(self.vectors.values()))) if self.vectors else 0,
                "vectors": self.vectors,
            }, f)
        print(f"Saved vectors to {vectors_file}")

        return len(self.vectors)

    def generate_phi3_training_data(self) -> int:
        """Generate LoRA training data for Phi-3 (generative threat analysis)."""
        print("\n" + "=" * 70)
        print("Generating Phi-3 LoRA Training Data")
        print("=" * 70)

        training_data = []

        # Training format: instruction-response pairs
        for doc in self.documents:
            # Type 1: Technique explanation
            if doc.doc_type == "technique":
                training_data.append({
                    "instruction": f"Explain the MITRE ATT&CK technique {doc.mitre_techniques[0] if doc.mitre_techniques else 'unknown'}: {doc.title}",
                    "input": "",
                    "output": doc.content[:1500],
                    "metadata": {"type": "technique_explanation", "source": doc.source}
                })

            # Type 2: Detection rule generation
            elif doc.doc_type == "rule":
                training_data.append({
                    "instruction": f"Generate a detection rule for: {doc.title}",
                    "input": f"MITRE Techniques: {', '.join(doc.mitre_techniques)}" if doc.mitre_techniques else "",
                    "output": doc.content[:1500],
                    "metadata": {"type": "rule_generation", "source": doc.source}
                })

            # Type 3: Test case generation
            elif doc.doc_type == "test":
                training_data.append({
                    "instruction": f"Create an atomic test for technique {doc.mitre_techniques[0] if doc.mitre_techniques else 'unknown'}",
                    "input": f"Platforms: {', '.join(doc.platforms)}" if doc.platforms else "",
                    "output": f"Test: {doc.title}\n{doc.content[:1200]}",
                    "metadata": {"type": "test_generation", "source": doc.source}
                })

            # Type 4: LOLBin analysis
            elif doc.source in ["lolbas", "gtfobins"]:
                training_data.append({
                    "instruction": f"Analyze the security implications of {doc.title}",
                    "input": f"Platform: {', '.join(doc.platforms)}" if doc.platforms else "",
                    "output": doc.content[:1500],
                    "metadata": {"type": "lolbin_analysis", "source": doc.source}
                })

            # Type 5: OSINT tool description
            elif doc.source == "osint":
                training_data.append({
                    "instruction": f"Describe the OSINT tool: {doc.title}",
                    "input": "",
                    "output": doc.content[:1000],
                    "metadata": {"type": "osint_description", "source": doc.source}
                })

        # Save in multiple formats

        # 1. JSON Lines (for transformers)
        jsonl_file = TRAINING_DIR / "phi3_lora_training.jsonl"
        with open(jsonl_file, 'w') as f:
            for item in training_data:
                f.write(json.dumps(item) + "\n")

        # 2. Alpaca format
        alpaca_file = TRAINING_DIR / "phi3_alpaca_format.json"
        with open(alpaca_file, 'w') as f:
            json.dump(training_data, f, indent=2)

        # 3. Chat format for Phi-3
        chat_data = []
        for item in training_data:
            chat_data.append({
                "messages": [
                    {"role": "user", "content": item["instruction"] + ("\n" + item["input"] if item["input"] else "")},
                    {"role": "assistant", "content": item["output"]}
                ]
            })

        chat_file = TRAINING_DIR / "phi3_chat_format.jsonl"
        with open(chat_file, 'w') as f:
            for item in chat_data:
                f.write(json.dumps(item) + "\n")

        print(f"Generated {len(training_data)} Phi-3 training examples")
        print(f"  - {jsonl_file}")
        print(f"  - {alpaca_file}")
        print(f"  - {chat_file}")

        return len(training_data)

    def generate_distilbert_training_data(self) -> int:
        """Generate LoRA training data for DistilBERT (classification/NER)."""
        print("\n" + "=" * 70)
        print("Generating DistilBERT LoRA Training Data")
        print("=" * 70)

        # Classification data: text -> technique label
        classification_data = []

        # NER data: text with entity annotations
        ner_data = []

        # Multi-label classification: text -> [tactics]
        multilabel_data = []

        for doc in self.documents:
            # Classification: predict primary technique
            if doc.mitre_techniques:
                classification_data.append({
                    "text": doc.to_embedding_text()[:512],
                    "label": doc.mitre_techniques[0],
                    "source": doc.source,
                })

            # Multi-label: predict tactics
            if doc.tactics:
                multilabel_data.append({
                    "text": doc.to_embedding_text()[:512],
                    "labels": doc.tactics,
                    "source": doc.source,
                })

            # NER: annotate technique IDs in text
            text = doc.content[:512]
            entities = []
            for tech in doc.mitre_techniques:
                start = text.find(tech)
                if start >= 0:
                    entities.append({
                        "start": start,
                        "end": start + len(tech),
                        "label": "TECHNIQUE",
                        "text": tech,
                    })

            if entities:
                ner_data.append({
                    "text": text,
                    "entities": entities,
                })

        # Save classification data
        cls_file = TRAINING_DIR / "distilbert_classification.jsonl"
        with open(cls_file, 'w') as f:
            for item in classification_data:
                f.write(json.dumps(item) + "\n")

        # Save multi-label data
        ml_file = TRAINING_DIR / "distilbert_multilabel.jsonl"
        with open(ml_file, 'w') as f:
            for item in multilabel_data:
                f.write(json.dumps(item) + "\n")

        # Save NER data
        ner_file = TRAINING_DIR / "distilbert_ner.jsonl"
        with open(ner_file, 'w') as f:
            for item in ner_data:
                f.write(json.dumps(item) + "\n")

        # Create label mapping
        all_techniques = set()
        all_tactics = set()
        for doc in self.documents:
            all_techniques.update(doc.mitre_techniques)
            all_tactics.update(doc.tactics)

        label_map = {
            "techniques": {tech: i for i, tech in enumerate(sorted(all_techniques))},
            "tactics": {tac: i for i, tac in enumerate(sorted(all_tactics))},
        }

        label_file = TRAINING_DIR / "distilbert_label_map.json"
        with open(label_file, 'w') as f:
            json.dump(label_map, f, indent=2)

        print(f"Generated DistilBERT training data:")
        print(f"  Classification: {len(classification_data)} examples ({len(all_techniques)} techniques)")
        print(f"  Multi-label: {len(multilabel_data)} examples ({len(all_tactics)} tactics)")
        print(f"  NER: {len(ner_data)} examples")

        return len(classification_data) + len(multilabel_data) + len(ner_data)

    def generate_cypher_queries(self) -> int:
        """Generate Neo4j Cypher queries for threat graph."""
        print("\n" + "=" * 70)
        print("Generating Neo4j Cypher Queries")
        print("=" * 70)

        cypher_statements = []

        # Schema creation
        cypher_statements.append("""
// ============================================================
// CTAS-7 Threat Graph Schema - Neo4j Cypher
// RFC-9011: Threat Content Graph Database
// ============================================================

// Create constraints and indexes
CREATE CONSTRAINT technique_id IF NOT EXISTS FOR (t:Technique) REQUIRE t.id IS UNIQUE;
CREATE CONSTRAINT tactic_name IF NOT EXISTS FOR (t:Tactic) REQUIRE t.name IS UNIQUE;
CREATE CONSTRAINT tool_name IF NOT EXISTS FOR (t:Tool) REQUIRE t.name IS UNIQUE;
CREATE CONSTRAINT rule_id IF NOT EXISTS FOR (r:Rule) REQUIRE r.id IS UNIQUE;
CREATE CONSTRAINT test_id IF NOT EXISTS FOR (t:Test) REQUIRE t.id IS UNIQUE;
CREATE CONSTRAINT group_id IF NOT EXISTS FOR (g:Group) REQUIRE g.id IS UNIQUE;
CREATE CONSTRAINT platform_name IF NOT EXISTS FOR (p:Platform) REQUIRE p.name IS UNIQUE;

CREATE INDEX technique_name IF NOT EXISTS FOR (t:Technique) ON (t.name);
CREATE INDEX tool_source IF NOT EXISTS FOR (t:Tool) ON (t.source);
""")

        # Create tactics
        tactics = set()
        for doc in self.documents:
            tactics.update(doc.tactics)

        for tactic in sorted(tactics):
            if tactic:
                cypher_statements.append(f"""
MERGE (t:Tactic {{name: "{tactic}"}});""")

        # Create platforms
        platforms = set()
        for doc in self.documents:
            platforms.update(doc.platforms)

        for platform in sorted(platforms):
            if platform:
                cypher_statements.append(f"""
MERGE (p:Platform {{name: "{platform}"}});""")

        # Create techniques with relationships
        technique_docs = [d for d in self.documents if d.doc_type == "technique"]
        for doc in technique_docs:
            if doc.mitre_techniques:
                tech_id = doc.mitre_techniques[0]
                title_escaped = doc.title.replace('"', '\\"').replace("'", "\\'")
                content_escaped = doc.content[:500].replace('"', '\\"').replace("'", "\\'").replace('\n', ' ')

                cypher_statements.append(f"""
MERGE (t:Technique {{id: "{tech_id}"}})
SET t.name = "{title_escaped}",
    t.description = "{content_escaped}",
    t.source = "mitre_attack";""")

                # Link to tactics
                for tactic in doc.tactics:
                    if tactic:
                        cypher_statements.append(f"""
MATCH (tech:Technique {{id: "{tech_id}"}}), (tac:Tactic {{name: "{tactic}"}})
MERGE (tech)-[:BELONGS_TO]->(tac);""")

                # Link to platforms
                for platform in doc.platforms:
                    if platform:
                        cypher_statements.append(f"""
MATCH (tech:Technique {{id: "{tech_id}"}}), (p:Platform {{name: "{platform}"}})
MERGE (tech)-[:TARGETS]->(p);""")

        # Create tests linked to techniques
        test_docs = [d for d in self.documents if d.doc_type == "test"]
        for doc in test_docs[:500]:  # Limit for file size
            title_escaped = doc.title.replace('"', '\\"').replace("'", "\\'")[:100]
            cypher_statements.append(f"""
MERGE (test:Test {{id: "{doc.id}"}})
SET test.name = "{title_escaped}",
    test.source = "{doc.source}";""")

            for tech_id in doc.mitre_techniques:
                cypher_statements.append(f"""
MATCH (test:Test {{id: "{doc.id}"}}), (tech:Technique {{id: "{tech_id}"}})
MERGE (test)-[:TESTS]->(tech);""")

        # Create rules linked to techniques
        rule_docs = [d for d in self.documents if d.doc_type == "rule"]
        for doc in rule_docs[:500]:
            title_escaped = doc.title.replace('"', '\\"').replace("'", "\\'")[:100]
            cypher_statements.append(f"""
MERGE (rule:Rule {{id: "{doc.id}"}})
SET rule.name = "{title_escaped}",
    rule.source = "{doc.source}";""")

            for tech_id in doc.mitre_techniques:
                cypher_statements.append(f"""
MATCH (rule:Rule {{id: "{doc.id}"}}), (tech:Technique {{id: "{tech_id}"}})
MERGE (rule)-[:DETECTS]->(tech);""")

        # Create tools (LOLBAS, GTFOBins, OSINT)
        tool_docs = [d for d in self.documents if d.doc_type in ["binary", "tool"]]
        for doc in tool_docs:
            title_escaped = doc.title.replace('"', '\\"').replace("'", "\\'")
            cypher_statements.append(f"""
MERGE (tool:Tool {{name: "{title_escaped}"}})
SET tool.id = "{doc.id}",
    tool.source = "{doc.source}";""")

            for tech_id in doc.mitre_techniques:
                cypher_statements.append(f"""
MATCH (tool:Tool {{name: "{title_escaped}"}}), (tech:Technique {{id: "{tech_id}"}})
MERGE (tool)-[:IMPLEMENTS]->(tech);""")

            for platform in doc.platforms:
                if platform:
                    cypher_statements.append(f"""
MATCH (tool:Tool {{name: "{title_escaped}"}}), (p:Platform {{name: "{platform}"}})
MERGE (tool)-[:RUNS_ON]->(p);""")

        # Add useful query templates
        cypher_statements.append("""
// ============================================================
// USEFUL QUERY TEMPLATES
// ============================================================

// Find all tests for a technique
// MATCH (test:Test)-[:TESTS]->(tech:Technique {id: "T1059"})
// RETURN test.name, test.source;

// Find detection coverage for a technique
// MATCH (rule:Rule)-[:DETECTS]->(tech:Technique {id: "T1059"})
// RETURN rule.name, rule.source;

// Find tools implementing a technique
// MATCH (tool:Tool)-[:IMPLEMENTS]->(tech:Technique {id: "T1059"})
// RETURN tool.name, tool.source;

// Find techniques by tactic
// MATCH (tech:Technique)-[:BELONGS_TO]->(tac:Tactic {name: "execution"})
// RETURN tech.id, tech.name;

// Find Windows-specific LOLBins
// MATCH (tool:Tool {source: "lolbas"})-[:RUNS_ON]->(p:Platform {name: "windows"})
// RETURN tool.name;

// Find technique coverage gaps (techniques without detection rules)
// MATCH (tech:Technique)
// WHERE NOT EXISTS((tech)<-[:DETECTS]-(:Rule))
// RETURN tech.id, tech.name;

// Find cross-platform techniques
// MATCH (tech:Technique)-[:TARGETS]->(p:Platform)
// WITH tech, COUNT(p) as platform_count
// WHERE platform_count > 1
// RETURN tech.id, tech.name, platform_count
// ORDER BY platform_count DESC;
""")

        # Write to file
        cypher_file = CYPHER_DIR / "threat_graph.cypher"
        with open(cypher_file, 'w') as f:
            f.write("\n".join(cypher_statements))

        # Also create a separate file for data import
        data_file = CYPHER_DIR / "threat_data_import.cypher"
        with open(data_file, 'w') as f:
            # Skip schema, just data
            f.write("\n".join([s for s in cypher_statements if "MERGE" in s or "MATCH" in s]))

        print(f"Generated {len(cypher_statements)} Cypher statements")
        print(f"  Schema + Data: {cypher_file}")
        print(f"  Data only: {data_file}")

        # Generate summary stats
        stats = {
            "techniques": len(technique_docs),
            "tests": len(test_docs),
            "rules": len(rule_docs),
            "tools": len(tool_docs),
            "tactics": len(tactics),
            "platforms": len(platforms),
        }

        stats_file = CYPHER_DIR / "graph_stats.json"
        with open(stats_file, 'w') as f:
            json.dump(stats, f, indent=2)

        return len(cypher_statements)

    def embed_atl_physical(self) -> int:
        """Embed ATL-Physical tasks into ChromaDB.

        ATL-Physical is INVISIBLE operationally but INCLUDED in training
        for cross-domain pattern recognition.

        Returns:
            Number of documents embedded
        """
        if not HAS_ATL_PHYSICAL:
            print("[ATL-Physical] Loader not available, skipping")
            return 0

        docs = load_atl_physical()

        if not docs:
            print("[ATL-Physical] No documents to embed")
            return 0

        print(f"\n[ATL-Physical] Embedding {len(docs)} documents...")

        # Convert to ThreatDocument format and add to documents list
        for doc in docs:
            if doc.get('type') != 'adversary_task':
                continue

            threat_doc = ThreatDocument(
                id=doc['id'],
                source='ATL-Physical',
                doc_type='adversary_task',
                title=doc.get('metadata', {}).get('task_id', 'unknown'),
                content=doc.get('text', ''),
                metadata={
                    'domain': 'physical',
                    'modality': doc.get('modality', 'IED'),
                    'phase': doc.get('phase', 0),
                    'hd4_phases': doc.get('hd4_phases', []),
                    'is_mandatory': doc.get('is_mandatory', False),
                    'is_interdiction_point': doc.get('is_interdiction_point', False),
                },
                mitre_techniques=[],
                tactics=[],
                platforms=['physical'],
            )
            self.documents.append(threat_doc)

        # If ChromaDB collection exists, add directly
        if self.collection and self.embedding_model:
            ids = []
            documents = []
            metadatas = []

            for doc in docs:
                ids.append(doc['id'])
                documents.append(doc.get('text', ''))
                metadatas.append({
                    'source': 'ATL-Physical',
                    'domain': 'physical',
                    'type': doc.get('type', 'adversary_task'),
                    'phase': doc.get('phase', 0),
                    'hd4_phases': ','.join(doc.get('hd4_phases', [])) if isinstance(doc.get('hd4_phases'), list) else str(doc.get('hd4_phases', '')),
                    'is_mandatory': doc.get('is_mandatory', False),
                    'is_interdiction_point': doc.get('is_interdiction_point', False),
                    'modality': doc.get('modality', 'IED'),
                })

            # Batch add
            batch_size = 500
            for i in range(0, len(ids), batch_size):
                batch_ids = ids[i:i+batch_size]
                batch_docs = documents[i:i+batch_size]
                batch_meta = metadatas[i:i+batch_size]

                self.collection.add(
                    ids=batch_ids,
                    documents=batch_docs,
                    metadatas=batch_meta
                )

            print(f"[ATL-Physical] Embedded {len(docs)} documents to ChromaDB")

        return len(docs)

    def run_full_pipeline(self, threat_dir: Path) -> Dict[str, int]:
        """Run the complete vectorization and training data pipeline."""
        results = {}

        # 1. Load content
        results["documents_loaded"] = self.load_threat_content(threat_dir)

        # 1b. Load ATL sources (invisible operationally, included in training)
        results["atl_physical_loaded"] = self.embed_atl_physical()
        # Future: results["atl_cyber_loaded"] = self.embed_atl_cyber()

        # 2. Generate embeddings
        results["vectors_generated"] = self.generate_embeddings()

        # 3. Generate Phi-3 training data
        results["phi3_examples"] = self.generate_phi3_training_data()

        # 4. Generate DistilBERT training data
        results["distilbert_examples"] = self.generate_distilbert_training_data()

        # 5. Generate Cypher queries
        results["cypher_statements"] = self.generate_cypher_queries()

        print("\n" + "=" * 70)
        print("PIPELINE COMPLETE")
        print("=" * 70)
        for key, value in results.items():
            print(f"  {key}: {value}")

        return results


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Threat Vector Pipeline")
    parser.add_argument("--threat-dir", type=str,
                       default=str(OUTPUT_DIR / "threat_content"),
                       help="Directory containing fetched threat content")
    parser.add_argument("--model", type=str, default="all-MiniLM-L6-v2",
                       help="Sentence transformer model for embeddings")
    parser.add_argument("--vectors-only", action="store_true",
                       help="Only generate vectors")
    parser.add_argument("--training-only", action="store_true",
                       help="Only generate training data")
    parser.add_argument("--cypher-only", action="store_true",
                       help="Only generate Cypher queries")
    args = parser.parse_args()

    threat_dir = Path(args.threat_dir)
    if not threat_dir.exists():
        print(f"Error: Threat content directory not found: {threat_dir}")
        print("Run threat_content_fetcher.py --all first")
        return

    pipeline = ThreatVectorPipeline(model_name=args.model)

    if args.vectors_only:
        pipeline.load_threat_content(threat_dir)
        pipeline.generate_embeddings()
    elif args.training_only:
        pipeline.load_threat_content(threat_dir)
        pipeline.generate_phi3_training_data()
        pipeline.generate_distilbert_training_data()
    elif args.cypher_only:
        pipeline.load_threat_content(threat_dir)
        pipeline.generate_cypher_queries()
    else:
        pipeline.run_full_pipeline(threat_dir)


if __name__ == "__main__":
    main()
