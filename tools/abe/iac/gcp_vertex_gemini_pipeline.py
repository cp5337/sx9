#!/usr/bin/env python3
"""
CTAS-7 GCP Vertex AI + Gemini Pipeline
=======================================

Dual AI Pipeline running on Google Cloud:
1. Vertex AI (Gemini 1.5 Flash) - Fast initial extraction
2. Gemini 1.5 Pro - Deep refinement, scholarly refs, test harness

GPU-accelerated embeddings via Vertex AI Embeddings API.

Usage:
    python gcp_vertex_gemini_pipeline.py --all
    python gcp_vertex_gemini_pipeline.py --extract-rfcs
    python gcp_vertex_gemini_pipeline.py --scholarly-refs
    python gcp_vertex_gemini_pipeline.py --test-harness
    python gcp_vertex_gemini_pipeline.py --embeddings
"""

import os
import sys
import json
import asyncio
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
from datetime import datetime, timezone
import re

# Google AI imports
try:
    import google.generativeai as genai
    HAS_GENAI = True
except ImportError:
    HAS_GENAI = False
    print("⚠️  google-generativeai not installed. Run: pip install google-generativeai")

# ============================================================================
# CONFIGURATION
# ============================================================================

@dataclass
class GCPConfig:
    """GCP Pipeline configuration."""
    project_id: str = "cognetix-alpha"
    region: str = "us-central1"
    api_key: str = ""
    
    # Models
    flash_model: str = "gemini-1.5-flash"  # Fast extraction
    pro_model: str = "gemini-1.5-pro"      # Deep refinement
    embedding_model: str = "models/text-embedding-004"
    
    # Paths
    rfc_path: str = "/Users/cp5337/Developer/ctas-7-shipyard-staging/01-rfc"
    output_path: str = "/Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac/output/gcp_extraction"
    
    # Limits
    max_rfcs: int = 50
    max_tokens: int = 8192

# ============================================================================
# VERTEX AI / GEMINI CLIENT
# ============================================================================

