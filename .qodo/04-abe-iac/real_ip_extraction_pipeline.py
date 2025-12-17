#!/usr/bin/env python3
"""
CTAS-7 REAL IP Extraction Pipeline
===================================

NO FAKE CODE. NO HARDCODED OUTPUTS. REAL API CALLS.

This pipeline:
1. Extracts IP from markdown files using Gemini API
2. Generates scholarly references for RFCs
3. Creates test harnesses
4. Generates LaTeX research papers
5. Creates EA artifacts (diagrams, Cypher, etc.)

Cost: See COST_ESTIMATE_AND_PLAN.md

Usage:
    # Dry run (estimate only, no API calls)
    python real_ip_extraction_pipeline.py --estimate
    
    # Minimal run (~$1)
    python real_ip_extraction_pipeline.py --rfcs-only --model flash
    
    # Standard run (~$8.50)
    python real_ip_extraction_pipeline.py --hybrid
    
    # Full run (~$25)
    python real_ip_extraction_pipeline.py --all --model pro
"""

import os
import sys
import json
import asyncio
import argparse
from pathlib import Path
from datetime import datetime, timezone
from typing import List, Dict, Optional, Any
from dataclasses import dataclass
import re

# ============================================================================
# ABE KEY VAULT INTEGRATION (Federated)
# ============================================================================

# Vault priority order (per ABE_API_VAULT_INTEGRATION.md):
# 1. ABE sx9-api-vault (~/.sx9-api-vault/keys.json)
# 2. CTAS7 API Vault (CTAS7_API_VAULT.json)
# 3. Environment variables

ABE_VAULT_PATH = Path.home() / ".sx9-api-vault" / "keys.json"
CTAS7_VAULT_PATH = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging/CTAS7_API_VAULT.json")

def load_api_key_from_vault(service: str = "google_gemini") -> Optional[str]:
    """Load API key from ABE Federated Vault System.
    
    Follows ABE_API_VAULT_INTEGRATION.md federation strategy:
    - Tier 1: Local ABE vault (~/.sx9-api-vault/)
    - Tier 2: CTAS7 API Vault
    - Tier 3: Environment variables
    """
    
    # Tier 1: ABE sx9-api-vault
    if ABE_VAULT_PATH.exists():
        try:
            with open(ABE_VAULT_PATH) as f:
                vault = json.load(f)
            
            keys = vault.get("keys", {})
            if service in keys:
                key = keys[service].get("key_value")
                if key and key not in ["NEEDS_REPLACEMENT", "NEEDS_SETUP"]:
                    print(f"🔐 Loaded {service} from ABE Vault (Tier 1)")
                    return key
        except Exception as e:
            print(f"⚠️  ABE Vault load error: {e}")
    
    # Tier 2: CTAS7 API Vault
    if CTAS7_VAULT_PATH.exists():
        try:
            with open(CTAS7_VAULT_PATH) as f:
                vault = json.load(f)
            
            api_keys = vault.get("api_keys", {})
            service_config = api_keys.get(service, {})
            
            # Get the key
            key = service_config.get("api_key") or service_config.get("access_token")
            
            if key and key not in ["NEEDS_REPLACEMENT", "NEEDS_SETUP", "CONFIGURE_IN_APP"]:
                print(f"🔐 Loaded {service} from CTAS7 Vault (Tier 2)")
                return key
            else:
                print(f"⚠️  {service} key in CTAS7 vault needs replacement")
        except Exception as e:
            print(f"⚠️  CTAS7 Vault load error: {e}")
    
    # Tier 3: Environment variables
    env_key = os.environ.get("GOOGLE_API_KEY") or os.environ.get("GEMINI_API_KEY")
    if env_key:
        print("🔐 Using API key from environment (Tier 3)")
        return env_key
    
    return None

