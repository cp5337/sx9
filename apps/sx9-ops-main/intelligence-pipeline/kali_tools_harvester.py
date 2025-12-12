#!/usr/bin/env python3
"""
Kali Tools Web Scraper - Core Module
Harvests tool metadata from https://www.kali.org/tools/
"""

import asyncio
import aiohttp
import json
import csv
from pathlib import Path
from typing import Dict, List, Optional, Set
from dataclasses import dataclass, asdict
from bs4 import BeautifulSoup
import re
import logging
from urllib.parse import urljoin

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class KaliTool:
    """Represents a single Kali tool with metadata"""
    name: str
    url: str
    description: str
    category: str
    homepage: Optional[str] = None
    repository: Optional[str] = None
    author: Optional[str] = None
    license: Optional[str] = None
    ctas_primitives: List[str] = None
    
    def __post_init__(self):
        if self.ctas_primitives is None:
            self.ctas_primitives = []

class KaliWebScraper:
    """Web scraper for Kali tools information"""
    
    def __init__(self, output_dir: str = "./kali_tools_data", delay: float = 1.0):
        self.base_url = "https://www.kali.org"
        self.tools_url = "https://www.kali.org/tools/"
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        self.delay = delay
        self.session: Optional[aiohttp.ClientSession] = None
        self.tools: List[KaliTool] = []
        
        # CTAS primitive mapping keywords
        self.primitive_keywords = {
            "SENSE": ["scan", "enum", "recon", "discover", "probe", "sniff", "monitor", 
                     "capture", "gather", "collect", "fingerprint", "detect", "search",
                     "osint", "intelligence", "surveillance", "reconnaissance"],
            "ACT": ["exploit", "attack", "brute", "crack", "inject", "payload", "shell",
                   "penetrate", "bypass", "escalate", "execute", "weaponize", "strike"],
            "ENCODE": ["encrypt", "decode", "encode", "hash", "crypto", "cipher", 
                      "steganography", "obfuscate", "compress", "convert", "transform"],
            "ANALYZE": ["analyze", "forensic", "reverse", "disassemble", "debug", "trace",
                       "examine", "investigate", "parse", "extract", "correlate"],
            "ORCHESTRATE": ["framework", "suite", "automation", "orchestrate", "manage", 
                           "control", "coordinate", "workflow", "campaign", "multi"],
            "MONITOR": ["monitor", "watch", "track", "log", "audit", "observe", 
                       "continuous", "real-time", "surveillance", "alerting"]
        }
    
    async def __aenter__(self):
        """Async context manager entry"""
        self.session = aiohttp.ClientSession(
            timeout=aiohttp.ClientTimeout(total=30),
            headers={'User-Agent': 'Mozilla/5.0 (compatible; CTAS-Research-Bot/1.0)'}
        )
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit"""
        if self.session:
            await self.session.close()
    
    async def get_page(self, url: str) -> Optional[str]:
        """Fetch a single page with error handling and delays"""
        try:
            await asyncio.sleep(self.delay)
            async with self.session.get(url) as response:
                if response.status == 200:
                    content = await response.text()
                    logger.info(f"Fetched: {url}")
                    return content
                else:
                    logger.warning(f"HTTP {response.status} for {url}")
                    return None
        except Exception as e:
            logger.error(f"Error fetching {url}: {e}")
            return None
    
    def parse_tools_index(self, html: str) -> List[str]:
        """Parse main tools index page to get tool URLs"""
        soup = BeautifulSoup(html, 'html.parser')
        tool_urls = []
        
        # Look for tool links
        tool_links = soup.find_all('a', href=re.compile(r'/tools/[^/]+/?$'))
        
        for link in tool_links:
            href = link.get('href')
            if href:
                full_url = urljoin(self.base_url, href)
                tool_urls.append(full_url)
        
        logger.info(f"Found {len(tool_urls)} tool URLs")
        return tool_urls
    
    def parse_tool_page(self, html: str, url: str) -> Optional[KaliTool]:
        """Parse individual tool page to extract metadata"""
        soup = BeautifulSoup(html, 'html.parser')
        
        try:
            # Extract tool name
            name_elem = soup.find('h1') or soup.find('title')
            name = name_elem.get_text().strip() if name_elem else url.split('/')[-2]
            name = re.sub(r'\s*-\s*Kali.*', '', name, flags=re.IGNORECASE)
            
            # Extract description
            description = ""
            meta_desc = soup.find('meta', attrs={'name': 'description'})
            if meta_desc:
                description = meta_desc.get('content', '').strip()
            else:
                first_p = soup.find('p')
                if first_p:
                    description = first_p.get_text().strip()
            
            # Extract category from breadcrumbs
            category = "general"
            breadcrumbs = soup.find_all(['nav', 'ol'], class_=re.compile(r'breadcrumb', re.I))
            if breadcrumbs:
                for breadcrumb in breadcrumbs:
                    links = breadcrumb.find_all('a')
                    if len(links) >= 2:
                        category = links[-2].get_text().strip().lower()
                        break
            
            # Extract metadata
            metadata = self.extract_metadata(soup)
            
            # Map to CTAS primitives
            ctas_primitives = self.map_to_ctas_primitives(name, description, category)
            
            tool = KaliTool(
                name=name,
                url=url,
                description=description,
                category=category,
                homepage=metadata.get('homepage'),
                repository=metadata.get('repository'),
                author=metadata.get('author'),
                license=metadata.get('license'),
                ctas_primitives=ctas_primitives
            )
            
            logger.info(f"Parsed tool: {name} -> {ctas_primitives}")
            return tool
            
        except Exception as e:
            logger.error(f"Error parsing tool page {url}: {e}")
            return None
    
    def extract_metadata(self, soup: BeautifulSoup) -> Dict[str, str]:
        """Extract metadata from page structures"""
        metadata = {}
        
        # Look for definition lists
        dl_elements = soup.find_all('dl')
        for dl in dl_elements:
            terms = dl.find_all('dt')
            definitions = dl.find_all('dd')
            
            for term, definition in zip(terms, definitions):
                key = term.get_text().strip().lower()
                value = definition.get_text().strip()
                
                if 'homepage' in key or 'website' in key:
                    metadata['homepage'] = value
                elif 'repository' in key or 'repo' in key or 'git' in key:
                    metadata['repository'] = value
                elif 'author' in key or 'maintainer' in key:
                    metadata['author'] = value
                elif 'license' in key:
                    metadata['license'] = value
        
        # Look for GitHub/GitLab links
        links = soup.find_all('a', href=True)
        for link in links:
            href = link['href']
            if 'github.com' in href or 'gitlab.com' in href:
                metadata['repository'] = href
        
        return metadata
    
    def map_to_ctas_primitives(self, name: str, description: str, category: str) -> List[str]:
        """Map tool to CTAS primitives based on keywords"""
        text_to_analyze = f"{name} {description} {category}".lower()
        mapped_primitives = []
        
        for primitive, keywords in self.primitive_keywords.items():
            if any(keyword in text_to_analyze for keyword in keywords):
                mapped_primitives.append(primitive)
        
        # Default to SENSE if no mapping found
        if not mapped_primitives:
            mapped_primitives = ["SENSE"]
        
        return mapped_primitives
    
    async def harvest_all_tools(self):
        """Main method to harvest all tools"""
        logger.info("Starting Kali tools harvest...")
        
        # Get tools index page
        html = await self.get_page(self.tools_url)
        if not html:
            logger.error("Failed to fetch tools index page")
            return
        
        # Parse tool URLs
        tool_urls = self.parse_tools_index(html)
        
        # Process each tool
        for i, url in enumerate(tool_urls, 1):
            logger.info(f"Processing tool {i}/{len(tool_urls)}")
            
            html = await self.get_page(url)
            if not html:
                continue
            
            tool = self.parse_tool_page(html, url)
            if tool:
                self.tools.append(tool)
        
        logger.info(f"Harvest complete! Collected {len(self.tools)} tools")
    
    def save_data(self):
        """Save harvested data to files"""
        # Save as JSON
        json_file = self.output_dir / "kali_tools.json"
        with open(json_file, 'w', encoding='utf-8') as f:
            json.dump([asdict(tool) for tool in self.tools], f, indent=2)
        
        # Save as CSV
        csv_file = self.output_dir / "kali_tools.csv"
        if self.tools:
            with open(csv_file, 'w', newline='', encoding='utf-8') as f:
                fieldnames = list(asdict(self.tools[0]).keys())
                writer = csv.DictWriter(f, fieldnames=fieldnames)
                writer.writeheader()
                for tool in self.tools:
                    row = asdict(tool)
                    # Convert lists to strings for CSV
                    for key, value in row.items():
                        if isinstance(value, list):
                            row[key] = '; '.join(value) if value else ''
                    writer.writerow(row)
        
        logger.info(f"Data saved to {self.output_dir}")

async def main():
    """Main execution function"""
    scraper = KaliWebScraper()
    
    async with scraper:
        await scraper.harvest_all_tools()
        scraper.save_data()

if __name__ == "__main__":
    asyncio.run(main())
