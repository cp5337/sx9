#!/usr/bin/env python3
"""
🏴‍☠️ sx9-api-vault: Centralized API Key Management System
Mission: Track ALL API keys, their origins, and which tools need them

This extends the pirate system to:
1. Crawl 600+ Kali tools to find API key requirements
2. Track where each key came from (service, account, signup URL)
3. Auto-provision keys to tools that need them
4. Eliminate the "searching for fucking keys" problem
"""

import json
import os
import re
import subprocess
from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass, asdict
from datetime import datetime

@dataclass
class APIKeyMetadata:
    """Complete metadata for an API key"""
    key_name: str  # e.g., "SHODAN_API_KEY"
    service: str  # e.g., "Shodan"
    key_value: str  # The actual key
    origin: str  # Where it came from (account email, etc.)
    signup_url: str  # Where to get a new one
    free_tier: bool  # Is there a free tier?
    rate_limits: Dict[str, int]  # {"requests_per_day": 100}
    required_by: List[str]  # Tools that need this key
    obtained_date: str  # When we got it
    expires: Optional[str]  # Expiration date if any
    notes: str  # Any additional notes
    
@dataclass
class ToolAPIRequirement:
    """API requirements for a specific tool"""
    tool_name: str  # e.g., "theHarvester"
    tool_path: str  # Path to the tool
    required_keys: List[str]  # API keys it needs
    optional_keys: List[str]  # Optional API keys
    config_file: Optional[str]  # Where to put the keys
    env_vars: List[str]  # Environment variables it expects
    setup_instructions: str  # How to configure it

