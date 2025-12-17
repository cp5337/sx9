"""
SX9 IaC Data Pull Orchestrator
===============================

Dynamically pulls threat intelligence data to materialize infrastructure.

Triggered by:
  - Unicode operation (E800-E8FF for tools)
  - Schedule (cron)
  - API call
  - Manual trigger

Materializes:
  - Kali Linux environments with specific tools
  - Network scanning configurations
  - Exploit frameworks
  - Defense testing setups
"""

import os
import json
from typing import Dict, List, Optional
from dataclasses import dataclass
from enum import Enum

# ============================================================================
# PULL SOURCES (Priority Order)
# ============================================================================

class DataSource(Enum):
    """Priority-ordered data sources for IaC"""
    CLOUDFLARE_KV = 1      # Fastest - edge cache (<5ms)
    CLOUDFLARE_R2 = 2      # Fast - CDN (~10-20ms)
    SUPABASE = 3           # Medium - primary storage (~50ms)
    NEON = 4               # Medium - ACID source (~50ms)
    NEO4J = 5              # Slower - graph queries (~100ms)

@dataclass
class ToolManifest:
    """Tool configuration pulled from databases"""
    unicode: str                    # E800-E8FF
    name: str
    category: str
    binary_path: Optional[str]
    dependencies: List[str]
    mitre_techniques: List[str]
    network_config: Dict
    docker_image: Optional[str]
    terraform_module: Optional[str]

# ============================================================================
# IAC DATA PULL ORCHESTRATOR
# ============================================================================

