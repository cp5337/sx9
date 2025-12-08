#!/Users/cp5337/Developer/sx9/sx9-conda/.conda/bin/python
"""
Antigravity Bootloader Orchestrator
"""
import sys
import os
from pathlib import Path

# --- Environment Setup ---
# Add canonical library paths to sys.path AND environment for subprocesses
SX9_CONDA_ROOT = Path("/Users/cp5337/Developer/sx9/sx9-conda")
site_packages = SX9_CONDA_ROOT / "python-packages"
src_root = SX9_CONDA_ROOT / "src"

sys.path.append(str(src_root))
sys.path.append(str(site_packages))

# Ensure subprocesses (and the agent) have this context
current_path = os.environ.get("PYTHONPATH", "")
extra_paths = f"{src_root}:{site_packages}"
os.environ["PYTHONPATH"] = f"{extra_paths}:{current_path}" if current_path else extra_paths
# -------------------------

try:
    import tomllib as toml
except ImportError:
    try:
        import toml
    except ImportError:
        print("❌ Error: No TOML parser found. Install 'toml' or use Python 3.11+")
        sys.exit(1)

try:
    from atomic_tools import DistilBertRouter, csearch, clook
    AI_ENABLED = True
except ImportError:
    print("⚠️  Warning: Could not import Atomic Tools. AI enhancement disabled.")
    AI_ENABLED = False

MANIFEST_PATH = Path("boot_manifest.toml")
BOOT_SEQUENCE_TARGET = Path("# ANTIGRAVITY AGENT BOOT SEQUENCE")
CANONICAL_PYTHON = sys.executable
CANONICAL_PYTHONPATH = os.environ["PYTHONPATH"]

def enhance_objective(objective):
    """Uses Atomic Tools to find context relevant to the objective."""
    if not AI_ENABLED or not objective:
        return None
        
    print(f"🧠 Thalamic Filter: Analyzing objective '{objective}'...")
    try:
        router = DistilBertRouter()
        # We classify the intent. If it's "find file" or "search content", we execute.
        
        # Determine intent (Re-using router logic but capturing return)
        # Note: Router.route currently executes. We might need to tap into it or just trust it.
        # Ideally we'd use router.classifier directly, but let's use the router's logic to keep it DRY.
        # But atomic_tools.csearch now returns a list, so we can capture output!
        
        # We'll use the classifier directly here for fine-grained control or refactor router.
        # Let's inspect router.action_map.
        
        result = router.classifier(objective, router.labels)
        top_intent = result['labels'][0]
        confidence = result['scores'][0]
        
        print(f"   Intent: '{top_intent}' ({confidence:.2f})")
        
        enhancement = []
        if confidence > 0.5:
            args = router._extract_arg(objective, top_intent)
            if top_intent == "find file":
                print(f"   🔎 Auto-searching for files: {args}")
                files = csearch(args)
                if files:
                    enhancement.append(f"Auto-Context (Files): {', '.join(files[:5])}")
            elif top_intent == "search content":
                print(f"   🔎 Auto-scanning content for: {args}")
                matches = clook(args)
                if matches:
                    enhancement.append(f"Auto-Context (Matches): {len(matches)} found.")
        
        return " | ".join(enhancement) if enhancement else None

    except Exception as e:
        print(f"⚠️  Enhancement failed: {e}")
        return None

def infer_vertical(objective):
    """Uses DistilBERT to guess the best Vertical based on objective."""
    if not AI_ENABLED or not objective:
        return None
        
    try:
        router = DistilBertRouter()
        # Define Vertical-specific labels
        labels = ["cyber security attack", "devops deployment infrastructure", "satellite orbital physics"]
        label_map = {
            "cyber security attack": "ctas",
            "devops deployment infrastructure": "devops",
            "satellite orbital physics": "orbital"
        }
        
        result = router.classifier(objective, labels)
        top_label = result['labels'][0]
        confidence = result['scores'][0]
        
        if confidence > 0.4:
            vertical = label_map[top_label]
            print(f"🧠 DistilBERT Inference: '{objective}' maps to VERTICAL: {vertical.upper()} ({confidence:.2f})")
            return vertical
    except Exception as e:
        print(f"⚠️  Vertical inference failed: {e}")
        
    return None

def load_manifest():
    with open(MANIFEST_PATH, "rb") as f:
        return toml.load(f)

