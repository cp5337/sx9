#!/usr/bin/env python3
"""
Check existing threat content and SPIRES ontology status.
Shows what's already downloaded and what needs to be fetched.
"""

from pathlib import Path
import json

THREAT_DIR = Path(__file__).parent / "node-interview-generator" / "output" / "threat_content"
ONTOLOGY_DIR = Path(__file__).parent / "output" / "ontology"

def check_threat_content():
    """Check what threat content is already downloaded."""
    print("=" * 70)
    print("EXISTING THREAT CONTENT STATUS")
    print("=" * 70)
    
    if not THREAT_DIR.exists():
        print("❌ Threat content directory does not exist")
        return {}
    
    status = {}
    
    # Check MITRE files
    mitre_files = {
        "mitre_attack": THREAT_DIR / "mitre_attack.json",
        "mitre_attack_ics": THREAT_DIR / "mitre_attack_ics.json",
        "mitre_attack_mobile": THREAT_DIR / "mitre_attack_mobile.json",
        "d3fend": THREAT_DIR / "d3fend.json",
    }
    
    for name, path in mitre_files.items():
        if path.exists():
            size = path.stat().st_size
            status[name] = {"exists": True, "size": size, "path": str(path)}
            print(f"✅ {name}: {size:,} bytes")
        else:
            status[name] = {"exists": False}
            print(f"❌ {name}: Not found")
    
    # Check directories
    dirs_to_check = [
        "atomic-red-team",
        "nuclei-templates",
        "sigma",
        "wazuh",
        "yara-rules",
        "atlas",
        "car",
        "lolbas",
        "gtfobins",
        "loldrivers",
        "hijacklibs",
        "wadcoms",
    ]
    
    print("\n📁 Directories:")
    for dir_name in dirs_to_check:
        dir_path = THREAT_DIR / dir_name
        if dir_path.exists():
            # Count files
            file_count = len(list(dir_path.rglob("*"))) - len(list(dir_path.rglob("*/")))
            status[dir_name] = {"exists": True, "file_count": file_count}
            print(f"✅ {dir_name}: {file_count} files")
        else:
            status[dir_name] = {"exists": False}
            print(f"❌ {dir_name}: Not found")
    
    return status

def check_spires_ontology():
    """Check existing SPIRES ontology."""
    print("\n" + "=" * 70)
    print("EXISTING SPIRES ONTOLOGY STATUS")
    print("=" * 70)
    
    if not ONTOLOGY_DIR.exists():
        print("❌ Ontology directory does not exist")
        return {}
    
    ontology_files = {
        "ontology_raw": ONTOLOGY_DIR / "ontology_raw.json",
        "ontology_cypher": ONTOLOGY_DIR / "ontology.cypher",
        "ontology_surql": ONTOLOGY_DIR / "ontology.surql",
        "ontology_enriched": ONTOLOGY_DIR / "ontology_enriched.json",
    }
    
    status = {}
    
    for name, path in ontology_files.items():
        if path.exists():
            size = path.stat().st_size
            status[name] = {"exists": True, "size": size}
            
            # Try to get term/relation counts for JSON files
            if name.endswith("_raw") or name.endswith("_enriched"):
                try:
                    with open(path) as f:
                        data = json.load(f)
                    terms = len(data.get("terms", []))
                    relations = len(data.get("relations", []))
                    print(f"✅ {name}: {size:,} bytes ({terms} terms, {relations} relations)")
                except:
                    print(f"✅ {name}: {size:,} bytes")
            else:
                print(f"✅ {name}: {size:,} bytes")
        else:
            status[name] = {"exists": False}
            print(f"❌ {name}: Not found")
    
    return status

def main():
    threat_status = check_threat_content()
    ontology_status = check_spires_ontology()
    
    print("\n" + "=" * 70)
    print("SUMMARY")
    print("=" * 70)
    
    threat_exists = sum(1 for s in threat_status.values() if s.get("exists"))
    threat_total = len(threat_status)
    
    ontology_exists = sum(1 for s in ontology_status.values() if s.get("exists"))
    ontology_total = len(ontology_status)
    
    print(f"Threat Content: {threat_exists}/{threat_total} sources downloaded")
    print(f"SPIRES Ontology: {ontology_exists}/{ontology_total} files generated")
    
    if ontology_status.get("ontology_raw", {}).get("exists"):
        print("\n💡 Next run will:")
        print("   1. Load existing ontology (deduplicate)")
        print("   2. Extract NEW terms from threat content")
        print("   3. Merge new with existing")
        print("   4. Generate updated Cypher + SurrealQL")

if __name__ == "__main__":
    main()