class APIVault:
    """Centralized API Key Vault"""
    
    def __init__(self, vault_path: str = "~/.sx9-api-vault"):
        self.vault_path = Path(vault_path).expanduser()
        self.vault_path.mkdir(parents=True, exist_ok=True)
        
        self.keys_db = self.vault_path / "keys.json"
        self.tools_db = self.vault_path / "tools.json"
        self.registry_db = self.vault_path / "registry.json"
        
        self.keys: Dict[str, APIKeyMetadata] = self.load_keys()
        self.tools: Dict[str, ToolAPIRequirement] = self.load_tools()
        self.registry = self.load_registry()
    
    def load_keys(self) -> Dict[str, APIKeyMetadata]:
        """Load API keys from vault"""
        if not self.keys_db.exists():
            return {}
        
        with open(self.keys_db) as f:
            data = json.load(f)
            return {
                k: APIKeyMetadata(**v) for k, v in data.items()
            }
    
    def save_keys(self):
        """Save API keys to vault"""
        with open(self.keys_db, 'w') as f:
            json.dump(
                {k: asdict(v) for k, v in self.keys.items()},
                f, indent=2
            )
    
    def load_tools(self) -> Dict[str, ToolAPIRequirement]:
        """Load tool requirements from vault"""
        if not self.tools_db.exists():
            return {}
        
        with open(self.tools_db) as f:
            data = json.load(f)
            return {
                k: ToolAPIRequirement(**v) for k, v in data.items()
            }
    
    def save_tools(self):
        """Save tool requirements to vault"""
        with open(self.tools_db, 'w') as f:
            json.dump(
                {k: asdict(v) for k, v in self.tools.items()},
                f, indent=2
            )
    
    def load_registry(self) -> Dict:
        """Load API service registry"""
        if not self.registry_db.exists():
            return self.get_default_registry()
        
        with open(self.registry_db) as f:
            return json.load(f)
    
    def save_registry(self):
        """Save API service registry"""
        with open(self.registry_db, 'w') as f:
            json.dump(self.registry, f, indent=2)
    
    def get_default_registry(self) -> Dict:
        """Default registry of API services"""
        return {
            "shodan": {
                "name": "Shodan",
                "signup_url": "https://account.shodan.io/register",
                "key_location": "https://account.shodan.io/",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 100},
                "env_var": "SHODAN_API_KEY",
                "used_by": ["shodan", "theHarvester", "recon-ng"]
            },
            "virustotal": {
                "name": "VirusTotal",
                "signup_url": "https://www.virustotal.com/gui/join-us",
                "key_location": "https://www.virustotal.com/gui/user/YOUR_USERNAME/apikey",
                "free_tier": True,
                "rate_limits": {"requests_per_day": 500},
                "env_var": "VT_API_KEY",
                "used_by": ["vt-cli", "theHarvester", "recon-ng"]
            },
            "censys": {
                "name": "Censys",
                "signup_url": "https://censys.io/register",
                "key_location": "https://censys.io/account/api",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 250},
                "env_var": "CENSYS_API_ID",
                "used_by": ["censys-cli", "theHarvester"]
            },
            "hunter": {
                "name": "Hunter.io",
                "signup_url": "https://hunter.io/users/sign_up",
                "key_location": "https://hunter.io/api_keys",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 50},
                "env_var": "HUNTER_API_KEY",
                "used_by": ["theHarvester"]
            },
            "securitytrails": {
                "name": "SecurityTrails",
                "signup_url": "https://securitytrails.com/app/signup",
                "key_location": "https://securitytrails.com/app/account/credentials",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 50},
                "env_var": "SECURITYTRAILS_API_KEY",
                "used_by": ["theHarvester", "recon-ng"]
            },
            "binaryedge": {
                "name": "BinaryEdge",
                "signup_url": "https://app.binaryedge.io/sign-up",
                "key_location": "https://app.binaryedge.io/account/api",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 250},
                "env_var": "BINARYEDGE_API_KEY",
                "used_by": ["theHarvester"]
            },
            "fullhunt": {
                "name": "FullHunt",
                "signup_url": "https://fullhunt.io/auth/register",
                "key_location": "https://fullhunt.io/user/settings",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 1000},
                "env_var": "FULLHUNT_API_KEY",
                "used_by": ["theHarvester"]
            },
            "github": {
                "name": "GitHub",
                "signup_url": "https://github.com/settings/tokens/new",
                "key_location": "https://github.com/settings/tokens",
                "free_tier": True,
                "rate_limits": {"requests_per_hour": 5000},
                "env_var": "GITHUB_TOKEN",
                "used_by": ["theHarvester", "gitrob", "truffleHog"]
            },
            "intelx": {
                "name": "Intelligence X",
                "signup_url": "https://intelx.io/signup",
                "key_location": "https://intelx.io/account?tab=developer",
                "free_tier": True,
                "rate_limits": {"requests_per_month": 1000},
                "env_var": "INTELX_API_KEY",
                "used_by": ["theHarvester"]
            },
            "openai": {
                "name": "OpenAI",
                "signup_url": "https://platform.openai.com/signup",
                "key_location": "https://platform.openai.com/api-keys",
                "free_tier": False,
                "rate_limits": {"requests_per_minute": 60},
                "env_var": "OPENAI_API_KEY",
                "used_by": ["gpt-cli", "ai-tools"]
            },
            "anthropic": {
                "name": "Anthropic (Claude)",
                "signup_url": "https://console.anthropic.com/",
                "key_location": "https://console.anthropic.com/settings/keys",
                "free_tier": False,
                "rate_limits": {"requests_per_minute": 50},
                "env_var": "ANTHROPIC_API_KEY",
                "used_by": ["claude-cli", "ai-tools"]
            },
            "gemini": {
                "name": "Google Gemini",
                "signup_url": "https://makersuite.google.com/app/apikey",
                "key_location": "https://makersuite.google.com/app/apikey",
                "free_tier": True,
                "rate_limits": {"requests_per_day": 1500},
                "env_var": "GEMINI_API_KEY",
                "used_by": ["gemini-cli", "ai-tools"]
            }
        }
    
    def register_key(self, key_name: str, key_value: str, service: str, 
                    origin: str, notes: str = ""):
        """Register a new API key in the vault"""
        
        # Get service info from registry
        service_info = self.registry.get(service.lower(), {})
        
        metadata = APIKeyMetadata(
            key_name=key_name,
            service=service,
            key_value=key_value,
            origin=origin,
            signup_url=service_info.get("signup_url", ""),
            free_tier=service_info.get("free_tier", False),
            rate_limits=service_info.get("rate_limits", {}),
            required_by=service_info.get("used_by", []),
            obtained_date=datetime.now().isoformat(),
            expires=None,
            notes=notes
        )
        
        self.keys[key_name] = metadata
        self.save_keys()
        
        print(f"✅ Registered {key_name} for {service}")
        print(f"   Origin: {origin}")
        print(f"   Used by: {', '.join(metadata.required_by)}")
    
    def crawl_kali_tools(self, kali_path: str = "/usr/share"):
        """Crawl Kali tools to find API key requirements"""
        print("🔍 Crawling Kali tools for API key requirements...")
        
        kali_tools = Path(kali_path)
        if not kali_tools.exists():
            print(f"⚠️  Kali tools path not found: {kali_path}")
            return
        
        # Common patterns for API key requirements
        patterns = {
            'env_var': r'os\.getenv\(["\']([A-Z_]*API_KEY["\'])\)',
            'config': r'api[_-]?key\s*[:=]\s*["\']?([^"\']+)["\']?',
            'comment': r'#.*API.*key.*:?\s*([A-Z_]+)',
        }
        
        tools_found = 0
        
        for tool_dir in kali_tools.iterdir():
            if not tool_dir.is_dir():
                continue
            
            # Scan Python files
            for py_file in tool_dir.rglob("*.py"):
                requirements = self.scan_file_for_api_keys(py_file, patterns)
                if requirements:
                    self.register_tool_requirements(
                        tool_name=tool_dir.name,
                        tool_path=str(tool_dir),
                        requirements=requirements
                    )
                    tools_found += 1
        
        print(f"✅ Found {tools_found} tools with API key requirements")
        self.save_tools()
    
    def scan_file_for_api_keys(self, file_path: Path, patterns: Dict) -> Dict:
        """Scan a file for API key requirements"""
        try:
            with open(file_path, 'r', errors='ignore') as f:
                content = f.read()
            
            found_keys = set()
            
            for pattern_type, pattern in patterns.items():
                matches = re.findall(pattern, content, re.IGNORECASE)
                found_keys.update(matches)
            
            if found_keys:
                return {
                    'required_keys': list(found_keys),
                    'file': str(file_path)
                }
            
        except Exception as e:
            pass
        
        return {}
    
    def register_tool_requirements(self, tool_name: str, tool_path: str, 
                                   requirements: Dict):
        """Register a tool's API key requirements"""
        
        tool_req = ToolAPIRequirement(
            tool_name=tool_name,
            tool_path=tool_path,
            required_keys=requirements.get('required_keys', []),
            optional_keys=[],
            config_file=None,
            env_vars=requirements.get('required_keys', []),
            setup_instructions=f"See {requirements.get('file', '')}"
        )
        
        self.tools[tool_name] = tool_req
    
    def provision_tool(self, tool_name: str) -> bool:
        """Auto-provision API keys for a tool"""
        if tool_name not in self.tools:
            print(f"❌ Tool {tool_name} not found in registry")
            return False
        
        tool = self.tools[tool_name]
        missing_keys = []
        
        print(f"🔧 Provisioning {tool_name}...")
        
        for key_name in tool.required_keys:
            if key_name not in self.keys:
                missing_keys.append(key_name)
            else:
                # Export to environment
                os.environ[key_name] = self.keys[key_name].key_value
                print(f"  ✅ {key_name} provisioned")
        
        if missing_keys:
            print(f"  ⚠️  Missing keys: {', '.join(missing_keys)}")
            for key in missing_keys:
                self.show_how_to_get_key(key)
            return False
        
        return True
    
    def show_how_to_get_key(self, key_name: str):
        """Show instructions for obtaining a missing key"""
        # Try to find in registry
        for service_id, service_info in self.registry.items():
            if service_info.get('env_var') == key_name:
                print(f"\n📋 How to get {key_name}:")
                print(f"   Service: {service_info['name']}")
                print(f"   Signup: {service_info['signup_url']}")
                print(f"   Get Key: {service_info['key_location']}")
                print(f"   Free Tier: {'Yes' if service_info['free_tier'] else 'No'}")
                print(f"   Rate Limits: {service_info['rate_limits']}")
                return
        
        print(f"\n❓ {key_name} - Unknown service. Search online for signup.")
    
    def export_to_env_file(self, output_path: str = ".env"):
        """Export all keys to .env file"""
        with open(output_path, 'w') as f:
            f.write("# sx9-api-vault - Auto-generated API Keys\n")
            f.write(f"# Generated: {datetime.now().isoformat()}\n\n")
            
            for key_name, metadata in self.keys.items():
                f.write(f"# {metadata.service} - {metadata.origin}\n")
                f.write(f"{key_name}={metadata.key_value}\n\n")
        
        print(f"✅ Exported {len(self.keys)} keys to {output_path}")
    
    def generate_report(self) -> Dict:
        """Generate comprehensive vault report"""
        return {
            'total_keys': len(self.keys),
            'total_tools': len(self.tools),
            'keys_by_service': self.group_keys_by_service(),
            'tools_missing_keys': self.find_tools_missing_keys(),
            'free_tier_keys': self.count_free_tier_keys(),
            'coverage': self.calculate_coverage()
        }
    
    def group_keys_by_service(self) -> Dict:
        """Group keys by service"""
        grouped = {}
        for key_name, metadata in self.keys.items():
            service = metadata.service
            if service not in grouped:
                grouped[service] = []
            grouped[service].append(key_name)
        return grouped
    
    def find_tools_missing_keys(self) -> List[str]:
        """Find tools that are missing required keys"""
        missing = []
        for tool_name, tool in self.tools.items():
            for key in tool.required_keys:
                if key not in self.keys:
                    missing.append(f"{tool_name} needs {key}")
        return missing
    
    def count_free_tier_keys(self) -> int:
        """Count how many keys are free tier"""
        return sum(1 for k in self.keys.values() if k.free_tier)
    
    def calculate_coverage(self) -> float:
        """Calculate what % of tools have all required keys"""
        if not self.tools:
            return 0.0
        
        fully_provisioned = 0
        for tool in self.tools.values():
            if all(key in self.keys for key in tool.required_keys):
                fully_provisioned += 1
        
        return (fully_provisioned / len(self.tools)) * 100

