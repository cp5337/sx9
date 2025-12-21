"""
TETH - Tool Entropy Testing Harness
PLASMA Defender Integration

Fire TETH-analyzed chains at PLASMA Defender via NATS.
"""

from __future__ import annotations
import asyncio
import json
import uuid
from datetime import datetime, timedelta
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any
import random

try:
    import aiohttp
    HAS_AIOHTTP = True
except ImportError:
    HAS_AIOHTTP = False

try:
    from nats.aio.client import Client as NATS
    HAS_NATS = True
except ImportError:
    HAS_NATS = False

from .entropy_models import (
    Tool, TOOL_DATABASE, APTGroup, HD4Phase, PersonaLevel,
    calculate_chain_entropy
)
from .apt_profiles import APT_PROFILES, AttributionEngine
from .chain_optimizer import OptimizedChain, ChainOptimizer, OptimizationObjective
from .campaign_analysis import Campaign, CampaignEvent


# ═══════════════════════════════════════════════════════════════════════════════
# DATA CLASSES
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class ToolSignature:
    """Tool signature for PLASMA ingestion."""
    id: str
    name: str
    category: str
    hd4_phase: str
    entropy: float
    entropy_uncertainty: float
    mitre_techniques: List[str]
    persona_minimum: str
    
    @classmethod
    def from_tool(cls, tool: Tool) -> "ToolSignature":
        entropy, uncertainty = tool.calculate_entropy()
        return cls(
            id=tool.id,
            name=tool.name,
            category=tool.category.value,
            hd4_phase=tool.hd4_phase.value,
            entropy=entropy,
            entropy_uncertainty=uncertainty,
            mitre_techniques=tool.mitre,
            persona_minimum=tool.persona_min.value
        )


@dataclass
class ChainContext:
    """Context for a tool within a chain."""
    chain_id: str
    position: int
    total_tools: int
    chain_entropy: float
    current_phase: str


@dataclass
class Attribution:
    """Attribution data."""
    apt_group: str
    confidence: float
    evidence: List[str]


@dataclass
class TethToolEvent:
    """TETH → PLASMA event message."""
    event_id: str
    timestamp: str
    tool: ToolSignature
    chain_context: Optional[ChainContext] = None
    attribution: Optional[Attribution] = None
    
    def to_dict(self) -> Dict[str, Any]:
        result = {
            "event_id": self.event_id,
            "timestamp": self.timestamp,
            "tool": asdict(self.tool)
        }
        if self.chain_context:
            result["chain_context"] = asdict(self.chain_context)
        if self.attribution:
            result["attribution"] = asdict(self.attribution)
        return result
    
    def to_json(self) -> str:
        return json.dumps(self.to_dict())


@dataclass
class PLASMAResponse:
    """Response from PLASMA Defender."""
    event_id: str
    detected: bool
    detection_time_ms: float
    alerts: List[str]
    threat_score: float
    recommended_action: str
    
    @classmethod
    def from_dict(cls, data: Dict) -> "PLASMAResponse":
        return cls(
            event_id=data.get("event_id", ""),
            detected=data.get("detected", False),
            detection_time_ms=data.get("detection_time_ms", 0),
            alerts=data.get("alerts", []),
            threat_score=data.get("threat_score", 0),
            recommended_action=data.get("recommended_action", "")
        )


@dataclass
class CampaignResult:
    """Result of simulated campaign."""
    apt: APTGroup
    events: List[PLASMAResponse]
    duration_hours: float
    tools_used: int
    detection_rate: float
    avg_detection_time_ms: float


# ═══════════════════════════════════════════════════════════════════════════════
# NATS PUBLISHER
# ═══════════════════════════════════════════════════════════════════════════════

