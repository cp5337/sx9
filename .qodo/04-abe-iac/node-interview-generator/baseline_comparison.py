#!/usr/bin/env python3
"""
Baseline Quality Comparison - 3 LLM Providers
Establishes foundation benchmarks for node interview generation
"""

import json
import time
import os
from pathlib import Path
from datetime import datetime

# Refined voice format based on user requirements
SYSTEM_PROMPT_V2 = """You are an expert threat intelligence analyst creating node interviews for CTAS (Cognitive Threat Analysis System) v7.3.1.

Generate a FIRST-PERSON adversary narrative. The node speaks AS the adversary capability itself.

VOICE FORMAT (REQUIRED STRUCTURE):
"I am [Node Name]. My objectives are [goals/mission].

I am found in attacks such as [APT examples with years]. I operate in locations such as [geographic/platform targets]. I use tools such as [specific tools/techniques].

I maintain [persistence mechanisms]. I remain undetected by [OPSEC/evasion]. My potential indicators are [detection signatures]."

SEARCH OPTIMIZATION (REQUIRED):
Include keywords, synonyms, long-tail phrases, AND slang/forum terminology for embedding and detection systems.

OUTPUT: Return valid JSON matching the schema exactly."""

def get_user_prompt_v2(task: dict) -> str:
    """Enhanced prompt with refined voice and search requirements."""
    return f"""Generate a node interview for this CTAS task:

TASK: {task['task_name']}
CATEGORY: {task['category']}
HD4 PHASE: {task['hd4_phase']}
DESCRIPTION: {task['description']}

Return JSON with this structure:
{{
    "task_id": "{task['task_id']}",
    "task_name": "{task['task_name']}",
    "hd4_phase": "{task['hd4_phase']}",
    "category": "{task['category']}",

    "voice": "I am {task['task_name']}. My objectives are [specific goals]. I am found in attacks such as [APT29/SolarWinds 2020, APT28/DNC 2016]. I operate in locations such as [targets]. I use tools such as [specific tools]. I maintain [persistence]. I remain undetected by [OPSEC methods]. My potential indicators are [detection points].",

    "purpose": "[One sentence: what this task accomplishes]",
    "ownership": "[One sentence: what this task owns in the kill chain]",

    "perspective_1n": "[Defender view: What am I seeing? How do I detect this?]",
    "perspective_2n": "[Adversary view: Why do I need this? What does success look like?]",

    "mitre_techniques": ["T1595.001", "T1592"],
    "d3fend_countermeasures": ["D3-NTA", "D3-SYSM"],

    "toolchain": {{
        "kali": ["nmap", "masscan", "metasploit", "burpsuite"],
        "osint": ["Shodan", "Censys", "Maltego", "SpiderFoot"],
        "commercial": ["Wazuh", "Splunk", "CrowdStrike", "Carbon Black"],
        "automation": ["ansible", "terraform", "python scripts"],
        "analysis": ["wireshark", "volatility", "yara", "sigma rules"],
        "evasion": ["proxychains", "tor", "obfuscation tools"]
    }},

    "ctas_hooks": {{
        "pre_execution": ["validate_target", "check_authorization"],
        "post_execution": ["log_results", "update_slotgraph"],
        "alert_triggers": ["threshold_breach", "anomaly_detected"],
        "integration_points": ["siem_webhook", "ticket_system", "threat_intel_feed"]
    }},

    "indicators": {{
        "network": ["Specific network signatures"],
        "behavioral": ["Behavioral patterns"],
        "temporal": ["Timing patterns"]
    }},

    "search": {{
        "keywords": ["5-10 short searchable terms"],
        "synonyms": ["alternative names for this technique"],
        "long_tail_phrases": ["multi-word search phrases for detection"],
        "slang": ["forum/underground terminology - darkweb speak, hacker slang"]
    }},

    "apt_examples": [
        {{"apt": "APT29", "campaign": "SolarWinds", "year": 2020}},
        {{"apt": "APT28", "campaign": "DNC Hack", "year": 2016}}
    ],

    "risk_dimensions": {{
        "likelihood": 0.7,
        "impact": 0.6,
        "detectability": 0.5,
        "reversibility": 0.3
    }},

    "task_label": "mandatory|desirable|optional",
    "is_key_indicator": true,
    "is_interdiction_point": false
}}

CRITICAL:
- Voice MUST follow the exact format: "I am X. My objectives are... I am found in... I operate in... I use tools such as... I maintain... I remain undetected by... My potential indicators are..."
- Search section MUST include slang (underground forum terms, hacker speak)
- All MITRE IDs must be real
- APT examples must be historically accurate
- Be SPECIFIC not generic"""


