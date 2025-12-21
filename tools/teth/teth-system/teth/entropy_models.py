"""
TETH - Tool Entropy Testing Harness
Core Entropy Models and Tool Database

Mathematical foundation for quantifying tool complexity through
multiple dimensions: branching, cognitive load, risk, and variability.
"""

from __future__ import annotations
import math
from enum import Enum
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple


# ═══════════════════════════════════════════════════════════════════════════════
# ENUMS
# ═══════════════════════════════════════════════════════════════════════════════

class ToolCategory(Enum):
    RECONNAISSANCE = "reconnaissance"
    INITIAL_ACCESS = "initial_access"
    EXECUTION = "execution"
    PERSISTENCE = "persistence"
    PRIVILEGE_ESCALATION = "privilege_escalation"
    DEFENSE_EVASION = "defense_evasion"
    CREDENTIAL_ACCESS = "credential_access"
    DISCOVERY = "discovery"
    LATERAL_MOVEMENT = "lateral_movement"
    COLLECTION = "collection"
    COMMAND_CONTROL = "command_control"
    EXFILTRATION = "exfiltration"
    IMPACT = "impact"


class HD4Phase(Enum):
    """Hunt, Detect, Disrupt, Disable, Dominate"""
    HUNT = "hunt"
    DETECT = "detect"
    DISRUPT = "disrupt"
    DISABLE = "disable"
    DOMINATE = "dominate"


class PersonaLevel(Enum):
    SCRIPT_KIDDIE = "script_kiddie"
    CARTEL = "cartel"
    APT = "apt"
    NATION_STATE = "nation_state"


class APTGroup(Enum):
    APT28 = "apt28"
    APT29 = "apt29"
    LAZARUS = "lazarus"
    APT1 = "apt1"
    FIN7 = "fin7"


# ═══════════════════════════════════════════════════════════════════════════════
# DATA CLASSES
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class ToolProperties:
    """Multi-dimensional entropy properties for a tool."""
    branching_paths: int = 100
    cognitive_load: float = 5.0
    variability: float = 5.0
    operational_risk: float = 0.5
    feedback_clarity: float = 0.7


@dataclass
class Tool:
    """A cyber tool with entropy signature."""
    id: str
    name: str
    category: ToolCategory
    hd4_phase: HD4Phase
    persona_min: PersonaLevel
    properties: ToolProperties
    mitre: List[str] = field(default_factory=list)
    apt_exclusive: Optional[str] = None
    
    @property
    def branching(self) -> int:
        return self.properties.branching_paths
    
    @property
    def cognitive(self) -> float:
        return self.properties.cognitive_load
    
    @property
    def variability(self) -> float:
        return self.properties.variability
    
    @property
    def risk(self) -> float:
        return self.properties.operational_risk
    
    @property
    def feedback(self) -> float:
        return self.properties.feedback_clarity
    
    def calculate_entropy(self) -> Tuple[float, float]:
        """
        Calculate composite entropy with uncertainty bounds.
        
        Returns: (entropy_score, uncertainty)
        """
        # Base entropy from branching complexity
        base = math.log2(max(1, self.branching))
        
        # Cognitive factor with variability interaction
        cognitive = self.cognitive * (1 + 0.1 * self.variability)
        
        # Risk amplification when feedback is poor
        risk_factor = self.risk * (2.0 if self.feedback < 0.5 else 1.0)
        
        # Interaction penalty
        interaction = 0.2 * base * cognitive
        
        # Composite score
        entropy = base + cognitive + risk_factor + interaction
        
        # Uncertainty based on variability
        uncertainty = self.variability * 0.5
        
        return entropy, uncertainty