class IaCDataPuller:
    """
    Pulls threat intelligence data from optimal source
    to materialize infrastructure
    """
    
    def __init__(self):
        # CloudFlare
        self.cf_worker_url = os.getenv('CF_WORKER_URL', 'https://sx9-backend.usneodcp.workers.dev')
        
        # Supabase
        self.supabase_url = os.getenv('SUPABASE_URL')
        self.supabase_key = os.getenv('SUPABASE_KEY')
        
        # Neon
        self.neon_url = os.getenv('NEON_DATABASE_URL')
        
        # Neo4j
        self.neo4j_uri = os.getenv('NEO4J_URI')
        
        self.cache = {}
    
    # ========================================================================
    # PULL METHODS (Waterfall Pattern)
    # ========================================================================
    
    async def pull_tool_manifest(self, unicode: str) -> Optional[ToolManifest]:
        """
        Pull tool manifest using waterfall pattern:
        1. Try CloudFlare KV (edge cache)
        2. Fall back to CloudFlare R2 (CDN)
        3. Fall back to Supabase (primary)
        4. Fall back to Neon (ACID)
        5. Fall back to Neo4j (graph)
        """
        
        # STEP 1: CloudFlare KV (edge - <5ms)
        tool_data = await self._pull_from_cloudflare_kv(unicode)
        if tool_data:
            print(f"✅ Pulled {unicode} from CloudFlare KV (<5ms)")
            return self._to_manifest(tool_data)
        
        # STEP 2: CloudFlare R2 (CDN - ~10-20ms)
        tool_data = await self._pull_from_cloudflare_r2(unicode)
        if tool_data:
            print(f"✅ Pulled {unicode} from CloudFlare R2 (~10ms)")
            return self._to_manifest(tool_data)
        
        # STEP 3: Supabase (primary - ~50ms)
        tool_data = await self._pull_from_supabase(unicode)
        if tool_data:
            print(f"✅ Pulled {unicode} from Supabase (~50ms)")
            return self._to_manifest(tool_data)
        
        # STEP 4: Neon (ACID - ~50ms)
        tool_data = await self._pull_from_neon(unicode)
        if tool_data:
            print(f"✅ Pulled {unicode} from Neon (~50ms)")
            return self._to_manifest(tool_data)
        
        # STEP 5: Neo4j (graph - ~100ms)
        tool_data = await self._pull_from_neo4j(unicode)
        if tool_data:
            print(f"✅ Pulled {unicode} from Neo4j (~100ms)")
            return self._to_manifest(tool_data)
        
        print(f"❌ Tool {unicode} not found in any data source")
        return None
    
    async def _pull_from_cloudflare_kv(self, unicode: str) -> Optional[Dict]:
        """Pull from CloudFlare KV (fastest)"""
        try:
            import aiohttp
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.cf_worker_url}/api/tool/{unicode}"
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        # Check X-Cache header
                        if response.headers.get('X-Cache') == 'HIT':
                            return data
        except Exception as e:
            print(f"⚠️  CloudFlare KV miss: {e}")
        return None
    
    async def _pull_from_cloudflare_r2(self, unicode: str) -> Optional[Dict]:
        """Pull from CloudFlare R2 CDN"""
        try:
            import aiohttp
            async with aiohttp.ClientSession() as session:
                # Download full threat-tools.json
                async with session.get(
                    f"https://cdn.sx9.io/threat-intel/threat-tools.json"
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        # Find tool by unicode
                        for tool in data.get('tools', []):
                            if tool.get('hashes', {}).get('unicode', {}).get('sch') == unicode:
                                return tool
        except Exception as e:
            print(f"⚠️  CloudFlare R2 miss: {e}")
        return None
    
    async def _pull_from_supabase(self, unicode: str) -> Optional[Dict]:
        """Pull from Supabase (primary storage)"""
        if not self.supabase_url or not self.supabase_key:
            return None
        
        try:
            import aiohttp
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.supabase_url}/rest/v1/entities",
                    params={'unicode_address': f'eq.{unicode}', 'select': '*'},
                    headers={
                        'apikey': self.supabase_key,
                        'Authorization': f'Bearer {self.supabase_key}'
                    }
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        return data[0] if data else None
        except Exception as e:
            print(f"⚠️  Supabase miss: {e}")
        return None
    
    async def _pull_from_neon(self, unicode: str) -> Optional[Dict]:
        """Pull from Neon PostgreSQL"""
        if not self.neon_url:
            return None
        
        try:
            import asyncpg
            conn = await asyncpg.connect(self.neon_url)
            row = await conn.fetchrow(
                "SELECT * FROM tool_registry WHERE unicode = $1",
                unicode
            )
            await conn.close()
            return dict(row) if row else None
        except Exception as e:
            print(f"⚠️  Neon miss: {e}")
        return None
    
    async def _pull_from_neo4j(self, unicode: str) -> Optional[Dict]:
        """Pull from Neo4j (graph queries)"""
        if not self.neo4j_uri:
            return None
        
        try:
            from neo4j import AsyncGraphDatabase
            async with AsyncGraphDatabase.driver(self.neo4j_uri) as driver:
                async with driver.session() as session:
                    result = await session.run(
                        "MATCH (t:Tool {unicode: $unicode}) RETURN t",
                        unicode=unicode
                    )
                    record = await result.single()
                    return dict(record['t']) if record else None
        except Exception as e:
            print(f"⚠️  Neo4j miss: {e}")
        return None
    
    def _to_manifest(self, tool_data: Dict) -> ToolManifest:
        """Convert raw tool data to IaC manifest"""
        return ToolManifest(
            unicode=tool_data.get('unicode_address') or tool_data.get('unicode'),
            name=tool_data.get('name', ''),
            category=tool_data.get('category', ''),
            binary_path=tool_data.get('binary_path'),
            dependencies=tool_data.get('dependencies', []),
            mitre_techniques=tool_data.get('mitre_techniques', []),
            network_config=tool_data.get('network_config', {}),
            docker_image=tool_data.get('docker_image'),
            terraform_module=tool_data.get('terraform_module')
        )
    
    # ========================================================================
    # BULK OPERATIONS
    # ========================================================================
    
    async def pull_tool_chain(self, unicode_sequence: List[str]) -> List[ToolManifest]:
        """Pull multiple tools in sequence (for tool chains)"""
        manifests = []
        for unicode in unicode_sequence:
            manifest = await self.pull_tool_manifest(unicode)
            if manifest:
                manifests.append(manifest)
        return manifests
    
    async def pull_by_category(self, category: str) -> List[ToolManifest]:
        """Pull all tools in a category"""
        # Query Supabase for all tools in category
        if not self.supabase_url or not self.supabase_key:
            return []
        
        try:
            import aiohttp
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.supabase_url}/rest/v1/entities",
                    params={'category': f'eq.{category}', 'select': '*'},
                    headers={
                        'apikey': self.supabase_key,
                        'Authorization': f'Bearer {self.supabase_key}'
                    }
                ) as response:
                    if response.status == 200:
                        tools = await response.json()
                        return [self._to_manifest(tool) for tool in tools]
        except Exception as e:
            print(f"❌ Error pulling category {category}: {e}")
        
        return []
    
    async def pull_by_mitre_technique(self, technique_id: str) -> List[ToolManifest]:
        """Pull all tools that implement a MITRE technique"""
        # Query via CloudFlare Worker or Supabase
        # Then filter by MITRE technique
        
        # For now, use Supabase JSONB query
        if not self.supabase_url or not self.supabase_key:
            return []
        
        try:
            import aiohttp
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.supabase_url}/rest/v1/entities",
                    params={
                        'select': '*',
                        'type_extensions->>mitre_techniques': f'cs.{{{technique_id}}}'
                    },
                    headers={
                        'apikey': self.supabase_key,
                        'Authorization': f'Bearer {self.supabase_key}'
                    }
                ) as response:
                    if response.status == 200:
                        tools = await response.json()
                        return [self._to_manifest(tool) for tool in tools]
        except Exception as e:
            print(f"❌ Error pulling MITRE {technique_id}: {e}")
        
        return []

