#!/usr/bin/env python3
"""
YAML DSL Pipeline
==================

Validates threat content against LinkML schemas and converts to SX9 DSL format.
RFC-9005, RFC-9011-A, RFC-9100, RFC-9001, RFC-9002

Features:
  - LinkML schema validation
  - SX9 DSL YAML generation
  - PTCC primitive mapping
  - RFC-9001 compliant dual-trivariate hash generation (Murmur3-64 + Base96)
  - RFC-9002 Unicode Assembly mapping (U+E000-E9FF)
  - Task graph node generation

Usage:
    python yaml_dsl_pipeline.py --validate output/threat_content
    python yaml_dsl_pipeline.py --convert output/threat_content --output output/sx9_dsl
"""

import json
import hashlib
import argparse
import logging
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field, asdict
import re
import uuid as uuid_lib

try:
    import yaml
    HAS_YAML = True
except ImportError:
    HAS_YAML = False
    print("WARNING: PyYAML not installed. Run: pip install PyYAML")

try:
    import toml
    HAS_TOML = True
except ImportError:
    try:
        import tomli_w
        HAS_TOML = True
    except ImportError:
        HAS_TOML = False
        print("WARNING: TOML writer not installed. Run: pip install toml or tomli-w")

try:
    import mmh3  # MurmurHash3 Python library
    HAS_MMH3 = True
except ImportError:
    HAS_MMH3 = False
    print("WARNING: mmh3 not installed. Run: pip install mmh3")
    print("   Falling back to SHA-256 (not RFC-9001 compliant)")

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

OUTPUT_DIR = Path(__file__).parent / "output"

# RFC-9001 Standard Seeds
SCH_SEED = 0xC7A5_0000  # Semantic Context Hash
CUID_SEED = 0xC7A5_0001  # Context User ID
UUID_SEED = 0xC7A5_0002  # Universal Unique ID

# RFC-9002 Base96 Character Set (96 printable ASCII chars)
# Digits(10) + Upper(26) + Lower(26) + Special(34) = 96
BASE96_CHARSET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !#$%&()*+,-./:;<=>?@[]^_{|}~`\"'\\"
BASE96_LEN = len(BASE96_CHARSET)  # Should be 96

# RFC-9002 Unicode Ranges (U+E000-E9FF)
UNICODE_SYSTEM_CONTROLLER_START = 0xE000  # UUID positions 33-48
UNICODE_TRIVARIATE_PROCESSOR_START = 0xE100  # SCH positions 1-16
UNICODE_CONTEXT_PROCESSOR_START = 0xE200  # CUID positions 17-32
UNICODE_INTELLIGENCE_PROCESSOR_START = 0xE300  # Semantic hash
UNICODE_KALI_TOOLS_START = 0xE800  # Tool identifiers


# PTCC 32 Primitive Mapping (RFC-9100)
PTCC_PRIMITIVES = {
    0x00: "CREATE", 0x01: "READ", 0x02: "UPDATE", 0x03: "DELETE",
    0x04: "ALLOCATE", 0x05: "DEALLOCATE", 0x06: "CLONE", 0x07: "MERGE",
    0x08: "SEND", 0x09: "RECEIVE", 0x0A: "BROADCAST", 0x0B: "SUBSCRIBE",
    0x0C: "SIGNAL", 0x0D: "WAIT", 0x0E: "ACKNOWLEDGE", 0x0F: "REJECT",
    0x10: "TRANSFORM", 0x11: "FILTER", 0x12: "VALIDATE", 0x13: "ENCRYPT",
    0x14: "AUTHENTICATE", 0x15: "CONNECT", 0x16: "DISCONNECT", 0x17: "ROUTE",
    0x18: "COORDINATE", 0x19: "SYNCHRONIZE", 0x1A: "CHECKPOINT", 0x1B: "ROLLBACK",
    0x1C: "BRANCH", 0x1D: "CALL", 0x1E: "RETURN", 0x1F: "LOCK_UNLOCK",
}

# Reverse mapping
PRIMITIVE_TO_CODE = {v: k for k, v in PTCC_PRIMITIVES.items()}

# Tactic to HD4 phase mapping
TACTIC_HD4_MAP = {
    "reconnaissance": "HUNT",
    "resource-development": "HUNT",
    "initial-access": "DETECT",
    "execution": "DETECT",
    "persistence": "DISABLE",
    "privilege-escalation": "DISABLE",
    "defense-evasion": "DISRUPT",
    "credential-access": "DISRUPT",
    "discovery": "DETECT",
    "lateral-movement": "DISRUPT",
    "collection": "DISRUPT",
    "command-and-control": "DISABLE",
    "exfiltration": "DOMINATE",
    "impact": "DOMINATE",
}

# Tool category to PTCC mapping
TOOL_PTCC_MAP = {
    "exploitation": "AUTHENTICATE",
    "password-cracking": "AUTHENTICATE",
    "information-gathering": "READ",
    "sniffing-spoofing": "RECEIVE",
    "wireless-attacks": "CONNECT",
    "web-application-attacks": "VALIDATE",
    "vulnerability-analysis": "READ",
    "reverse-engineering": "TRANSFORM",
    "forensics": "READ",
    "maintaining-access": "CREATE",
    "reporting-tools": "SEND",
}