@dataclass
class Persona:
    """An operator with entropy tolerance range."""
    id: str
    name: str
    level: PersonaLevel
    experience_hours: int = 1000
    
    @property
    def min_entropy(self) -> float:
        ranges = {
            PersonaLevel.SCRIPT_KIDDIE: 0,
            PersonaLevel.CARTEL: 10,
            PersonaLevel.APT: 20,
            PersonaLevel.NATION_STATE: 30,
        }
        return ranges.get(self.level, 0)
    
    @property
    def max_entropy(self) -> float:
        ranges = {
            PersonaLevel.SCRIPT_KIDDIE: 15,
            PersonaLevel.CARTEL: 25,
            PersonaLevel.APT: 35,
            PersonaLevel.NATION_STATE: 50,
        }
        return ranges.get(self.level, 15)
    
    @property
    def optimal_entropy(self) -> float:
        return (self.min_entropy + self.max_entropy) / 2


@dataclass
class MatchResult:
    """Result of persona-tool matching."""
    compatibility_score: float
    success_probability: float
    entropy_delta: float
    recommendation: str


# ═══════════════════════════════════════════════════════════════════════════════
# TOOL DATABASE
# ═══════════════════════════════════════════════════════════════════════════════

def _create_tool(
    id: str, name: str, category: ToolCategory, hd4: HD4Phase,
    persona: PersonaLevel, branching: int, cognitive: float,
    variability: float, risk: float, feedback: float,
    mitre: List[str], apt_exclusive: Optional[str] = None
) -> Tool:
    return Tool(
        id=id, name=name, category=category, hd4_phase=hd4,
        persona_min=persona,
        properties=ToolProperties(
            branching_paths=branching,
            cognitive_load=cognitive,
            variability=variability,
            operational_risk=risk,
            feedback_clarity=feedback
        ),
        mitre=mitre,
        apt_exclusive=apt_exclusive
    )


