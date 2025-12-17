#!/usr/bin/env python3
"""
OSSEC TOML Rule Generator - RFC-9302 Nine-Sided Compliant
Converts threat intelligence data to Plasma Defender TOML format
RFC-9001 compliant with dual trivariate hashes (primary + secondary)
RFC-9302 Nonagon Analytic Node (NAN) with 9 vertices, 3 trivariates
"""

import json
import hashlib
import os
import math
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Any, Optional, Tuple
import random

# Output directories
BASE_DIR = Path(__file__).parent
OUTPUT_DIR = BASE_DIR / "output"
TOML_OUTPUT_DIR = OUTPUT_DIR / "ossec_toml_rules"

# RFC-9302 Nonagon Constants
NONAGON_VERTICES = 9
NONAGON_EDGES = 9
NONAGON_DIAGONALS = 27
INTERIOR_ANGLE = 140.0  # degrees
CENTRAL_ANGLE = 40.0    # degrees (360/9)
PRECISION = 6           # 6 decimal places

# RFC-9001 32 Primitives (Trivariate Hashing)
PRIMITIVES = [
    "READ", "WRITE", "FILTER", "TRANSFORM", "EXECUTE", "AUTHENTICATE",
    "AUTHORIZE", "ENCRYPT", "DECRYPT", "VALIDATE", "ROUTE", "BUFFER",
    "QUEUE", "CACHE", "REPLICATE", "SYNCHRONIZE", "OBSERVE", "MEASURE",
    "ALERT", "LOG", "NOTIFY", "ESCALATE", "CONTAIN", "ISOLATE",
    "REMEDIATE", "RECOVER", "RECONNAISSANCE", "WEAPONIZE", "DELIVER",
    "EXPLOIT", "INSTALL", "COMMAND_CONTROL"
]

# MITRE Tactic to Primitive mapping
TACTIC_PRIMITIVE_MAP = {
    "reconnaissance": "RECONNAISSANCE",
    "resource-development": "WEAPONIZE",
    "initial-access": "DELIVER",
    "execution": "EXECUTE",
    "persistence": "INSTALL",
    "privilege-escalation": "ESCALATE",
    "defense-evasion": "FILTER",
    "credential-access": "AUTHENTICATE",
    "discovery": "OBSERVE",
    "lateral-movement": "ROUTE",
    "collection": "READ",
    "command-and-control": "COMMAND_CONTROL",
    "exfiltration": "WRITE",
    "impact": "TRANSFORM",
}

# Unicode trigger ranges for different primitives (U+E400-E4FF reserved for OSSEC)
UNICODE_TRIGGERS = {
    "RECONNAISSANCE": "U+E400",
    "WEAPONIZE": "U+E401",
    "DELIVER": "U+E402",
    "EXPLOIT": "U+E403",
    "INSTALL": "U+E404",
    "COMMAND_CONTROL": "U+E405",
    "EXECUTE": "U+E406",
    "AUTHENTICATE": "U+E407",
    "AUTHORIZE": "U+E408",
    "ESCALATE": "U+E409",
    "OBSERVE": "U+E40A",
    "READ": "U+E40B",
    "WRITE": "U+E40C",
    "FILTER": "U+E40D",
    "TRANSFORM": "U+E40E",
    "ROUTE": "U+E40F",
    "ENCRYPT": "U+E410",
    "DECRYPT": "U+E411",
    "VALIDATE": "U+E412",
    "BUFFER": "U+E413",
    "QUEUE": "U+E414",
    "CACHE": "U+E415",
    "REPLICATE": "U+E416",
    "SYNCHRONIZE": "U+E417",
    "MEASURE": "U+E418",
    "ALERT": "U+E419",
    "LOG": "U+E41A",
    "NOTIFY": "U+E41B",
    "CONTAIN": "U+E41C",
    "ISOLATE": "U+E41D",
    "REMEDIATE": "U+E41E",
    "RECOVER": "U+E41F",
}

# HD4 Phase levels (RFC-9300)
HD4_LEVELS = {
    "Hunt": (1, 6),      # Low severity, reconnaissance
    "Detect": (7, 10),   # Medium severity, detection
    "Disrupt": (11, 13), # High severity, active response
    "Disable": (14, 15), # Critical severity, neutralization
    "Dominate": (16, 16) # Maximum severity, full control
}


