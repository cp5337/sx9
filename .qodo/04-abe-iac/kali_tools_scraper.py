#!/usr/bin/env python3
"""
Kali Tools Inventory Scraper
RFC-9011 Phase 1: Acquisition Layer

Scrapes Kali.org tools index and pkg.kali.org for comprehensive tool metadata.
Output feeds into neo4j_threat_loader.py for graph population.
"""

import requests
from bs4 import BeautifulSoup
import json
from pathlib import Path
from typing import List, Dict, Any

# --- Configuration ---
KALI_TOOLS_INDEX_URL = "https://www.kali.org/tools/"
PKG_KALI_ORG_BASE = "https://pkg.kali.org/pkg/"
OUTPUT_FILE = Path("output/threat_content/kali_tools_inventory.json")


def fetch_package_metadata(package_name: str) -> Dict[str, Any]:
    """
    Fetches dependency, repo, and version info from pkg.kali.org.
    
    Args:
        package_name: Kali package name (e.g., 'nmap')
        
    Returns:
        Dict with git_repo, dependencies, version
    """
    url = PKG_KALI_ORG_BASE + package_name
    metadata = {"git_repo": None, "dependencies": [], "version": None}
    
    try:
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        soup = BeautifulSoup(response.content, 'html.parser')
        
        # Extract Git Repository URL
        repo_link = soup.find('a', href=lambda href: href and 'gitlab.com/kalilinux/packages' in href)
        if repo_link:
            metadata['git_repo'] = repo_link['href']

        # Extract Version
        version_tag = soup.find('a', href=lambda href: 'changelog' in href)
        if version_tag:
            metadata['version'] = version_tag.text.split()[0]
        
        # Extract Dependencies (Simplified lookup)
        dependency_list = soup.find('div', id='pkginfo-dependencies')
        if dependency_list:
            metadata['dependencies'] = [li.text.strip() for li in dependency_list.find_all('li')]
            
    except Exception as e:
        # Silently continue on metadata fetch failures
        pass

    return metadata


def scrape_kali_tools_complete() -> List[Dict[str, Any]]:
    """
    Scrapes the Kali index page and drills down for details.
    
    Fulfills Layer 1: Acquisition goal for Tool nodes in the knowledge graph.
    
    Returns:
        List of tool records with metadata
    """
    print("PHASE 1: Starting comprehensive Kali.org tool scrape...")
    
    tools_data: List[Dict[str, Any]] = []
    
    try:
        index_response = requests.get(KALI_TOOLS_INDEX_URL, timeout=30)
        index_response.raise_for_status()
        index_soup = BeautifulSoup(index_response.content, 'html.parser')
        
        # Find all tool links from index page
        tool_links = index_soup.select('a[href^="/tools/"][class="list-group-item"]')
        
        for link in tool_links:
            # package_name is the path segment, e.g., 'nmap'
            package_name = link['href'].strip('/').split('/')[-1]
            if not package_name:
                continue
            
            # Extract basic details from the list item
            title_elem = link.find('h4')
            title = title_elem.text.strip() if title_elem else package_name
            categories = [c.text.strip() for c in link.find_all('span', class_='badge')]
            
            # Fetch comprehensive package metadata from pkg.kali.org
            pkg_metadata = fetch_package_metadata(package_name)

            tool_record = {
                "name": title,
                "package_name": package_name,
                "commands": [package_name],  # Placeholder, commands need detailed scrape
                "categories": categories,
                "homepage": f"https://www.kali.org/tools/{package_name}/",
                "kali_gitlab": pkg_metadata['git_repo'],
                "version": pkg_metadata['version'],
                "dependencies": pkg_metadata['dependencies'],
            }
            tools_data.append(tool_record)
            
            if len(tools_data) % 50 == 0:
                print(f"   -> Scraped {len(tools_data)} tools so far...")

    except requests.exceptions.RequestException as e:
        print(f"CRITICAL ERROR during scraping: {e}")
        return []

    print(f"PHASE 1: Scrape complete. Found {len(tools_data)} tools.")
    return tools_data


if __name__ == '__main__':
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    
    # Execute Phase 1: Acquisition
    inventory = scrape_kali_tools_complete()
    
    with open(OUTPUT_FILE, 'w') as f:
        json.dump(inventory, f, indent=2)
    
    print(f"Inventory saved to {OUTPUT_FILE}")
    print(f"\nNext step: Run neo4j_threat_loader.py --load-tools")