def list_vault_status():
    """Show status of all vault tiers."""
    print("\n🔐 ABE Vault Federation Status:")
    print("-" * 50)
    
    # Tier 1
    if ABE_VAULT_PATH.exists():
        try:
            with open(ABE_VAULT_PATH) as f:
                vault = json.load(f)
            keys = vault.get("keys", {})
            print(f"   Tier 1 (ABE): ✅ {len(keys)} keys")
        except:
            print(f"   Tier 1 (ABE): ⚠️  Error reading")
    else:
        print(f"   Tier 1 (ABE): ❌ Not configured")
    
    # Tier 2
    if CTAS7_VAULT_PATH.exists():
        try:
            with open(CTAS7_VAULT_PATH) as f:
                vault = json.load(f)
            api_keys = vault.get("api_keys", {})
            active = sum(1 for k, v in api_keys.items() 
                        if v.get("status") == "active")
            print(f"   Tier 2 (CTAS7): ✅ {active} active keys")
        except:
            print(f"   Tier 2 (CTAS7): ⚠️  Error reading")
    else:
        print(f"   Tier 2 (CTAS7): ❌ Not found")
    
    # Tier 3
    if os.environ.get("GOOGLE_API_KEY") or os.environ.get("GEMINI_API_KEY"):
        print(f"   Tier 3 (Env): ✅ GOOGLE_API_KEY set")
    else:
        print(f"   Tier 3 (Env): ❌ No API key in environment")
    
    print("-" * 50)

# ============================================================================
# DEPENDENCY CHECK
# ============================================================================

def check_dependencies():
    """Check all required dependencies are available."""
    missing = []
    
    try:
        import google.generativeai
    except ImportError:
        missing.append("google-generativeai")
    
    if missing:
        print("❌ Missing dependencies:")
        for dep in missing:
            print(f"   pip install {dep}")
        return False
    
    return True

# ============================================================================
# CONFIGURATION
# ============================================================================

@dataclass
class PipelineConfig:
    api_key: str
    model: str  # "flash" or "pro"
    output_dir: Path
    rfc_dir: Path
    md_dirs: List[Path]
    max_files: int
    dry_run: bool

def get_model_name(model: str) -> str:
    """Get full model name."""
    if model == "flash":
        return "gemini-2.0-flash"
    elif model == "pro":
        return "gemini-2.5-pro"
    else:
        return model

def estimate_tokens(text: str) -> int:
    """Estimate token count (rough: 4 chars per token)."""
    return len(text) // 4

def estimate_cost(input_tokens: int, output_tokens: int, model: str) -> float:
    """Estimate API cost."""
    if model == "flash" or "flash" in model:
        # Flash: $0.075/1M input, $0.30/1M output
        return (input_tokens * 0.075 / 1_000_000) + (output_tokens * 0.30 / 1_000_000)
    else:
        # Pro: $1.25/1M input, $5.00/1M output
        return (input_tokens * 1.25 / 1_000_000) + (output_tokens * 5.00 / 1_000_000)

# ============================================================================
# GEMINI CLIENT
# ============================================================================

class GeminiClient:
    """Real Gemini API client."""
    
    def __init__(self, api_key: str, model: str = "flash"):
        import google.generativeai as genai
        
        self.api_key = api_key
        self.model_name = get_model_name(model)
        
        genai.configure(api_key=api_key)
        self.model = genai.GenerativeModel(self.model_name)
        self.genai = genai
        
        # Track usage
        self.total_input_tokens = 0
        self.total_output_tokens = 0
        self.total_calls = 0
    
    async def generate(self, prompt: str, max_output_tokens: int = 2048) -> str:
        """Make a real API call to Gemini."""
        try:
            # Estimate input tokens
            input_tokens = estimate_tokens(prompt)
            self.total_input_tokens += input_tokens
            
            # Make the actual API call
            response = await asyncio.to_thread(
                self.model.generate_content,
                prompt,
                generation_config={"max_output_tokens": max_output_tokens}
            )
            
            # Get response text
            result = response.text
            
            # Track output tokens
            output_tokens = estimate_tokens(result)
            self.total_output_tokens += output_tokens
            self.total_calls += 1
            
            return result
            
        except Exception as e:
            print(f"❌ API Error: {e}")
            raise
    
    def get_usage_summary(self) -> Dict:
        """Get usage statistics."""
        cost = estimate_cost(self.total_input_tokens, self.total_output_tokens, self.model_name)
        return {
            "total_calls": self.total_calls,
            "input_tokens": self.total_input_tokens,
            "output_tokens": self.total_output_tokens,
            "estimated_cost": f"${cost:.4f}"
        }

