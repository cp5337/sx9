#!/usr/bin/env python3
"""
Refactor threat_content_fetcher.py using Vertex AI Gemini
===========================================================

Uses Vertex AI Gemini 2.0 Flash or Gemini 1.5 Pro to refactor the large
threat_content_fetcher.py file into modular components.

Requirements:
    pip install google-cloud-aiplatform vertexai

Usage:
    python refactor_with_vertex_gemini.py --project-id YOUR_PROJECT_ID --region us-central1
"""

import argparse
import json
import sys
from pathlib import Path
from typing import List, Dict, Any
import time

try:
    import vertexai
    from vertexai.generative_models import GenerativeModel, GenerationConfig
    VERTEX_AVAILABLE = True
except ImportError:
    VERTEX_AVAILABLE = False
    print("❌ Vertex AI not available. Install with: pip install google-cloud-aiplatform vertexai")

# Configuration
SOURCE_FILE = Path(__file__).parent / "threat_content_fetcher.py"
OUTPUT_DIR = Path(__file__).parent / "refactored_output"
REFACTORING_PLAN = Path(__file__).parent / "REFACTORING_PLAN.md"

# Gemini model configuration
MODEL_NAME = "gemini-2.0-flash-exp"  # Fast and cost-effective
# Alternative: "gemini-1.5-pro" for better quality (slower, more expensive)

# Chunk size for processing (Gemini has token limits)
MAX_CHUNK_LINES = 500  # Process ~500 lines at a time


def detect_project_id() -> str:
    """Auto-detect GCP project ID from existing codebase patterns."""
    # Check common project IDs used in codebase
    common_projects = [
        "gen-lang-client-0290627006",  # From two_stage_generator.py
        "ctas-7",  # From generate_node_interviews.py
        "cognetix-alpha",  # From gcp_vertex_gemini_pipeline.py
    ]
    
    # Try to get from gcloud config
    import subprocess
    try:
        result = subprocess.run(
            ["gcloud", "config", "get-value", "project"],
            capture_output=True,
            text=True,
            timeout=2
        )
        if result.returncode == 0 and result.stdout.strip():
            return result.stdout.strip()
    except:
        pass
    
    # Try environment variable
    import os
    if os.environ.get("GOOGLE_CLOUD_PROJECT"):
        return os.environ.get("GOOGLE_CLOUD_PROJECT")
    
    # Default to first common project
    return common_projects[0]