def generate_sch(content: str, variant: str = "T") -> str:
    """Generate SCH (Semantic Content Hash) - RFC-9001 §2.2"""
    h = hashlib.sha256(f"{content}:{variant}".encode()).hexdigest()[:16]
    return f"SCH_{h}"


def generate_cuid(context: str, variant: str = "T") -> str:
    """Generate CUID (Context Unique ID) - RFC-9001 §2.2"""
    h = hashlib.md5(f"{context}:{variant}".encode()).hexdigest()[:16]
    return f"CUID_{h}"


def generate_uuid(lineage: str, variant: str = "T") -> str:
    """Generate UUID (Lineage anchor) - RFC-9001 §2.2"""
    h = hashlib.sha1(f"{lineage}:{variant}".encode()).hexdigest()[:16]
    return f"UUID_{h}"


class NonagonNode:
    """RFC-9302 Nonagon Analytic Node (NAN) - 9 vertices, 3 trivariates

    Vertices:
    - A0-A2: Trivariate α (Semantic) - Context, Meaning, Intent
    - A3-A5: Trivariate β (Operational) - Phase, Intensity, Duration
    - A6-A8: Trivariate γ (Temporal) - Historical, Current, Predictive
    """

    # Vertex labels
    ASPECT_LABELS = [
        "α.X (Context)",    # A0
        "α.Y (Meaning)",    # A1
        "α.Z (Intent)",     # A2
        "β.X (Phase)",      # A3
        "β.Y (Intensity)",  # A4
        "β.Z (Duration)",   # A5
        "γ.X (Historical)", # A6
        "γ.Y (Current)",    # A7
        "γ.Z (Predictive)", # A8
    ]

    def __init__(self, rule_id: int, technique: Dict[str, Any]):
        self.rule_id = rule_id
        self.vertices = [0.0] * 9
        self.edges = [1.0] * 9  # Adjacent connections
        self.center = 0.0
        self.confidence = 0.0

        # Calculate vertices from technique
        self._calculate_from_technique(technique)

    def _calculate_from_technique(self, tech: Dict[str, Any]):
        """Calculate all 9 vertices from technique attributes"""

        # Get attributes
        name = tech.get("name", "")
        description = tech.get("description", "")
        tactic = tech.get("tactic", "discovery")
        platforms = tech.get("platforms", [])
        data_sources = tech.get("data_sources", [])
        mitre_id = tech.get("id", "")

        # === TRIVARIATE α (SEMANTIC): Context, Meaning, Intent ===
        # A0: Context - based on platforms and environment
        self.vertices[0] = self._calc_context_score(platforms, description)

        # A1: Meaning - based on description length and keywords
        self.vertices[1] = self._calc_meaning_score(description, name)

        # A2: Intent - based on tactic and kill chain position
        self.vertices[2] = self._calc_intent_score(tactic)

        # === TRIVARIATE β (OPERATIONAL): Phase, Intensity, Duration ===
        # A3: Phase - HD4 phase mapping (0.0=Hunt, 1.0=Dominate)
        self.vertices[3] = self._calc_phase_score(tactic)

        # A4: Intensity - based on severity and impact
        self.vertices[4] = self._calc_intensity_score(tactic, description)

        # A5: Duration - persistence characteristics
        self.vertices[5] = self._calc_duration_score(tactic, tech)

        # === TRIVARIATE γ (TEMPORAL): Historical, Current, Predictive ===
        # A6: Historical - based on technique age and known usage
        self.vertices[6] = self._calc_historical_score(mitre_id)

        # A7: Current - active threat level
        self.vertices[7] = self._calc_current_score(data_sources)

        # A8: Predictive - future risk projection
        self.vertices[8] = self._calc_predictive_score(tactic, platforms)

        # Quantize to 6 decimal places
        self.vertices = [round(v, PRECISION) for v in self.vertices]

        # Calculate center (weighted average)
        self._calculate_center()

        # Calculate confidence
        self.confidence = self._calculate_confidence()

    def _calc_context_score(self, platforms: List, description: str) -> float:
        """Context based on platforms and environment"""
        base = 0.3
        if platforms:
            base += 0.1 * min(len(platforms), 5)
        if "cloud" in description.lower():
            base += 0.1
        if "network" in description.lower():
            base += 0.1
        return min(base, 1.0)

    def _calc_meaning_score(self, description: str, name: str) -> float:
        """Meaning based on semantic content"""
        score = 0.2
        if description:
            # Longer descriptions = more semantic content
            score += min(len(description) / 500, 0.4)
        if name:
            score += 0.2
        return min(score, 1.0)

    def _calc_intent_score(self, tactic: str) -> float:
        """Intent based on kill chain position"""
        intent_map = {
            "reconnaissance": 0.1,
            "resource-development": 0.2,
            "initial-access": 0.3,
            "execution": 0.5,
            "persistence": 0.4,
            "privilege-escalation": 0.6,
            "defense-evasion": 0.5,
            "credential-access": 0.6,
            "discovery": 0.4,
            "lateral-movement": 0.7,
            "collection": 0.6,
            "command-and-control": 0.8,
            "exfiltration": 0.9,
            "impact": 1.0,
        }
        return intent_map.get(tactic.lower().replace(" ", "-"), 0.5)

    def _calc_phase_score(self, tactic: str) -> float:
        """HD4 Phase (Hunt→Detect→Disrupt→Disable→Dominate)"""
        phase_map = {
            "reconnaissance": 0.1,
            "resource-development": 0.15,
            "initial-access": 0.25,
            "execution": 0.4,
            "persistence": 0.35,
            "privilege-escalation": 0.5,
            "defense-evasion": 0.45,
            "credential-access": 0.55,
            "discovery": 0.3,
            "lateral-movement": 0.6,
            "collection": 0.65,
            "command-and-control": 0.75,
            "exfiltration": 0.85,
            "impact": 0.95,
        }
        return phase_map.get(tactic.lower().replace(" ", "-"), 0.5)

    def _calc_intensity_score(self, tactic: str, description: str) -> float:
        """Intensity based on severity"""
        base = self._calc_phase_score(tactic)
        # Boost for destructive keywords
        if any(kw in description.lower() for kw in ["destroy", "delete", "wipe", "ransom"]):
            base += 0.2
        return min(base, 1.0)

    def _calc_duration_score(self, tactic: str, tech: Dict) -> float:
        """Duration/persistence characteristics"""
        persistent_tactics = ["persistence", "command-and-control", "collection"]
        base = 0.3
        if tactic.lower().replace(" ", "-") in persistent_tactics:
            base = 0.7
        return base

    def _calc_historical_score(self, mitre_id: str) -> float:
        """Historical score based on technique ID age"""
        if not mitre_id:
            return 0.5
        # Older techniques (lower numbers) have more history
        try:
            num = int(''.join(filter(str.isdigit, mitre_id)))
            return max(0.3, min(1.0, 1.0 - (num / 2000)))
        except:
            return 0.5

    def _calc_current_score(self, data_sources: List) -> float:
        """Current threat level based on detection coverage"""
        base = 0.5
        if data_sources:
            base += 0.05 * min(len(data_sources), 10)
        return min(base, 1.0)

    def _calc_predictive_score(self, tactic: str, platforms: List) -> float:
        """Predictive score for future risk"""
        # Modern platforms = higher future risk
        modern_platforms = ["cloud", "containers", "saas", "iaas"]
        base = 0.4
        if platforms:
            for p in platforms:
                if any(mp in p.lower() for mp in modern_platforms):
                    base += 0.15
        return min(base, 1.0)

    def _calculate_center(self):
        """Calculate center as weighted average (RFC-9302 §4.1)"""
        weights = self.edges
        weighted_sum = sum(v * w for v, w in zip(self.vertices, weights))
        weight_sum = sum(weights)
        self.center = round(weighted_sum / weight_sum if weight_sum > 0 else 0.0, PRECISION)

    def _calculate_confidence(self) -> float:
        """Calculate overall confidence based on vertex coverage"""
        active = sum(1 for v in self.vertices if v > 0.1)
        return round(active / 9.0, PRECISION)

    def get_trivariate_alpha(self) -> Tuple[float, float, float]:
        """Get Semantic trivariate (Context, Meaning, Intent)"""
        return (self.vertices[0], self.vertices[1], self.vertices[2])

    def get_trivariate_beta(self) -> Tuple[float, float, float]:
        """Get Operational trivariate (Phase, Intensity, Duration)"""
        return (self.vertices[3], self.vertices[4], self.vertices[5])

    def get_trivariate_gamma(self) -> Tuple[float, float, float]:
        """Get Temporal trivariate (Historical, Current, Predictive)"""
        return (self.vertices[6], self.vertices[7], self.vertices[8])

    def vertex_position(self, idx: int) -> Tuple[float, float]:
        """Get vertex position on unit circle (for visualization)"""
        angle = 2 * math.pi * idx / 9
        return (round(math.cos(angle), PRECISION), round(math.sin(angle), PRECISION))

    def to_toml_section(self) -> str:
        """Generate TOML section for nine_sided"""
        alpha = self.get_trivariate_alpha()
        beta = self.get_trivariate_beta()
        gamma = self.get_trivariate_gamma()

        return f'''
[nine_sided]
# RFC-9302 Nonagon Analytic Node (NAN) - 9 vertices, 3 trivariates

# Trivariate α (Semantic)
alpha_x_context = {alpha[0]}
alpha_y_meaning = {alpha[1]}
alpha_z_intent = {alpha[2]}

# Trivariate β (Operational)
beta_x_phase = {beta[0]}
beta_y_intensity = {beta[1]}
beta_z_duration = {beta[2]}

# Trivariate γ (Temporal)
gamma_x_historical = {gamma[0]}
gamma_y_current = {gamma[1]}
gamma_z_predictive = {gamma[2]}

# Fusion
center = {self.center}
confidence = {self.confidence}

# Vertex positions (unit circle)
vertices = [{", ".join(f"{v:.6f}" for v in self.vertices)}]
edges = [{", ".join(f"{e:.6f}" for e in self.edges)}]'''


