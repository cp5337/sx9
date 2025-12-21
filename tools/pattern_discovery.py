#!/usr/bin/env python3
"""
tools/pattern_discovery.py
Scans the SX9 repository to identify the Primary Design Pattern based on strict heuristics.
Brackets size and scope (LOC) for each pattern category.
"""

import os
from pathlib import Path
from collections import defaultdict
import statistics

# Definitions of Pattern Heuristics
# Definitions of Pattern Heuristics
PATTERNS = {
    "reactor": {
        "files": ["Cargo.toml"],
        "content": ["async-nats", "tokio", "loop", "select!"],
        "type": "Rust Crate"
    },
    "react_component": {
        "files": [".tsx"],
        "content": ["export const", "return (", "className="],
        "type": "Frontend UI"
    },
    "python_core": {
        "files": [".py"],
        "content": ["class ", "try:", "def __init__", "dataclass"],
        "type": "Core Tooling"
    },
    "python_adhoc": {
        "files": [".py"],
        "content": [],
        "type": "One-off Script"
    },
    "smart_crate": {
        "files": ["smart-crate.toml"],
        "content": [],
        "type": "SX9 Standard"
    }
}

def analyze_file(path: Path) -> tuple[str, int]:
    try:
        content = path.read_text(errors='ignore')
        loc = len(content.splitlines())
        
        # Check specific markers first (Manifests)
        if path.name == "smart-crate.toml":
            return "smart_crate", 0 # Don't count LOC for manifest marker

        # Check content heuristics
        if path.suffix == ".rs":
            if "async-nats" in content or "tokio" in content:
                return "reactor", loc
            return "rust_lib", loc
            
        if path.suffix == ".tsx":
            if "className=" in content:
                return "react_component", loc
                
        if path.suffix == ".py":
            # Heuristic for Core vs Adhoc
            is_core = False
            
            # 1. Structural checks
            if "class " in content or "def __init__" in content or "@dataclass" in content:
                is_core = True
            
            # 2. Location validation (must be in specific structured dirs)
            path_str = str(path)
            if "sx9-conda" in path_str or "packages/" in path_str or "tools/" in path_str:
                 # Even in core dirs, it might be a quick script, but likely core.
                 pass
            elif "scripts/" in path_str or "archive/" in path_str or "tmp/" in path_str:
                 is_core = False # Downgrade if in scripts/archive

            # 3. Complexity check
            if loc < 50 and not is_core:
                return "python_adhoc", loc
            
            if "if __name__ == \"__main__\":" in content and not is_core:
                # If it has main but no classes and wasn't caught above, it's likely a script
                # But could be a robust script.
                pass

            return ("python_core", loc) if is_core else ("python_adhoc", loc)
                
        return "unknown", loc
    except:
        return "error", 0

def main():
    root = Path(".")
    stats = defaultdict(list) # pattern -> [loc, loc, ...]
    
    ignore_dirs = {".git", "node_modules", "target", "dist", ".venv", ".next"}
    
    print(f"🔍 Scanning Repository: {root.resolve()}")
    
    for path in root.rglob("*"):
        if any(p in str(path) for p in ignore_dirs):
            continue
            
        if path.is_file():
            # Heuristic: Identify component type based on file
            # For crates, we count the crate root?
            # Let's count individual significant files for granular pattern density.
            
            pattern, loc = analyze_file(path)
            if pattern != "unknown" and pattern != "error" and loc > 0:
                stats[pattern].append(loc)

    # Aggregating Results
    print("\n📊 SX9 Design Pattern Distribution (Refined)")
    print(f"{'Pattern':<20} | {'Type':<15} | {'Count':<8} | {'Avg LOC':<10} | {'Scope Bracket'}")
    print("-" * 90)
    
    primary_pattern = None
    max_count = 0
    
    for pattern, locs in sorted(stats.items(), key=lambda x: len(x[1]), reverse=True):
        count = len(locs)
        avg = int(statistics.mean(locs))
        # total = sum(locs)
        
        # Determine strict bracket
        if avg < 50: bracket = "Micro (<50)"
        elif avg < 200: bracket = "Small (<200)"
        elif avg < 1000: bracket = "Medium (<1k)"
        else: bracket = "Large (>1k)"
        
        type_str = PATTERNS.get(pattern, {}).get("type", "Unknown")
        if pattern == "rust_lib": type_str = "Rust Lib"
        
        print(f"{pattern:<20} | {type_str:<15} | {count:<8} | {avg:<10} | {bracket}")
        
        # Determine primary (exclude adhoc)
        if pattern != "python_adhoc" and count > max_count:
            max_count = count
            primary_pattern = pattern

    print("\n🏆 Primary Foundational Pattern: " + (primary_pattern.upper() if primary_pattern else "NONE"))

if __name__ == "__main__":
    main()
