#!/usr/bin/env python3
"""
TETH Chain Launcher

Fire tool chains at PLASMA Defender from the command line.

Usage:
    python fire_chain.py                     # Fire default stealth chain
    python fire_chain.py --apt apt29         # Simulate APT29 campaign
    python fire_chain.py --chain nmap,metasploit,mimikatz  # Custom chain
    python fire_chain.py --objective speed   # Optimize for speed
"""

import argparse
import asyncio
import os
import sys

# Add parent to path for imports
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from teth import (
    TOOL_DATABASE, APTGroup, PersonaLevel, HD4Phase,
    calculate_chain_entropy, get_info,
    ChainOptimizer, ChainConstraints, OptimizationObjective,
    AttributionEngine,
)
from teth.plasma_interface import PLASMAInterface, PLASMAResponse


def print_banner():
    """Print TETH banner."""
    print("""
╔════════════════════════════════════════════════════════════════════╗
║                                                                    ║
║   ████████╗███████╗████████╗██╗  ██╗                              ║
║   ╚══██╔══╝██╔════╝╚══██╔══╝██║  ██║                              ║
║      ██║   █████╗     ██║   ███████║                              ║
║      ██║   ██╔══╝     ██║   ██╔══██║                              ║
║      ██║   ███████╗   ██║   ██║  ██║                              ║
║      ╚═╝   ╚══════╝   ╚═╝   ╚═╝  ╚═╝                              ║
║                                                                    ║
║   Tool Entropy Testing Harness                                     ║
║   → Firing at PLASMA Defender                                      ║
║                                                                    ║
╚════════════════════════════════════════════════════════════════════╝
""")


async def fire_chain(
    plasma_url: str,
    tools: list,
    delay: float = 1.0,
    verbose: bool = True
):
    """Fire a tool chain at PLASMA."""
    async with PLASMAInterface(plasma_url) as plasma:
        # Health check
        healthy = await plasma.health_check()
        if not healthy:
            print(f"❌ PLASMA not reachable at {plasma_url}")
            return
        
        print(f"✅ PLASMA healthy at {plasma_url}")
        print()
        
        # Get attribution
        engine = AttributionEngine()
        attr = engine.get_attribution(tools)
        
        print("Chain Analysis:")
        print(f"  Tools: {' → '.join(tools)}")
        print(f"  Total Entropy: {calculate_chain_entropy(tools):.2f}")
        
        if attr:
            print(f"  Attribution: {attr.apt_group.value} ({attr.confidence:.1%})")
        print()
        
        print("Firing...")
        print("-" * 60)
        
        detected_count = 0
        total_detection_time = 0.0
        
        for i, tool_id in enumerate(tools):
            tool = TOOL_DATABASE.get(tool_id)
            if not tool:
                print(f"  ⚠️  Unknown tool: {tool_id}")
                continue
            
            response = await plasma.fire_tool(tool)
            
            status = "🚨" if response.detected else "✅"
            print(f"  {status} [{i+1}/{len(tools)}] {tool.name}")
            
            if verbose:
                print(f"      Entropy: {tool.calculate_entropy()[0]:.1f}")
                print(f"      Threat Score: {response.threat_score:.2f}")
                if response.alerts:
                    for alert in response.alerts:
                        print(f"      ⚠️  {alert}")
            
            if response.detected:
                detected_count += 1
            total_detection_time += response.detection_time_ms
            
            await asyncio.sleep(delay)
        
        print("-" * 60)
        print()
        print("Summary:")
        print(f"  Detected: {detected_count}/{len(tools)} ({detected_count/len(tools):.0%})")
        print(f"  Avg Detection Time: {total_detection_time/len(tools):.1f}ms")


async def fire_optimized(
    plasma_url: str,
    objective: str,
    persona: str,
    max_tools: int,
    verbose: bool = True
):
    """Fire an optimized chain."""
    obj_map = {
        "stealth": OptimizationObjective.STEALTH,
        "speed": OptimizationObjective.SPEED,
        "coverage": OptimizationObjective.COVERAGE,
        "balanced": OptimizationObjective.BALANCED,
    }
    
    persona_map = {
        "scriptkiddie": PersonaLevel.SCRIPT_KIDDIE,
        "cartel": PersonaLevel.CARTEL,
        "apt": PersonaLevel.APT,
        "nationstate": PersonaLevel.NATION_STATE,
    }
    
    constraints = ChainConstraints(
        max_tools=max_tools,
        persona_level=persona_map.get(persona.lower(), PersonaLevel.CARTEL)
    )
    
    optimizer = ChainOptimizer(constraints)
    chain = optimizer.optimize_chain(obj_map.get(objective.lower(), OptimizationObjective.BALANCED))
    
    print(f"Optimized for: {objective.upper()}")
    print(f"Persona Level: {persona}")
    print()
    
    await fire_chain(plasma_url, chain.tool_ids, verbose=verbose)


