#!/usr/bin/env python3
"""
Two-Stage Node Interview Generator
Stage 1: Gemini API (fast, more slang)
Stage 2: Vertex AI (richer voice)
Merges best attributes from both passes
"""

import warnings
# Suppress Vertex AI deprecation warnings
warnings.filterwarnings("ignore", category=UserWarning, module="vertexai")
warnings.filterwarnings("ignore", message=".*deprecated.*")

import json
import time
import os
from datetime import datetime
from pathlib import Path

# Import prompts from baseline
from baseline_comparison import SYSTEM_PROMPT_V2, get_user_prompt_v2, evaluate_quality

OUTPUT_DIR = Path(__file__).parent / "output"
STAGE1_DIR = OUTPUT_DIR / "stage1_gemini"
STAGE2_DIR = OUTPUT_DIR / "stage2_vertex"
MERGED_DIR = OUTPUT_DIR / "merged_interviews"


def load_tasks_from_supabase():
    """Load tasks using cached file or fetch fresh."""
    tasks_file = OUTPUT_DIR / "ctas_tasks.json"
    if tasks_file.exists():
        with open(tasks_file, 'r') as f:
            return json.load(f)
    return []


def generate_gemini(task: dict, api_key: str) -> dict:
    """Stage 1: Gemini API generation."""
    import google.generativeai as genai

    genai.configure(api_key=api_key)
    model = genai.GenerativeModel(
        model_name="gemini-2.0-flash-exp",
        system_instruction=SYSTEM_PROMPT_V2
    )

    start = time.time()
    response = model.generate_content(
        get_user_prompt_v2(task),
        generation_config=genai.types.GenerationConfig(
            max_output_tokens=4096,
            temperature=0.7
        )
    )
    elapsed = time.time() - start

    content = response.text
    if "```json" in content:
        json_str = content.split("```json")[1].split("```")[0]
    elif "```" in content:
        json_str = content.split("```")[1].split("```")[0]
    else:
        json_str = content

    interview = json.loads(json_str.strip())
    interview["_meta"] = {
        "provider": "gemini_api",
        "time_seconds": round(elapsed, 2),
        "generated_at": datetime.now().isoformat()
    }
    return interview


STAGE2_SYSTEM_PROMPT = """You are an expert threat intelligence analyst reviewing and IMPROVING a node interview for CTAS (Cognitive Threat Analysis System) v7.3.1.

You will receive a draft interview. Your job is to ENHANCE it:

1. VOICE: Make it more specific, add more real APT examples, tools, and techniques
2. SEARCH: Add MORE slang terms, underground forum terminology, hacker speak
3. INDICATORS: Add more specific, actionable detection signatures
4. MITRE: Ensure all technique IDs are real and comprehensive
5. TOOLCHAIN: Add comprehensive tools across categories (kali, osint, commercial, automation, analysis, evasion)
6. CTAS_HOOKS: Add system integration hooks (pre_execution, post_execution, alert_triggers, integration_points)

CRITICAL: Keep the exact JSON structure. Return improved JSON only."""


def get_stage2_prompt(task: dict, stage1_result: dict) -> str:
    """Generate Stage 2 improvement prompt with Stage 1 output."""
    return f"""IMPROVE this node interview for: {task['task_name']}

ORIGINAL TASK:
- Category: {task['category']}
- HD4 Phase: {task['hd4_phase']}
- Description: {task['description']}

DRAFT INTERVIEW TO IMPROVE:
```json
{json.dumps(stage1_result, indent=2)}
```

IMPROVEMENTS NEEDED:
1. VOICE: Expand with more specific APT campaigns, real incidents, actual tools
2. SLANG: Add 5-10 MORE underground/forum terms (darkweb speak, chan boards, hacker forums)
3. INDICATORS: Add more specific network signatures, behavioral patterns
4. LONG_TAIL_PHRASES: Add phrases security analysts would actually search for
5. TOOLCHAIN: Expand with more tools - add automation (ansible, terraform), analysis (wireshark, yara, sigma), evasion (proxychains, obfuscation)
6. CTAS_HOOKS: ADD a ctas_hooks section with:
   - pre_execution: validation checks before task runs
   - post_execution: logging/update actions after task
   - alert_triggers: conditions that trigger alerts
   - integration_points: SIEM/ticketing/intel feed webhooks

Return the IMPROVED JSON with same structure. Keep all existing good content, ADD more including ctas_hooks."""