# ============================================================================
# EXTRACTION FUNCTIONS
# ============================================================================

async def extract_ip_from_document(client: GeminiClient, content: str, filename: str) -> Dict:
    """Extract IP from a single document."""
    
    prompt = f"""Analyze this technical document and extract intellectual property:

Document: {filename}

Content (first 6000 chars):
{content[:6000]}

Return ONLY valid JSON (no markdown, no explanation):
{{
    "title": "extracted title",
    "summary": "2-3 sentence summary",
    "key_concepts": ["concept1", "concept2"],
    "algorithms": ["any algorithms mentioned"],
    "novel_claims": ["potential patentable claims"],
    "technical_terms": ["domain-specific terms"],
    "dependencies": ["referenced documents"]
}}"""

    response = await client.generate(prompt, max_output_tokens=1024)
    
    # Parse JSON from response
    try:
        # Find JSON in response
        json_match = re.search(r'\{[\s\S]*\}', response)
        if json_match:
            return json.loads(json_match.group())
    except json.JSONDecodeError:
        pass
    
    return {"raw_response": response, "parse_error": True}

async def generate_scholarly_refs(client: GeminiClient, content: str, rfc_id: str) -> Dict:
    """Generate scholarly references for an RFC."""
    
    prompt = f"""You are an academic researcher. For RFC {rfc_id}, provide scholarly references.

RFC Content (first 5000 chars):
{content[:5000]}

Return ONLY valid JSON with real, verifiable academic papers:
{{
    "rfc_id": "{rfc_id}",
    "references": [
        {{
            "authors": "Author names",
            "title": "Paper title",
            "venue": "Conference/Journal name",
            "year": 2020,
            "doi": "10.xxxx/xxxxx if known",
            "relevance": "Why this paper is relevant"
        }}
    ],
    "bibtex": "@article{{...}}"
}}

Focus on:
- Hash functions and content addressing
- Graph algorithms and knowledge graphs
- Cognitive computing and neural architectures
- Compression algorithms
- Real-time systems"""

    response = await client.generate(prompt, max_output_tokens=2048)
    
    try:
        json_match = re.search(r'\{[\s\S]*\}', response)
        if json_match:
            return json.loads(json_match.group())
    except json.JSONDecodeError:
        pass
    
    return {"rfc_id": rfc_id, "raw_response": response}

async def generate_test_harness(client: GeminiClient, content: str, rfc_id: str) -> Dict:
    """Generate Rust test harness for an RFC."""
    
    prompt = f"""Generate a Rust test module for RFC {rfc_id}.

RFC Content (first 4000 chars):
{content[:4000]}

Return ONLY valid JSON:
{{
    "rfc_id": "{rfc_id}",
    "module_name": "test_{rfc_id.lower().replace('-', '_')}",
    "rust_tests": "// Rust test code here",
    "test_cases": [
        {{"name": "test_name", "description": "what it tests"}}
    ]
}}

Include:
- Unit tests for core functions
- Property-based tests
- Edge cases
- Performance assertions"""

    response = await client.generate(prompt, max_output_tokens=2048)
    
    try:
        json_match = re.search(r'\{[\s\S]*\}', response)
        if json_match:
            return json.loads(json_match.group())
    except json.JSONDecodeError:
        pass
    
    return {"rfc_id": rfc_id, "raw_response": response}