class NATSPublisher:
    """Publish TETH events to NATS."""
    
    def __init__(self, nats_url: str = "nats://localhost:4222"):
        if not HAS_NATS:
            raise ImportError("nats-py required: pip install nats-py")
        
        self.nats_url = nats_url
        self.nc: Optional[NATS] = None
        self.subject_prefix = "teth"
    
    async def connect(self):
        """Connect to NATS server."""
        self.nc = NATS()
        await self.nc.connect(self.nats_url)
        print(f"Connected to NATS: {self.nats_url}")
    
    async def disconnect(self):
        """Disconnect from NATS."""
        if self.nc:
            await self.nc.close()
    
    async def publish_event(self, event: TethToolEvent) -> None:
        """Publish a tool event."""
        if not self.nc:
            raise RuntimeError("Not connected to NATS")
        
        subject = f"{self.subject_prefix}.tool.{event.tool.category}"
        await self.nc.publish(subject, event.to_json().encode())
    
    async def publish_chain(self, chain: OptimizedChain) -> str:
        """Publish an entire chain."""
        chain_id = str(uuid.uuid4())
        total_entropy = chain.total_entropy
        
        # Get attribution
        engine = AttributionEngine()
        attr_result = engine.get_attribution(chain.tool_ids)
        
        attribution = None
        if attr_result:
            attribution = Attribution(
                apt_group=attr_result.apt_group.value,
                confidence=attr_result.confidence,
                evidence=attr_result.evidence[:5]
            )
        
        for i, tool in enumerate(chain.tools):
            event = TethToolEvent(
                event_id=str(uuid.uuid4()),
                timestamp=datetime.utcnow().isoformat() + "Z",
                tool=ToolSignature.from_tool(tool),
                chain_context=ChainContext(
                    chain_id=chain_id,
                    position=i,
                    total_tools=len(chain.tools),
                    chain_entropy=total_entropy,
                    current_phase=tool.hd4_phase.value
                ),
                attribution=attribution
            )
            
            await self.publish_event(event)
            
            # Simulate timing between tools
            delay = tool.properties.cognitive_load * 0.1
            await asyncio.sleep(delay)
        
        return chain_id


# ═══════════════════════════════════════════════════════════════════════════════
# HTTP INTERFACE
# ═══════════════════════════════════════════════════════════════════════════════

class PLASMAInterface:
    """HTTP interface for PLASMA Defender."""
    
    def __init__(self, plasma_endpoint: str = "http://localhost:8080"):
        if not HAS_AIOHTTP:
            raise ImportError("aiohttp required: pip install aiohttp")
        
        self.endpoint = plasma_endpoint.rstrip("/")
        self.session: Optional[aiohttp.ClientSession] = None
    
    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self
    
    async def __aexit__(self, *args):
        if self.session:
            await self.session.close()
    
    async def health_check(self) -> bool:
        """Check if PLASMA is healthy."""
        try:
            async with self.session.get(f"{self.endpoint}/health") as resp:
                return resp.status == 200
        except Exception:
            return False
    
    async def fire_tool(
        self, 
        tool: Tool,
        source_ip: str = "10.0.0.100",
        target_ip: str = "192.168.1.50"
    ) -> PLASMAResponse:
        """Fire a single tool at PLASMA."""
        event = TethToolEvent(
            event_id=str(uuid.uuid4()),
            timestamp=datetime.utcnow().isoformat() + "Z",
            tool=ToolSignature.from_tool(tool)
        )
        
        payload = event.to_dict()
        payload["source_ip"] = source_ip
        payload["target_ip"] = target_ip
        
        async with self.session.post(
            f"{self.endpoint}/api/v1/ingest/tool",
            json=payload
        ) as resp:
            data = await resp.json()
            return PLASMAResponse.from_dict(data)
    
    async def fire_chain(
        self,
        chain: OptimizedChain,
        delay_factor: float = 1.0
    ) -> List[PLASMAResponse]:
        """Fire an entire chain at PLASMA."""
        responses = []
        chain_id = str(uuid.uuid4())
        
        # Get attribution
        engine = AttributionEngine()
        attr_result = engine.get_attribution(chain.tool_ids)
        
        attribution = None
        if attr_result:
            attribution = Attribution(
                apt_group=attr_result.apt_group.value,
                confidence=attr_result.confidence,
                evidence=attr_result.evidence[:5]
            )
        
        for i, tool in enumerate(chain.tools):
            event = TethToolEvent(
                event_id=str(uuid.uuid4()),
                timestamp=datetime.utcnow().isoformat() + "Z",
                tool=ToolSignature.from_tool(tool),
                chain_context=ChainContext(
                    chain_id=chain_id,
                    position=i,
                    total_tools=len(chain.tools),
                    chain_entropy=chain.total_entropy,
                    current_phase=tool.hd4_phase.value
                ),
                attribution=attribution
            )
            
            payload = event.to_dict()
            
            async with self.session.post(
                f"{self.endpoint}/api/v1/ingest/chain",
                json=payload
            ) as resp:
                data = await resp.json()
                responses.append(PLASMAResponse.from_dict(data))
            
            # Delay between tools
            delay = tool.properties.cognitive_load * delay_factor * 0.5
            await asyncio.sleep(delay)
        
        return responses
    
    async def simulate_campaign(
        self,
        apt: APTGroup,
        duration_hours: float = 1.0,
        intensity: float = 1.0
    ) -> CampaignResult:
        """Simulate an APT campaign against PLASMA."""
        profile = APT_PROFILES.get(apt.value)
        if not profile:
            raise ValueError(f"Unknown APT: {apt.value}")
        
        events: List[PLASMAResponse] = []
        start_time = datetime.now()
        end_time = start_time + timedelta(hours=duration_hours)
        current_time = start_time
        
        while current_time < end_time:
            # Select tool based on APT profile
            tools = profile.preferred_tools
            weights = [profile.tool_weights.get(t, 1.0) for t in tools]
            total_weight = sum(weights)
            weights = [w / total_weight for w in weights]
            
            tool_id = random.choices(tools, weights=weights)[0]
            tool = TOOL_DATABASE.get(tool_id)
            
            if tool:
                response = await self.fire_tool(tool)
                events.append(response)
                
                # Progress time
                delay_minutes = tool.properties.cognitive_load * intensity
                delay_minutes *= random.uniform(0.5, 2.0)
                current_time += timedelta(minutes=delay_minutes)
            
            # Small async delay to not overwhelm
            await asyncio.sleep(0.1)
        
        # Calculate results
        detection_rate = sum(1 for e in events if e.detected) / len(events) if events else 0
        avg_detection_time = (
            sum(e.detection_time_ms for e in events) / len(events) if events else 0
        )
        
        return CampaignResult(
            apt=apt,
            events=events,
            duration_hours=duration_hours,
            tools_used=len(set(e.event_id for e in events)),
            detection_rate=detection_rate,
            avg_detection_time_ms=avg_detection_time
        )


