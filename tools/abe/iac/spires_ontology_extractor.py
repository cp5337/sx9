#!/usr/bin/env python3
"""
SPIRES Ontology Extractor - RFC-9105 Compliant
===============================================

Creates a unified SX9 ontology from RFC corpus using:
1. LinkML schema conformance
2. Trivariate hash identity (SCH-CUID-UUID)
3. Delta-angle semantic relationships
4. SPIRES zero-shot extraction via Gemini

Palantir-competitive ontology layer for enterprise intelligence.

Usage:
    python spires_ontology_extractor.py --extract     # Extract terms from RFCs
    python spires_ontology_extractor.py --threats     # Extract from threat content
    python spires_ontology_extractor.py --normalize   # Normalize terminology
    python spires_ontology_extractor.py --linkml      # Generate LinkML schema
    python spires_ontology_extractor.py --all         # Full pipeline (RFCs + Threats)
"""

import os
import sys
import json
import hashlib
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Set, Tuple
from datetime import datetime, timezone
from collections import Counter, defaultdict
import re

# Add sx9-conda to path for key loader
sys.path.insert(0, str(Path.home() / "Developer/sx9-conda"))

try:
    from sx9_keys import keys
    HAS_KEYS = True
except ImportError:
    HAS_KEYS = False

try:
    import google.generativeai as genai
    HAS_GENAI = True
except ImportError:
    HAS_GENAI = False

# ============================================================================
# CONFIGURATION
# ============================================================================

BASE_PATH = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging")
RFC_PATH = BASE_PATH / "01-rfc"
OUTPUT_PATH = BASE_PATH / "04-abe-iac/output/ontology"
EXTRACTION_PATH = BASE_PATH / "04-abe-iac/output/real_extraction"
THREAT_CONTENT_PATH = BASE_PATH / "04-abe-iac/node-interview-generator/output/threat_content"

# Core SX9 Ontology Categories (LinkML compatible)
ONTOLOGY_CATEGORIES = {
    # Core SX9
    "primitive": "Low-level computational primitives (P-01 through P-32)",
    "hash": "Hashing algorithms and trivariate components",
    "database": "Data storage systems and graph databases",
    "protocol": "Communication and integration protocols",
    "framework": "Architectural frameworks (HD4, PBIOMR, OODA)",
    "cognitive": "AI/ML and cognitive computing concepts",
    "interface": "User interface and interaction patterns",
    "infrastructure": "Deployment and infrastructure terms",
    "domain": "Business domain and vertical concepts",
    # Threat/Security
    "tactic": "MITRE ATT&CK tactics (14 phases)",
    "technique": "MITRE ATT&CK techniques and sub-techniques",
    "detection": "Detection rules (Sigma, YARA, Wazuh, Nuclei)",
    "mitigation": "D3FEND defensive countermeasures",
    "tool": "Offensive/defensive security tools",
    "actor": "Threat actor groups and campaigns",
    "platform": "Target platforms (Windows, Linux, macOS, ICS)",
}

# Canonical term mappings (normalize variants)
CANONICAL_TERMS = {
    # Hash components
    "sch": "SCH",
    "synaptic convergent hash": "SCH",
    "semantic convergent hash": "SCH",
    "cuid": "CUID",
    "contextual unique identifier": "CUID",
    "uuid": "UUIDv7",
    "uuidv7": "UUIDv7",

    # Core systems
    "surrealdb": "SurrealDB",
    "surreal": "SurrealDB",
    "neo4j": "Neo4j",
    "sledis": "Sledis",
    "sled": "Sledis",

    # Frameworks
    "hd4": "HD4",
    "hunt detect disable disrupt dominate": "HD4",
    "ptcc": "PTCC",
    "physical tactical cognitive continuum": "PTCC",
    "ooda": "OODA",
    "observe orient decide act": "OODA",

    # Cognitive
    "neural mux": "Neural Mux",
    "neuralmux": "Neural Mux",
    "neural multiplexer": "Neural Mux",
    "bernoulli zone": "Bernoulli Zone",
    "bernoulli": "Bernoulli Zone",
    "thalamic filter": "Thalamic Filter",
    "hourglass": "Hourglass Convergence",

    # Infrastructure
    "wasm": "WASM",
    "webassembly": "WASM",
    "grpc": "gRPC",
    "nats": "NATS",
    "nats jetstream": "NATS JetStream",

    # Standards
    "mitre attack": "MITRE ATT&CK",
    "mitre att&ck": "MITRE ATT&CK",
    "att&ck": "MITRE ATT&CK",
}

