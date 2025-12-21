"""
TETH - Tool Entropy Testing Harness
Campaign Analysis with OODA/HD4 Phase Detection

Real-time campaign tracking, phase detection, and prediction.
"""

from __future__ import annotations
import uuid
from datetime import datetime, timedelta
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
from collections import Counter
from enum import Enum

from .entropy_models import (
    HD4Phase, Tool, TOOL_DATABASE, calculate_chain_entropy, APTGroup
)
from .apt_profiles import AttributionEngine, APTProfile, APT_PROFILES


# ═══════════════════════════════════════════════════════════════════════════════
# OODA LOOP PHASES
# ═══════════════════════════════════════════════════════════════════════════════

class OODAPhase(Enum):
    OBSERVE = "observe"
    ORIENT = "orient"
    DECIDE = "decide"
    ACT = "act"


# ═══════════════════════════════════════════════════════════════════════════════
# DATA CLASSES
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class CampaignEvent:
    """A single tool event in a campaign."""
    event_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    timestamp: datetime = field(default_factory=datetime.now)
    tool_id: str = ""
    source_ip: Optional[str] = None
    target_ip: Optional[str] = None
    metadata: Dict = field(default_factory=dict)
    
    @property
    def tool(self) -> Optional[Tool]:
        return TOOL_DATABASE.get(self.tool_id)


@dataclass
class Campaign:
    """An ongoing campaign with events."""
    campaign_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = "Unknown Campaign"
    events: List[CampaignEvent] = field(default_factory=list)
    created_at: datetime = field(default_factory=datetime.now)
    
    # Analysis state
    current_hd4_phase: HD4Phase = HD4Phase.HUNT
    current_ooda_phase: OODAPhase = OODAPhase.OBSERVE
    attributed_apt: Optional[APTGroup] = None
    attribution_confidence: float = 0.0
    
    @property
    def tool_chain(self) -> List[str]:
        return [e.tool_id for e in self.events]
    
    @property
    def duration(self) -> timedelta:
        if len(self.events) < 2:
            return timedelta(0)
        return self.events[-1].timestamp - self.events[0].timestamp
    
    @property
    def total_entropy(self) -> float:
        return calculate_chain_entropy(self.tool_chain)


@dataclass
class CampaignAnalysis:
    """Complete analysis of a campaign."""
    campaign_id: str
    hd4_phase: HD4Phase
    ooda_phase: OODAPhase
    attributed_apt: Optional[APTGroup]
    attribution_confidence: float
    total_entropy: float
    avg_entropy: float
    tool_count: int
    phase_distribution: Dict[str, int]
    predicted_next_tools: List[Tuple[str, float]]
    threat_level: str
    recommended_actions: List[str]


# ═══════════════════════════════════════════════════════════════════════════════
# PHASE DETECTION
# ═══════════════════════════════════════════════════════════════════════════════

def detect_hd4_phase(events: List[CampaignEvent]) -> HD4Phase:
    """
    Detect current HD4 phase from campaign events.
    
    Phases progress: HUNT → DETECT → DISRUPT → DISABLE → DOMINATE
    """
    if not events:
        return HD4Phase.HUNT
    
    # Get phases from tools
    phases = []
    for event in events:
        tool = TOOL_DATABASE.get(event.tool_id)
        if tool:
            phases.append(tool.hd4_phase)
    
    if not phases:
        return HD4Phase.HUNT
    
    # Recent tools matter more
    recent = phases[-5:] if len(phases) >= 5 else phases
    phase_counts = Counter(recent)
    
    # Check for phase progression
    phase_order = [HD4Phase.HUNT, HD4Phase.DETECT, HD4Phase.DISRUPT, 
                   HD4Phase.DISABLE, HD4Phase.DOMINATE]
    
    # If dominate tools present in last 3, we're in dominate
    if len(phases) >= 3:
        last_three = phases[-3:]
        if all(p == HD4Phase.DOMINATE for p in last_three):
            return HD4Phase.DOMINATE
        if all(p in [HD4Phase.DISABLE, HD4Phase.DOMINATE] for p in last_three):
            return HD4Phase.DISABLE
    
    # Otherwise, return most common recent phase
    return phase_counts.most_common(1)[0][0]


