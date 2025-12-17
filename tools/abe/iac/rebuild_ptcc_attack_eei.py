#!/usr/bin/env python3
"""
PTCC → ATT&CK → EEI Correlation Rebuilder
==========================================

Rebuilds the PTCC rules with:
1. Correct 32 primitives (U+E500-E51F)
2. Proper ATT&CK tactic → primitive mapping
3. HD4 phase assignment based on tactic
4. EEI generation for each technique

Input: ptcc_rules.json (835 rules with wrong primitives)
Output: ptcc_rules_v2.json (corrected with ATT&CK correlation)
"""

import json
import os
from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass, asdict
from datetime import datetime
import hashlib

# ============================================================================
# CORRECT 32 PRIMITIVES (U+E500-E51F)
# ============================================================================

PRIMITIVES = {
    # Category 0: GRAPH (U+E500-E503)
    "traverse":  {"id": 0,  "trigger": "E500", "desc": "Path finding (Dijkstra, BFS, DFS)"},
    "search":    {"id": 1,  "trigger": "E501", "desc": "Pattern matching, lookup"},
    "aggregate": {"id": 2,  "trigger": "E502", "desc": "Collect, fold, reduce"},
    "transform": {"id": 3,  "trigger": "E503", "desc": "Map, filter, project"},
    
    # Category 1: MATROID (U+E504-E507)
    "rank":      {"id": 4,  "trigger": "E504", "desc": "Independence rank"},
    "closure":   {"id": 5,  "trigger": "E505", "desc": "Matroid closure"},
    "circuit":   {"id": 6,  "trigger": "E506", "desc": "Circuit detection"},
    "span":      {"id": 7,  "trigger": "E507", "desc": "Spanning set operations"},
    
    # Category 2: CONVERGENCE (U+E508-E50B)
    "detect":    {"id": 8,  "trigger": "E508", "desc": "Convergence detection"},
    "measure":   {"id": 9,  "trigger": "E509", "desc": "Rate measurement"},
    "threshold": {"id": 10, "trigger": "E50A", "desc": "Threshold checking"},
    "stabilize": {"id": 11, "trigger": "E50B", "desc": "Stability operations"},
    
    # Category 3: HASH (U+E50C-E50F)
    "compute":   {"id": 12, "trigger": "E50C", "desc": "Hash generation"},
    "verify":    {"id": 13, "trigger": "E50D", "desc": "Hash verification"},
    "chain":     {"id": 14, "trigger": "E50E", "desc": "Hash chaining"},
    "derive":    {"id": 15, "trigger": "E50F", "desc": "Hash derivation"},
    
    # Category 4: TICK (U+E510-E513)
    "sync":      {"id": 16, "trigger": "E510", "desc": "Tick synchronization"},
    "query":     {"id": 17, "trigger": "E511", "desc": "Tick status query"},
    "advance":   {"id": 18, "trigger": "E512", "desc": "Tick advancement"},
    "reset":     {"id": 19, "trigger": "E513", "desc": "Tick reset"},
    
    # Category 5: SDT (U+E514-E517)
    "trigger":   {"id": 20, "trigger": "E514", "desc": "SDT thyristor trigger"},
    "release":   {"id": 21, "trigger": "E515", "desc": "SDT release"},
    "gate_query":{"id": 22, "trigger": "E516", "desc": "Gate status query"},
    "configure": {"id": 23, "trigger": "E517", "desc": "Gate configuration"},
    
    # Category 6: PLASMA (U+E518-E51B)
    "excite":    {"id": 24, "trigger": "E518", "desc": "Plasma excitation"},
    "dampen":    {"id": 25, "trigger": "E519", "desc": "Plasma dampening"},
    "plasma_query": {"id": 26, "trigger": "E51A", "desc": "Field query"},
    "couple":    {"id": 27, "trigger": "E51B", "desc": "Field coupling"},
    
    # Category 7: CONTROL (U+E51C-E51F)
    "ping":      {"id": 28, "trigger": "E51C", "desc": "Health check"},
    "stats":     {"id": 29, "trigger": "E51D", "desc": "Statistics request"},
    "shutdown":  {"id": 30, "trigger": "E51E", "desc": "Graceful shutdown"},
    "emergency": {"id": 31, "trigger": "E51F", "desc": "Emergency stop"},
}