def generate_vertex(task: dict, stage1_result: dict = None) -> dict:
    """Stage 2: Vertex AI improves Stage 1 output."""
    import vertexai
    from vertexai.generative_models import GenerativeModel, GenerationConfig

    vertexai.init(project="gen-lang-client-0290627006", location="us-central1")
    model = GenerativeModel(
        model_name="gemini-2.0-flash-exp",
        system_instruction=STAGE2_SYSTEM_PROMPT
    )

    # Use improvement prompt if we have Stage 1 result
    if stage1_result:
        prompt = get_stage2_prompt(task, stage1_result)
    else:
        prompt = get_user_prompt_v2(task)

    start = time.time()
    response = model.generate_content(
        prompt,
        generation_config=GenerationConfig(
            max_output_tokens=4096,
            temperature=0.7
        )
    )
    elapsed = time.time() - start

    content = response.text
    if "```json" in content:
        json_str = content.split("```json")[1].split("```")[0]
    elif "```" in content:
        json_str = content.split("```")[1].split("```")[0]
    else:
        json_str = content

    interview = json.loads(json_str.strip())
    interview["_meta"] = {
        "provider": "vertex_ai",
        "mode": "improvement" if stage1_result else "standalone",
        "time_seconds": round(elapsed, 2),
        "generated_at": datetime.now().isoformat()
    }
    return interview


def merge_interviews(gemini: dict, vertex: dict) -> dict:
    """Merge best attributes from both interviews."""
    merged = {}

    # Use longer voice (richer content)
    gemini_voice = gemini.get("voice", "")
    vertex_voice = vertex.get("voice", "")
    merged["voice"] = vertex_voice if len(vertex_voice) > len(gemini_voice) else gemini_voice

    # Merge search terms (union of both)
    gemini_search = gemini.get("search", {})
    vertex_search = vertex.get("search", {})
    merged["search"] = {
        "keywords": list(set(gemini_search.get("keywords", []) + vertex_search.get("keywords", []))),
        "synonyms": list(set(gemini_search.get("synonyms", []) + vertex_search.get("synonyms", []))),
        "long_tail_phrases": list(set(gemini_search.get("long_tail_phrases", []) + vertex_search.get("long_tail_phrases", []))),
        "slang": list(set(gemini_search.get("slang", []) + vertex_search.get("slang", [])))
    }

    # Merge MITRE techniques (union)
    merged["mitre_techniques"] = list(set(
        gemini.get("mitre_techniques", []) + vertex.get("mitre_techniques", [])
    ))
    merged["d3fend_countermeasures"] = list(set(
        gemini.get("d3fend_countermeasures", []) + vertex.get("d3fend_countermeasures", [])
    ))

    # Merge APT examples (deduplicate by apt name)
    apt_map = {}
    for apt in gemini.get("apt_examples", []) + vertex.get("apt_examples", []):
        key = apt.get("apt", "")
        if key and key not in apt_map:
            apt_map[key] = apt
    merged["apt_examples"] = list(apt_map.values())

    # Merge indicators (union)
    gemini_ind = gemini.get("indicators", {})
    vertex_ind = vertex.get("indicators", {})
    merged["indicators"] = {
        "network": list(set(gemini_ind.get("network", []) + vertex_ind.get("network", []))),
        "behavioral": list(set(gemini_ind.get("behavioral", []) + vertex_ind.get("behavioral", []))),
        "temporal": list(set(gemini_ind.get("temporal", []) + vertex_ind.get("temporal", [])))
    }

    # Use Gemini for toolchain (typically more specific)
    merged["toolchain"] = gemini.get("toolchain", vertex.get("toolchain", {}))

    # Copy core fields from either (prefer gemini for consistency)
    for field in ["task_id", "task_name", "hd4_phase", "category", "purpose", "ownership",
                  "perspective_1n", "perspective_2n", "risk_dimensions",
                  "task_label", "is_key_indicator", "is_interdiction_point"]:
        merged[field] = gemini.get(field, vertex.get(field))

    # Add merge metadata
    merged["_meta"] = {
        "merge_strategy": "two_stage",
        "stage1_provider": "gemini_api",
        "stage2_provider": "vertex_ai",
        "stage1_time": gemini.get("_meta", {}).get("time_seconds", 0),
        "stage2_time": vertex.get("_meta", {}).get("time_seconds", 0),
        "merged_at": datetime.now().isoformat()
    }

    return merged


