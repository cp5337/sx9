"""
TETH - Tool Entropy Testing Harness
Monte Carlo Validation System

Statistical validation through simulation.
"""

from __future__ import annotations
import random
import math
from dataclasses import dataclass, field
from typing import Dict, List, Tuple, Optional
from datetime import datetime

from .entropy_models import (
    Tool, TOOL_DATABASE, PersonaLevel, APTGroup,
    calculate_chain_entropy, ToolProperties, ToolCategory, HD4Phase
)
from .apt_profiles import APT_PROFILES, AttributionEngine
from .chain_optimizer import ChainOptimizer, OptimizationObjective, ChainConstraints


# ═══════════════════════════════════════════════════════════════════════════════
# DATA CLASSES
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class ValidationResult:
    """Result of a validation run."""
    metric: str
    value: float
    confidence_interval: Optional[Tuple[float, float]] = None
    bounds: Optional[Tuple[float, float]] = None
    num_simulations: int = 0
    passed: bool = True
    details: Dict = field(default_factory=dict)


@dataclass
class FullValidationReport:
    """Complete validation report."""
    timestamp: datetime = field(default_factory=datetime.now)
    results: Dict[str, ValidationResult] = field(default_factory=dict)
    overall_passed: bool = True
    summary: str = ""


# ═══════════════════════════════════════════════════════════════════════════════
# MONTE CARLO VALIDATOR
# ═══════════════════════════════════════════════════════════════════════════════