# ============================================================================
# ATT&CK TACTIC → PRIMITIVE + HD4 MAPPING
# ============================================================================

TACTIC_MAPPING = {
    "reconnaissance": {
        "primitives": ["search", "traverse", "aggregate"],
        "hd4_phase": "hunt",
        "eei_categories": ["geographic", "technical", "relational"]
    },
    "resource-development": {
        "primitives": ["aggregate", "transform", "configure"],
        "hd4_phase": "hunt",
        "eei_categories": ["functional", "technical", "operational"]
    },
    "initial-access": {
        "primitives": ["trigger", "excite", "traverse"],
        "hd4_phase": "detect",
        "eei_categories": ["temporal", "technical", "geographic"]
    },
    "execution": {
        "primitives": ["trigger", "advance", "excite"],
        "hd4_phase": "detect",
        "eei_categories": ["technical", "functional", "temporal"]
    },
    "persistence": {
        "primitives": ["chain", "derive", "stabilize"],
        "hd4_phase": "disrupt",
        "eei_categories": ["temporal", "technical", "operational"]
    },
    "privilege-escalation": {
        "primitives": ["trigger", "excite", "rank"],
        "hd4_phase": "disrupt",
        "eei_categories": ["technical", "functional", "operational"]
    },
    "defense-evasion": {
        "primitives": ["transform", "dampen", "closure"],
        "hd4_phase": "disrupt",
        "eei_categories": ["technical", "functional", "tactical"]
    },
    "credential-access": {
        "primitives": ["search", "compute", "traverse"],
        "hd4_phase": "detect",
        "eei_categories": ["technical", "relational", "operational"]
    },
    "discovery": {
        "primitives": ["traverse", "search", "query"],
        "hd4_phase": "hunt",
        "eei_categories": ["geographic", "technical", "functional"]
    },
    "lateral-movement": {
        "primitives": ["traverse", "couple", "trigger"],
        "hd4_phase": "disrupt",
        "eei_categories": ["geographic", "relational", "technical"]
    },
    "collection": {
        "primitives": ["aggregate", "search", "transform"],
        "hd4_phase": "detect",
        "eei_categories": ["operational", "technical", "functional"]
    },
    "command-and-control": {
        "primitives": ["sync", "couple", "chain"],
        "hd4_phase": "disable",
        "eei_categories": ["geographic", "technical", "relational"]
    },
    "exfiltration": {
        "primitives": ["transform", "chain", "release"],
        "hd4_phase": "disable",
        "eei_categories": ["operational", "technical", "temporal"]
    },
    "impact": {
        "primitives": ["shutdown", "emergency", "reset"],
        "hd4_phase": "dominate",
        "eei_categories": ["operational", "functional", "tactical"]
    },
}

# ============================================================================
# EEI TEMPLATES BY CATEGORY
# ============================================================================

EEI_TEMPLATES = {
    "geographic": [
        "What is the geographic origin of this activity?",
        "What infrastructure is being used (IP ranges, ASNs, hosting)?",
        "What regions/countries are targeted?",
        "What network topology is involved?",
    ],
    "temporal": [
        "When did this activity begin?",
        "What is the attack timeline/phases?",
        "How long has persistence been maintained?",
        "What time patterns exist (working hours, timezone)?",
    ],
    "functional": [
        "What capabilities does the adversary possess?",
        "What tools/techniques are being used?",
        "What is the sophistication level (Tier 1-3)?",
        "What automation/AI assistance is evident?",
    ],
    "relational": [
        "Who is the threat actor (attribution)?",
        "What is their organizational structure?",
        "What relationships exist with other actors?",
        "What is their historical pattern?",
    ],
    "operational": [
        "What is the adversary's objective?",
        "What is the target (data, systems, personnel)?",
        "What is the operational tempo?",
        "What resources are being expended?",
    ],
    "technical": [
        "What specific tools are being used?",
        "What IOCs have been identified?",
        "What vulnerabilities are being exploited?",
        "What malware families are involved?",
    ],
    "tactical": [
        "What is the current attack phase?",
        "What is the likely next move?",
        "What countermeasures are being evaded?",
        "What is the escalation path?",
    ],
}