TOOL_DATABASE: Dict[str, Tool] = {
    # ═══════════════════════════════════════════════════════════════════════════
    # RECONNAISSANCE
    # ═══════════════════════════════════════════════════════════════════════════
    "nmap": _create_tool(
        "nmap", "Nmap", ToolCategory.RECONNAISSANCE, HD4Phase.HUNT,
        PersonaLevel.SCRIPT_KIDDIE, 1000, 3.0, 2.0, 0.2, 0.9,
        ["T1046", "T1595"]
    ),
    "shodan": _create_tool(
        "shodan", "Shodan", ToolCategory.RECONNAISSANCE, HD4Phase.HUNT,
        PersonaLevel.SCRIPT_KIDDIE, 500, 2.0, 3.0, 0.1, 0.95,
        ["T1596"]
    ),
    "masscan": _create_tool(
        "masscan", "Masscan", ToolCategory.RECONNAISSANCE, HD4Phase.HUNT,
        PersonaLevel.SCRIPT_KIDDIE, 100, 2.5, 1.5, 0.3, 0.85,
        ["T1046"]
    ),
    "recon_ng": _create_tool(
        "recon_ng", "Recon-ng", ToolCategory.RECONNAISSANCE, HD4Phase.HUNT,
        PersonaLevel.CARTEL, 5000, 5.0, 4.0, 0.15, 0.8,
        ["T1592", "T1589"]
    ),
    "theharvester": _create_tool(
        "theharvester", "theHarvester", ToolCategory.RECONNAISSANCE, HD4Phase.HUNT,
        PersonaLevel.SCRIPT_KIDDIE, 200, 2.0, 2.5, 0.1, 0.9,
        ["T1589"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # INITIAL ACCESS
    # ═══════════════════════════════════════════════════════════════════════════
    "phishing_kit": _create_tool(
        "phishing_kit", "Phishing Kit", ToolCategory.INITIAL_ACCESS, HD4Phase.DISRUPT,
        PersonaLevel.SCRIPT_KIDDIE, 100, 4.0, 6.0, 0.4, 0.7,
        ["T1566"]
    ),
    "exploit_pack": _create_tool(
        "exploit_pack", "Exploit Pack", ToolCategory.INITIAL_ACCESS, HD4Phase.DISRUPT,
        PersonaLevel.CARTEL, 10000, 6.0, 5.0, 0.5, 0.6,
        ["T1190", "T1203"]
    ),
    "spearphish_attach": _create_tool(
        "spearphish_attach", "Spearphish Attachment", ToolCategory.INITIAL_ACCESS, HD4Phase.DISRUPT,
        PersonaLevel.APT, 500, 7.0, 7.0, 0.6, 0.5,
        ["T1566.001"]
    ),
    "watering_hole": _create_tool(
        "watering_hole", "Watering Hole", ToolCategory.INITIAL_ACCESS, HD4Phase.DISRUPT,
        PersonaLevel.APT, 5000, 8.0, 6.0, 0.4, 0.4,
        ["T1189"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # EXECUTION
    # ═══════════════════════════════════════════════════════════════════════════
    "powershell_empire": _create_tool(
        "powershell_empire", "PowerShell Empire", ToolCategory.EXECUTION, HD4Phase.DISRUPT,
        PersonaLevel.CARTEL, 50000, 6.5, 5.0, 0.5, 0.7,
        ["T1059.001"]
    ),
    "metasploit": _create_tool(
        "metasploit", "Metasploit", ToolCategory.EXECUTION, HD4Phase.DISRUPT,
        PersonaLevel.CARTEL, 100000, 7.0, 4.0, 0.4, 0.8,
        ["T1059", "T1203"]
    ),
    "cobalt_strike": _create_tool(
        "cobalt_strike", "Cobalt Strike", ToolCategory.EXECUTION, HD4Phase.DISABLE,
        PersonaLevel.APT, 500000, 8.0, 3.0, 0.3, 0.85,
        ["T1059", "T1071"]
    ),
    "koadic": _create_tool(
        "koadic", "Koadic", ToolCategory.EXECUTION, HD4Phase.DISRUPT,
        PersonaLevel.CARTEL, 10000, 5.5, 4.0, 0.4, 0.75,
        ["T1059.007"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # CREDENTIAL ACCESS
    # ═══════════════════════════════════════════════════════════════════════════
    "mimikatz": _create_tool(
        "mimikatz", "Mimikatz", ToolCategory.CREDENTIAL_ACCESS, HD4Phase.DISABLE,
        PersonaLevel.CARTEL, 10000, 5.5, 2.0, 0.6, 0.9,
        ["T1003", "T1558"]
    ),
    "hashcat": _create_tool(
        "hashcat", "Hashcat", ToolCategory.CREDENTIAL_ACCESS, HD4Phase.DETECT,
        PersonaLevel.SCRIPT_KIDDIE, 1000, 4.0, 2.0, 0.1, 0.95,
        ["T1110"]
    ),
    "responder": _create_tool(
        "responder", "Responder", ToolCategory.CREDENTIAL_ACCESS, HD4Phase.DETECT,
        PersonaLevel.CARTEL, 500, 4.5, 3.0, 0.4, 0.8,
        ["T1557"]
    ),
    "lazagne": _create_tool(
        "lazagne", "LaZagne", ToolCategory.CREDENTIAL_ACCESS, HD4Phase.DISABLE,
        PersonaLevel.SCRIPT_KIDDIE, 100, 2.0, 1.5, 0.3, 0.9,
        ["T1555"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # LATERAL MOVEMENT
    # ═══════════════════════════════════════════════════════════════════════════
    "psexec": _create_tool(
        "psexec", "PsExec", ToolCategory.LATERAL_MOVEMENT, HD4Phase.DISABLE,
        PersonaLevel.CARTEL, 100, 3.0, 2.0, 0.5, 0.9,
        ["T1569.002"]
    ),
    "wmi": _create_tool(
        "wmi", "WMI", ToolCategory.LATERAL_MOVEMENT, HD4Phase.DISABLE,
        PersonaLevel.CARTEL, 5000, 5.0, 3.0, 0.4, 0.75,
        ["T1047"]
    ),
    "bloodhound": _create_tool(
        "bloodhound", "BloodHound", ToolCategory.LATERAL_MOVEMENT, HD4Phase.DETECT,
        PersonaLevel.CARTEL, 10000, 6.0, 2.0, 0.3, 0.9,
        ["T1087", "T1069"]
    ),
    "crackmapexec": _create_tool(
        "crackmapexec", "CrackMapExec", ToolCategory.LATERAL_MOVEMENT, HD4Phase.DISABLE,
        PersonaLevel.CARTEL, 20000, 6.0, 3.0, 0.5, 0.8,
        ["T1021"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # COMMAND & CONTROL
    # ═══════════════════════════════════════════════════════════════════════════
    "netcat": _create_tool(
        "netcat", "Netcat", ToolCategory.COMMAND_CONTROL, HD4Phase.DISABLE,
        PersonaLevel.SCRIPT_KIDDIE, 50, 2.0, 2.0, 0.3, 0.9,
        ["T1095"]
    ),
    "dns_tunnel": _create_tool(
        "dns_tunnel", "DNS Tunneling", ToolCategory.COMMAND_CONTROL, HD4Phase.DISABLE,
        PersonaLevel.APT, 1000, 7.0, 4.0, 0.2, 0.6,
        ["T1071.004"]
    ),
    "custom_c2": _create_tool(
        "custom_c2", "Custom C2 Framework", ToolCategory.COMMAND_CONTROL, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 1000000, 9.0, 2.0, 0.1, 0.95,
        ["T1071", "T1573"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # PERSISTENCE
    # ═══════════════════════════════════════════════════════════════════════════
    "scheduled_task": _create_tool(
        "scheduled_task", "Scheduled Task", ToolCategory.PERSISTENCE, HD4Phase.DISABLE,
        PersonaLevel.SCRIPT_KIDDIE, 100, 3.0, 2.0, 0.4, 0.85,
        ["T1053"]
    ),
    "registry_run": _create_tool(
        "registry_run", "Registry Run Keys", ToolCategory.PERSISTENCE, HD4Phase.DISABLE,
        PersonaLevel.SCRIPT_KIDDIE, 50, 2.5, 1.5, 0.5, 0.9,
        ["T1547.001"]
    ),
    "bootkit": _create_tool(
        "bootkit", "Bootkit", ToolCategory.PERSISTENCE, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 100000, 9.5, 3.0, 0.2, 0.4,
        ["T1542"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # DEFENSE EVASION
    # ═══════════════════════════════════════════════════════════════════════════
    "process_injection": _create_tool(
        "process_injection", "Process Injection", ToolCategory.DEFENSE_EVASION, HD4Phase.DISABLE,
        PersonaLevel.APT, 10000, 7.5, 4.0, 0.4, 0.6,
        ["T1055"]
    ),
    "timestomp": _create_tool(
        "timestomp", "Timestomp", ToolCategory.DEFENSE_EVASION, HD4Phase.DISABLE,
        PersonaLevel.CARTEL, 10, 2.0, 1.0, 0.2, 0.95,
        ["T1070.006"]
    ),
    "rootkit": _create_tool(
        "rootkit", "Rootkit", ToolCategory.DEFENSE_EVASION, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 500000, 9.0, 2.0, 0.15, 0.5,
        ["T1014"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # EXFILTRATION
    # ═══════════════════════════════════════════════════════════════════════════
    "rclone": _create_tool(
        "rclone", "Rclone", ToolCategory.EXFILTRATION, HD4Phase.DOMINATE,
        PersonaLevel.CARTEL, 1000, 4.0, 3.0, 0.5, 0.8,
        ["T1567"]
    ),
    "steganography": _create_tool(
        "steganography", "Steganography", ToolCategory.EXFILTRATION, HD4Phase.DOMINATE,
        PersonaLevel.APT, 5000, 7.0, 5.0, 0.1, 0.4,
        ["T1027.003"]
    ),
    
    # ═══════════════════════════════════════════════════════════════════════════
    # APT-SPECIFIC TOOLS
    # ═══════════════════════════════════════════════════════════════════════════
    "sunburst": _create_tool(
        "sunburst", "SUNBURST", ToolCategory.COMMAND_CONTROL, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 10000000, 9.5, 1.0, 0.05, 0.9,
        ["T1195.002"], apt_exclusive="apt29"
    ),
    "teardrop": _create_tool(
        "teardrop", "TEARDROP", ToolCategory.EXECUTION, HD4Phase.DISABLE,
        PersonaLevel.NATION_STATE, 100000, 8.0, 2.0, 0.1, 0.7,
        ["T1059"], apt_exclusive="apt29"
    ),
    "xagent": _create_tool(
        "xagent", "X-Agent", ToolCategory.COMMAND_CONTROL, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 500000, 8.5, 2.0, 0.1, 0.8,
        ["T1071"], apt_exclusive="apt28"
    ),
    "sofacy": _create_tool(
        "sofacy", "Sofacy", ToolCategory.INITIAL_ACCESS, HD4Phase.DISRUPT,
        PersonaLevel.NATION_STATE, 50000, 7.5, 3.0, 0.2, 0.6,
        ["T1566.001"], apt_exclusive="apt28"
    ),
    "zebrocy": _create_tool(
        "zebrocy", "Zebrocy", ToolCategory.EXECUTION, HD4Phase.DISRUPT,
        PersonaLevel.APT, 20000, 6.0, 3.0, 0.25, 0.7,
        ["T1059"], apt_exclusive="apt28"
    ),
    "destover": _create_tool(
        "destover", "Destover", ToolCategory.IMPACT, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 10000, 6.0, 2.0, 0.8, 0.9,
        ["T1485", "T1561"], apt_exclusive="lazarus"
    ),
    "wannacry": _create_tool(
        "wannacry", "WannaCry", ToolCategory.IMPACT, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 100000, 5.0, 1.0, 0.9, 0.95,
        ["T1486"], apt_exclusive="lazarus"
    ),
    "fastcash": _create_tool(
        "fastcash", "FASTCash", ToolCategory.IMPACT, HD4Phase.DOMINATE,
        PersonaLevel.NATION_STATE, 50000, 8.0, 2.0, 0.7, 0.8,
        ["T1565"], apt_exclusive="lazarus"
    ),
    "poison_ivy": _create_tool(
        "poison_ivy", "Poison Ivy", ToolCategory.COMMAND_CONTROL, HD4Phase.DISABLE,
        PersonaLevel.APT, 50000, 6.0, 3.0, 0.3, 0.75,
        ["T1071"], apt_exclusive="apt1"
    ),
    "gh0st_rat": _create_tool(
        "gh0st_rat", "Gh0st RAT", ToolCategory.COMMAND_CONTROL, HD4Phase.DISABLE,
        PersonaLevel.APT, 30000, 5.5, 3.5, 0.35, 0.7,
        ["T1071"], apt_exclusive="apt1"
    ),
    "carbanak": _create_tool(
        "carbanak", "Carbanak", ToolCategory.EXECUTION, HD4Phase.DOMINATE,
        PersonaLevel.APT, 200000, 7.5, 3.0, 0.2, 0.7,
        ["T1059", "T1055"], apt_exclusive="fin7"
    ),
}


# ═══════════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

def calculate_chain_entropy(tool_ids: List[str]) -> float:
    """Calculate total entropy for a tool chain."""
    total = 0.0
    for tool_id in tool_ids:
        if tool_id in TOOL_DATABASE:
            entropy, _ = TOOL_DATABASE[tool_id].calculate_entropy()
            total += entropy
    return total


def get_tools_by_category(category: ToolCategory) -> List[Tool]:
    """Get all tools in a category."""
    return [t for t in TOOL_DATABASE.values() if t.category == category]


def get_tools_by_hd4_phase(phase: HD4Phase) -> List[Tool]:
    """Get all tools in an HD4 phase."""
    return [t for t in TOOL_DATABASE.values() if t.hd4_phase == phase]


def get_tools_by_persona(level: PersonaLevel) -> List[Tool]:
    """Get all tools usable by a persona level."""
    level_order = [PersonaLevel.SCRIPT_KIDDIE, PersonaLevel.CARTEL, 
                   PersonaLevel.APT, PersonaLevel.NATION_STATE]
    level_idx = level_order.index(level)
    
    return [t for t in TOOL_DATABASE.values() 
            if level_order.index(t.persona_min) <= level_idx]


def get_apt_exclusive_tools(apt: APTGroup) -> List[Tool]:
    """Get tools exclusive to an APT group."""
    return [t for t in TOOL_DATABASE.values() if t.apt_exclusive == apt.value]
