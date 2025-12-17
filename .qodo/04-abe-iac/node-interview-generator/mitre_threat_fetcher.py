#!/usr/bin/env python3
"""
MITRE ATT&CK and Threat Intelligence Fetcher
Pulls real MITRE data for interview validation and enrichment
"""

import json
import requests
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Set
import re

OUTPUT_DIR = Path(__file__).parent / "output"
MITRE_DIR = OUTPUT_DIR / "mitre_data"

# MITRE ATT&CK Enterprise STIX data URL
MITRE_ENTERPRISE_URL = "https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json"
MITRE_GROUPS_URL = "https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json"


class MITREDataFetcher:
    """Fetches and caches MITRE ATT&CK data."""

    def __init__(self, cache_dir: Path = MITRE_DIR):
        self.cache_dir = cache_dir
        self.cache_dir.mkdir(parents=True, exist_ok=True)

        self.techniques: Dict[str, dict] = {}
        self.tactics: Dict[str, dict] = {}
        self.groups: Dict[str, dict] = {}
        self.software: Dict[str, dict] = {}
        self.technique_to_tactics: Dict[str, List[str]] = {}
        self.group_to_techniques: Dict[str, List[str]] = {}

    def fetch_enterprise_attack(self, force_refresh: bool = False) -> bool:
        """Fetch MITRE ATT&CK Enterprise data."""
        cache_file = self.cache_dir / "enterprise_attack.json"

        # Use cache if fresh (less than 24 hours old)
        if cache_file.exists() and not force_refresh:
            age_hours = (datetime.now().timestamp() - cache_file.stat().st_mtime) / 3600
            if age_hours < 24:
                print(f"Using cached MITRE data ({age_hours:.1f} hours old)")
                with open(cache_file, 'r') as f:
                    data = json.load(f)
                self._parse_stix_data(data)
                return True

        print("Fetching fresh MITRE ATT&CK data...")
        try:
            response = requests.get(MITRE_ENTERPRISE_URL, timeout=60)
            response.raise_for_status()
            data = response.json()

            # Cache the data
            with open(cache_file, 'w') as f:
                json.dump(data, f)

            self._parse_stix_data(data)
            print(f"Loaded {len(self.techniques)} techniques, {len(self.groups)} groups")
            return True

        except Exception as e:
            print(f"ERROR fetching MITRE data: {e}")
            return False

    def _parse_stix_data(self, data: dict):
        """Parse STIX 2.0 bundle into usable structures."""
        objects = data.get("objects", [])

        for obj in objects:
            obj_type = obj.get("type")

            if obj_type == "attack-pattern":
                # Techniques
                external_refs = obj.get("external_references", [])
                for ref in external_refs:
                    if ref.get("source_name") == "mitre-attack":
                        tech_id = ref.get("external_id")
                        if tech_id:
                            self.techniques[tech_id] = {
                                "id": tech_id,
                                "name": obj.get("name"),
                                "description": obj.get("description", "")[:500],
                                "platforms": obj.get("x_mitre_platforms", []),
                                "tactics": [p.get("phase_name") for p in obj.get("kill_chain_phases", [])],
                                "is_subtechnique": "." in tech_id,
                                "detection": obj.get("x_mitre_detection", "")[:300],
                            }
                            self.technique_to_tactics[tech_id] = self.techniques[tech_id]["tactics"]

            elif obj_type == "intrusion-set":
                # APT Groups
                external_refs = obj.get("external_references", [])
                for ref in external_refs:
                    if ref.get("source_name") == "mitre-attack":
                        group_id = ref.get("external_id")
                        if group_id:
                            aliases = obj.get("aliases", [])
                            self.groups[group_id] = {
                                "id": group_id,
                                "name": obj.get("name"),
                                "aliases": aliases,
                                "description": obj.get("description", "")[:500],
                            }

            elif obj_type == "malware" or obj_type == "tool":
                # Software/Tools
                external_refs = obj.get("external_references", [])
                for ref in external_refs:
                    if ref.get("source_name") == "mitre-attack":
                        soft_id = ref.get("external_id")
                        if soft_id:
                            self.software[soft_id] = {
                                "id": soft_id,
                                "name": obj.get("name"),
                                "type": obj_type,
                                "description": obj.get("description", "")[:300],
                            }

            elif obj_type == "x-mitre-tactic":
                # Tactics
                external_refs = obj.get("external_references", [])
                for ref in external_refs:
                    if ref.get("source_name") == "mitre-attack":
                        tactic_id = ref.get("external_id")
                        if tactic_id:
                            self.tactics[tactic_id] = {
                                "id": tactic_id,
                                "name": obj.get("name"),
                                "shortname": obj.get("x_mitre_shortname"),
                            }

        # Build group-to-techniques mapping from relationships
        for obj in objects:
            if obj.get("type") == "relationship" and obj.get("relationship_type") == "uses":
                source_ref = obj.get("source_ref", "")
                target_ref = obj.get("target_ref", "")

                # Find group using technique
                if "intrusion-set" in source_ref and "attack-pattern" in target_ref:
                    # Need to resolve refs - for now just note the relationship exists
                    pass

    def validate_technique_id(self, tech_id: str) -> bool:
        """Check if a technique ID is valid."""
        return tech_id in self.techniques

    def validate_technique_ids(self, tech_ids: List[str]) -> Dict[str, bool]:
        """Validate multiple technique IDs."""
        return {tid: self.validate_technique_id(tid) for tid in tech_ids}

    def get_technique_info(self, tech_id: str) -> Optional[dict]:
        """Get detailed info for a technique."""
        return self.techniques.get(tech_id)

    def get_techniques_for_tactic(self, tactic: str) -> List[str]:
        """Get all techniques for a given tactic."""
        tactic_lower = tactic.lower().replace(" ", "-")
        return [
            tid for tid, info in self.techniques.items()
            if tactic_lower in [t.lower() for t in info.get("tactics", [])]
        ]

    def suggest_techniques(self, keywords: List[str], limit: int = 10) -> List[dict]:
        """Suggest techniques based on keywords."""
        scores = {}
        keywords_lower = [k.lower() for k in keywords]

        for tech_id, info in self.techniques.items():
            score = 0
            name_lower = info["name"].lower()
            desc_lower = info.get("description", "").lower()

            for kw in keywords_lower:
                if kw in name_lower:
                    score += 3
                if kw in desc_lower:
                    score += 1

            if score > 0:
                scores[tech_id] = score

        # Sort by score descending
        sorted_techs = sorted(scores.items(), key=lambda x: x[1], reverse=True)[:limit]
        return [{"id": tid, "score": score, **self.techniques[tid]} for tid, score in sorted_techs]

    def get_group_info(self, group_name: str) -> Optional[dict]:
        """Get group info by name or alias."""
        group_name_lower = group_name.lower()

        for gid, info in self.groups.items():
            if group_name_lower == info["name"].lower():
                return info
            if group_name_lower in [a.lower() for a in info.get("aliases", [])]:
                return info

        return None

    def export_summary(self) -> dict:
        """Export summary for interview enrichment."""
        return {
            "fetched_at": datetime.now().isoformat(),
            "technique_count": len(self.techniques),
            "group_count": len(self.groups),
            "tactic_count": len(self.tactics),
            "software_count": len(self.software),
            "techniques": {k: {"name": v["name"], "tactics": v["tactics"]}
                          for k, v in self.techniques.items()},
            "groups": {k: {"name": v["name"], "aliases": v["aliases"]}
                      for k, v in self.groups.items()},
        }

    def save_lookup_tables(self):
        """Save lookup tables for quick reference."""
        # Technique lookup
        tech_lookup = {
            tid: {
                "name": info["name"],
                "tactics": info["tactics"],
                "platforms": info["platforms"],
            }
            for tid, info in self.techniques.items()
        }
        with open(self.cache_dir / "technique_lookup.json", 'w') as f:
            json.dump(tech_lookup, f, indent=2)

        # Group lookup (by alias)
        group_lookup = {}
        for gid, info in self.groups.items():
            group_lookup[info["name"].lower()] = gid
            for alias in info.get("aliases", []):
                group_lookup[alias.lower()] = gid

        with open(self.cache_dir / "group_lookup.json", 'w') as f:
            json.dump(group_lookup, f, indent=2)

        print(f"Saved lookup tables to {self.cache_dir}")