# ============================================================================
# LINKML SCHEMA DEFINITIONS
# ============================================================================

LINKML_BASE_SCHEMA = """
id: https://w3id.org/sx9/ontology
name: SX9Ontology
title: SX9 Enterprise Intelligence Ontology
description: >-
  Unified ontology for the SX9 platform, extracted via SPIRES pipeline.
  RFC-9105 compliant with trivariate hash identity.

prefixes:
  sx9: https://w3id.org/sx9/
  linkml: https://w3id.org/linkml/
  schema: http://schema.org/
  dcterms: http://purl.org/dc/terms/

default_prefix: sx9
default_range: string

classes:
  OntologyTerm:
    description: A term in the SX9 ontology
    attributes:
      id:
        identifier: true
        description: Trivariate hash identity (SCH component)
      canonical_name:
        required: true
        description: Normalized canonical term
      category:
        range: TermCategory
        required: true
      aliases:
        multivalued: true
        description: Alternative names and spellings
      definition:
        description: Term definition
      rfc_sources:
        multivalued: true
        description: RFCs where term appears
      frequency:
        range: integer
        description: Occurrence count across corpus
      delta_angle:
        range: float
        description: Semantic distance from core ontology
      related_terms:
        multivalued: true
        range: OntologyTerm
        description: Semantically related terms

  TermRelation:
    description: Relationship between ontology terms
    attributes:
      source:
        range: OntologyTerm
        required: true
      target:
        range: OntologyTerm
        required: true
      relation_type:
        range: RelationType
        required: true
      weight:
        range: float
        description: Relationship strength (0-1)
      source_rfc:
        description: RFC where relationship is defined

enums:
  TermCategory:
    permissible_values:
{category_values}

  RelationType:
    permissible_values:
      is_a:
        description: Hierarchical inheritance
      part_of:
        description: Compositional relationship
      uses:
        description: Dependency relationship
      implements:
        description: Implementation relationship
      related_to:
        description: General semantic relationship
      supersedes:
        description: Version/replacement relationship
"""

# ============================================================================
# TRIVARIATE HASH GENERATION (RFC-9001 COMPLIANT)
# ============================================================================

def murmur3_64(data: bytes, seed: int = 0) -> int:
    """Simplified Murmur3-64 hash for term identity."""
    h = seed
    for byte in data:
        h ^= byte
        h = (h * 0x5bd1e995) & 0xFFFFFFFFFFFFFFFF
        h ^= (h >> 47)
    return h

def generate_sch(term: str, category: str) -> str:
    """Generate SCH (Synaptic Convergent Hash) for term."""
    # Combine term + category for unique identity
    data = f"{term.lower()}:{category}".encode('utf-8')
    hash_val = murmur3_64(data, seed=0x9001)  # RFC-9001 seed
    # Base96 encode (simplified - just hex for now)
    return f"SCH{hash_val:016x}"[:20]

def calculate_delta_angle(term1: str, term2: str) -> float:
    """Calculate semantic delta-angle between terms."""
    # Simple Jaccard-based angle (0 = identical, 1 = unrelated)
    set1 = set(term1.lower().split())
    set2 = set(term2.lower().split())

    if not set1 or not set2:
        return 1.0

    intersection = len(set1 & set2)
    union = len(set1 | set2)

    return 1.0 - (intersection / union) if union > 0 else 1.0

