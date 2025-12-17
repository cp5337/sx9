#!/usr/bin/env python3
"""
Leptose Training Data Preparation
==================================

Generates LoRA training datasets for Phi-3 and DistilBERT from threat content.
RFC-9012, RFC-9021

Output Formats:
  - phi3_lora_training.jsonl - Instruct-tuning format for Phi-3
  - distilbert_classification.jsonl - Classification fine-tuning for DistilBERT
  - alpaca_format.json - Alpaca-style instruction dataset

Usage:
    python leptose_training_prep.py --input output/threat_content --output output/training_data
    python leptose_training_prep.py --format phi3 --input output/threat_content
"""

import json
import argparse
import logging
import random
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field, asdict

try:
    import yaml
    HAS_YAML = True
except ImportError:
    HAS_YAML = False

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

OUTPUT_DIR = Path(__file__).parent / "output"

# ATL-Physical training data path (invisible operationally, included in training)
ATL_PHYSICAL_PATH = Path(__file__).parent.parent.parent / "ctas-dir" / "20-atl" / "physical" / "data" / "atl_physical_ied.yaml"


def load_atl_physical() -> List[Dict]:
    """Load ATL-Physical tasks as training documents.

    ATL-Physical contains adversary task decomposition for physical threats
    (IED, active shooter, intrusion, etc). Invisible operationally but
    included in training for cross-domain pattern recognition.

    Returns:
        List of document dicts with id, source, domain, type, text, metadata
    """
    if not HAS_YAML:
        logger.warning("PyYAML not installed, cannot load ATL-Physical")
        return []

    if not ATL_PHYSICAL_PATH.exists():
        logger.warning(f"ATL-Physical not found at {ATL_PHYSICAL_PATH}")
        return []

    with open(ATL_PHYSICAL_PATH, 'r', encoding='utf-8') as f:
        data = yaml.safe_load(f)

    documents = []

    # Process tasks
    for task in data.get('tasks', []):
        doc = {
            'id': f"atl-physical-{task['task_id']}",
            'source': 'ATL-Physical',
            'domain': 'physical',
            'modality': task.get('modality', ['IED'])[0] if isinstance(task.get('modality'), list) else task.get('modality', 'IED'),
            'type': 'adversary_task',
            'phase': task.get('phase', 0),
            'hd4_phases': task.get('hd4_phases', []),
            'classification': task.get('classification', 'OPTIONAL'),
            'is_mandatory': task.get('classification') == 'MANDATORY',
            'is_interdiction_point': task.get('is_interdiction_point', False),
            'is_key_indicator': task.get('is_key_indicator', False),
            'node_form': task.get('node_form', '1n'),
            'text': f"{task.get('title', '')}\n\n{task.get('description', '')}".strip(),
            'metadata': {
                'task_id': task.get('task_id'),
                'parent_task': task.get('parent_task'),
                'related_tasks': task.get('related_tasks', []),
                'mundanity_score': task.get('mundanity_score', 0.5),
            }
        }
        documents.append(doc)

    # Process relationships as training context
    for idx, rel in enumerate(data.get('relationships', [])):
        doc = {
            'id': f"atl-physical-rel-{idx}-{rel.get('source', 'unknown')}-{rel.get('target_page', 0)}",
            'source': 'ATL-Physical',
            'domain': 'physical',
            'type': 'task_relationship',
            'text': f"Task {rel.get('source', '')} is related to {rel.get('target_title', '')}",
            'metadata': rel
        }
        documents.append(doc)

    logger.info(f"[ATL-Physical] Loaded {len(documents)} training documents")
    return documents


@dataclass
class Phi3Sample:
    """Phi-3 instruct-tuning sample format."""
    instruction: str
    input: str
    output: str
    system: str = "You are a threat intelligence analyst assistant."


@dataclass
class ClassificationSample:
    """DistilBERT classification sample."""
    text: str
    label: str
    label_id: int
    metadata: Dict[str, Any] = field(default_factory=dict)


@dataclass
class AlpacaSample:
    """Alpaca-style instruction sample."""
    instruction: str
    input: str
    output: str