def validate_interview_mitre(interview: dict, fetcher: MITREDataFetcher) -> dict:
    """Validate and enrich MITRE data in an interview."""
    results = {
        "valid_techniques": [],
        "invalid_techniques": [],
        "suggested_techniques": [],
        "valid_groups": [],
        "unknown_groups": [],
    }

    # Validate techniques
    mitre_techs = interview.get("mitre_techniques", [])
    for tech in mitre_techs:
        if fetcher.validate_technique_id(tech):
            info = fetcher.get_technique_info(tech)
            results["valid_techniques"].append({
                "id": tech,
                "name": info["name"],
                "tactics": info["tactics"]
            })
        else:
            results["invalid_techniques"].append(tech)

    # Validate APT examples
    apt_examples = interview.get("apt_examples", [])
    for apt in apt_examples:
        apt_name = apt.get("apt", "")
        group_info = fetcher.get_group_info(apt_name)
        if group_info:
            results["valid_groups"].append({
                "provided": apt_name,
                "mitre_id": group_info["id"],
                "official_name": group_info["name"]
            })
        else:
            results["unknown_groups"].append(apt_name)

    # Suggest additional techniques based on keywords
    keywords = interview.get("search", {}).get("keywords", [])
    if keywords:
        suggestions = fetcher.suggest_techniques(keywords, limit=5)
        # Filter out already included techniques
        existing = set(mitre_techs)
        results["suggested_techniques"] = [
            s for s in suggestions if s["id"] not in existing
        ]

    return results