# ============================================================================
# IAC MATERIALIZER
# ============================================================================

class IaCMaterializer:
    """
    Takes tool manifests and materializes infrastructure
    """
    
    def __init__(self, puller: IaCDataPuller):
        self.puller = puller
    
    async def materialize_kali_env(self, tool_unicodes: List[str]) -> Dict:
        """
        Materialize Kali Linux environment with specific tools
        
        Returns Terraform/Pulumi config
        """
        # Pull tool manifests
        manifests = await self.puller.pull_tool_chain(tool_unicodes)
        
        # Generate Terraform config
        terraform_config = {
            "resource": {
                "docker_container": {
                    "kali_env": {
                        "name": "sx9-kali-runtime",
                        "image": "kalilinux/kali-rolling:latest",
                        "env": self._generate_env_vars(manifests),
                        "volumes": self._generate_volumes(manifests),
                        "command": self._generate_startup_script(manifests)
                    }
                }
            }
        }
        
        return terraform_config
    
    async def materialize_network_scan(self, target: str, unicode: str) -> Dict:
        """
        Materialize network scanning infrastructure
        """
        # Pull scanning tool manifest
        manifest = await self.puller.pull_tool_manifest(unicode)
        
        if not manifest:
            raise ValueError(f"Tool {unicode} not found")
        
        # Generate config
        return {
            "tool": manifest.name,
            "target": target,
            "command": f"{manifest.binary_path} {target}",
            "network_mode": manifest.network_config.get('mode', 'host'),
            "capabilities": manifest.network_config.get('capabilities', [])
        }
    
    def _generate_env_vars(self, manifests: List[ToolManifest]) -> List[str]:
        """Generate environment variables for container"""
        env_vars = []
        for manifest in manifests:
            env_vars.append(f"TOOL_{manifest.name.upper()}_PATH={manifest.binary_path}")
        return env_vars
    
    def _generate_volumes(self, manifests: List[ToolManifest]) -> List[Dict]:
        """Generate volume mounts"""
        volumes = []
        for manifest in manifests:
            if manifest.binary_path:
                volumes.append({
                    "host_path": f"/opt/sx9/tools/{manifest.name}",
                    "container_path": manifest.binary_path
                })
        return volumes
    
    def _generate_startup_script(self, manifests: List[ToolManifest]) -> List[str]:
        """Generate container startup commands"""
        commands = ["#!/bin/bash", "set -e"]
        
        for manifest in manifests:
            # Install dependencies
            if manifest.dependencies:
                commands.append(f"apt-get update && apt-get install -y {' '.join(manifest.dependencies)}")
            
            # Setup tool
            if manifest.binary_path:
                commands.append(f"ln -sf {manifest.binary_path} /usr/local/bin/{manifest.name}")
        
        commands.append("exec /bin/bash")
        return commands

# ============================================================================
# USAGE EXAMPLE
# ============================================================================

async def main():
    """Example usage"""
    puller = IaCDataPuller()
    materializer = IaCMaterializer(puller)
    
    # Example 1: Pull single tool
    print("📥 Pulling Nmap manifest...")
    nmap_manifest = await puller.pull_tool_manifest("E800")
    print(f"✅ Got: {nmap_manifest}")
    
    # Example 2: Pull tool chain
    print("\n📥 Pulling reconnaissance tool chain...")
    recon_chain = await puller.pull_tool_chain(["E800", "E801", "E802"])
    print(f"✅ Got {len(recon_chain)} tools")
    
    # Example 3: Materialize Kali environment
    print("\n🏗️  Materializing Kali environment...")
    terraform_config = await materializer.materialize_kali_env(["E800", "E810"])
    print(f"✅ Terraform config generated")
    print(json.dumps(terraform_config, indent=2))
    
    # Example 4: Pull by MITRE technique
    print("\n📥 Pulling tools for T1046 (Network Service Discovery)...")
    t1046_tools = await puller.pull_by_mitre_technique("T1046")
    print(f"✅ Found {len(t1046_tools)} tools")

if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