def run_two_stage(tasks: list, api_key: str, limit: int = None):
    """Run two-stage generation for all tasks."""
    STAGE1_DIR.mkdir(parents=True, exist_ok=True)
    STAGE2_DIR.mkdir(parents=True, exist_ok=True)
    MERGED_DIR.mkdir(parents=True, exist_ok=True)

    if limit:
        tasks = tasks[:limit]

    results = {
        "stage1": [],
        "stage2": [],
        "merged": [],
        "errors": []
    }

    print("=" * 70)
    print("TWO-STAGE NODE INTERVIEW GENERATION")
    print(f"Tasks: {len(tasks)}")
    print("=" * 70)

    for i, task in enumerate(tasks):
        task_id = task.get("task_id", f"task_{i}")
        task_name = task.get("task_name", "Unknown")
        print(f"\n[{i+1}/{len(tasks)}] {task_name}")

        gemini_result = None
        vertex_result = None

        # Stage 1: Gemini API
        print("  Stage 1 (Gemini API)...", end=" ", flush=True)
        try:
            gemini_result = generate_gemini(task, api_key)
            with open(STAGE1_DIR / f"{task_id}.json", 'w') as f:
                json.dump(gemini_result, f, indent=2)
            results["stage1"].append({"task_id": task_id, "status": "success"})
            print(f"OK ({gemini_result['_meta']['time_seconds']}s)")
        except Exception as e:
            print(f"ERROR: {e}")
            results["errors"].append({"task_id": task_id, "stage": 1, "error": str(e)})

        # Stage 2: Vertex AI (improves Stage 1 output)
        print("  Stage 2 (Vertex AI improving)...", end=" ", flush=True)
        try:
            vertex_result = generate_vertex(task, stage1_result=gemini_result)
            with open(STAGE2_DIR / f"{task_id}.json", 'w') as f:
                json.dump(vertex_result, f, indent=2)
            results["stage2"].append({"task_id": task_id, "status": "success"})
            print(f"OK ({vertex_result['_meta']['time_seconds']}s)")
        except Exception as e:
            print(f"ERROR: {e}")
            results["errors"].append({"task_id": task_id, "stage": 2, "error": str(e)})

        # Merge if both succeeded
        if gemini_result and vertex_result:
            print("  Merging...", end=" ", flush=True)
            merged = merge_interviews(gemini_result, vertex_result)
            with open(MERGED_DIR / f"{task_id}.json", 'w') as f:
                json.dump(merged, f, indent=2)
            results["merged"].append({"task_id": task_id, "status": "success"})

            # Quick quality check
            slang_count = len(merged.get("search", {}).get("slang", []))
            voice_len = len(merged.get("voice", ""))
            print(f"OK (voice: {voice_len} chars, slang: {slang_count} terms)")
        elif gemini_result:
            # Use Gemini only
            with open(MERGED_DIR / f"{task_id}.json", 'w') as f:
                json.dump(gemini_result, f, indent=2)
            results["merged"].append({"task_id": task_id, "status": "gemini_only"})
            print("  Using Gemini only")
        elif vertex_result:
            # Use Vertex only
            with open(MERGED_DIR / f"{task_id}.json", 'w') as f:
                json.dump(vertex_result, f, indent=2)
            results["merged"].append({"task_id": task_id, "status": "vertex_only"})
            print("  Using Vertex only")

    # Summary
    print("\n" + "=" * 70)
    print("TWO-STAGE GENERATION COMPLETE")
    print("=" * 70)
    print(f"Stage 1 (Gemini): {len(results['stage1'])} successful")
    print(f"Stage 2 (Vertex): {len(results['stage2'])} successful")
    print(f"Merged:           {len(results['merged'])} interviews")
    print(f"Errors:           {len(results['errors'])}")
    print(f"\nOutput directories:")
    print(f"  Stage 1: {STAGE1_DIR}")
    print(f"  Stage 2: {STAGE2_DIR}")
    print(f"  Merged:  {MERGED_DIR}")

    # Save run summary
    summary_file = OUTPUT_DIR / "two_stage_run_summary.json"
    with open(summary_file, 'w') as f:
        json.dump({
            "generated_at": datetime.now().isoformat(),
            "total_tasks": len(tasks),
            "results": results
        }, f, indent=2)
    print(f"\nRun summary: {summary_file}")

    return results


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Two-Stage Node Interview Generator")
    parser.add_argument("--limit", type=int, help="Limit number of tasks")
    parser.add_argument("--dry-run", action="store_true", help="Show what would be processed")
    args = parser.parse_args()

    # Load credentials
    vault_path = Path(__file__).parent.parent / "credentials-vault" / "command-center-credentials.json"
    api_key = None
    if vault_path.exists():
        with open(vault_path, 'r') as f:
            vault = json.load(f)
            for key_name, entries in vault.get("credentials", {}).items():
                if key_name == "GEMINI_API_KEY" and entries:
                    best = max(entries, key=lambda x: x.get("confidence", 0))
                    api_key = best.get("value")
                    break

    if not api_key:
        api_key = os.environ.get("GEMINI_API_KEY")

    if not api_key:
        print("ERROR: No Gemini API key found in vault or environment")
        return

    # Load tasks
    tasks = load_tasks_from_supabase()
    if not tasks:
        print("ERROR: No tasks found. Run export first.")
        return

    if args.dry_run:
        print(f"Would process {len(tasks[:args.limit] if args.limit else tasks)} tasks")
        return

    run_two_stage(tasks, api_key, limit=args.limit)


if __name__ == "__main__":
    main()
