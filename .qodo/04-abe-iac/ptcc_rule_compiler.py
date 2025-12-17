#!/usr/bin/env python3
"""
PTCC Rule Compiler - RFC-9100 Compliant
========================================

Compiles PTCC S-expression rules to:
1. Neo4j Cypher queries
2. OSSEC XML rules
3. Wazuh/PLASMA format
4. JSON threat graph

PTCC Primitives (Unicode U+E400-E41F):
- Observe (E410), Analyze (E40C), Correlate (E40D), Score (E40E)
- Lock (E414), Unlock (E415), Spawn (E416), Terminate (E417)
- Checkpoint (E418), Restore (E419), Sync (E41A), Merge (E41B)
- Alert (E41E), Log (E41D)

Usage:
    python ptcc_rule_compiler.py --input rules.ptcc --output-neo4j rules.cypher
    python ptcc_rule_compiler.py --input rules.ptcc --output-ossec rules.xml
    python ptcc_rule_compiler.py --compile-mitre  # Convert MITRE to PTCC
"""

import re
import json
import hashlib
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any, Tuple
from datetime import datetime, timezone
from enum import IntEnum

# ============================================================================
# PTCC PRIMITIVE DEFINITIONS (RFC-9100)
# ============================================================================

class Primitive(IntEnum):
    """PTCC Primitives mapped to Unicode PUA (U+E400-E41F)"""
    # CRUD (0x00-0x03)
    CREATE = 0x00
    READ = 0x01
    UPDATE = 0x02
    DELETE = 0x03
    # Data (0x04-0x07)
    TRANSFORM = 0x04
    VALIDATE = 0x05
    COMPRESS = 0x06
    ENCRYPT = 0x07
    # Network (0x08-0x0B)
    ROUTE = 0x08
    BROADCAST = 0x09
    SUBSCRIBE = 0x0A
    PUBLISH = 0x0B
    # Analysis (0x0C-0x0F)
    ANALYZE = 0x0C
    CORRELATE = 0x0D
    SCORE = 0x0E
    PREDICT = 0x0F
    # Cognitive/OODA (0x10-0x13)
    OBSERVE = 0x10
    ORIENT = 0x11
    DECIDE = 0x12
    ACT = 0x13
    # Control (0x14-0x17)
    LOCK = 0x14
    UNLOCK = 0x15
    SPAWN = 0x16
    TERMINATE = 0x17
    # State (0x18-0x1B)
    CHECKPOINT = 0x18
    RESTORE = 0x19
    SYNC = 0x1A
    MERGE = 0x1B
    # Meta (0x1C-0x1F)
    MEASURE = 0x1C
    LOG = 0x1D
    ALERT = 0x1E
    NOOP = 0x1F

    def to_unicode(self) -> str:
        """Convert to Unicode PUA character"""
        return chr(0xE400 + self.value)

    def to_hex(self) -> str:
        """Convert to hex string (E40C format)"""
        return f"E4{self.value:02X}"

    @classmethod
    def from_hex(cls, hex_str: str) -> 'Primitive':
        """Parse from hex string"""
        if hex_str.upper().startswith("E4"):
            val = int(hex_str[2:], 16)
            return cls(val)
        raise ValueError(f"Invalid PTCC hex: {hex_str}")

# HD4 Phase mapping
HD4_PHASES = {
    "hunt": [Primitive.OBSERVE, Primitive.READ],
    "detect": [Primitive.ANALYZE, Primitive.CORRELATE, Primitive.SCORE],
    "disable": [Primitive.LOCK, Primitive.TERMINATE],
    "disrupt": [Primitive.SPAWN, Primitive.ACT],
    "dominate": [Primitive.SYNC, Primitive.ALERT, Primitive.CHECKPOINT],
}

# ============================================================================
# S-EXPRESSION PARSER
# ============================================================================

@dataclass
class SExpr:
    """S-expression node"""
    op: str  # Primitive hex or keyword
    attrs: Dict[str, Any] = field(default_factory=dict)
    children: List['SExpr'] = field(default_factory=list)

    def to_dict(self) -> Dict:
        return {
            "op": self.op,
            "attrs": self.attrs,
            "children": [c.to_dict() for c in self.children]
        }