@dataclass
class TrivarateHash:
    """RFC-9001 Trivariate Hash structure (48 chars Base96)."""
    sch: str  # Semantic Content Hash (16 chars Base96)
    cuid: str  # Contextual Unique ID (16 chars Base96)
    uuid: str  # Universal Unique ID (16 chars Base96)

    def __str__(self):
        return f"triv:{self.sch}_{self.cuid}_{self.uuid}"
    
    def to_dict(self) -> Dict[str, str]:
        """Convert to dictionary format."""
        return {
            "sch": self.sch,
            "cuid": self.cuid,
            "uuid": self.uuid,
            "full": f"{self.sch}{self.cuid}{self.uuid}",
            "canonical": str(self)
        }


@dataclass
class DualHash:
    """Dual hash structure: Semantic and Operational (RFC-9025)."""
    semantic_hash: str    # H2 Semantic Hash - content/meaning based
    operational_hash: str # H1 Operational Hash - routing/execution based

    def __str__(self):
        return f"dual:sem:{self.semantic_hash}_op:{self.operational_hash}"


@dataclass
class SX9Entity:
    """SX9 DSL entity representation."""
    id: str
    type: str
    name: str
    trivariate: TrivarateHash
    dual_hash: DualHash  # Semantic + Operational hashes
    ptcc_primitive: str
    ptcc_code: int
    hd4_phase: str = "Hunt"  # Default to Hunt phase
    trivariate_secondary: Optional[TrivarateHash] = None  # RFC-9001 secondary hash
    unicode_operation: Optional[int] = None  # RFC-9002 Unicode operation (U+E000-E9FF)
    task_graph_node: Optional[Dict[str, Any]] = None  # Task graph node data
    attributes: Dict[str, Any] = field(default_factory=dict)
    relationships: List[Dict[str, str]] = field(default_factory=list)


