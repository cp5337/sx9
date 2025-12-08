# +---------------------------------------------+
# | CTAS SYSTEM FILE                            |
# | usim_schema.py                              |
# | Author: CTAS Engineering Team               |
# | Date: 2025-01-04                            |
# | Purpose: Pydantic schema for Universal Symbolic Intelligence Module (USIM) |
# +---------------------------------------------+

from typing import List, Optional, Dict
from pydantic import BaseModel, Field
from uuid import UUID
import json
# import yaml  # Optional dependency

# ------------------------------------------------
# Core USIM Data Schema
# ------------------------------------------------

class Toolchain(BaseModel):
    name: str
    type: str  # e.g., "network", "exploit", "recon"
    version: Optional[str] = None
    execution_mode: Optional[str] = Field(default="cli", description="CLI, GUI, script, daemon")

class GraphLink(BaseModel):
    target_uuid: str
    relationship: str  # e.g., "REQUIRES", "SUPPORTS", "FOLLOWED_BY"

class UsimData(BaseModel):
    # Core identifiers
    usim_id: str
    sch_id: str = Field(..., regex=r"^SCH\d{3}-\d{3}$")
    cuid: Optional[str]
    uuid: str

    # Task metadata
    task_name: str
    description: str
    category: str
    phase: str  # HD4 phase: Hunt, Detect, Disable, Disrupt, Dominate

    # Probabilistic metrics
    probability: float = Field(..., ge=0.0, le=1.0)
    transition: float = Field(..., ge=0.0, le=1.0)
    human_skill_index: float = Field(..., ge=0.0, le=1.0)

    # Operational semantics
    capabilities: List[str]
    limitations: List[str]
    ttp: List[str]
    indicators: List[str]
    graph_links: List[GraphLink]
    toolchains: List[Toolchain]

    # Optional: semantic or persona narrative
    persona_narrative: Optional[str]

# ------------------------------------------------
# USIM Header (for YAML/JSON comments)
# ------------------------------------------------

class UsimHeader(BaseModel):
    usim_version: str = "6.5"
    timestamp: str
    source: str  # e.g., "CTAS > CLI > :simulate > :breach"
    hash_level: str = "synaptic"  # synaptic, semantic, symbolic
    type: str = "node_inject"

# ------------------------------------------------
# Complete USIM with Header
# ------------------------------------------------

class CompleteUsim(BaseModel):
    header: UsimHeader
    data: UsimData

# ------------------------------------------------
# Loader Functions
# ------------------------------------------------

def load_usim_json(path: str) -> UsimData:
    """Load USIM from JSON file"""
    with open(path, 'r') as f:
        data = json.load(f)
    usim = UsimData(**data)
    print(f"[✓] USIM {usim.usim_id} loaded successfully from JSON.")
    return usim

def load_usim_yaml(path: str) -> UsimData:
    """Load USIM from YAML file, ignoring CTAS header comments"""
    with open(path, 'r') as f:
        # Skip header comments and load YAML data
        content = f.read()
        yaml_start = content.find('task_name:')
        if yaml_start == -1:
            yaml_start = content.find('usim_id:')
        yaml_content = content[yaml_start:]
        data = yaml.safe_load(yaml_content)
    
    usim = UsimData(**data)
    print(f"[✓] USIM {usim.usim_id} loaded successfully from YAML.")
    return usim

def extract_usim_header(path: str) -> Optional[Dict]:
    """Extract CTAS header from YAML file"""
    with open(path, 'r') as f:
        lines = f.readlines()
    
    header_data = {}
    in_header = False
    
    for line in lines:
        if "CTAS_USIM_START" in line:
            in_header = True
            continue
        elif "CTAS_USIM_END" in line or line.strip().startswith('task_name:'):
            break
        elif in_header and ':' in line and line.startswith('#'):
            # Parse header line like "# SCH: SCH007-006"
            clean_line = line.replace('#', '').strip()
            if ':' in clean_line:
                key, value = clean_line.split(':', 1)
                header_data[key.strip()] = value.strip()
    
    return header_data if header_data else None

# ------------------------------------------------
# CLI Test Example
# ------------------------------------------------

if __name__ == "__main__":
    # Test JSON loading
    test_json_path = "sample_usim.json"
    test_yaml_path = "sample_usim.yaml"
    
    print("=== USIM Schema Validator ===")
    
    try:
        if test_json_path:
            usim_obj = load_usim_json(test_json_path)
            print(f"Task: {usim_obj.task_name}")
            print(f"Phase: {usim_obj.phase}")
            print(f"Probability: {usim_obj.probability}")
    except Exception as e:
        print(f"[ERROR] Failed to load JSON USIM: {e}")
    
    try:
        if test_yaml_path:
            header = extract_usim_header(test_yaml_path)
            print(f"Header extracted: {header}")
            usim_obj = load_usim_yaml(test_yaml_path)
            print(f"Task: {usim_obj.task_name}")
    except Exception as e:
        print(f"[ERROR] Failed to load YAML USIM: {e}")