async def generate_latex_paper(client: GeminiClient, rfc_content: str, rfc_id: str, refs: Dict) -> str:
    """Generate LaTeX research paper for an RFC."""
    
    refs_text = json.dumps(refs.get("references", []), indent=2) if refs else "[]"
    
    prompt = f"""Generate a complete IEEE-format LaTeX research paper for RFC {rfc_id}.

RFC Content:
{rfc_content[:8000]}

Available References:
{refs_text}

Generate a complete LaTeX document with:
- \\documentclass{{IEEEtran}}
- Title, authors, abstract
- Introduction
- Background/Related Work (use provided references)
- System Architecture
- Implementation
- Evaluation
- Conclusion
- Bibliography

Return ONLY the LaTeX code, starting with \\documentclass."""

    response = await client.generate(prompt, max_output_tokens=4096)
    return response

async def generate_mermaid_diagram(client: GeminiClient, content: str, diagram_type: str) -> str:
    """Generate Mermaid diagram."""
    
    prompt = f"""Generate a Mermaid diagram ({diagram_type}) for this system:

Content:
{content[:3000]}

Return ONLY the Mermaid code, starting with the diagram type (flowchart, sequenceDiagram, etc.)."""

    response = await client.generate(prompt, max_output_tokens=1024)
    return response

# ============================================================================
# PIPELINE
# ============================================================================