def get_hd4_level(tactic: str, severity: Optional[int] = None) -> int:
    """Map tactic to HD4 level (RFC-9300)"""
    if severity:
        return min(max(severity, 1), 16)

    # Map tactics to HD4 phases
    hunt_tactics = ["reconnaissance", "resource-development"]
    detect_tactics = ["initial-access", "discovery", "collection"]
    disrupt_tactics = ["execution", "persistence", "defense-evasion"]
    disable_tactics = ["privilege-escalation", "credential-access", "lateral-movement"]
    dominate_tactics = ["command-and-control", "exfiltration", "impact"]

    tactic_lower = tactic.lower().replace(" ", "-")

    if tactic_lower in hunt_tactics:
        return random.randint(*HD4_LEVELS["Hunt"])
    elif tactic_lower in detect_tactics:
        return random.randint(*HD4_LEVELS["Detect"])
    elif tactic_lower in disrupt_tactics:
        return random.randint(*HD4_LEVELS["Disrupt"])
    elif tactic_lower in disable_tactics:
        return random.randint(*HD4_LEVELS["Disable"])
    elif tactic_lower in dominate_tactics:
        return random.randint(*HD4_LEVELS["Dominate"])
    else:
        return random.randint(7, 12)  # Default to Detect/Disrupt