async def simulate_apt(
    plasma_url: str,
    apt: str,
    duration_minutes: float,
    intensity: float
):
    """Simulate an APT campaign."""
    apt_map = {
        "apt28": APTGroup.APT28,
        "apt29": APTGroup.APT29,
        "lazarus": APTGroup.LAZARUS,
        "apt1": APTGroup.APT1,
        "fin7": APTGroup.FIN7,
    }
    
    apt_group = apt_map.get(apt.lower())
    if not apt_group:
        print(f"❌ Unknown APT: {apt}")
        print(f"   Available: {', '.join(apt_map.keys())}")
        return
    
    async with PLASMAInterface(plasma_url) as plasma:
        healthy = await plasma.health_check()
        if not healthy:
            print(f"❌ PLASMA not reachable at {plasma_url}")
            return
        
        print(f"Simulating {apt.upper()} campaign")
        print(f"  Duration: {duration_minutes} minutes")
        print(f"  Intensity: {intensity}")
        print()
        print("Running campaign...")
        print("-" * 60)
        
        result = await plasma.simulate_campaign(
            apt_group,
            duration_hours=duration_minutes / 60,
            intensity=intensity
        )
        
        print("-" * 60)
        print()
        print("Campaign Results:")
        print(f"  APT: {result.apt.value}")
        print(f"  Events: {len(result.events)}")
        print(f"  Detection Rate: {result.detection_rate:.1%}")
        print(f"  Avg Detection Time: {result.avg_detection_time_ms:.1f}ms")


def main():
    parser = argparse.ArgumentParser(
        description="TETH Chain Launcher - Fire at PLASMA Defender"
    )
    
    parser.add_argument(
        "--plasma-url",
        default=os.environ.get("PLASMA_URL", "http://localhost:8080"),
        help="PLASMA Defender URL"
    )
    
    parser.add_argument(
        "--chain",
        type=str,
        help="Comma-separated list of tools (e.g., nmap,metasploit,mimikatz)"
    )
    
    parser.add_argument(
        "--objective",
        choices=["stealth", "speed", "coverage", "balanced"],
        default="balanced",
        help="Optimization objective for auto-generated chain"
    )
    
    parser.add_argument(
        "--persona",
        choices=["scriptkiddie", "cartel", "apt", "nationstate"],
        default="cartel",
        help="Persona level for optimization"
    )
    
    parser.add_argument(
        "--max-tools",
        type=int,
        default=6,
        help="Maximum tools in optimized chain"
    )
    
    parser.add_argument(
        "--apt",
        choices=["apt28", "apt29", "lazarus", "apt1", "fin7"],
        help="Simulate APT campaign"
    )
    
    parser.add_argument(
        "--duration",
        type=float,
        default=5.0,
        help="Campaign duration in minutes (for --apt)"
    )
    
    parser.add_argument(
        "--intensity",
        type=float,
        default=1.0,
        help="Campaign intensity 0.1-2.0 (for --apt)"
    )
    
    parser.add_argument(
        "--delay",
        type=float,
        default=1.0,
        help="Delay between tools in seconds"
    )
    
    parser.add_argument(
        "-q", "--quiet",
        action="store_true",
        help="Minimal output"
    )
    
    args = parser.parse_args()
    
    print_banner()
    
    if args.apt:
        asyncio.run(simulate_apt(
            args.plasma_url,
            args.apt,
            args.duration,
            args.intensity
        ))
    elif args.chain:
        tools = [t.strip() for t in args.chain.split(",")]
        asyncio.run(fire_chain(
            args.plasma_url,
            tools,
            args.delay,
            verbose=not args.quiet
        ))
    else:
        asyncio.run(fire_optimized(
            args.plasma_url,
            args.objective,
            args.persona,
            args.max_tools,
            verbose=not args.quiet
        ))


if __name__ == "__main__":
    main()
