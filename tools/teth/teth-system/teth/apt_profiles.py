"""
TETH - Tool Entropy Testing Harness
APT Behavioral Profiles and Attribution Engine

Profiles for major APT groups with tool preferences,
entropy signatures, and behavioral patterns.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import Dict, List, Tuple, Optional
from collections import Counter

from .entropy_models import (
    APTGroup, HD4Phase, Tool, TOOL_DATABASE, calculate_chain_entropy
)


# ═══════════════════════════════════════════════════════════════════════════════
# APT PROFILE DATA CLASS
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class APTProfile:
    """Behavioral profile for an APT group."""
    id: str
    name: str
    aliases: List[str]
    nation: str
    agency: str
    motivation: str  # espionage, financial, disruption
    
    # Tool preferences
    preferred_tools: List[str]
    tool_weights: Dict[str, float] = field(default_factory=dict)
    
    # Behavioral signatures
    entropy_mean: float = 25.0
    entropy_stddev: float = 5.0
    preferred_phases: List[HD4Phase] = field(default_factory=list)
    stealth_preference: float = 0.5
    
    # Timing patterns
    active_hours_utc: Tuple[int, int] = (0, 24)
    campaign_duration_days: Tuple[int, int] = (30, 180)
    
    # Behavioral markers
    uses_supply_chain: bool = False
    uses_spearphishing: bool = True
    uses_watering_holes: bool = False
    uses_destructive_malware: bool = False
    targets_government: bool = False
    targets_financial: bool = False
    targets_defense: bool = False


# ═══════════════════════════════════════════════════════════════════════════════
# APT PROFILES DATABASE
# ═══════════════════════════════════════════════════════════════════════════════

APT_PROFILES: Dict[str, APTProfile] = {
    # ───────────────────────────────────────────────────────────────────────────
    # APT28 - Fancy Bear (Russia/GRU)
    # ───────────────────────────────────────────────────────────────────────────
    "apt28": APTProfile(
        id="apt28",
        name="Fancy Bear",
        aliases=["APT28", "Sofacy", "Sednit", "Pawn Storm", "Strontium"],
        nation="Russia",
        agency="GRU Unit 26165",
        motivation="espionage",
        
        preferred_tools=[
            "xagent", "sofacy", "zebrocy", "koadic",
            "mimikatz", "psexec", "cobalt_strike",
            "responder", "bloodhound", "powershell_empire"
        ],
        tool_weights={
            "xagent": 2.0, "sofacy": 2.0, "zebrocy": 1.8,
            "cobalt_strike": 1.5, "mimikatz": 1.3
        },
        
        entropy_mean=28.5,
        entropy_stddev=4.2,
        stealth_preference=0.6,
        preferred_phases=[HD4Phase.DISRUPT, HD4Phase.DISABLE],
        active_hours_utc=(6, 18),
        campaign_duration_days=(30, 180),
        
        uses_spearphishing=True,
        targets_government=True,
        targets_defense=True,
    ),
    
    # ───────────────────────────────────────────────────────────────────────────
    # APT29 - Cozy Bear (Russia/SVR)
    # ───────────────────────────────────────────────────────────────────────────
    "apt29": APTProfile(
        id="apt29",
        name="Cozy Bear",
        aliases=["APT29", "The Dukes", "NOBELIUM", "Midnight Blizzard"],
        nation="Russia",
        agency="SVR",
        motivation="espionage",
        
        preferred_tools=[
            "sunburst", "teardrop", "cobalt_strike",
            "mimikatz", "bloodhound", "dns_tunnel",
            "process_injection", "custom_c2"
        ],
        tool_weights={
            "sunburst": 3.0, "teardrop": 2.5,
            "cobalt_strike": 1.5, "dns_tunnel": 1.8
        },
        
        entropy_mean=35.2,
        entropy_stddev=5.8,
        stealth_preference=0.9,
        preferred_phases=[HD4Phase.DISABLE, HD4Phase.DOMINATE],
        active_hours_utc=(5, 17),
        campaign_duration_days=(90, 365),
        
        uses_supply_chain=True,
        targets_government=True,
    ),
    
    # ───────────────────────────────────────────────────────────────────────────
    # Lazarus - Hidden Cobra (North Korea)
    # ───────────────────────────────────────────────────────────────────────────
    "lazarus": APTProfile(
        id="lazarus",
        name="Lazarus Group",
        aliases=["Hidden Cobra", "Zinc", "APT38", "Diamond Sleet"],
        nation="North Korea",
        agency="RGB",
        motivation="financial",
        
        preferred_tools=[
            "destover", "wannacry", "fastcash",
            "powershell_empire", "mimikatz", "cobalt_strike",
            "watering_hole", "spearphish_attach"
        ],
        tool_weights={
            "destover": 2.5, "wannacry": 2.0, "fastcash": 2.5,
            "cobalt_strike": 1.5
        },
        
        entropy_mean=26.8,
        entropy_stddev=6.1,
        stealth_preference=0.4,
        preferred_phases=[HD4Phase.DISRUPT, HD4Phase.DOMINATE],
        active_hours_utc=(0, 12),
        campaign_duration_days=(7, 90),
        
        uses_destructive_malware=True,
        uses_watering_holes=True,
        targets_financial=True,
    ),
    
    # ───────────────────────────────────────────────────────────────────────────
    # APT1 - Comment Crew (China/PLA)
    # ───────────────────────────────────────────────────────────────────────────
    "apt1": APTProfile(
        id="apt1",
        name="Comment Crew",
        aliases=["APT1", "Comment Panda", "Unit 61398"],
        nation="China",
        agency="PLA Unit 61398",
        motivation="espionage",
        
        preferred_tools=[
            "poison_ivy", "gh0st_rat", "netcat",
            "mimikatz", "psexec", "scheduled_task",
            "registry_run"
        ],
        tool_weights={
            "poison_ivy": 2.0, "gh0st_rat": 2.0,
            "mimikatz": 1.3, "psexec": 1.2
        },
        
        entropy_mean=22.4,
        entropy_stddev=3.8,
        stealth_preference=0.5,
        preferred_phases=[HD4Phase.DETECT, HD4Phase.DISABLE],
        active_hours_utc=(0, 8),
        campaign_duration_days=(180, 730),
        
        uses_spearphishing=True,
        targets_defense=True,
    ),
    
    # ───────────────────────────────────────────────────────────────────────────
    # FIN7 - Carbanak (Criminal)
    # ───────────────────────────────────────────────────────────────────────────
    "fin7": APTProfile(
        id="fin7",
        name="FIN7",
        aliases=["Carbanak", "Navigator", "Carbon Spider"],
        nation="Eastern Europe",
        agency="Criminal Organization",
        motivation="financial",
        
        preferred_tools=[
            "carbanak", "cobalt_strike", "metasploit",
            "powershell_empire", "mimikatz", "bloodhound",
            "psexec", "rclone"
        ],
        tool_weights={
            "carbanak": 2.5, "cobalt_strike": 2.0,
            "metasploit": 1.5, "powershell_empire": 1.3
        },
        
        entropy_mean=25.6,
        entropy_stddev=4.5,
        stealth_preference=0.7,
        preferred_phases=[HD4Phase.DISRUPT, HD4Phase.DISABLE, HD4Phase.DOMINATE],
        active_hours_utc=(8, 20),
        campaign_duration_days=(14, 60),
        
        targets_financial=True,
    ),
}


# ═══════════════════════════════════════════════════════════════════════════════
# ATTRIBUTION ENGINE
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class AttributionResult:
    """Result of APT attribution analysis."""
    apt_group: APTGroup
    confidence: float
    evidence: List[str]
    entropy_match: float
    tool_overlap: float
    alternative_hypotheses: List[Tuple[APTGroup, float]]


class AttributionEngine:
    """Engine for attributing tool chains to APT groups."""
    
    def __init__(self):
        self.profiles = APT_PROFILES
    
    def get_attribution(
        self, 
        tool_chain: List[str],
        include_alternatives: bool = True
    ) -> Optional[AttributionResult]:
        """
        Attribute a tool chain to an APT group.
        
        Args:
            tool_chain: List of tool IDs used in chain
            include_alternatives: Include alternative hypotheses
        
        Returns:
            AttributionResult with best match and confidence
        """
        if not tool_chain:
            return None
        
        scores: Dict[str, float] = {}
        evidence: Dict[str, List[str]] = {}
        
        for apt_id, profile in self.profiles.items():
            score = 0.0
            apt_evidence = []
            
            # Tool overlap scoring
            for tool_id in tool_chain:
                if tool_id in profile.preferred_tools:
                    weight = profile.tool_weights.get(tool_id, 1.0)
                    score += weight
                    apt_evidence.append(f"Tool match: {tool_id} (weight={weight:.1f})")
                
                # Check exclusive tools - strong signal
                tool = TOOL_DATABASE.get(tool_id)
                if tool and tool.apt_exclusive == apt_id:
                    score += 5.0
                    apt_evidence.append(f"EXCLUSIVE tool: {tool_id}")
            
            # Entropy signature matching
            chain_entropy = calculate_chain_entropy(tool_chain)
            avg_entropy = chain_entropy / len(tool_chain) if tool_chain else 0
            entropy_distance = abs(avg_entropy - profile.entropy_mean)
            entropy_score = max(0, 1.0 - entropy_distance / (profile.entropy_stddev * 2))
            score += entropy_score * 3
            
            if entropy_score > 0.5:
                apt_evidence.append(f"Entropy signature match: {entropy_score:.2f}")
            
            # Phase pattern matching
            chain_phases = [TOOL_DATABASE[t].hd4_phase for t in tool_chain if t in TOOL_DATABASE]
            phase_counts = Counter(chain_phases)
            if phase_counts:
                dominant_phase = phase_counts.most_common(1)[0][0]
                if dominant_phase in profile.preferred_phases:
                    score += 1.0
                    apt_evidence.append(f"Phase pattern match: {dominant_phase.value}")
            
            # Normalize by chain length
            if len(tool_chain) > 0:
                score /= len(tool_chain)
            
            scores[apt_id] = score
            evidence[apt_id] = apt_evidence
        
        if not scores:
            return None
        
        # Find best match
        best_apt = max(scores, key=scores.get)
        total_score = sum(scores.values())
        confidence = scores[best_apt] / total_score if total_score > 0 else 0.0
        
        # Calculate tool overlap
        profile = self.profiles[best_apt]
        overlap_count = len(set(tool_chain) & set(profile.preferred_tools))
        tool_overlap = overlap_count / len(tool_chain) if tool_chain else 0
        
        # Calculate entropy match
        chain_entropy = calculate_chain_entropy(tool_chain)
        avg_entropy = chain_entropy / len(tool_chain) if tool_chain else 0
        entropy_match = max(0, 1.0 - abs(avg_entropy - profile.entropy_mean) / profile.entropy_stddev)
        
        # Build alternatives
        alternatives = []
        if include_alternatives:
            sorted_scores = sorted(scores.items(), key=lambda x: x[1], reverse=True)
            for apt_id, score in sorted_scores[1:4]:  # Top 3 alternatives
                alt_confidence = score / total_score if total_score > 0 else 0.0
                alternatives.append((APTGroup(apt_id), alt_confidence))
        
        return AttributionResult(
            apt_group=APTGroup(best_apt),
            confidence=confidence,
            evidence=evidence[best_apt],
            entropy_match=entropy_match,
            tool_overlap=tool_overlap,
            alternative_hypotheses=alternatives
        )
    
    def get_likely_next_tools(
        self, 
        tool_chain: List[str],
        top_k: int = 5
    ) -> List[Tuple[str, float]]:
        """
        Predict likely next tools based on attribution.
        
        Args:
            tool_chain: Current tool chain
            top_k: Number of predictions to return
        
        Returns:
            List of (tool_id, probability) tuples
        """
        attribution = self.get_attribution(tool_chain, include_alternatives=False)
        if not attribution or attribution.confidence < 0.3:
            return []
        
        profile = self.profiles[attribution.apt_group.value]
        
        # Get tools not yet used
        used = set(tool_chain)
        candidates = [t for t in profile.preferred_tools if t not in used]
        
        # Score by weight and position in typical chain
        predictions = []
        for tool_id in candidates:
            weight = profile.tool_weights.get(tool_id, 1.0)
            probability = weight * attribution.confidence
            predictions.append((tool_id, probability))
        
        # Normalize
        total = sum(p for _, p in predictions)
        if total > 0:
            predictions = [(t, p/total) for t, p in predictions]
        
        return sorted(predictions, key=lambda x: x[1], reverse=True)[:top_k]


# ═══════════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

def get_apt_profile(apt: APTGroup) -> Optional[APTProfile]:
    """Get profile for an APT group."""
    return APT_PROFILES.get(apt.value)


def list_apt_groups() -> List[APTGroup]:
    """List all profiled APT groups."""
    return [APTGroup(apt_id) for apt_id in APT_PROFILES.keys()]


def get_apt_tools(apt: APTGroup) -> List[Tool]:
    """Get all preferred tools for an APT group."""
    profile = APT_PROFILES.get(apt.value)
    if not profile:
        return []
    return [TOOL_DATABASE[t] for t in profile.preferred_tools if t in TOOL_DATABASE]