# ============================================================================
# ONTOLOGY EXTRACTION
# ============================================================================

@dataclass
class OntologyTerm:
    """Extracted ontology term."""
    canonical_name: str
    category: str
    aliases: List[str] = field(default_factory=list)
    definition: str = ""
    rfc_sources: List[str] = field(default_factory=list)
    frequency: int = 0
    sch_id: str = ""
    delta_angle: float = 0.0
    related_terms: List[str] = field(default_factory=list)

    def __post_init__(self):
        if not self.sch_id:
            self.sch_id = generate_sch(self.canonical_name, self.category)

@dataclass
class OntologyGraph:
    """Full ontology graph."""
    terms: Dict[str, OntologyTerm] = field(default_factory=dict)
    relations: List[Dict] = field(default_factory=list)
    metadata: Dict = field(default_factory=dict)

    def add_term(self, term: OntologyTerm):
        """Add or merge term."""
        if term.canonical_name in self.terms:
            existing = self.terms[term.canonical_name]
            existing.frequency += term.frequency
            existing.aliases = list(set(existing.aliases + term.aliases))
            existing.rfc_sources = list(set(existing.rfc_sources + term.rfc_sources))
        else:
            self.terms[term.canonical_name] = term

    def add_relation(self, source: str, target: str, rel_type: str, weight: float = 1.0):
        """Add relationship between terms (deduplicates)."""
        # Check if relation already exists
        rel_key = (source, target, rel_type)
        for existing_rel in self.relations:
            if (existing_rel["source"] == source and 
                existing_rel["target"] == target and 
                existing_rel["relation_type"] == rel_type):
                # Update weight if new weight is higher
                if weight > existing_rel["weight"]:
                    existing_rel["weight"] = weight
                return
        
        # Add new relation
        self.relations.append({
            "source": source,
            "target": target,
            "relation_type": rel_type,
            "weight": weight,
            "delta_angle": calculate_delta_angle(source, target)
        })
    
    def load_from_json(self, json_path: Path) -> int:
        """Load existing ontology from JSON file and merge with current graph."""
        if not json_path.exists():
            return 0
        
        try:
            with open(json_path) as f:
                data = json.load(f)
            
            loaded_count = 0
            
            # Load terms
            for term_data in data.get("terms", []):
                # Reconstruct OntologyTerm from JSON
                term = OntologyTerm(
                    canonical_name=term_data.get("canonical_name", ""),
                    category=term_data.get("category", "domain"),
                    aliases=term_data.get("aliases", []),
                    definition=term_data.get("definition", ""),
                    rfc_sources=term_data.get("rfc_sources", []),
                    frequency=term_data.get("frequency", 1),
                    delta_angle=term_data.get("delta_angle", 0.0)
                )
                # Use add_term which handles merging
                self.add_term(term)
                loaded_count += 1
            
            # Load relations (add_term will deduplicate)
            for rel_data in data.get("relations", []):
                self.add_relation(
                    rel_data.get("source", ""),
                    rel_data.get("target", ""),
                    rel_data.get("relation_type", "RELATED"),
                    rel_data.get("weight", 1.0)
                )
            
            return loaded_count
        except Exception as e:
            print(f"  ⚠️  Error loading existing ontology: {e}")
            return 0