class DualAIClient:
    """Client for dual Vertex AI + Gemini pipeline."""
    
    def __init__(self, config: GCPConfig):
        self.config = config
        self.flash_model = None
        self.pro_model = None
        
        if HAS_GENAI and config.api_key:
            genai.configure(api_key=config.api_key)
            self.flash_model = genai.GenerativeModel(config.flash_model)
            self.pro_model = genai.GenerativeModel(config.pro_model)
            print(f"✅ Gemini models initialized")
            print(f"   Flash: {config.flash_model}")
            print(f"   Pro: {config.pro_model}")
        else:
            print("⚠️  Gemini not configured - check API key")
    
    async def extract_with_flash(self, content: str, context: str = "") -> Dict:
        """
        Stage 1: Fast extraction with Gemini Flash.
        Used for initial entity extraction, summarization.
        """
        if not self.flash_model:
            return {"error": "Flash model not configured"}
        
        prompt = f"""Extract key information from this document:

Context: {context}

Document:
{content[:6000]}

Extract and return as JSON:
{{
    "title": "document title",
    "summary": "2-3 sentence summary",
    "key_concepts": ["list", "of", "concepts"],
    "entities": {{
        "algorithms": [],
        "protocols": [],
        "systems": [],
        "standards": []
    }},
    "ip_claims": ["potential IP claims"],
    "dependencies": ["referenced documents/RFCs"]
}}
"""
        
        try:
            response = await asyncio.to_thread(
                self.flash_model.generate_content, prompt
            )
            # Try to parse JSON from response
            text = response.text
            # Find JSON in response
            json_match = re.search(r'\{[\s\S]*\}', text)
            if json_match:
                return json.loads(json_match.group())
            return {"raw_response": text}
        except Exception as e:
            return {"error": str(e)}
    
    async def refine_with_pro(self, extraction: Dict, original_content: str) -> Dict:
        """
        Stage 2: Deep refinement with Gemini Pro.
        Improves extraction, adds scholarly refs, generates test ideas.
        """
        if not self.pro_model:
            return {"error": "Pro model not configured"}
        
        prompt = f"""You are an expert technical reviewer. Improve this extraction:

Initial Extraction:
{json.dumps(extraction, indent=2)}

Original Document (first 4000 chars):
{original_content[:4000]}

Please:
1. Verify and correct any errors in the extraction
2. Add missing key concepts or entities
3. Improve the summary for clarity
4. Identify specific IP claims that could be patentable
5. Suggest 3-5 scholarly references that should be cited
6. Propose 3-5 test cases to validate the claims

Return as JSON:
{{
    "refined_extraction": {{ ... improved extraction ... }},
    "corrections": ["list of corrections made"],
    "scholarly_refs": [
        {{"title": "...", "authors": "...", "year": "...", "relevance": "..."}}
    ],
    "test_cases": [
        {{"name": "...", "description": "...", "expected_result": "..."}}
    ],
    "ip_assessment": {{
        "novelty_score": 0-100,
        "patentability": "high/medium/low",
        "prior_art_concerns": ["..."]
    }}
}}
"""
        
        try:
            response = await asyncio.to_thread(
                self.pro_model.generate_content, prompt
            )
            text = response.text
            json_match = re.search(r'\{[\s\S]*\}', text)
            if json_match:
                return json.loads(json_match.group())
            return {"raw_response": text}
        except Exception as e:
            return {"error": str(e)}
    
    async def generate_scholarly_refs(self, rfc_content: str, rfc_id: str) -> Dict:
        """Generate scholarly references for an RFC."""
        if not self.pro_model:
            return {"error": "Pro model not configured"}
        
        prompt = f"""For RFC {rfc_id}, identify relevant scholarly references:

RFC Content (first 5000 chars):
{rfc_content[:5000]}

Provide 5-10 academic papers that should be cited. For each:
1. Full citation (authors, title, venue, year)
2. DOI or URL if known
3. Why it's relevant to this RFC
4. Importance (1-5, where 5 is essential)

Focus on:
- Foundational algorithms mentioned
- Similar systems in literature
- Theoretical foundations
- Industry standards

Return as JSON:
{{
    "rfc_id": "{rfc_id}",
    "references": [
        {{
            "citation": "Authors. Title. Venue, Year.",
            "doi": "10.xxxx/xxxxx",
            "relevance": "Why this is relevant",
            "importance": 5
        }}
    ],
    "missing_foundations": ["Areas that need more scholarly grounding"]
}}
"""
        
        try:
            response = await asyncio.to_thread(
                self.pro_model.generate_content, prompt
            )
            text = response.text
            json_match = re.search(r'\{[\s\S]*\}', text)
            if json_match:
                return json.loads(json_match.group())
            return {"raw_response": text}
        except Exception as e:
            return {"error": str(e)}
    
    async def generate_test_harness(self, rfc_content: str, rfc_id: str) -> Dict:
        """Generate test harness for an RFC."""
        if not self.pro_model:
            return {"error": "Pro model not configured"}
        
        prompt = f"""Generate a Rust test harness for RFC {rfc_id}:

RFC Content (first 5000 chars):
{rfc_content[:5000]}

Create comprehensive tests that prove the RFC's claims:

1. Unit tests for each major function/algorithm
2. Property-based tests where applicable
3. Integration tests for system interactions
4. Performance benchmarks for latency claims
5. Edge case tests

Return as JSON:
{{
    "rfc_id": "{rfc_id}",
    "test_module_name": "test_{rfc_id.lower().replace('-', '_')}",
    "rust_code": "// Complete Rust test module code here",
    "test_descriptions": [
        {{"name": "test_xxx", "proves": "What claim this test proves"}}
    ],
    "coverage_notes": "What aspects are tested and what needs manual verification"
}}
"""
        
        try:
            response = await asyncio.to_thread(
                self.pro_model.generate_content, prompt
            )
            text = response.text
            json_match = re.search(r'\{[\s\S]*\}', text)
            if json_match:
                return json.loads(json_match.group())
            return {"raw_response": text, "rfc_id": rfc_id}
        except Exception as e:
            return {"error": str(e), "rfc_id": rfc_id}
    
    async def generate_embedding(self, text: str) -> Optional[List[float]]:
        """Generate embedding using Vertex AI."""
        if not HAS_GENAI:
            return None
        
        try:
            result = genai.embed_content(
                model=self.config.embedding_model,
                content=text[:2000],  # Limit text length
                task_type="retrieval_document"
            )
            return result['embedding']
        except Exception as e:
            print(f"Embedding error: {e}")
            return None

# ============================================================================
# PIPELINE ORCHESTRATOR
# ============================================================================

