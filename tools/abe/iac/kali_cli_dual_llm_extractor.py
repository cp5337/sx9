#!/usr/bin/env python3
"""
Kali CLI Extractor using Needle + Dual LLM (Vertex Gemini + OpenAI)
Scrapes Kali.org and GitLab, extracts CLI commands using both models for comparison
"""

import json
import re
import time
import os
from pathlib import Path
from typing import Dict, Any, List
import requests
from bs4 import BeautifulSoup

# Try importing Needle/Crawl4AI
try:
    from crawl4ai import WebCrawler
    from crawl4ai.extraction_strategy import LLMExtractionStrategy
    HAS_CRAWL4AI = True
except ImportError:
    HAS_CRAWL4AI = False
    print("⚠️  crawl4ai not installed. Using requests fallback.")

# Try importing Google Gemini API (regular)
try:
    import google.generativeai as genai
    HAS_GEMINI = True
except ImportError:
    HAS_GEMINI = False
    print("⚠️  google-generativeai not installed. Skipping Gemini API.")

# Try importing Google Vertex AI
try:
    import vertexai
    from vertexai.generative_models import GenerativeModel
    HAS_VERTEX = True
except ImportError:
    HAS_VERTEX = False
    print("⚠️  vertexai not installed. Skipping Vertex AI.")

# Try importing OpenAI
try:
    from openai import OpenAI
    HAS_OPENAI = True
except ImportError:
    HAS_OPENAI = False
    print("⚠️  openai not installed. Skipping OpenAI.")

# Configuration
TOOLS_FILE = Path("/Users/cp5337/Developer/sx9/.qodo/04-abe-iac/node-interview-generator/output/kali_tools/kali_tools_complete.json")
OUTPUT_FILE = Path("/Users/cp5337/Developer/sx9/.qodo/04-abe-iac/node-interview-generator/output/kali_tools/kali_tools_with_commands.json")

# LLM Extraction Prompt
EXTRACTION_PROMPT = """
Extract the primary CLI command and common arguments for this Kali Linux tool.

Tool name: {slug}
Description: {description}
URL: {url}

Return ONLY valid JSON (no markdown, no code blocks):
{{
    "command": "primary_executable_name",
    "executables": ["all", "binary", "names"],
    "default_args": ["common", "flags"],
    "usage_example": "command -flag target",
    "hd4_phase": "Hunt|Detect|Disrupt|Disable|Dominate",
    "ptcc_primitive": 1
}}

Rules:
- command: single primary executable (e.g., "nmap")
- executables: all binaries this package provides
- default_args: common flags (e.g., ["-v", "-A"])
- usage_example: full command example
- hd4_phase: one of Hunt, Detect, Disrupt, Disable, Dominate
- ptcc_primitive: number 1-32 (1=READ, 8=TRANSFORM, 15=DECRYPT, 22=CALL)
"""

