#!/usr/bin/env python3
"""
CTAS Crate Interview Generator
==============================

Generates first-person "interviews" for Rust crates in the CTAS-7 workspace.
Each crate speaks as if it were a sentient component explaining its purpose,
dependencies, and role in the system.

Port: 18146 (crate interview service)
"""

import json
import time
import os
import asyncio
from datetime import datetime
from pathlib import Path
from typing import Optional, Callable
import google.generativeai as genai

OUTPUT_DIR = Path(__file__).parent / "output"

CRATE_SYSTEM_PROMPT = """You are an expert Rust systems engineer interviewing CTAS-7 crates.
Generate a first-person "interview" where the crate speaks about itself.

The crate should describe:
1. Its core purpose and mission in the CTAS-7 system
2. Key capabilities and what makes it unique
3. Its dependencies and why they matter
4. How it integrates with other CTAS-7 components
5. Performance characteristics and design decisions
6. Security considerations if applicable

Output MUST be valid JSON matching this schema:
{
  "crate_id": "string (crate identifier)",
  "crate_name": "string (package name)",
  "voice": "string (first-person narrative, 200-400 words)",
  "purpose": "string (one-line mission statement)",
  "capabilities": ["list of key capabilities"],
  "dependencies": {
    "critical": ["deps the crate cannot function without"],
    "optional": ["optional/feature-gated deps"]
  },
  "integration_points": ["list of other CTAS-7 components it connects to"],
  "design_patterns": ["architectural patterns used"],
  "performance": {
    "characteristics": "string describing perf profile",
    "optimizations": ["list of optimizations"]
  },
  "foundation_blocks": {
    "uses": ["foundation blocks this crate uses"],
    "provides": ["foundation blocks this crate provides"]
  },
  "h1_operational": "string (64-char operational hash placeholder)",
  "h2_semantic": "string (semantic understanding hash placeholder)"
}

Respond ONLY with the JSON object. No markdown, no explanation."""


def get_crate_prompt(crate: dict) -> str:
    """Build interview prompt for a crate."""
    deps_str = ", ".join(crate.get('dependencies', [])[:15])

    return f"""Interview the following CTAS-7 Rust crate:

Crate Name: {crate.get('crate_name', 'Unknown')}
Version: {crate.get('version', '0.0.0')}
Path: {crate.get('path', '')}
Edition: {crate.get('edition', '2021')}
Description: {crate.get('description', 'No description available')}
Dependencies ({crate.get('dep_count', 0)} total): {deps_str}

Generate a comprehensive first-person interview where this crate explains itself.
Focus on its role in the CTAS-7 cognitive threat analysis system.

If this is a foundation crate (ctas7-foundation-*), emphasize the Smart Crate System
and how it provides reusable blocks for other crates.

If this is an operational crate (ctas7-cdn-*, ctas7-qa-*, etc.), focus on its
specific mission and how it uses foundation blocks."""