class SExprParser:
    """Parse PTCC S-expressions"""

    def __init__(self, text: str):
        self.text = text
        self.pos = 0

    def parse(self) -> List[SExpr]:
        """Parse all expressions"""
        exprs = []
        while self.pos < len(self.text):
            self._skip_whitespace()
            if self.pos < len(self.text) and self.text[self.pos] == '(':
                exprs.append(self._parse_expr())
            elif self.pos < len(self.text) and self.text[self.pos] == ';':
                self._skip_comment()
            else:
                break
        return exprs

    def _skip_whitespace(self):
        while self.pos < len(self.text) and self.text[self.pos] in ' \t\n\r':
            self.pos += 1

    def _skip_comment(self):
        while self.pos < len(self.text) and self.text[self.pos] != '\n':
            self.pos += 1

    def _parse_expr(self) -> SExpr:
        """Parse single S-expression"""
        assert self.text[self.pos] == '('
        self.pos += 1
        self._skip_whitespace()

        # Parse operator (E40C or keyword)
        op = self._parse_atom()
        attrs = {}
        children = []

        while True:
            self._skip_whitespace()
            if self.pos >= len(self.text):
                break
            if self.text[self.pos] == ')':
                self.pos += 1
                break
            if self.text[self.pos] == ';':
                self._skip_comment()
                continue
            if self.text[self.pos] == ':':
                # Keyword argument
                self.pos += 1
                key = self._parse_atom()
                self._skip_whitespace()
                val = self._parse_value()
                attrs[key] = val
            elif self.text[self.pos] == '(':
                # Child expression
                children.append(self._parse_expr())
            else:
                # Positional value (add to attrs)
                val = self._parse_value()
                if "value" not in attrs:
                    attrs["value"] = val

        return SExpr(op=op, attrs=attrs, children=children)

    def _parse_atom(self) -> str:
        """Parse identifier/keyword"""
        start = self.pos
        while self.pos < len(self.text) and self.text[self.pos] not in ' \t\n\r():;':
            self.pos += 1
        return self.text[start:self.pos]

    def _parse_value(self) -> Any:
        """Parse value (string, number, list, or atom)"""
        self._skip_whitespace()
        if self.pos >= len(self.text):
            return None

        ch = self.text[self.pos]

        if ch == '"':
            # String
            self.pos += 1
            start = self.pos
            while self.pos < len(self.text) and self.text[self.pos] != '"':
                if self.text[self.pos] == '\\':
                    self.pos += 1
                self.pos += 1
            result = self.text[start:self.pos]
            self.pos += 1
            return result

        if ch == '(':
            # Could be list or nested expr
            if self._peek_is_list():
                return self._parse_list()
            return self._parse_expr()

        # Atom (number or symbol)
        atom = self._parse_atom()
        if atom.isdigit() or (atom.startswith('-') and atom[1:].isdigit()):
            return int(atom)
        if re.match(r'^-?\d+\.\d+$', atom):
            return float(atom)
        if atom.lower() == 'true':
            return True
        if atom.lower() == 'false':
            return False
        return atom

    def _peek_is_list(self) -> bool:
        """Check if next ( starts a list vs expr"""
        save = self.pos
        self.pos += 1
        self._skip_whitespace()
        is_list = self.pos < len(self.text) and self.text[self.pos] not in 'E:('
        self.pos = save
        return is_list

    def _parse_list(self) -> List:
        """Parse list (x y z)"""
        assert self.text[self.pos] == '('
        self.pos += 1
        items = []
        while True:
            self._skip_whitespace()
            if self.pos >= len(self.text) or self.text[self.pos] == ')':
                self.pos += 1
                break
            items.append(self._parse_value())
        return items

# ============================================================================
# SCH HASH GENERATION (RFC-9001)
# ============================================================================