class RealIPPipeline:
    """Real IP extraction pipeline."""
    
    def __init__(self, config: PipelineConfig):
        self.config = config
        self.client = None
        self.results = {
            "extractions": [],
            "scholarly_refs": [],
            "test_harnesses": [],
            "latex_papers": [],
            "diagrams": []
        }
    
    def initialize_client(self):
        """Initialize Gemini client."""
        if self.config.dry_run:
            print("🔍 DRY RUN - No API calls will be made")
            return
        
        if not self.config.api_key:
            raise ValueError("API key required. Set GOOGLE_API_KEY environment variable.")
        
        self.client = GeminiClient(self.config.api_key, self.config.model)
        print(f"✅ Gemini client initialized: {self.client.model_name}")
    
    def find_files(self, pattern: str, dirs: List[Path], max_files: int = None) -> List[Path]:
        """Find files matching pattern."""
        files = []
        for d in dirs:
            if d.exists():
                for f in d.rglob(pattern):
                    # Skip node_modules, target, and check file exists
                    if "node_modules" in str(f) or "/target/" in str(f):
                        continue
                    try:
                        if f.exists() and f.is_file():
                            files.append(f)
                    except:
                        pass
        
        # Sort by size (smaller first for testing)
        try:
            files.sort(key=lambda f: f.stat().st_size if f.exists() else 0)
        except:
            pass
        
        if max_files:
            files = files[:max_files]
        
        return files
    
    def estimate_run(self) -> Dict:
        """Estimate costs without running."""
        rfc_files = self.find_files("RFC-*.md", [self.config.rfc_dir])
        md_files = self.find_files("*.md", self.config.md_dirs, self.config.max_files)
        
        # Calculate total content size
        rfc_size = sum(f.stat().st_size for f in rfc_files)
        md_size = sum(f.stat().st_size for f in md_files)
        
        # Estimate tokens
        rfc_tokens = rfc_size // 4
        md_tokens = md_size // 4
        
        # Estimate output tokens (roughly 20% of input)
        rfc_output = rfc_tokens // 5
        md_output = md_tokens // 5
        
        # Calculate costs
        model = self.config.model
        rfc_cost = estimate_cost(rfc_tokens, rfc_output, model)
        md_cost = estimate_cost(md_tokens, md_output, model)
        
        return {
            "rfc_files": len(rfc_files),
            "rfc_size_mb": rfc_size / 1_000_000,
            "rfc_tokens": rfc_tokens,
            "rfc_cost": f"${rfc_cost:.2f}",
            "md_files": len(md_files),
            "md_size_mb": md_size / 1_000_000,
            "md_tokens": md_tokens,
            "md_cost": f"${md_cost:.2f}",
            "total_cost": f"${rfc_cost + md_cost:.2f}",
            "model": get_model_name(model)
        }
    
    async def run_rfc_extraction(self) -> List[Dict]:
        """Extract IP from RFCs."""
        rfc_files = self.find_files("RFC-*.md", [self.config.rfc_dir])
        print(f"\n📄 Processing {len(rfc_files)} RFCs...")
        
        results = []
        for i, rfc_file in enumerate(rfc_files):
            rfc_id = rfc_file.stem
            print(f"   [{i+1}/{len(rfc_files)}] {rfc_id}...")
            
            content = rfc_file.read_text(encoding='utf-8', errors='ignore')
            
            if self.config.dry_run:
                results.append({"rfc_id": rfc_id, "dry_run": True})
                continue
            
            extraction = await extract_ip_from_document(self.client, content, rfc_id)
            extraction["rfc_id"] = rfc_id
            extraction["file_path"] = str(rfc_file)
            results.append(extraction)
            
            # Save individual result
            output_file = self.config.output_dir / f"{rfc_id}_extraction.json"
            with open(output_file, 'w') as f:
                json.dump(extraction, f, indent=2)
        
        self.results["extractions"] = results
        return results
    
    async def run_scholarly_refs(self) -> List[Dict]:
        """Generate scholarly references."""
        rfc_files = self.find_files("RFC-*.md", [self.config.rfc_dir])
        print(f"\n📚 Generating scholarly refs for {len(rfc_files)} RFCs...")
        
        results = []
        for i, rfc_file in enumerate(rfc_files):
            rfc_id = rfc_file.stem
            print(f"   [{i+1}/{len(rfc_files)}] {rfc_id}...")
            
            content = rfc_file.read_text(encoding='utf-8', errors='ignore')
            
            if self.config.dry_run:
                results.append({"rfc_id": rfc_id, "dry_run": True})
                continue
            
            refs = await generate_scholarly_refs(self.client, content, rfc_id)
            results.append(refs)
            
            # Save individual result
            output_file = self.config.output_dir / f"{rfc_id}_scholarly_refs.json"
            with open(output_file, 'w') as f:
                json.dump(refs, f, indent=2)
        
        self.results["scholarly_refs"] = results
        return results
    
    async def run_test_harnesses(self) -> List[Dict]:
        """Generate test harnesses."""
        rfc_files = self.find_files("RFC-*.md", [self.config.rfc_dir])
        print(f"\n🧪 Generating test harnesses for {len(rfc_files)} RFCs...")
        
        results = []
        for i, rfc_file in enumerate(rfc_files):
            rfc_id = rfc_file.stem
            print(f"   [{i+1}/{len(rfc_files)}] {rfc_id}...")
            
            content = rfc_file.read_text(encoding='utf-8', errors='ignore')
            
            if self.config.dry_run:
                results.append({"rfc_id": rfc_id, "dry_run": True})
                continue
            
            harness = await generate_test_harness(self.client, content, rfc_id)
            results.append(harness)
            
            # Save JSON
            output_file = self.config.output_dir / f"{rfc_id}_test_harness.json"
            with open(output_file, 'w') as f:
                json.dump(harness, f, indent=2)
            
            # Save Rust code if present
            if "rust_tests" in harness:
                rust_file = self.config.output_dir / f"{rfc_id}_tests.rs"
                with open(rust_file, 'w') as f:
                    f.write(harness["rust_tests"])
        
        self.results["test_harnesses"] = results
        return results
    
    async def run_latex_papers(self, rfc_ids: List[str] = None) -> List[str]:
        """Generate LaTeX papers for selected RFCs."""
        if not rfc_ids:
            # Default: key RFCs
            rfc_ids = ["RFC-9001", "RFC-9026", "RFC-9100", "RFC-9016", "RFC-9021"]
        
        print(f"\n📝 Generating LaTeX papers for: {', '.join(rfc_ids)}")
        
        results = []
        for rfc_id in rfc_ids:
            rfc_file = self.config.rfc_dir / f"{rfc_id}.md"
            if not rfc_file.exists():
                # Try subdirectories
                matches = list(self.config.rfc_dir.rglob(f"{rfc_id}*.md"))
                if matches:
                    rfc_file = matches[0]
                else:
                    print(f"   ⚠️ {rfc_id} not found")
                    continue
            
            print(f"   📄 {rfc_id}...")
            
            content = rfc_file.read_text(encoding='utf-8', errors='ignore')
            
            if self.config.dry_run:
                results.append(f"% DRY RUN: {rfc_id}")
                continue
            
            # Get refs if available
            refs_file = self.config.output_dir / f"{rfc_id}_scholarly_refs.json"
            refs = {}
            if refs_file.exists():
                with open(refs_file) as f:
                    refs = json.load(f)
            
            latex = await generate_latex_paper(self.client, content, rfc_id, refs)
            results.append(latex)
            
            # Save LaTeX file
            output_file = self.config.output_dir / f"{rfc_id}_paper.tex"
            with open(output_file, 'w') as f:
                f.write(latex)
            print(f"   ✅ Saved: {output_file.name}")
        
        self.results["latex_papers"] = results
        return results
    
    async def run_diagrams(self) -> List[Dict]:
        """Generate architecture diagrams."""
        print(f"\n📊 Generating architecture diagrams...")
        
        # Key system descriptions for diagrams
        systems = [
            ("Hourglass-Bernoulli Architecture", "flowchart", "RFC-9026"),
            ("Dual Trivariate Hash Flow", "sequenceDiagram", "RFC-9001"),
            ("PTCC Primitive Classification", "flowchart", "RFC-9100"),
        ]
        
        results = []
        for name, diagram_type, rfc_id in systems:
            print(f"   📈 {name}...")
            
            rfc_file = list(self.config.rfc_dir.rglob(f"{rfc_id}*.md"))
            if not rfc_file:
                continue
            
            content = rfc_file[0].read_text(encoding='utf-8', errors='ignore')
            
            if self.config.dry_run:
                results.append({"name": name, "dry_run": True})
                continue
            
            diagram = await generate_mermaid_diagram(self.client, content, diagram_type)
            
            result = {
                "name": name,
                "type": diagram_type,
                "rfc": rfc_id,
                "mermaid": diagram
            }
            results.append(result)
            
            # Save diagram
            safe_name = name.lower().replace(" ", "_").replace("-", "_")
            output_file = self.config.output_dir / f"{safe_name}.mmd"
            with open(output_file, 'w') as f:
                f.write(diagram)
        
        self.results["diagrams"] = results
        return results
    
    def save_summary(self):
        """Save pipeline summary."""
        summary = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "config": {
                "model": self.config.model,
                "dry_run": self.config.dry_run,
                "output_dir": str(self.config.output_dir)
            },
            "results": {
                "extractions": len(self.results["extractions"]),
                "scholarly_refs": len(self.results["scholarly_refs"]),
                "test_harnesses": len(self.results["test_harnesses"]),
                "latex_papers": len(self.results["latex_papers"]),
                "diagrams": len(self.results["diagrams"])
            }
        }
        
        if self.client and not self.config.dry_run:
            summary["usage"] = self.client.get_usage_summary()
        
        output_file = self.config.output_dir / "pipeline_summary.json"
        with open(output_file, 'w') as f:
            json.dump(summary, f, indent=2)
        
        return summary

