#!/usr/bin/env python3
"""
RFC-9011: Kali Linux Tools Complete Inventory Scraper

Scrapes all 600+ tools from kali.org/tools with:
- Tool name and package name
- Homepage URL
- Git repository (gitlab.com/kalilinux/packages/{tool}.git)
- Commands/binaries
- Categories
- Installed size
- Dependencies
"""

import json
import re
import time
import hashlib
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Set
from concurrent.futures import ThreadPoolExecutor, as_completed

try:
    import requests
    from bs4 import BeautifulSoup
except ImportError:
    print("ERROR: Required packages not found. Install with:")
    print("  pip install requests beautifulsoup4")
    exit(1)

OUTPUT_DIR = Path(__file__).parent / "output"
KALI_TOOLS_DIR = OUTPUT_DIR / "kali_tools"

# Kali.org tools listing
KALI_TOOLS_URL = "https://www.kali.org/tools/"
KALI_TOOL_BASE = "https://www.kali.org/tools/"
PKG_KALI_BASE = "https://pkg.kali.org/pkg/"
GITLAB_KALI_BASE = "https://gitlab.com/kalilinux/packages/"

# Rate limiting
REQUEST_DELAY = 0.5  # seconds between requests
MAX_WORKERS = 4


class KaliToolsScraper:
    """Comprehensive Kali Linux tools scraper."""

    def __init__(self, cache_dir: Path = KALI_TOOLS_DIR):
        self.cache_dir = cache_dir
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        self.session = requests.Session()
        self.session.headers.update({
            "User-Agent": "CTAS7-ThreatIntel-Scraper/1.0 (Security Research)"
        })

        self.tools: Dict[str, Dict] = {}
        self.categories: Dict[str, List[str]] = {}
        self.errors: List[str] = []

    def fetch_tools_listing(self) -> List[str]:
        """Fetch the main tools listing page to get all tool URLs."""
        print(f"[1/3] Fetching tools listing from {KALI_TOOLS_URL}...")

        try:
            response = self.session.get(KALI_TOOLS_URL, timeout=30)
            response.raise_for_status()

            soup = BeautifulSoup(response.text, 'html.parser')

            # Find all tool links - they're in card format on kali.org/tools/
            tool_slugs = set()

            # Look for links to individual tool pages
            for link in soup.find_all('a', href=True):
                href = link.get('href', '')
                # Match patterns like /tools/nmap/ or https://www.kali.org/tools/nmap/
                if '/tools/' in href and href != '/tools/':
                    # Extract tool slug
                    match = re.search(r'/tools/([a-zA-Z0-9_-]+)/?', href)
                    if match:
                        slug = match.group(1).lower()
                        # Filter out non-tool pages
                        if slug not in ['all', 'categories', 'page', '']:
                            tool_slugs.add(slug)

            print(f"  Found {len(tool_slugs)} unique tool slugs")
            return sorted(list(tool_slugs))

        except Exception as e:
            print(f"  ERROR fetching listing: {e}")
            return []

    def fetch_tool_details(self, tool_slug: str) -> Optional[Dict]:
        """Fetch detailed information for a single tool."""
        tool_url = f"{KALI_TOOL_BASE}{tool_slug}/"

        try:
            response = self.session.get(tool_url, timeout=20)
            if response.status_code == 404:
                return None
            response.raise_for_status()

            soup = BeautifulSoup(response.text, 'html.parser')

            tool_data = {
                "slug": tool_slug,
                "name": tool_slug,
                "url": tool_url,
                "package_name": tool_slug,
                "description": "",
                "homepage": "",
                "git_repo": f"{GITLAB_KALI_BASE}{tool_slug}.git",
                "categories": [],
                "commands": [],
                "installed_size": "",
                "version": "",
                "author": "",
                "scraped_at": datetime.now().isoformat(),
            }

            # Extract title/name
            title = soup.find('h1')
            if title:
                tool_data["name"] = title.get_text(strip=True)

            # Extract description from meta or first paragraph
            meta_desc = soup.find('meta', attrs={'name': 'description'})
            if meta_desc:
                tool_data["description"] = meta_desc.get('content', '')[:500]
            else:
                first_p = soup.find('p')
                if first_p:
                    tool_data["description"] = first_p.get_text(strip=True)[:500]

            # Look for homepage link
            for link in soup.find_all('a', href=True):
                link_text = link.get_text(strip=True).lower()
                if 'homepage' in link_text or 'official' in link_text or 'website' in link_text:
                    tool_data["homepage"] = link.get('href', '')
                    break

            # Extract categories from breadcrumbs or category links
            for link in soup.find_all('a', href=True):
                href = link.get('href', '')
                if '/tools/' in href and 'category' in href.lower():
                    cat = link.get_text(strip=True)
                    if cat and cat not in tool_data["categories"]:
                        tool_data["categories"].append(cat)

            # Look for category badges/tags
            for badge in soup.find_all(['span', 'div'], class_=re.compile(r'badge|tag|category', re.I)):
                cat = badge.get_text(strip=True)
                if cat and len(cat) < 50 and cat not in tool_data["categories"]:
                    tool_data["categories"].append(cat)

            # Extract commands from code blocks or command listings
            for code in soup.find_all(['code', 'pre']):
                cmd_text = code.get_text(strip=True)
                # Look for command-like patterns
                if cmd_text and len(cmd_text) < 100:
                    if cmd_text.startswith(tool_slug) or cmd_text.startswith('./'):
                        if cmd_text not in tool_data["commands"]:
                            tool_data["commands"].append(cmd_text)

            # Try to fetch package info from pkg.kali.org
            pkg_info = self._fetch_pkg_info(tool_slug)
            if pkg_info:
                tool_data.update(pkg_info)

            return tool_data

        except Exception as e:
            self.errors.append(f"{tool_slug}: {str(e)}")
            return None

    def _fetch_pkg_info(self, tool_slug: str) -> Optional[Dict]:
        """Fetch package info from pkg.kali.org for additional metadata."""
        pkg_url = f"{PKG_KALI_BASE}{tool_slug}"

        try:
            time.sleep(REQUEST_DELAY / 2)  # Extra delay for pkg.kali.org
            response = self.session.get(pkg_url, timeout=15)
            if response.status_code != 200:
                return None

            soup = BeautifulSoup(response.text, 'html.parser')

            info = {}

            # Extract version
            version_elem = soup.find(text=re.compile(r'Version:', re.I))
            if version_elem:
                parent = version_elem.parent
                if parent:
                    info["version"] = parent.get_text(strip=True).replace('Version:', '').strip()

            # Extract installed size
            size_elem = soup.find(text=re.compile(r'Installed-Size:', re.I))
            if size_elem:
                parent = size_elem.parent
                if parent:
                    info["installed_size"] = parent.get_text(strip=True).replace('Installed-Size:', '').strip()

            # Extract maintainer
            maint_elem = soup.find(text=re.compile(r'Maintainer:', re.I))
            if maint_elem:
                parent = maint_elem.parent
                if parent:
                    info["author"] = parent.get_text(strip=True).replace('Maintainer:', '').strip()[:100]

            # Confirm git repo exists
            for link in soup.find_all('a', href=True):
                href = link.get('href', '')
                if 'gitlab.com/kalilinux/packages' in href:
                    info["git_repo"] = href
                    break

            return info

        except Exception:
            return None

    def scrape_all_tools(self, limit: Optional[int] = None) -> int:
        """Scrape all Kali tools with parallel fetching."""
        print("\n" + "=" * 70)
        print("RFC-9011: Kali Linux Tools Complete Inventory")
        print("=" * 70)

        # Get tool listing
        tool_slugs = self.fetch_tools_listing()

        if not tool_slugs:
            print("  No tools found! Trying alternative approach...")
            tool_slugs = self._get_known_tools()

        if limit:
            tool_slugs = tool_slugs[:limit]

        print(f"\n[2/3] Scraping {len(tool_slugs)} tools...")

        # Process tools with rate limiting
        count = 0

        with ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
            future_to_slug = {
                executor.submit(self._fetch_with_delay, slug, i): slug
                for i, slug in enumerate(tool_slugs)
            }

            for future in as_completed(future_to_slug):
                slug = future_to_slug[future]
                try:
                    tool_data = future.result()
                    if tool_data:
                        self.tools[slug] = tool_data
                        # Track categories
                        for cat in tool_data.get("categories", []):
                            if cat not in self.categories:
                                self.categories[cat] = []
                            self.categories[cat].append(slug)
                        count += 1

                        if count % 50 == 0:
                            print(f"    Processed {count}/{len(tool_slugs)} tools...")

                except Exception as e:
                    self.errors.append(f"{slug}: {str(e)}")

        print(f"\n  Successfully scraped {count} tools")
        if self.errors:
            print(f"  Errors: {len(self.errors)}")

        return count

    def _fetch_with_delay(self, slug: str, index: int) -> Optional[Dict]:
        """Fetch tool with rate limiting delay."""
        time.sleep(REQUEST_DELAY * (index % MAX_WORKERS))
        return self.fetch_tool_details(slug)

    def _get_known_tools(self) -> List[str]:
        """Fallback: Return known Kali tool slugs from ctas7-exploit-arsenal."""
        # Comprehensive list from multiple sources
        return [
            # Network Reconnaissance
            "nmap", "masscan", "netdiscover", "arp-scan", "unicornscan", "zmap",
            "p0f", "netcat", "socat", "proxychains4", "proxychains-ng",

            # Web Application Testing
            "nikto", "sqlmap", "gobuster", "dirb", "dirbuster", "wfuzz", "ffuf",
            "burpsuite", "zaproxy", "whatweb", "wpscan", "joomscan", "droopescan",
            "nuclei", "httpx", "subfinder", "amass", "sublist3r", "assetfinder",
            "aquatone", "eyewitness", "gowitness", "webanalyze", "arjun",
            "paramspider", "gau", "waybackurls", "hakrawler", "gospider",
            "katana", "feroxbuster", "rustbuster", "dirsearch",

            # Exploitation Frameworks
            "metasploit-framework", "armitage", "beef-xss", "set",
            "exploitdb", "searchsploit", "msfpc", "veil", "shellter",
            "empire", "starkiller", "covenant", "sliver", "havoc",
            "cobalt-strike", "pupy", "merlin", "villain",

            # Password Attacks
            "hydra", "hashcat", "john", "medusa", "ncrack", "ophcrack",
            "cewl", "crunch", "cupp", "mentalist", "hashid", "hash-identifier",
            "haiti", "name-that-hash", "patator", "thc-pptp-bruter",
            "crowbar", "spray", "kerbrute", "crackmapexec",

            # Wireless Attacks
            "aircrack-ng", "wifite", "reaver", "fern-wifi-cracker", "kismet",
            "bettercap", "mdk3", "mdk4", "pixiewps", "cowpatty", "asleap",
            "eaphammer", "hostapd-wpe", "wifiphisher", "fluxion",
            "airgeddon", "wifi-honey", "wifi-pumpkin3",

            # Bluetooth
            "bluez", "bluesnarfer", "spooftooph", "btscanner", "redfang",
            "ubertooth", "bettercap", "blueborne-scanner",

            # OSINT/Information Gathering
            "theharvester", "recon-ng", "maltego", "spiderfoot", "shodan",
            "metagoofil", "exiftool", "foca", "creepy", "sherlock",
            "social-analyzer", "twint", "photon", "infoga", "osrframework",
            "h8mail", "pwnedornot", "ghunt", "phoneinfoga",

            # Forensics
            "autopsy", "binwalk", "volatility", "volatility3", "sleuthkit",
            "foremost", "scalpel", "testdisk", "photorec", "bulk-extractor",
            "dc3dd", "dcfldd", "guymager", "rdd", "afflib-tools",
            "plaso", "log2timeline", "regripper", "rekall",

            # Reverse Engineering
            "ghidra", "radare2", "gdb", "objdump", "ltrace", "strace",
            "edb-debugger", "ollydbg", "x64dbg", "apktool", "dex2jar",
            "jadx", "jd-gui", "bytecode-viewer", "hopper", "cutter",
            "iaito", "rizin", "r2ghidra",

            # Sniffing/Spoofing
            "wireshark", "tcpdump", "ettercap", "bettercap", "dsniff",
            "arpspoof", "dnsspoof", "macchanger", "mitmproxy", "responder",
            "impacket", "yersinia", "lans", "netsniff-ng", "driftnet",

            # Vulnerability Analysis
            "nessus", "openvas", "nikto", "lynis", "wapiti", "arachni",
            "vuls", "trivy", "grype", "syft", "retire-js", "npm-audit",
            "snyk", "dependency-check", "safety",

            # Post-Exploitation
            "empire", "covenant", "sliver", "bloodhound", "mimikatz",
            "lazagne", "crackmapexec", "evil-winrm", "powersploit",
            "sharphound", "rubeus", "seatbelt", "sharpup", "certify",
            "kerbrute", "kerberoast", "secretsdump", "lsassy",

            # Social Engineering
            "gophish", "king-phisher", "evilginx2", "modlishka", "muraena",
            "socialfish", "blackeye", "shellphish", "zphisher",

            # Stress Testing
            "hping3", "slowloris", "goldeneye", "hulk", "siege", "ab",
            "wrk", "hey", "vegeta", "locust",

            # Reporting
            "dradis", "faraday", "serpico", "lair", "plextrac",
            "pipal", "magictree", "metagoofil", "cutycapt",

            # Hardware Hacking
            "arduino", "flashrom", "openocd", "urjtag", "avrdude",

            # Android
            "apktool", "dex2jar", "jadx", "androguard", "frida",
            "objection", "drozer", "mobsf", "qark", "androwarn",

            # Database
            "sqlmap", "sqlninja", "bbqsql", "jsql-injection", "nosqlmap",
            "mongoaudit", "redis-cli", "pgcli", "mycli",

            # Crypto
            "hashcat", "john", "ciphey", "rsactftool", "featherduster",
            "xortool", "pkcrack", "fcrackzip",

            # Cloud
            "pacu", "cloudsploit", "prowler", "scoutsuite", "steampipe",
            "cloudmapper", "cartography", "cloudfox", "enumerate-iam",

            # Container Security
            "trivy", "grype", "syft", "dive", "hadolint", "dockle",
            "kube-hunter", "kube-bench", "kubeaudit", "popeye",

            # Misc Tools
            "tmux", "screen", "vim", "nano", "curl", "wget", "git",
            "python3", "ruby", "perl", "go", "nodejs", "jq", "yq",
            "fzf", "ripgrep", "bat", "exa", "fd-find", "httpie",

            # Additional Tools from Kali
            "wordlists", "seclists", "payloadsallthethings", "fuzzdb",
            "rockyou", "dirbuster-lists", "wfuzz-wordlists",
        ]

    def save_inventory(self):
        """Save the complete inventory to JSON files."""
        print("\n[3/3] Saving inventory...")

        # Main inventory file
        inventory = {
            "scraped_at": datetime.now().isoformat(),
            "source": "kali.org/tools",
            "tool_count": len(self.tools),
            "category_count": len(self.categories),
            "tools": self.tools,
        }

        inventory_file = self.cache_dir / "kali_tools_complete.json"
        with open(inventory_file, 'w') as f:
            json.dump(inventory, f, indent=2)
        print(f"  Saved: {inventory_file}")

        # Categories index
        categories_file = self.cache_dir / "kali_categories.json"
        with open(categories_file, 'w') as f:
            json.dump({
                "categories": self.categories,
                "category_counts": {k: len(v) for k, v in self.categories.items()}
            }, f, indent=2)
        print(f"  Saved: {categories_file}")

        # Simple tool list for quick reference
        tool_list = sorted(self.tools.keys())
        list_file = self.cache_dir / "kali_tool_list.txt"
        with open(list_file, 'w') as f:
            f.write(f"# Kali Linux Tools Inventory\n")
            f.write(f"# Scraped: {datetime.now().isoformat()}\n")
            f.write(f"# Count: {len(tool_list)}\n\n")
            for tool in tool_list:
                f.write(f"{tool}\n")
        print(f"  Saved: {list_file}")

        # Git repos list for cloning
        repos_file = self.cache_dir / "kali_git_repos.txt"
        with open(repos_file, 'w') as f:
            f.write("# Kali Linux Tool Git Repositories\n")
            f.write("# Clone with: git clone <url>\n\n")
            for slug, data in sorted(self.tools.items()):
                repo = data.get("git_repo", "")
                if repo:
                    f.write(f"{repo}\n")
        print(f"  Saved: {repos_file}")

        # Error log
        if self.errors:
            error_file = self.cache_dir / "scraper_errors.log"
            with open(error_file, 'w') as f:
                f.write(f"# Scraper Errors - {datetime.now().isoformat()}\n\n")
                for error in self.errors:
                    f.write(f"{error}\n")
            print(f"  Errors logged: {error_file}")

        # Summary
        print(f"\n  SUMMARY:")
        print(f"    Tools scraped: {len(self.tools)}")
        print(f"    Categories: {len(self.categories)}")
        print(f"    Errors: {len(self.errors)}")

        return len(self.tools)

    def get_tools_by_category(self, category: str) -> List[Dict]:
        """Get all tools in a specific category."""
        tools = []
        for slug in self.categories.get(category, []):
            if slug in self.tools:
                tools.append(self.tools[slug])
        return tools

    def search_tools(self, query: str) -> List[Dict]:
        """Search tools by name or description."""
        query = query.lower()
        results = []
        for slug, data in self.tools.items():
            if (query in slug.lower() or
                query in data.get("name", "").lower() or
                query in data.get("description", "").lower()):
                results.append(data)
        return results


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Kali Linux Tools Scraper")
    parser.add_argument("--limit", type=int, help="Limit number of tools to scrape")
    parser.add_argument("--search", type=str, help="Search tools")
    parser.add_argument("--category", type=str, help="List tools in category")
    parser.add_argument("--fallback", action="store_true",
                       help="Use known tools list instead of scraping")
    args = parser.parse_args()

    scraper = KaliToolsScraper()

    if args.fallback:
        # Use known tools list
        print("Using fallback known tools list...")
        known_tools = scraper._get_known_tools()
        for tool in known_tools:
            scraper.tools[tool] = {
                "slug": tool,
                "name": tool,
                "url": f"{KALI_TOOL_BASE}{tool}/",
                "package_name": tool,
                "git_repo": f"{GITLAB_KALI_BASE}{tool}.git",
                "categories": [],
                "commands": [],
                "scraped_at": datetime.now().isoformat(),
            }
        scraper.save_inventory()
    elif args.search:
        # Load existing inventory and search
        inventory_file = KALI_TOOLS_DIR / "kali_tools_complete.json"
        if inventory_file.exists():
            with open(inventory_file) as f:
                data = json.load(f)
                scraper.tools = data.get("tools", {})

            results = scraper.search_tools(args.search)
            print(f"Found {len(results)} tools matching '{args.search}':")
            for tool in results:
                print(f"  - {tool['name']}: {tool.get('description', '')[:60]}...")
        else:
            print("No inventory found. Run scraper first.")
    elif args.category:
        # List tools by category
        inventory_file = KALI_TOOLS_DIR / "kali_tools_complete.json"
        if inventory_file.exists():
            with open(inventory_file) as f:
                data = json.load(f)
                scraper.tools = data.get("tools", {})

            # Build categories
            for slug, tool in scraper.tools.items():
                for cat in tool.get("categories", []):
                    if cat not in scraper.categories:
                        scraper.categories[cat] = []
                    scraper.categories[cat].append(slug)

            tools = scraper.get_tools_by_category(args.category)
            print(f"Tools in category '{args.category}': {len(tools)}")
            for tool in tools:
                print(f"  - {tool['name']}")
        else:
            print("No inventory found. Run scraper first.")
    else:
        # Full scrape
        scraper.scrape_all_tools(limit=args.limit)
        scraper.save_inventory()


if __name__ == "__main__":
    main()