def generate_sch(rule_id: str, content: str) -> str:
    """Generate SCH (Synaptic Convergent Hash) for rule"""
    data = f"{rule_id}:{content}".encode('utf-8')
    # Murmur3-like hash
    h = 0x9001  # RFC-9001 seed
    for byte in data:
        h ^= byte
        h = (h * 0x5bd1e995) & 0xFFFFFFFFFFFFFFFF
        h ^= (h >> 47)
    return f"SCH{h:016x}"[:20]

# ============================================================================
# PTCC RULE
# ============================================================================

@dataclass
class PTCCRule:
    """Compiled PTCC Rule"""
    sch: str
    rule_id: str
    name: str
    hd4_phase: str
    primitives: List[Primitive]
    observe: Dict = field(default_factory=dict)
    analyze: Dict = field(default_factory=dict)
    correlate: Dict = field(default_factory=dict)
    score: Dict = field(default_factory=dict)
    controls: List[Dict] = field(default_factory=list)
    alert: Dict = field(default_factory=dict)
    mitre_technique: Optional[str] = None
    mitre_tactic: Optional[str] = None

    def to_dict(self) -> Dict:
        return {
            "sch": self.sch,
            "rule_id": self.rule_id,
            "name": self.name,
            "hd4_phase": self.hd4_phase,
            "primitives": [p.to_hex() for p in self.primitives],
            "observe": self.observe,
            "analyze": self.analyze,
            "correlate": self.correlate,
            "score": self.score,
            "controls": self.controls,
            "alert": self.alert,
            "mitre_technique": self.mitre_technique,
            "mitre_tactic": self.mitre_tactic,
        }

# ============================================================================
# COMPILER
# ============================================================================

class PTCCCompiler:
    """Compile S-expressions to PTCC rules"""

    def __init__(self):
        self.rules: List[PTCCRule] = []

    def compile(self, exprs: List[SExpr]) -> List[PTCCRule]:
        """Compile list of S-expressions"""
        for expr in exprs:
            rule = self._compile_expr(expr)
            if rule:
                self.rules.append(rule)
        return self.rules

    def _compile_expr(self, expr: SExpr) -> Optional[PTCCRule]:
        """Compile single expression to rule"""
        # Main expression should be Analyze (E40C)
        if not expr.op.upper().startswith("E4"):
            return None

        rule_id = expr.attrs.get("id", f"PTCC-{len(self.rules)+1:04d}")
        name = expr.attrs.get("name", expr.attrs.get("value", rule_id))
        sch = expr.attrs.get("sch", generate_sch(rule_id, str(expr.to_dict())))

        primitives = [Primitive.from_hex(expr.op)]
        observe = {}
        analyze = {}
        correlate = {}
        score = {}
        controls = []
        alert = {}
        mitre_technique = None
        mitre_tactic = None

        # Process children
        for child in expr.children:
            try:
                prim = Primitive.from_hex(child.op)
                primitives.append(prim)

                if prim == Primitive.OBSERVE:
                    observe = child.attrs
                elif prim == Primitive.ANALYZE:
                    analyze = child.attrs
                elif prim == Primitive.CORRELATE:
                    correlate = child.attrs
                elif prim == Primitive.SCORE:
                    score = child.attrs
                    mitre_technique = child.attrs.get("technique")
                    mitre_tactic = child.attrs.get("tactic")
                elif prim in (Primitive.LOCK, Primitive.UNLOCK,
                             Primitive.SPAWN, Primitive.TERMINATE,
                             Primitive.CHECKPOINT):
                    controls.append({"primitive": prim.to_hex(), **child.attrs})
                elif prim == Primitive.ALERT:
                    alert = child.attrs
            except (ValueError, KeyError):
                pass

        # Determine HD4 phase
        hd4_phase = self._determine_hd4_phase(primitives, controls)

        return PTCCRule(
            sch=sch,
            rule_id=rule_id,
            name=name,
            hd4_phase=hd4_phase,
            primitives=primitives,
            observe=observe,
            analyze=analyze,
            correlate=correlate,
            score=score,
            controls=controls,
            alert=alert,
            mitre_technique=mitre_technique,
            mitre_tactic=mitre_tactic,
        )

    def _determine_hd4_phase(self, primitives: List[Primitive],
                             controls: List[Dict]) -> str:
        """Determine primary HD4 phase"""
        if controls:
            ctrl_prims = [Primitive.from_hex(c["primitive"]) for c in controls]
            if Primitive.LOCK in ctrl_prims or Primitive.TERMINATE in ctrl_prims:
                return "disable"
            if Primitive.SPAWN in ctrl_prims:
                return "disrupt"
        if Primitive.CORRELATE in primitives:
            return "detect"
        if Primitive.OBSERVE in primitives:
            return "hunt"
        return "dominate"