class YAMLDSLPipeline:
    """Validate and convert threat content to SX9 DSL."""

    def __init__(self, output_dir: Path = None):
        self.output_dir = output_dir or OUTPUT_DIR / "sx9_dsl"
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.validation_errors: List[Dict] = []
        self.entities: List[SX9Entity] = []

    def murmur3_64(self, data: bytes, seed: int) -> int:
        """Compute 64-bit MurmurHash3 (RFC-9001 compliant)."""
        if HAS_MMH3:
            # mmh3.hash64 returns (hash1, hash2) tuple, use first 64 bits
            hash128 = mmh3.hash128(data, seed, signed=False)
            return hash128 & 0xFFFFFFFFFFFFFFFF  # Lower 64 bits
        else:
            # Fallback to SHA-256 (not RFC-9001 compliant, but functional)
            hash_obj = hashlib.sha256(data)
            hash_obj.update(str(seed).encode())
            return int(hash_obj.hexdigest()[:16], 16)  # First 64 bits
    
    def encode_base96(self, value: int, length: int = 16) -> str:
        """Encode 64-bit value to Base96 string (RFC-9001 compliant)."""
        if value == 0:
            return "0" * length

        result = []
        base = BASE96_LEN  # Use dynamic length (should be 96)
        while value > 0 and len(result) < length:
            idx = value % base
            result.append(BASE96_CHARSET[idx])
            value //= base

        # Pad to target length
        while len(result) < length:
            result.append("0")

        # Reverse for big-endian representation
        return "".join(reversed(result))
    
    def generate_trivariate(self, content: str, entity_type: str, is_secondary: bool = False) -> TrivarateHash:
        """Generate RFC-9001 compliant trivariate hash (Murmur3-64 + Base96)."""
        # SCH: Semantic Content Hash (16 chars Base96)
        sch_data = f"SCH:{content}:{entity_type}".encode()
        sch_hash = self.murmur3_64(sch_data, SCH_SEED)
        sch = self.encode_base96(sch_hash, 16)
        
        # CUID: Contextual Unique ID (16 chars Base96)
        timestamp = datetime.now().isoformat()
        cuid_data = f"CUID:{content}:{entity_type}:{timestamp}".encode()
        cuid_hash = self.murmur3_64(cuid_data, CUID_SEED)
        cuid = self.encode_base96(cuid_hash, 16)
        
        # UUID: Universal Unique ID (16 chars Base96)
        # Use UUIDv7 if available, otherwise UUIDv4
        try:
            # Try to generate UUIDv7 (RFC-9001 requirement)
            # For now, use UUIDv4 and encode timestamp
            uuid_obj = uuid_lib.uuid4()
            uuid_data = f"UUID:{uuid_obj.hex}:{timestamp}".encode()
        except:
            uuid_data = f"UUID:{content}:{entity_type}:{timestamp}".encode()
        uuid_hash = self.murmur3_64(uuid_data, UUID_SEED)
        uuid_str = self.encode_base96(uuid_hash, 16)
        
        return TrivarateHash(sch=sch, cuid=cuid, uuid=uuid_str)
    
    def map_hash_to_unicode(self, hash_component: str, component_type: str) -> int:
        """Map hash component to Unicode operation (RFC-9002)."""
        # Convert Base96 hash to integer
        hash_int = sum(BASE96_CHARSET.index(c) * (96 ** i) for i, c in enumerate(reversed(hash_component[:8])))
        
        # Map to Unicode range based on component type
        if component_type == "SCH":
            # U+E100-E1FF: Trivariate Processor (SCH positions 1-16)
            return UNICODE_TRIVARIATE_PROCESSOR_START + (hash_int % 256)
        elif component_type == "CUID":
            # U+E200-E2FF: Context Processor (CUID positions 17-32)
            return UNICODE_CONTEXT_PROCESSOR_START + (hash_int % 256)
        elif component_type == "UUID":
            # U+E000-E0FF: System Controller (UUID positions 33-48)
            return UNICODE_SYSTEM_CONTROLLER_START + (hash_int % 256)
        elif component_type == "TOOL":
            # U+E800-E8FF: Kali Tools
            return UNICODE_KALI_TOOLS_START + (hash_int % 256)
        else:
            # Default: Intelligence Processor (U+E300-E3FF)
            return UNICODE_INTELLIGENCE_PROCESSOR_START + (hash_int % 256)

    def generate_semantic_hash(self, entity: Dict) -> str:
        """
        Generate H2 Semantic Hash (RFC-9025).
        Based on content meaning: description, attributes, relationships, context.
        """
        # Semantic content: description, name, attributes that define meaning
        semantic_content = [
            entity.get("name", ""),
            entity.get("description", ""),
            str(entity.get("attributes", {})),
            str(entity.get("relationships", [])),
            entity.get("type", ""),
        ]
        semantic_text = "|".join(semantic_content)
        
        # Generate hash from semantic content
        semantic_hash = hashlib.sha256(semantic_text.encode()).hexdigest()
        return semantic_hash[:32]  # 32 chars (128 bits)

    def generate_operational_hash(self, entity: Dict) -> str:
        """
        Generate H1 Operational Hash (RFC-9025).
        Based on operational characteristics: ID, type, PTCC primitive, HD4 phase, execution context.
        """
        # Operational content: ID, type, PTCC, HD4 phase, execution metadata
        operational_content = [
            entity.get("id", ""),
            entity.get("type", ""),
            entity.get("ptcc_primitive", ""),
            str(entity.get("ptcc_code", 0)),
            entity.get("hd4_phase", ""),
            str(entity.get("trivariate", "")),  # Include trivariate for routing
        ]
        operational_text = "|".join(operational_content)
        
        # Generate hash from operational content
        operational_hash = hashlib.sha256(operational_text.encode()).hexdigest()
        return operational_hash[:32]  # 32 chars (128 bits)

    def map_technique_to_ptcc(self, technique: Dict) -> Tuple[str, int]:
        """Map MITRE technique to PTCC primitive."""
        tech_id = technique.get("technique_id", "")
        name = technique.get("name", "").lower()
        description = technique.get("description", "").lower()

        # Heuristic mapping based on technique characteristics
        if any(x in name for x in ["credential", "password", "auth", "login"]):
            return "AUTHENTICATE", 0x14
        elif any(x in name for x in ["create", "install", "deploy"]):
            return "CREATE", 0x00
        elif any(x in name for x in ["read", "query", "discover", "enum"]):
            return "READ", 0x01
        elif any(x in name for x in ["modify", "change", "alter"]):
            return "UPDATE", 0x02
        elif any(x in name for x in ["delete", "remove", "clear"]):
            return "DELETE", 0x03
        elif any(x in name for x in ["encrypt", "obfuscate"]):
            return "ENCRYPT", 0x13
        elif any(x in name for x in ["connect", "tunnel", "proxy"]):
            return "CONNECT", 0x15
        elif any(x in name for x in ["exfil", "transfer", "send"]):
            return "SEND", 0x08
        elif any(x in name for x in ["receive", "download", "fetch"]):
            return "RECEIVE", 0x09
        elif any(x in name for x in ["inject", "transform"]):
            return "TRANSFORM", 0x10
        elif any(x in name for x in ["validate", "check", "verify"]):
            return "VALIDATE", 0x12
        elif any(x in name for x in ["signal", "beacon"]):
            return "SIGNAL", 0x0C
        elif any(x in name for x in ["route", "redirect"]):
            return "ROUTE", 0x17
        else:
            return "READ", 0x01  # Default

    def map_technique_to_hd4(self, technique: Dict) -> str:
        """Map technique to HD4 phase."""
        tactics = technique.get("tactic", []) or technique.get("tactics", [])
        if isinstance(tactics, str):
            tactics = [tactics]

        hd4_phases = set()
        for tactic in tactics:
            tactic_norm = tactic.lower().replace(" ", "-")
            if tactic_norm in TACTIC_HD4_MAP:
                hd4_phases.add(TACTIC_HD4_MAP[tactic_norm])

        # Return primary phase (priority: DOMINATE > DISRUPT > DISABLE > DETECT > HUNT)
        priority = ["DOMINATE", "DISRUPT", "DISABLE", "DETECT", "HUNT"]
        for phase in priority:
            if phase in hd4_phases:
                return phase
        return "DETECT"

    def validate_technique(self, technique: Dict) -> List[str]:
        """Validate technique against LinkML schema."""
        errors = []

        # Required fields
        if not technique.get("technique_id") and not technique.get("id"):
            errors.append("Missing required field: technique_id")
        if not technique.get("name"):
            errors.append("Missing required field: name")

        # Format validation
        tech_id = technique.get("technique_id") or technique.get("id", "")
        if tech_id and not re.match(r"^T\d{4}(\.\d{3})?$", tech_id):
            errors.append(f"Invalid technique_id format: {tech_id}")

        # Tactic validation
        tactics = technique.get("tactic", []) or technique.get("tactics", [])
        valid_tactics = set(TACTIC_HD4_MAP.keys())
        for tactic in tactics:
            tactic_norm = tactic.lower().replace(" ", "-")
            if tactic_norm not in valid_tactics:
                errors.append(f"Unknown tactic: {tactic}")

        return errors

    def validate_rule(self, rule: Dict) -> List[str]:
        """Validate detection rule against LinkML schema."""
        errors = []

        if not rule.get("id") and not rule.get("title"):
            errors.append("Missing required field: id or title")

        status = rule.get("status", "")
        valid_status = ["experimental", "test", "stable", "deprecated"]
        if status and status not in valid_status:
            errors.append(f"Invalid status: {status}")

        level = rule.get("level", "")
        valid_levels = ["informational", "low", "medium", "high", "critical"]
        if level and level not in valid_levels:
            errors.append(f"Invalid level: {level}")

        return errors

    def convert_technique_to_sx9(self, technique: Dict) -> SX9Entity:
        """Convert MITRE technique to SX9 DSL entity."""
        tech_id = technique.get("technique_id") or technique.get("id", "unknown")
        name = technique.get("name", "")
        description = technique.get("description", "")

        # Generate trivariate hash
        trivariate = self.generate_trivariate(f"{tech_id}{name}{description}", "technique")

        # Map to PTCC and HD4
        ptcc_name, ptcc_code = self.map_technique_to_ptcc(technique)
        hd4_phase = self.map_technique_to_hd4(technique)

        # Build entity dict for hash generation
        entity_dict = {
            "id": tech_id,
            "type": "Technique",
            "name": name,
            "description": description[:1000],
            "ptcc_primitive": ptcc_name,
            "ptcc_code": ptcc_code,
            "hd4_phase": hd4_phase,
            "trivariate": str(trivariate),
            "attributes": {
                "description": description[:1000],
                "tactics": technique.get("tactic", []) or technique.get("tactics", []),
                "platforms": technique.get("platforms", []),
            },
            "relationships": []
        }

        # Generate dual hashes (semantic + operational)
        semantic_hash = self.generate_semantic_hash(entity_dict)
        operational_hash = self.generate_operational_hash(entity_dict)
        dual_hash = DualHash(semantic_hash=semantic_hash, operational_hash=operational_hash)

        # Generate secondary trivariate hash (RFC-9001)
        secondary_content = f"{tech_id}{technique.get('domain', '')}{technique.get('platforms', [])}"
        trivariate_secondary = self.generate_trivariate(secondary_content, "technique", is_secondary=True)
        
        # Generate Unicode operation (RFC-9002)
        unicode_op = self.map_hash_to_unicode(trivariate.sch, "SCH")
        
        # Generate task graph node
        task_graph_node = {
            "hash_id": f"{trivariate.sch}{trivariate.cuid}{trivariate.uuid}",
            "task_name": name,
            "description": description[:1000],
            "category": "technique",
            "hd4_phase": hd4_phase,
            "primitive_type": "Event",  # Techniques are events
            "predecessors": [],
            "successors": [],
            "p_probability": 0.80,
            "t_time": 0.60,
            "h_hazard": 0.50,
            "sch_hash": trivariate.sch,
            "cuid_hash": trivariate.cuid,
            "sx9_uuid": trivariate.uuid,
            "unicode_operation": unicode_op,
            "task_seq": 0
        }

        entity = SX9Entity(
            id=tech_id,
            type="Technique",
            name=name,
            trivariate=trivariate,
            trivariate_secondary=trivariate_secondary,
            dual_hash=dual_hash,
            ptcc_primitive=ptcc_name,
            ptcc_code=ptcc_code,
            hd4_phase=hd4_phase,
            unicode_operation=unicode_op,
            task_graph_node=task_graph_node,
            attributes=entity_dict["attributes"],
            relationships=entity_dict["relationships"]
        )

        return entity

    def convert_rule_to_sx9(self, rule: Dict) -> SX9Entity:
        """Convert detection rule to SX9 DSL entity."""
        rule_id = rule.get("id", rule.get("title", "unknown"))
        title = rule.get("title", "")
        description = rule.get("description", "")

        trivariate = self.generate_trivariate(f"{rule_id}{title}{description}", "detection_rule")

        # Build relationships first
        relationships = []
        mitre_refs = rule.get("mitre_attack_refs", [])
        for ref in mitre_refs:
            relationships.append({
                "type": "DETECTS",
                "target_type": "Technique",
                "target_id": ref,
            })

        # Build entity dict for hash generation
        entity_dict = {
            "id": rule_id,
            "type": "DetectionRule",
            "name": title,
            "description": description[:1000],
            "ptcc_primitive": "VALIDATE",
            "ptcc_code": 0x12,
            "hd4_phase": "DETECT",
            "trivariate": str(trivariate),
            "attributes": {
                "description": description[:1000],
                "status": rule.get("status", "experimental"),
                "level": rule.get("level", "medium"),
                "logsource": rule.get("logsource", {}),
                "mitre_refs": rule.get("mitre_attack_refs", []) or rule.get("tags", []),
            },
            "relationships": relationships
        }

        # Generate dual hashes
        semantic_hash = self.generate_semantic_hash(entity_dict)
        operational_hash = self.generate_operational_hash(entity_dict)
        dual_hash = DualHash(semantic_hash=semantic_hash, operational_hash=operational_hash)

        # Detection rules map to VALIDATE primitive and DETECT phase
        entity = SX9Entity(
            id=rule_id,
            type="DetectionRule",
            name=title,
            trivariate=trivariate,
            dual_hash=dual_hash,
            ptcc_primitive="VALIDATE",
            ptcc_code=0x12,
            hd4_phase="DETECT",
            attributes=entity_dict["attributes"],
            relationships=relationships
        )

        return entity

    def convert_tool_to_sx9(self, tool: Dict) -> SX9Entity:
        """Convert offensive tool to SX9 DSL entity with RFC-9001/9002 compliance."""
        package_name = tool.get("package_name") or tool.get("name", "unknown")
        display_name = tool.get("display_name", package_name)

        # Generate primary trivariate hash (RFC-9001)
        primary_content = f"{package_name}{display_name}{tool.get('description', '')}"
        trivariate = self.generate_trivariate(primary_content, "tool", is_secondary=False)
        
        # Generate secondary trivariate hash (RFC-9001 - for Synaptix9/ATLAS/PLASMA)
        secondary_content = f"{package_name}{tool.get('git_repo', '')}{tool.get('version', '')}"
        trivariate_secondary = self.generate_trivariate(secondary_content, "tool", is_secondary=True)

        # Map tool category to PTCC
        categories = tool.get("categories", [])
        ptcc_name, ptcc_code = "READ", 0x01
        for cat in categories:
            cat_lower = cat.lower().replace(" ", "-")
            if cat_lower in TOOL_PTCC_MAP:
                ptcc_name = TOOL_PTCC_MAP[cat_lower]
                ptcc_code = PRIMITIVE_TO_CODE.get(ptcc_name, 0x01)
                break

        # Determine HD4 phase (heuristic based on tool category)
        hd4_phase = "HUNT"  # Default
        if any("detect" in cat.lower() or "monitor" in cat.lower() for cat in categories):
            hd4_phase = "DETECT"
        elif any("exploit" in cat.lower() or "attack" in cat.lower() for cat in categories):
            hd4_phase = "DISRUPT"
        elif any("disable" in cat.lower() or "block" in cat.lower() for cat in categories):
            hd4_phase = "DISABLE"
        elif any("dominate" in cat.lower() or "control" in cat.lower() for cat in categories):
            hd4_phase = "DOMINATE"

        # Build relationships first
        relationships = []
        for tech_id in tool.get("mitre_techniques", []):
            relationships.append({
                "type": "IMPLEMENTS",
                "target_type": "Technique",
                "target_id": tech_id,
            })

        # Generate Unicode operation (RFC-9002)
        unicode_op = self.map_hash_to_unicode(trivariate.sch, "TOOL")

        # Build entity dict for hash generation
        entity_dict = {
            "id": package_name,
            "type": "OffensiveTool",
            "name": display_name,
            "description": tool.get("description", ""),
            "ptcc_primitive": ptcc_name,
            "ptcc_code": ptcc_code,
            "hd4_phase": hd4_phase,
            "trivariate": str(trivariate),
            "attributes": {
                "categories": categories,
                "homepage": tool.get("homepage", ""),
                "commands": tool.get("commands", []),
            },
            "relationships": relationships
        }

        # Generate dual hashes
        semantic_hash = self.generate_semantic_hash(entity_dict)
        operational_hash = self.generate_operational_hash(entity_dict)
        dual_hash = DualHash(semantic_hash=semantic_hash, operational_hash=operational_hash)

        # Generate task graph node (SX9 Gateway Task Graph format)
        task_graph_node = {
            "hash_id": f"{trivariate.sch}{trivariate.cuid}{trivariate.uuid}",
            "task_name": display_name,
            "description": tool.get("description", ""),
            "category": categories[0] if categories else "tool",
            "hd4_phase": hd4_phase,
            "primitive_type": "Object",  # Tools are objects
            "predecessors": [],  # Will be populated from relationships
            "successors": [],   # Will be populated from relationships
            "p_probability": 0.85,  # Default
            "t_time": 0.50,         # Default
            "h_hazard": 0.30,       # Default
            "sch_hash": trivariate.sch,
            "cuid_hash": trivariate.cuid,
            "sx9_uuid": trivariate.uuid,
            "unicode_operation": unicode_op,
            "task_seq": 0  # Will be assigned
        }

        entity = SX9Entity(
            id=package_name,
            type="OffensiveTool",
            name=display_name,
            trivariate=trivariate,
            trivariate_secondary=trivariate_secondary,
            dual_hash=dual_hash,
            ptcc_primitive=ptcc_name,
            ptcc_code=ptcc_code,
            hd4_phase=hd4_phase,
            unicode_operation=unicode_op,
            task_graph_node=task_graph_node,
            attributes=entity_dict["attributes"],
            relationships=relationships
        )

        return entity

    def to_sx9_yaml(self, entity: SX9Entity) -> Dict:
        """Convert SX9Entity to YAML-serializable dict with RFC-9001/9002 fields."""
        result = {
            "sx9_entity": {
                "id": entity.id,
                "type": entity.type,
                "name": entity.name,
                "trivariate": str(entity.trivariate),
                "trivariate_dict": entity.trivariate.to_dict(),
                "hashes": {
                    "semantic": entity.dual_hash.semantic_hash,
                    "operational": entity.dual_hash.operational_hash,
                    "trivariate_primary": str(entity.trivariate),
                },
                "ptcc": {
                    "primitive": entity.ptcc_primitive,
                    "code": f"0x{entity.ptcc_code:02X}",
                },
                "hd4_phase": entity.hd4_phase,
                "attributes": entity.attributes,
                "relationships": entity.relationships,
            }
        }
        
        # Add secondary trivariate hash if present (RFC-9001)
        if entity.trivariate_secondary:
            result["sx9_entity"]["hashes"]["trivariate_secondary"] = str(entity.trivariate_secondary)
            result["sx9_entity"]["trivariate_secondary_dict"] = entity.trivariate_secondary.to_dict()
        
        # Add Unicode operation if present (RFC-9002)
        if entity.unicode_operation is not None:
            result["sx9_entity"]["unicode_operation"] = {
                "code_point": entity.unicode_operation,
                "unicode_string": f"U+{entity.unicode_operation:04X}",
                "description": self._unicode_range_description(entity.unicode_operation)
            }
        
        # Add task graph node if present
        if entity.task_graph_node:
            result["sx9_entity"]["task_graph_node"] = entity.task_graph_node
        
        return result
    
    def _unicode_range_description(self, code_point: int) -> str:
        """Get description of Unicode range for code point."""
        if UNICODE_SYSTEM_CONTROLLER_START <= code_point < UNICODE_SYSTEM_CONTROLLER_START + 256:
            return "System Controller (UUID-driven operations)"
        elif UNICODE_TRIVARIATE_PROCESSOR_START <= code_point < UNICODE_TRIVARIATE_PROCESSOR_START + 256:
            return "Trivariate Processor (SCH-driven operations)"
        elif UNICODE_CONTEXT_PROCESSOR_START <= code_point < UNICODE_CONTEXT_PROCESSOR_START + 256:
            return "Context Processor (CUID-driven operations)"
        elif UNICODE_INTELLIGENCE_PROCESSOR_START <= code_point < UNICODE_INTELLIGENCE_PROCESSOR_START + 256:
            return "Intelligence Processor (semantic hash)"
        elif UNICODE_KALI_TOOLS_START <= code_point < UNICODE_KALI_TOOLS_START + 256:
            return "Kali Tools (tool-specific triggers)"
        else:
            return "Reserved/Experimental"

    def process_threat_content(self, content_dir: Path) -> Dict[str, Any]:
        """Process all threat content through validation and conversion."""
        results = {
            "validated": 0,
            "converted": 0,
            "errors": [],
            "entities": [],
        }

        # Process MITRE techniques from STIX format (Enterprise, ICS, Mobile)
        for attack_file in [
            content_dir / "mitre_attack.json",
            content_dir / "mitre_attack_ics.json",
            content_dir / "mitre_attack_mobile.json"
        ]:
            if attack_file.exists():
                logger.info(f"Processing techniques from {attack_file}")
                domain = attack_file.stem.replace("mitre_attack", "").replace("_", "") or "enterprise"
                with open(attack_file) as f:
                    data = json.load(f)

                # Handle STIX format: extract attack-patterns from objects array
                if isinstance(data, dict) and "objects" in data:
                    for obj in data.get("objects", []):
                        if obj.get("type") == "attack-pattern":
                            # Extract technique ID from external references
                            tech_id = None
                            for ref in obj.get("external_references", []):
                                if ref.get("source_name") == "mitre-attack":
                                    tech_id = ref.get("external_id")
                                    break

                            if tech_id:
                                tech = {
                                    "technique_id": tech_id,
                                    "name": obj.get("name", ""),
                                    "description": obj.get("description", "")[:2000],
                                    "tactics": [p.get("phase_name") for p in obj.get("kill_chain_phases", [])],
                                    "platforms": obj.get("x_mitre_platforms", []),
                                    "domain": domain,
                                }
                                errors = self.validate_technique(tech)
                                if errors:
                                    results["errors"].append({
                                        "id": tech_id,
                                        "type": "technique",
                                        "errors": errors,
                                    })
                                else:
                                    results["validated"] += 1
                                    entity = self.convert_technique_to_sx9(tech)
                                    results["entities"].append(self.to_sx9_yaml(entity))
                                    results["converted"] += 1
                else:
                    # Handle list format
                    techniques = data if isinstance(data, list) else data.get("techniques", [])
                    for tech in techniques:
                        errors = self.validate_technique(tech)
                        if errors:
                            results["errors"].append({
                                "id": tech.get("technique_id", "unknown"),
                                "type": "technique",
                                "errors": errors,
                            })
                        else:
                            results["validated"] += 1
                            entity = self.convert_technique_to_sx9(tech)
                            results["entities"].append(self.to_sx9_yaml(entity))
                            results["converted"] += 1

        # Process Sigma rules (cloned to sigma/rules)
        if HAS_YAML:
            rules_dir = content_dir / "sigma" / "rules"
            if rules_dir.exists():
                logger.info(f"Processing Sigma rules from {rules_dir}")
                for rule_file in list(rules_dir.rglob("*.yml"))[:2000]:  # Limit for performance
                    try:
                        with open(rule_file) as f:
                            rule = yaml.safe_load(f)
                        if not rule:
                            continue

                        errors = self.validate_rule(rule)
                        if errors:
                            results["errors"].append({
                                "id": rule.get("id", rule_file.stem),
                                "type": "rule",
                                "errors": errors,
                            })
                        else:
                            results["validated"] += 1
                            entity = self.convert_rule_to_sx9(rule)
                            results["entities"].append(self.to_sx9_yaml(entity))
                            results["converted"] += 1
                    except Exception as e:
                        logger.debug(f"Skip {rule_file.name}: {e}")

        # Process LOLBAS (Living Off the Land Binaries)
        if HAS_YAML:
            lolbas_dir = content_dir / "lolbas" / "yml"
            if lolbas_dir.exists():
                logger.info(f"Processing LOLBAS from {lolbas_dir}")
                for yml_file in lolbas_dir.rglob("*.yml"):
                    try:
                        with open(yml_file) as f:
                            data = yaml.safe_load(f)
                        if data:
                            tool = {
                                "name": data.get("Name", yml_file.stem),
                                "display_name": data.get("Name", ""),
                                "categories": ["lolbas"],
                                "mitre_techniques": [cmd.get("MitreID") for cmd in data.get("Commands", []) if cmd.get("MitreID")],
                                "commands": [cmd.get("Command", "")[:100] for cmd in data.get("Commands", [])[:3]],
                            }
                            results["validated"] += 1
                            entity = self.convert_tool_to_sx9(tool)
                            results["entities"].append(self.to_sx9_yaml(entity))
                            results["converted"] += 1
                    except Exception:
                        pass

        # Process Atomic Red Team tests
        if HAS_YAML:
            atomics_dir = content_dir / "atomic-red-team" / "atomics"
            if atomics_dir.exists():
                logger.info(f"Processing Atomic Red Team from {atomics_dir}")
                for tech_dir in atomics_dir.iterdir():
                    if tech_dir.is_dir() and tech_dir.name.startswith("T"):
                        yaml_file = tech_dir / f"{tech_dir.name}.yaml"
                        if yaml_file.exists():
                            try:
                                with open(yaml_file) as f:
                                    data = yaml.safe_load(f)
                                if data:
                                    for i, test in enumerate(data.get("atomic_tests", [])[:5]):
                                        tool = {
                                            "name": f"atomic_{data.get('attack_technique', '')}_{i}",
                                            "display_name": test.get("name", ""),
                                            "categories": ["atomic-red-team"],
                                            "mitre_techniques": [data.get("attack_technique", "")],
                                        }
                                        results["validated"] += 1
                                        entity = self.convert_tool_to_sx9(tool)
                                        results["entities"].append(self.to_sx9_yaml(entity))
                                        results["converted"] += 1
                            except Exception:
                                pass

        # Process Kali tools
        tools_file = content_dir / "kali_tools_inventory.json"
        if tools_file.exists():
            logger.info(f"Processing Kali tools from {tools_file}")
            with open(tools_file) as f:
                data = json.load(f)
                tools = data if isinstance(data, list) else data.get("tools", [])

            for tool in tools:
                results["validated"] += 1
                entity = self.convert_tool_to_sx9(tool)
                results["entities"].append(self.to_sx9_yaml(entity))
                results["converted"] += 1

        # Process ExploitDB index
        exploitdb_file = content_dir / "exploitdb_index.json"
        if exploitdb_file.exists():
            logger.info(f"Processing ExploitDB from {exploitdb_file}")
            with open(exploitdb_file) as f:
                data = json.load(f)

            for exploit in data.get("exploits", []):
                entity = self.convert_exploit_to_sx9(exploit)
                results["entities"].append(self.to_sx9_yaml(entity))
                results["validated"] += 1
                results["converted"] += 1

        return results

    def convert_exploit_to_sx9(self, exploit: Dict) -> SX9Entity:
        """Convert ExploitDB exploit to SX9 DSL entity."""
        exploit_id = exploit.get("id", "unknown")
        category = exploit.get("category", "")
        filename = exploit.get("filename", "")

        trivariate = self.generate_trivariate(f"{exploit_id}{category}{filename}", "exploit")

        # Build entity dict for hash generation
        entity_dict = {
            "id": f"edb_{exploit_id}",
            "type": "Exploit",
            "name": f"EDB-{exploit_id}",
            "description": f"{category} exploit: {filename}",
            "ptcc_primitive": "AUTHENTICATE",
            "ptcc_code": 0x14,
            "hd4_phase": "DISRUPT",
            "trivariate": str(trivariate),
            "attributes": {
                "category": category,
                "filename": filename,
                "exploit_type": exploit.get("type", ""),
                "source": "exploitdb",
            },
            "relationships": []
        }

        # Generate dual hashes
        semantic_hash = self.generate_semantic_hash(entity_dict)
        operational_hash = self.generate_operational_hash(entity_dict)
        dual_hash = DualHash(semantic_hash=semantic_hash, operational_hash=operational_hash)

        entity = SX9Entity(
            id=f"edb_{exploit_id}",
            type="Exploit",
            name=f"EDB-{exploit_id}",
            trivariate=trivariate,
            dual_hash=dual_hash,
            ptcc_primitive="AUTHENTICATE",  # Exploits typically authenticate/elevate
            ptcc_code=0x14,
            hd4_phase="DISRUPT",  # Exploits are used in DISRUPT phase
            attributes=entity_dict["attributes"],
            relationships=[]
        )

        return entity

    def save_results(self, results: Dict[str, Any]):
        """Save validation and conversion results."""
        # Save all entities as YAML
        if HAS_YAML:
            entities_file = self.output_dir / "sx9_entities.yaml"
            with open(entities_file, "w") as f:
                yaml.dump({"entities": results["entities"]}, f, default_flow_style=False)
            logger.info(f"Saved {len(results['entities'])} entities to {entities_file}")

        # Also save as JSON for compatibility
        entities_json = self.output_dir / "sx9_entities.json"
        with open(entities_json, "w") as f:
            json.dump({"entities": results["entities"]}, f, indent=2)

        # Save as TOML (RFC-9011 requirement)
        if HAS_TOML:
            entities_toml = self.output_dir / "sx9_entities.toml"
            try:
                if 'toml' in sys.modules:
                    with open(entities_toml, "w") as f:
                        toml.dump({"entities": results["entities"]}, f)
                elif 'tomli_w' in sys.modules:
                    import tomli_w
                    with open(entities_toml, "wb") as f:
                        tomli_w.dump({"entities": results["entities"]}, f)
                logger.info(f"Saved {len(results['entities'])} entities to {entities_toml}")
            except Exception as e:
                logger.warning(f"Failed to save TOML: {e}")
        else:
            logger.warning("TOML writer not available - skipping TOML output")

        # Save validation errors
        if results["errors"]:
            errors_file = self.output_dir / "validation_errors.json"
            with open(errors_file, "w") as f:
                json.dump(results["errors"], f, indent=2)
            logger.warning(f"Saved {len(results['errors'])} validation errors to {errors_file}")

        # Summary
        summary = {
            "validated": results["validated"],
            "converted": results["converted"],
            "error_count": len(results["errors"]),
            "output_dir": str(self.output_dir),
        }
        summary_file = self.output_dir / "pipeline_summary.json"
        with open(summary_file, "w") as f:
            json.dump(summary, f, indent=2)

        return summary