class VertexRefactoringEngine:
    """Refactor code using Vertex AI Gemini (single call, not sequential)."""
    
    def __init__(self, project_id: str = None, region: str = "us-central1", model_name: str = MODEL_NAME):
        if not VERTEX_AVAILABLE:
            raise ImportError("Vertex AI not available. Install: pip install google-cloud-aiplatform vertexai")
        
        # Auto-detect project if not provided
        self.project_id = project_id or detect_project_id()
        self.region = region
        self.model_name = model_name
        
        # Initialize Vertex AI (uses gcloud auth, no API keys needed)
        # Follows pattern from two_stage_generator.py and generate_node_interviews.py
        vertexai.init(project=self.project_id, location=region)
        self.model = GenerativeModel(model_name)
        self.generation_config = GenerationConfig(
            max_output_tokens=8192,
            temperature=0.1  # Low temperature for code generation
        )
        
        print(f"✅ Initialized Vertex AI: {model_name}")
        print(f"   Project: {self.project_id} (auto-detected)")
        print(f"   Region: {region}")
        print(f"   Auth: gcloud application-default credentials")
    
    def read_file_chunks(self, file_path: Path, chunk_size: int = MAX_CHUNK_LINES) -> List[Dict[str, Any]]:
        """Split file into manageable chunks for processing."""
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        chunks = []
        for i in range(0, len(lines), chunk_size):
            chunk_lines = lines[i:i+chunk_size]
            chunk_text = ''.join(chunk_lines)
            
            chunks.append({
                "chunk_id": i // chunk_size,
                "start_line": i + 1,
                "end_line": min(i + chunk_size, len(lines)),
                "content": chunk_text,
                "line_count": len(chunk_lines)
            })
        
        return chunks
    
    def generate_refactoring_prompt(self, chunk: Dict[str, Any], refactoring_plan: str) -> str:
        """Generate prompt for Gemini to refactor a chunk."""
        return f"""You are a senior Python engineer refactoring a large monolithic script into modular components.

REFACTORING PLAN:
{refactoring_plan}

CURRENT CHUNK (lines {chunk['start_line']}-{chunk['end_line']}):
```python
{chunk['content']}
```

TASK:
1. Analyze this chunk and identify which fetcher/processor module it belongs to
2. Extract methods/classes that should be moved to:
   - fetchers/mitre_fetcher.py (MITRE ATT&CK, D3FEND, CAR, ATLAS)
   - fetchers/detection_fetcher.py (Sigma, YARA, Wazuh, Nuclei)
   - fetchers/emulation_fetcher.py (Atomic Red Team, Caldera)
   - fetchers/osint_fetcher.py (OSINT tools)
   - fetchers/kali_fetcher.py (Kali tools)
   - fetchers/lolbas_fetcher.py (LOLBAS, GTFOBins, etc.)
   - processors/spires_processor.py (SPIRES ontology)
   - processors/dsl_processor.py (YAML to DSL)
   - processors/ml_training_processor.py (ML training)
   - utils/git_utils.py (Git operations)
   - utils/http_utils.py (HTTP downloads)
   - utils/index_utils.py (Index generation)

3. Refactor the code to:
   - Create a base class if needed (BaseFetcher)
   - Extract methods into appropriate modules
   - Maintain backward compatibility (same API)
   - Add proper imports and type hints
   - Follow PEP 8 style

4. Output JSON with:
   {{
     "module": "fetchers/mitre_fetcher.py",
     "classes": [{{"name": "MitreFetcher", "code": "..."}}],
     "functions": [{{"name": "fetch_mitre_attack", "code": "..."}}],
     "imports": ["from pathlib import Path", ...],
     "dependencies": ["requests", "yaml"],
     "notes": "Extracted MITRE ATT&CK fetching logic"
   }}

Be precise and maintain all functionality. Do not remove or break existing code.
"""
    
    def refactor_chunk(self, chunk: Dict[str, Any], refactoring_plan: str) -> Dict[str, Any]:
        """Refactor a single chunk using Gemini."""
        print(f"\n🔄 Processing chunk {chunk['chunk_id']} (lines {chunk['start_line']}-{chunk['end_line']})...")
        
        prompt = self.generate_refactoring_prompt(chunk, refactoring_plan)
        
        try:
            # Single Vertex AI call (not sequential Vertex→Gemini)
            # Follows pattern from two_stage_generator.py
            response = self.model.generate_content(
                prompt,
                generation_config=self.generation_config
            )
            
            # Extract JSON from response
            response_text = response.text.strip()
            
            # Try to extract JSON if wrapped in markdown
            if "```json" in response_text:
                json_start = response_text.find("```json") + 7
                json_end = response_text.find("```", json_start)
                response_text = response_text[json_start:json_end].strip()
            elif "```" in response_text:
                json_start = response_text.find("```") + 3
                json_end = response_text.find("```", json_start)
                response_text = response_text[json_start:json_end].strip()
            
            result = json.loads(response_text)
            result["chunk_id"] = chunk["chunk_id"]
            result["original_lines"] = f"{chunk['start_line']}-{chunk['end_line']}"
            
            print(f"   ✅ Extracted: {result.get('module', 'unknown')}")
            print(f"   📦 Classes: {len(result.get('classes', []))}")
            print(f"   🔧 Functions: {len(result.get('functions', []))}")
            
            return result
            
        except Exception as e:
            print(f"   ❌ Error processing chunk: {e}")
            return {
                "chunk_id": chunk["chunk_id"],
                "error": str(e),
                "module": "unknown"
            }
    
    def consolidate_results(self, results: List[Dict[str, Any]]) -> Dict[str, List[Dict[str, Any]]]:
        """Group refactored code by target module."""
        consolidated = {}
        
        for result in results:
            if "error" in result:
                continue
            
            module = result.get("module", "unknown")
            if module not in consolidated:
                consolidated[module] = []
            consolidated[module].append(result)
        
        return consolidated
    
    def generate_module_code(self, module_name: str, results: List[Dict[str, Any]]) -> str:
        """Generate complete module code from refactored chunks."""
        prompt = f"""You are consolidating refactored code chunks into a complete Python module.

MODULE: {module_name}

REFACTORED CHUNKS:
{json.dumps(results, indent=2)}

TASK:
1. Merge all classes and functions into a single, coherent module
2. Organize imports at the top
3. Ensure no duplicate code
4. Maintain proper class hierarchy (BaseFetcher if applicable)
5. Add module docstring
6. Ensure all dependencies are in imports
7. Follow PEP 8 style

Output the complete Python module code, ready to use.
"""
        
        try:
            # Single Vertex AI call for module consolidation
            large_config = GenerationConfig(
                max_output_tokens=16384,  # Larger for full modules
                temperature=0.1
            )
            response = self.model.generate_content(
                prompt,
                generation_config=large_config
            )
            
            code = response.text.strip()
            
            # Extract code if wrapped in markdown
            if "```python" in code:
                code_start = code.find("```python") + 9
                code_end = code.find("```", code_start)
                code = code[code_start:code_end].strip()
            elif "```" in code:
                code_start = code.find("```") + 3
                code_end = code.find("```", code_start)
                code = code[code_start:code_end].strip()
            
            return code
            
        except Exception as e:
            print(f"   ❌ Error generating module code: {e}")
            return f"# Error generating {module_name}: {e}\n"
    
    def refactor_file(self, source_file: Path, output_dir: Path, refactoring_plan: Path):
        """Refactor entire file using Gemini."""
        print(f"\n🚀 Starting refactoring of {source_file.name}")
        print(f"   Source: {source_file}")
        print(f"   Output: {output_dir}")
        
        # Read refactoring plan
        if refactoring_plan.exists():
            with open(refactoring_plan, 'r') as f:
                plan_text = f.read()
        else:
            plan_text = "Refactor into modular components: fetchers/, processors/, utils/"
        
        # Create output directory
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # Split file into chunks
        chunks = self.read_file_chunks(source_file)
        print(f"\n📊 File split into {len(chunks)} chunks")
        
        # Process each chunk
        results = []
        for i, chunk in enumerate(chunks):
            result = self.refactor_chunk(chunk, plan_text)
            results.append(result)
            
            # Rate limiting (Gemini has quotas)
            if i < len(chunks) - 1:
                time.sleep(1)  # 1 second between requests
        
        # Consolidate by module
        print(f"\n📦 Consolidating {len(results)} results...")
        consolidated = self.consolidate_results(results)
        
        print(f"\n✅ Generated {len(consolidated)} modules:")
        for module_name in consolidated:
            print(f"   • {module_name} ({len(consolidated[module_name])} chunks)")
        
        # Generate complete modules
        print(f"\n🔨 Generating complete module files...")
        for module_name, module_results in consolidated.items():
            print(f"   Generating {module_name}...")
            module_code = self.generate_module_code(module_name, module_results)
            
            # Write module file
            module_path = output_dir / module_name
            module_path.parent.mkdir(parents=True, exist_ok=True)
            
            with open(module_path, 'w') as f:
                f.write(module_code)
            
            print(f"   ✅ Written: {module_path}")
        
        # Generate main orchestrator
        print(f"\n🎯 Generating main orchestrator...")
        orchestrator_prompt = f"""Generate a main orchestrator class that uses all the refactored modules.

REFACTORED MODULES:
{json.dumps(list(consolidated.keys()), indent=2)}

TASK:
Create a ThreatContentFetcher class that:
1. Imports and uses all the refactored fetcher modules
2. Maintains the same API as the original (backward compatible)
3. Orchestrates all fetchers
4. Handles SPIRES, DSL, and ML training processors

Output complete Python code for the main orchestrator.
"""
        
        try:
            response = self.model.generate_content(orchestrator_prompt)
            orchestrator_code = response.text.strip()
            
            if "```python" in orchestrator_code:
                orchestrator_code = orchestrator_code.split("```python")[1].split("```")[0].strip()
            
            orchestrator_path = output_dir / "threat_content_fetcher.py"
            with open(orchestrator_path, 'w') as f:
                f.write(orchestrator_code)
            
            print(f"   ✅ Written: {orchestrator_path}")
            
        except Exception as e:
            print(f"   ❌ Error generating orchestrator: {e}")
        
        print(f"\n🎉 Refactoring complete!")
        print(f"   Output directory: {output_dir}")
        print(f"   Review and test the refactored code before deploying.")


