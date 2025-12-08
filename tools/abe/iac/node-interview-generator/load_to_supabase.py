#!/usr/bin/env python3
"""
Load processed threat content to Supabase
Uses existing plasma_threats and plasma_entities tables + new tables
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Any

try:
    from supabase import create_client, Client
    SUPABASE_AVAILABLE = True
except ImportError:
    SUPABASE_AVAILABLE = False
    print("⚠️  supabase-py not installed. Install with: pip install supabase")
    sys.exit(1)

# Load Supabase credentials from environment or config
import os
SUPABASE_URL = os.environ.get("SUPABASE_URL")
SUPABASE_KEY = os.environ.get("SUPABASE_KEY") or os.environ.get("SUPABASE_ANON_KEY")

if not SUPABASE_URL or not SUPABASE_KEY:
    print("❌ SUPABASE_URL and SUPABASE_KEY must be set")
    sys.exit(1)

def load_threat_tools(supabase: Client, tools_file: Path):
    """Load threat tools to Supabase."""
    if not tools_file.exists():
        return 0
    
    with open(tools_file) as f:
        tools = json.load(f)
    
    if not isinstance(tools, list):
        tools = tools.get("tools", [])
    
    count = 0
    for tool in tools:
        try:
            # Insert into plasma_entities (tools are entities)
            supabase.table("plasma_entities").insert({
                "type": "tool",
                "value": tool.get("name", ""),
                "metadata": {
                    "description": tool.get("description", ""),
                    "categories": tool.get("categories", []),
                    "homepage": tool.get("homepage", ""),
                    "trivariate_hash": tool.get("trivariate_hash", ""),
                    "unicode_operation": tool.get("unicode_operation", ""),
                    "hd4_phase": tool.get("hd4_phase", ""),
                    "task_graph_node": tool.get("task_graph_node", {})
                },
                "tags": tool.get("categories", []),
                "trivariate_hash": tool.get("trivariate_hash", "")
            }).execute()
            count += 1
        except Exception as e:
            print(f"   ⚠️  Error loading tool {tool.get('name', 'unknown')}: {e}")
    
    return count

def load_ontology(supabase: Client, ontology_file: Path):
    """Load SPIRES ontology to Supabase."""
    if not ontology_file.exists():
        return 0
    
    with open(ontology_file) as f:
        ontology = json.load(f)
    
    # Create threat_ontology table if needed
    # (This would normally be a migration, but for now we'll use plasma_threats)
    
    terms = ontology.get("terms", [])
    count = 0
    
    for term in terms:
        try:
            # Store ontology terms as threats with special source
            supabase.table("plasma_threats").insert({
                "level": "medium",
                "source": "spires_ontology",
                "target": term.get("category", ""),
                "description": term.get("description", ""),
                "mitre": term.get("mitre_mappings", []),
                "raw_data": {
                    "term": term.get("name", ""),
                    "category": term.get("category", ""),
                    "relationships": term.get("relationships", []),
                    "spires_metadata": term.get("metadata", {})
                },
                "source_feed": "spires_ontology"
            }).execute()
            count += 1
        except Exception as e:
            print(f"   ⚠️  Error loading term {term.get('name', 'unknown')}: {e}")
    
    return count

def main():
    output_dir = Path(__file__).parent.parent / "output"
    
    if not SUPABASE_AVAILABLE:
        print("❌ Supabase client not available")
        return
    
    supabase: Client = create_client(SUPABASE_URL, SUPABASE_KEY)
    
    print("📊 Loading threat content to Supabase...")
    print("")
    
    # Load Kali tools
    tools_file = output_dir / "threat_content" / "kali_tools_inventory.json"
    if tools_file.exists():
        print("   Loading Kali tools...")
        count = load_threat_tools(supabase, tools_file)
        print(f"   ✅ Loaded {count} tools")
    
    # Load ontology
    ontology_file = output_dir / "ontology" / "sx9_ontology.json"
    if ontology_file.exists():
        print("   Loading SPIRES ontology...")
        count = load_ontology(supabase, ontology_file)
        print(f"   ✅ Loaded {count} ontology terms")
    
    print("")
    print("✅ Supabase loading complete")

if __name__ == "__main__":
    main()