class KaliCLIExtractor:
    def __init__(self):
        self.stats = {
            'tier1_regex': 0,
            'tier2_gitlab': 0,
            'tier3_gemini': 0,
            'tier3_vertex': 0,
            'tier3_openai': 0,
            'fallback': 0
        }
        
        # Initialize regular Gemini API
        if HAS_GEMINI:
            gemini_key = os.environ.get('GEMINI_API_KEY', 'AIzaSyA9wvdjofLJjzLpnEgfsoSyKgU0OhSnCeM')
            genai.configure(api_key=gemini_key)
            self.gemini_model = genai.GenerativeModel('gemini-2.0-flash-exp')
        
        # Initialize Vertex AI
        if HAS_VERTEX:
            vertexai.init(project="gen-lang-client-0290627006", location="us-central1")
            self.vertex_model = GenerativeModel("gemini-2.0-flash-exp")
        
        # Initialize OpenAI (skip - quota exceeded)
        self.openai_client = None
    
    def extract_tier1_regex(self, tool_url: str, slug: str) -> Dict[str, Any]:
        """Tier 1: Extract from Kali.org page using regex"""
        try:
            response = requests.get(tool_url, timeout=10)
            response.raise_for_status()
            soup = BeautifulSoup(response.content, 'html.parser')
            
            # Look for command examples
            text = soup.get_text()
            pattern = r'root@kali:~#\s+(\S+)\s+([^\n]*)'
            matches = re.findall(pattern, text)
            
            if matches:
                command = matches[0][0]
                args = matches[0][1].strip()
                self.stats['tier1_regex'] += 1
                return {
                    "command": command,
                    "default_args": args.split()[:5] if args else [],
                    "usage_example": f"{command} {args}",
                    "source": "tier1_regex",
                    "confidence": 0.9
                }
        except Exception as e:
            pass
        
        return None
    
    def extract_tier2_gitlab(self, git_repo: str, slug: str) -> Dict[str, Any]:
        """Tier 2: Extract from GitLab debian/control"""
        try:
            control_url = git_repo.replace('.git', '') + '/-/raw/kali/master/debian/control'
            response = requests.get(control_url, timeout=10)
            response.raise_for_status()
            
            content = response.text
            
            # Parse debian/control
            package_match = re.search(r'Package:\s+(\S+)', content)
            binary_match = re.search(r'Binary:\s+([^\n]+)', content)
            
            if package_match:
                command = package_match.group(1)
                binaries = []
                if binary_match:
                    binaries = [b.strip() for b in binary_match.group(1).split(',')]
                
                self.stats['tier2_gitlab'] += 1
                return {
                    "command": command,
                    "executables": binaries or [command],
                    "source": "tier2_gitlab",
                    "confidence": 0.95
                }
        except Exception as e:
            pass
        
        return None
    
    def extract_tier3_gemini(self, slug: str, description: str, url: str) -> Dict[str, Any]:
        """Tier 3a: Extract using regular Gemini API (first layer)"""
        if not HAS_GEMINI:
            return None
        
        try:
            prompt = EXTRACTION_PROMPT.format(
                slug=slug,
                description=description[:500],
                url=url
            )
            
            response = self.gemini_model.generate_content(prompt)
            result_text = response.text.strip()
            
            # Remove markdown code blocks if present
            result_text = re.sub(r'```json\n?', '', result_text)
            result_text = re.sub(r'```\n?', '', result_text)
            
            result = json.loads(result_text)
            result['source'] = 'tier3_gemini'
            result['confidence'] = 0.75
            
            self.stats['tier3_gemini'] += 1
            return result
            
        except Exception as e:
            print(f"  ⚠️  Gemini API error for {slug}: {e}")
            return None
    
    def extract_tier3_vertex(self, slug: str, description: str, url: str) -> Dict[str, Any]:
        """Tier 3: Extract using Vertex AI Gemini"""
        if not HAS_VERTEX:
            return None
        
        try:
            prompt = EXTRACTION_PROMPT.format(
                slug=slug,
                description=description[:500],
                url=url
            )
            
            response = self.vertex_model.generate_content(prompt)
            result_text = response.text.strip()
            
            # Remove markdown code blocks if present
            result_text = re.sub(r'```json\n?', '', result_text)
            result_text = re.sub(r'```\n?', '', result_text)
            
            result = json.loads(result_text)
            result['source'] = 'tier3_vertex'
            result['confidence'] = 0.7
            
            self.stats['tier3_vertex'] += 1
            return result
            
        except Exception as e:
            print(f"  ⚠️  Vertex AI error for {slug}: {e}")
            return None
    
    def extract_tier3_openai(self, slug: str, description: str, url: str) -> Dict[str, Any]:
        """Tier 3: Extract using OpenAI GPT-4o-mini"""
        if not HAS_OPENAI:
            return None
        
        try:
            prompt = EXTRACTION_PROMPT.format(
                slug=slug,
                description=description[:500],
                url=url
            )
            
            response = self.openai_client.chat.completions.create(
                model="gpt-4o-mini",
                messages=[
                    {"role": "system", "content": "You are a CLI command extraction expert. Return only valid JSON."},
                    {"role": "user", "content": prompt}
                ],
                temperature=0.1
            )
            
            result_text = response.choices[0].message.content.strip()
            
            # Remove markdown code blocks if present
            result_text = re.sub(r'```json\n?', '', result_text)
            result_text = re.sub(r'```\n?', '', result_text)
            
            result = json.loads(result_text)
            result['source'] = 'tier3_openai'
            result['confidence'] = 0.7
            
            self.stats['tier3_openai'] += 1
            return result
            
        except Exception as e:
            print(f"  ⚠️  OpenAI error for {slug}: {e}")
            return None
    
    def extract_fallback(self, slug: str) -> Dict[str, Any]:
        """Fallback: Use slug as command"""
        self.stats['fallback'] += 1
        return {
            "command": slug,
            "executables": [slug],
            "default_args": [],
            "source": "fallback",
            "confidence": 0.3
        }
    
    def extract_tool(self, slug: str, tool_data: Dict) -> Dict[str, Any]:
        """Extract CLI command using multi-tier strategy"""
        print(f"  Processing: {slug}")
        
        # Tier 1: Regex extraction from Kali.org
        result = self.extract_tier1_regex(tool_data['url'], slug)
        if result and result['confidence'] >= 0.9:
            print(f"    ✅ Tier 1 (regex): {result['command']}")
            return result
        
        # Tier 2: GitLab debian/control
        if tool_data.get('git_repo'):
            result = self.extract_tier2_gitlab(tool_data['git_repo'], slug)
            if result and result['confidence'] >= 0.9:
                print(f"    ✅ Tier 2 (gitlab): {result['command']}")
                return result
        
        # Tier 3: LLM extraction (layered Gemini approach)
        description = tool_data.get('description', '')
        
        # Layer 1: Try regular Gemini API first
        gemini_result = self.extract_tier3_gemini(slug, description, tool_data['url'])
        if gemini_result:
            print(f"    ✅ Tier 3a (gemini): {gemini_result['command']}")
            
            # Layer 2: Verify with Vertex AI Gemini
            time.sleep(0.5)  # Rate limiting
            vertex_result = self.extract_tier3_vertex(slug, description, tool_data['url'])
            
            if vertex_result:
                # Compare results from both Gemini APIs
                if gemini_result['command'] == vertex_result['command']:
                    print(f"    ✓ Verified by Vertex AI (match!)")
                    gemini_result['verified'] = True
                    gemini_result['confidence'] = 0.95
                    gemini_result['vertex_confirmed'] = True
                else:
                    print(f"    ⚠️  Vertex AI disagrees: {vertex_result['command']}")
                    gemini_result['vertex_alternative'] = vertex_result['command']
            
            return gemini_result
        
        # Fallback to Vertex AI if regular Gemini failed
        vertex_result = self.extract_tier3_vertex(slug, description, tool_data['url'])
        if vertex_result:
            print(f"    ✅ Tier 3b (vertex): {vertex_result['command']}")
            return vertex_result
        
        # Fallback
        result = self.extract_fallback(slug)
        print(f"    ⚠️  Fallback: {result['command']}")
        return result
    
    def run(self):
        """Run extraction on all tools"""
        print("═" * 60)
        print("Kali CLI Extractor - Dual LLM (Vertex + OpenAI)")
        print("═" * 60)
        
        # Load existing tools
        with open(TOOLS_FILE, 'r') as f:
            data = json.load(f)
        
        tools = data['tools']
        enriched_tools = {}
        
        print(f"\n📥 Loaded {len(tools)} tools")
        print(f"🤖 LLMs available: Gemini={HAS_GEMINI}, Vertex={HAS_VERTEX}\n")
        
        # Process each tool
        for i, (slug, tool_data) in enumerate(tools.items(), 1):
            print(f"[{i}/{len(tools)}]", end=" ")
            
            # Extract CLI command
            cli_data = self.extract_tool(slug, tool_data)
            
            # Merge with existing data
            enriched_tools[slug] = {
                **tool_data,
                **cli_data
            }
            
            # Rate limiting
            if i % 10 == 0:
                print(f"\n  Progress: {i}/{len(tools)} tools processed\n")
                time.sleep(1)
        
        # Save enriched data
        output_data = {
            **data,
            "tools": enriched_tools,
            "enrichment_complete": True,
            "stats": self.stats
        }
        
        OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
        with open(OUTPUT_FILE, 'w') as f:
            json.dump(output_data, f, indent=2)
        
        # Print stats
        print("\n" + "═" * 60)
        print("✅ Extraction Complete!")
        print("═" * 60)
        print(f"Tier 1 (regex):    {self.stats['tier1_regex']} tools")
        print(f"Tier 2 (gitlab):   {self.stats['tier2_gitlab']} tools")
        print(f"Tier 3a (gemini):  {self.stats['tier3_gemini']} tools")
        print(f"Tier 3b (vertex):  {self.stats['tier3_vertex']} tools")
        print(f"Fallback:          {self.stats['fallback']} tools")
        print(f"\n📁 Saved to: {OUTPUT_FILE}")
        print("═" * 60)

if __name__ == '__main__':
    extractor = KaliCLIExtractor()
    extractor.run()