def test_gemini_api(task: dict, api_key: str) -> dict:
    """Test Gemini API provider."""
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

    return {
        "provider": "gemini_api",
        "model": "gemini-2.0-flash-exp",
        "time_seconds": round(elapsed, 2),
        "interview": json.loads(json_str.strip()),
        "raw_length": len(content)
    }


def test_vertex_ai(task: dict) -> dict:
    """Test Vertex AI provider (uses gcloud auth)."""
    import vertexai
    from vertexai.generative_models import GenerativeModel, GenerationConfig

    vertexai.init(project="gen-lang-client-0290627006", location="us-central1")
    model = GenerativeModel(
        model_name="gemini-2.0-flash-exp",
        system_instruction=SYSTEM_PROMPT_V2
    )

    start = time.time()
    response = model.generate_content(
        get_user_prompt_v2(task),
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

    return {
        "provider": "vertex_ai",
        "model": "gemini-2.0-flash-exp",
        "time_seconds": round(elapsed, 2),
        "interview": json.loads(json_str.strip()),
        "raw_length": len(content)
    }


def test_anthropic(task: dict, api_key: str) -> dict:
    """Test Anthropic Claude provider."""
    import anthropic

    client = anthropic.Anthropic(api_key=api_key)

    start = time.time()
    response = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=4096,
        system=SYSTEM_PROMPT_V2,
        messages=[{"role": "user", "content": get_user_prompt_v2(task)}]
    )
    elapsed = time.time() - start

    content = response.content[0].text
    if "```json" in content:
        json_str = content.split("```json")[1].split("```")[0]
    elif "```" in content:
        json_str = content.split("```")[1].split("```")[0]
    else:
        json_str = content

    return {
        "provider": "anthropic",
        "model": "claude-sonnet-4-20250514",
        "time_seconds": round(elapsed, 2),
        "interview": json.loads(json_str.strip()),
        "raw_length": len(content)
    }


def test_openai(task: dict, api_key: str) -> dict:
    """Test OpenAI GPT-4 provider."""
    from openai import OpenAI

    client = OpenAI(api_key=api_key)

    start = time.time()
    response = client.chat.completions.create(
        model="gpt-4o",
        max_tokens=4096,
        temperature=0.7,
        messages=[
            {"role": "system", "content": SYSTEM_PROMPT_V2},
            {"role": "user", "content": get_user_prompt_v2(task)}
        ]
    )
    elapsed = time.time() - start

    content = response.choices[0].message.content
    if "```json" in content:
        json_str = content.split("```json")[1].split("```")[0]
    elif "```" in content:
        json_str = content.split("```")[1].split("```")[0]
    else:
        json_str = content

    return {
        "provider": "openai",
        "model": "gpt-4o",
        "time_seconds": round(elapsed, 2),
        "interview": json.loads(json_str.strip()),
        "raw_length": len(content)
    }


def evaluate_quality(result: dict) -> dict:
    """Evaluate interview quality against requirements."""
    interview = result.get("interview", {})
    voice = interview.get("voice", "")
    search = interview.get("search", {})

    scores = {
        "voice_format": 0,
        "search_completeness": 0,
        "mitre_present": 0,
        "apt_examples": 0,
        "specificity": 0
    }

    # Voice format check
    voice_elements = [
        "My objectives are",
        "found in attacks such as",
        "operate in locations",
        "use tools such as",
        "maintain",
        "remain undetected",
        "indicators are"
    ]
    voice_score = sum(1 for e in voice_elements if e.lower() in voice.lower())
    scores["voice_format"] = round(voice_score / len(voice_elements), 2)

    # Search completeness
    search_fields = ["keywords", "synonyms", "long_tail_phrases", "slang"]
    search_score = sum(1 for f in search_fields if search.get(f) and len(search.get(f, [])) > 0)
    scores["search_completeness"] = round(search_score / len(search_fields), 2)

    # MITRE presence
    has_mitre = bool(interview.get("mitre_techniques")) and len(interview.get("mitre_techniques", [])) > 0
    has_d3fend = bool(interview.get("d3fend_countermeasures")) and len(interview.get("d3fend_countermeasures", [])) > 0
    scores["mitre_present"] = 1.0 if (has_mitre and has_d3fend) else 0.5 if has_mitre else 0.0

    # APT examples
    apt_list = interview.get("apt_examples", [])
    scores["apt_examples"] = min(1.0, len(apt_list) / 2)

    # Specificity (voice length as proxy)
    scores["specificity"] = min(1.0, len(voice) / 800)

    # Overall score
    overall = sum(scores.values()) / len(scores)

    return {
        "scores": scores,
        "overall": round(overall, 2),
        "voice_length": len(voice),
        "keywords_count": len(search.get("keywords", [])),
        "slang_count": len(search.get("slang", []))
    }