# ═══════════════════════════════════════════════════════════════════════════════
# CLI RUNNER
# ═══════════════════════════════════════════════════════════════════════════════

async def run_demo(plasma_url: str = "http://localhost:8080"):
    """Run a demo against PLASMA Defender."""
    print("TETH → PLASMA Demo")
    print("=" * 50)
    
    async with PLASMAInterface(plasma_url) as plasma:
        # Health check
        healthy = await plasma.health_check()
        if not healthy:
            print(f"❌ PLASMA not reachable at {plasma_url}")
            return
        
        print(f"✅ PLASMA healthy at {plasma_url}")
        print()
        
        # Fire single tool
        print("Firing single tool: nmap")
        tool = TOOL_DATABASE["nmap"]
        response = await plasma.fire_tool(tool)
        print(f"  Detected: {response.detected}")
        print(f"  Threat Score: {response.threat_score}")
        print()
        
        # Fire optimized chain
        print("Firing optimized stealth chain...")
        optimizer = ChainOptimizer()
        chain = optimizer.optimize_chain(OptimizationObjective.STEALTH)
        
        print(f"  Chain: {' → '.join(chain.tool_ids[:5])}")
        responses = await plasma.fire_chain(chain)
        
        detected = sum(1 for r in responses if r.detected)
        print(f"  Detected: {detected}/{len(responses)}")
        print()
        
        # Simulate campaign
        print("Simulating APT29 campaign (10 minutes)...")
        result = await plasma.simulate_campaign(
            APTGroup.APT29,
            duration_hours=0.167,  # 10 minutes
            intensity=0.5
        )
        
        print(f"  Tools fired: {len(result.events)}")
        print(f"  Detection rate: {result.detection_rate:.1%}")
        print(f"  Avg detection time: {result.avg_detection_time_ms:.1f}ms")


def main():
    """CLI entry point."""
    import sys
    
    url = sys.argv[1] if len(sys.argv) > 1 else "http://localhost:8080"
    asyncio.run(run_demo(url))


if __name__ == "__main__":
    main()