class SPIRESExtractor:
    """SPIRES-based ontology extractor."""

    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key
        self.model = None

        if HAS_GENAI:
            if not self.api_key and HAS_KEYS:
                self.api_key = keys.google_gemini
            if not self.api_key:
                self.api_key = os.environ.get("GOOGLE_API_KEY")

            if self.api_key:
                genai.configure(api_key=self.api_key)
                self.model = genai.GenerativeModel('gemini-2.0-flash')

    def normalize_term(self, term: str) -> str:
        """Normalize term to canonical form."""
        term_lower = term.lower().strip()
        return CANONICAL_TERMS.get(term_lower, term)

    def categorize_term(self, term: str, context: str = "") -> str:
        """Categorize term into ontology category."""
        term_lower = term.lower()

        # Pattern-based categorization
        if re.match(r'p-\d{2}|primitive', term_lower):
            return "primitive"
        if any(x in term_lower for x in ["hash", "sch", "cuid", "uuid", "murmur", "trivariate"]):
            return "hash"
        if any(x in term_lower for x in ["db", "database", "surreal", "neo4j", "sled", "graph"]):
            return "database"
        if any(x in term_lower for x in ["grpc", "nats", "http", "websocket", "protocol", "mcp"]):
            return "protocol"
        if any(x in term_lower for x in ["hd4", "ooda", "ptcc", "framework", "architecture"]):
            return "framework"
        if any(x in term_lower for x in ["neural", "cognitive", "ai", "ml", "bernoulli", "thalamic", "gnn"]):
            return "cognitive"
        if any(x in term_lower for x in ["threat", "attack", "mitre", "security", "sigma", "yara"]):
            return "security"
        if any(x in term_lower for x in ["ui", "interface", "gis", "cesium", "dashboard"]):
            return "interface"
        if any(x in term_lower for x in ["docker", "k8s", "kubernetes", "wasm", "cloud"]):
            return "infrastructure"

        return "domain"

    def extract_from_extractions(self) -> OntologyGraph:
        """Extract ontology from existing RFC extractions."""
        graph = OntologyGraph()
        graph.metadata = {
            "extracted_at": datetime.now(timezone.utc).isoformat(),
            "source": "RFC extractions via SPIRES",
            "version": "1.0"
        }

        # Load all extraction JSONs
        extraction_files = list(EXTRACTION_PATH.glob("*_extraction.json"))
        print(f"Processing {len(extraction_files)} extraction files...")

        for ext_file in extraction_files:
            try:
                with open(ext_file) as f:
                    data = json.load(f)

                rfc_id = data.get("rfc_id", ext_file.stem)

                # Extract technical terms
                for term in data.get("technical_terms", []):
                    canonical = self.normalize_term(term)
                    category = self.categorize_term(term)

                    ont_term = OntologyTerm(
                        canonical_name=canonical,
                        category=category,
                        aliases=[term] if term.lower() != canonical.lower() else [],
                        rfc_sources=[rfc_id],
                        frequency=1
                    )
                    graph.add_term(ont_term)

                # Extract key concepts
                for concept in data.get("key_concepts", []):
                    canonical = self.normalize_term(concept)
                    category = self.categorize_term(concept)

                    ont_term = OntologyTerm(
                        canonical_name=canonical,
                        category=category,
                        aliases=[concept] if concept.lower() != canonical.lower() else [],
                        rfc_sources=[rfc_id],
                        frequency=1
                    )
                    graph.add_term(ont_term)

                # Extract dependencies as relations
                for dep in data.get("dependencies", []):
                    if dep in graph.terms:
                        graph.add_relation(rfc_id, dep, "uses")

            except Exception as e:
                print(f"Error processing {ext_file}: {e}")

        return graph

    async def enrich_with_gemini(self, graph: OntologyGraph) -> OntologyGraph:
        """Use Gemini to enrich ontology with definitions."""
        if not self.model:
            print("Gemini not configured, skipping enrichment")
            return graph

        # Get top 50 terms by frequency
        top_terms = sorted(
            graph.terms.values(),
            key=lambda t: t.frequency,
            reverse=True
        )[:50]

        term_list = [t.canonical_name for t in top_terms]

        prompt = f"""You are the SX9 Ontology Engineer. Define these terms precisely for our enterprise intelligence platform:

Terms: {json.dumps(term_list)}

For each term, provide:
1. A precise 1-2 sentence definition
2. 2-3 related terms from the list
3. Whether it's a core term (essential to SX9) or supporting term

Return as JSON array with objects containing: term, definition, related_terms, is_core

Focus on:
- Precision over verbosity
- How terms relate to trivariate hashing (SCH-CUID-UUID)
- Enterprise intelligence applications
- Competing with Palantir Foundry ontology
"""

        try:
            response = self.model.generate_content(prompt)
            result_text = response.text

            # Extract JSON from response
            json_match = re.search(r'\[[\s\S]*\]', result_text)
            if json_match:
                definitions = json.loads(json_match.group())

                for defn in definitions:
                    term_name = defn.get("term", "")
                    if term_name in graph.terms:
                        graph.terms[term_name].definition = defn.get("definition", "")
                        graph.terms[term_name].related_terms = defn.get("related_terms", [])

                        # Add relations for related terms
                        for related in defn.get("related_terms", []):
                            if related in graph.terms:
                                graph.add_relation(term_name, related, "related_to", 0.8)

        except Exception as e:
            print(f"Gemini enrichment error: {e}")

        return graph

    def extract_from_threats(self) -> OntologyGraph:
        """Extract ontology from threat content (MITRE, Sigma, etc.)."""
        graph = OntologyGraph()
        graph.metadata = {
            "extracted_at": datetime.now(timezone.utc).isoformat(),
            "source": "Threat content via SPIRES",
            "version": "1.0"
        }

        # MITRE ATT&CK
        mitre_file = THREAT_CONTENT_PATH / "mitre_attack.json"
        if mitre_file.exists():
            print(f"  Processing MITRE ATT&CK...")
            try:
                with open(mitre_file) as f:
                    data = json.load(f)

                for obj in data.get("objects", []):
                    if obj.get("type") == "attack-pattern":
                        ext_refs = obj.get("external_references", [])
                        tech_id = next((r.get("external_id") for r in ext_refs
                                       if r.get("source_name") == "mitre-attack"), None)
                        if tech_id:
                            name = obj.get("name", "")
                            tactics = [p.get("phase_name") for p in obj.get("kill_chain_phases", [])]

                            graph.add_term(OntologyTerm(
                                canonical_name=name,
                                category="technique",
                                aliases=[tech_id],
                                definition=obj.get("description", "")[:200],
                                rfc_sources=[tech_id],
                                frequency=1
                            ))

                            # Add tactics
                            for tactic in tactics:
                                tactic_name = tactic.replace("-", " ").title()
                                graph.add_term(OntologyTerm(
                                    canonical_name=tactic_name,
                                    category="tactic",
                                    frequency=1
                                ))
                                graph.add_relation(name, tactic_name, "belongs_to")

            except Exception as e:
                print(f"    MITRE extraction error: {e}")

        # Crosswalk index
        crosswalk_file = THREAT_CONTENT_PATH / "crosswalk_index.json"
        if crosswalk_file.exists():
            print(f"  Processing crosswalk index...")
            try:
                with open(crosswalk_file) as f:
                    data = json.load(f)
                for tech_id, mappings in data.items():
                    if isinstance(mappings, dict):
                        for map_type, items in mappings.items():
                            if isinstance(items, list):
                                for item in items[:3]:
                                    if isinstance(item, str):
                                        graph.add_term(OntologyTerm(
                                            canonical_name=item,
                                            category="detection",
                                            rfc_sources=[tech_id],
                                            frequency=1
                                        ))
            except Exception as e:
                print(f"    Crosswalk error: {e}")

        # Kali tools
        kali_file = THREAT_CONTENT_PATH / "kali_tools_inventory.json"
        if kali_file.exists():
            print(f"  Processing Kali tools...")
            try:
                with open(kali_file) as f:
                    data = json.load(f)
                tools = data if isinstance(data, list) else data.get("tools", [])
                for tool in tools:
                    if isinstance(tool, dict):
                        name = tool.get("name", tool.get("package", ""))
                        if name:
                            graph.add_term(OntologyTerm(
                                canonical_name=name,
                                category="tool",
                                definition=tool.get("description", "")[:200],
                                frequency=1
                            ))
            except Exception as e:
                print(f"    Kali tools error: {e}")

        return graph