class CrateInterviewGenerator:
    """Generator for Rust crate interviews."""

    def __init__(self, api_key: str):
        self.api_key = api_key
        genai.configure(api_key=api_key)
        self.model = genai.GenerativeModel(
            model_name="gemini-2.0-flash-exp",
            system_instruction=CRATE_SYSTEM_PROMPT
        )

    async def generate_interview(
        self,
        crate: dict,
        on_token: Optional[Callable[[str], None]] = None
    ) -> dict:
        """Generate interview for a single crate."""
        crate_id = crate.get('crate_id', 'unknown')
        crate_name = crate.get('crate_name', 'Unknown')

        prompt = get_crate_prompt(crate)
        start = time.time()

        full_response = ""
        try:
            response = self.model.generate_content(
                prompt,
                generation_config=genai.types.GenerationConfig(
                    max_output_tokens=2048,
                    temperature=0.7
                ),
                stream=True
            )

            for chunk in response:
                if chunk.text:
                    full_response += chunk.text
                    if on_token:
                        on_token(chunk.text)
                    await asyncio.sleep(0)

        except Exception as e:
            raise RuntimeError(f"Generation failed for {crate_name}: {e}")

        elapsed = time.time() - start

        # Parse JSON
        content = full_response
        if "```json" in content:
            json_str = content.split("```json")[1].split("```")[0]
        elif "```" in content:
            json_str = content.split("```")[1].split("```")[0]
        else:
            json_str = content

        interview = json.loads(json_str.strip())
        interview["_meta"] = {
            "provider": "gemini_crate_interview",
            "time_seconds": round(elapsed, 2),
            "generated_at": datetime.now().isoformat(),
            "source_path": crate.get('path', ''),
            "version": crate.get('version', '')
        }

        return interview

    async def run_batch(
        self,
        crates: list,
        output_dir: Path,
        on_progress: Optional[Callable[[int, int, str], None]] = None
    ) -> dict:
        """Run batch generation for all crates."""
        output_dir.mkdir(parents=True, exist_ok=True)

        total = len(crates)
        print(f"\n{'='*70}")
        print("CRATE INTERVIEW GENERATION")
        print(f"Crates: {total}")
        print(f"{'='*70}\n")

        results = {"success": 0, "failed": 0, "interviews": []}

        for i, crate in enumerate(crates):
            crate_id = crate.get('crate_id', f'crate_{i}')
            crate_name = crate.get('crate_name', 'Unknown')

            print(f"[{i+1}/{total}] {crate_name}")
            print(f"  Streaming: ", end="", flush=True)

            if on_progress:
                on_progress(i + 1, total, crate_name)

            token_count = [0]
            def on_token(token: str):
                token_count[0] += 1
                if token_count[0] % 30 == 0:
                    print(".", end="", flush=True)

            try:
                interview = await self.generate_interview(crate, on_token=on_token)

                # Save immediately
                out_file = output_dir / f"{crate_id}.json"
                with open(out_file, 'w') as f:
                    json.dump(interview, f, indent=2)

                voice_len = len(interview.get('voice', ''))
                caps_count = len(interview.get('capabilities', []))
                print(f" OK ({interview['_meta']['time_seconds']}s, {voice_len} chars, {caps_count} caps)")

                results["success"] += 1
                results["interviews"].append({
                    "crate_id": crate_id,
                    "crate_name": crate_name,
                    "voice_length": voice_len
                })

            except Exception as e:
                print(f" ERROR: {e}")
                results["failed"] += 1

        print(f"\n{'='*70}")
        print("CRATE INTERVIEW GENERATION COMPLETE")
        print(f"{'='*70}")
        print(f"Success: {results['success']}")
        print(f"Failed:  {results['failed']}")
        print(f"Output:  {output_dir}")

        return results


async def main():
    import argparse
    parser = argparse.ArgumentParser(description="Crate Interview Generator")
    parser.add_argument("--limit", type=int, help="Limit number of crates")
    parser.add_argument("--crate", type=str, help="Generate for specific crate name")
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
        print("ERROR: No Gemini API key found")
        return

    # Load crates
    crates_file = OUTPUT_DIR / "ctas_crates.json"
    if not crates_file.exists():
        print("ERROR: No crates found. Run crate extraction first.")
        return

    with open(crates_file, 'r') as f:
        crates = json.load(f)

    # Filter if specific crate requested
    if args.crate:
        crates = [c for c in crates if args.crate in c.get('crate_name', '')]
        if not crates:
            print(f"ERROR: No crate matching '{args.crate}'")
            return

    if args.limit:
        crates = crates[:args.limit]

    # Run generation
    generator = CrateInterviewGenerator(api_key=api_key)
    output_dir = OUTPUT_DIR / "crate_interviews"

    results = await generator.run_batch(crates=crates, output_dir=output_dir)

    # Save summary
    summary_file = output_dir / "generation_summary.json"
    with open(summary_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nSummary saved to {summary_file}")


if __name__ == "__main__":
    asyncio.run(main())