class GCPExtractionPipeline:
    """Main pipeline orchestrator."""
    
    def __init__(self, config: GCPConfig):
        self.config = config
        self.client = DualAIClient(config)
        self.output_dir = Path(config.output_path)
        self.output_dir.mkdir(parents=True, exist_ok=True)
    
    def find_rfcs(self) -> List[Path]:
        """Find all RFC files."""
        rfc_path = Path(self.config.rfc_path)
        if not rfc_path.exists():
            print(f"❌ RFC path not found: {rfc_path}")
            return []
        return list(rfc_path.rglob("RFC-*.md"))[:self.config.max_rfcs]
    
    async def process_rfc(self, rfc_path: Path) -> Dict:
        """Process a single RFC through the dual AI pipeline."""
        content = rfc_path.read_text(encoding='utf-8', errors='ignore')
        rfc_id = rfc_path.stem
        
        print(f"\n📄 Processing {rfc_id}...")
        
        # Stage 1: Flash extraction
        print(f"   ⚡ Stage 1: Flash extraction...")
        extraction = await self.client.extract_with_flash(content, f"RFC: {rfc_id}")
        
        # Stage 2: Pro refinement
        print(f"   🧠 Stage 2: Pro refinement...")
        refinement = await self.client.refine_with_pro(extraction, content)
        
        return {
            "rfc_id": rfc_id,
            "rfc_path": str(rfc_path),
            "stage1_extraction": extraction,
            "stage2_refinement": refinement,
            "processed_at": datetime.now(timezone.utc).isoformat()
        }
    
    async def run_extraction(self) -> List[Dict]:
        """Run extraction on all RFCs."""
        rfcs = self.find_rfcs()
        print(f"Found {len(rfcs)} RFCs to process")
        
        results = []
        for rfc_path in rfcs:
            try:
                result = await self.process_rfc(rfc_path)
                results.append(result)
                
                # Save individual result
                output_file = self.output_dir / f"{rfc_path.stem}_extraction.json"
                with open(output_file, 'w') as f:
                    json.dump(result, f, indent=2)
                    
            except Exception as e:
                print(f"   ❌ Error: {e}")
                results.append({"rfc_id": rfc_path.stem, "error": str(e)})
        
        # Save combined results
        combined_file = self.output_dir / "all_extractions.json"
        with open(combined_file, 'w') as f:
            json.dump(results, f, indent=2)
        
        return results
    
    async def run_scholarly_refs(self) -> List[Dict]:
        """Generate scholarly refs for all RFCs."""
        rfcs = self.find_rfcs()
        print(f"Generating scholarly refs for {len(rfcs)} RFCs")
        
        results = []
        for rfc_path in rfcs:
            content = rfc_path.read_text(encoding='utf-8', errors='ignore')
            rfc_id = rfc_path.stem
            
            print(f"\n📚 {rfc_id}: Generating scholarly refs...")
            refs = await self.client.generate_scholarly_refs(content, rfc_id)
            results.append(refs)
            
            # Save individual result
            output_file = self.output_dir / f"{rfc_id}_scholarly_refs.json"
            with open(output_file, 'w') as f:
                json.dump(refs, f, indent=2)
        
        # Save combined
        combined_file = self.output_dir / "all_scholarly_refs.json"
        with open(combined_file, 'w') as f:
            json.dump(results, f, indent=2)
        
        return results
    
    async def run_test_harness(self) -> List[Dict]:
        """Generate test harnesses for all RFCs."""
        rfcs = self.find_rfcs()
        print(f"Generating test harnesses for {len(rfcs)} RFCs")
        
        results = []
        for rfc_path in rfcs:
            content = rfc_path.read_text(encoding='utf-8', errors='ignore')
            rfc_id = rfc_path.stem
            
            print(f"\n🧪 {rfc_id}: Generating test harness...")
            harness = await self.client.generate_test_harness(content, rfc_id)
            results.append(harness)
            
            # Save individual result
            output_file = self.output_dir / f"{rfc_id}_test_harness.json"
            with open(output_file, 'w') as f:
                json.dump(harness, f, indent=2)
            
            # Also save Rust code separately if present
            if "rust_code" in harness and harness["rust_code"]:
                rust_file = self.output_dir / f"{rfc_id}_tests.rs"
                with open(rust_file, 'w') as f:
                    f.write(harness["rust_code"])
        
        return results
    
    async def run_embeddings(self, extractions: List[Dict] = None) -> List[Dict]:
        """Generate embeddings for all extractions."""
        if not extractions:
            # Load from file
            combined_file = self.output_dir / "all_extractions.json"
            if combined_file.exists():
                with open(combined_file) as f:
                    extractions = json.load(f)
            else:
                extractions = await self.run_extraction()
        
        print(f"Generating embeddings for {len(extractions)} documents")
        
        for ext in extractions:
            if "stage1_extraction" in ext:
                summary = ext["stage1_extraction"].get("summary", "")
                if summary:
                    print(f"   🧮 Embedding {ext['rfc_id']}...")
                    embedding = await self.client.generate_embedding(summary)
                    ext["embedding"] = embedding
                    ext["embedding_dim"] = len(embedding) if embedding else 0
        
        # Save with embeddings
        output_file = self.output_dir / "extractions_with_embeddings.json"
        with open(output_file, 'w') as f:
            json.dump(extractions, f, indent=2)
        
        return extractions