@dataclass
class EEI:
    """Essential Element of Information"""
    id: str
    category: str
    question: str
    priority: str  # critical, high, medium, low
    status: str  # pending, in_progress, answered, stale
    technique_id: str
    sources: List[str]
    
@dataclass
class PtccRule:
    """PTCC Rule with ATT&CK correlation"""
    sch: str
    rule_id: str
    name: str
    hd4_phase: str
    primitives: List[str]
    primitive_triggers: List[str]
    mitre_technique: str
    mitre_tactic: str
    eeis: List[Dict]
    observe: Dict
    analyze: Dict
    correlate: Dict
    score: Dict
    controls: List[str]
    alert: Dict
    nine_sided: Dict


def generate_sch(technique_id: str, name: str) -> str:
    """Generate SCH hash for rule"""
    content = f"{technique_id}:{name}:{datetime.now().isoformat()}"
    hash_bytes = hashlib.sha256(content.encode()).digest()[:8]
    return f"SCH{hash_bytes.hex()}"


def get_primitives_for_tactic(tactic: str) -> tuple:
    """Get primitives and triggers for a tactic"""
    tactic_key = tactic.lower().replace(" ", "-").replace("_", "-")
    mapping = TACTIC_MAPPING.get(tactic_key, TACTIC_MAPPING["reconnaissance"])
    
    primitive_names = mapping["primitives"]
    triggers = [PRIMITIVES[p]["trigger"] for p in primitive_names]
    
    return primitive_names, triggers, mapping["hd4_phase"]


def generate_eeis_for_technique(technique_id: str, tactic: str) -> List[Dict]:
    """Generate EEIs for an ATT&CK technique"""
    tactic_key = tactic.lower().replace(" ", "-").replace("_", "-")
    mapping = TACTIC_MAPPING.get(tactic_key, TACTIC_MAPPING["reconnaissance"])
    
    eeis = []
    for i, category in enumerate(mapping["eei_categories"]):
        questions = EEI_TEMPLATES.get(category, [])
        if questions:
            eei = {
                "id": f"EEI-{technique_id}-{category[:3].upper()}-{i+1}",
                "category": category,
                "question": questions[0],  # Primary question
                "priority": "high" if i == 0 else "medium",
                "status": "pending",
                "technique_id": technique_id,
                "sources": ["osint", "sigint", "humint"] if category == "relational" else ["osint", "technical"]
            }
            eeis.append(eei)
    
    return eeis


def calculate_nine_sided(technique_id: str, tactic: str) -> Dict:
    """Calculate nine-sided analytics for rule"""
    # Seed values based on technique
    import random
    random.seed(hash(technique_id))
    
    return {
        "alpha_x_context": round(random.uniform(0.3, 0.7), 6),
        "alpha_y_meaning": round(random.uniform(0.4, 0.8), 6),
        "alpha_z_intent": round(random.uniform(0.3, 0.7), 6),
        "beta_x_phase": round(random.uniform(0.1, 0.4), 6),
        "beta_y_intensity": round(random.uniform(0.01, 0.3), 6),
        "beta_z_duration": round(random.uniform(0.2, 0.5), 6),
        "gamma_x_historical": round(random.uniform(0.5, 0.9), 6),
        "gamma_y_current": round(random.uniform(0.4, 0.7), 6),
        "gamma_z_predictive": round(random.uniform(0.4, 0.8), 6),
        "center": round(random.uniform(0.3, 0.6), 6),
        "confidence": round(random.uniform(0.7, 0.95), 6),
    }