def detect_ooda_phase(hd4_phase: HD4Phase) -> OODAPhase:
    """
    Map HD4 phase to OODA loop phase.
    """
    mapping = {
        HD4Phase.HUNT: OODAPhase.OBSERVE,
        HD4Phase.DETECT: OODAPhase.ORIENT,
        HD4Phase.DISRUPT: OODAPhase.DECIDE,
        HD4Phase.DISABLE: OODAPhase.ACT,
        HD4Phase.DOMINATE: OODAPhase.ACT,
    }
    return mapping.get(hd4_phase, OODAPhase.OBSERVE)


def get_next_phase(current: HD4Phase) -> HD4Phase:
    """Get the expected next HD4 phase."""
    progression = {
        HD4Phase.HUNT: HD4Phase.DETECT,
        HD4Phase.DETECT: HD4Phase.DISRUPT,
        HD4Phase.DISRUPT: HD4Phase.DISABLE,
        HD4Phase.DISABLE: HD4Phase.DOMINATE,
        HD4Phase.DOMINATE: HD4Phase.DOMINATE,  # Terminal
    }
    return progression.get(current, HD4Phase.HUNT)


# ═══════════════════════════════════════════════════════════════════════════════
# CAMPAIGN ANALYZER
# ═══════════════════════════════════════════════════════════════════════════════

class CampaignAnalyzer:
    """Analyzes campaigns for phase detection and attribution."""
    
    def __init__(self):
        self.campaigns: Dict[str, Campaign] = {}
        self.attribution_engine = AttributionEngine()
    
    def create_campaign(self, name: str = "Unknown Campaign") -> Campaign:
        """Create a new campaign."""
        campaign = Campaign(name=name)
        self.campaigns[campaign.campaign_id] = campaign
        return campaign
    
    def add_event(
        self, 
        campaign_id: str, 
        tool_id: str,
        source_ip: Optional[str] = None,
        target_ip: Optional[str] = None,
        timestamp: Optional[datetime] = None,
        metadata: Optional[Dict] = None
    ) -> Optional[CampaignEvent]:
        """Add an event to a campaign."""
        campaign = self.campaigns.get(campaign_id)
        if not campaign:
            return None
        
        event = CampaignEvent(
            tool_id=tool_id,
            source_ip=source_ip,
            target_ip=target_ip,
            timestamp=timestamp or datetime.now(),
            metadata=metadata or {}
        )
        
        campaign.events.append(event)
        
        # Update campaign state
        campaign.current_hd4_phase = detect_hd4_phase(campaign.events)
        campaign.current_ooda_phase = detect_ooda_phase(campaign.current_hd4_phase)
        
        # Update attribution
        attribution = self.attribution_engine.get_attribution(campaign.tool_chain)
        if attribution:
            campaign.attributed_apt = attribution.apt_group
            campaign.attribution_confidence = attribution.confidence
        
        return event
    
    def analyze_campaign(self, campaign_id: str) -> Optional[CampaignAnalysis]:
        """Perform complete analysis of a campaign."""
        campaign = self.campaigns.get(campaign_id)
        if not campaign:
            return None
        
        # Phase distribution
        phase_dist: Dict[str, int] = {}
        for event in campaign.events:
            tool = TOOL_DATABASE.get(event.tool_id)
            if tool:
                phase = tool.hd4_phase.value
                phase_dist[phase] = phase_dist.get(phase, 0) + 1
        
        # Get predictions
        predicted = self.attribution_engine.get_likely_next_tools(
            campaign.tool_chain, top_k=5
        )
        
        # Calculate threat level
        threat_level = self._calculate_threat_level(campaign)
        
        # Generate recommendations
        recommendations = self._generate_recommendations(campaign, threat_level)
        
        return CampaignAnalysis(
            campaign_id=campaign_id,
            hd4_phase=campaign.current_hd4_phase,
            ooda_phase=campaign.current_ooda_phase,
            attributed_apt=campaign.attributed_apt,
            attribution_confidence=campaign.attribution_confidence,
            total_entropy=campaign.total_entropy,
            avg_entropy=campaign.total_entropy / len(campaign.events) if campaign.events else 0,
            tool_count=len(campaign.events),
            phase_distribution=phase_dist,
            predicted_next_tools=predicted,
            threat_level=threat_level,
            recommended_actions=recommendations
        )
    
    def _calculate_threat_level(self, campaign: Campaign) -> str:
        """Calculate threat level from campaign characteristics."""
        score = 0
        
        # Phase-based scoring
        phase_scores = {
            HD4Phase.HUNT: 1,
            HD4Phase.DETECT: 2,
            HD4Phase.DISRUPT: 3,
            HD4Phase.DISABLE: 4,
            HD4Phase.DOMINATE: 5,
        }
        score += phase_scores.get(campaign.current_hd4_phase, 0)
        
        # Entropy-based scoring
        if campaign.total_entropy > 100:
            score += 3
        elif campaign.total_entropy > 50:
            score += 2
        elif campaign.total_entropy > 25:
            score += 1
        
        # APT attribution scoring
        if campaign.attributed_apt:
            score += 2
            if campaign.attribution_confidence > 0.7:
                score += 1
        
        # Nation-state tool scoring
        for event in campaign.events:
            tool = TOOL_DATABASE.get(event.tool_id)
            if tool and tool.apt_exclusive:
                score += 2
                break
        
        # Map to threat level
        if score >= 10:
            return "CRITICAL"
        elif score >= 7:
            return "HIGH"
        elif score >= 4:
            return "MEDIUM"
        else:
            return "LOW"
    
    def _generate_recommendations(
        self, 
        campaign: Campaign, 
        threat_level: str
    ) -> List[str]:
        """Generate recommended defensive actions."""
        recommendations = []
        
        # Based on threat level
        if threat_level == "CRITICAL":
            recommendations.append("IMMEDIATE: Isolate affected systems")
            recommendations.append("Engage incident response team")
            recommendations.append("Notify executive leadership")
        elif threat_level == "HIGH":
            recommendations.append("Increase monitoring on affected segments")
            recommendations.append("Prepare incident response team")
        
        # Based on phase
        if campaign.current_hd4_phase == HD4Phase.DOMINATE:
            recommendations.append("Assume full compromise - begin containment")
            recommendations.append("Preserve forensic evidence")
        elif campaign.current_hd4_phase == HD4Phase.DISABLE:
            recommendations.append("Hunt for persistence mechanisms")
            recommendations.append("Rotate affected credentials")
        elif campaign.current_hd4_phase == HD4Phase.DISRUPT:
            recommendations.append("Block identified C2 channels")
            recommendations.append("Increase endpoint monitoring")
        
        # Based on attribution
        if campaign.attributed_apt:
            profile = APT_PROFILES.get(campaign.attributed_apt.value)
            if profile:
                if profile.uses_supply_chain:
                    recommendations.append("Audit software supply chain")
                if profile.uses_destructive_malware:
                    recommendations.append("Verify backup integrity")
        
        return recommendations
    
    def get_campaign(self, campaign_id: str) -> Optional[Campaign]:
        """Get a campaign by ID."""
        return self.campaigns.get(campaign_id)
    
    def list_campaigns(self) -> List[Campaign]:
        """List all campaigns."""
        return list(self.campaigns.values())