def check_environment():
    """Verifies the Canonical Environment is active and capable."""
    print(f"🐍 Python: {CANONICAL_PYTHON}")
    print(f"📚 PYTHONPATH: {CANONICAL_PYTHONPATH}")
    
    required_libs = ["pandas", "requests", "toml"] # Basic check
    missing = []
    
    for lib in required_libs:
        try:
            __import__(lib)
            print(f"   [OK] {lib}")
        except ImportError:
            print(f"   [FAIL] {lib}")
            missing.append(lib)
            
    if missing:
        print(f"⚠️  Environment incomplete. Missing: {missing}")
        return False
    print("✅ Canonical Environment Verified.")
    return True

def update_boot_sequence(mode, config):
    print(f"🔄 Booting into mode: {mode}")
    
    with open(BOOT_SEQUENCE_TARGET, "r") as f:
        content = f.read()
    
HUD_TEMPLATE = """
## 5. THE HUD (CANONICAL STATE TRACKER)
⚠️ **CRITICAL PROTOCOL:** You MUST begin **EVERY** response with this dashboard. It is the only way to track state across context windows. Do not output text before this block.

```text
| SYSTEM: ONLINE | FABRIC: [NATS:UNKNOWN] | RFC: [STRICT] |
|----------------|------------------------|---------------|
| MISSION       : [Current Objective from Section 4]      |
| CONTEXT       : [Active Files / Focus]                  |
| FABRIC STATE  : [Message Rate / Active Channels]        |
| HEALTH        : [Component Status / Alerts]             |
| PHASE         : [PLAN | ACT | VERIFY]                   |
```
"""

import argparse

# ... (HUD_TEMPLATE remains unchanged) ...

def update_boot_sequence(mode, config, vertical_config=None, objective_override=None, shuttle_content=None):
    print(f"🔄 Booting into mode: {mode}")
    if vertical_config:
        print(f"   Vertical: {vertical_config.get('description', 'Unknown')}")
    
    with open(BOOT_SEQUENCE_TARGET, "r") as f:
        content = f.read()
    
    new_content = []
    lines = content.splitlines()
    
    # Inject Environment Metadata
    env_injected = False
    
    in_mission_section = False

    # Filter out existing HUD if present to avoid duplication on re-runs
    filtered_lines = []
    skip_hud = False
    for line in lines:
        if line.strip().startswith("## 5. THE HUD"):
            skip_hud = True
        if not skip_hud:
            filtered_lines.append(line)

    for line in filtered_lines:
        if line.strip().startswith("## 4. MISSION"):
            in_mission_section = True
        elif line.startswith("## "):
            in_mission_section = False
            
        if in_mission_section and line.strip().startswith("**CONTEXT:**"):
            context_line = f"**CONTEXT:** We are in HD4 Phase: **{mode}**."
            if vertical_config:
                context_line += f" | Vertical: **{vertical_config.get('context_prefix')}**"
            new_content.append(context_line)
            # Inject Shuttle Content immediately after Context line
            if shuttle_content:
                new_content.append(f"> **SHUTTLE PACKET:** {shuttle_content}")

        elif in_mission_section and line.strip().startswith("**OBJECTIVE:**"):
            objective = objective_override if objective_override else config['description']
            new_content.append(f"**OBJECTIVE:** {objective}")
        elif "> Loading `cognitive_engine.yaml`" in line or "> Loading `soft_dev_brain.toml`" in line:
             new_content.append(f"> Loading `{config['brain']}`... [OK]")
        elif "> Loading `agent_identity.toml`" in line:
             new_content.append(f"> Loading `{config['identity']}`... [OK]")
             if not env_injected:
                 new_content.append(f"> **RUNTIME:** `{CANONICAL_PYTHON}`")
                 new_content.append(f"> **CONTEXT:** `{CANONICAL_PYTHONPATH}`")
                 env_injected = True
        elif "**RUNTIME:**" in line or (line.strip().startswith("> **CONTEXT:**") and not in_mission_section):
            pass # Skip existing injected environment lines to avoid duplication
        elif "> **SHUTTLE PACKET:**" in line:
            pass # Skip previous shuttle packets
        else:
            new_content.append(line)
            
    # Append HUD
    new_content.append(HUD_TEMPLATE)
            
    with open(BOOT_SEQUENCE_TARGET, "w") as f:
        f.write("\n".join(new_content) + "\n")
        
    print("✅ Boot sequence updated.")

