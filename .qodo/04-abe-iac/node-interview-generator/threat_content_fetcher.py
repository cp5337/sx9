#!/usr/bin/env python3
"""
RFC-9011: SX9 Threat Content Ingestion
RFC-9023: Security Framework Integration Map

Comprehensive threat content fetcher:
- MITRE Suite: ATT&CK (Enterprise/ICS/Mobile), D3FEND, ENGAGE, ATLAS, CAR
- Adversary Emulation: Caldera, Atomic Red Team
- Detection: Nuclei, Sigma, YARA, Wazuh
- Scanning: Nmap NSE scripts
- Exploits: ExploitDB, NVD CVE
- Kali Tools: Integrated from ctas7-exploit-arsenal inventory
"""

import json
import os
import subprocess
import requests
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional
import yaml
import re
import hashlib
import sys

# SPIRES integration for ontology generation
try:
    # Add parent directory to path for SPIRES imports
    spires_path = Path(__file__).parent.parent.parent / "04-abe-iac"
    if str(spires_path) not in sys.path:
        sys.path.insert(0, str(spires_path))
    
    from spires_ontology_extractor import SPIRESExtractor, OntologyGraph, generate_json_export, generate_cypher_export, generate_surreal_export, generate_linkml_schema
    from spires_threat_extractor import SPIRESThreatExtractor
    SPIRES_AVAILABLE = True
except ImportError as e:
    SPIRES_AVAILABLE = False
    print(f"⚠️  SPIRES not available: {e}")
    print("   Ontology generation will be skipped. Install ontogpt for full SPIRES support.")

# YAML to DSL conversion pipeline
try:
    # Import YAML DSL pipeline from same directory
    from yaml_dsl_pipeline import YAMLDSLPipeline
    DSL_PIPELINE_AVAILABLE = True
except ImportError as e:
    DSL_PIPELINE_AVAILABLE = False
    print(f"⚠️  YAML DSL pipeline not available: {e}")
    print("   YAML to DSL conversion will be skipped.")

# ML Model Training integration
try:
    # Add sx9-conda to path for ML models
    sx9_conda_path = Path.home() / "Developer" / "sx9-conda" / "python-packages"
    if str(sx9_conda_path) not in sys.path:
        sys.path.insert(0, str(sx9_conda_path))
    
    from sx9_ml_models.training import (
        train_distilbert,
        train_phi_lora,
        train_gnn,
        TrainingConfig
    )
    TRAINING_AVAILABLE = True
except ImportError as e:
    TRAINING_AVAILABLE = False
    print(f"⚠️  ML Model Training not available: {e}")
    print("   Model training will be skipped. Install sx9-ml-models for training support.")

OUTPUT_DIR = Path(__file__).parent / "output"
THREAT_DIR = OUTPUT_DIR / "threat_content"

# Comprehensive source URLs per RFC-9011 and RFC-9023
SOURCES = {
    # ========== MITRE ATT&CK Suite ==========
    "mitre_attack": {
        "url": "https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json",
        "type": "json",
        "description": "MITRE ATT&CK Enterprise techniques",
        "category": "mitre"
    },
    "mitre_attack_ics": {
        "url": "https://raw.githubusercontent.com/mitre/cti/master/ics-attack/ics-attack.json",
        "type": "json",
        "description": "MITRE ATT&CK for ICS",
        "category": "mitre"
    },
    "mitre_attack_mobile": {
        "url": "https://raw.githubusercontent.com/mitre/cti/master/mobile-attack/mobile-attack.json",
        "type": "json",
        "description": "MITRE ATT&CK Mobile",
        "category": "mitre"
    },

    # ========== MITRE Defense Suite ==========
    "mitre_d3fend": {
        "url": "https://d3fend.mitre.org/ontologies/d3fend.json",
        "type": "json",
        "description": "MITRE D3FEND countermeasures ontology",
        "category": "mitre_defense"
    },
    "mitre_car": {
        "repo": "https://github.com/mitre-attack/car",
        "path": "analytics",
        "type": "yaml",
        "description": "MITRE Cyber Analytics Repository",
        "category": "mitre_defense"
    },
    "mitre_engage": {
        "repo": "https://github.com/mitre/engage",
        "path": "data",
        "type": "yaml",
        "description": "MITRE ENGAGE adversary engagement",
        "category": "mitre_defense"
    },
    "mitre_atlas": {
        "repo": "https://github.com/mitre-atlas/atlas-data",
        "path": "data",
        "type": "yaml",
        "description": "MITRE ATLAS AI/ML adversarial attacks",
        "category": "mitre_defense"
    },

    # ========== Adversary Emulation ==========
    "atomic_red_team": {
        "repo": "https://github.com/redcanaryco/atomic-red-team",
        "path": "atomics",
        "type": "yaml",
        "description": "Atomic Red Team test definitions",
        "category": "emulation"
    },
    "caldera": {
        "repo": "https://github.com/mitre/caldera",
        "path": "data",
        "type": "yaml",
        "description": "MITRE Caldera adversary emulation",
        "category": "emulation"
    },

    # ========== Vulnerability/Detection ==========
    "nuclei_templates": {
        "repo": "https://github.com/projectdiscovery/nuclei-templates",
        "type": "yaml",
        "description": "Nuclei vulnerability templates",
        "category": "detection"
    },
    "sigma_rules": {
        "repo": "https://github.com/SigmaHQ/sigma",
        "path": "rules",
        "type": "yaml",
        "description": "Sigma detection rules",
        "category": "detection"
    },
    "yara_rules": {
        "repo": "https://github.com/Yara-Rules/rules",
        "path": ".",
        "type": "yar",
        "description": "YARA malware detection rules",
        "category": "detection"
    },
    "wazuh_rules": {
        "repo": "https://github.com/wazuh/wazuh",
        "path": "ruleset/rules",
        "type": "xml",
        "description": "Wazuh SIEM detection rules",
        "category": "detection"
    },

    # ========== Reconnaissance/Scanning ==========
    "nmap_scripts": {
        "repo": "https://github.com/nmap/nmap",
        "path": "scripts",
        "type": "nse",
        "description": "Nmap NSE scripts",
        "category": "recon"
    },

    # ========== Exploit/CVE Data ==========
    "exploitdb": {
        "repo": "https://gitlab.com/exploit-database/exploitdb",
        "path": "exploits",
        "type": "various",
        "description": "Exploit-DB exploits",
        "category": "exploits"
    },
    "cve_nvd": {
        "url": "https://services.nvd.nist.gov/rest/json/cves/2.0",
        "type": "api",
        "description": "NIST NVD CVE database",
        "category": "exploits"
    },

    # ========== Threat Intel Feeds ==========
    "alienvault_otx": {
        "url": "https://otx.alienvault.com/api/v1/pulses/subscribed",
        "type": "api",
        "description": "AlienVault OTX threat intel",
        "category": "intel"
    },
    "abuse_ch": {
        "url": "https://feodotracker.abuse.ch/downloads/ipblocklist.json",
        "type": "json",
        "description": "abuse.ch IOC feeds",
        "category": "intel"
    },
}

# Kali tools categories from ctas7-exploit-arsenal/kali_tools_inventory.rs
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

# OSINT Resources
OSINT_SOURCES = {
    "awesome_osint": {
        "repo": "https://github.com/jivoi/awesome-osint",
        "path": "README.md",
        "type": "md",
        "description": "Awesome OSINT - comprehensive OSINT tools list",
        "category": "osint"
    },
    "osint_framework": {
        "repo": "https://github.com/lockfale/osint-framework",
        "path": "arf.json",
        "type": "json",
        "description": "OSINT Framework - structured OSINT resource tree",
        "category": "osint"
    },
    "bellingcat_toolkit": {
        "repo": "https://github.com/bellingcat/toolkit",
        "path": "src/data",
        "type": "yaml",
        "description": "Bellingcat Online Investigation Toolkit",
        "category": "osint"
    },
    "osintagram": {
        "repo": "https://github.com/Datalux/Osintgram",
        "type": "python",
        "description": "Instagram OSINT tool",
        "category": "osint"
    },
    "socialpwned": {
        "repo": "https://github.com/MrTuxx/SocialPwned",
        "type": "python",
        "description": "OSINT tool for social engineering",
        "category": "osint"
    },
    "phoneinfoga": {
        "repo": "https://github.com/sundowndev/phoneinfoga",
        "type": "go",
        "description": "Phone number OSINT framework",
        "category": "osint"
    },
    "sherlock": {
        "repo": "https://github.com/sherlock-project/sherlock",
        "path": "sherlock/resources/data.json",
        "type": "json",
        "description": "Hunt usernames across social networks",
        "category": "osint"
    },
    "maigret": {
        "repo": "https://github.com/soxoj/maigret",
        "path": "maigret/resources/data.json",
        "type": "json",
        "description": "Username enumeration across 3000+ sites",
        "category": "osint"
    },
    "holehe": {
        "repo": "https://github.com/megadose/holehe",
        "type": "python",
        "description": "Check if email is used on different sites",
        "category": "osint"
    },
    "ghunt": {
        "repo": "https://github.com/mxrch/GHunt",
        "type": "python",
        "description": "Google account investigation tool",
        "category": "osint"
    },
}