# ============================================================================
# OUTPUT GENERATORS
# ============================================================================

def generate_linkml_schema(graph: OntologyGraph) -> str:
    """Generate LinkML schema from ontology graph."""
    # Build category enum values
    category_values = ""
    for cat, desc in ONTOLOGY_CATEGORIES.items():
        category_values += f"      {cat}:\n        description: {desc}\n"

    schema = LINKML_BASE_SCHEMA.format(category_values=category_values)
    return schema

def generate_cypher_export(graph: OntologyGraph) -> str:
    """Generate Cypher queries for Neo4j."""
    queries = [
        "// SX9 Ontology - Cypher Export",
        f"// Generated: {datetime.now(timezone.utc).isoformat()}",
        "// SPIRES extraction via RFC-9105",
        "",
        "// Create indexes",
        "CREATE INDEX IF NOT EXISTS FOR (t:Term) ON (t.sch_id);",
        "CREATE INDEX IF NOT EXISTS FOR (t:Term) ON (t.canonical_name);",
        "CREATE INDEX IF NOT EXISTS FOR (t:Term) ON (t.category);",
        ""
    ]

    # Create term nodes
    for term in graph.terms.values():
        aliases_str = json.dumps(term.aliases)
        sources_str = json.dumps(term.rfc_sources)
        definition = term.definition.replace("'", "\\'").replace('"', '\\"')

        query = f"""
CREATE (t:Term {{
  sch_id: '{term.sch_id}',
  canonical_name: '{term.canonical_name}',
  category: '{term.category}',
  aliases: {aliases_str},
  definition: '{definition}',
  rfc_sources: {sources_str},
  frequency: {term.frequency},
  delta_angle: {term.delta_angle}
}});"""
        queries.append(query.strip())

    # Create relationships
    queries.append("\n// Relationships")
    for rel in graph.relations:
        query = f"""
MATCH (s:Term {{canonical_name: '{rel["source"]}'}}),
      (t:Term {{canonical_name: '{rel["target"]}'}})
MERGE (s)-[r:{rel["relation_type"].upper()} {{
  weight: {rel["weight"]},
  delta_angle: {rel["delta_angle"]}
}}]->(t);"""
        queries.append(query.strip())

    return "\n".join(queries)

