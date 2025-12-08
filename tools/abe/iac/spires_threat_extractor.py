#!/usr/bin/env python3
"""
SPIRES-based Threat Content Extraction using OntoGPT
RFC-9105: Zero-shot semantic extraction for threat intelligence

Usage:
    python spires_threat_extractor.py --input output/threat_content --output output/spires_extracted
    python spires_threat_extractor.py --to-cypher output/spires_extracted > output/cypher/spires_graph.cypher
"""

import argparse
import json
import logging
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field

# OntoGPT imports (pip install ontogpt)
try:
    from ontogpt.engines.spires_engine import SPIRESEngine
    from ontogpt.io.yaml_wrapper import dump_minimal_yaml
    ONTOGPT_AVAILABLE = True
except ImportError:
    ONTOGPT_AVAILABLE = False
    logging.warning("OntoGPT not installed. Run: pip install ontogpt")

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


@dataclass
class ExtractionResult:
    """Container for extraction results."""
    techniques: List[Dict] = field(default_factory=list)
    actors: List[Dict] = field(default_factory=list)
    rules: List[Dict] = field(default_factory=list)
    tools: List[Dict] = field(default_factory=list)
    campaigns: List[Dict] = field(default_factory=list)


class SPIRESThreatExtractor:
    """
    Zero-shot semantic extraction for threat intelligence using OntoGPT SPIRES.
    
    SPIRES = Structured Prompt Interrogation and Recursive Extraction of Semantics
    """

    def __init__(self, template_dir: Optional[Path] = None):
        """
        Initialize SPIRES extractor with LinkML templates.
        
        Args:
            template_dir: Path to LinkML template YAML files
        """
        self.template_dir = template_dir or Path(__file__).parent / "linkml_templates"
        self.engines: Dict[str, Any] = {}
        
        if not ONTOGPT_AVAILABLE:
            logger.error("OntoGPT not available. Install with: pip install ontogpt")
        
        # Verify templates exist
        self._verify_templates()

    def _verify_templates(self):
        """Verify all required LinkML templates are present."""
        required_templates = [
            "threat_technique.yaml",
            "threat_actor.yaml", 
            "detection_rule.yaml",
            "offensive_tool.yaml",
            "threat_campaign.yaml",
        ]
        
        missing = []
        for template in required_templates:
            template_path = self.template_dir / template
            if not template_path.exists():
                missing.append(template)
        
        if missing:
            logger.warning(f"Missing LinkML templates: {missing}")

    def get_engine(self, template_name: str) -> Optional[Any]:
        """
        Load or create SPIRES engine for a template.
        
        Args:
            template_name: Name of the template (without .yaml extension)
            
        Returns:
            SPIRESEngine instance or None if not available
        """
        if not ONTOGPT_AVAILABLE:
            return None
            
        if template_name not in self.engines:
            template_path = self.template_dir / f"{template_name}.yaml"
            if not template_path.exists():
                logger.error(f"Template not found: {template_path}")
                return None
                
            try:
                self.engines[template_name] = SPIRESEngine(
                    template_path=str(template_path)
                )
                logger.info(f"Loaded SPIRES engine for: {template_name}")
            except Exception as e:
                logger.error(f"Failed to load SPIRES engine for {template_name}: {e}")
                return None
                
        return self.engines[template_name]

    def extract_techniques(self, text: str) -> Optional[Dict]:
        """
        Extract MITRE ATT&CK techniques from unstructured text.
        
        Args:
            text: Raw text containing technique information
            
        Returns:
            LinkML-conformant dict with extracted technique data
        """
        engine = self.get_engine("threat_technique")
        if not engine:
            return None
            
        try:
            result = engine.extract_from_text(text)
            return result.object
        except Exception as e:
            logger.error(f"Technique extraction failed: {e}")
            return None

    def extract_actors(self, text: str) -> Optional[Dict]:
        """
        Extract threat actor information from text.
        
        Args:
            text: Raw text containing actor information
            
        Returns:
            LinkML-conformant dict with extracted actor data
        """
        engine = self.get_engine("threat_actor")
        if not engine:
            return None
            
        try:
            result = engine.extract_from_text(text)
            return result.object
        except Exception as e:
            logger.error(f"Actor extraction failed: {e}")
            return None

    def extract_detection_rules(self, rule_text: str) -> Optional[Dict]:
        """
        Extract detection rule semantics from Sigma/YARA/Wazuh rules.
        
        Args:
            rule_text: Raw rule content (YAML or text)
            
        Returns:
            LinkML-conformant dict with extracted rule data
        """
        engine = self.get_engine("detection_rule")
        if not engine:
            return None
            
        try:
            result = engine.extract_from_text(rule_text)
            return result.object
        except Exception as e:
            logger.error(f"Rule extraction failed: {e}")
            return None

    def extract_tools(self, text: str) -> Optional[Dict]:
        """
        Extract offensive tool information.
        
        Args:
            text: Raw text containing tool information
            
        Returns:
            LinkML-conformant dict with extracted tool data
        """
        engine = self.get_engine("offensive_tool")
        if not engine:
            return None
            
        try:
            result = engine.extract_from_text(text)
            return result.object
        except Exception as e:
            logger.error(f"Tool extraction failed: {e}")
            return None

    def extract_campaigns(self, text: str) -> Optional[Dict]:
        """
        Extract threat campaign information.
        
        Args:
            text: Raw text containing campaign information
            
        Returns:
            LinkML-conformant dict with extracted campaign data
        """
        engine = self.get_engine("threat_campaign")
        if not engine:
            return None
            
        try:
            result = engine.extract_from_text(text)
            return result.object
        except Exception as e:
            logger.error(f"Campaign extraction failed: {e}")
            return None

    def process_threat_content(self, content_dir: Path) -> ExtractionResult:
        """
        Process all threat content through SPIRES extraction.
        
        Args:
            content_dir: Directory containing threat content files
            
        Returns:
            ExtractionResult with all extracted entities
        """
        results = ExtractionResult()
        
        # Process MITRE ATT&CK techniques
        attack_file = content_dir / "mitre_attack.json"
        if attack_file.exists():
            logger.info("Processing MITRE ATT&CK techniques...")
            attack_data = json.loads(attack_file.read_text())
            for technique in attack_data.get("techniques", []):
                extracted = self.extract_techniques(json.dumps(technique))
                if extracted:
                    results.techniques.append(extracted)
            logger.info(f"Extracted {len(results.techniques)} techniques")

        # Process threat actors/groups
        groups_file = content_dir / "mitre_groups.json"
        if groups_file.exists():
            logger.info("Processing threat actors...")
            groups_data = json.loads(groups_file.read_text())
            for group in groups_data.get("groups", []):
                extracted = self.extract_actors(json.dumps(group))
                if extracted:
                    results.actors.append(extracted)
            logger.info(f"Extracted {len(results.actors)} actors")

        # Process Sigma rules
        sigma_dir = content_dir / "sigma_rules"
        if sigma_dir.exists():
            logger.info("Processing Sigma rules...")
            for rule_file in sigma_dir.glob("**/*.yml"):
                rule_text = rule_file.read_text()
                extracted = self.extract_detection_rules(rule_text)
                if extracted:
                    results.rules.append(extracted)
            logger.info(f"Extracted {len(results.rules)} detection rules")

        # Process YARA rules
        yara_dir = content_dir / "yara_rules"
        if yara_dir.exists():
            logger.info("Processing YARA rules...")
            for rule_file in yara_dir.glob("**/*.yar"):
                rule_text = rule_file.read_text()
                extracted = self.extract_detection_rules(rule_text)
                if extracted:
                    results.rules.append(extracted)

        # Process Kali tools
        tools_file = content_dir / "kali_tools_inventory.json"
        if tools_file.exists():
            logger.info("Processing offensive tools...")
            tools_data = json.loads(tools_file.read_text())
            for tool in tools_data.get("tools", tools_data):
                if isinstance(tool, dict):
                    extracted = self.extract_tools(json.dumps(tool))
                    if extracted:
                        results.tools.append(extracted)
            logger.info(f"Extracted {len(results.tools)} tools")

        return results

    def to_neo4j_cypher(self, extracted: ExtractionResult) -> List[str]:
        """
        Convert extracted data to Neo4j Cypher statements.
        
        Args:
            extracted: ExtractionResult containing all extracted entities
            
        Returns:
            List of Cypher CREATE/MERGE statements
        """
        cypher = []
        
        # Create technique nodes
        for tech in extracted.techniques:
            tech_id = tech.get('technique_id', '')
            name = self._escape_cypher(tech.get('name', ''))
            tactics = tech.get('tactic', [])
            platforms = tech.get('platforms', [])
            
            cypher.append(f'''
CREATE (t:Technique {{
    id: "{tech_id}",
    name: "{name}",
    tactics: {json.dumps(tactics)},
    platforms: {json.dumps(platforms)}
}});''')
            
            # Link detection methods
            for method in tech.get('detection_methods', []):
                method_name = self._escape_cypher(method.get('name', ''))
                data_source = self._escape_cypher(method.get('data_source', ''))
                cypher.append(f'''
MERGE (d:DetectionMethod {{name: "{method_name}", data_source: "{data_source}"}})
WITH d
MATCH (t:Technique {{id: "{tech_id}"}})
MERGE (t)-[:DETECTED_BY]->(d);''')

            # Link mitigations
            for mitigation in tech.get('mitigations', []):
                mit_name = self._escape_cypher(mitigation.get('name', ''))
                d3fend_id = mitigation.get('d3fend_id', '')
                cypher.append(f'''
MERGE (m:Mitigation {{name: "{mit_name}", d3fend_id: "{d3fend_id}"}})
WITH m
MATCH (t:Technique {{id: "{tech_id}"}})
MERGE (t)-[:MITIGATED_BY]->(m);''')

        # Create actor nodes
        for actor in extracted.actors:
            actor_id = actor.get('actor_id', '')
            name = self._escape_cypher(actor.get('name', ''))
            aliases = actor.get('aliases', [])
            
            cypher.append(f'''
CREATE (a:ThreatActor {{
    id: "{actor_id}",
    name: "{name}",
    aliases: {json.dumps(aliases)}
}});''')
            
            # Link techniques used
            for tech_id in actor.get('techniques_used', []):
                if isinstance(tech_id, str):
                    cypher.append(f'''
MATCH (a:ThreatActor {{id: "{actor_id}"}}), (t:Technique {{id: "{tech_id}"}})
MERGE (a)-[:USES]->(t);''')

        # Create detection rule nodes
        for rule in extracted.rules:
            rule_id = rule.get('rule_id', '')
            title = self._escape_cypher(rule.get('title', ''))
            status = rule.get('status', 'experimental')
            level = rule.get('level', 'medium')
            
            cypher.append(f'''
CREATE (r:DetectionRule {{
    id: "{rule_id}",
    title: "{title}",
    status: "{status}",
    level: "{level}"
}});''')
            
            # Link to techniques
            for tech_ref in rule.get('mitre_attack_refs', []):
                cypher.append(f'''
MATCH (r:DetectionRule {{id: "{rule_id}"}}), (t:Technique {{id: "{tech_ref}"}})
MERGE (r)-[:DETECTS]->(t);''')

        # Create tool nodes
        for tool in extracted.tools:
            package_name = tool.get('package_name', '')
            display_name = self._escape_cypher(tool.get('display_name', ''))
            categories = tool.get('categories', [])
            
            cypher.append(f'''
CREATE (tool:OffensiveTool {{
    package_name: "{package_name}",
    display_name: "{display_name}",
    categories: {json.dumps(categories)}
}});''')
            
            # Link to techniques
            for tech_id in tool.get('mitre_techniques', []):
                cypher.append(f'''
MATCH (tool:OffensiveTool {{package_name: "{package_name}"}}), (t:Technique {{id: "{tech_id}"}})
MERGE (tool)-[:IMPLEMENTS]->(t);''')

        return cypher

    def _escape_cypher(self, value: str) -> str:
        """Escape special characters for Cypher strings."""
        if not isinstance(value, str):
            return str(value)
        return value.replace('\\', '\\\\').replace('"', '\\"').replace("'", "\\'")

    def save_results(self, results: ExtractionResult, output_dir: Path):
        """
        Save extraction results to JSON files.
        
        Args:
            results: ExtractionResult to save
            output_dir: Directory for output files
        """
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # Save each entity type
        (output_dir / "techniques.json").write_text(
            json.dumps(results.techniques, indent=2)
        )
        (output_dir / "actors.json").write_text(
            json.dumps(results.actors, indent=2)
        )
        (output_dir / "rules.json").write_text(
            json.dumps(results.rules, indent=2)
        )
        (output_dir / "tools.json").write_text(
            json.dumps(results.tools, indent=2)
        )
        
        # Save summary
        summary = {
            "techniques_count": len(results.techniques),
            "actors_count": len(results.actors),
            "rules_count": len(results.rules),
            "tools_count": len(results.tools),
        }
        (output_dir / "extraction_summary.json").write_text(
            json.dumps(summary, indent=2)
        )
        
        logger.info(f"Results saved to {output_dir}")