# ============================================================================
# MAIN
# ============================================================================

async def main():
    parser = argparse.ArgumentParser(description="CTAS-7 Real IP Extraction Pipeline")
    
    # Run modes
    parser.add_argument("--estimate", action="store_true", help="Estimate costs only (no API calls)")
    parser.add_argument("--dry-run", action="store_true", help="Dry run (no API calls)")
    parser.add_argument("--rfcs-only", action="store_true", help="Process RFCs only")
    parser.add_argument("--hybrid", action="store_true", help="Hybrid run (extraction + refs + tests)")
    parser.add_argument("--all", action="store_true", help="Full run including LaTeX papers")
    
    # Options
    parser.add_argument("--model", choices=["flash", "pro"], default="flash", help="Gemini model")
    parser.add_argument("--max-files", type=int, default=100, help="Max files to process")
    parser.add_argument("--output", default="output/real_extraction", help="Output directory")
    parser.add_argument("--latex-rfcs", nargs="+", help="Specific RFCs for LaTeX papers")
    
    args = parser.parse_args()
    
    # Check dependencies
    if not check_dependencies():
        sys.exit(1)
    
    # Get API key from ABE Vault Federation
    api_key = load_api_key_from_vault("google_gemini")
    
    # Setup config
    base_path = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging")
    config = PipelineConfig(
        api_key=api_key or "",
        model=args.model,
        output_dir=base_path / "04-abe-iac" / args.output,
        rfc_dir=base_path / "01-rfc",
        md_dirs=[base_path, Path("/Users/cp5337/Developer/sx9-development-center")],
        max_files=args.max_files,
        dry_run=args.dry_run or args.estimate
    )
    
    # Create output directory
    config.output_dir.mkdir(parents=True, exist_ok=True)
    
    print("=" * 70)
    print("🚀 CTAS-7 REAL IP EXTRACTION PIPELINE")
    print("=" * 70)
    print(f"Model: {get_model_name(config.model)}")
    print(f"Output: {config.output_dir}")
    print(f"Mode: {'DRY RUN' if config.dry_run else 'LIVE'}")
    
    # Show vault status
    list_vault_status()
    
    print("=" * 70)
    
    pipeline = RealIPPipeline(config)
    
    # Estimate mode
    if args.estimate:
        print("\n📊 COST ESTIMATE")
        print("-" * 50)
        estimate = pipeline.estimate_run()
        for k, v in estimate.items():
            print(f"   {k}: {v}")
        print("\n⚠️  To run for real, remove --estimate and set GOOGLE_API_KEY")
        return
    
    # Check API key for live runs
    if not config.dry_run and not api_key:
        print("\n❌ API key required for live runs!")
        print("   export GOOGLE_API_KEY='your-key-here'")
        print("   Or use --dry-run for testing")
        sys.exit(1)
    
    # Initialize client
    pipeline.initialize_client()
    
    # Run pipeline stages
    if args.rfcs_only or args.hybrid or args.all:
        await pipeline.run_rfc_extraction()
    
    if args.hybrid or args.all:
        await pipeline.run_scholarly_refs()
        await pipeline.run_test_harnesses()
    
    if args.all:
        rfc_ids = args.latex_rfcs or ["RFC-9001", "RFC-9026", "RFC-9100"]
        await pipeline.run_latex_papers(rfc_ids)
        await pipeline.run_diagrams()
    
    # Save summary
    summary = pipeline.save_summary()
    
    print("\n" + "=" * 70)
    print("📊 PIPELINE COMPLETE")
    print("=" * 70)
    print(f"Output: {config.output_dir}")
    
    if "usage" in summary:
        print(f"\n💰 API Usage:")
        for k, v in summary["usage"].items():
            print(f"   {k}: {v}")
    
    print(f"\n📁 Generated files:")
    for f in sorted(config.output_dir.glob("*")):
        print(f"   - {f.name}")

if __name__ == "__main__":
    asyncio.run(main())

