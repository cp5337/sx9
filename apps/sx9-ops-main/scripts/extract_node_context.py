import sys
import os
import json
import toml
import hashlib
import time
import math
import random
from typing import Dict, List, Any

# =============================================================================
# CONSTANTS & CONFIGURATION
# =============================================================================

# Output Directories
OUTPUT_DIR = "synaptix9-glaf-results"
TOML_DIR = os.path.join(OUTPUT_DIR, "identity")
JSON_DIR = os.path.join(OUTPUT_DIR, "execution")
VECTOR_DIR = os.path.join(OUTPUT_DIR, "vectors")

# Ensure directories exist
for d in [TOML_DIR, JSON_DIR, VECTOR_DIR]:
    os.makedirs(d, exist_ok=True)

# Domain Ontology (Cyber Domain - Phase 1)
DOMAIN_TAG = "CYBER_PLASMA_CORE"

# MITRE Phase Mapping
PHASE_MAP = {
    "Reconnaissance": "T1595",
    "Resource Development": "T1583",
    "Initial Access": "T1190",
    "Execution": "T1059",
    "Persistence": "T1136",
    "Privilege Escalation": "T1068",
    "Defense Evasion": "T1070",
    "Credential Access": "T1003",
    "Discovery": "T1082",
    "Lateral Movement": "T1021",
    "Collection": "T1005",
    "Command and Control": "T1071",
    "Exfiltration": "T1048",
    "Impact": "T1485"
}

# Workflow Node Type Mapping (Graph-DB Compatibility)
# Maps generic tool types to specific graph-db node types
TOOL_TYPE_MAP = {
    "scanner": "data_http_request",
    "exploit": "action_emit_event",
    "analyzer": "ai_llm_call",
    "listener": "trigger_unicode",
    "database": "data_supabase_query"
}

# =============================================================================
# OPTICAL & CRYSTAL CONSTANTS
# =============================================================================
CRYSTAL_MAP = {
    "Cyber": "VARNISH_CRYSTAL_E0",
    "Kinetic": "RUBY_CRYSTAL_E4", 
    "Orbital": "DIAMOND_CRYSTAL_E7",
    "Intelligence": "SAPPHIRE_CRYSTAL_EA"
}

# =============================================================================
# MOCK NEO4J DATA (Simulation of Raw Graph)
# =============================================================================
MOCK_NEO4J_NODES = [
    {
        "id": "node_001",
        "labels": ["Interview", "Cyber"],
        "properties": {
            "name": "Nmap Network Scan",
            "phase": "Reconnaissance",
            "tool_ref": "nmap",
            "args": "-sV -p-",
            "category": "scanner"
        }
    },
    {
        "id": "node_002",
        "labels": ["Interview", "Cyber"],
        "properties": {
            "name": "Metasploit Exploit Execution",
            "phase": "Execution",
            "tool_ref": "msfconsole",
            "args": "exploit/multi/handler",
            "category": "exploit"
        }
    },
    {
        "id": "node_003",
        "labels": ["Interview", "Cyber"],
        "properties": {
            "name": "Exfill to S3",
            "phase": "Exfiltration",
            "tool_ref": "aws_cli",
            "args": "s3 cp --recursive",
            "category": "action"
        }
    },
    {
        "id": "node_004",
        "labels": ["Interview", "Cyber"],
        "properties": {
            "name": "Convergence Check",
            "phase": "Command and Control",
            "tool_ref": "glaf_core",
            "args": "verify_h1_h2",
            "category": "ai_ml"  # Will map to ai_convergence
        }
    }
]

# =============================================================================
# MATH & LOGIC COMPONENTS
# =============================================================================

class SoftwareDefinedThyristor:
    """
    Implements the 75% Convergence Line latching logic (RFC-9025).
    Functions as a binary gate: Below Line (Hunt) vs Above Line (HD4 Transition).
    """
    TRIGGER_THRESHOLD = 0.75

    @staticmethod
    def check_convergence(h1_score: float, h2_score: float) -> Dict[str, Any]:
        """
        Determines if the 'thyristor' fires based on H1/H2 convergence.
        """
        converged = (h1_score >= SoftwareDefinedThyristor.TRIGGER_THRESHOLD and 
                     h2_score >= SoftwareDefinedThyristor.TRIGGER_THRESHOLD)
        
        status = "LATCHED_HIGH" if converged else "GATE_LOW"
        phase = "HD4_DOMINATE" if converged else "OODA_HUNT"
        
        return {
            "status": status,
            "phase": phase,
            "threshold": SoftwareDefinedThyristor.TRIGGER_THRESHOLD,
            "converged": converged
        }