# ═══════════════════════════════════════════════════════════════════════════════
# SIMULATION
# ═══════════════════════════════════════════════════════════════════════════════

def simulate_apt_campaign(
    apt: APTGroup,
    num_events: int = 10,
    start_time: Optional[datetime] = None
) -> Campaign:
    """
    Simulate a campaign for an APT group.
    
    Useful for testing and validation.
    """
    import random
    
    profile = APT_PROFILES.get(apt.value)
    if not profile:
        raise ValueError(f"No profile for APT: {apt.value}")
    
    analyzer = CampaignAnalyzer()
    campaign = analyzer.create_campaign(f"Simulated {profile.name} Campaign")
    
    current_time = start_time or datetime.now()
    
    # Generate events following APT patterns
    for i in range(num_events):
        # Select tool based on weights
        tools = profile.preferred_tools
        weights = [profile.tool_weights.get(t, 1.0) for t in tools]
        total_weight = sum(weights)
        weights = [w / total_weight for w in weights]
        
        tool_id = random.choices(tools, weights=weights)[0]
        
        # Progress time
        tool = TOOL_DATABASE.get(tool_id)
        if tool:
            delay_minutes = tool.properties.cognitive_load * random.uniform(5, 30)
            current_time += timedelta(minutes=delay_minutes)
        
        analyzer.add_event(
            campaign.campaign_id,
            tool_id,
            source_ip=f"10.{random.randint(0, 255)}.{random.randint(0, 255)}.{random.randint(1, 254)}",
            target_ip=f"192.168.{random.randint(0, 255)}.{random.randint(1, 254)}",
            timestamp=current_time
        )
    
    return campaign