# LOLBAS/LOLBins - Living Off The Land resources
LOLTL_SOURCES = {
    "lolbas": {
        "repo": "https://github.com/LOLBAS-Project/LOLBAS",
        "path": "yml/OSBinaries",
        "type": "yaml",
        "description": "LOLBAS - Living Off The Land Binaries and Scripts (Windows)",
        "category": "loltl"
    },
    "gtfobins": {
        "repo": "https://github.com/GTFOBins/GTFOBins.github.io",
        "path": "_gtfobins",
        "type": "md",
        "description": "GTFOBins - Unix binaries for privilege escalation",
        "category": "loltl"
    },
    "loflcab": {
        "repo": "https://github.com/LOLBAS-Project/LOLBAS",
        "path": "yml/OSLibraries",
        "type": "yaml",
        "description": "LOLBins - Living Off The Land Libraries",
        "category": "loltl"
    },
    "wadcoms": {
        "repo": "https://github.com/WADComs/WADComs.github.io",
        "path": "_wadcoms",
        "type": "md",
        "description": "WADComs - Windows/AD offensive cheatsheets",
        "category": "loltl"
    },
    "hijacklibs": {
        "repo": "https://github.com/wietze/HijackLibs",
        "path": "yml",
        "type": "yaml",
        "description": "HijackLibs - DLL Hijacking database",
        "category": "loltl"
    },
    "loldrivers": {
        "repo": "https://github.com/magicsword-io/LOLDrivers",
        "path": "yaml/drivers",
        "type": "yaml",
        "description": "LOLDrivers - Vulnerable Windows drivers",
        "category": "loltl"
    },
}


