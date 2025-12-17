#!/usr/bin/env python3
"""
SX9 IaC Executor - Unicode to Infrastructure
=============================================

Usage:
    # Single tool
    python3 iac_executor.py --unicode E800 --target 192.168.1.0/24
    
    # Tool chain
    python3 iac_executor.py --chain E800,E810,E820 --target example.com
    
    # Custom tool
    python3 iac_executor.py --unicode E900 --target custom-target
    
    # Scheduled sync
    python3 iac_executor.py --sync --sources mitre,kali,sigma
"""

import asyncio
import json
import os
import subprocess
import tempfile
from typing import Dict, List, Optional
from dataclasses import dataclass
from pathlib import Path

# ============================================================================
# CONFIGURATION
# ============================================================================

CONFIG = {
    'cloudflare_worker_url': os.getenv('CF_WORKER_URL'),  # Optional - falls back to Supabase
    'supabase_url': os.getenv('SUPABASE_URL'),
    'supabase_key': os.getenv('SUPABASE_KEY'),
    'neon_url': os.getenv('NEON_DATABASE_URL'),
    'neo4j_uri': os.getenv('NEO4J_URI'),
    'terraform_bin': os.getenv('TERRAFORM_BIN', 'terraform'),
    'docker_bin': os.getenv('DOCKER_BIN', 'docker'),
    'output_dir': os.getenv('SX9_OUTPUT_DIR', '/var/sx9/runs'),
    'cleanup_after': int(os.getenv('SX9_CLEANUP_SECONDS', '300'))  # 5 minutes
}

# ============================================================================
# DATA MODELS
# ============================================================================

@dataclass
class ToolManifest:
    """Tool configuration pulled from databases"""
    unicode: str
    name: str
    category: str
    binary_path: Optional[str]
    docker_image: Optional[str]
    capabilities: List[str]
    dependencies: List[str]
    mitre_techniques: List[str]
    terraform_module: Optional[str]
    command_template: str
    network_mode: str = "bridge"

# ============================================================================
# DATA PULLER (CloudFlare KV First)
# ============================================================================

class DataPuller:
    """Pulls tool manifests from optimal source"""
    
    def __init__(self):
        self.cache = {}
    
    async def pull(self, unicode: str) -> Optional[ToolManifest]:
        """
        Pull tool manifest using waterfall pattern
        """
        # Check local cache first
        if unicode in self.cache:
            print(f"✅ Pulled {unicode} from local cache (0ms)")
            return self.cache[unicode]
        
        # Try CloudFlare KV (edge cache)
        manifest = await self._pull_from_cloudflare(unicode)
        if manifest:
            self.cache[unicode] = manifest
            return manifest
        
        # Try Supabase (primary storage)
        manifest = await self._pull_from_supabase(unicode)
        if manifest:
            self.cache[unicode] = manifest
            return manifest
        
        print(f"❌ Tool {unicode} not found")
        return None
    
    async def _pull_from_cloudflare(self, unicode: str) -> Optional[ToolManifest]:
        """Pull from CloudFlare Worker"""
        if not CONFIG.get('cloudflare_worker_url'):
            # CloudFlare not configured - skip silently
            return None
        
        try:
            import aiohttp
            url = f"{CONFIG['cloudflare_worker_url']}/api/tool/{unicode}"
            
            async with aiohttp.ClientSession() as session:
                async with session.get(url, timeout=aiohttp.ClientTimeout(total=5)) as response:
                    if response.status == 200:
                        data = await response.json()
                        cache_status = response.headers.get('X-Cache', 'MISS')
                        latency_ms = 3 if cache_status == 'HIT' else 50
                        
                        print(f"✅ Pulled {unicode} from CloudFlare (X-Cache: {cache_status}, ~{latency_ms}ms)")
                        
                        return ToolManifest(
                            unicode=unicode,
                            name=data.get('name', ''),
                            category=data.get('category', ''),
                            binary_path=data.get('binary_path'),
                            docker_image=data.get('docker_image'),
                            capabilities=data.get('capabilities', []),
                            dependencies=data.get('dependencies', []),
                            mitre_techniques=data.get('mitre_techniques', []),
                            terraform_module=data.get('terraform_module'),
                            command_template=data.get('command_template', '{binary} {target}'),
                            network_mode=data.get('network_mode', 'bridge')
                        )
        except Exception as e:
            # CloudFlare failed - will fall back to Supabase
            pass
        
        return None
    
    async def _pull_from_supabase(self, unicode: str) -> Optional[ToolManifest]:
        """Pull from Supabase (fallback)"""
        if not CONFIG['supabase_url'] or not CONFIG['supabase_key']:
            return None
        
        try:
            import aiohttp
            url = f"{CONFIG['supabase_url']}/rest/v1/entities"
            
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    url,
                    params={'unicode_address': f'eq.{unicode}', 'select': '*'},
                    headers={
                        'apikey': CONFIG['supabase_key'],
                        'Authorization': f"Bearer {CONFIG['supabase_key']}"
                    },
                    timeout=aiohttp.ClientTimeout(total=10)
                ) as response:
                    if response.status == 200:
                        data_list = await response.json()
                        if data_list:
                            data = data_list[0]
                            ext = json.loads(data.get('type_extensions', '{}'))
                            
                            print(f"✅ Pulled {unicode} from Supabase (~50ms)")
                            
                            return ToolManifest(
                                unicode=unicode,
                                name=data.get('name', ''),
                                category=ext.get('category', ''),
                                binary_path=ext.get('binary_path'),
                                docker_image=ext.get('docker_image'),
                                capabilities=ext.get('capabilities', []),
                                dependencies=ext.get('dependencies', []),
                                mitre_techniques=ext.get('mitre_techniques', []),
                                terraform_module=ext.get('terraform_module'),
                                command_template=ext.get('command_template', '{binary} {target}')
                            )
        except Exception as e:
            print(f"⚠️  Supabase pull failed: {e}")
        
        return None