def get_primitive(tactic: str) -> str:
    """Map tactic to RFC-9001 primitive"""
    tactic_lower = tactic.lower().replace(" ", "-")
    return TACTIC_PRIMITIVE_MAP.get(tactic_lower, "OBSERVE")


def generate_toml_rule(
    rule_id: int,
    technique: Dict[str, Any],
    detection: Optional[Dict[str, Any]] = None,
    use_nonagon: bool = True
) -> str:
    """Generate a Plasma Defender TOML rule from technique data

    Args:
        rule_id: Unique rule identifier
        technique: MITRE technique data
        detection: Optional detection rule data
        use_nonagon: If True, use RFC-9302 NonagonNode for nine_sided section
    """

    # Extract fields
    name = technique.get("name", "Unknown Technique")
    description = technique.get("description", "")[:200] if technique.get("description") else name
    tactic = technique.get("tactic", "discovery")
    mitre_id = technique.get("id", technique.get("technique_id", ""))

    # Determine primitive and level
    primitive = get_primitive(tactic)
    level = get_hd4_level(tactic)
    unicode_trigger = UNICODE_TRIGGERS.get(primitive, "U+E400")

    # Generate regex pattern from detection if available
    if detection:
        regex = detection.get("regex", detection.get("pattern", f".*{name}.*"))
        countermeasures = detection.get("countermeasures", [])
    else:
        # Generate regex from technique name/description
        keywords = name.replace("-", " ").replace("_", " ").split()[:3]
        regex = f".*({'|'.join(keywords)}).*"
        countermeasures = ["ossec-active-response:log-alert", f"plasma-notify:{primitive.lower()}"]

    # Generate evasion tactics
    evasion_tactics = []
    if technique.get("platforms"):
        evasion_tactics.append(f"target platforms: {', '.join(technique.get('platforms', [])[:3])}")
    if technique.get("data_sources"):
        evasion_tactics.append(f"evade: {', '.join(technique.get('data_sources', [])[:2])}")
    if not evasion_tactics:
        evasion_tactics = ["obfuscation", "timing variation", "protocol tunneling"]

    # Generate dual trivariate hashes (RFC-9001 §2.2)
    # Primary (Tactical/Execution)
    sch_primary = generate_sch(f"{mitre_id}:{name}:{tactic}", "T")
    cuid_primary = generate_cuid(f"{rule_id}:{primitive}:{level}", "T")
    uuid_primary = generate_uuid(f"{mitre_id}:{rule_id}", "T")

    # Secondary (Semantic/Analysis)
    sch_secondary = generate_sch(f"{description}:{primitive}", "S")
    cuid_secondary = generate_cuid(f"{tactic}:{name}", "S")
    uuid_secondary = generate_uuid(f"{technique.get('url', '')}:{rule_id}", "S")

    # Build base TOML content
    toml_content = f'''[rule]
id = {rule_id}
level = {level}
description = "{description.replace('"', "'")}"
primitive = "{primitive}"
unicode_trigger = "{unicode_trigger}"
mitre_id = "{mitre_id}"

[1nf.indicators.plasma]
regex = "{regex.replace('"', "'")}"
countermeasures = {json.dumps(countermeasures[:3])}

[2nf.evasion]
tactics = {json.dumps(evasion_tactics[:3])}
'''

    # Add nine_sided section
    if use_nonagon:
        # RFC-9302 NonagonNode with full 9-vertex calculation
        nonagon = NonagonNode(rule_id, technique)
        toml_content += nonagon.to_toml_section()

        # Add trivariate hashes as additional context
        toml_content += f'''

# Trivariate Hashes (RFC-9001 §2.2)
sch_primary = "{sch_primary}"
cuid_primary = "{cuid_primary}"
uuid_primary = "{uuid_primary}"
sch_secondary = "{sch_secondary}"
cuid_secondary = "{cuid_secondary}"
uuid_secondary = "{uuid_secondary}"
'''
    else:
        # Legacy mode: just hashes
        toml_content += f'''
[nine_sided]
# Primary Trivariate (Tactical/Execution) - RFC-9001 §2.2
sch_primary = "{sch_primary}"
cuid_primary = "{cuid_primary}"
uuid_primary = "{uuid_primary}"

# Secondary Trivariate (Semantic/Analysis) - RFC-9001 §2.2
sch_secondary = "{sch_secondary}"
cuid_secondary = "{cuid_secondary}"
uuid_secondary = "{uuid_secondary}"
'''

    # Add active response
    toml_content += f'''
[rule.active_response]
command = "plasma-{primitive.lower()}"
location = "local"
level = {max(level - 2, 1)}
timeout = {300 + (level * 30)}
'''

    return toml_content


