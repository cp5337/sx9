#!/usr/bin/env python3
"""
SX9 Threat Intelligence Normalization Pipeline
===============================================

Ingests raw threat data from 27 sources and normalizes into:
- tools (unified tool registry)
- techniques (MITRE ATT&CK)
- tool_technique_map (tool → technique)
- task_technique_map (ctas_task → technique)
- playbooks (tool chains)

All entities receive RFC-9001 trivariate hashes.

Usage:
    python3 normalize_threat_intel.py --input ./output/threat_content --output ./normalized
"""

import os
import json
import csv
import hashlib
import re
from pathlib import Path
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional, Set, Tuple
from datetime import datetime
import yaml

# ═══════════════════════════════════════════════════════════════════════════
# RFC-9001 Hashing
# ═══════════════════════════════════════════════════════════════════════════

try:
    import mmh3
    HAS_MMH3 = True
except ImportError:
    HAS_MMH3 = False
    print("⚠️  mmh3 not installed. Using hashlib fallback. pip install mmh3")


def murmur3_64(data: str) -> int:
    """Generate Murmur3-64 hash."""
    if HAS_MMH3:
        return mmh3.hash64(data.encode('utf-8'), signed=False)[0]
    else:
        # Fallback to SHA256 truncated to 64 bits
        h = hashlib.sha256(data.encode('utf-8')).digest()
        return int.from_bytes(h[:8], 'big')


def generate_sch(name: str, category: str, source: str) -> str:
    """Generate SCH (Semantic Content Hash) - 16 hex chars."""
    content = f"{name}|{category}|{source}"
    h = murmur3_64(content)
    return f"{h:016x}"


def generate_cuid(name: str, timestamp: str = None) -> str:
    """Generate CUID (Content Unique ID) - 16 hex chars."""
    ts = timestamp or datetime.utcnow().isoformat()
    content = f"{name}|{ts}"
    h = murmur3_64(content)
    return f"{h:016x}"


def generate_uuid_hash(name: str, source: str, idx: int) -> str:
    """Generate UUID component - 16 hex chars."""
    content = f"{name}|{source}|{idx}"
    h = murmur3_64(content)
    return f"{h:016x}"


def generate_trivariate(name: str, category: str, source: str, idx: int) -> Dict[str, str]:
    """Generate full RFC-9001 trivariate hash."""
    sch = generate_sch(name, category, source)
    cuid = generate_cuid(name)
    uuid_h = generate_uuid_hash(name, source, idx)
    
    # H1 Operational = SCH + CUID + UUID (48 hex chars)
    h1_operational = f"{sch}{cuid}{uuid_h}"
    
    # H2 Semantic = hash of H1 (16 hex chars)
    h2_semantic = f"{murmur3_64(h1_operational):016x}"
    
    # Unicode visual rune (Class A: E000-E0FF for tools)
    rune_offset = int(sch[:4], 16) % 256
    unicode_rune = f"E0{rune_offset:02X}"
    
    return {
        "h1_operational": h1_operational,
        "h1_sch": sch,
        "h1_cuid": cuid,
        "h1_uuid": uuid_h,
        "h2_semantic": h2_semantic,
        "unicode_rune": unicode_rune
    }


# ═══════════════════════════════════════════════════════════════════════════
# Data Models
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class Tool:
    """Normalized tool record."""
    id: str
    name: str
    source: str  # kali, atomic, caldera, nuclei, lolbas, gtfobins, etc
    category: str
    description: str
    command: Optional[str] = None
    default_args: List[str] = field(default_factory=list)
    url: Optional[str] = None
    techniques: List[str] = field(default_factory=list)  # ATT&CK technique IDs
    
    # Hashes
    h1_operational: str = ""
    h1_sch: str = ""
    h1_cuid: str = ""
    h1_uuid: str = ""
    h2_semantic: str = ""
    unicode_rune: str = ""
    
    # Metadata
    hd4_phase: str = "Hunt"
    ptcc_primitive: int = 1  # READ by default
    installed: bool = False
    success_rate: int = 85


@dataclass
class Technique:
    """MITRE ATT&CK technique."""
    id: str  # T1234 or T1234.001
    name: str
    tactic: str
    description: str
    detection: str = ""
    platforms: List[str] = field(default_factory=list)
    data_sources: List[str] = field(default_factory=list)
    
    # Hashes
    h1_operational: str = ""
    h2_semantic: str = ""