def architect_mode():
    """Interactive Wizard powered by DistilBERT."""
    print("\n🏗️  ATOMIC ARCHITECT: Let's build your mission context.\n")
    
    # Step 1: Objective
    objective = input("1. What is your primary objective? (e.g., 'Scan the firewall for open ports')\n> ").strip()
    if not objective:
        print("Objective required.")
        sys.exit(1)
        
    # Step 2: AI Inference
    vertical = infer_vertical(objective)
    if vertical:
        confirm = input(f"   🤖 Suggested Vertical: {vertical.upper()}. Accept? [Y/n] ").strip().lower()
        if confirm == 'n':
            vertical = input("   Enter Vertical (devops/ctas/orbital): ").strip()
    else:
        vertical = input("   Enter Vertical (devops/ctas/orbital): ").strip()
        
    # Step 3: Context Drag-and-Drop
    print("\n2. Context Injection (Drag & Drop files/folders here, Enter to finish):")
    context_paths = []
    while True:
        path_input = input("   > ").strip()
        if not path_input:
             break
        
        # Handle multiple paths or quoted paths from drag-drop
        # Naive split by space (imperfect for spaces in filenames but standard for simple drag-drop)
        # Better: use shlex.split if imported, but let's assume simple paths for now or single drag.
        
        p = Path(path_input.replace("\\", "").strip("'").strip('"')) # Clean up escape chars
        if p.exists():
            print(f"     ✅ Linked: {p.name}")
            context_paths.append(str(p.absolute()))
            # Auto-enhance: if dir, maybe list top files?
        else:
            print(f"     ❌ Path not found: {path_input}")
            
    # Compile
    shuttle_content = f"Manually Injected Context:\n" + "\n".join(context_paths)
    
    return vertical, objective, shuttle_content

def main():
    parser = argparse.ArgumentParser(description="Antigravity Bootloader")
    parser.add_argument("mode", nargs="?", help="Operational Mode (e.g., DEVELOPMENT)")
    parser.add_argument("--check-env", action="store_true", help="Verify environment integrity")
    parser.add_argument("--vertical", help="Target Vertical (devops, ctas, orbital)")
    parser.add_argument("--objective", help="Override Mission Objective")
    parser.add_argument("--shuttle", help="Path to context file (or '-' for stdin)")
    parser.add_argument("--architect", action="store_true", help="Launch Interactive Prompt Architect")
    
    args = parser.parse_args()

    # Architect Mode Override
    if args.architect:
        vert, obj, shut = architect_mode()
        args.vertical = vert
        args.objective = obj
        args.shuttle = None # Handled via parsed return
        # Merge manual shuttle content
        if shut:
             # We need to pass this manually since args.shuttle is path-based in logic below
             # Let's hack it: we'll set args.shuttle to "-" to trigger stdin logic? 
             # No, main logic expects a file or stdin.
             # We will just pass `shut` to update_boot_sequence directly.
             pass
    else:
        vert = args.vertical
        obj = args.objective
        shut = None

    if args.check_env:
        if not check_environment():
            sys.exit(1)
        if not args.mode:
            sys.exit(0)
            
    manifest = load_manifest()
    
    selected_mode = args.mode if args.mode else manifest["default_mode"]
        
    if selected_mode not in manifest["modes"]:
        print(f"❌ Invalid mode: {selected_mode}")
        print(f"Available modes: {list(manifest['modes'].keys())}")
        sys.exit(1)
    
    # Process Vertical
    vertical_config = None
    if vert:
        verticals = manifest.get("verticals", {})
        if vert in verticals:
            vertical_config = verticals[vert]
        else:
            print(f"⚠️  Warning: Vertical '{vert}' not found in manifest.")
            
    # Process Shuttle (CLI Argument)
    shuttle_content = shut
    if args.shuttle:
        if args.shuttle == "-":
            print("📥 Reading shuttle packet from stdin...")
            shuttle_content = sys.stdin.read().strip()
        else:
            p = Path(args.shuttle)
            if p.exists():
                shuttle_content = p.read_text().strip()
            else:
                print(f"⚠️  Shuttle file not found: {args.shuttle}")
                
    # Smart Enhancement (AI Filter)
    if obj:
         enhancement = enhance_objective(obj)
         if enhancement:
             print(f"✨ Enhancing Shuttle Packet with: {enhancement}")
             if shuttle_content:
                 shuttle_content += f"\n\n[SYSTEM ENHANCEMENT]: {enhancement}"
             else:
                 shuttle_content = f"[SYSTEM ENHANCEMENT]: {enhancement}"

    update_boot_sequence(
        selected_mode, 
        manifest["modes"][selected_mode], 
        vertical_config=vertical_config,
        objective_override=obj,
        shuttle_content=shuttle_content
    )

if __name__ == "__main__":
    os.chdir(Path(__file__).parent) # Ensure running from script dir
    main()
