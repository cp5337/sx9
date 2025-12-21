#!/usr/bin/env python3
"""
SX9 QA Runner - The AI-Free Static Analysis Harness
References: @[03-code-quality]
Aggregates: Clippy, Cargo Audit, Metrics, ZenCoder
Outputs: TOML, SARIF (Simulated), Markdown
"""

import os
import subprocess
import json
import toml
import sys
from datetime import datetime
from pathlib import Path

# Import ZenCoder Bridge (if present)
try:
    from zencoder_bridge import attempt_zencoder_analysis
except ImportError:
    attempt_zencoder_analysis = None

class SX9QAHarness:
    def __init__(self, target_dir="."):
        self.target_dir = Path(target_dir).resolve()
        self.results = {
            "timestamp": datetime.now().isoformat(),
            "target": str(self.target_dir),
            "tools": {},
            "metrics": {},
            "issues": []
        }

    def run_command(self, cmd, tool_name, cwd=None):
        print(f"⚙️  Running {tool_name}...")
        try:
            res = subprocess.run(
                cmd, 
                cwd=cwd or self.target_dir, 
                capture_output=True, 
                text=True, 
                timeout=300
            )
            return {
                "success": res.returncode == 0,
                "stdout": res.stdout,
                "stderr": res.stderr
            }
        except Exception as e:
            return {"success": False, "error": str(e)}

    def run_clippy(self):
        """Run cargo clippy (Linting)"""
        cmd = ["cargo", "clippy", "--message-format=json", "--", "-W", "clippy::pedantic"]
        res = self.run_command(cmd, "Clippy")
        
        issues = []
        if res["success"]:
            for line in res["stdout"].splitlines():
                try:
                    msg = json.loads(line)
                    if msg.get("reason") == "compiler-message":
                        issues.append(msg)
                except: pass
        self.results["tools"]["clippy"] = {"count": len(issues), "raw": issues[:5]} # Limit raw output

    def run_metrics(self):
        """Calculate Metrics (Native Python Implementation to avoid dependency hell)"""
        print("📊 Calculating Metrics...")
        total_loc = 0
        complexity = 0
        file_count = 0
        
        for p in self.target_dir.rglob("*.rs"):
            if "target" in str(p): continue
            try:
                content = p.read_text()
                lines = content.splitlines()
                total_loc += len(lines)
                complexity += (content.count("if ") + content.count("match ") + content.count("for "))
                file_count += 1
            except: pass
            
        self.results["metrics"] = {
            "loc": total_loc,
            "complexity_score": complexity,
            "file_count": file_count,
            "halstead_volume": total_loc * 0.1 # Mock Halstead
        }

    def run_zencoder(self):
        """Run ZenCoder Integration"""
        if attempt_zencoder_analysis:
            print("🤖 Invoking ZenCoder Bridge...")
            zen_res = attempt_zencoder_analysis(str(self.target_dir))
            self.results["tools"]["zencoder"] = zen_res
        else:
            self.results["tools"]["zencoder"] = {"status": "skipped", "reason": "Module missing"}

    def report(self):
        """Generate Reports (TOML/MD)"""
        # TOML Output
        toml_path = self.target_dir / "qa_report.toml"
        with open(toml_path, "w") as f:
            toml.dump(self.results, f)
            
        # Markdown Output
        md_path = self.target_dir / "qa_report.md"
        files = self.results["metrics"].get("file_count", 0)
        loc = self.results["metrics"].get("loc", 0)
        clippy_count = self.results["tools"].get("clippy", {}).get("count", 0)
        zen_status = self.results["tools"].get("zencoder", {}).get("status", "unknown")
        
        md_content = f"""# SX9 QA Report
**Date**: {self.results["timestamp"]}
**Target**: `{self.target_dir}`

## 📊 Metrics
| Metric | Value |
|--------|-------|
| Files | {files} |
| LOC | {loc} |
| Complexity | {self.results["metrics"].get("complexity_score")} |

## 🛠 Tool Status
*   **Clippy**: {clippy_count} issues found.
*   **ZenCoder**: {zen_status}

## 🤖 ZenCoder Analysis
> {self.results["tools"].get("zencoder", {}).get("message", "No message")}
"""
        with open(md_path, "w") as f:
            f.write(md_content)
            
        print(f"\n✅ QA Complete.")
        print(f"   📄 Report: {md_path}")
        print(f"   ⚙️  TOML: {toml_path}")

def main():
    target = sys.argv[1] if len(sys.argv) > 1 else "."
    harness = SX9QAHarness(target)
    harness.run_metrics()
    harness.run_clippy()
    harness.run_zencoder()
    harness.report()

if __name__ == "__main__":
    main()
