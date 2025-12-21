"""
TETH - Tool Entropy Testing Harness

Quantify cyber tool complexity. Attribute APT campaigns.
Optimize attack chains. Validate through Monte Carlo simulation.
"""

__version__ = "1.0.0"
__author__ = "Charles E. Payne"
__license__ = "Proprietary"

from .entropy_models import (
    # Enums
    ToolCategory,
    HD4Phase,
    PersonaLevel,
    APTGroup,
    
    # Classes
    Tool,
    ToolProperties,
    Persona,
    MatchResult,
    
    # Database
    TOOL_DATABASE,
    
    # Functions
    calculate_chain_entropy,
    get_tools_by_category,
    get_tools_by_hd4_phase,
    get_tools_by_persona,
    get_apt_exclusive_tools,
)

from .apt_profiles import (
    APTProfile,
    APT_PROFILES,
    AttributionEngine,
    AttributionResult,
    get_apt_profile,
    list_apt_groups,
    get_apt_tools,
)

from .campaign_analysis import (
    OODAPhase,
    CampaignEvent,
    Campaign,
    CampaignAnalysis,
    CampaignAnalyzer,
    detect_hd4_phase,
    detect_ooda_phase,
    get_next_phase,
    simulate_apt_campaign,
)

from .chain_optimizer import (
    OptimizationObjective,
    ChainConstraints,
    OptimizedChain,
    ChainOptimizer,
)

from .monte_carlo_validator import (
    ValidationResult,
    FullValidationReport,
    MonteCarloValidator,
    quick_validate,
)


def get_version() -> str:
    """Return TETH version."""
    return __version__


def get_info() -> dict:
    """Return TETH information."""
    return {
        "name": "TETH",
        "version": __version__,
        "description": "Tool Entropy Testing Harness",
        "author": __author__,
        "license": __license__,
        "components": {
            "entropy_models": "Tool and persona definitions",
            "apt_profiles": "APT behavioral signatures",
            "campaign_analysis": "Real-time campaign tracking",
            "chain_optimizer": "Hungarian algorithm optimization",
            "monte_carlo_validator": "Statistical validation",
        },
        "stats": {
            "tools": len(TOOL_DATABASE),
            "apt_groups": len(APT_PROFILES),
            "categories": len(ToolCategory),
            "phases": len(HD4Phase),
        }
    }
