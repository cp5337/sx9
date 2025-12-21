#!/usr/bin/env python3
"""
RFC-9005 Entity Converter
=========================

Converts normalized threat intelligence (tools, techniques, mappings)
into RFC-9005 unified entities format for Neon PostgreSQL.

Input: tools.json, techniques.json, tool_technique_map.json, playbooks.json
Output: entities.json, relationships.json, neon_seed.sql

Usage:
    python3 rfc9005_converter.py --input ./normalized --output ./neon_ready
"""

import os
import json
import uuid
import argparse
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field, asdict

# ═══════════════════════════════════════════════════════════════════════════
# RFC-9005 Entity Format
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class Entity:
    """RFC-9005 Unified Entity."""
    id: str                             # UUID
    trivariate_hash: str                # SCH-CUID-UUID (with dashes)
    sch_hash: str                       # Murmur3-128 (24 chars)
    cuid: str                           # Base96 (16 chars)
    unicode_address: str                # U+E000-EFFF
    unicode_class: str                  # A-H
    operation_class: str                # intelligence, defensive, offensive, administrative
    escalation_tier: int                # 1-7
    name: str
    entity_type: str                    # tool, technique, node, playbook, etc.
    description: str
    source: Optional[str] = None
    source_id: Optional[str] = None
    hd4_phase: Optional[str] = None
    ptcc_primitive: Optional[int] = None
    capabilities: Dict = field(default_factory=dict)
    limitations: Dict = field(default_factory=dict)
    tactical: Dict = field(default_factory=lambda: {"ttps": [], "toolchain_refs": [], "attack_vectors": []})
    relationships: Dict = field(default_factory=lambda: {"dependencies": [], "provides_to": [], "coordinates_with": [], "escalates_to": []})
    type_extensions: Dict = field(default_factory=dict)
    current_state: str = "active"
    sled_key: Optional[str] = None
    slot_graph_id: Optional[str] = None
    hash_slot: Optional[int] = None
    created_at: str = field(default_factory=lambda: datetime.utcnow().isoformat())
    updated_at: str = field(default_factory=lambda: datetime.utcnow().isoformat())


@dataclass
class Relationship:
    """RFC-9005 Relationship."""
    id: str
    source_entity_id: str
    target_entity_id: str
    relationship_type: str              # covers_technique, exploits_technique, etc.
    mapping_source: Optional[str] = None
    confidence: float = 1.0
    unicode_linkage: Optional[str] = None
    neural_weight: float = 1.0
    escalation_tier: int = 1
    created_at: str = field(default_factory=lambda: datetime.utcnow().isoformat())


# ═══════════════════════════════════════════════════════════════════════════
# Converter Class
# ═══════════════════════════════════════════════════════════════════════════