def main():
    """Main CLI interface"""
    import argparse
    
    parser = argparse.ArgumentParser(description="sx9-api-vault: Centralized API Key Management")
    subparsers = parser.add_subparsers(dest='command')
    
    # Register key
    register = subparsers.add_parser('register', help='Register a new API key')
    register.add_argument('key_name', help='Key name (e.g., SHODAN_API_KEY)')
    register.add_argument('key_value', help='The actual API key')
    register.add_argument('service', help='Service name (e.g., shodan)')
    register.add_argument('origin', help='Where you got it (email, account, etc.)')
    register.add_argument('--notes', default='', help='Additional notes')
    
    # Crawl Kali tools
    crawl = subparsers.add_parser('crawl', help='Crawl Kali tools for API requirements')
    crawl.add_argument('--path', default='/usr/share', help='Path to Kali tools')
    
    # Provision tool
    provision = subparsers.add_parser('provision', help='Provision API keys for a tool')
    provision.add_argument('tool_name', help='Tool name')
    
    # Export keys
    export = subparsers.add_parser('export', help='Export keys to .env file')
    export.add_argument('--output', default='.env', help='Output file')
    
    # Show report
    subparsers.add_parser('report', help='Show vault report')
    
    # List keys
    subparsers.add_parser('list', help='List all keys')
    
    args = parser.parse_args()
    
    vault = APIVault()
    
    if args.command == 'register':
        vault.register_key(args.key_name, args.key_value, args.service, args.origin, args.notes)
    
    elif args.command == 'crawl':
        vault.crawl_kali_tools(args.path)
    
    elif args.command == 'provision':
        vault.provision_tool(args.tool_name)
    
    elif args.command == 'export':
        vault.export_to_env_file(args.output)
    
    elif args.command == 'report':
        report = vault.generate_report()
        print("\n📊 sx9-api-vault Report")
        print("=" * 60)
        print(f"Total Keys: {report['total_keys']}")
        print(f"Total Tools: {report['total_tools']}")
        print(f"Free Tier Keys: {report['free_tier_keys']}")
        print(f"Coverage: {report['coverage']:.1f}%")
        print(f"\nKeys by Service:")
        for service, keys in report['keys_by_service'].items():
            print(f"  {service}: {len(keys)} keys")
        print(f"\nTools Missing Keys ({len(report['tools_missing_keys'])}):")
        for missing in report['tools_missing_keys'][:10]:
            print(f"  ⚠️  {missing}")
    
    elif args.command == 'list':
        print("\n🔑 Registered API Keys")
        print("=" * 60)
        for key_name, metadata in vault.keys.items():
            print(f"\n{key_name}")
            print(f"  Service: {metadata.service}")
            print(f"  Origin: {metadata.origin}")
            print(f"  Free Tier: {'Yes' if metadata.free_tier else 'No'}")
            print(f"  Used by: {', '.join(metadata.required_by)}")
    
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