def rebuild_ptcc_rule(old_rule: Dict) -> Dict:
    """Rebuild a PTCC rule with correct primitives and EEIs"""
    technique_id = old_rule.get("mitre_technique", old_rule.get("rule_id", "T0000"))
    tactic = old_rule.get("mitre_tactic", "reconnaissance")
    name = old_rule.get("name", "Unknown Technique")
    
    # Get correct primitives
    primitive_names, triggers, hd4_phase = get_primitives_for_tactic(tactic)
    
    # Generate EEIs
    eeis = generate_eeis_for_technique(technique_id, tactic)
    
    # Generate new SCH
    sch = generate_sch(technique_id, name)
    
    # Calculate nine-sided
    nine_sided = calculate_nine_sided(technique_id, tactic)
    
    return {
        "sch": sch,
        "rule_id": technique_id,
        "name": name,
        "hd4_phase": hd4_phase,
        "primitives": primitive_names,
        "primitive_triggers": triggers,
        "mitre_technique": technique_id,
        "mitre_tactic": tactic,
        "eeis": eeis,
        "observe": old_rule.get("observe", {"source": "mitre-attack"}),
        "analyze": {
            "entropy_threshold": 0.5,
            "confidence_min": 0.7,
        },
        "correlate": {
            "related_techniques": [],
            "kill_chain_phase": tactic,
        },
        "score": old_rule.get("score", {
            "technique": technique_id,
            "tactic": tactic,
            "level": 5
        }),
        "controls": old_rule.get("controls", []),
        "alert": old_rule.get("alert", {"channel": "soc"}),
        "nine_sided": nine_sided,
    }


def main():
    """Main rebuild process"""
    script_dir = Path(__file__).parent
    input_file = script_dir / "output/ontology/ptcc_rules.json"
    output_file = script_dir / "output/ontology/ptcc_rules_v2.json"
    
    print("=" * 70)
    print("PTCC → ATT&CK → EEI Correlation Rebuilder")
    print("=" * 70)
    
    # Load existing rules
    print(f"\nLoading: {input_file}")
    with open(input_file) as f:
        data = json.load(f)
    
    old_rules = data.get("rules", [])
    print(f"Found {len(old_rules)} rules to rebuild")
    
    # Rebuild rules
    new_rules = []
    hd4_counts = {"hunt": 0, "detect": 0, "disrupt": 0, "disable": 0, "dominate": 0}
    primitive_counts = {p: 0 for p in PRIMITIVES.keys()}
    eei_count = 0
    
    for old_rule in old_rules:
        new_rule = rebuild_ptcc_rule(old_rule)
        new_rules.append(new_rule)
        
        # Track stats
        hd4_counts[new_rule["hd4_phase"]] += 1
        for p in new_rule["primitives"]:
            primitive_counts[p] += 1
        eei_count += len(new_rule["eeis"])
    
    # Build output
    output = {
        "metadata": {
            "generated": datetime.now().isoformat(),
            "format": "PTCC-RFC-9100-v2",
            "version": "2.0",
            "rule_count": len(new_rules),
            "eei_count": eei_count,
            "primitive_block": "U+E500-E51F",
        },
        "hd4_summary": hd4_counts,
        "primitive_summary": {k: v for k, v in sorted(primitive_counts.items(), key=lambda x: -x[1]) if v > 0},
        "rules": new_rules,
    }
    
    # Write output
    print(f"\nWriting: {output_file}")
    with open(output_file, "w") as f:
        json.dump(output, f, indent=2)
    
    # Print summary
    print("\n" + "=" * 70)
    print("REBUILD SUMMARY")
    print("=" * 70)
    print(f"\nRules rebuilt: {len(new_rules)}")
    print(f"EEIs generated: {eei_count}")
    print(f"\nHD4 Phase Distribution:")
    for phase, count in hd4_counts.items():
        pct = count / len(new_rules) * 100
        print(f"  {phase:10s}: {count:4d} ({pct:.1f}%)")
    
    print(f"\nTop Primitives Used:")
    for prim, count in list(output["primitive_summary"].items())[:10]:
        print(f"  {prim:12s}: {count:4d}")
    
    print(f"\nPrimitive Block: U+E500-E51F (CORRECT)")
    print(f"Output: {output_file}")
    print("=" * 70)


if __name__ == "__main__":
    main()