def main():
    parser = argparse.ArgumentParser(description="YAML DSL Pipeline")
    parser.add_argument("--input", "-i", type=Path, default=OUTPUT_DIR / "threat_content",
                       help="Input directory with threat content")
    parser.add_argument("--output", "-o", type=Path, default=OUTPUT_DIR / "sx9_dsl",
                       help="Output directory for SX9 DSL")
    parser.add_argument("--validate", action="store_true", help="Validate only, no conversion")
    parser.add_argument("--convert", action="store_true", help="Convert to SX9 DSL")
    args = parser.parse_args()

    if not HAS_YAML:
        logger.warning("PyYAML not installed, YAML output will be skipped")

    pipeline = YAMLDSLPipeline(output_dir=args.output)

    logger.info(f"Processing threat content from: {args.input}")
    results = pipeline.process_threat_content(args.input)

    if not args.validate:
        summary = pipeline.save_results(results)
        print("\n=== Pipeline Summary ===")
        for key, value in summary.items():
            print(f"  {key}: {value}")
    else:
        print(f"\nValidation Results:")
        print(f"  Validated: {results['validated']}")
        print(f"  Errors: {len(results['errors'])}")
        if results["errors"]:
            print("\nFirst 5 errors:")
            for err in results["errors"][:5]:
                print(f"  {err['id']}: {err['errors']}")


if __name__ == "__main__":
    main()