class MonteCarloValidator:
    """Validates TETH components through Monte Carlo simulation."""
    
    def __init__(self, seed: int = 42):
        self.seed = seed
        random.seed(seed)
        self.attribution_engine = AttributionEngine()
    
    def _calculate_ci(
        self, 
        successes: int, 
        total: int, 
        confidence: float = 0.95
    ) -> Tuple[float, float]:
        """Calculate confidence interval for proportion."""
        if total == 0:
            return (0.0, 0.0)
        
        p = successes / total
        z = 1.96 if confidence == 0.95 else 2.576  # 95% or 99%
        
        margin = z * math.sqrt(p * (1 - p) / total)
        return (max(0, p - margin), min(1, p + margin))
    
    def validate_attribution(
        self, 
        num_simulations: int = 2000
    ) -> ValidationResult:
        """
        Validate attribution engine accuracy.
        
        Generates synthetic campaigns for known APTs and
        measures attribution accuracy.
        """
        correct = 0
        total = 0
        apt_results: Dict[str, Dict[str, int]] = {}
        
        for apt_id in APT_PROFILES.keys():
            apt_results[apt_id] = {"correct": 0, "total": 0}
        
        for _ in range(num_simulations):
            # Pick random APT
            apt_id = random.choice(list(APT_PROFILES.keys()))
            profile = APT_PROFILES[apt_id]
            
            # Generate synthetic chain
            chain_length = random.randint(3, 8)
            chain = []
            
            for _ in range(chain_length):
                # Weighted selection from preferred tools
                tools = profile.preferred_tools
                weights = [profile.tool_weights.get(t, 1.0) for t in tools]
                total_weight = sum(weights)
                weights = [w / total_weight for w in weights]
                
                tool = random.choices(tools, weights=weights)[0]
                if tool in TOOL_DATABASE:
                    chain.append(tool)
            
            if len(chain) < 2:
                continue
            
            # Run attribution
            result = self.attribution_engine.get_attribution(chain)
            
            total += 1
            apt_results[apt_id]["total"] += 1
            
            if result and result.apt_group.value == apt_id:
                correct += 1
                apt_results[apt_id]["correct"] += 1
        
        accuracy = correct / total if total > 0 else 0
        ci = self._calculate_ci(correct, total)
        
        return ValidationResult(
            metric="attribution_accuracy",
            value=accuracy,
            confidence_interval=ci,
            num_simulations=num_simulations,
            passed=accuracy >= 0.75,  # 75% threshold
            details={
                "per_apt_accuracy": {
                    apt: r["correct"] / r["total"] if r["total"] > 0 else 0
                    for apt, r in apt_results.items()
                }
            }
        )
    
    def validate_entropy_model(
        self, 
        num_simulations: int = 2000
    ) -> ValidationResult:
        """
        Validate entropy calculations are stable and bounded.
        """
        entropies = []
        
        for _ in range(num_simulations):
            # Generate random tool properties
            props = ToolProperties(
                branching_paths=random.randint(1, 10000000),
                cognitive_load=random.uniform(1.0, 10.0),
                variability=random.uniform(1.0, 10.0),
                operational_risk=random.uniform(0.0, 1.0),
                feedback_clarity=random.uniform(0.0, 1.0)
            )
            
            tool = Tool(
                id="test",
                name="Test Tool",
                category=ToolCategory.EXECUTION,
                hd4_phase=HD4Phase.DISRUPT,
                persona_min=PersonaLevel.CARTEL,
                properties=props
            )
            
            entropy, uncertainty = tool.calculate_entropy()
            entropies.append(entropy)
        
        mean_entropy = sum(entropies) / len(entropies)
        std_entropy = math.sqrt(
            sum((e - mean_entropy) ** 2 for e in entropies) / len(entropies)
        )
        
        return ValidationResult(
            metric="entropy_stability",
            value=std_entropy,
            bounds=(min(entropies), max(entropies)),
            num_simulations=num_simulations,
            passed=std_entropy < 20,  # Reasonable variation
            details={
                "mean": mean_entropy,
                "std": std_entropy,
                "min": min(entropies),
                "max": max(entropies)
            }
        )
    
    def validate_optimizer(
        self, 
        num_simulations: int = 500
    ) -> ValidationResult:
        """
        Validate chain optimizer produces valid results.
        """
        valid_chains = 0
        total = 0
        improvements = []
        
        for _ in range(num_simulations):
            # Random constraints
            constraints = ChainConstraints(
                max_entropy=random.uniform(50, 150),
                max_tools=random.randint(3, 10),
                persona_level=random.choice(list(PersonaLevel))
            )
            
            optimizer = ChainOptimizer(constraints)
            
            # Try each objective
            for objective in [OptimizationObjective.STEALTH, 
                             OptimizationObjective.SPEED,
                             OptimizationObjective.BALANCED]:
                try:
                    result = optimizer.optimize_chain(objective)
                    total += 1
                    
                    # Validate chain
                    if result.tools:
                        valid_chains += 1
                        
                        # Check entropy constraint
                        if result.total_entropy <= constraints.max_entropy:
                            improvements.append(result.objective_score)
                except Exception:
                    total += 1
        
        success_rate = valid_chains / total if total > 0 else 0
        avg_improvement = sum(improvements) / len(improvements) if improvements else 0
        
        return ValidationResult(
            metric="optimizer_validity",
            value=success_rate,
            confidence_interval=self._calculate_ci(valid_chains, total),
            num_simulations=num_simulations,
            passed=success_rate >= 0.9,
            details={
                "valid_chains": valid_chains,
                "total_attempts": total,
                "avg_objective_score": avg_improvement
            }
        )
    
    def validate_phase_detection(
        self, 
        num_simulations: int = 1000
    ) -> ValidationResult:
        """
        Validate phase detection accuracy.
        """
        from .campaign_analysis import detect_hd4_phase, CampaignEvent
        
        correct = 0
        total = 0
        
        for _ in range(num_simulations):
            # Generate events with known phase progression
            target_phase = random.choice(list(HD4Phase))
            
            # Get tools for this phase
            phase_tools = [t for t in TOOL_DATABASE.values() 
                          if t.hd4_phase == target_phase]
            
            if not phase_tools:
                continue
            
            # Generate events primarily from this phase
            events = []
            for _ in range(random.randint(3, 7)):
                if random.random() < 0.8:  # 80% from target phase
                    tool = random.choice(phase_tools)
                else:
                    tool = random.choice(list(TOOL_DATABASE.values()))
                
                events.append(CampaignEvent(tool_id=tool.id))
            
            # Detect phase
            detected = detect_hd4_phase(events)
            
            total += 1
            if detected == target_phase:
                correct += 1
        
        accuracy = correct / total if total > 0 else 0
        
        return ValidationResult(
            metric="phase_detection_accuracy",
            value=accuracy,
            confidence_interval=self._calculate_ci(correct, total),
            num_simulations=num_simulations,
            passed=accuracy >= 0.7
        )
    
    def validate_persona_matching(
        self, 
        num_simulations: int = 1000
    ) -> ValidationResult:
        """
        Validate persona-tool matching logic.
        """
        from .entropy_models import Persona, get_tools_by_persona
        
        valid_matches = 0
        total = 0
        
        for level in PersonaLevel:
            accessible_tools = get_tools_by_persona(level)
            
            for _ in range(num_simulations // len(PersonaLevel)):
                if not accessible_tools:
                    continue
                
                tool = random.choice(accessible_tools)
                persona = Persona(
                    id="test",
                    name="Test",
                    level=level,
                    experience_hours=random.randint(100, 10000)
                )
                
                # Check tool is within persona range
                entropy, _ = tool.calculate_entropy()
                
                total += 1
                if persona.min_entropy <= entropy <= persona.max_entropy + 10:
                    valid_matches += 1
        
        match_rate = valid_matches / total if total > 0 else 0
        
        return ValidationResult(
            metric="persona_match_validity",
            value=match_rate,
            confidence_interval=self._calculate_ci(valid_matches, total),
            num_simulations=num_simulations,
            passed=match_rate >= 0.6
        )
    
    def run_full_validation(
        self,
        attribution_sims: int = 2000,
        entropy_sims: int = 2000,
        optimizer_sims: int = 500,
        phase_sims: int = 1000,
        persona_sims: int = 1000
    ) -> FullValidationReport:
        """Run complete validation suite."""
        results = {}
        
        print("Running attribution validation...")
        results["attribution"] = self.validate_attribution(attribution_sims)
        
        print("Running entropy model validation...")
        results["entropy"] = self.validate_entropy_model(entropy_sims)
        
        print("Running optimizer validation...")
        results["optimizer"] = self.validate_optimizer(optimizer_sims)
        
        print("Running phase detection validation...")
        results["phase_detection"] = self.validate_phase_detection(phase_sims)
        
        print("Running persona matching validation...")
        results["persona_matching"] = self.validate_persona_matching(persona_sims)
        
        overall_passed = all(r.passed for r in results.values())
        
        summary_lines = [
            "TETH Validation Report",
            "=" * 50,
            ""
        ]
        
        for name, result in results.items():
            status = "✅ PASS" if result.passed else "❌ FAIL"
            summary_lines.append(f"{name}: {result.value:.2%} {status}")
            if result.confidence_interval:
                summary_lines.append(
                    f"  95% CI: [{result.confidence_interval[0]:.2%}, "
                    f"{result.confidence_interval[1]:.2%}]"
                )
        
        summary_lines.append("")
        summary_lines.append(
            f"Overall: {'PASSED' if overall_passed else 'FAILED'}"
        )
        
        return FullValidationReport(
            results=results,
            overall_passed=overall_passed,
            summary="\n".join(summary_lines)
        )


# ═══════════════════════════════════════════════════════════════════════════════
# QUICK VALIDATION
# ═══════════════════════════════════════════════════════════════════════════════

def quick_validate(num_sims: int = 100) -> Dict[str, ValidationResult]:
    """Quick validation for development/testing."""
    validator = MonteCarloValidator(seed=42)
    return {
        "attribution": validator.validate_attribution(num_sims),
        "entropy": validator.validate_entropy_model(num_sims),
        "optimizer": validator.validate_optimizer(num_sims // 4),
    }
