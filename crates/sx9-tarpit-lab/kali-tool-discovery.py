#!/usr/bin/env python3
"""
Kali Tool Discovery Script

Enumerates all Kali tools and captures their help output to understand:
- Available commands
- Required arguments
- Output formats
- Automation potential
"""

import subprocess
import json
from pathlib import Path
from typing import Dict, List, Optional
import re

class KaliToolDiscovery:
    def __init__(self, output_dir: str = "./tool-discovery"):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        
        # Known Kali tool categories
        self.tool_categories = {
            'information_gathering': [
                'nmap', 'masscan', 'nikto', 'whatweb', 'wafw00f',
                'dnsenum', 'dnsrecon', 'fierce', 'sublist3r',
                'theharvester', 'recon-ng', 'maltego'
            ],
            'vulnerability_analysis': [
                'nuclei', 'nikto', 'wpscan', 'sqlmap', 'commix',
                'xsser', 'skipfish', 'wapiti', 'burpsuite'
            ],
            'exploitation': [
                'metasploit-framework', 'msfconsole', 'msfvenom',
                'exploit-db', 'searchsploit', 'beef-xss',
                'armitage', 'social-engineer-toolkit'
            ],
            'post_exploitation': [
                'mimikatz', 'powersploit', 'empire', 'covenant',
                'bloodhound', 'crackmapexec', 'impacket',
                'responder', 'evil-winrm'
            ],
            'password_attacks': [
                'john', 'hashcat', 'hydra', 'medusa', 'ncrack',
                'ophcrack', 'patator', 'thc-pptp-bruter'
            ],
            'wireless_attacks': [
                'aircrack-ng', 'reaver', 'wifite', 'kismet',
                'fern-wifi-cracker', 'pixiewps'
            ],
            'web_applications': [
                'burpsuite', 'zaproxy', 'sqlmap', 'wpscan',
                'nikto', 'dirb', 'dirbuster', 'gobuster',
                'wfuzz', 'ffuf', 'commix'
            ],
        }
    
    def find_tool_binary(self, tool_name: str) -> Optional[str]:
        """Find the actual binary path for a tool"""
        try:
            result = subprocess.run(
                ['which', tool_name],
                capture_output=True,
                text=True,
                timeout=5
            )
            if result.returncode == 0:
                return result.stdout.strip()
        except:
            pass
        return None
    
    def get_help_output(self, tool_name: str, binary_path: str) -> Dict:
        """Try multiple ways to get help output"""
        help_variants = [
            ['--help'],
            ['-h'],
            ['-help'],
            ['help'],
            []  # Some tools show help with no args
        ]
        
        for variant in help_variants:
            try:
                result = subprocess.run(
                    [binary_path] + variant,
                    capture_output=True,
                    text=True,
                    timeout=10
                )
                
                # Check if we got useful output
                output = result.stdout + result.stderr
                if len(output) > 50:  # Arbitrary threshold
                    return {
                        'tool': tool_name,
                        'binary': binary_path,
                        'help_flag': ' '.join(variant) if variant else '(no args)',
                        'output': output,
                        'returncode': result.returncode
                    }
            except subprocess.TimeoutExpired:
                continue
            except Exception as e:
                continue
        
        return {
            'tool': tool_name,
            'binary': binary_path,
            'help_flag': 'FAILED',
            'output': 'Could not retrieve help',
            'returncode': -1
        }
    
    def parse_help_for_automation(self, help_output: str) -> Dict:
        """Parse help output to find automation-friendly flags"""
        automation_hints = {
            'json_output': bool(re.search(r'--json|-j\s|--output-json', help_output, re.I)),
            'xml_output': bool(re.search(r'--xml|-x\s|--output-xml', help_output, re.I)),
            'quiet_mode': bool(re.search(r'--quiet|-q\s|--silent', help_output, re.I)),
            'batch_mode': bool(re.search(r'--batch|--non-interactive', help_output, re.I)),
            'output_file': bool(re.search(r'--output|-o\s|--out', help_output, re.I)),
            'target_file': bool(re.search(r'--target-file|--input-file|-iL', help_output, re.I)),
        }
        
        return automation_hints
    
    def discover_all_tools(self):
        """Discover all tools and capture their help output"""
        results = {}
        
        for category, tools in self.tool_categories.items():
            print(f"\n🔍 Discovering {category} tools...")
            results[category] = []
            
            for tool in tools:
                print(f"  Checking {tool}...", end=' ')
                
                binary = self.find_tool_binary(tool)
                if not binary:
                    print("❌ Not found")
                    continue
                
                print(f"✅ Found at {binary}")
                
                # Get help output
                help_data = self.get_help_output(tool, binary)
                
                # Parse for automation hints
                automation = self.parse_help_for_automation(help_data['output'])
                help_data['automation'] = automation
                
                results[category].append(help_data)
                
                # Save individual tool help
                tool_file = self.output_dir / f"{tool}_help.txt"
                with open(tool_file, 'w') as f:
                    f.write(help_data['output'])
        
        # Save summary
        summary_file = self.output_dir / "tool_discovery_summary.json"
        with open(summary_file, 'w') as f:
            # Remove full output from summary (too large)
            summary = {}
            for cat, tool_list in results.items():
                summary[cat] = [
                    {
                        'tool': t['tool'],
                        'binary': t['binary'],
                        'help_flag': t['help_flag'],
                        'automation': t['automation']
                    }
                    for t in tool_list
                ]
            json.dump(summary, f, indent=2)
        
        return results
    
    def generate_automation_report(self, results: Dict):
        """Generate report of which tools are automation-friendly"""
        report = []
        report.append("# Kali Tool Automation Report\n")
        report.append("## Tools Suitable for Automation\n")
        
        for category, tools in results.items():
            automatable = [
                t for t in tools
                if t['automation']['json_output'] or
                   t['automation']['batch_mode'] or
                   t['automation']['output_file']
            ]
            
            if automatable:
                report.append(f"\n### {category.replace('_', ' ').title()}\n")
                for tool in automatable:
                    flags = []
                    if tool['automation']['json_output']:
                        flags.append('JSON output')
                    if tool['automation']['batch_mode']:
                        flags.append('Batch mode')
                    if tool['automation']['quiet_mode']:
                        flags.append('Quiet mode')
                    
                    report.append(f"- **{tool['tool']}**: {', '.join(flags)}\n")
        
        report_file = self.output_dir / "automation_report.md"
        with open(report_file, 'w') as f:
            f.writelines(report)
        
        print(f"\n📊 Automation report saved to {report_file}")

if __name__ == "__main__":
    print("🔧 Kali Tool Discovery Script")
    print("=" * 60)
    
    discovery = KaliToolDiscovery()
    results = discovery.discover_all_tools()
    discovery.generate_automation_report(results)
    
    print("\n✅ Discovery complete!")
    print(f"📁 Results saved to: {discovery.output_dir}")