def generate_surreal_export(graph: OntologyGraph) -> str:
    """Generate SurrealQL queries for SurrealDB from ontology graph."""
    queries = [
        "-- SX9 Ontology - SurrealDB Export (SurrealQL)",
        f"-- Generated: {datetime.now(timezone.utc).isoformat()}",
        "-- SPIRES extraction via RFC-9105",
        "",
        "USE NS ctas7 DB threat_ontology;",
        "",
        "-- Define schema",
        "DEFINE TABLE term SCHEMAFULL;",
        "DEFINE FIELD sch_id ON term TYPE string;",
        "DEFINE FIELD canonical_name ON term TYPE string;",
        "DEFINE FIELD category ON term TYPE string;",
        "DEFINE FIELD aliases ON term TYPE array;",
        "DEFINE FIELD definition ON term TYPE string;",
        "DEFINE FIELD rfc_sources ON term TYPE array;",
        "DEFINE FIELD frequency ON term TYPE int;",
        "DEFINE FIELD delta_angle ON term TYPE float;",
        "DEFINE INDEX sch_id_index ON term FIELDS sch_id UNIQUE;",
        "DEFINE INDEX canonical_name_index ON term FIELDS canonical_name;",
        "DEFINE INDEX category_index ON term FIELDS category;",
        "",
        "DEFINE TABLE relation SCHEMAFULL;",
        "DEFINE FIELD source ON relation TYPE string;",
        "DEFINE FIELD target ON relation TYPE string;",
        "DEFINE FIELD relation_type ON relation TYPE string;",
        "DEFINE FIELD weight ON relation TYPE float;",
        "DEFINE FIELD delta_angle ON relation TYPE float;",
        "",
        "-- Create term nodes",
    ]
    
    # Create term records
    for term in graph.terms.values():
        aliases_str = json.dumps(term.aliases)
        sources_str = json.dumps(term.rfc_sources)
        definition = term.definition.replace("'", "\\'").replace('"', '\\"')
        
        # Use sch_id as record ID (sanitized)
        record_id = term.sch_id.replace(":", "_").replace("/", "_")
        
        query = f"""
CREATE term:{record_id} SET
    sch_id = '{term.sch_id}',
    canonical_name = '{term.canonical_name}',
    category = '{term.category}',
    aliases = {aliases_str},
    definition = '{definition}',
    rfc_sources = {sources_str},
    frequency = {term.frequency},
    delta_angle = {term.delta_angle},
    created_at = time::now();"""
        queries.append(query.strip())
    
    # Create relationships
    queries.append("\n-- Create relationships")
    for rel in graph.relations:
        source_id = rel["source"].replace(":", "_").replace("/", "_")
        target_id = rel["target"].replace(":", "_").replace("/", "_")
        rel_type = rel["relation_type"].upper().replace(" ", "_")
        
        query = f"""
RELATE term:{source_id}->{rel_type}->term:{target_id} SET
    weight = {rel["weight"]},
    delta_angle = {rel["delta_angle"]},
    created_at = time::now();"""
        queries.append(query.strip())
    
    return "\n".join(queries)