def load_threat_content() -> Dict[str, Any]:
    """Load threat content from output directory"""
    content = {
        "techniques": [],
        "detections": [],
        "sigma_rules": [],
        "atomic_tests": []
    }

    # Load MITRE ATT&CK techniques
    mitre_file = OUTPUT_DIR / "threat_content" / "mitre_attack.json"
    if mitre_file.exists():
        with open(mitre_file) as f:
            data = json.load(f)
            techniques = data.get("techniques", [])
            if isinstance(techniques, dict):
                content["techniques"] = list(techniques.values())
            else:
                content["techniques"] = techniques
            print(f"  Loaded {len(content['techniques'])} MITRE techniques")

    # Load Sigma rules
    sigma_index = OUTPUT_DIR / "threat_content" / "sigma_index.json"
    if sigma_index.exists():
        with open(sigma_index) as f:
            data = json.load(f)
            content["sigma_rules"] = data.get("rules", [])
            print(f"  Loaded {len(content['sigma_rules'])} Sigma rules")

    # Load Atomic Red Team
    atomic_file = OUTPUT_DIR / "threat_content" / "atomic_red_team.json"
    if atomic_file.exists():
        with open(atomic_file) as f:
            data = json.load(f)
            content["atomic_tests"] = data.get("tests", [])
            print(f"  Loaded {len(content['atomic_tests'])} Atomic tests")

    return content