def main():
    # Sample task for baseline
    task = {
        "task_id": "uuid-000-000-001",
        "task_name": "Ideological Formation",
        "category": "Ideation",
        "hd4_phase": "Hunt",
        "description": "Forming motivations via exposure."
    }

    # Load credentials
    vault_path = Path(__file__).parent.parent / "credentials-vault" / "command-center-credentials.json"
    vault_keys = {}
    if vault_path.exists():
        with open(vault_path, 'r') as f:
            vault = json.load(f)
            for key_name, entries in vault.get("credentials", {}).items():
                if entries and isinstance(entries, list):
                    best = max(entries, key=lambda x: x.get("confidence", 0))
                    vault_keys[key_name] = best.get("value")

    results = []

    print("=" * 70)
    print("BASELINE QUALITY COMPARISON - 3 LLM Providers")
    print(f"Task: {task['task_name']} ({task['hd4_phase']})")
    print("=" * 70)

    # Test Gemini API
    gemini_key = vault_keys.get("GEMINI_API_KEY") or os.environ.get("GEMINI_API_KEY")
    if gemini_key:
        print("\n[1/3] Testing Gemini API...")
        try:
            result = test_gemini_api(task, gemini_key)
            result["quality"] = evaluate_quality(result)
            results.append(result)
            print(f"  Time: {result['time_seconds']}s")
            print(f"  Quality: {result['quality']['overall']}")
        except Exception as e:
            print(f"  ERROR: {e}")
            results.append({"provider": "gemini_api", "error": str(e)})
    else:
        print("\n[1/3] Gemini API: SKIPPED (no key)")

    # Test Vertex AI
    print("\n[2/3] Testing Vertex AI...")
    try:
        result = test_vertex_ai(task)
        result["quality"] = evaluate_quality(result)
        results.append(result)
        print(f"  Time: {result['time_seconds']}s")
        print(f"  Quality: {result['quality']['overall']}")
    except Exception as e:
        print(f"  ERROR: {e}")
        results.append({"provider": "vertex_ai", "error": str(e)})

    # Test OpenAI
    openai_key = vault_keys.get("OPENAI_API_KEY") or os.environ.get("OPENAI_API_KEY")
    if openai_key:
        print("\n[3/4] Testing OpenAI GPT-4o...")
        try:
            result = test_openai(task, openai_key)
            result["quality"] = evaluate_quality(result)
            results.append(result)
            print(f"  Time: {result['time_seconds']}s")
            print(f"  Quality: {result['quality']['overall']}")
        except Exception as e:
            print(f"  ERROR: {e}")
            results.append({"provider": "openai", "error": str(e)})
    else:
        print("\n[3/4] OpenAI: SKIPPED (no key in vault)")

    # Test Anthropic
    anthropic_key = vault_keys.get("ANTHROPIC_API_KEY") or os.environ.get("ANTHROPIC_API_KEY")
    if anthropic_key:
        print("\n[4/4] Testing Anthropic Claude...")
        try:
            result = test_anthropic(task, anthropic_key)
            result["quality"] = evaluate_quality(result)
            results.append(result)
            print(f"  Time: {result['time_seconds']}s")
            print(f"  Quality: {result['quality']['overall']}")
        except Exception as e:
            print(f"  ERROR: {e}")
            results.append({"provider": "anthropic", "error": str(e)})
    else:
        print("\n[4/4] Anthropic: SKIPPED (no key in vault)")

    # Summary
    print("\n" + "=" * 70)
    print("BASELINE RESULTS SUMMARY")
    print("=" * 70)

    for r in results:
        if "error" in r:
            print(f"\n{r['provider']}: ERROR - {r['error']}")
        else:
            q = r["quality"]
            print(f"\n{r['provider']} ({r['model']}):")
            print(f"  Time: {r['time_seconds']}s")
            print(f"  Overall Quality: {q['overall']}")
            print(f"  Voice Format: {q['scores']['voice_format']}")
            print(f"  Search Complete: {q['scores']['search_completeness']}")
            print(f"  MITRE Present: {q['scores']['mitre_present']}")
            print(f"  Voice Length: {q['voice_length']} chars")
            print(f"  Slang Terms: {q['slang_count']}")

    # Save results
    output_file = Path(__file__).parent / "output" / "baseline_comparison.json"
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, 'w') as f:
        json.dump({
            "generated_at": datetime.now().isoformat(),
            "task": task,
            "results": results
        }, f, indent=2, default=str)

    print(f"\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()