class RFC9005Converter:
    """Converts normalized threat intel to RFC-9005 format."""
    
    def __init__(self, input_dir: str, output_dir: str):
        self.input_dir = Path(input_dir)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        self.entities: Dict[str, Entity] = {}
        self.relationships: List[Relationship] = []
        
        # ID mappings (old_id → new_uuid)
        self.tool_id_map: Dict[str, str] = {}
        self.technique_id_map: Dict[str, str] = {}
        self.playbook_id_map: Dict[str, str] = {}
        
    def run(self):
        """Run conversion pipeline."""
        print("═" * 60)
        print("RFC-9005 Entity Converter")
        print("═" * 60)
        
        # Load source files
        tools = self._load_json("tools.json")
        techniques = self._load_json("techniques.json")
        mappings = self._load_json("tool_technique_map.json")
        playbooks = self._load_json("playbooks.json")
        
        # Convert to entities
        self._convert_techniques(techniques)
        self._convert_tools(tools)
        self._convert_playbooks(playbooks)
        
        # Convert mappings to relationships
        self._convert_mappings(mappings)
        
        # Output
        self._output_all()
        
        print("═" * 60)
        print(f"✅ Conversion complete!")
        print(f"   Entities: {len(self.entities)}")
        print(f"   Relationships: {len(self.relationships)}")
        print("═" * 60)
    
    def _load_json(self, filename: str) -> List[Dict]:
        """Load JSON file."""
        filepath = self.input_dir / filename
        if not filepath.exists():
            print(f"   ⚠️  {filename} not found")
            return []
        with open(filepath) as f:
            data = json.load(f)
            print(f"   📥 Loaded {len(data)} from {filename}")
            return data
    
    # ─────────────────────────────────────────────────────────────────────────
    # Converters
    # ─────────────────────────────────────────────────────────────────────────
    
    def _convert_techniques(self, techniques: List[Dict]):
        """Convert techniques to entities."""
        for tech in techniques:
            entity_id = str(uuid.uuid4())
            self.technique_id_map[tech.get("id", "")] = entity_id
            
            # Format trivariate hash with dashes (RFC-9005 style)
            h1 = tech.get("h1_operational", "")
            if h1 and len(h1) == 48:
                trivariate = f"{h1[:16]}-{h1[16:32]}-{h1[32:48]}"
            else:
                trivariate = h1
            
            entity = Entity(
                id=entity_id,
                trivariate_hash=trivariate,
                sch_hash=tech.get("h1_sch", h1[:24] if h1 else ""),
                cuid=tech.get("h1_cuid", h1[16:32] if len(h1) >= 32 else ""),
                unicode_address=self._format_unicode(tech.get("unicode_rune", "E100")),
                unicode_class="C",  # Techniques are Class C (Semantic Routing)
                operation_class="intelligence",
                escalation_tier=1,
                name=tech.get("name", ""),
                entity_type="technique",
                description=tech.get("description", ""),
                source="mitre",
                source_id=tech.get("id", ""),
                type_extensions={
                    "tactic": tech.get("tactic", ""),
                    "detection": tech.get("detection", ""),
                    "platforms": tech.get("platforms", []),
                    "data_sources": tech.get("data_sources", [])
                },
                tactical={
                    "ttps": [tech.get("id", "")],
                    "toolchain_refs": [],
                    "attack_vectors": []
                }
            )
            self.entities[entity_id] = entity
    
    def _convert_tools(self, tools: List[Dict]):
        """Convert tools to entities."""
        for tool in tools:
            entity_id = str(uuid.uuid4())
            tool_id = tool.get("id", "")
            self.tool_id_map[tool_id] = entity_id
            
            # Format trivariate hash
            h1 = tool.get("h1_operational", "")
            if h1 and len(h1) == 48:
                trivariate = f"{h1[:16]}-{h1[16:32]}-{h1[32:48]}"
            else:
                trivariate = h1
            
            # Map source to operation_class
            source = tool.get("source", "")
            op_class = self._get_operation_class(source, tool.get("category", ""))
            
            entity = Entity(
                id=entity_id,
                trivariate_hash=trivariate,
                sch_hash=tool.get("h1_sch", ""),
                cuid=tool.get("h1_cuid", ""),
                unicode_address=self._format_unicode(tool.get("unicode_rune", "E000")),
                unicode_class="A",  # Tools are Class A (Core Components)
                operation_class=op_class,
                escalation_tier=self._get_escalation_tier(tool.get("category", "")),
                name=tool.get("name", ""),
                entity_type="tool",
                description=tool.get("description", ""),
                source=source,
                source_id=tool_id,
                hd4_phase=tool.get("hd4_phase"),
                ptcc_primitive=tool.get("ptcc_primitive"),
                capabilities={
                    "installed": tool.get("installed", False),
                    "success_rate": tool.get("success_rate", 85)
                },
                type_extensions={
                    "command": tool.get("command", ""),
                    "default_args": tool.get("default_args", []),
                    "url": tool.get("url", ""),
                    "category": tool.get("category", "")
                },
                tactical={
                    "ttps": tool.get("techniques", []),
                    "toolchain_refs": [],
                    "attack_vectors": []
                },
                sled_key=f"tool:{tool_id}",
                hash_slot=int(tool.get("h1_sch", "0")[:4], 16) % 16384 if tool.get("h1_sch") else None
            )
            self.entities[entity_id] = entity
    
    def _convert_playbooks(self, playbooks: List[Dict]):
        """Convert playbooks to entities."""
        for pb in playbooks:
            entity_id = str(uuid.uuid4())
            pb_id = pb.get("id", "")
            self.playbook_id_map[pb_id] = entity_id
            
            h1 = pb.get("h1_operational", "")
            if h1 and len(h1) == 48:
                trivariate = f"{h1[:16]}-{h1[16:32]}-{h1[32:48]}"
            else:
                trivariate = h1
            
            # Resolve tool IDs to UUIDs
            tool_uuids = [
                self.tool_id_map.get(tid, tid) 
                for tid in pb.get("tools", [])
            ]
            
            entity = Entity(
                id=entity_id,
                trivariate_hash=trivariate,
                sch_hash=pb.get("h1_sch", ""),
                cuid=pb.get("h1_cuid", ""),
                unicode_address=self._format_unicode(pb.get("unicode_rune", "E200")),
                unicode_class="B",  # Playbooks are Class B (CUID Slot Mapping)
                operation_class="offensive",
                escalation_tier=3,
                name=pb.get("name", ""),
                entity_type="playbook",
                description=pb.get("description", ""),
                source="generated",
                source_id=pb_id,
                hd4_phase=pb.get("hd4_phase"),
                type_extensions={
                    "tool_ids": tool_uuids,
                    "technique_ids": pb.get("techniques", []),
                    "category": pb.get("category", ""),
                    "automation_level": 50
                },
                tactical={
                    "ttps": pb.get("techniques", []),
                    "toolchain_refs": tool_uuids,
                    "attack_vectors": []
                }
            )
            self.entities[entity_id] = entity
    
    def _convert_mappings(self, mappings: List[Dict]):
        """Convert tool→technique mappings to relationships."""
        for m in mappings:
            tool_id = m.get("tool_id", "")
            tech_id = m.get("technique_id", "")
            
            source_uuid = self.tool_id_map.get(tool_id)
            target_uuid = self.technique_id_map.get(tech_id)
            
            if not source_uuid or not target_uuid:
                continue
            
            rel = Relationship(
                id=str(uuid.uuid4()),
                source_entity_id=source_uuid,
                target_entity_id=target_uuid,
                relationship_type="covers_technique",
                mapping_source=m.get("source", ""),
                confidence=m.get("confidence", 1.0)
            )
            self.relationships.append(rel)
    
    # ─────────────────────────────────────────────────────────────────────────
    # Helpers
    # ─────────────────────────────────────────────────────────────────────────
    
    def _format_unicode(self, rune: str) -> str:
        """Format unicode rune as U+XXXX."""
        if rune.startswith("U+"):
            return rune
        return f"U+{rune}"
    
    def _get_operation_class(self, source: str, category: str) -> str:
        """Map source/category to operation class."""
        offensive = {"kali", "atomic", "caldera", "lolbas", "gtfobins", "exploitation", "password"}
        defensive = {"sigma", "yara", "wazuh", "snort", "detection", "forensics"}
        
        if source in offensive or category.lower() in offensive:
            return "offensive"
        elif source in defensive or category.lower() in defensive:
            return "defensive"
        elif category.lower() in {"reconnaissance", "osint"}:
            return "intelligence"
        else:
            return "offensive"  # Default for security tools
    
    def _get_escalation_tier(self, category: str) -> int:
        """Map category to escalation tier."""
        tier_map = {
            "reconnaissance": 1,
            "enumeration": 2,
            "vulnerability": 3,
            "exploitation": 5,
            "password": 4,
            "post-exploitation": 6,
            "privilege-escalation": 5,
            "lateral-movement": 6,
            "persistence": 6,
            "exfiltration": 7
        }
        return tier_map.get(category.lower(), 3)
    
    # ─────────────────────────────────────────────────────────────────────────
    # Output
    # ─────────────────────────────────────────────────────────────────────────
    
    def _output_all(self):
        """Output all formats."""
        print("\n📤 Writing output files...")
        
        # JSON
        entities_list = [asdict(e) for e in self.entities.values()]
        relationships_list = [asdict(r) for r in self.relationships]
        
        with open(self.output_dir / "entities.json", "w") as f:
            json.dump(entities_list, f, indent=2)
        print(f"   ✅ entities.json ({len(entities_list)} records)")
        
        with open(self.output_dir / "relationships.json", "w") as f:
            json.dump(relationships_list, f, indent=2)
        print(f"   ✅ relationships.json ({len(relationships_list)} records)")
        
        # SQL
        self._output_sql(entities_list, relationships_list)
    
    def _output_sql(self, entities: List[Dict], relationships: List[Dict]):
        """Generate Neon-compatible SQL."""
        sql_file = self.output_dir / "neon_seed.sql"
        
        with open(sql_file, "w") as f:
            f.write("-- ═══════════════════════════════════════════════════════════════════════════\n")
            f.write("-- RFC-9005 Entity Seed Data for Neon\n")
            f.write(f"-- Generated: {datetime.utcnow().isoformat()}\n")
            f.write("-- ═══════════════════════════════════════════════════════════════════════════\n\n")
            
            f.write("BEGIN;\n\n")
            
            # Entities
            f.write("-- ═══════════════════════════════════════════════════════════════════════════\n")
            f.write("-- ENTITIES\n")
            f.write("-- ═══════════════════════════════════════════════════════════════════════════\n\n")
            
            for e in entities:
                f.write(self._entity_to_sql(e))
            
            f.write("\n")
            
            # Relationships
            f.write("-- ═══════════════════════════════════════════════════════════════════════════\n")
            f.write("-- RELATIONSHIPS\n")
            f.write("-- ═══════════════════════════════════════════════════════════════════════════\n\n")
            
            for r in relationships:
                f.write(self._relationship_to_sql(r))
            
            f.write("\nCOMMIT;\n")
            
            # Stats
            f.write("\n-- Stats:\n")
            f.write(f"-- Entities: {len(entities)}\n")
            f.write(f"-- Relationships: {len(relationships)}\n")
        
        print(f"   ✅ neon_seed.sql")
    
    def _entity_to_sql(self, e: Dict) -> str:
        """Convert entity to SQL INSERT."""
        return f"""INSERT INTO entities (
    id, trivariate_hash, sch_hash, cuid, unicode_address, unicode_class,
    operation_class, escalation_tier, name, entity_type, description,
    source, source_id, hd4_phase, ptcc_primitive, capabilities, limitations,
    tactical, relationships, type_extensions, current_state, sled_key,
    slot_graph_id, hash_slot, created_at, updated_at
) VALUES (
    '{e['id']}',
    {self._sql_str(e.get('trivariate_hash'))},
    {self._sql_str(e.get('sch_hash'))},
    {self._sql_str(e.get('cuid'))},
    {self._sql_str(e.get('unicode_address'))},
    {self._sql_str(e.get('unicode_class'))},
    {self._sql_enum(e.get('operation_class'))},
    {e.get('escalation_tier', 1)},
    {self._sql_str(e.get('name'))},
    {self._sql_str(e.get('entity_type'))},
    {self._sql_str(e.get('description'))},
    {self._sql_str(e.get('source'))},
    {self._sql_str(e.get('source_id'))},
    {self._sql_str(e.get('hd4_phase'))},
    {e.get('ptcc_primitive') if e.get('ptcc_primitive') is not None else 'NULL'},
    {self._sql_json(e.get('capabilities', {}))},
    {self._sql_json(e.get('limitations', {}))},
    {self._sql_json(e.get('tactical', {}))},
    {self._sql_json(e.get('relationships', {}))},
    {self._sql_json(e.get('type_extensions', {}))},
    '{e.get('current_state', 'active')}'::entity_state_enum,
    {self._sql_str(e.get('sled_key'))},
    {self._sql_str(e.get('slot_graph_id'))},
    {e.get('hash_slot') if e.get('hash_slot') is not None else 'NULL'},
    NOW(),
    NOW()
) ON CONFLICT (trivariate_hash) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    type_extensions = EXCLUDED.type_extensions,
    updated_at = NOW();

"""
    
    def _relationship_to_sql(self, r: Dict) -> str:
        """Convert relationship to SQL INSERT."""
        return f"""INSERT INTO relationships (
    id, source_entity_id, target_entity_id, relationship_type,
    mapping_source, confidence, unicode_linkage, neural_weight,
    escalation_tier, created_at
) VALUES (
    '{r['id']}',
    '{r['source_entity_id']}',
    '{r['target_entity_id']}',
    '{r['relationship_type']}',
    {self._sql_str(r.get('mapping_source'))},
    {r.get('confidence', 1.0)},
    {self._sql_str(r.get('unicode_linkage'))},
    {r.get('neural_weight', 1.0)},
    {r.get('escalation_tier', 1)},
    NOW()
) ON CONFLICT (source_entity_id, target_entity_id, relationship_type) DO NOTHING;

"""
    
    def _sql_str(self, val: Any) -> str:
        """Format value as SQL string."""
        if val is None:
            return "NULL"
        escaped = str(val).replace("'", "''")
        return f"'{escaped}'"
    
    def _sql_enum(self, val: Any) -> str:
        """Format value as SQL enum."""
        if val is None:
            return "NULL"
        return f"'{val}'::operation_class_enum"
    
    def _sql_json(self, val: Any) -> str:
        """Format value as SQL JSONB."""
        if val is None:
            return "'{}'::jsonb"
        j = json.dumps(val).replace("'", "''")
        return f"'{j}'::jsonb"


# ═══════════════════════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════════════════════

def main():
    parser = argparse.ArgumentParser(description="RFC-9005 Entity Converter")
    parser.add_argument("--input", default="./normalized", help="Input directory")
    parser.add_argument("--output", default="./neon_ready", help="Output directory")
    args = parser.parse_args()
    
    converter = RFC9005Converter(args.input, args.output)
    converter.run()


if __name__ == "__main__":
    main()