# ============================================================================
# MAIN
# ============================================================================

async def main():
    parser = argparse.ArgumentParser(description="CTAS-7 GCP Vertex AI + Gemini Pipeline")
    parser.add_argument("--project", default="cognetix-alpha", help="GCP Project ID")
    parser.add_argument("--region", default="us-central1", help="GCP Region")
    parser.add_argument("--api-key", help="Gemini API Key")
    parser.add_argument("--output", default="output/gcp_extraction", help="Output directory")
    parser.add_argument("--max-rfcs", type=int, default=10, help="Max RFCs to process")
    
    parser.add_argument("--all", action="store_true", help="Run all stages")
    parser.add_argument("--extract-rfcs", action="store_true", help="Extract RFCs")
    parser.add_argument("--scholarly-refs", action="store_true", help="Generate scholarly refs")
    parser.add_argument("--test-harness", action="store_true", help="Generate test harnesses")
    parser.add_argument("--embeddings", action="store_true", help="Generate embeddings")
    
    args = parser.parse_args()
    
    # Get API key
    api_key = args.api_key or os.environ.get("GOOGLE_API_KEY") or os.environ.get("GEMINI_API_KEY")
    
    if not api_key:
        print("❌ No API key provided!")
        print("   Set GOOGLE_API_KEY environment variable or use --api-key")
        sys.exit(1)
    
    config = GCPConfig(
        project_id=args.project,
        region=args.region,
        api_key=api_key,
        output_path=args.output,
        max_rfcs=args.max_rfcs
    )
    
    print("=" * 70)
    print("🚀 CTAS-7 GCP VERTEX AI + GEMINI PIPELINE")
    print("=" * 70)
    print(f"Project: {config.project_id}")
    print(f"Region: {config.region}")
    print(f"Max RFCs: {config.max_rfcs}")
    print(f"Output: {config.output_path}")
    print("=" * 70)
    
    pipeline = GCPExtractionPipeline(config)
    
    results = {}
    
    if args.all or args.extract_rfcs:
        print("\n📄 STAGE: RFC Extraction (Flash + Pro)")
        print("-" * 50)
        results["extractions"] = await pipeline.run_extraction()
        print(f"✅ Extracted {len(results['extractions'])} RFCs")
    
    if args.all or args.scholarly_refs:
        print("\n📚 STAGE: Scholarly References")
        print("-" * 50)
        results["scholarly_refs"] = await pipeline.run_scholarly_refs()
        print(f"✅ Generated refs for {len(results['scholarly_refs'])} RFCs")
    
    if args.all or args.test_harness:
        print("\n🧪 STAGE: Test Harness Generation")
        print("-" * 50)
        results["test_harnesses"] = await pipeline.run_test_harness()
        print(f"✅ Generated harnesses for {len(results['test_harnesses'])} RFCs")
    
    if args.all or args.embeddings:
        print("\n🧮 STAGE: Embeddings")
        print("-" * 50)
        extractions = results.get("extractions")
        results["with_embeddings"] = await pipeline.run_embeddings(extractions)
        print(f"✅ Generated embeddings for {len(results['with_embeddings'])} documents")
    
    print("\n" + "=" * 70)
    print("📊 GCP PIPELINE COMPLETE")
    print("=" * 70)
    print(f"Output directory: {config.output_path}")
    
    output_dir = Path(config.output_path)
    if output_dir.exists():
        print("\nGenerated files:")
        for f in sorted(output_dir.glob("*")):
            size = f.stat().st_size
            print(f"  - {f.name} ({size:,} bytes)")

if __name__ == "__main__":
    asyncio.run(main())

