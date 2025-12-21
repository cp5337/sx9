#!/usr/bin/env python3
"""
TETH - Tool Entropy Testing Harness
Main Entry Point and Examples

Demonstrates all TETH capabilities:
- Tool entropy analysis
- APT attribution
- Campaign analysis
- Chain optimization
- Monte Carlo validation
"""

import json
from datetime import datetime

from teth import (
    # Core
    TOOL_DATABASE, APTGroup, PersonaLevel, HD4Phase,
    calculate_chain_entropy, get_tools_by_persona, get_info,
    
    # Attribution
    APT_PROFILES, AttributionEngine, get_apt_profile,
    
    # Campaign
    CampaignAnalyzer, simulate_apt_campaign,
    
    # Optimization
    ChainOptimizer, ChainConstraints, OptimizationObjective,
    
    # Validation
    MonteCarloValidator, quick_validate,
)


def print_header(title: str):
    """Print formatted header."""
    print()
    print("=" * 70)
    print(f"  {title}")
    print("=" * 70)
    print()


def example_tool_entropy():
    """Demonstrate entropy calculation."""
    print_header("TOOL ENTROPY ANALYSIS")
    
    # Show tool database stats
    info = get_info()
    print(f"TETH Database: {info['stats']['tools']} tools, "
          f"{info['stats']['apt_groups']} APT groups")
    print()
    
    # Calculate entropy for specific tools
    examples = ["nmap", "metasploit", "cobalt_strike", "sunburst", "bootkit"]
    
    print("Tool Entropy Examples:")
    print("-" * 50)
    print(f"{'Tool':<20} {'Category':<15} {'Entropy':<10} {'±':<10}")
    print("-" * 50)
    
    for tool_id in examples:
        tool = TOOL_DATABASE.get(tool_id)
        if tool:
            entropy, uncertainty = tool.calculate_entropy()
            print(f"{tool.name:<20} {tool.category.value[:15]:<15} "
                  f"{entropy:<10.2f} {uncertainty:<10.2f}")
    
    print()
    
    # Chain entropy
    chain = ["nmap", "metasploit", "mimikatz", "cobalt_strike"]
    chain_entropy = calculate_chain_entropy(chain)
    print(f"Chain {' → '.join(chain)}")
    print(f"Total Entropy: {chain_entropy:.2f}")


def example_apt_attribution():
    """Demonstrate APT attribution."""
    print_header("APT ATTRIBUTION")
    
    engine = AttributionEngine()
    
    # Test chains for different APTs
    test_chains = {
        "APT29-like": ["sunburst", "teardrop", "cobalt_strike", "mimikatz"],
        "APT28-like": ["sofacy", "xagent", "mimikatz", "psexec"],
        "Lazarus-like": ["destover", "wannacry", "powershell_empire"],
        "FIN7-like": ["carbanak", "metasploit", "cobalt_strike", "bloodhound"],
    }
    
    for name, chain in test_chains.items():
        result = engine.get_attribution(chain)
        
        print(f"\nChain: {name}")
        print(f"  Tools: {' → '.join(chain)}")
        
        if result:
            print(f"  Attributed to: {result.apt_group.value.upper()}")
            print(f"  Confidence: {result.confidence:.1%}")
            print(f"  Evidence:")
            for ev in result.evidence[:3]:
                print(f"    - {ev}")
            
            if result.alternative_hypotheses:
                print(f"  Alternatives:")
                for apt, conf in result.alternative_hypotheses[:2]:
                    print(f"    - {apt.value}: {conf:.1%}")
        else:
            print("  Attribution: Unable to determine")