class ThreatContentFetcher:
    """Fetches and indexes threat content from multiple sources."""

    def __init__(self, cache_dir: Path = THREAT_DIR, cleanup_repos_after_processing: bool = False):
        self.cache_dir = cache_dir
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        self.cleanup_repos_after_processing = cleanup_repos_after_processing

        self.content: Dict[str, Dict] = {
            # MITRE ATT&CK Suite
            "mitre_techniques": {},
            "mitre_groups": {},
            "mitre_ics_techniques": {},
            "mitre_mobile_techniques": {},
            # MITRE Defense Suite
            "d3fend_techniques": {},
            "car_analytics": {},
            "atlas_techniques": {},
            "atlas_case_studies": {},
            "engage_activities": {},
            "engage_activities": {},
            # Adversary Emulation
            "atomic_tests": {},
            "caldera_abilities": {},
            # Detection
            "nuclei_templates": {},
            "sigma_rules": {},
            "yara_rules": {},
            "wazuh_rules": {},
            # Recon
            "nmap_scripts": {},
            # LOLTL - Living Off The Land
            "lolbas_binaries": {},
            "gtfobins": {},
            "loldrivers": {},
            "hijacklibs": {},
            "wadcoms": {},
            # Exploits
            "exploitdb": {},
            # Kali Tools
            "kali_tools": {},
            # OSINT Tools
            "osint_tools": {},
            "osint_sites": {},
            # CTAS Internal Systems
            "ptcc_configurations": {},
            "teth_algorithms": {},
            "ptcc_tool_chains": {},  # Tool chains derived from PTCC configs
        }

        self.crosswalk: Dict[str, Dict] = {
            "technique_to_atomic": {},
            "technique_to_nuclei": {},
            "technique_to_sigma": {},
            "technique_to_car": {},
            "technique_to_lolbas": {},
        }

    def fetch_mitre_attack(self) -> int:
        """Fetch MITRE ATT&CK Enterprise data."""
        print("\n[1/6] Fetching MITRE ATT&CK...")
        cache_file = self.cache_dir / "mitre_attack.json"

        # Skip if already exists
        if cache_file.exists():
            print(f"  ✅ Already exists: {cache_file.name} ({cache_file.stat().st_size:,} bytes)")
            try:
                with open(cache_file) as f:
                    data = json.load(f)
            except Exception as e:
                print(f"  ⚠️  Error reading existing file, re-downloading: {e}")
                data = None
        else:
            data = None

        if data is None:
            try:
                response = requests.get(SOURCES["mitre_attack"]["url"], timeout=60)
                response.raise_for_status()
                data = response.json()

                with open(cache_file, 'w') as f:
                    json.dump(data, f)
                print(f"  ✅ Downloaded: {cache_file.name}")
            except Exception as e:
                print(f"  ERROR: {e}")
                return 0

        try:
            # Parse techniques
            for obj in data.get("objects", []):
                if obj.get("type") == "attack-pattern":
                    refs = obj.get("external_references", [])
                    for ref in refs:
                        if ref.get("source_name") == "mitre-attack":
                            tech_id = ref.get("external_id")
                            if tech_id:
                                self.content["mitre_techniques"][tech_id] = {
                                    "id": tech_id,
                                    "name": obj.get("name"),
                                    "tactics": [p.get("phase_name") for p in obj.get("kill_chain_phases", [])],
                                    "platforms": obj.get("x_mitre_platforms", []),
                                    "description": obj.get("description", "")[:500],
                                }

                elif obj.get("type") == "intrusion-set":
                    refs = obj.get("external_references", [])
                    for ref in refs:
                        if ref.get("source_name") == "mitre-attack":
                            group_id = ref.get("external_id")
                            if group_id:
                                self.content["mitre_groups"][group_id] = {
                                    "id": group_id,
                                    "name": obj.get("name"),
                                    "aliases": obj.get("aliases", []),
                                }

            print(f"  Loaded {len(self.content['mitre_techniques'])} techniques, {len(self.content['mitre_groups'])} groups")
            return len(self.content["mitre_techniques"])

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_atomic_red_team(self, clone_depth: int = 1) -> int:
        """Fetch Atomic Red Team test definitions."""
        print("\n[2/6] Fetching Atomic Red Team...")
        repo_dir = self.cache_dir / "atomic-red-team"

        try:
            if repo_dir.exists():
                # Pull latest
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                # Clone
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["atomic_red_team"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse atomics
            atomics_dir = repo_dir / "atomics"
            count = 0

            for tech_dir in atomics_dir.iterdir():
                if tech_dir.is_dir() and tech_dir.name.startswith("T"):
                    yaml_file = tech_dir / f"{tech_dir.name}.yaml"
                    if yaml_file.exists():
                        try:
                            with open(yaml_file, 'r') as f:
                                data = yaml.safe_load(f)

                            tech_id = data.get("attack_technique")
                            if tech_id:
                                tests = data.get("atomic_tests", [])
                                self.content["atomic_tests"][tech_id] = {
                                    "technique_id": tech_id,
                                    "display_name": data.get("display_name"),
                                    "test_count": len(tests),
                                    "tests": [
                                        {
                                            "name": t.get("name"),
                                            "platforms": t.get("supported_platforms", []),
                                            "executor": t.get("executor", {}).get("name"),
                                        }
                                        for t in tests[:5]  # Limit for storage
                                    ]
                                }
                                self.crosswalk["technique_to_atomic"][tech_id] = len(tests)
                                count += 1

                        except Exception as e:
                            pass

            print(f"  Loaded {count} technique test definitions")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_nuclei_templates(self, clone_depth: int = 1) -> int:
        """Fetch Nuclei vulnerability templates."""
        print("\n[3/6] Fetching Nuclei templates...")
        repo_dir = self.cache_dir / "nuclei-templates"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["nuclei_templates"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse templates by category
            categories = {}
            count = 0

            for yaml_file in repo_dir.rglob("*.yaml"):
                try:
                    with open(yaml_file, 'r') as f:
                        content = f.read()
                        # Quick parse just the info section
                        if "id:" in content and "info:" in content:
                            # Extract template ID
                            id_match = re.search(r'^id:\s*(.+)$', content, re.MULTILINE)
                            severity_match = re.search(r'severity:\s*(\w+)', content)
                            tags_match = re.search(r'tags:\s*(.+)$', content, re.MULTILINE)

                            if id_match:
                                template_id = id_match.group(1).strip()
                                category = yaml_file.parent.name

                                self.content["nuclei_templates"][template_id] = {
                                    "id": template_id,
                                    "category": category,
                                    "severity": severity_match.group(1) if severity_match else "unknown",
                                    "tags": tags_match.group(1).strip() if tags_match else "",
                                    "path": str(yaml_file.relative_to(repo_dir)),
                                }

                                categories[category] = categories.get(category, 0) + 1
                                count += 1

                                # Extract MITRE references if present
                                mitre_match = re.search(r'attack\.mitre\.org/techniques/(T\d+(?:\.\d+)?)', content)
                                if mitre_match:
                                    tech_id = mitre_match.group(1)
                                    if tech_id not in self.crosswalk["technique_to_nuclei"]:
                                        self.crosswalk["technique_to_nuclei"][tech_id] = []
                                    self.crosswalk["technique_to_nuclei"][tech_id].append(template_id)

                except Exception:
                    pass

            print(f"  Loaded {count} templates across {len(categories)} categories")
            # Show top categories
            top_cats = sorted(categories.items(), key=lambda x: x[1], reverse=True)[:5]
            for cat, cnt in top_cats:
                print(f"    {cat}: {cnt}")

            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_sigma_rules(self, clone_depth: int = 1) -> int:
        """Fetch Sigma detection rules."""
        print("\n[4/6] Fetching Sigma rules...")
        repo_dir = self.cache_dir / "sigma"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["sigma_rules"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse rules
            rules_dir = repo_dir / "rules"
            count = 0

            for yaml_file in rules_dir.rglob("*.yml"):
                try:
                    with open(yaml_file, 'r') as f:
                        data = yaml.safe_load(f)

                    if data and isinstance(data, dict):
                        rule_id = data.get("id", yaml_file.stem)
                        self.content["sigma_rules"][rule_id] = {
                            "id": rule_id,
                            "title": data.get("title", ""),
                            "status": data.get("status", ""),
                            "level": data.get("level", ""),
                            "logsource": data.get("logsource", {}),
                            "tags": data.get("tags", []),
                        }

                        # Extract MITRE tags
                        tags = data.get("tags", [])
                        for tag in tags:
                            if tag.startswith("attack.t"):
                                tech_id = tag.replace("attack.", "").upper()
                                if tech_id not in self.crosswalk["technique_to_sigma"]:
                                    self.crosswalk["technique_to_sigma"][tech_id] = []
                                self.crosswalk["technique_to_sigma"][tech_id].append(rule_id)

                        count += 1

                except Exception:
                    pass

            print(f"  Loaded {count} detection rules")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_caldera_abilities(self, clone_depth: int = 1) -> int:
        """Fetch Caldera adversary abilities."""
        print("\n[5/6] Fetching Caldera abilities...")
        repo_dir = self.cache_dir / "caldera"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["caldera"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse abilities
            count = 0

            for yaml_file in repo_dir.rglob("*.yml"):
                if "abilities" in str(yaml_file) or "adversaries" in str(yaml_file):
                    try:
                        with open(yaml_file, 'r') as f:
                            data = yaml.safe_load(f)

                        if isinstance(data, list):
                            for item in data:
                                if "ability_id" in item:
                                    ability_id = item.get("ability_id")
                                    self.content["caldera_abilities"][ability_id] = {
                                        "id": ability_id,
                                        "name": item.get("name", ""),
                                        "technique_id": item.get("technique_id", ""),
                                        "tactic": item.get("tactic", ""),
                                        "platforms": list(item.get("executors", {}).keys()) if isinstance(item.get("executors"), dict) else [],
                                    }
                                    count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} abilities")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_nmap_scripts(self, clone_depth: int = 1) -> int:
        """Fetch Nmap NSE script metadata."""
        print("\n[6/12] Fetching Nmap scripts...")
        repo_dir = self.cache_dir / "nmap"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth), "--filter=blob:none",
                    SOURCES["nmap_scripts"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse scripts
            scripts_dir = repo_dir / "scripts"
            count = 0

            if scripts_dir.exists():
                for nse_file in scripts_dir.glob("*.nse"):
                    try:
                        with open(nse_file, 'r', errors='ignore') as f:
                            content = f.read(2000)  # Just read beginning for metadata

                        # Extract description
                        desc_match = re.search(r'description\s*=\s*\[\[([^\]]+)\]\]', content, re.DOTALL)
                        categories_match = re.search(r'categories\s*=\s*\{([^}]+)\}', content)

                        self.content["nmap_scripts"][nse_file.stem] = {
                            "name": nse_file.stem,
                            "description": desc_match.group(1)[:200].strip() if desc_match else "",
                            "categories": [c.strip().strip('"\'') for c in categories_match.group(1).split(",")] if categories_match else [],
                        }
                        count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} NSE scripts")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_mitre_d3fend(self) -> int:
        """Fetch MITRE D3FEND countermeasures ontology."""
        print("\n[7/12] Fetching MITRE D3FEND...")
        cache_file = self.cache_dir / "d3fend.json"

        try:
            response = requests.get(SOURCES["mitre_d3fend"]["url"], timeout=60)
            response.raise_for_status()
            data = response.json()

            with open(cache_file, 'w') as f:
                json.dump(data, f)

            # Parse D3FEND techniques/countermeasures
            count = 0
            if "@graph" in data:
                for item in data["@graph"]:
                    item_type = item.get("@type", [])
                    if isinstance(item_type, str):
                        item_type = [item_type]

                    # Look for defensive techniques
                    if any("DefensiveTechnique" in t or "d3f:" in str(t) for t in item_type):
                        d3f_id = item.get("@id", "").replace("d3f:", "")
                        if d3f_id:
                            self.content["d3fend_techniques"][d3f_id] = {
                                "id": d3f_id,
                                "label": item.get("rdfs:label", ""),
                                "definition": item.get("d3f:definition", "")[:300] if item.get("d3f:definition") else "",
                                "type": item_type,
                            }
                            count += 1

            print(f"  Loaded {count} D3FEND countermeasures")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_mitre_car(self, clone_depth: int = 1) -> int:
        """Fetch MITRE CAR analytics."""
        print("\n[8/12] Fetching MITRE CAR...")
        repo_dir = self.cache_dir / "car"

        try:
            if repo_dir.exists() and (repo_dir / ".git").exists():
                result = subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, text=True, timeout=120)
                if result.returncode != 0:
                    print(f"  ⚠️  Git pull failed: {result.stderr[:200]}")
            else:
                result = subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["mitre_car"]["repo"], str(repo_dir)
                ], capture_output=True, text=True, timeout=300)
                if result.returncode != 0:
                    print(f"  ❌ Git clone failed: {result.stderr[:200]}")
                    return 0

            # Parse CAR analytics
            analytics_dir = repo_dir / "analytics"
            count = 0

            if analytics_dir.exists():
                for yaml_file in analytics_dir.rglob("*.yaml"):
                    try:
                        with open(yaml_file, 'r') as f:
                            data = yaml.safe_load(f)

                        if data and isinstance(data, dict):
                            car_id = data.get("id", yaml_file.stem)
                            self.content["car_analytics"][car_id] = {
                                "id": car_id,
                                "title": data.get("title", ""),
                                "submission_date": data.get("submission_date", ""),
                                "platforms": data.get("platforms", []),
                                "coverage": data.get("coverage", []),
                                "implementations": len(data.get("implementations", [])),
                            }

                            # Map to ATT&CK techniques
                            for cov in data.get("coverage", []):
                                tech_id = cov.get("technique")
                                if tech_id:
                                    if tech_id not in self.crosswalk.get("technique_to_car", {}):
                                        self.crosswalk.setdefault("technique_to_car", {})[tech_id] = []
                                    self.crosswalk["technique_to_car"][tech_id].append(car_id)

                            count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} CAR analytics")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_mitre_atlas(self, clone_depth: int = 1) -> int:
        """Fetch MITRE ATLAS AI/ML adversarial attacks."""
        print("\n[9/12] Fetching MITRE ATLAS...")
        repo_dir = self.cache_dir / "atlas"

        try:
            if repo_dir.exists() and (repo_dir / ".git").exists():
                result = subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, text=True, timeout=120)
                if result.returncode != 0:
                    print(f"  ⚠️  Git pull failed: {result.stderr[:200]}")
            else:
                result = subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["mitre_atlas"]["repo"], str(repo_dir)
                ], capture_output=True, text=True, timeout=300)
                if result.returncode != 0:
                    print(f"  ❌ Git clone failed: {result.stderr[:200]}")
                    return 0

            # Parse ATLAS data
            count = 0
            data_dir = repo_dir / "data"

            # Look for techniques, case studies, etc.
            for json_file in data_dir.rglob("*.json") if data_dir.exists() else []:
                try:
                    with open(json_file, 'r') as f:
                        data = json.load(f)

                    if isinstance(data, dict):
                        # Parse techniques
                        for tech in data.get("techniques", []):
                            atlas_id = tech.get("id", "")
                            if atlas_id:
                                self.content["atlas_techniques"][atlas_id] = {
                                    "id": atlas_id,
                                    "name": tech.get("name", ""),
                                    "tactics": tech.get("tactics", []),
                                    "description": tech.get("description", "")[:300],
                                }
                                count += 1

                        # Parse case studies
                        for case in data.get("case-studies", []):
                            case_id = case.get("id", "")
                            if case_id:
                                self.content["atlas_case_studies"][case_id] = {
                                    "id": case_id,
                                    "name": case.get("name", ""),
                                    "summary": case.get("summary", "")[:300],
                                }

                except Exception:
                    pass

            # Also check YAML files
            for yaml_file in data_dir.rglob("*.yaml") if data_dir.exists() else []:
                try:
                    with open(yaml_file, 'r') as f:
                        data = yaml.safe_load(f)
                    if data and "id" in data:
                        self.content["atlas_techniques"][data["id"]] = {
                            "id": data["id"],
                            "name": data.get("name", ""),
                            "description": data.get("description", "")[:300],
                        }
                        count += 1
                except Exception:
                    pass

            print(f"  Loaded {count} ATLAS AI/ML techniques")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_mitre_engage(self, clone_depth: int = 1) -> int:
        """Fetch MITRE ENGAGE adversary engagement framework."""
        print("\n[9.5/12] Fetching MITRE ENGAGE...")
        repo_dir = self.cache_dir / "engage"

        try:
            if repo_dir.exists() and (repo_dir / ".git").exists():
                result = subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, text=True, timeout=120)
                if result.returncode != 0:
                    print(f"  ⚠️  Git pull failed: {result.stderr[:200]}")
            else:
                result = subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["mitre_engage"]["repo"], str(repo_dir)
                ], capture_output=True, text=True, timeout=300)
                if result.returncode != 0:
                    print(f"  ❌ Git clone failed: {result.stderr[:200]}")
                    return 0

            # Parse ENGAGE data
            count = 0
            data_dir = repo_dir / "data" if (repo_dir / "data").exists() else repo_dir

            # Look for YAML files with activities, tactics, etc.
            for yaml_file in data_dir.rglob("*.yaml") if data_dir.exists() else []:
                try:
                    with open(yaml_file, 'r') as f:
                        data = yaml.safe_load(f)

                    if data and isinstance(data, dict):
                        # Parse activities
                        if "id" in data and "name" in data:
                            engage_id = data.get("id", yaml_file.stem)
                            self.content["engage_activities"][engage_id] = {
                                "id": engage_id,
                                "name": data.get("name", ""),
                                "description": data.get("description", "")[:300],
                                "tactics": data.get("tactics", []),
                            }
                            count += 1

                except Exception:
                    pass

            # Also check for JSON files
            for json_file in data_dir.rglob("*.json") if data_dir.exists() else []:
                try:
                    with open(json_file, 'r') as f:
                        data = json.load(f)

                    if isinstance(data, dict) and "id" in data:
                        engage_id = data.get("id", json_file.stem)
                        self.content["engage_activities"][engage_id] = {
                            "id": engage_id,
                            "name": data.get("name", ""),
                            "description": data.get("description", "")[:300],
                        }
                        count += 1
                except Exception:
                    pass

            print(f"  Loaded {count} ENGAGE activities")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_yara_rules(self, clone_depth: int = 1) -> int:
        """Fetch YARA malware detection rules."""
        print("\n[10/12] Fetching YARA rules...")
        repo_dir = self.cache_dir / "yara-rules"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    SOURCES["yara_rules"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse YARA rules
            count = 0
            categories = {}

            for yar_file in repo_dir.rglob("*.yar"):
                try:
                    with open(yar_file, 'r', errors='ignore') as f:
                        content = f.read()

                    # Extract rule names
                    rule_matches = re.findall(r'rule\s+(\w+)', content)
                    category = yar_file.parent.name

                    for rule_name in rule_matches:
                        self.content["yara_rules"][rule_name] = {
                            "name": rule_name,
                            "category": category,
                            "file": str(yar_file.relative_to(repo_dir)),
                        }
                        categories[category] = categories.get(category, 0) + 1
                        count += 1

                except Exception:
                    pass

            print(f"  Loaded {count} YARA rules across {len(categories)} categories")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_wazuh_rules(self, clone_depth: int = 1) -> int:
        """Fetch Wazuh SIEM detection rules."""
        print("\n[11/12] Fetching Wazuh rules...")
        repo_dir = self.cache_dir / "wazuh"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                # Sparse clone just the rules directory
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth), "--filter=blob:none", "--sparse",
                    SOURCES["wazuh_rules"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)
                subprocess.run(
                    ["git", "-C", str(repo_dir), "sparse-checkout", "set", "ruleset/rules"],
                    capture_output=True, timeout=60
                )

            # Parse Wazuh XML rules
            rules_dir = repo_dir / "ruleset" / "rules"
            count = 0

            if rules_dir.exists():
                for xml_file in rules_dir.glob("*.xml"):
                    try:
                        with open(xml_file, 'r', errors='ignore') as f:
                            content = f.read()

                        # Extract rule IDs and descriptions
                        rule_matches = re.findall(
                            r'<rule\s+id="(\d+)"[^>]*>.*?<description>([^<]+)</description>',
                            content, re.DOTALL
                        )

                        for rule_id, description in rule_matches:
                            self.content["wazuh_rules"][rule_id] = {
                                "id": rule_id,
                                "description": description.strip()[:200],
                                "file": xml_file.name,
                            }
                            count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} Wazuh detection rules")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_kali_tools(self) -> int:
        """Index Kali tools from embedded inventory."""
        print("\n[12/14] Indexing Kali tools...")

        count = 0
        for category, tools in KALI_TOOL_CATEGORIES.items():
            for tool in tools:
                self.content["kali_tools"][tool] = {
                    "name": tool,
                    "category": category,
                    "deployment": "bare_metal" if category in ["NetworkRecon", "ExploitationFrameworks", "WebApplicationTesting"] else "iso",
                }
                count += 1

        print(f"  Indexed {count} Kali tools across {len(KALI_TOOL_CATEGORIES)} categories")
        return count

    def fetch_lolbas(self, clone_depth: int = 1) -> int:
        """Fetch LOLBAS - Living Off The Land Binaries and Scripts."""
        print("\n[13/14] Fetching LOLBAS/GTFOBins...")
        repo_dir = self.cache_dir / "lolbas"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    LOLTL_SOURCES["lolbas"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse LOLBAS YAML files
            count = 0
            binaries_dir = repo_dir / "yml" / "OSBinaries"
            libraries_dir = repo_dir / "yml" / "OSLibraries"
            scripts_dir = repo_dir / "yml" / "OSScripts"

            for yml_dir in [binaries_dir, libraries_dir, scripts_dir]:
                if yml_dir.exists():
                    for yaml_file in yml_dir.glob("*.yml"):
                        try:
                            with open(yaml_file, 'r') as f:
                                data = yaml.safe_load(f)

                            if data and isinstance(data, dict):
                                name = data.get("Name", yaml_file.stem)
                                self.content["lolbas_binaries"][name] = {
                                    "name": name,
                                    "description": data.get("Description", "")[:200],
                                    "author": data.get("Author", ""),
                                    "commands": [
                                        {
                                            "command": cmd.get("Command", "")[:200],
                                            "description": cmd.get("Description", ""),
                                            "usecase": cmd.get("Usecase", ""),
                                            "category": cmd.get("Category", ""),
                                            "mitre_id": cmd.get("MitreID", ""),
                                        }
                                        for cmd in data.get("Commands", [])[:5]
                                    ],
                                    "paths": data.get("Full_Path", [])[:3],
                                    "type": yml_dir.name,
                                }

                                # Map to MITRE techniques
                                for cmd in data.get("Commands", []):
                                    mitre_id = cmd.get("MitreID", "")
                                    if mitre_id:
                                        if mitre_id not in self.crosswalk["technique_to_lolbas"]:
                                            self.crosswalk["technique_to_lolbas"][mitre_id] = []
                                        self.crosswalk["technique_to_lolbas"][mitre_id].append(name)

                                count += 1

                        except Exception:
                            pass

            print(f"  Loaded {count} LOLBAS entries")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_gtfobins(self, clone_depth: int = 1) -> int:
        """Fetch GTFOBins - Unix binaries for privilege escalation."""
        print("\n[14/14] Fetching GTFOBins...")
        repo_dir = self.cache_dir / "gtfobins"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    LOLTL_SOURCES["gtfobins"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse GTFOBins markdown files
            count = 0
            gtfobins_dir = repo_dir / "_gtfobins"

            if gtfobins_dir.exists():
                for md_file in gtfobins_dir.glob("*.md"):
                    try:
                        with open(md_file, 'r', errors='ignore') as f:
                            content = f.read()

                        name = md_file.stem
                        # Extract functions from YAML front matter
                        functions = []
                        func_match = re.search(r'functions:\s*\n((?:\s+-\s+\w+\n)+)', content)
                        if func_match:
                            functions = [f.strip().lstrip('- ') for f in func_match.group(1).split('\n') if f.strip()]

                        self.content["gtfobins"][name] = {
                            "name": name,
                            "functions": functions[:10],
                            "file": md_file.name,
                        }
                        count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} GTFOBins entries")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_awesome_osint(self, clone_depth: int = 1) -> int:
        """Fetch awesome-osint - comprehensive OSINT tools list."""
        print("\n[OSINT] Fetching Awesome OSINT...")
        repo_dir = self.cache_dir / "awesome-osint"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    OSINT_SOURCES["awesome_osint"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse the README.md which contains the tool list
            readme_file = repo_dir / "README.md"
            count = 0

            if readme_file.exists():
                with open(readme_file, 'r', errors='ignore') as f:
                    content = f.read()

                # Extract tools from markdown links: [Tool Name](url)
                # Also extract categories from ## headers
                current_category = "General"
                categories = {}

                for line in content.split('\n'):
                    # Check for category headers
                    if line.startswith('## '):
                        current_category = line[3:].strip()
                        if current_category not in categories:
                            categories[current_category] = []

                    # Extract tool links
                    link_matches = re.findall(r'\[([^\]]+)\]\(([^)]+)\)', line)
                    for name, url in link_matches:
                        if url.startswith('http') and not url.startswith('https://github.com/jivoi'):
                            tool_id = re.sub(r'[^a-z0-9]', '_', name.lower())
                            self.content["osint_tools"][tool_id] = {
                                "name": name,
                                "url": url,
                                "category": current_category,
                            }
                            if current_category not in categories:
                                categories[current_category] = []
                            categories[current_category].append(name)
                            count += 1

                print(f"  Loaded {count} OSINT tools across {len(categories)} categories")
                # Show top categories
                top_cats = sorted(categories.items(), key=lambda x: len(x[1]), reverse=True)[:5]
                for cat, tools in top_cats:
                    print(f"    {cat}: {len(tools)}")

            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_osint_framework(self, clone_depth: int = 1) -> int:
        """Fetch OSINT Framework - structured resource tree."""
        print("\n[OSINT] Fetching OSINT Framework...")
        repo_dir = self.cache_dir / "osint-framework"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    OSINT_SOURCES["osint_framework"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse the arf.json file which contains structured data
            arf_file = repo_dir / "arf.json"
            count = 0

            if arf_file.exists():
                with open(arf_file, 'r') as f:
                    data = json.load(f)

                # Recursively parse the tree structure
                def parse_node(node, category=""):
                    nonlocal count
                    if isinstance(node, dict):
                        name = node.get("name", "")
                        url = node.get("url", "")
                        if url and url.startswith("http"):
                            site_id = re.sub(r'[^a-z0-9]', '_', name.lower())
                            self.content["osint_sites"][site_id] = {
                                "name": name,
                                "url": url,
                                "category": category,
                            }
                            count += 1

                        # Process children
                        for child in node.get("children", []):
                            parse_node(child, name or category)

                    elif isinstance(node, list):
                        for item in node:
                            parse_node(item, category)

                parse_node(data)

            print(f"  Loaded {count} OSINT sites/resources")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_sherlock_sites(self, clone_depth: int = 1) -> int:
        """Fetch Sherlock sites - username hunting database."""
        print("\n[OSINT] Fetching Sherlock sites...")
        repo_dir = self.cache_dir / "sherlock"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    OSINT_SOURCES["sherlock"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            # Parse data.json
            data_file = repo_dir / "sherlock" / "resources" / "data.json"
            count = 0

            if data_file.exists():
                with open(data_file, 'r') as f:
                    data = json.load(f)

                for site_name, site_data in data.items():
                    self.content["osint_sites"][f"sherlock_{site_name.lower()}"] = {
                        "name": site_name,
                        "url": site_data.get("url", ""),
                        "category": "username_search",
                        "source": "sherlock",
                    }
                    count += 1

            print(f"  Loaded {count} Sherlock target sites")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_mitre_attack_ics(self) -> int:
        """Fetch MITRE ATT&CK for ICS."""
        print("\n[ICS] Fetching MITRE ATT&CK for ICS...")
        cache_file = self.cache_dir / "mitre_attack_ics.json"

        try:
            response = requests.get(SOURCES["mitre_attack_ics"]["url"], timeout=60)
            response.raise_for_status()
            data = response.json()

            with open(cache_file, 'w') as f:
                json.dump(data, f)

            count = 0
            for obj in data.get("objects", []):
                if obj.get("type") == "attack-pattern":
                    refs = obj.get("external_references", [])
                    for ref in refs:
                        if ref.get("source_name") == "mitre-attack":
                            tech_id = ref.get("external_id")
                            if tech_id:
                                self.content["mitre_ics_techniques"][tech_id] = {
                                    "id": tech_id,
                                    "name": obj.get("name"),
                                    "tactics": [p.get("phase_name") for p in obj.get("kill_chain_phases", [])],
                                    "description": obj.get("description", "")[:500],
                                }
                                count += 1

            print(f"  Loaded {count} ICS techniques")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_mitre_attack_mobile(self) -> int:
        """Fetch MITRE ATT&CK Mobile."""
        print("\n[Mobile] Fetching MITRE ATT&CK Mobile...")
        cache_file = self.cache_dir / "mitre_attack_mobile.json"

        try:
            response = requests.get(SOURCES["mitre_attack_mobile"]["url"], timeout=60)
            response.raise_for_status()
            data = response.json()

            with open(cache_file, 'w') as f:
                json.dump(data, f)

            count = 0
            for obj in data.get("objects", []):
                if obj.get("type") == "attack-pattern":
                    refs = obj.get("external_references", [])
                    for ref in refs:
                        if ref.get("source_name") == "mitre-attack":
                            tech_id = ref.get("external_id")
                            if tech_id:
                                self.content["mitre_mobile_techniques"][tech_id] = {
                                    "id": tech_id,
                                    "name": obj.get("name"),
                                    "tactics": [p.get("phase_name") for p in obj.get("kill_chain_phases", [])],
                                    "platforms": obj.get("x_mitre_platforms", []),
                                    "description": obj.get("description", "")[:500],
                                }
                                count += 1

            print(f"  Loaded {count} Mobile techniques")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_loldrivers(self, clone_depth: int = 1) -> int:
        """Fetch LOLDrivers - Vulnerable Windows drivers."""
        print("\n[LOLTL] Fetching LOLDrivers...")
        repo_dir = self.cache_dir / "loldrivers"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    LOLTL_SOURCES["loldrivers"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            count = 0
            drivers_dir = repo_dir / "yaml" / "drivers"

            if drivers_dir.exists():
                for yaml_file in drivers_dir.glob("*.yaml"):
                    try:
                        with open(yaml_file, 'r') as f:
                            data = yaml.safe_load(f)

                        if data and isinstance(data, dict):
                            driver_id = data.get("Id", yaml_file.stem)
                            self.content["loldrivers"][driver_id] = {
                                "id": driver_id,
                                "name": data.get("Name", ""),
                                "category": data.get("Category", ""),
                                "commands": data.get("Commands", [])[:5],
                                "verified": data.get("Verified", False),
                            }
                            count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} LOLDrivers entries")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_hijacklibs(self, clone_depth: int = 1) -> int:
        """Fetch HijackLibs - DLL Hijacking database."""
        print("\n[LOLTL] Fetching HijackLibs...")
        repo_dir = self.cache_dir / "hijacklibs"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    LOLTL_SOURCES["hijacklibs"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            count = 0
            yml_dir = repo_dir / "yml"

            if yml_dir.exists():
                for yaml_file in yml_dir.rglob("*.yml"):
                    try:
                        with open(yaml_file, 'r') as f:
                            data = yaml.safe_load(f)

                        if data and isinstance(data, dict):
                            name = data.get("Name", yaml_file.stem)
                            self.content["hijacklibs"][name] = {
                                "name": name,
                                "author": data.get("Author", ""),
                                "vendor": data.get("Vendor", ""),
                                "expected_locations": data.get("ExpectedLocations", [])[:5],
                                "vulnerable_executables": data.get("VulnerableExecutables", [])[:5],
                            }
                            count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} HijackLibs entries")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_exploitdb(self, clone_depth: int = 1) -> int:
        """Fetch ExploitDB from GitLab."""
        print("\n[Exploits] Fetching ExploitDB...")
        repo_dir = self.cache_dir / "exploitdb"
        
        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=300)
            else:
                print("  Cloning ExploitDB (this may take a while)...")
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth), "--filter=blob:none", "--sparse",
                    SOURCES["exploitdb"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=600)
                subprocess.run(
                    ["git", "-C", str(repo_dir), "sparse-checkout", "set", "exploits"],
                    capture_output=True, timeout=60
                )
            
            # Count and index exploits
            exploits_dir = repo_dir / "exploits"
            count = 0
            
            if exploits_dir.exists():
                # Create index of exploits
                for exploit_file in exploits_dir.rglob("*.txt"):
                    try:
                        # Extract exploit ID from path (e.g., exploits/windows/remote/12345.txt)
                        exploit_id = exploit_file.stem
                        platform = exploit_file.parent.name if exploit_file.parent.name != "exploits" else "unknown"
                        
                        # Read first few lines for metadata
                        with open(exploit_file, 'r', errors='ignore') as f:
                            lines = [f.readline().strip() for _ in range(5)]
                            description = lines[0] if lines else ""
                        
                        self.content.setdefault("exploitdb", {})[exploit_id] = {
                            "id": exploit_id,
                            "file": str(exploit_file.relative_to(exploits_dir)),
                            "platform": platform,
                            "description": description[:200],
                            "type": "exploit",
                        }
                        count += 1
                    except Exception:
                        pass
                
                # Also count Python exploits
                py_count = sum(1 for _ in exploits_dir.rglob("*.py"))
                count += py_count
                
            print(f"  Loaded {count} ExploitDB exploits")
            return count
            
        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_wadcoms(self, clone_depth: int = 1) -> int:
        """Fetch WADComs - Windows/AD offensive cheatsheets."""
        print("\n[LOLTL] Fetching WADComs...")
        repo_dir = self.cache_dir / "wadcoms"

        try:
            if repo_dir.exists():
                subprocess.run(["git", "-C", str(repo_dir), "pull", "--depth", "1"],
                             capture_output=True, timeout=120)
            else:
                subprocess.run([
                    "git", "clone", "--depth", str(clone_depth),
                    LOLTL_SOURCES["wadcoms"]["repo"], str(repo_dir)
                ], capture_output=True, timeout=300)

            count = 0
            wadcoms_dir = repo_dir / "_wadcoms"

            if wadcoms_dir.exists():
                for md_file in wadcoms_dir.glob("*.md"):
                    try:
                        with open(md_file, 'r', errors='ignore') as f:
                            content = f.read()

                        name = md_file.stem
                        # Extract info from YAML front matter
                        self.content["wadcoms"][name] = {
                            "name": name,
                            "file": md_file.name,
                        }
                        count += 1

                    except Exception:
                        pass

            print(f"  Loaded {count} WADComs entries")
            return count

        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_ptcc_configurations(self) -> int:
        """Fetch PTCC (Primitive Tactical Cognitive Classification) configurations from ctas7-ptcc-teth-database."""
        print("\n[PTCC] Fetching PTCC configurations...")
        
        # Path to PTCC database directory
        ptcc_db_dir = Path(__file__).parent.parent.parent.parent / "ctas7-ptcc-teth-database"
        abe_results_dir = ptcc_db_dir / "abe_results"
        
        count = 0
        
        try:
            # Check for ABE results (recovered/generated PTCC configs)
            ptcc_files = [
                abe_results_dir / "abe_ptcc_results.json",
                abe_results_dir / "abe_recovered_ptcc.json",
                abe_results_dir / "abe_generated_ptcc.json",
            ]
            
            for ptcc_file in ptcc_files:
                if ptcc_file.exists():
                    try:
                        with open(ptcc_file, 'r') as f:
                            data = json.load(f)
                        
                        # Handle different JSON structures
                        configs = []
                        if isinstance(data, dict):
                            configs = data.get("recovered_configurations", []) + \
                                     data.get("generated_configurations", []) + \
                                     data.get("configurations", [])
                        elif isinstance(data, list):
                            configs = data
                        
                        for config in configs:
                            if isinstance(config, dict):
                                ptcc_id = config.get("ptcc_id") or config.get("operator", "unknown")
                                self.content["ptcc_configurations"][str(ptcc_id)] = {
                                    "ptcc_id": ptcc_id,
                                    "operator": config.get("operator", "unknown"),
                                    "skill_level": config.get("skill_level", 0.0),
                                    "region": config.get("region", "unknown"),
                                    "tool": config.get("tool", "unknown"),
                                    "rig": config.get("rig", "unknown"),
                                    "ai_assist": config.get("ai_assist", "None"),
                                    "viable": config.get("viable", False),
                                    "ai_force_multiplier": config.get("ai_force_multiplier") or config.get("ai_force_mult", 1.0),
                                    "region_shielding": config.get("region_shielding", 0.0),
                                    "entropy_h": config.get("entropy_h") or config.get("entropy_H", 0.0),
                                    "recommended_hd4_phase": config.get("recommended_hd4_phase", "Detect"),
                                    "source_file": ptcc_file.name,
                                }
                                count += 1
                        
                        print(f"  ✅ Loaded {len(configs)} PTCC configs from {ptcc_file.name}")
                    except Exception as e:
                        print(f"  ⚠️  Error reading {ptcc_file.name}: {e}")
            
            if count == 0:
                print("  ⚠️  No PTCC configurations found (ctas7-ptcc-teth-database may need extraction)")
            else:
                print(f"  ✅ Total PTCC configurations: {count}")
                
                # Derive tool chains from PTCC configurations (iTunes tools/albums concept)
                print("\n  🔗 Deriving tool chains from PTCC configurations...")
                tool_chain_count = self.derive_ptcc_tool_chains()
                print(f"  ✅ Derived {tool_chain_count} tool chains from PTCC configs")
            
            return count
            
        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_teth_algorithms(self) -> int:
        """Fetch TETH (Topological Entropy Threat Heuristic) algorithms from ctas7-ptcc-teth-database."""
        print("\n[TETH] Fetching TETH algorithms...")
        
        # Path to TETH database directory
        teth_db_dir = Path(__file__).parent.parent.parent.parent / "ctas7-ptcc-teth-database"
        teth_output_dir = teth_db_dir / "teth_format_output"
        extracted_data_dir = teth_db_dir / "extracted_data"
        
        count = 0
        
        try:
            # Check for TETH algorithm files
            teth_files = [
                teth_output_dir / "teth_format_summary.json",
                teth_output_dir / "ptcc_teth_format.json",
                extracted_data_dir / "teth_algorithms.json",
            ]
            
            for teth_file in teth_files:
                if teth_file.exists():
                    try:
                        with open(teth_file, 'r') as f:
                            data = json.load(f)
                        
                        # Handle different JSON structures
                        algorithms = []
                        if isinstance(data, dict):
                            algorithms = data.get("teth_algorithms", []) + \
                                        data.get("algorithms", [])
                            # Also check if data itself is algorithm structure
                            if "algorithm_type" in data or "algorithm_name" in data:
                                algorithms = [data]
                        elif isinstance(data, list):
                            algorithms = data
                        
                        for algo in algorithms:
                            if isinstance(algo, dict):
                                algo_id = algo.get("teth_id") or algo.get("algorithm_name", f"teth_{count}")
                                self.content["teth_algorithms"][str(algo_id)] = {
                                    "teth_id": algo_id,
                                    "algorithm_name": algo.get("algorithm_name", "unknown"),
                                    "algorithm_type": algo.get("algorithm_type", "unknown"),
                                    "description": algo.get("description", ""),
                                    "entropy_calculation_method": algo.get("entropy_calculation_method", "shannon_entropy"),
                                    "complexity_threshold": algo.get("complexity_threshold", 0.75),
                                    "iterations": algo.get("iterations", 1000000),
                                    "baseline_percentage": algo.get("baseline_percentage", 83.56),
                                    "source_file": teth_file.name,
                                }
                                count += 1
                        
                        print(f"  ✅ Loaded {len(algorithms)} TETH algorithms from {teth_file.name}")
                    except Exception as e:
                        print(f"  ⚠️  Error reading {teth_file.name}: {e}")
            
            # If no files found, generate default TETH algorithms
            if count == 0:
                print("  ⚠️  No TETH algorithm files found, generating defaults...")
                default_teth = [
                    {
                        "teth_id": 1,
                        "algorithm_name": "TETH-Topological",
                        "algorithm_type": "topological_entropy_analysis",
                        "entropy_calculation_method": "shannon_entropy",
                        "complexity_threshold": 0.75,
                        "iterations": 1000000,
                        "baseline_percentage": 83.56,
                    },
                    {
                        "teth_id": 2,
                        "algorithm_name": "TETH-Heuristic",
                        "algorithm_type": "threat_heuristic_scoring",
                    },
                    {
                        "teth_id": 3,
                        "algorithm_name": "TETH-Behavioral",
                        "algorithm_type": "behavioral_entropy_modeling",
                    },
                    {
                        "teth_id": 4,
                        "algorithm_name": "TETH-Predictive",
                        "algorithm_type": "predictive_threat_modeling",
                    },
                ]
                
                for algo in default_teth:
                    self.content["teth_algorithms"][str(algo["teth_id"])] = algo
                    count += 1
                
                print(f"  ✅ Generated {count} default TETH algorithms")
            else:
                print(f"  ✅ Total TETH algorithms: {count}")
            
            return count
            
        except Exception as e:
            print(f"  ERROR: {e}")
            return 0

    def fetch_all(self) -> dict:
        """Fetch ALL threat content sources per RFC-9011 and RFC-9023."""
        print("=" * 70)
        print("RFC-9011: SX9 Threat Content Ingestion - COMPLETE SUITE")
        print("RFC-9023: Security Framework Integration Map")
        print("=" * 70)

        results = {}

        # ========== MITRE ATT&CK SUITE (ALL) ==========
        print("\n" + "-" * 50)
        print("MITRE ATT&CK Suite")
        print("-" * 50)
        results["mitre_attack_enterprise"] = self.fetch_mitre_attack()
        results["mitre_attack_ics"] = self.fetch_mitre_attack_ics()
        results["mitre_attack_mobile"] = self.fetch_mitre_attack_mobile()

        # ========== MITRE DEFENSE SUITE ==========
        print("\n" + "-" * 50)
        print("MITRE Defense Suite")
        print("-" * 50)
        results["mitre_d3fend"] = self.fetch_mitre_d3fend()
        results["mitre_car"] = self.fetch_mitre_car()
        results["mitre_atlas"] = self.fetch_mitre_atlas()
        results["mitre_engage"] = self.fetch_mitre_engage()

        # ========== ADVERSARY EMULATION ==========
        print("\n" + "-" * 50)
        print("Adversary Emulation Frameworks")
        print("-" * 50)
        results["atomic_red_team"] = self.fetch_atomic_red_team()
        results["caldera_abilities"] = self.fetch_caldera_abilities()

        # ========== DETECTION RULES ==========
        print("\n" + "-" * 50)
        print("Detection Rules & Signatures")
        print("-" * 50)
        results["nuclei_templates"] = self.fetch_nuclei_templates()
        results["sigma_rules"] = self.fetch_sigma_rules()
        results["yara_rules"] = self.fetch_yara_rules()
        results["wazuh_rules"] = self.fetch_wazuh_rules()

        # ========== RECONNAISSANCE ==========
        print("\n" + "-" * 50)
        print("Reconnaissance Tools")
        print("-" * 50)
        results["nmap_scripts"] = self.fetch_nmap_scripts()

        # ========== LOLTL (Living Off The Land) ==========
        print("\n" + "-" * 50)
        print("Living Off The Land (LOLTL)")
        print("-" * 50)
        results["lolbas"] = self.fetch_lolbas()
        results["gtfobins"] = self.fetch_gtfobins()
        results["loldrivers"] = self.fetch_loldrivers()
        results["hijacklibs"] = self.fetch_hijacklibs()
        results["wadcoms"] = self.fetch_wadcoms()

        # ========== KALI TOOLS ==========
        print("\n" + "-" * 50)
        print("Kali Linux Tools")
        print("-" * 50)
        results["kali_tools"] = self.fetch_kali_tools()

        # ========== OSINT RESOURCES ==========
        print("\n" + "-" * 50)
        print("OSINT Resources")
        print("-" * 50)
        results["awesome_osint"] = self.fetch_awesome_osint()
        results["osint_framework"] = self.fetch_osint_framework()
        results["sherlock_sites"] = self.fetch_sherlock_sites()

        # ========== EXPLOITS ==========
        print("\n" + "-" * 50)
        print("Exploit Databases")
        print("-" * 50)
        results["exploitdb"] = self.fetch_exploitdb()

        # ========== CTAS INTERNAL SYSTEMS ==========
        print("\n" + "-" * 50)
        print("CTAS Internal Systems (PTCC & TETH)")
        print("-" * 50)
        results["ptcc_configurations"] = self.fetch_ptcc_configurations()
        results["teth_algorithms"] = self.fetch_teth_algorithms()
        # Tool chains are derived during PTCC fetch, count them here
        results["ptcc_tool_chains"] = len(self.content.get("ptcc_tool_chains", {}))

        # ========== SUMMARY ==========
        print("\n" + "=" * 70)
        print("COMPLETE SUMMARY")
        print("=" * 70)

        total = 0
        for source, count in results.items():
            print(f"  {source}: {count}")
            total += count

        print(f"\n  TOTAL ITEMS: {total}")

        # Build crosswalk summary
        print(f"\nCrosswalk Mappings:")
        print(f"  Technique → Atomic:  {len(self.crosswalk['technique_to_atomic'])} techniques")
        print(f"  Technique → Nuclei:  {len(self.crosswalk['technique_to_nuclei'])} techniques")
        print(f"  Technique → Sigma:   {len(self.crosswalk['technique_to_sigma'])} techniques")
        print(f"  Technique → CAR:     {len(self.crosswalk.get('technique_to_car', {}))} techniques")
        print(f"  Technique → LOLBAS:  {len(self.crosswalk['technique_to_lolbas'])} techniques")

        return results

    def generate_spires_ontology(self):
        """Generate SPIRES ontology from fetched threat content."""
        if not SPIRES_AVAILABLE:
            print("\n⚠️  SPIRES not available - skipping ontology generation")
            return None

        print("\n" + "=" * 70)
        print("SPIRES ONTOLOGY GENERATION - RFC-9105")
        print("=" * 70)

        try:
            # Initialize SPIRES extractor
            extractor = SPIRESExtractor()
            
            # Load existing ontology to avoid duplicates
            existing_ontology_path = self.cache_dir.parent.parent / "output" / "ontology" / "ontology_raw.json"
            threat_graph = OntologyGraph()
            
            if existing_ontology_path.exists():
                print(f"\n[SPIRES] Loading existing ontology: {existing_ontology_path}")
                existing_count = threat_graph.load_from_json(existing_ontology_path)
                print(f"  ✅ Loaded {existing_count} existing terms, {len(threat_graph.relations)} relations")
            else:
                print("\n[SPIRES] No existing ontology found, starting fresh")
            
            # Temporarily update THREAT_CONTENT_PATH to point to our cache directory
            # The SPIRES extractor uses a hardcoded path, so we need to ensure it points correctly
            import spires_ontology_extractor as spires_module
            original_path = getattr(spires_module, 'THREAT_CONTENT_PATH', None)
            spires_module.THREAT_CONTENT_PATH = self.cache_dir
            
            # Extract NEW ontology from threat content (will merge with existing)
            print("\n[SPIRES] Extracting NEW ontology from threat content...")
            new_threat_graph = extractor.extract_from_threats()
            
            # Merge new extraction into existing graph
            for term in new_threat_graph.terms.values():
                threat_graph.add_term(term)  # add_term handles deduplication
            for rel in new_threat_graph.relations:
                threat_graph.add_relation(
                    rel["source"],
                    rel["target"],
                    rel["relation_type"],
                    rel.get("weight", 1.0)
                )
            
            print(f"  ✅ After merge: {len(threat_graph.terms)} total terms, {len(threat_graph.relations)} total relations")
            print(f"  📊 New terms added: {len(new_threat_graph.terms)}, New relations: {len(new_threat_graph.relations)}")
            
            # Restore original path if it existed
            if original_path:
                spires_module.THREAT_CONTENT_PATH = original_path
            
            print(f"  ✅ Extracted {len(threat_graph.terms)} terms, {len(threat_graph.relations)} relations")

            # Generate outputs
            ontology_output_dir = self.cache_dir.parent / "ontology"
            ontology_output_dir.mkdir(parents=True, exist_ok=True)

            # JSON export
            json_export = generate_json_export(threat_graph)
            json_file = ontology_output_dir / "threat_ontology.json"
            with open(json_file, 'w') as f:
                json.dump(json_export, f, indent=2)
            print(f"  ✅ JSON ontology saved: {json_file}")

            # Cypher export for Neo4j (generated FROM ontology)
            cypher_export = generate_cypher_export(threat_graph)
            cypher_file = ontology_output_dir / "threat_ontology.cypher"
            with open(cypher_file, 'w') as f:
                f.write(cypher_export)
            print(f"  ✅ Cypher queries saved: {cypher_file}")
            
            # SurrealQL export for SurrealDB (generated FROM ontology)
            surreal_export = generate_surreal_export(threat_graph)
            surreal_file = ontology_output_dir / "threat_ontology.surql"
            with open(surreal_file, 'w') as f:
                f.write(surreal_export)
            print(f"  ✅ SurrealQL queries saved: {surreal_file}")

            # LinkML schema
            linkml_schema = generate_linkml_schema(threat_graph)
            linkml_file = ontology_output_dir / "threat_ontology.linkml.yaml"
            with open(linkml_file, 'w') as f:
                f.write(linkml_schema)
            print(f"  ✅ LinkML schema saved: {linkml_file}")

            # Summary
            print(f"\n📊 Ontology Statistics:")
            print(f"   Terms: {len(threat_graph.terms)}")
            print(f"   Relations: {len(threat_graph.relations)}")
            categories = {}
            for term in threat_graph.terms.values():
                categories[term.category] = categories.get(term.category, 0) + 1
            print(f"   Categories: {len(categories)}")
            for cat, count in sorted(categories.items(), key=lambda x: x[1], reverse=True)[:5]:
                print(f"     - {cat}: {count}")

            return threat_graph

        except Exception as e:
            print(f"  ❌ SPIRES ontology generation failed: {e}")
            import traceback
            traceback.print_exc()
            return None

    def convert_yamls_to_dsl(self):
        """Convert all YAML threat content to SX9 DSL format."""
        if not DSL_PIPELINE_AVAILABLE:
            print("\n⚠️  YAML DSL pipeline not available - skipping DSL conversion")
            return None

        print("\n" + "=" * 70)
        print("YAML TO DSL CONVERSION - RFC-9011-B")
        print("=" * 70)

        try:
            # Initialize DSL pipeline
            dsl_output_dir = self.cache_dir.parent / "sx9_dsl"
            pipeline = YAMLDSLPipeline(output_dir=dsl_output_dir)
            
            # Process all threat content
            print("\n[DSL] Converting YAML threat content to SX9 DSL format...")
            results = pipeline.process_threat_content(self.cache_dir)
            
            print(f"  ✅ Validated: {results['validated']} items")
            print(f"  ✅ Converted: {results['converted']} entities")
            
            if results['errors']:
                print(f"  ⚠️  Errors: {len(results['errors'])} validation errors")
            
            # Save results
            summary = pipeline.save_results(results)
            
            print(f"\n📊 DSL Conversion Summary:")
            print(f"   Validated: {summary['validated']}")
            print(f"   Converted: {summary['converted']}")
            print(f"   Errors: {summary['error_count']}")
            print(f"   Output: {summary['output_dir']}")
            
            # Show output files
            if dsl_output_dir.exists():
                output_files = list(dsl_output_dir.glob("*"))
                print(f"\n📁 Generated Files:")
                for f in output_files[:5]:
                    size = f.stat().st_size if f.is_file() else 0
                    print(f"     - {f.name} ({size:,} bytes)")
                if len(output_files) > 5:
                    print(f"     ... and {len(output_files) - 5} more files")

            return summary

        except Exception as e:
            print(f"  ❌ YAML to DSL conversion failed: {e}")
            import traceback
            traceback.print_exc()
            return None

    def train_ml_models(self):
        """Train DistilBERT, Phi, and GNN models on fetched threat content."""
        if not TRAINING_AVAILABLE:
            print("\n⚠️  ML Model Training not available - skipping model training")
            return None

        print("\n" + "=" * 70)
        print("ML MODEL TRAINING - RFC-9012, RFC-9021")
        print("=" * 70)

        try:
            models_output_dir = self.cache_dir.parent / "models"
            models_output_dir.mkdir(parents=True, exist_ok=True)

            # Training configuration
            training_config = TrainingConfig(
                epochs=5,
                batch_size=16,
                learning_rate=2e-5,
                output_dir=str(models_output_dir),
            )

            # 1. Train DistilBERT for MITRE classification
            print("\n[1/3] Training DistilBERT MITRE classifier...")
            distilbert_success = train_distilbert(
                threat_content_dir=str(self.cache_dir),
                output_dir=str(models_output_dir / "distilbert-mitre"),
                config=training_config
            )
            if distilbert_success:
                print("  ✅ DistilBERT training complete")
            else:
                print("  ⚠️  DistilBERT training failed or skipped")

            # 2. Train Phi-3 LoRA
            print("\n[2/3] Training Phi-3 LoRA adapter...")
            phi_success = train_phi_lora(
                threat_content_dir=str(self.cache_dir),
                output_dir=str(models_output_dir / "phi3-mitre-lora"),
                config=training_config
            )
            if phi_success:
                print("  ✅ Phi-3 LoRA training data generated")
            else:
                print("  ⚠️  Phi-3 LoRA training failed or skipped")

            # 3. Train GNN for threat graph analysis
            print("\n[3/3] Training GNN threat analysis model...")
            # Check for Neo4j export (if available from ontology generation)
            neo4j_export = self.cache_dir.parent / "ontology" / "threat_ontology.cypher"
            neo4j_json = None
            if neo4j_export.exists():
                # Convert Cypher to JSON if needed, or use existing JSON
                json_export = self.cache_dir.parent / "ontology" / "threat_ontology.json"
                if json_export.exists():
                    neo4j_json = str(json_export)

            gnn_success = train_gnn(
                threat_content_dir=str(self.cache_dir),
                output_dir=str(models_output_dir / "gnn-threat"),
                neo4j_export=neo4j_json,
                config=training_config
            )
            if gnn_success:
                print("  ✅ GNN training data generated")
            else:
                print("  ⚠️  GNN training failed or skipped")

            print(f"\n📊 Training Summary:")
            print(f"   Models directory: {models_output_dir}")
            print(f"   DistilBERT: {'✅' if distilbert_success else '❌'}")
            print(f"   Phi-3 LoRA: {'✅' if phi_success else '❌'}")
            print(f"   GNN: {'✅' if gnn_success else '❌'}")

            return {
                "distilbert": distilbert_success,
                "phi": phi_success,
                "gnn": gnn_success,
                "output_dir": str(models_output_dir)
            }

        except Exception as e:
            print(f"  ❌ ML model training failed: {e}")
            import traceback
            traceback.print_exc()
            return None

    def save_indexes(self):
        """Save content indexes for quick lookup."""
        # MITRE technique index
        with open(self.cache_dir / "mitre_index.json", 'w') as f:
            json.dump({
                "techniques": self.content["mitre_techniques"],
                "groups": self.content["mitre_groups"],
            }, f, indent=2)

        # Crosswalk index
        with open(self.cache_dir / "crosswalk_index.json", 'w') as f:
            json.dump(self.crosswalk, f, indent=2)

        # Full content summary
        summary = {
            "fetched_at": datetime.now().isoformat(),
            "counts": {
                "mitre_techniques": len(self.content["mitre_techniques"]),
                "mitre_groups": len(self.content["mitre_groups"]),
                "atomic_tests": len(self.content["atomic_tests"]),
                "nuclei_templates": len(self.content["nuclei_templates"]),
                "sigma_rules": len(self.content["sigma_rules"]),
                "caldera_abilities": len(self.content["caldera_abilities"]),
                "nmap_scripts": len(self.content["nmap_scripts"]),
            },
            "crosswalk_counts": {
                "technique_to_atomic": len(self.crosswalk["technique_to_atomic"]),
                "technique_to_nuclei": len(self.crosswalk["technique_to_nuclei"]),
                "technique_to_sigma": len(self.crosswalk["technique_to_sigma"]),
            }
        }

        with open(self.cache_dir / "threat_content_summary.json", 'w') as f:
            json.dump(summary, f, indent=2)

        print(f"\nIndexes saved to {self.cache_dir}")
        
        # Cleanup external repos if requested (to minimize local data)
        if self.cleanup_repos_after_processing:
            self._cleanup_external_repos()
    
    def _cleanup_external_repos(self):
        """Delete external repository clones after processing to minimize local storage.
        
        Keeps only processed JSON files. Repos can be re-cloned when needed.
        """
        external_repos = [
            "car", "atlas", "engage", "atomic-red-team", "nuclei-templates",
            "sigma", "yara-rules", "wazuh", "lolbas", "gtfobins", "loldrivers",
            "hijacklibs", "wadcoms", "nmap", "awesome-osint", "osint-framework",
            "sherlock", "caldera", "exploitdb"
        ]
        
        total_freed = 0
        removed_count = 0
        
        print("\n🧹 Cleaning up external repositories (keeping processed JSON only)...")
        
        for repo_name in external_repos:
            repo_path = self.cache_dir / repo_name
            if repo_path.exists() and repo_path.is_dir():
                try:
                    # Calculate size before deletion
                    size = sum(f.stat().st_size for f in repo_path.rglob('*') if f.is_file())
                    total_freed += size
                    
                    # Remove directory
                    shutil.rmtree(repo_path, ignore_errors=True)
                    removed_count += 1
                    print(f"   ✅ Removed: {repo_name} ({size / 1024 / 1024:.1f} MB)")
                except Exception as e:
                    print(f"   ⚠️  Failed to remove {repo_name}: {e}")
        
        print(f"\n✅ Cleanup complete:")
        print(f"   • Removed {removed_count} repositories")
        print(f"   • Freed {total_freed / 1024 / 1024:.1f} MB")
        print(f"   • Kept processed JSON files only")
        print(f"   • Repos can be re-cloned when needed for updates")

    def get_technique_coverage(self, tech_id: str) -> dict:
        """Get all content covering a specific technique."""
        return {
            "mitre": self.content["mitre_techniques"].get(tech_id),
            "atomic_tests": self.content["atomic_tests"].get(tech_id, {}).get("test_count", 0),
            "nuclei_templates": len(self.crosswalk["technique_to_nuclei"].get(tech_id, [])),
            "sigma_rules": len(self.crosswalk["technique_to_sigma"].get(tech_id, [])),
        }


def main():
    import argparse
    parser = argparse.ArgumentParser(description="RFC-9011 Threat Content Fetcher with SPIRES Ontology")
    parser.add_argument("--all", action="store_true", help="Fetch all sources and generate ontology")
    parser.add_argument("--mitre", action="store_true", help="Fetch MITRE ATT&CK only")
    parser.add_argument("--atomic", action="store_true", help="Fetch Atomic Red Team only")
    parser.add_argument("--nuclei", action="store_true", help="Fetch Nuclei templates only")
    parser.add_argument("--sigma", action="store_true", help="Fetch Sigma rules only")
    parser.add_argument("--lookup", type=str, help="Look up coverage for technique ID")
    parser.add_argument("--no-ontology", action="store_true", help="Skip SPIRES ontology generation")
    parser.add_argument("--no-dsl", action="store_true", help="Skip YAML to DSL conversion")
    parser.add_argument("--no-training", action="store_true", help="Skip ML model training")
    parser.add_argument("--train-only", action="store_true", help="Only run training (skip fetch/ontology/DSL)")
    parser.add_argument("--cleanup-repos", action="store_true", help="Delete external repos after processing (minimize local data)")
    args = parser.parse_args()

    fetcher = ThreatContentFetcher(cleanup_repos_after_processing=args.cleanup_repos)

    # Training-only mode
    if args.train_only:
        print("🎯 Training-only mode: Skipping fetch/ontology/DSL")
        fetcher.train_ml_models()
        return

    # Normal pipeline
    if args.all or not any([args.mitre, args.atomic, args.nuclei, args.sigma, args.lookup]):
        fetcher.fetch_all()
        fetcher.save_indexes()
        
        # Generate SPIRES ontology after fetching all content
        if not args.no_ontology and (args.all or not args.lookup):
            fetcher.generate_spires_ontology()
        
        # Convert YAMLs to DSL after fetching all content
        if not args.no_dsl and (args.all or not args.lookup):
            fetcher.convert_yamls_to_dsl()
        
        # Train ML models after all processing
        if not args.no_training and (args.all or not args.lookup):
            fetcher.train_ml_models()

    else:
        if args.mitre:
            fetcher.fetch_mitre_attack()
        if args.atomic:
            fetcher.fetch_atomic_red_team()
        if args.nuclei:
            fetcher.fetch_nuclei_templates()
        if args.sigma:
            fetcher.fetch_sigma_rules()

        fetcher.save_indexes()
        
        # Generate SPIRES ontology if content was fetched
        if not args.no_ontology and any([args.mitre, args.atomic, args.nuclei, args.sigma]):
            fetcher.generate_spires_ontology()
        
        # Convert YAMLs to DSL if content was fetched
        if not args.no_dsl and any([args.mitre, args.atomic, args.nuclei, args.sigma]):
            fetcher.convert_yamls_to_dsl()
        
        # Train ML models if content was fetched
        if not args.no_training and any([args.mitre, args.atomic, args.nuclei, args.sigma]):
            fetcher.train_ml_models()

    if args.lookup:
        coverage = fetcher.get_technique_coverage(args.lookup)
        print(f"\nCoverage for {args.lookup}:")
        print(json.dumps(coverage, indent=2))


if __name__ == "__main__":
    main()
