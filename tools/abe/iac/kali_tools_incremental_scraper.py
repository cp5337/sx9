#!/usr/bin/env python3
"""
Kali Tools Incremental Scraper
Continues from existing 335 tools to get remaining ~265 tools (target: 600)
"""

import requests
from bs4 import BeautifulSoup
import json
import time
from pathlib import Path
from typing import List, Dict, Any

# Configuration
KALI_TOOLS_INDEX_URL = "https://www.kali.org/tools/"
EXISTING_DATA = Path("/Users/cp5337/Developer/sx9/.qodo/04-abe-iac/node-interview-generator/output/kali_tools/kali_tools_complete.json")
OUTPUT_FILE = Path("/Users/cp5337/Developer/sx9/.qodo/04-abe-iac/node-interview-generator/output/kali_tools/kali_tools_complete_600.json")

def load_existing_tools():
    """Load existing 335 tools"""
    with open(EXISTING_DATA, 'r') as f:
        data = json.load(f)
    return data

def scrape_tool_detail(tool_url: str, slug: str) -> Dict[str, Any]:
    """Scrape individual tool page for details"""
    try:
        time.sleep(0.5)  # Rate limiting: 2 requests/second
        response = requests.get(tool_url, timeout=15)
        response.raise_for_status()
        soup = BeautifulSoup(response.content, 'html.parser')
        
        tool_data = {
            "slug": slug,
            "name": slug,
            "url": tool_url,
            "package_name": slug,
            "description": "",
            "homepage": "",
            "git_repo": f"https://gitlab.com/kalilinux/packages/{slug}.git",
            "categories": [],
            "commands": [],
            "installed_size": "",
            "version": "version:",
            "author": "maintainer:",
            "scraped_at": time.strftime("%Y-%m-%dT%H:%M:%S"),
            "source": "scraped"
        }
        
        # Extract description
        desc_div = soup.find('div', class_='tool-description') or soup.find('div', class_='content')
        if desc_div:
            tool_data['description'] = desc_div.get_text(strip=True)[:500]
        
        # Extract homepage
        homepage_link = soup.find('a', string=lambda s: s and 'homepage' in s.lower())
        if homepage_link:
            tool_data['homepage'] = homepage_link.get('href', '')
        
        # Extract categories
        category_badges = soup.find_all('span', class_='badge')
        tool_data['categories'] = [badge.get_text(strip=True) for badge in category_badges]
        
        return tool_data
        
    except Exception as e:
        print(f"  ⚠️  Error scraping {slug}: {e}")
        return {
            "slug": slug,
            "name": slug,
            "url": tool_url,
            "package_name": slug,
            "git_repo": f"https://gitlab.com/kalilinux/packages/{slug}.git",
            "source": "fallback"
        }

def scrape_remaining_tools(existing_data: Dict, start_from: int = 335, target: int = 600):
    """Scrape remaining tools from Kali.org"""
    print(f"🔍 Scraping Kali tools from #{start_from} to #{target}...")
    
    existing_tools = existing_data.get('tools', {})
    existing_slugs = set(existing_tools.keys())
    
    print(f"   Already have: {len(existing_slugs)} tools")
    
    # Fetch main tools index
    try:
        response = requests.get(KALI_TOOLS_INDEX_URL, timeout=30)
        response.raise_for_status()
        soup = BeautifulSoup(response.content, 'html.parser')
        
        # Find all tool links
        tool_links = soup.select('a[href^="/tools/"]')
        all_tool_slugs = []
        
        for link in tool_links:
            href = link.get('href', '')
            if href and href.startswith('/tools/') and href != '/tools/':
                slug = href.strip('/').split('/')[-1]
                if slug and slug not in existing_slugs:
                    all_tool_slugs.append(slug)
        
        # Remove duplicates
        all_tool_slugs = list(set(all_tool_slugs))
        print(f"   Found {len(all_tool_slugs)} new tools to scrape")
        
        # Scrape new tools
        new_tools = {}
        for i, slug in enumerate(all_tool_slugs, start=1):
            if len(existing_tools) + len(new_tools) >= target:
                print(f"   ✅ Reached target of {target} tools!")
                break
            
            tool_url = f"https://www.kali.org/tools/{slug}/"
            print(f"   [{len(existing_tools) + len(new_tools) + 1}/{target}] Scraping: {slug}")
            
            tool_data = scrape_tool_detail(tool_url, slug)
            new_tools[slug] = tool_data
            
            if (i % 10) == 0:
                print(f"      Progress: {len(new_tools)} new tools scraped")
        
        # Merge with existing
        merged_data = {
            "scraped_at": time.strftime("%Y-%m-%dT%H:%M:%S"),
            "source": "kali.org + incremental",
            "tool_count": len(existing_tools) + len(new_tools),
            "tools": {**existing_tools, **new_tools}
        }
        
        return merged_data
        
    except Exception as e:
        print(f"❌ Error during scraping: {e}")
        return existing_data

if __name__ == '__main__':
    print("═" * 60)
    print("Kali Tools Incremental Scraper")
    print("═" * 60)
    
    # Load existing data
    existing_data = load_existing_tools()
    print(f"✅ Loaded existing data: {existing_data.get('tool_count', 0)} tools")
    
    # Scrape remaining
    merged_data = scrape_remaining_tools(existing_data, start_from=335, target=600)
    
    # Save
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, 'w') as f:
        json.dump(merged_data, f, indent=2)
    
    print("═" * 60)
    print(f"✅ Complete! Total tools: {merged_data['tool_count']}")
    print(f"📁 Saved to: {OUTPUT_FILE}")
    print("═" * 60)