def enrich_interviews_batch(interviews_dir: Path, fetcher: MITREDataFetcher) -> dict:
    """Validate all interviews in a directory."""
    results = {
        "total": 0,
        "validated": 0,
        "errors": 0,
        "invalid_techniques_found": [],
        "all_valid_techniques": set(),
    }

    for interview_file in interviews_dir.glob("*.json"):
        results["total"] += 1
        try:
            with open(interview_file, 'r') as f:
                interview = json.load(f)

            validation = validate_interview_mitre(interview, fetcher)

            # Track invalid techniques
            if validation["invalid_techniques"]:
                results["invalid_techniques_found"].append({
                    "file": interview_file.name,
                    "invalid": validation["invalid_techniques"]
                })

            # Track all valid techniques
            for tech in validation["valid_techniques"]:
                results["all_valid_techniques"].add(tech["id"])

            results["validated"] += 1

        except Exception as e:
            results["errors"] += 1
            print(f"Error processing {interview_file.name}: {e}")

    results["all_valid_techniques"] = list(results["all_valid_techniques"])
    return results


def main():
    import argparse
    parser = argparse.ArgumentParser(description="MITRE ATT&CK Data Fetcher")
    parser.add_argument("--refresh", action="store_true", help="Force refresh cache")
    parser.add_argument("--validate", type=str, help="Validate interviews in directory")
    parser.add_argument("--export", action="store_true", help="Export lookup tables")
    parser.add_argument("--lookup", type=str, help="Look up a technique ID")
    args = parser.parse_args()

    fetcher = MITREDataFetcher()

    if not fetcher.fetch_enterprise_attack(force_refresh=args.refresh):
        print("Failed to fetch MITRE data")
        return

    print(f"\nMITRE ATT&CK Data Loaded:")
    print(f"  Techniques: {len(fetcher.techniques)}")
    print(f"  Groups: {len(fetcher.groups)}")
    print(f"  Tactics: {len(fetcher.tactics)}")
    print(f"  Software: {len(fetcher.software)}")

    if args.export:
        fetcher.save_lookup_tables()

    if args.lookup:
        info = fetcher.get_technique_info(args.lookup)
        if info:
            print(f"\n{args.lookup}: {info['name']}")
            print(f"  Tactics: {', '.join(info['tactics'])}")
            print(f"  Platforms: {', '.join(info['platforms'])}")
        else:
            print(f"\n{args.lookup}: NOT FOUND")

    if args.validate:
        validate_dir = Path(args.validate)
        if validate_dir.exists():
            print(f"\nValidating interviews in {validate_dir}...")
            results = enrich_interviews_batch(validate_dir, fetcher)
            print(f"\nValidation Results:")
            print(f"  Total: {results['total']}")
            print(f"  Validated: {results['validated']}")
            print(f"  Errors: {results['errors']}")
            print(f"  Unique valid techniques: {len(results['all_valid_techniques'])}")

            if results["invalid_techniques_found"]:
                print(f"\nFiles with invalid techniques:")
                for item in results["invalid_techniques_found"][:10]:
                    print(f"  {item['file']}: {item['invalid']}")


if __name__ == "__main__":
    main()