def main():
    parser = argparse.ArgumentParser(
        description="SPIRES-based threat content extraction using OntoGPT"
    )
    parser.add_argument(
        "--input", "-i",
        type=Path,
        required=True,
        help="Input directory containing threat content"
    )
    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=Path("output/spires_extracted"),
        help="Output directory for extracted data"
    )
    parser.add_argument(
        "--templates", "-t",
        type=Path,
        default=None,
        help="Directory containing LinkML templates"
    )
    parser.add_argument(
        "--to-cypher",
        action="store_true",
        help="Output Neo4j Cypher statements to stdout"
    )
    parser.add_argument(
        "--cypher-file",
        type=Path,
        help="Write Cypher statements to file"
    )
    
    args = parser.parse_args()
    
    # Initialize extractor
    extractor = SPIRESThreatExtractor(template_dir=args.templates)
    
    # Process content
    logger.info(f"Processing threat content from: {args.input}")
    results = extractor.process_threat_content(args.input)
    
    # Save JSON results
    extractor.save_results(results, args.output)
    
    # Generate Cypher if requested
    if args.to_cypher or args.cypher_file:
        cypher_statements = extractor.to_neo4j_cypher(results)
        cypher_output = "\n".join(cypher_statements)
        
        if args.cypher_file:
            args.cypher_file.parent.mkdir(parents=True, exist_ok=True)
            args.cypher_file.write_text(cypher_output)
            logger.info(f"Cypher statements written to: {args.cypher_file}")
        
        if args.to_cypher:
            print(cypher_output)
    
    # Print summary
    print(f"\n=== Extraction Summary ===")
    print(f"Techniques: {len(results.techniques)}")
    print(f"Actors: {len(results.actors)}")
    print(f"Detection Rules: {len(results.rules)}")
    print(f"Offensive Tools: {len(results.tools)}")
    print(f"\nResults saved to: {args.output}")


if __name__ == "__main__":
    main()