def example_campaign_analysis():
    """Demonstrate campaign analysis."""
    print_header("CAMPAIGN ANALYSIS")
    
    # Simulate an APT29 campaign
    print("Simulating APT29 (Cozy Bear) campaign...")
    campaign = simulate_apt_campaign(APTGroup.APT29, num_events=8)
    
    analyzer = CampaignAnalyzer()
    analyzer.campaigns[campaign.campaign_id] = campaign
    
    analysis = analyzer.analyze_campaign(campaign.campaign_id)
    
    print(f"\nCampaign: {campaign.name}")
    print(f"  Duration: {campaign.duration}")
    print(f"  Events: {len(campaign.events)}")
    print()
    
    print("Tool Chain:")
    for i, event in enumerate(campaign.events):
        tool = TOOL_DATABASE.get(event.tool_id)
        if tool:
            print(f"  {i+1}. {tool.name} ({tool.hd4_phase.value})")
    
    print()
    print(f"Analysis:")
    print(f"  HD4 Phase: {analysis.hd4_phase.value}")
    print(f"  OODA Phase: {analysis.ooda_phase.value}")
    print(f"  Total Entropy: {analysis.total_entropy:.2f}")
    print(f"  Threat Level: {analysis.threat_level}")
    
    if analysis.attributed_apt:
        print(f"  Attribution: {analysis.attributed_apt.value} "
              f"({analysis.attribution_confidence:.1%})")
    
    print()
    print("Predicted Next Tools:")
    for tool_id, prob in analysis.predicted_next_tools[:3]:
        print(f"  - {tool_id}: {prob:.1%}")
    
    print()
    print("Recommended Actions:")
    for action in analysis.recommended_actions[:3]:
        print(f"  • {action}")


def example_chain_optimization():
    """Demonstrate chain optimization."""
    print_header("CHAIN OPTIMIZATION")
    
    constraints = ChainConstraints(
        max_entropy=80,
        max_tools=6,
        persona_level=PersonaLevel.APT,
        required_phases=[HD4Phase.HUNT, HD4Phase.DOMINATE]
    )
    
    optimizer = ChainOptimizer(constraints)
    
    print("Constraints:")
    print(f"  Max Entropy: {constraints.max_entropy}")
    print(f"  Max Tools: {constraints.max_tools}")
    print(f"  Persona Level: {constraints.persona_level.value}")
    print()
    
    # Compare objectives
    results = optimizer.compare_objectives()
    
    for objective, result in results.items():
        print(f"\n{objective.value.upper()} Optimization:")
        print(f"  Chain: {' → '.join(result.tool_ids[:5])}")
        print(f"  Total Entropy: {result.total_entropy:.2f}")
        print(f"  Objective Score: {result.objective_score:.2f}")
        print(f"  Est. Success: {result.estimated_success:.1%}")
        print(f"  Phases: {', '.join(p.value for p in result.phases_covered)}")


def example_validation():
    """Demonstrate Monte Carlo validation."""
    print_header("MONTE CARLO VALIDATION")
    
    print("Running quick validation (100 simulations per metric)...")
    print()
    
    results = quick_validate(100)
    
    for name, result in results.items():
        status = "✅" if result.passed else "❌"
        print(f"{status} {name}: {result.value:.2%}")
        
        if result.confidence_interval:
            print(f"   95% CI: [{result.confidence_interval[0]:.2%}, "
                  f"{result.confidence_interval[1]:.2%}]")
        
        if result.details:
            for key, value in list(result.details.items())[:2]:
                if isinstance(value, float):
                    print(f"   {key}: {value:.2f}")
    
    print()
    print("For full validation, run:")
    print("  validator = MonteCarloValidator()")
    print("  report = validator.run_full_validation()")
    print("  print(report.summary)")


def example_apt_profiles():
    """Show APT profile details."""
    print_header("APT PROFILES")
    
    for apt_id, profile in APT_PROFILES.items():
        print(f"\n{profile.name} ({apt_id.upper()})")
        print(f"  Aliases: {', '.join(profile.aliases[:3])}")
        print(f"  Nation: {profile.nation}")
        print(f"  Motivation: {profile.motivation}")
        print(f"  Entropy Signature: {profile.entropy_mean:.1f} ± {profile.entropy_stddev:.1f}")
        print(f"  Stealth Preference: {profile.stealth_preference:.0%}")
        print(f"  Preferred Phases: {', '.join(p.value for p in profile.preferred_phases)}")
        print(f"  Top Tools: {', '.join(profile.preferred_tools[:4])}")


def run_all_examples():
    """Run all examples."""
    example_tool_entropy()
    example_apt_attribution()
    example_campaign_analysis()
    example_chain_optimization()
    example_apt_profiles()
    example_validation()
    
    print()
    print("=" * 70)
    print("  TETH Examples Complete")
    print("=" * 70)
    print()


if __name__ == "__main__":
    run_all_examples()
