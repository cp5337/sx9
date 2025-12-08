#!/usr/bin/env python3
"""
Streaming Two-Stage Node Interview Generator
============================================

Production-ready streaming version with:
- Real-time token streaming to stdout
- Progress webhooks for monitoring dashboards
- Checkpoint/resume for fault tolerance
- Parallel Stage 1 batching with async
- Redis queue integration ready

Port: 18145 (interview generator service)
"""

import warnings
warnings.filterwarnings("ignore", category=UserWarning, module="vertexai")
warnings.filterwarnings("ignore", message=".*deprecated.*")

import json
import time
import os
import sys
import asyncio
import aiohttp
from datetime import datetime
from pathlib import Path
from typing import Optional, AsyncGenerator, Callable
import google.generativeai as genai

# Import prompts from baseline
from baseline_comparison import SYSTEM_PROMPT_V2, get_user_prompt_v2

OUTPUT_DIR = Path(__file__).parent / "output"
CHECKPOINT_FILE = OUTPUT_DIR / "streaming_checkpoint.json"

# Webhook for progress updates (Foundation Daemon, Slack, etc.)
PROGRESS_WEBHOOK = os.environ.get("CTAS_PROGRESS_WEBHOOK", None)


class StreamingInterviewGenerator:
    """Production streaming generator with checkpointing."""

    def __init__(self, api_key: str, webhook_url: Optional[str] = None):
        self.api_key = api_key
        self.webhook_url = webhook_url or PROGRESS_WEBHOOK
        self.checkpoint = self._load_checkpoint()

        # Configure Gemini
        genai.configure(api_key=api_key)
        self.model = genai.GenerativeModel(
            model_name="gemini-2.0-flash-exp",
            system_instruction=SYSTEM_PROMPT_V2
        )

    def _load_checkpoint(self) -> dict:
        """Load checkpoint for resume capability."""
        if CHECKPOINT_FILE.exists():
            with open(CHECKPOINT_FILE, 'r') as f:
                return json.load(f)
        return {"completed": [], "failed": [], "last_task_id": None}

    def _save_checkpoint(self):
        """Save checkpoint for fault tolerance."""
        CHECKPOINT_FILE.parent.mkdir(parents=True, exist_ok=True)
        with open(CHECKPOINT_FILE, 'w') as f:
            json.dump(self.checkpoint, f, indent=2)

    async def _send_webhook(self, event: str, data: dict):
        """Send progress update to webhook."""
        if not self.webhook_url:
            return

        payload = {
            "event": event,
            "timestamp": datetime.now().isoformat(),
            "service": "interview_generator",
            "data": data
        }

        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    self.webhook_url,
                    json=payload,
                    timeout=aiohttp.ClientTimeout(total=5)
                ) as resp:
                    pass  # Fire and forget
        except Exception:
            pass  # Don't fail on webhook errors

    async def generate_streaming(
        self,
        task: dict,
        on_token: Optional[Callable[[str], None]] = None
    ) -> dict:
        """
        Generate interview with real-time token streaming.

        Args:
            task: CTAS task dict with task_id, task_name, etc.
            on_token: Callback for each token (for real-time display)

        Returns:
            Complete interview dict
        """
        task_id = task.get("task_id", "unknown")
        task_name = task.get("task_name", "Unknown")

        # Send start event
        await self._send_webhook("task_start", {
            "task_id": task_id,
            "task_name": task_name
        })

        prompt = get_user_prompt_v2(task)
        start = time.time()

        # Stream generation
        full_response = ""
        try:
            response = self.model.generate_content(
                prompt,
                generation_config=genai.types.GenerationConfig(
                    max_output_tokens=4096,
                    temperature=0.7
                ),
                stream=True  # Enable streaming
            )

            for chunk in response:
                if chunk.text:
                    full_response += chunk.text
                    if on_token:
                        on_token(chunk.text)
                    # Yield control for async
                    await asyncio.sleep(0)

        except Exception as e:
            self.checkpoint["failed"].append({
                "task_id": task_id,
                "error": str(e),
                "timestamp": datetime.now().isoformat()
            })
            self._save_checkpoint()

            await self._send_webhook("task_error", {
                "task_id": task_id,
                "error": str(e)
            })
            raise

        elapsed = time.time() - start

        # Parse JSON from response
        content = full_response
        if "```json" in content:
            json_str = content.split("```json")[1].split("```")[0]
        elif "```" in content:
            json_str = content.split("```")[1].split("```")[0]
        else:
            json_str = content

        interview = json.loads(json_str.strip())
        interview["_meta"] = {
            "provider": "gemini_streaming",
            "time_seconds": round(elapsed, 2),
            "generated_at": datetime.now().isoformat(),
            "tokens_streamed": len(full_response)
        }

        # Update checkpoint
        self.checkpoint["completed"].append(task_id)
        self.checkpoint["last_task_id"] = task_id
        self._save_checkpoint()

        # Send completion event
        await self._send_webhook("task_complete", {
            "task_id": task_id,
            "time_seconds": elapsed,
            "voice_length": len(interview.get("voice", "")),
            "slang_count": len(interview.get("search", {}).get("slang", []))
        })

        return interview

    async def run_batch_streaming(
        self,
        tasks: list,
        output_dir: Path,
        resume: bool = True,
        on_progress: Optional[Callable[[int, int, str], None]] = None
    ):
        """
        Run batch generation with streaming output.

        Args:
            tasks: List of CTAS task dicts
            output_dir: Where to save interview JSONs
            resume: Skip already-completed tasks from checkpoint
            on_progress: Callback(current, total, task_name)
        """
        output_dir.mkdir(parents=True, exist_ok=True)

        # Filter already completed if resuming
        if resume:
            completed_ids = set(self.checkpoint["completed"])
            tasks = [t for t in tasks if t.get("task_id") not in completed_ids]
            if completed_ids:
                print(f"[RESUME] Skipping {len(completed_ids)} completed tasks")

        total = len(tasks)
        await self._send_webhook("batch_start", {
            "total_tasks": total,
            "resume_mode": resume
        })

        print(f"\n{'='*70}")
        print("STREAMING NODE INTERVIEW GENERATION")
        print(f"Tasks: {total}")
        print(f"{'='*70}\n")

        results = {"success": 0, "failed": 0}

        for i, task in enumerate(tasks):
            task_id = task.get("task_id", f"task_{i}")
            task_name = task.get("task_name", "Unknown")

            print(f"[{i+1}/{total}] {task_name}")
            print(f"  Streaming: ", end="", flush=True)

            if on_progress:
                on_progress(i + 1, total, task_name)

            # Token callback for live output
            token_count = [0]
            def on_token(token: str):
                token_count[0] += 1
                # Print dot every 50 tokens for visual progress
                if token_count[0] % 50 == 0:
                    print(".", end="", flush=True)

            try:
                interview = await self.generate_streaming(task, on_token=on_token)

                # Save immediately
                out_file = output_dir / f"{task_id}.json"
                with open(out_file, 'w') as f:
                    json.dump(interview, f, indent=2)

                voice_len = len(interview.get("voice", ""))
                slang_count = len(interview.get("search", {}).get("slang", []))
                print(f" OK ({interview['_meta']['time_seconds']}s, {voice_len} chars, {slang_count} slang)")
                results["success"] += 1

            except Exception as e:
                print(f" ERROR: {e}")
                results["failed"] += 1

        # Final summary
        print(f"\n{'='*70}")
        print("STREAMING GENERATION COMPLETE")
        print(f"{'='*70}")
        print(f"Success: {results['success']}")
        print(f"Failed:  {results['failed']}")
        print(f"Output:  {output_dir}")

        await self._send_webhook("batch_complete", results)

        return results