def main():
    parser = argparse.ArgumentParser(description="Refactor threat_content_fetcher.py using Vertex AI Gemini")
    parser.add_argument("--project-id", default=None, help="GCP Project ID (auto-detected if not provided)")
    parser.add_argument("--region", default="us-central1", help="GCP Region (default: us-central1)")
    parser.add_argument("--model", default=MODEL_NAME, help=f"Gemini model (default: {MODEL_NAME})")
    parser.add_argument("--source", default=str(SOURCE_FILE), help="Source file to refactor")
    parser.add_argument("--output", default=str(OUTPUT_DIR), help="Output directory")
    
    args = parser.parse_args()
    
    if not VERTEX_AVAILABLE:
        print("❌ Vertex AI not available. Install with: pip install google-cloud-aiplatform vertexai")
        sys.exit(1)
    
    if not Path(args.source).exists():
        print(f"❌ Source file not found: {args.source}")
        sys.exit(1)
    
    # Initialize refactoring engine (auto-detects project)
    engine = VertexRefactoringEngine(
        project_id=args.project_id,
        region=args.region,
        model_name=args.model
    )
    
    # Run refactoring
    engine.refactor_file(
        source_file=Path(args.source),
        output_dir=Path(args.output),
        refactoring_plan=REFACTORING_PLAN
    )


if __name__ == "__main__":
    main()