class LeptoseTrainingPrep:
    """Generate training datasets for Leptose inference engine."""

    def __init__(self, output_dir: Path = None):
        self.output_dir = output_dir or OUTPUT_DIR / "training_data"
        self.output_dir.mkdir(parents=True, exist_ok=True)

        # Label mappings for classification
        self.tactic_labels = {
            "reconnaissance": 0,
            "resource-development": 1,
            "initial-access": 2,
            "execution": 3,
            "persistence": 4,
            "privilege-escalation": 5,
            "defense-evasion": 6,
            "credential-access": 7,
            "discovery": 8,
            "lateral-movement": 9,
            "collection": 10,
            "command-and-control": 11,
            "exfiltration": 12,
            "impact": 13,
        }

        self.hd4_labels = {
            "HUNT": 0,
            "DETECT": 1,
            "DISABLE": 2,
            "DISRUPT": 3,
            "DOMINATE": 4,
        }

    def generate_phi3_samples(self, techniques: List[Dict]) -> List[Dict]:
        """Generate Phi-3 instruct-tuning samples from techniques."""
        samples = []

        for tech in techniques:
            tech_id = tech.get("technique_id") or tech.get("id", "")
            name = tech.get("name", "")
            description = tech.get("description", "")[:1500]
            tactics = tech.get("tactic", []) or tech.get("tactics", [])
            platforms = tech.get("platforms", [])
            detection = tech.get("detection", "")[:1000]
            mitigations = tech.get("mitigations", [])

            if not name or not description:
                continue

            # Sample 1: Technique explanation
            samples.append(asdict(Phi3Sample(
                instruction="Explain this MITRE ATT&CK technique and its implications.",
                input=f"Technique: {tech_id} - {name}",
                output=f"{description}\n\nTactics: {', '.join(tactics) if tactics else 'N/A'}\nPlatforms: {', '.join(platforms) if platforms else 'N/A'}"
            )))

            # Sample 2: Detection guidance
            if detection:
                samples.append(asdict(Phi3Sample(
                    instruction="How can this technique be detected?",
                    input=f"Technique: {tech_id} - {name}",
                    output=detection
                )))

            # Sample 3: Mitigation recommendations
            if mitigations:
                mit_text = "\n".join([f"- {m.get('name', m) if isinstance(m, dict) else m}" for m in mitigations[:5]])
                samples.append(asdict(Phi3Sample(
                    instruction="What mitigations are effective against this technique?",
                    input=f"Technique: {tech_id} - {name}",
                    output=f"Recommended mitigations:\n{mit_text}"
                )))

            # Sample 4: Tactic classification
            if tactics:
                samples.append(asdict(Phi3Sample(
                    instruction="What MITRE ATT&CK tactic does this technique belong to?",
                    input=f"Technique: {name}\nDescription: {description[:500]}",
                    output=f"This technique belongs to the following tactics: {', '.join(tactics)}"
                )))

        logger.info(f"Generated {len(samples)} Phi-3 training samples from techniques")
        return samples

    def generate_phi3_from_rules(self, rules: List[Dict]) -> List[Dict]:
        """Generate Phi-3 samples from detection rules."""
        samples = []

        for rule in rules:
            title = rule.get("title", "")
            description = rule.get("description", "")
            level = rule.get("level", "medium")
            status = rule.get("status", "experimental")
            mitre_refs = rule.get("mitre_attack_refs", [])
            logsource = rule.get("logsource", {})
            false_positives = rule.get("false_positives", [])

            if not title:
                continue

            # Sample: Rule explanation
            output_parts = [description] if description else []
            if logsource:
                ls_str = ", ".join([f"{k}: {v}" for k, v in logsource.items() if v])
                output_parts.append(f"Log Source: {ls_str}")
            if mitre_refs:
                output_parts.append(f"MITRE ATT&CK: {', '.join(mitre_refs)}")
            output_parts.append(f"Severity: {level}, Status: {status}")

            samples.append(asdict(Phi3Sample(
                instruction="Explain this detection rule and what it detects.",
                input=f"Rule: {title}",
                output="\n".join(output_parts)
            )))

            # Sample: False positive guidance
            if false_positives:
                fp_text = "\n".join([f"- {fp}" for fp in false_positives[:5]])
                samples.append(asdict(Phi3Sample(
                    instruction="What are the known false positives for this detection rule?",
                    input=f"Rule: {title}",
                    output=f"Known false positives:\n{fp_text}"
                )))

        logger.info(f"Generated {len(samples)} Phi-3 training samples from rules")
        return samples

    def generate_classification_samples(self, techniques: List[Dict]) -> List[Dict]:
        """Generate DistilBERT classification samples."""
        samples = []

        for tech in techniques:
            name = tech.get("name", "")
            description = tech.get("description", "")[:500]
            tactics = tech.get("tactic", []) or tech.get("tactics", [])

            if not name or not tactics:
                continue

            text = f"{name}. {description}"

            # Multi-label: create sample for each tactic
            for tactic in tactics:
                tactic_norm = tactic.lower().replace(" ", "-")
                if tactic_norm in self.tactic_labels:
                    samples.append(asdict(ClassificationSample(
                        text=text,
                        label=tactic_norm,
                        label_id=self.tactic_labels[tactic_norm],
                        metadata={"technique_id": tech.get("technique_id", "")}
                    )))

        logger.info(f"Generated {len(samples)} classification samples")
        return samples

    def generate_hd4_samples(self, techniques: List[Dict], rules: List[Dict]) -> List[Dict]:
        """Generate HD4 phase classification samples."""
        samples = []

        # Map tactics to HD4 phases
        tactic_to_hd4 = {
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

        for tech in techniques:
            name = tech.get("name", "")
            description = tech.get("description", "")[:500]
            tactics = tech.get("tactic", []) or tech.get("tactics", [])

            if not name or not tactics:
                continue

            # Determine primary HD4 phase
            hd4_phases = set()
            for tactic in tactics:
                tactic_norm = tactic.lower().replace(" ", "-")
                if tactic_norm in tactic_to_hd4:
                    hd4_phases.add(tactic_to_hd4[tactic_norm])

            for phase in hd4_phases:
                text = f"{name}. {description}"
                samples.append(asdict(ClassificationSample(
                    text=text,
                    label=phase,
                    label_id=self.hd4_labels[phase],
                    metadata={"technique_id": tech.get("technique_id", ""), "source": "technique"}
                )))

        # Detection rules → DETECT phase
        for rule in rules:
            title = rule.get("title", "")
            description = rule.get("description", "")[:500]

            if not title:
                continue

            text = f"{title}. {description}"
            samples.append(asdict(ClassificationSample(
                text=text,
                label="DETECT",
                label_id=self.hd4_labels["DETECT"],
                metadata={"rule_id": rule.get("id", ""), "source": "detection_rule"}
            )))

        logger.info(f"Generated {len(samples)} HD4 classification samples")
        return samples

    def generate_alpaca_samples(self, techniques: List[Dict], rules: List[Dict]) -> List[Dict]:
        """Generate Alpaca-format instruction samples."""
        samples = []

        # Technique-based samples
        for tech in techniques:
            tech_id = tech.get("technique_id") or tech.get("id", "")
            name = tech.get("name", "")
            description = tech.get("description", "")[:1000]

            if not name:
                continue

            samples.append(asdict(AlpacaSample(
                instruction="Describe this cybersecurity attack technique.",
                input=name,
                output=f"{tech_id}: {description}"
            )))

        # Q&A style samples
        qa_templates = [
            ("What is {name}?", "{description}"),
            ("How does the {name} attack work?", "{description}"),
            ("Explain the {name} technique used by attackers.", "{description}"),
        ]

        for tech in random.sample(techniques, min(100, len(techniques))):
            name = tech.get("name", "")
            description = tech.get("description", "")[:800]

            if not name or not description:
                continue

            template = random.choice(qa_templates)
            samples.append(asdict(AlpacaSample(
                instruction=template[0].format(name=name),
                input="",
                output=template[1].format(description=description)
            )))

        logger.info(f"Generated {len(samples)} Alpaca-format samples")
        return samples

    def generate_atl_physical_samples(self, atl_docs: List[Dict]) -> Dict[str, List]:
        """Generate training samples from ATL-Physical documents.

        ATL-Physical is INVISIBLE operationally but INCLUDED in training
        for cross-domain pattern recognition.
        """
        phi3_samples = []
        hd4_samples = []
        alpaca_samples = []

        for doc in atl_docs:
            if doc.get('type') != 'adversary_task':
                continue

            task_id = doc.get('metadata', {}).get('task_id', '')
            text = doc.get('text', '')
            hd4_phases = doc.get('hd4_phases', [])
            phase = doc.get('phase', 0)
            modality = doc.get('modality', 'IED')
            is_mandatory = doc.get('is_mandatory', False)
            is_interdiction = doc.get('is_interdiction_point', False)

            if not text:
                continue

            # Phi-3 instruction samples
            phi3_samples.append(asdict(Phi3Sample(
                instruction="Describe this adversary task in the physical threat domain.",
                input=f"Task {task_id} ({modality})",
                output=text,
                system="You are a physical security threat analyst."
            )))

            # Phase-specific instruction
            if phase > 0:
                phi3_samples.append(asdict(Phi3Sample(
                    instruction=f"What happens in phase {phase} of adversary operations?",
                    input=f"Task: {task_id}",
                    output=text,
                    system="You are a physical security threat analyst."
                )))

            # Interdiction point guidance
            if is_interdiction:
                phi3_samples.append(asdict(Phi3Sample(
                    instruction="Identify interdiction opportunities for this adversary task.",
                    input=f"Task {task_id}: {text[:200]}",
                    output=f"This task ({task_id}) is an interdiction point. Early detection at this stage can disrupt the adversary's operational timeline.",
                    system="You are a physical security threat analyst."
                )))

            # HD4 classification samples
            for hd4_phase in hd4_phases:
                if hd4_phase in self.hd4_labels:
                    hd4_samples.append(asdict(ClassificationSample(
                        text=text,
                        label=hd4_phase,
                        label_id=self.hd4_labels[hd4_phase],
                        metadata={
                            'task_id': task_id,
                            'source': 'ATL-Physical',
                            'domain': 'physical',
                            'modality': modality,
                        }
                    )))

            # Alpaca samples
            alpaca_samples.append(asdict(AlpacaSample(
                instruction="Explain this physical domain adversary task.",
                input=f"Task {task_id}",
                output=text
            )))

        logger.info(f"[ATL-Physical] Generated {len(phi3_samples)} Phi-3, {len(hd4_samples)} HD4, {len(alpaca_samples)} Alpaca samples")

        return {
            'phi3': phi3_samples,
            'hd4': hd4_samples,
            'alpaca': alpaca_samples,
        }

    def process_threat_content(self, content_dir: Path) -> Dict[str, List]:
        """Process all threat content and generate training data."""
        results = {
            "phi3": [],
            "classification": [],
            "hd4": [],
            "alpaca": [],
        }

        techniques = []
        rules = []

        # Load techniques
        attack_file = content_dir / "mitre_attack.json"
        if attack_file.exists():
            logger.info(f"Loading techniques from {attack_file}")
            with open(attack_file) as f:
                data = json.load(f)
                techniques = data if isinstance(data, list) else data.get("techniques", [])
                logger.info(f"Loaded {len(techniques)} techniques")

        # Load rules (simplified - just load metadata)
        rules_dir = content_dir / "sigma_rules"
        if rules_dir.exists():
            logger.info(f"Loading rules from {rules_dir}")
            try:
                import yaml
                for rule_file in rules_dir.glob("**/*.yml"):
                    try:
                        with open(rule_file) as f:
                            rule = yaml.safe_load(f)
                            if rule:
                                rules.append(rule)
                    except:
                        continue
                logger.info(f"Loaded {len(rules)} rules")
            except ImportError:
                logger.warning("PyYAML not installed, skipping rule loading")

        # Generate training samples from cyber threat content
        results["phi3"] = self.generate_phi3_samples(techniques)
        results["phi3"].extend(self.generate_phi3_from_rules(rules))
        results["classification"] = self.generate_classification_samples(techniques)
        results["hd4"] = self.generate_hd4_samples(techniques, rules)
        results["alpaca"] = self.generate_alpaca_samples(techniques, rules)

        # ATL sources (invisible operationally, included in training)
        atl_physical_docs = load_atl_physical()
        if atl_physical_docs:
            atl_samples = self.generate_atl_physical_samples(atl_physical_docs)
            results["phi3"].extend(atl_samples.get('phi3', []))
            results["hd4"].extend(atl_samples.get('hd4', []))
            results["alpaca"].extend(atl_samples.get('alpaca', []))
        # Future: load_atl_cyber(), load_atl_wmd() (reserved)

        return results

    def save_datasets(self, data: Dict[str, List]):
        """Save all training datasets to files."""
        # Phi-3 LoRA format (JSONL)
        phi3_file = self.output_dir / "phi3_lora_training.jsonl"
        with open(phi3_file, "w") as f:
            for sample in data["phi3"]:
                f.write(json.dumps(sample) + "\n")
        logger.info(f"Saved {len(data['phi3'])} samples to {phi3_file}")

        # DistilBERT classification (JSONL)
        class_file = self.output_dir / "distilbert_classification.jsonl"
        with open(class_file, "w") as f:
            for sample in data["classification"]:
                f.write(json.dumps(sample) + "\n")
        logger.info(f"Saved {len(data['classification'])} samples to {class_file}")

        # HD4 classification (JSONL)
        hd4_file = self.output_dir / "hd4_classification.jsonl"
        with open(hd4_file, "w") as f:
            for sample in data["hd4"]:
                f.write(json.dumps(sample) + "\n")
        logger.info(f"Saved {len(data['hd4'])} samples to {hd4_file}")

        # Alpaca format (JSON)
        alpaca_file = self.output_dir / "alpaca_format.json"
        with open(alpaca_file, "w") as f:
            json.dump(data["alpaca"], f, indent=2)
        logger.info(f"Saved {len(data['alpaca'])} samples to {alpaca_file}")

        # Save label mappings
        labels_file = self.output_dir / "label_mappings.json"
        with open(labels_file, "w") as f:
            json.dump({
                "tactic_labels": self.tactic_labels,
                "hd4_labels": self.hd4_labels,
            }, f, indent=2)

        # Summary
        summary = {
            "phi3_samples": len(data["phi3"]),
            "classification_samples": len(data["classification"]),
            "hd4_samples": len(data["hd4"]),
            "alpaca_samples": len(data["alpaca"]),
            "output_dir": str(self.output_dir),
        }
        summary_file = self.output_dir / "training_summary.json"
        with open(summary_file, "w") as f:
            json.dump(summary, f, indent=2)

        return summary


def main():
    parser = argparse.ArgumentParser(description="Leptose Training Data Preparation")
    parser.add_argument("--input", "-i", type=Path, default=OUTPUT_DIR / "threat_content",
                       help="Input directory with threat content")
    parser.add_argument("--output", "-o", type=Path, default=OUTPUT_DIR / "training_data",
                       help="Output directory for training data")
    parser.add_argument("--format", choices=["all", "phi3", "classification", "hd4", "alpaca"],
                       default="all", help="Output format to generate")
    args = parser.parse_args()

    prep = LeptoseTrainingPrep(output_dir=args.output)

    logger.info(f"Processing threat content from: {args.input}")
    data = prep.process_threat_content(args.input)

    # Filter by format if specified
    if args.format != "all":
        data = {args.format: data.get(args.format, [])}

    summary = prep.save_datasets(data)

    print("\n=== Training Data Summary ===")
    for key, value in summary.items():
        print(f"  {key}: {value}")


if __name__ == "__main__":
    main()