def generate_json_export(graph: OntologyGraph) -> Dict:
    """Generate JSON export of ontology."""
    return {
        "metadata": graph.metadata,
        "statistics": {
            "total_terms": len(graph.terms),
            "total_relations": len(graph.relations),
            "categories": dict(Counter(t.category for t in graph.terms.values())),
            "top_terms": [
                {"name": t.canonical_name, "frequency": t.frequency, "category": t.category}
                for t in sorted(graph.terms.values(), key=lambda x: x.frequency, reverse=True)[:20]
            ]
        },
        "terms": [
            {
                "sch_id": t.sch_id,
                "canonical_name": t.canonical_name,
                "category": t.category,
                "aliases": t.aliases,
                "definition": t.definition,
                "rfc_sources": t.rfc_sources,
                "frequency": t.frequency,
                "delta_angle": t.delta_angle,
                "related_terms": t.related_terms
            }
            for t in graph.terms.values()
        ],
        "relations": graph.relations
    }

# ============================================================================
# MAIN
# ============================================================================

async def main():
    parser = argparse.ArgumentParser(description="SPIRES Ontology Extractor")
    parser.add_argument("--extract", action="store_true", help="Extract terms from RFCs")
    parser.add_argument("--threats", action="store_true", help="Extract from threat content (MITRE, Sigma, etc.)")
    parser.add_argument("--normalize", action="store_true", help="Normalize terminology")
    parser.add_argument("--enrich", action="store_true", help="Enrich with Gemini definitions")
    parser.add_argument("--linkml", action="store_true", help="Generate LinkML schema")
    parser.add_argument("--all", action="store_true", help="Run full pipeline (RFCs + Threats)")

    args = parser.parse_args()

    print("=" * 70)
    print("SPIRES ONTOLOGY EXTRACTOR - RFC-9105 COMPLIANT")
    print("Palantir-Competitive Enterprise Intelligence Ontology")
    print("=" * 70)

    # Ensure output directory exists
    OUTPUT_PATH.mkdir(parents=True, exist_ok=True)

    extractor = SPIRESExtractor()

    graph = OntologyGraph()

    # RFC extraction
    if args.all or args.extract:
        print("\n[1/5] Extracting ontology from RFC corpus...")
        rfc_graph = extractor.extract_from_extractions()
        print(f"  Extracted {len(rfc_graph.terms)} terms, {len(rfc_graph.relations)} relations from RFCs")

        # Merge into main graph
        for term in rfc_graph.terms.values():
            graph.add_term(term)
        graph.relations.extend(rfc_graph.relations)

        # Save RFC-only results
        with open(OUTPUT_PATH / "ontology_rfc_raw.json", 'w') as f:
            json.dump(generate_json_export(rfc_graph), f, indent=2)

    # Threat content extraction
    if args.all or args.threats:
        print("\n[2/5] Extracting ontology from threat content...")
        threat_graph = extractor.extract_from_threats()
        print(f"  Extracted {len(threat_graph.terms)} terms, {len(threat_graph.relations)} relations from threats")

        # Merge into main graph
        for term in threat_graph.terms.values():
            graph.add_term(term)
        graph.relations.extend(threat_graph.relations)

        # Save threat-only results
        with open(OUTPUT_PATH / "ontology_threats_raw.json", 'w') as f:
            json.dump(generate_json_export(threat_graph), f, indent=2)

    # Load existing if no extraction requested
    if not (args.all or args.extract or args.threats):
        raw_path = OUTPUT_PATH / "ontology_raw.json"
        if raw_path.exists():
            with open(raw_path) as f:
                data = json.load(f)
            for t in data.get("terms", []):
                graph.terms[t["canonical_name"]] = OntologyTerm(**t)
            graph.relations = data.get("relations", [])
        else:
            print("No existing extraction found. Run with --extract or --threats first.")
            return

    # Update metadata and save combined raw
    if args.all or args.extract or args.threats:
        graph.metadata = {
            "extracted_at": datetime.now(timezone.utc).isoformat(),
            "source": "SPIRES unified extraction (RFCs + Threats)",
            "version": "1.0"
        }
        with open(OUTPUT_PATH / "ontology_raw.json", 'w') as f:
            json.dump(generate_json_export(graph), f, indent=2)
        print(f"\n  Combined: {len(graph.terms)} terms, {len(graph.relations)} relations")

    if args.all or args.enrich:
        print("\n[3/5] Enriching ontology with Gemini definitions...")
        import asyncio
        graph = await extractor.enrich_with_gemini(graph)

        with open(OUTPUT_PATH / "ontology_enriched.json", 'w') as f:
            json.dump(generate_json_export(graph), f, indent=2)

    if args.all or args.linkml:
        print("\n[4/5] Generating LinkML schema...")
        schema = generate_linkml_schema(graph)
        with open(OUTPUT_PATH / "sx9_ontology.yaml", 'w') as f:
            f.write(schema)
        print(f"  Saved: {OUTPUT_PATH / 'sx9_ontology.yaml'}")

    # Always generate exports
    print("\n[5/5] Generating exports...")

    # Cypher export
    cypher = generate_cypher_export(graph)
    with open(OUTPUT_PATH / "ontology.cypher", 'w') as f:
        f.write(cypher)
    print(f"  Cypher: {OUTPUT_PATH / 'ontology.cypher'}")

    # Final JSON export
    final_json = generate_json_export(graph)
    with open(OUTPUT_PATH / "sx9_ontology.json", 'w') as f:
        json.dump(final_json, f, indent=2)
    print(f"  JSON: {OUTPUT_PATH / 'sx9_ontology.json'}")

    # Summary
    print("\n" + "=" * 70)
    print("ONTOLOGY EXTRACTION COMPLETE")
    print("=" * 70)
    print(f"Total terms: {len(graph.terms)}")
    print(f"Total relations: {len(graph.relations)}")
    print("\nCategory breakdown:")
    for cat, count in sorted(Counter(t.category for t in graph.terms.values()).items(), key=lambda x: -x[1]):
        print(f"  {cat:15s}: {count:4d} terms")

    print(f"\nTop 10 terms by frequency:")
    for t in sorted(graph.terms.values(), key=lambda x: x.frequency, reverse=True)[:10]:
        print(f"  {t.frequency:3d}x {t.canonical_name:30s} ({t.category})")

    print(f"\nOutputs saved to: {OUTPUT_PATH}")

if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