# ============================================================================
# OUTPUT GENERATORS
# ============================================================================

def generate_neo4j(rules: List[PTCCRule]) -> str:
    """Generate Neo4j Cypher"""
    lines = [
        "// PTCC Rules - Neo4j Import",
        f"// Generated: {datetime.now(timezone.utc).isoformat()}",
        "// RFC-9100 PTCC Primitives with HD4 Phase Mapping",
        "",
        "// Indexes",
        "CREATE INDEX IF NOT EXISTS FOR (r:PTCCRule) ON (r.sch);",
        "CREATE INDEX IF NOT EXISTS FOR (r:PTCCRule) ON (r.rule_id);",
        "CREATE INDEX IF NOT EXISTS FOR (r:PTCCRule) ON (r.hd4_phase);",
        "",
    ]

    for rule in rules:
        prims = json.dumps([p.to_hex() for p in rule.primitives])
        controls = json.dumps(rule.controls)

        cypher = f"""
MERGE (r:PTCCRule {{sch: '{rule.sch}'}})
SET r.rule_id = '{rule.rule_id}',
    r.name = '{rule.name.replace("'", "\\'")}',
    r.hd4_phase = '{rule.hd4_phase}',
    r.primitives = {prims},
    r.mitre_technique = '{rule.mitre_technique or ""}',
    r.mitre_tactic = '{rule.mitre_tactic or ""}',
    r.level = {rule.score.get('level', 0)},
    r.controls = '{controls.replace("'", "\\'")}';
"""
        lines.append(cypher.strip())

        # Link to MITRE technique if exists
        if rule.mitre_technique:
            lines.append(f"""
MATCH (r:PTCCRule {{sch: '{rule.sch}'}}), (t:Technique {{id: '{rule.mitre_technique}'}})
MERGE (r)-[:DETECTS]->(t);
""".strip())

        lines.append("")

    return "\n".join(lines)


def generate_ossec_xml(rules: List[PTCCRule]) -> str:
    """Generate OSSEC XML rules"""
    lines = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<!-- PTCC Rules - OSSEC/Wazuh Format -->',
        f'<!-- Generated: {datetime.now(timezone.utc).isoformat()} -->',
        '<!-- RFC-9100 PTCC Primitives -->',
        '',
        '<group name="ptcc,">',
        '',
    ]

    for rule in rules:
        level = rule.score.get('level', 3)
        freq = rule.correlate.get('frequency', 1)
        timeframe = rule.correlate.get('timeframe', 60)

        lines.append(f'  <rule id="{rule.rule_id}" level="{level}">')
        lines.append(f'    <!-- SCH: {rule.sch} -->')
        lines.append(f'    <!-- HD4 Phase: {rule.hd4_phase} -->')
        lines.append(f'    <!-- Primitives: {", ".join(p.to_hex() for p in rule.primitives)} -->')

        if rule.observe.get('decoder'):
            lines.append(f'    <decoded_as>{rule.observe["decoder"]}</decoded_as>')

        if rule.analyze.get('pattern'):
            lines.append(f'    <pcre2>{rule.analyze["pattern"]}</pcre2>')

        if rule.correlate.get('if-matched'):
            lines.append(f'    <if_matched_sid>{rule.correlate["if-matched"]}</if_matched_sid>')
            lines.append(f'    <frequency>{freq}</frequency>')
            lines.append(f'    <timeframe>{timeframe}</timeframe>')

        if rule.correlate.get('key') == 'srcip':
            lines.append('    <same_source_ip />')

        if rule.mitre_technique:
            lines.append(f'    <mitre>')
            lines.append(f'      <id>{rule.mitre_technique}</id>')
            lines.append(f'    </mitre>')

        lines.append(f'    <description>{rule.name}</description>')
        lines.append(f'    <group>ptcc,{rule.hd4_phase},</group>')
        lines.append('  </rule>')
        lines.append('')

        # Generate active response for controls
        for ctrl in rule.controls:
            prim = ctrl.get('primitive', '')
            if prim == 'E414':  # Lock
                lines.append(f'  <active-response>')
                lines.append(f'    <command>firewall-drop</command>')
                lines.append(f'    <location>local</location>')
                lines.append(f'    <rules_id>{rule.rule_id}</rules_id>')
                lines.append(f'    <timeout>{ctrl.get("duration", 3600)}</timeout>')
                lines.append(f'  </active-response>')
                lines.append('')

    lines.append('</group>')
    return "\n".join(lines)