def calculate_hash_delta_angle(vec_a: List[float], vec_b: List[float]) -> Dict[str, Any]:
    """
    Calculates the normalized hash delta angle (0.0-1.0) between two vectors.
    1.0 = 180 degrees (Opposite), 0.0 = 0 degrees (Aligned).
    Returns dictionary with value (6 decimals) and 'normalized_turn' unit.
    """
    # Dot product
    dot_product = sum(a * b for a, b in zip(vec_a, vec_b))
    
    # Magnitudes
    mag_a = math.sqrt(sum(a * a for a in vec_a))
    mag_b = math.sqrt(sum(b * b for b in vec_b))
    
    if mag_a == 0 or mag_b == 0:
        return {"value": "0.000000", "unit": "normalized_turn"}
        
    # Cosine similarity
    cosine_sim = dot_product / (mag_a * mag_b)
    cosine_sim = max(min(cosine_sim, 1.0), -1.0)
    
    # Angle in radians (0 to PI)
    angle_rad = math.acos(cosine_sim)
    
    # Normalize to 0.0 - 1.0 (where 1.0 is PI/180deg)
    # This avoids 'Industrial' radians and 'Standard' degrees
    angle_norm = angle_rad / math.pi
    
    return {
        "value": f"{angle_norm:.6f}",
        "unit": "normalized_turn"
    }

def calculate_teth_entropy(node_props: Dict) -> float:
    """
    Calculates Topological Entropy Threat Heuristic (TETH).
    Simulated based on node complexity/phase.
    """
    # Higher entropy for active phases
    phase_weights = {
        "Reconnaissance": 0.3,
        "Execution": 0.9,
        "Exfiltration": 0.8,
        "Command and Control": 0.7
    }
    
    base_entropy = phase_weights.get(node_props.get("phase"), 0.1)
    # Add some jitter for simulation
    return min(base_entropy + (len(node_props.get("args", "")) * 0.01), 1.0)

def generate_h1_hash(identity_data: Dict) -> str:
    """Generates SHA-256 H1 (Semantic/Identity) Hash."""
    payload = json.dumps(identity_data, sort_keys=True).encode('utf-8')
    return hashlib.sha256(payload).hexdigest()

def generate_h2_hash(execution_data: Dict) -> str:
    """Generates SHA-256 H2 (Execution/Operational) Hash."""
    payload = json.dumps(execution_data, sort_keys=True).encode('utf-8')
    return hashlib.sha256(payload).hexdigest()

def map_to_workflow_node(node_props: Dict) -> Dict:
    """
    Transforms a raw Neo4j node into a 'graph-db' compatible WorkflowNode.
    """
    tool_cat = node_props.get("category", "action")
    
    # Special overrides
    if node_props.get("name") == "Convergence Check":
        node_type = "ai_convergence"
    else:
        node_type = TOOL_TYPE_MAP.get(tool_cat, "action_emit_event")

    workflow_node = {
        "id": f"wf_{node_props.get('tool_ref')}_{int(time.time())}",
        "node_type": node_type,
        "category": tool_cat,
        "name": node_props.get("name"),
        "node_config": {
            # Standard config for graph-db nodes
            "tool": node_props.get("tool_ref"),
            "args": node_props.get("args"),
            "phase": node_props.get("phase"),
            "mitre_id": PHASE_MAP.get(node_props.get("phase"), "T0000")
        }
    }
    
    # Add type-specific configs
    if node_type == "ai_convergence":
        workflow_node["node_config"]["threshold"] = 0.85
    elif node_type == "data_http_request":
        workflow_node["node_config"]["method"] = "POST"
        workflow_node["node_config"]["timeout"] = 5000

    return workflow_node