@dataclass
class ToolTechniqueMap:
    """Tool → Technique mapping."""
    tool_id: str
    technique_id: str
    source: str  # Where this mapping came from
    confidence: float = 1.0


@dataclass 
class TaskTechniqueMap:
    """CTAS Task → Technique mapping."""
    task_id: str
    technique_id: str
    coverage_type: str  # "direct", "partial", "related"


@dataclass
class Playbook:
    """Tool chain / playbook."""
    id: str
    name: str
    description: str
    tools: List[str]  # Tool IDs
    techniques: List[str]  # Technique IDs covered
    hd4_phase: str
    category: str
    
    # Hashes
    h1_operational: str = ""
    h2_semantic: str = ""


# ═══════════════════════════════════════════════════════════════════════════
# Source Parsers
# ═══════════════════════════════════════════════════════════════════════════

class ThreatIntelNormalizer:
    """Normalizes threat intelligence from multiple sources."""
    
    def __init__(self, input_dir: str, output_dir: str):
        self.input_dir = Path(input_dir)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        self.tools: Dict[str, Tool] = {}
        self.techniques: Dict[str, Technique] = {}
        self.tool_technique_maps: List[ToolTechniqueMap] = []
        self.playbooks: List[Playbook] = []
        
        self.tool_idx = 0
        
    def run(self):
        """Run full normalization pipeline."""
        print("═" * 60)
        print("SX9 Threat Intelligence Normalization Pipeline")
        print("═" * 60)
        
        # 1. Load MITRE ATT&CK techniques first (they're referenced by everything)
        self.load_mitre_attack()
        
        # 2. Load tools from each source
        self.load_kali_tools()
        self.load_atomic_red_team()
        self.load_caldera()
        self.load_nuclei()
        self.load_lolbas()
        self.load_gtfobins()
        self.load_sigma()
        
        # 3. Generate playbooks from tool chains
        self.generate_playbooks()
        
        # 4. Output normalized data
        self.output_all()
        
        print("═" * 60)
        print(f"✅ Normalization complete!")
        print(f"   Tools: {len(self.tools)}")
        print(f"   Techniques: {len(self.techniques)}")
        print(f"   Tool→Technique maps: {len(self.tool_technique_maps)}")
        print(f"   Playbooks: {len(self.playbooks)}")
        print("═" * 60)
    
    # ─────────────────────────────────────────────────────────────────────────
    # MITRE ATT&CK
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_mitre_attack(self):
        """Load MITRE ATT&CK techniques from enterprise-attack.json."""
        print("\n📥 Loading MITRE ATT&CK...")
        
        attack_file = self.input_dir / "mitre_attack_enterprise" / "enterprise-attack.json"
        if not attack_file.exists():
            # Try alternate locations
            for alt in ["mitre_attack/enterprise-attack.json", "enterprise-attack.json"]:
                alt_path = self.input_dir / alt
                if alt_path.exists():
                    attack_file = alt_path
                    break
        
        if not attack_file.exists():
            print(f"   ⚠️  ATT&CK file not found: {attack_file}")
            return
        
        with open(attack_file, 'r') as f:
            data = json.load(f)
        
        # Parse STIX objects
        tactics = {}
        for obj in data.get('objects', []):
            if obj.get('type') == 'x-mitre-tactic':
                tactics[obj['x_mitre_shortname']] = obj['name']
        
        for obj in data.get('objects', []):
            if obj.get('type') != 'attack-pattern':
                continue
            
            ext_refs = obj.get('external_references', [])
            technique_id = None
            for ref in ext_refs:
                if ref.get('source_name') == 'mitre-attack':
                    technique_id = ref.get('external_id')
                    break
            
            if not technique_id:
                continue
            
            # Get tactic from kill_chain_phases
            tactic = "unknown"
            for phase in obj.get('kill_chain_phases', []):
                if phase.get('kill_chain_name') == 'mitre-attack':
                    tactic = phase.get('phase_name', 'unknown')
                    break
            
            tech = Technique(
                id=technique_id,
                name=obj.get('name', ''),
                tactic=tactic,
                description=obj.get('description', '')[:500],
                detection=obj.get('x_mitre_detection', '')[:500],
                platforms=obj.get('x_mitre_platforms', []),
                data_sources=obj.get('x_mitre_data_sources', [])
            )
            
            # Generate hashes
            hashes = generate_trivariate(tech.name, tactic, "mitre-attack", len(self.techniques))
            tech.h1_operational = hashes['h1_operational']
            tech.h2_semantic = hashes['h2_semantic']
            
            self.techniques[technique_id] = tech
        
        print(f"   ✅ Loaded {len(self.techniques)} techniques")
    
    # ─────────────────────────────────────────────────────────────────────────
    # Kali Tools
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_kali_tools(self):
        """Load Kali Linux tools."""
        print("\n📥 Loading Kali Tools...")
        
        kali_dir = self.input_dir / "kali_tools"
        if not kali_dir.exists():
            print(f"   ⚠️  Kali tools dir not found: {kali_dir}")
            return
        
        count = 0
        for file in kali_dir.glob("*.json"):
            with open(file, 'r') as f:
                try:
                    data = json.load(f)
                except:
                    continue
            
            if isinstance(data, list):
                for item in data:
                    self._add_kali_tool(item)
                    count += 1
            elif isinstance(data, dict):
                self._add_kali_tool(data)
                count += 1
        
        # Also try CSV
        for file in kali_dir.glob("*.csv"):
            with open(file, 'r') as f:
                reader = csv.DictReader(f)
                for row in reader:
                    self._add_kali_tool(row)
                    count += 1
        
        print(f"   ✅ Loaded {count} Kali tools")
    
    def _add_kali_tool(self, data: dict):
        """Add a Kali tool to registry."""
        name = data.get('name') or data.get('tool_name') or data.get('package', '')
        if not name:
            return
        
        tool_id = f"kali-{name.lower().replace(' ', '-')}"
        if tool_id in self.tools:
            return
        
        category = data.get('category', 'general').lower()
        
        # Map category to HD4 phase
        hd4_map = {
            'reconnaissance': 'Hunt',
            'information-gathering': 'Hunt',
            'vulnerability': 'Detect',
            'exploitation': 'Disrupt',
            'password': 'Disrupt',
            'wireless': 'Disrupt',
            'forensics': 'Detect',
            'sniffing': 'Detect',
            'web': 'Hunt',
            'post-exploitation': 'Dominate'
        }
        hd4_phase = 'Hunt'
        for key, phase in hd4_map.items():
            if key in category:
                hd4_phase = phase
                break
        
        # Map category to PTCC primitive
        ptcc_map = {
            'reconnaissance': 1,  # READ
            'exploitation': 8,    # TRANSFORM
            'password': 15,       # DECRYPT
            'wireless': 4,        # CONNECT
            'forensics': 9,       # VALIDATE
            'sniffing': 7,        # RECEIVE
        }
        ptcc = 1
        for key, code in ptcc_map.items():
            if key in category:
                ptcc = code
                break
        
        hashes = generate_trivariate(name, category, "kali", self.tool_idx)
        self.tool_idx += 1
        
        tool = Tool(
            id=tool_id,
            name=name,
            source="kali",
            category=category,
            description=data.get('description', '')[:500],
            command=data.get('command') or name.lower(),
            url=data.get('url', ''),
            hd4_phase=hd4_phase,
            ptcc_primitive=ptcc,
            **hashes
        )
        
        self.tools[tool_id] = tool
    
    # ─────────────────────────────────────────────────────────────────────────
    # Atomic Red Team
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_atomic_red_team(self):
        """Load Atomic Red Team tests."""
        print("\n📥 Loading Atomic Red Team...")
        
        atomic_dir = self.input_dir / "atomic_red_team" / "atomics"
        if not atomic_dir.exists():
            atomic_dir = self.input_dir / "atomic-red-team" / "atomics"
        if not atomic_dir.exists():
            print(f"   ⚠️  Atomic Red Team dir not found")
            return
        
        count = 0
        maps = 0
        
        for technique_dir in atomic_dir.iterdir():
            if not technique_dir.is_dir():
                continue
            
            # Directory name is technique ID (T1234)
            technique_id = technique_dir.name
            if not technique_id.startswith('T'):
                continue
            
            yaml_file = technique_dir / f"{technique_id}.yaml"
            if not yaml_file.exists():
                continue
            
            with open(yaml_file, 'r') as f:
                try:
                    data = yaml.safe_load(f)
                except:
                    continue
            
            if not data:
                continue
            
            # Each atomic test becomes a tool
            for i, test in enumerate(data.get('atomic_tests', [])):
                test_name = test.get('name', f'{technique_id}-test-{i}')
                tool_id = f"atomic-{technique_id.lower()}-{i}"
                
                if tool_id in self.tools:
                    continue
                
                # Extract command
                executor = test.get('executor', {})
                command = executor.get('command', '')
                if isinstance(command, str):
                    command = command[:200]  # Truncate
                
                hashes = generate_trivariate(test_name, "atomic", "atomic-red-team", self.tool_idx)
                self.tool_idx += 1
                
                tool = Tool(
                    id=tool_id,
                    name=test_name,
                    source="atomic-red-team",
                    category="adversary-emulation",
                    description=test.get('description', '')[:500],
                    command=command,
                    techniques=[technique_id],
                    hd4_phase="Disrupt",
                    ptcc_primitive=22,  # CALL
                    **hashes
                )
                
                self.tools[tool_id] = tool
                count += 1
                
                # Add tool→technique mapping
                self.tool_technique_maps.append(ToolTechniqueMap(
                    tool_id=tool_id,
                    technique_id=technique_id,
                    source="atomic-red-team",
                    confidence=1.0
                ))
                maps += 1
        
        print(f"   ✅ Loaded {count} atomic tests, {maps} technique mappings")
    
    # ─────────────────────────────────────────────────────────────────────────
    # Caldera
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_caldera(self):
        """Load Caldera abilities."""
        print("\n📥 Loading Caldera abilities...")
        
        caldera_dir = self.input_dir / "caldera"
        if not caldera_dir.exists():
            print(f"   ⚠️  Caldera dir not found")
            return
        
        count = 0
        maps = 0
        
        # Find ability files
        for yaml_file in caldera_dir.rglob("*.yml"):
            with open(yaml_file, 'r') as f:
                try:
                    data = yaml.safe_load(f)
                except:
                    continue
            
            if not isinstance(data, list):
                data = [data]
            
            for ability in data:
                if not isinstance(ability, dict):
                    continue
                
                ability_id = ability.get('id', '')
                name = ability.get('name', '')
                if not name:
                    continue
                
                tool_id = f"caldera-{ability_id}" if ability_id else f"caldera-{name.lower().replace(' ', '-')}"
                if tool_id in self.tools:
                    continue
                
                technique_id = ability.get('technique', {}).get('attack_id', '')
                
                hashes = generate_trivariate(name, "caldera", "caldera", self.tool_idx)
                self.tool_idx += 1
                
                tool = Tool(
                    id=tool_id,
                    name=name,
                    source="caldera",
                    category="adversary-emulation",
                    description=ability.get('description', '')[:500],
                    techniques=[technique_id] if technique_id else [],
                    hd4_phase="Disrupt",
                    ptcc_primitive=22,  # CALL
                    **hashes
                )
                
                self.tools[tool_id] = tool
                count += 1
                
                if technique_id:
                    self.tool_technique_maps.append(ToolTechniqueMap(
                        tool_id=tool_id,
                        technique_id=technique_id,
                        source="caldera",
                        confidence=1.0
                    ))
                    maps += 1
        
        print(f"   ✅ Loaded {count} Caldera abilities, {maps} technique mappings")
    
    # ─────────────────────────────────────────────────────────────────────────
    # Nuclei
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_nuclei(self):
        """Load Nuclei templates."""
        print("\n📥 Loading Nuclei templates...")
        
        nuclei_dir = self.input_dir / "nuclei_templates"
        if not nuclei_dir.exists():
            nuclei_dir = self.input_dir / "nuclei-templates"
        if not nuclei_dir.exists():
            print(f"   ⚠️  Nuclei templates dir not found")
            return
        
        count = 0
        
        for yaml_file in nuclei_dir.rglob("*.yaml"):
            with open(yaml_file, 'r') as f:
                try:
                    data = yaml.safe_load(f)
                except:
                    continue
            
            if not data:
                continue
            
            info = data.get('info', {})
            name = info.get('name', yaml_file.stem)
            
            tool_id = f"nuclei-{yaml_file.stem}"
            if tool_id in self.tools:
                continue
            
            # Extract category from path
            category = yaml_file.parent.name
            
            # Extract CVE/technique references
            techniques = []
            refs = info.get('reference', [])
            if isinstance(refs, list):
                for ref in refs:
                    if 'attack.mitre.org' in str(ref):
                        # Extract technique ID
                        match = re.search(r'T\d{4}(\.\d{3})?', str(ref))
                        if match:
                            techniques.append(match.group())
            
            hashes = generate_trivariate(name, category, "nuclei", self.tool_idx)
            self.tool_idx += 1
            
            tool = Tool(
                id=tool_id,
                name=name,
                source="nuclei",
                category=category,
                description=info.get('description', '')[:500],
                techniques=techniques,
                hd4_phase="Detect",
                ptcc_primitive=9,  # VALIDATE
                **hashes
            )
            
            self.tools[tool_id] = tool
            count += 1
            
            for tech_id in techniques:
                self.tool_technique_maps.append(ToolTechniqueMap(
                    tool_id=tool_id,
                    technique_id=tech_id,
                    source="nuclei",
                    confidence=0.8
                ))
        
        print(f"   ✅ Loaded {count} Nuclei templates")
    
    # ─────────────────────────────────────────────────────────────────────────
    # LOLBAS
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_lolbas(self):
        """Load LOLBAS (Living Off The Land Binaries And Scripts)."""
        print("\n📥 Loading LOLBAS...")
        
        lolbas_dir = self.input_dir / "lolbas"
        if not lolbas_dir.exists():
            lolbas_dir = self.input_dir / "LOLBAS"
        if not lolbas_dir.exists():
            print(f"   ⚠️  LOLBAS dir not found")
            return
        
        count = 0
        
        for yaml_file in lolbas_dir.rglob("*.yml"):
            with open(yaml_file, 'r') as f:
                try:
                    data = yaml.safe_load(f)
                except:
                    continue
            
            if not data:
                continue
            
            name = data.get('Name', yaml_file.stem)
            tool_id = f"lolbas-{name.lower()}"
            if tool_id in self.tools:
                continue
            
            # Extract technique mappings
            techniques = []
            for cmd in data.get('Commands', []):
                mitre = cmd.get('MitreID', '')
                if mitre:
                    techniques.append(mitre)
            
            hashes = generate_trivariate(name, "lolbas", "lolbas", self.tool_idx)
            self.tool_idx += 1
            
            tool = Tool(
                id=tool_id,
                name=name,
                source="lolbas",
                category="living-off-the-land",
                description=data.get('Description', '')[:500],
                command=name.lower(),
                techniques=list(set(techniques)),
                hd4_phase="Disrupt",
                ptcc_primitive=22,  # CALL
                **hashes
            )
            
            self.tools[tool_id] = tool
            count += 1
            
            for tech_id in set(techniques):
                self.tool_technique_maps.append(ToolTechniqueMap(
                    tool_id=tool_id,
                    technique_id=tech_id,
                    source="lolbas",
                    confidence=1.0
                ))
        
        print(f"   ✅ Loaded {count} LOLBAS binaries")
    
    # ─────────────────────────────────────────────────────────────────────────
    # GTFOBins
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_gtfobins(self):
        """Load GTFOBins (Unix binary abuse)."""
        print("\n📥 Loading GTFOBins...")
        
        gtfo_dir = self.input_dir / "gtfobins"
        if not gtfo_dir.exists():
            gtfo_dir = self.input_dir / "GTFOBins"
        if not gtfo_dir.exists():
            print(f"   ⚠️  GTFOBins dir not found")
            return
        
        count = 0
        
        for md_file in gtfo_dir.rglob("*.md"):
            name = md_file.stem
            if name.startswith('_'):
                continue
            
            tool_id = f"gtfobins-{name.lower()}"
            if tool_id in self.tools:
                continue
            
            # Read markdown to get functions
            with open(md_file, 'r') as f:
                content = f.read()
            
            # Extract functions (shell, file-read, file-write, etc)
            functions = []
            for line in content.split('\n'):
                if line.startswith('## '):
                    functions.append(line[3:].strip().lower())
            
            hashes = generate_trivariate(name, "gtfobins", "gtfobins", self.tool_idx)
            self.tool_idx += 1
            
            tool = Tool(
                id=tool_id,
                name=name,
                source="gtfobins",
                category="living-off-the-land",
                description=f"Unix binary abuse: {', '.join(functions[:5])}",
                command=name.lower(),
                hd4_phase="Dominate",
                ptcc_primitive=22,  # CALL
                **hashes
            )
            
            self.tools[tool_id] = tool
            count += 1
        
        print(f"   ✅ Loaded {count} GTFOBins")
    
    # ─────────────────────────────────────────────────────────────────────────
    # Sigma Rules
    # ─────────────────────────────────────────────────────────────────────────
    
    def load_sigma(self):
        """Load Sigma detection rules."""
        print("\n📥 Loading Sigma rules...")
        
        sigma_dir = self.input_dir / "sigma_rules"
        if not sigma_dir.exists():
            sigma_dir = self.input_dir / "sigma"
        if not sigma_dir.exists():
            print(f"   ⚠️  Sigma rules dir not found")
            return
        
        count = 0
        
        for yaml_file in sigma_dir.rglob("*.yml"):
            with open(yaml_file, 'r') as f:
                try:
                    data = yaml.safe_load(f)
                except:
                    continue
            
            if not data:
                continue
            
            title = data.get('title', yaml_file.stem)
            tool_id = f"sigma-{yaml_file.stem}"
            if tool_id in self.tools:
                continue
            
            # Extract ATT&CK tags
            techniques = []
            for tag in data.get('tags', []):
                if tag.startswith('attack.t'):
                    tech_id = tag.replace('attack.', '').upper()
                    techniques.append(tech_id)
            
            hashes = generate_trivariate(title, "detection", "sigma", self.tool_idx)
            self.tool_idx += 1
            
            tool = Tool(
                id=tool_id,
                name=title,
                source="sigma",
                category="detection-rule",
                description=data.get('description', '')[:500],
                techniques=techniques,
                hd4_phase="Detect",
                ptcc_primitive=9,  # VALIDATE
                **hashes
            )
            
            self.tools[tool_id] = tool
            count += 1
            
            for tech_id in techniques:
                self.tool_technique_maps.append(ToolTechniqueMap(
                    tool_id=tool_id,
                    technique_id=tech_id,
                    source="sigma",
                    confidence=0.9
                ))
        
        print(f"   ✅ Loaded {count} Sigma rules")
    
    # ─────────────────────────────────────────────────────────────────────────
    # Playbook Generation
    # ─────────────────────────────────────────────────────────────────────────
    
    def generate_playbooks(self):
        """Generate playbooks from tool combinations."""
        print("\n📥 Generating playbooks...")
        
        # Group tools by technique
        tools_by_technique: Dict[str, List[str]] = {}
        for mapping in self.tool_technique_maps:
            if mapping.technique_id not in tools_by_technique:
                tools_by_technique[mapping.technique_id] = []
            tools_by_technique[mapping.technique_id].append(mapping.tool_id)
        
        # Create playbooks for techniques with multiple tools
        count = 0
        for tech_id, tool_ids in tools_by_technique.items():
            if len(tool_ids) < 2:
                continue
            
            technique = self.techniques.get(tech_id)
            if not technique:
                continue
            
            # Take top 5 tools
            tools = tool_ids[:5]
            
            playbook_id = f"playbook-{tech_id.lower()}"
            hashes = generate_trivariate(technique.name, "playbook", "generated", count)
            
            playbook = Playbook(
                id=playbook_id,
                name=f"{technique.name} Chain",
                description=f"Tool chain for {tech_id}: {technique.name}",
                tools=tools,
                techniques=[tech_id],
                hd4_phase=self._tactic_to_hd4(technique.tactic),
                category=technique.tactic,
                h1_operational=hashes['h1_operational'],
                h2_semantic=hashes['h2_semantic']
            )
            
            self.playbooks.append(playbook)
            count += 1
        
        print(f"   ✅ Generated {count} playbooks")
    
    def _tactic_to_hd4(self, tactic: str) -> str:
        """Map ATT&CK tactic to HD4 phase."""
        mapping = {
            'reconnaissance': 'Hunt',
            'resource-development': 'Hunt',
            'initial-access': 'Disrupt',
            'execution': 'Disrupt',
            'persistence': 'Dominate',
            'privilege-escalation': 'Disrupt',
            'defense-evasion': 'Detect',
            'credential-access': 'Disrupt',
            'discovery': 'Hunt',
            'lateral-movement': 'Disrupt',
            'collection': 'Hunt',
            'command-and-control': 'Dominate',
            'exfiltration': 'Dominate',
            'impact': 'Disable'
        }
        return mapping.get(tactic, 'Hunt')
    
    # ─────────────────────────────────────────────────────────────────────────
    # Output
    # ─────────────────────────────────────────────────────────────────────────
    
    def output_all(self):
        """Output all normalized data."""
        print("\n📤 Writing output files...")
        
        # Tools
        tools_file = self.output_dir / "tools.json"
        with open(tools_file, 'w') as f:
            json.dump([asdict(t) for t in self.tools.values()], f, indent=2)
        print(f"   ✅ {tools_file}")
        
        # Tools CSV for Supabase
        tools_csv = self.output_dir / "tools.csv"
        with open(tools_csv, 'w', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=[
                'id', 'name', 'source', 'category', 'description', 'command',
                'hd4_phase', 'ptcc_primitive', 'h1_operational', 'h1_sch', 
                'h1_cuid', 'h1_uuid', 'h2_semantic', 'unicode_rune'
            ])
            writer.writeheader()
            for t in self.tools.values():
                writer.writerow({
                    'id': t.id, 'name': t.name, 'source': t.source,
                    'category': t.category, 'description': t.description[:200],
                    'command': t.command or '', 'hd4_phase': t.hd4_phase,
                    'ptcc_primitive': t.ptcc_primitive,
                    'h1_operational': t.h1_operational, 'h1_sch': t.h1_sch,
                    'h1_cuid': t.h1_cuid, 'h1_uuid': t.h1_uuid,
                    'h2_semantic': t.h2_semantic, 'unicode_rune': t.unicode_rune
                })
        print(f"   ✅ {tools_csv}")
        
        # Techniques
        tech_file = self.output_dir / "techniques.json"
        with open(tech_file, 'w') as f:
            json.dump([asdict(t) for t in self.techniques.values()], f, indent=2)
        print(f"   ✅ {tech_file}")
        
        # Tool→Technique mappings
        maps_file = self.output_dir / "tool_technique_map.json"
        with open(maps_file, 'w') as f:
            json.dump([asdict(m) for m in self.tool_technique_maps], f, indent=2)
        print(f"   ✅ {maps_file}")
        
        # Playbooks
        pb_file = self.output_dir / "playbooks.json"
        with open(pb_file, 'w') as f:
            json.dump([asdict(p) for p in self.playbooks], f, indent=2)
        print(f"   ✅ {pb_file}")
        
        # SQL seed file
        self._output_sql()
    
    def _output_sql(self):
        """Generate SQL seed files for Supabase."""
        sql_file = self.output_dir / "seed_all.sql"
        
        with open(sql_file, 'w') as f:
            f.write("-- SX9 Threat Intelligence Seed Data\n")
            f.write(f"-- Generated: {datetime.utcnow().isoformat()}\n")
            f.write("-- Tools: {}, Techniques: {}, Mappings: {}\n\n".format(
                len(self.tools), len(self.techniques), len(self.tool_technique_maps)
            ))
            
            # Tools
            f.write("-- ═══════════════════════════════════════════════════════════════\n")
            f.write("-- TOOLS\n")
            f.write("-- ═══════════════════════════════════════════════════════════════\n\n")
            
            for tool in list(self.tools.values())[:500]:  # Limit for reasonable SQL
                name_escaped = tool.name.replace("'", "''")
                desc_escaped = (tool.description or '').replace("'", "''")[:200]
                
                f.write(f"""INSERT INTO kali_tools (uuid, tool_name, category, description, hd4_phase, sch, cuid, created_at, updated_at)
VALUES ('{tool.unicode_rune}', '{name_escaped}', '{tool.category}', '{desc_escaped}', '{tool.hd4_phase}', '{tool.h1_sch}', '{tool.h1_cuid}', now(), now())
ON CONFLICT (tool_name) DO UPDATE SET category = EXCLUDED.category, updated_at = now();

""")
            
            f.write("\n-- Done\n")
        
        print(f"   ✅ {sql_file}")


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Normalize threat intelligence data")
    parser.add_argument('--input', '-i', default='./output/threat_content',
                        help='Input directory with raw threat data')
    parser.add_argument('--output', '-o', default='./normalized',
                        help='Output directory for normalized data')
    
    args = parser.parse_args()
    
    normalizer = ThreatIntelNormalizer(args.input, args.output)
    normalizer.run()


if __name__ == '__main__':
    main()