def generate_json(rules: List[PTCCRule]) -> Dict:
    """Generate JSON export"""
    return {
        "metadata": {
            "generated": datetime.now(timezone.utc).isoformat(),
            "format": "PTCC-RFC-9100",
            "version": "1.0",
            "rule_count": len(rules),
        },
        "hd4_summary": {
            phase: len([r for r in rules if r.hd4_phase == phase])
            for phase in ["hunt", "detect", "disable", "disrupt", "dominate"]
        },
        "rules": [r.to_dict() for r in rules],
    }

# ============================================================================
# MITRE CONVERTER
# ============================================================================

def convert_mitre_to_ptcc(mitre_file: Path) -> List[SExpr]:
    """Convert MITRE ATT&CK JSON to PTCC S-expressions"""
    with open(mitre_file) as f:
        data = json.load(f)

    exprs = []
    for obj in data.get("objects", []):
        if obj.get("type") != "attack-pattern":
            continue

        ext_refs = obj.get("external_references", [])
        tech_id = next((r.get("external_id") for r in ext_refs
                       if r.get("source_name") == "mitre-attack"), None)
        if not tech_id:
            continue

        name = obj.get("name", "")
        tactics = [p.get("phase_name") for p in obj.get("kill_chain_phases", [])]
        tactic = tactics[0] if tactics else "unknown"

        # Create PTCC S-expression
        expr = SExpr(
            op="E40C",  # Analyze
            attrs={
                "id": tech_id,
                "name": name,
            },
            children=[
                SExpr(op="E410", attrs={"source": "mitre-attack"}),  # Observe
                SExpr(op="E40E", attrs={  # Score
                    "technique": tech_id,
                    "tactic": tactic,
                    "level": 5,
                }),
                SExpr(op="E41E", attrs={"channel": "soc"}),  # Alert
            ]
        )
        exprs.append(expr)

    return exprs

# ============================================================================
# SAMPLE RULES
# ============================================================================

SAMPLE_RULES = """
;; SSH Brute Force Detection (OSSEC 5712 equivalent)
(E40C :id "PTCC-5712" :name "SSH Brute Force Attack"
  (E410 :decoder "sshd"
    :fields (srcip user port))
  (E40D :if-matched 5710
    :frequency 6 :timeframe 120
    :key srcip)
  (E40E :level 10
    :tactic "credential-access"
    :technique "T1110")
  (E414 :target srcip
    :duration 3600
    :firewall "iptables")
  (E416 :agent "incident-responder"
    :playbook "ssh-brute-force")
  (E418 :snapshot "pre-response")
  (E41E :channel "soc"
    :priority "high"
    :ticket true))

;; Process Injection Detection
(E40C :id "PTCC-T1055" :name "Process Injection Detected"
  (E410 :decoder "sysmon"
    :fields (srcip process parent_process))
  (E40C :pattern "CreateRemoteThread|NtMapViewOfSection")
  (E40E :level 12
    :tactic "defense-evasion"
    :technique "T1055")
  (E417 :target process
    :method "terminate")
  (E41E :channel "soc"
    :priority "critical"))

;; Lateral Movement via WMI
(E40C :id "PTCC-T1047" :name "WMI Lateral Movement"
  (E410 :decoder "windows-security"
    :event_id 4648)
  (E40D :if-matched "PTCC-T1078"
    :frequency 3 :timeframe 300
    :key srcip)
  (E40E :level 10
    :tactic "lateral-movement"
    :technique "T1047")
  (E414 :target srcip
    :duration 7200)
  (E41E :channel "soc"))
"""