# ============================================================================
# TERRAFORM GENERATOR
# ============================================================================

class TerraformGenerator:
    """Generates Terraform configuration from tool manifests"""
    
    def generate_single_tool(self, manifest: ToolManifest, target: str, run_id: str) -> str:
        """Generate Terraform for single tool execution"""
        
        # Parse command template
        command = manifest.command_template.format(
            binary=manifest.binary_path or manifest.name,
            target=target
        ).split()
        
        terraform_config = f"""
terraform {{
  required_providers {{
    docker = {{
      source  = "kreuzwerker/docker"
      version = "~> 3.0"
    }}
  }}
}}

provider "docker" {{}}

# ============================================================================
# SX9 Tool Execution: {manifest.name} (Unicode: {manifest.unicode})
# ============================================================================

resource "docker_image" "{manifest.name}" {{
  name = "{manifest.docker_image or 'kalilinux/kali-rolling:latest'}"
}}

resource "docker_container" "{manifest.name}_runner" {{
  name  = "sx9-{manifest.name}-{run_id}"
  image = docker_image.{manifest.name}.image_id
  
  command = {json.dumps(command)}
  
  network_mode = "{manifest.network_mode}"
  
  {self._generate_capabilities(manifest.capabilities)}
  
  volumes {{
    host_path      = "{CONFIG['output_dir']}/{run_id}"
    container_path = "/output"
  }}
  
  restart = "no"
  rm      = false  # Keep for result extraction
}}

# ============================================================================
# Results Output
# ============================================================================

output "container_id" {{
  value = docker_container.{manifest.name}_runner.id
}}

output "output_dir" {{
  value = "{CONFIG['output_dir']}/{run_id}"
}}

output "tool_name" {{
  value = "{manifest.name}"
}}

output "unicode" {{
  value = "{manifest.unicode}"
}}
"""
        return terraform_config
    
    def generate_tool_chain(self, manifests: List[ToolManifest], target: str, run_id: str) -> str:
        """Generate Terraform for sequential tool chain"""
        
        terraform_config = """
terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0"
    }
  }
}

provider "docker" {}

# Create shared network
resource "docker_network" "attack_network" {
  name = "sx9-chain-""" + run_id + """"
}

"""
        
        # Generate each tool in sequence with dependencies
        for i, manifest in enumerate(manifests):
            phase = i + 1
            depends = f"docker_container.phase{phase-1}_runner" if i > 0 else None
            
            command = manifest.command_template.format(
                binary=manifest.binary_path or manifest.name,
                target=target
            ).split()
            
            terraform_config += f"""
# ============================================================================
# Phase {phase}: {manifest.name} (Unicode: {manifest.unicode})
# ============================================================================

resource "docker_image" "phase{phase}" {{
  name = "{manifest.docker_image or 'kalilinux/kali-rolling:latest'}"
}}

resource "docker_container" "phase{phase}_runner" {{
  name    = "sx9-phase{phase}-{manifest.name}-{run_id}"
  image   = docker_image.phase{phase}.image_id
  network = docker_network.attack_network.name
  
  command = {json.dumps(command)}
  
  {self._generate_capabilities(manifest.capabilities)}
  
  volumes {{
    host_path      = "{CONFIG['output_dir']}/{run_id}/phase{phase}"
    container_path = "/output"
  }}
  
  restart = "no"
  rm      = false
  
  {f'depends_on = [{depends}]' if depends else ''}
}}

"""
        
        return terraform_config
    
    def _generate_capabilities(self, capabilities: List[str]) -> str:
        """Generate Docker capabilities block"""
        if not capabilities:
            return ""
        
        caps = '", "'.join(capabilities)
        return f"""
  capabilities {{
    add = ["{caps}"]
  }}
"""