def extract():
    print(f"[*] Starting GLAF Extraction for Domain: {DOMAIN_TAG}")
    print(f"[*] Logic: TETH, Thyristor Latch, Hash Delta (Normalized 0-1)")
    
    processed_count = 0
    
    for node in MOCK_NEO4J_NODES:
        props = node["properties"]
        
        # 0. Math Processing
        # ------------------
        # Simulate H1 (Operational) and H2 (Semantic) scores
        # In real system, this comes from Hawkes Process & Matroid Rank
        h1_score = 0.8 if props.get("phase") in ["Execution", "Exfiltration"] else 0.4
        h2_score = 0.78  # Mock semantic relevance
        
        # Thyristor Check
        thyristor_state = SoftwareDefinedThyristor.check_convergence(h1_score, h2_score)
        
        # TETH Entropy
        teth_entropy = calculate_teth_entropy(props)
        
        # Crystal Selection
        # Use TETH to maybe refine crystal, but default to Domain map
        domain_label = "Cyber" if "Cyber" in node["labels"] else "Kinetic"
        crystal_type = CRYSTAL_MAP.get(domain_label, "GLASS_E1")
        
        # Hash Delta Angle
        # Simulating two 3D vectors for the calculation
        # v1 = Identity Vector, v2 = Current State Vector
        vec_id = [1.0, 0.0, 0.0] 
        vec_state = [0.8, 0.5, 0.1] # Simulated drift
        hash_delta = calculate_hash_delta_angle(vec_id, vec_state)
        
        # 1. Identity Extraction (TOML)
        # -----------------------------
        mitre_id = PHASE_MAP.get(props.get("phase"), "T0000")
        
        identity_data = {
            "name": props.get("name"),
            "domain": DOMAIN_TAG,
            "crystal": crystal_type,
            "teth_entropy": f"{teth_entropy:.4f}",
            "ontology": {
                "phase": props.get("phase"),
                "mitre_id": mitre_id,
                "layer": "Layer 2 (Network)" if "Scan" in props.get("name") else "Layer 7 (App)"
            }
        }
        
        h1_hash = generate_h1_hash(identity_data)
        identity_data["h1_hash"] = h1_hash
        
        toml_filename = f"{mitre_id}_{h1_hash[:8]}.toml"
        with open(os.path.join(TOML_DIR, toml_filename), "w") as f:
            toml.dump(identity_data, f)
            
        # 2. Execution Transformation (JSON) -> "Workflow Node"
        # -----------------------------------------------------
        workflow_node = map_to_workflow_node(props)
        h2_hash = generate_h2_hash(workflow_node)
        
        workflow_node["h2_hash"] = h2_hash
        workflow_node["linked_identity_h1"] = h1_hash
        
        # Inject Math Metadata
        workflow_node["forge_metadata"] = {
            "thyristor_state": thyristor_state,
            "hash_delta": hash_delta,
            "teth_score": teth_entropy, 
            "crystal_context": crystal_type,
            "priority": 1 if thyristor_state["converged"] else 3,
            "classification": "SECRET" if thyristor_state["converged"] else "UNCLASS"
        }
        
        json_filename = f"{mitre_id}_{h2_hash[:8]}.json"
        with open(os.path.join(JSON_DIR, json_filename), "w") as f:
            json.dump(workflow_node, f, indent=2)
            
        # 3. Vector Placeholder
        # ---------------------
        # In a real run, this calls gnn.py. Here we create a manifest for it.
        vector_manifest = {
            "source_h1": h1_hash,
            "source_h2": h2_hash,
            "embedding_target": "plasma_threats",
            "math_layer": {
                "hash_delta": hash_delta,
                "thyristor_triggered": thyristor_state["converged"]
            },
            "features": [
                props.get("name"),
                props.get("phase"),
                props.get("tool_ref")
            ]
        }
        
        with open(os.path.join(VECTOR_DIR, f"vec_{h1_hash[:8]}.json"), "w") as f:
            json.dump(vector_manifest, f, indent=2)
            
        print(f"  [+] Processed {props.get('name')} | Thyristor: {thyristor_state['status']} | Δ: {hash_delta['value']} {hash_delta['unit']}")
        processed_count += 1
        
    print(f"[*] Extraction Complete. Processed {processed_count} nodes.")
    print(f"[*] Artifacts located in: {os.path.abspath(OUTPUT_DIR)}")

if __name__ == "__main__":
    extract()