# ============================================================================
# MAIN
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description="PTCC Rule Compiler")
    parser.add_argument("--input", "-i", help="Input .ptcc file")
    parser.add_argument("--output-neo4j", help="Output Neo4j Cypher file")
    parser.add_argument("--output-ossec", help="Output OSSEC XML file")
    parser.add_argument("--output-json", help="Output JSON file")
    parser.add_argument("--compile-mitre", help="Convert MITRE JSON to PTCC")
    parser.add_argument("--sample", action="store_true", help="Compile sample rules")

    args = parser.parse_args()

    print("=" * 70)
    print("PTCC RULE COMPILER - RFC-9100")
    print("Unicode Primitives + SCH Hashing + HD4 Phases")
    print("=" * 70)

    # Determine input source
    if args.compile_mitre:
        print(f"\nConverting MITRE: {args.compile_mitre}")
        exprs = convert_mitre_to_ptcc(Path(args.compile_mitre))
    elif args.input:
        print(f"\nParsing: {args.input}")
        with open(args.input) as f:
            text = f.read()
        exprs = SExprParser(text).parse()
    elif args.sample:
        print("\nCompiling sample rules...")
        exprs = SExprParser(SAMPLE_RULES).parse()
    else:
        print("\nNo input specified, using sample rules...")
        exprs = SExprParser(SAMPLE_RULES).parse()

    print(f"Parsed {len(exprs)} expressions")

    # Compile
    compiler = PTCCCompiler()
    rules = compiler.compile(exprs)
    print(f"Compiled {len(rules)} rules")

    # HD4 breakdown
    hd4_counts = {}
    for rule in rules:
        hd4_counts[rule.hd4_phase] = hd4_counts.get(rule.hd4_phase, 0) + 1
    print("\nHD4 Phase Distribution:")
    for phase in ["hunt", "detect", "disable", "disrupt", "dominate"]:
        print(f"  {phase:10s}: {hd4_counts.get(phase, 0):4d} rules")

    # Output
    output_dir = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac/output/ontology")

    if args.output_neo4j:
        out_path = Path(args.output_neo4j)
    else:
        out_path = output_dir / "ptcc_rules.cypher"
    cypher = generate_neo4j(rules)
    with open(out_path, 'w') as f:
        f.write(cypher)
    print(f"\nNeo4j Cypher: {out_path}")

    if args.output_ossec:
        out_path = Path(args.output_ossec)
    else:
        out_path = output_dir / "ptcc_rules.xml"
    ossec = generate_ossec_xml(rules)
    with open(out_path, 'w') as f:
        f.write(ossec)
    print(f"OSSEC XML: {out_path}")

    if args.output_json:
        out_path = Path(args.output_json)
    else:
        out_path = output_dir / "ptcc_rules.json"
    json_data = generate_json(rules)
    with open(out_path, 'w') as f:
        json.dump(json_data, f, indent=2)
    print(f"JSON: {out_path}")

    # Show sample output
    print("\n" + "=" * 70)
    print("SAMPLE OUTPUT")
    print("=" * 70)
    if rules:
        r = rules[0]
        print(f"\nRule: {r.rule_id} ({r.name})")
        print(f"SCH: {r.sch}")
        print(f"HD4 Phase: {r.hd4_phase}")
        print(f"Primitives: {', '.join(p.to_hex() for p in r.primitives)}")
        print(f"MITRE: {r.mitre_technique} ({r.mitre_tactic})")
        print(f"Controls: {len(r.controls)}")

if __name__ == "__main__":
    main()