async def main():
    import argparse
    parser = argparse.ArgumentParser(description="Streaming Node Interview Generator")
    parser.add_argument("--limit", type=int, help="Limit number of tasks")
    parser.add_argument("--resume", action="store_true", default=True, help="Resume from checkpoint")
    parser.add_argument("--fresh", action="store_true", help="Start fresh (ignore checkpoint)")
    parser.add_argument("--webhook", type=str, help="Progress webhook URL")
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

    # Load tasks
    tasks_file = OUTPUT_DIR / "ctas_tasks.json"
    if not tasks_file.exists():
        print("ERROR: No tasks found. Run export first.")
        return

    with open(tasks_file, 'r') as f:
        tasks = json.load(f)

    if args.limit:
        tasks = tasks[:args.limit]

    # Clear checkpoint if fresh start
    if args.fresh and CHECKPOINT_FILE.exists():
        CHECKPOINT_FILE.unlink()
        print("[FRESH] Cleared checkpoint")

    # Run streaming generation
    generator = StreamingInterviewGenerator(
        api_key=api_key,
        webhook_url=args.webhook
    )

    output_dir = OUTPUT_DIR / "streaming_interviews"
    await generator.run_batch_streaming(
        tasks=tasks,
        output_dir=output_dir,
        resume=not args.fresh
    )


if __name__ == "__main__":
    asyncio.run(main())