# ============================================================================
# IAC EXECUTOR
# ============================================================================

class IaCExecutor:
    """Executes infrastructure from tool manifests"""
    
    def __init__(self):
        self.puller = DataPuller()
        self.generator = TerraformGenerator()
    
    async def execute_single_tool(self, unicode: str, target: str) -> Dict:
        """Execute single tool"""
        print(f"\n{'='*70}")
        print(f"🚀 SX9 IaC Executor - Single Tool")
        print(f"{'='*70}")
        print(f"Unicode: {unicode}")
        print(f"Target:  {target}")
        print(f"{'='*70}\n")
        
        # Pull manifest
        print("📥 STEP 1: Pulling tool manifest...")
        manifest = await self.puller.pull(unicode)
        if not manifest:
            return {'error': f'Tool {unicode} not found'}
        
        print(f"   Tool: {manifest.name}")
        print(f"   Category: {manifest.category}")
        print(f"   Docker: {manifest.docker_image or 'default'}")
        
        # Generate Terraform
        print("\n🏗️  STEP 2: Generating Terraform configuration...")
        run_id = self._generate_run_id()
        terraform_config = self.generator.generate_single_tool(manifest, target, run_id)
        
        # Write Terraform files
        terraform_dir = self._create_terraform_dir(run_id)
        self._write_terraform(terraform_dir, terraform_config)
        print(f"   Written to: {terraform_dir}/main.tf")
        
        # Execute Terraform
        print("\n⚡ STEP 3: Executing Terraform apply...")
        result = self._terraform_apply(terraform_dir)
        
        if result['success']:
            print(f"   ✅ Container created: {result['container_id']}")
            print(f"   📁 Output directory: {result['output_dir']}")
            
            # Schedule cleanup
            print(f"\n⏰ STEP 4: Scheduling cleanup in {CONFIG['cleanup_after']}s...")
            asyncio.create_task(self._cleanup_after_delay(terraform_dir, CONFIG['cleanup_after']))
        else:
            print(f"   ❌ Terraform failed: {result['error']}")
        
        return result
    
    async def execute_tool_chain(self, unicode_sequence: List[str], target: str) -> Dict:
        """Execute tool chain (sequential)"""
        print(f"\n{'='*70}")
        print(f"🚀 SX9 IaC Executor - Tool Chain")
        print(f"{'='*70}")
        print(f"Tools:   {', '.join(unicode_sequence)}")
        print(f"Target:  {target}")
        print(f"{'='*70}\n")
        
        # Pull all manifests
        print("📥 STEP 1: Pulling tool manifests...")
        manifests = []
        for unicode in unicode_sequence:
            manifest = await self.puller.pull(unicode)
            if manifest:
                manifests.append(manifest)
                print(f"   ✅ {unicode}: {manifest.name}")
            else:
                print(f"   ❌ {unicode}: Not found")
        
        if not manifests:
            return {'error': 'No tools found'}
        
        # Generate Terraform
        print("\n🏗️  STEP 2: Generating Terraform configuration...")
        run_id = self._generate_run_id()
        terraform_config = self.generator.generate_tool_chain(manifests, target, run_id)
        
        # Write Terraform files
        terraform_dir = self._create_terraform_dir(run_id)
        self._write_terraform(terraform_dir, terraform_config)
        print(f"   Written to: {terraform_dir}/main.tf")
        
        # Execute Terraform
        print("\n⚡ STEP 3: Executing Terraform apply...")
        result = self._terraform_apply(terraform_dir)
        
        if result['success']:
            print(f"   ✅ Tool chain deployed")
            print(f"   📁 Output directory: {result['output_dir']}")
            
            # Schedule cleanup
            print(f"\n⏰ STEP 4: Scheduling cleanup in {CONFIG['cleanup_after']}s...")
            asyncio.create_task(self._cleanup_after_delay(terraform_dir, CONFIG['cleanup_after']))
        else:
            print(f"   ❌ Terraform failed: {result['error']}")
        
        return result
    
    def _generate_run_id(self) -> str:
        """Generate unique run ID"""
        import time
        return f"{int(time.time())}"
    
    def _create_terraform_dir(self, run_id: str) -> Path:
        """Create Terraform working directory"""
        terraform_dir = Path(CONFIG['output_dir']) / run_id / 'terraform'
        terraform_dir.mkdir(parents=True, exist_ok=True)
        return terraform_dir
    
    def _write_terraform(self, terraform_dir: Path, config: str):
        """Write Terraform configuration"""
        (terraform_dir / 'main.tf').write_text(config)
    
    def _terraform_apply(self, terraform_dir: Path) -> Dict:
        """Execute Terraform apply"""
        try:
            # Initialize
            subprocess.run(
                [CONFIG['terraform_bin'], 'init'],
                cwd=terraform_dir,
                check=True,
                capture_output=True
            )
            
            # Apply
            result = subprocess.run(
                [CONFIG['terraform_bin'], 'apply', '-auto-approve', '-json'],
                cwd=terraform_dir,
                check=True,
                capture_output=True,
                text=True
            )
            
            # Parse outputs
            output_result = subprocess.run(
                [CONFIG['terraform_bin'], 'output', '-json'],
                cwd=terraform_dir,
                check=True,
                capture_output=True,
                text=True
            )
            
            outputs = json.loads(output_result.stdout)
            
            return {
                'success': True,
                'container_id': outputs.get('container_id', {}).get('value'),
                'output_dir': outputs.get('output_dir', {}).get('value'),
                'terraform_dir': str(terraform_dir)
            }
        except subprocess.CalledProcessError as e:
            return {
                'success': False,
                'error': e.stderr.decode() if e.stderr else str(e)
            }
    
    async def _cleanup_after_delay(self, terraform_dir: Path, delay: int):
        """Cleanup infrastructure after delay"""
        await asyncio.sleep(delay)
        
        print(f"\n🧹 Cleaning up infrastructure...")
        try:
            subprocess.run(
                [CONFIG['terraform_bin'], 'destroy', '-auto-approve'],
                cwd=terraform_dir,
                check=True,
                capture_output=True
            )
            print(f"   ✅ Infrastructure destroyed")
        except subprocess.CalledProcessError as e:
            print(f"   ❌ Cleanup failed: {e}")

# ============================================================================
# CLI
# ============================================================================

async def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='SX9 IaC Executor')
    parser.add_argument('--unicode', help='Unicode operation (e.g., E800)')
    parser.add_argument('--chain', help='Unicode sequence (comma-separated, e.g., E800,E810,E820)')
    parser.add_argument('--target', required=True, help='Target (IP, URL, etc.)')
    
    args = parser.parse_args()
    
    executor = IaCExecutor()
    
    if args.chain:
        # Tool chain
        unicode_sequence = args.chain.split(',')
        result = await executor.execute_tool_chain(unicode_sequence, args.target)
    elif args.unicode:
        # Single tool
        result = await executor.execute_single_tool(args.unicode, args.target)
    else:
        print("Error: Must specify either --unicode or --chain")
        return
    
    print(f"\n{'='*70}")
    print(f"📊 RESULT:")
    print(json.dumps(result, indent=2))
    print(f"{'='*70}\n")

if __name__ == '__main__':
    asyncio.run(main())
