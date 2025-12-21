import os
import re
import sys

RFC_DIR = "/Users/cp5337/Developer/sx9/01-rfc"
REGISTRY_PATH = os.path.join(RFC_DIR, "REGISTRY.md")

def get_all_rfcs():
    rfcs = {}
    for root, _, files in os.walk(RFC_DIR):
        for file in files:
            if file.startswith("RFC-") and file.endswith(".md") and "REGISTRY" not in file:
                path = os.path.join(root, file)
                # Extract Number
                match = re.search(r'RFC-(\d+)', file)
                if match:
                    rfcs[match.group(1)] = path
    return rfcs

def parse_registry():
    with open(REGISTRY_PATH, 'r') as f:
        content = f.read()
    
    registered = {}
    # Find | RFC-XXXX |
    matches = re.findall(r'\| RFC-(\d+) ', content)
    for m in matches:
        registered[m] = True
    return registered

def audit():
    print("🦅 RFC Deep Audit Tool v1.0")
    print("==========================")
    
    files = get_all_rfcs()
    registry = parse_registry()
    
    errors = []
    
    # 1. Registry Check
    print(f"Checking {len(files)} files against Registry...")
    for number, path in files.items():
        if number not in registry:
            errors.append(f"[REGISTRY] RFC-{number} exists on disk but NOT in REGISTRY.md: {path}")
            
    # 2. Disk Check
    print(f"Checking {len(registry)} registry entries against Disk...")
    for number in registry:
        if number not in files:
            errors.append(f"[DISK] RFC-{number} is in REGISTRY.md but NOT on disk.")

    # 3. Content Check (Link Validation)
    print("Checking Internal Links...")
    link_pattern = re.compile(r'RFC-(\d+)')
    for number, path in files.items():
        with open(path, 'r') as f:
            content = f.read()
            links = link_pattern.findall(content)
            for link in links:
                if link not in files and link != number: # Don't error on self-ref or if file missing
                     # Only flagged if missing from both disk AND registry? 
                     # Actually, if it's in registry it might be "Planned".
                     if link not in registry:
                        errors.append(f"[LINK] {os.path.basename(path)} references RFC-{link} which does not exist.")

    if not errors:
        print("\n✅ AUDIT PASSED: 100% Alignment.")
    else:
        print(f"\n❌ AUDIT FAILED: {len(errors)} Issues Found:")
        for e in errors:
            print(e)
            
if __name__ == "__main__":
    audit()