def generate_ossec_toml_rules(max_rules: int = 1000) -> Dict[str, Any]:
    """Generate OSSEC TOML rules from threat content"""

    print("=" * 60)
    print("OSSEC TOML Rule Generator - RFC-9001 Compliant")
    print("=" * 60)

    # Create output directory
    TOML_OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    # Load threat content
    print("\n[1/3] Loading threat content...")
    content = load_threat_content()

    # Statistics
    stats = {
        "timestamp": datetime.now().isoformat(),
        "total_techniques": len(content["techniques"]),
        "total_sigma": len(content["sigma_rules"]),
        "total_atomic": len(content["atomic_tests"]),
        "rules_generated": 0,
        "primitives_used": {},
        "levels_distribution": {}
    }

    print(f"\n[2/3] Generating TOML rules...")

    rule_id = 60000  # Start from OSSEC custom rule range
    generated = []

    # Process MITRE techniques
    for tech in content["techniques"][:max_rules]:
        if not isinstance(tech, dict):
            continue

        try:
            toml_content = generate_toml_rule(rule_id, tech)

            # Save to file
            filename = f"{rule_id}.toml"
            filepath = TOML_OUTPUT_DIR / filename
            with open(filepath, "w") as f:
                f.write(toml_content)

            # Track statistics
            primitive = get_primitive(tech.get("tactic", "discovery"))
            level = get_hd4_level(tech.get("tactic", "discovery"))

            stats["primitives_used"][primitive] = stats["primitives_used"].get(primitive, 0) + 1
            stats["levels_distribution"][str(level)] = stats["levels_distribution"].get(str(level), 0) + 1

            generated.append({
                "rule_id": rule_id,
                "filename": filename,
                "technique": tech.get("name", "Unknown"),
                "mitre_id": tech.get("id", ""),
                "primitive": primitive,
                "level": level
            })

            rule_id += 1
            stats["rules_generated"] += 1

            if stats["rules_generated"] % 100 == 0:
                print(f"    Generated {stats['rules_generated']} rules...")

        except Exception as e:
            print(f"    Error processing technique: {e}")
            continue

    # Save statistics
    stats_file = TOML_OUTPUT_DIR / "generation_stats.json"
    with open(stats_file, "w") as f:
        json.dump(stats, f, indent=2)

    # Save rule index
    index_file = TOML_OUTPUT_DIR / "rule_index.json"
    with open(index_file, "w") as f:
        json.dump({
            "timestamp": stats["timestamp"],
            "total_rules": stats["rules_generated"],
            "rules": generated
        }, f, indent=2)

    print(f"\n[3/3] Summary")
    print(f"    Total rules generated: {stats['rules_generated']}")
    print(f"    Output directory: {TOML_OUTPUT_DIR}")
    print(f"\n    Primitives used:")
    for prim, count in sorted(stats["primitives_used"].items(), key=lambda x: -x[1])[:10]:
        print(f"      {prim}: {count}")
    print(f"\n    HD4 Level distribution:")
    for level, count in sorted(stats["levels_distribution"].items(), key=lambda x: int(x[0])):
        phase = "Hunt" if int(level) <= 6 else "Detect" if int(level) <= 10 else "Disrupt" if int(level) <= 13 else "Disable" if int(level) <= 15 else "Dominate"
        print(f"      Level {level} ({phase}): {count}")

    return stats


if __name__ == "__main__":
    stats = generate_ossec_toml_rules(max_rules=500)
    print(f"\n✅ OSSEC TOML generation complete!")
    print(f"   Files saved to: {TOML_OUTPUT_DIR}")
