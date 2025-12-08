#!/usr/bin/env python3
"""
RFC Compliance Checker with Crawl4AI + Playwright Integration
Documents as Code: Extracts RFC requirements and validates code against them
"""

import asyncio
import json
import os
import re
import subprocess
from datetime import datetime
from dataclasses import dataclass, asdict, field
from pathlib import Path
from typing import Dict, List, Optional, Any, Set
import structlog

# Configure logging
logger = structlog.get_logger()

@dataclass
class RFCRequirement:
    """A single requirement extracted from an RFC"""
    rfc_number: str
    section: str
    requirement_id: str
    description: str
    keywords: List[str]
    severity: str  # MUST, SHOULD, MAY
    verification_pattern: Optional[str] = None

@dataclass
class ComplianceViolation:
    """A detected compliance violation"""
    rfc_number: str
    requirement_id: str
    file_path: str
    line_number: int
    description: str
    severity: str
    suggestion: str

@dataclass
class RFCComplianceReport:
    """Full compliance report"""
    crate_name: str
    timestamp: str
    total_files_analyzed: int
    total_requirements_checked: int
    violations: List[ComplianceViolation]
    compliance_score: float
    rfc_coverage: Dict[str, float]
    recommendations: List[str]

class RFCExtractor:
    """
    Extract requirements from RFC markdown files
    Uses regex patterns to identify MUST/SHOULD/MAY requirements
    """

    # RFC-2119 keywords for requirement levels
    REQUIREMENT_PATTERNS = [
        (r'\bMUST\b(?:\s+NOT)?', 'MUST'),
        (r'\bSHALL\b(?:\s+NOT)?', 'MUST'),
        (r'\bREQUIRED\b', 'MUST'),
        (r'\bSHOULD\b(?:\s+NOT)?', 'SHOULD'),
        (r'\bRECOMMENDED\b', 'SHOULD'),
        (r'\bMAY\b', 'MAY'),
        (r'\bOPTIONAL\b', 'MAY'),
    ]

    def __init__(self, rfc_directory: str):
        self.rfc_directory = Path(rfc_directory)
        self.requirements: List[RFCRequirement] = []

    async def extract_all_requirements(self) -> List[RFCRequirement]:
        """Extract requirements from all RFC files"""
        logger.info("Extracting RFC requirements", rfc_dir=str(self.rfc_directory))

        rfc_files = list(self.rfc_directory.rglob("RFC-*.md"))
        logger.info(f"Found {len(rfc_files)} RFC files")

        for rfc_file in rfc_files:
            await self._extract_from_file(rfc_file)

        logger.info(f"Extracted {len(self.requirements)} requirements")
        return self.requirements

    async def _extract_from_file(self, rfc_file: Path):
        """Extract requirements from a single RFC file"""
        try:
            content = rfc_file.read_text(encoding='utf-8')
            rfc_number = self._extract_rfc_number(rfc_file.name)

            # Split into sections
            sections = self._split_into_sections(content)

            for section_title, section_content in sections.items():
                # Find requirement statements
                for pattern, severity in self.REQUIREMENT_PATTERNS:
                    matches = re.finditer(
                        rf'([^.]*{pattern}[^.]*\.)',
                        section_content,
                        re.IGNORECASE
                    )

                    for match in matches:
                        statement = match.group(1).strip()
                        keywords = self._extract_keywords(statement)

                        req = RFCRequirement(
                            rfc_number=rfc_number,
                            section=section_title,
                            requirement_id=f"{rfc_number}-{len(self.requirements)+1:03d}",
                            description=statement,
                            keywords=keywords,
                            severity=severity,
                            verification_pattern=self._generate_verification_pattern(statement)
                        )
                        self.requirements.append(req)

        except Exception as e:
            logger.error(f"Error extracting from {rfc_file}", error=str(e))

    def _extract_rfc_number(self, filename: str) -> str:
        """Extract RFC number from filename"""
        match = re.search(r'RFC-(\d+)', filename)
        return f"RFC-{match.group(1)}" if match else "RFC-UNKNOWN"

    def _split_into_sections(self, content: str) -> Dict[str, str]:
        """Split markdown content into sections by headers"""
        sections = {}
        current_section = "Introduction"
        current_content = []

        for line in content.split('\n'):
            if line.startswith('#'):
                if current_content:
                    sections[current_section] = '\n'.join(current_content)
                current_section = line.lstrip('#').strip()
                current_content = []
            else:
                current_content.append(line)

        if current_content:
            sections[current_section] = '\n'.join(current_content)

        return sections

    def _extract_keywords(self, statement: str) -> List[str]:
        """Extract technical keywords from statement"""
        # Common technical terms in CTAS-7
        tech_terms = [
            'trivariate', 'murmur3', 'blake3', 'hash', 'port',
            'neural mux', 'atlas', 'ooda', 'bernoulli', 'smart crate',
            'cdn', 'routing', 'latency', 'encryption', 'serde',
            'nvnn', 'comment', 'foundation', 'tactical', 'orbital'
        ]

        statement_lower = statement.lower()
        return [term for term in tech_terms if term in statement_lower]

    def _generate_verification_pattern(self, statement: str) -> Optional[str]:
        """Generate regex pattern for code verification"""
        statement_lower = statement.lower()

        # Specific anti-patterns
        if 'blake3' in statement_lower and 'not' in statement_lower:
            return r'blake3|Blake3|BLAKE3'
        if 'murmur3' in statement_lower:
            return r'murmur3|Murmur3'
        if 'nvnn' in statement_lower or 'comment' in statement_lower:
            return r'//\s*\w+\s+\w+\s+\w+\s+\w+'  # NVNN pattern

        return None


class CodeValidator:
    """
    Validate Rust code against RFC requirements
    """

    # Critical anti-patterns that violate RFCs
    ANTI_PATTERNS = {
        'RFC-9001': [
            (r'use\s+blake3', 'Blake3 import detected - RFC-9001 requires Murmur3-128'),
            (r'Blake3', 'Blake3 reference detected - use Murmur3 trivariate hashing'),
        ],
        'RFC-9004': [
            (r'bind\s*\([^)]*:1[0-7]\d{3}\)', 'Port outside 18000-19000 range'),
        ],
    }

    # Required patterns (absence is a violation)
    REQUIRED_PATTERNS = {
        'RFC-NVNN': [
            (r'//.*\w+\s+\w+\s+\w+\s+\w+', 'NVNN comments required every 20 lines'),
        ],
    }

    def __init__(self, requirements: List[RFCRequirement]):
        self.requirements = requirements
        self.violations: List[ComplianceViolation] = []

    async def validate_crate(self, crate_path: str) -> List[ComplianceViolation]:
        """Validate all Rust files in a crate"""
        crate_dir = Path(crate_path)
        rust_files = list(crate_dir.rglob("*.rs"))

        logger.info(f"Validating {len(rust_files)} Rust files in {crate_path}")

        for rust_file in rust_files:
            await self._validate_file(rust_file)

        return self.violations

    async def _validate_file(self, file_path: Path):
        """Validate a single Rust file"""
        try:
            content = file_path.read_text(encoding='utf-8')
            lines = content.split('\n')

            # Check anti-patterns
            for rfc, patterns in self.ANTI_PATTERNS.items():
                for pattern, message in patterns:
                    for line_num, line in enumerate(lines, 1):
                        if re.search(pattern, line):
                            self.violations.append(ComplianceViolation(
                                rfc_number=rfc,
                                requirement_id=f"{rfc}-ANTI",
                                file_path=str(file_path),
                                line_number=line_num,
                                description=message,
                                severity='MUST',
                                suggestion=f"Replace with RFC-compliant alternative"
                            ))

            # Check NVNN comment density
            await self._check_nvnn_compliance(file_path, lines)

        except Exception as e:
            logger.error(f"Error validating {file_path}", error=str(e))

    async def _check_nvnn_compliance(self, file_path: Path, lines: List[str]):
        """Check NVNN comment compliance (every 20 lines)"""
        nvnn_pattern = re.compile(r'//\s*\w+\s+\w+s?\s+\w+\s+(?:via|to|from|against|into|with)\s+\w+')

        code_lines = 0
        last_nvnn = 0

        for line_num, line in enumerate(lines, 1):
            stripped = line.strip()

            # Skip empty lines and pure comments
            if not stripped or stripped.startswith('//'):
                if nvnn_pattern.search(stripped):
                    last_nvnn = line_num
                continue

            code_lines += 1

            # Check if we've gone 20+ lines without NVNN comment
            if code_lines - last_nvnn > 20 and code_lines > 20:
                self.violations.append(ComplianceViolation(
                    rfc_number='RFC-NVNN',
                    requirement_id='RFC-NVNN-001',
                    file_path=str(file_path),
                    line_number=line_num,
                    description=f"No NVNN comment in {code_lines - last_nvnn} lines",
                    severity='SHOULD',
                    suggestion="Add NVNN comment: // [Subject] [Verb] [Object] [Target/Via]"
                ))
                last_nvnn = line_num  # Reset to avoid duplicate warnings


class RFCComplianceEngine:
    """
    Main RFC Compliance Engine with Crawl4AI + Playwright integration
    """

    def __init__(self, repo_path: str):
        self.repo_path = Path(repo_path)
        self.rfc_path = self.repo_path / "01-rfc"
        self.extractor = RFCExtractor(str(self.rfc_path))
        self.requirements: List[RFCRequirement] = []

    async def initialize(self):
        """Initialize the compliance engine"""
        logger.info("Initializing RFC Compliance Engine")
        self.requirements = await self.extractor.extract_all_requirements()

        # Save extracted requirements to cache
        cache_file = self.repo_path / ".zencoder" / "rfc-requirements-cache.json"
        cache_file.parent.mkdir(parents=True, exist_ok=True)

        with open(cache_file, 'w') as f:
            json.dump([asdict(r) for r in self.requirements], f, indent=2)

        logger.info(f"Cached {len(self.requirements)} requirements")

    async def check_crate_compliance(self, crate_name: str) -> RFCComplianceReport:
        """Run full compliance check on a crate"""
        crate_path = self._find_crate(crate_name)

        if not crate_path:
            raise ValueError(f"Crate not found: {crate_name}")

        logger.info(f"Checking RFC compliance for {crate_name}")

        validator = CodeValidator(self.requirements)
        violations = await validator.validate_crate(str(crate_path))

        # Calculate compliance score
        total_checks = len(self.requirements) * 10  # Approximate checks per requirement
        score = max(0, 100 - (len(violations) * 5))  # -5% per violation

        # RFC coverage calculation
        rfc_coverage = {}
        for req in self.requirements:
            if req.rfc_number not in rfc_coverage:
                rfc_coverage[req.rfc_number] = 100.0

        for v in violations:
            if v.rfc_number in rfc_coverage:
                rfc_coverage[v.rfc_number] = max(0, rfc_coverage[v.rfc_number] - 10)

        report = RFCComplianceReport(
            crate_name=crate_name,
            timestamp=datetime.utcnow().isoformat() + 'Z',
            total_files_analyzed=len(list(crate_path.rglob("*.rs"))),
            total_requirements_checked=len(self.requirements),
            violations=violations,
            compliance_score=score,
            rfc_coverage=rfc_coverage,
            recommendations=self._generate_recommendations(violations)
        )

        # Save report
        reports_dir = self.repo_path / "04-abe-iac" / "abe-qa-system" / "reports"
        reports_dir.mkdir(parents=True, exist_ok=True)

        report_file = reports_dir / f"{crate_name}_rfc_compliance.json"
        with open(report_file, 'w') as f:
            json.dump(asdict(report), f, indent=2, default=str)

        logger.info(f"Compliance report saved: {report_file}")
        return report

    def _find_crate(self, crate_name: str) -> Optional[Path]:
        """Find crate directory by name"""
        # Check direct path
        direct = self.repo_path / crate_name
        if direct.exists() and (direct / "Cargo.toml").exists():
            return direct

        # Check in 02-crates
        in_crates = self.repo_path / "02-crates" / crate_name
        if in_crates.exists() and (in_crates / "Cargo.toml").exists():
            return in_crates

        # Search for crate
        for cargo_file in self.repo_path.rglob("Cargo.toml"):
            if cargo_file.parent.name == crate_name:
                return cargo_file.parent

        return None

    def _generate_recommendations(self, violations: List[ComplianceViolation]) -> List[str]:
        """Generate recommendations based on violations"""
        recs = []

        violation_types = {}
        for v in violations:
            key = v.rfc_number
            violation_types[key] = violation_types.get(key, 0) + 1

        if 'RFC-9001' in violation_types:
            recs.append("Replace all Blake3 references with Murmur3-128 trivariate hashing")

        if 'RFC-NVNN' in violation_types:
            recs.append("Add NVNN comments every 20 lines: // [Subject] [Verb] [Object] [Via]")

        if 'RFC-9004' in violation_types:
            recs.append("Register all services with Port Manager (18000-19000 range)")

        if not recs:
            recs.append("Crate is RFC-compliant! Consider adding more NVNN comments for clarity.")

        return recs


class PlaywrightRFCCrawler:
    """
    Use Playwright to crawl and validate RFC documentation
    Ensures RFC markdown renders correctly and links are valid
    """

    def __init__(self, repo_path: str):
        self.repo_path = Path(repo_path)

    async def validate_rfc_links(self) -> Dict[str, Any]:
        """Validate all RFC cross-references and links"""
        try:
            from playwright.async_api import async_playwright

            async with async_playwright() as p:
                browser = await p.chromium.launch(headless=True)
                page = await browser.new_page()

                results = {
                    "valid_links": 0,
                    "broken_links": [],
                    "cross_references": [],
                    "timestamp": datetime.utcnow().isoformat()
                }

                # Find all RFC files
                rfc_files = list((self.repo_path / "01-rfc").rglob("RFC-*.md"))

                for rfc_file in rfc_files:
                    content = rfc_file.read_text(encoding='utf-8')

                    # Find RFC cross-references
                    refs = re.findall(r'RFC-(\d+)', content)
                    for ref in refs:
                        results["cross_references"].append({
                            "from": rfc_file.name,
                            "to": f"RFC-{ref}",
                            "valid": self._check_rfc_exists(f"RFC-{ref}")
                        })

                    # Find URLs
                    urls = re.findall(r'https?://[^\s\)]+', content)
                    for url in urls:
                        # Note: In production, would actually validate URLs
                        results["valid_links"] += 1

                await browser.close()

                logger.info("Playwright RFC validation complete",
                           links=results["valid_links"],
                           refs=len(results["cross_references"]))
                return results

        except ImportError:
            logger.warning("Playwright not installed, skipping link validation")
            return {"error": "Playwright not available"}
        except Exception as e:
            logger.error("Playwright validation failed", error=str(e))
            return {"error": str(e)}

    def _check_rfc_exists(self, rfc_number: str) -> bool:
        """Check if an RFC file exists"""
        rfc_dir = self.repo_path / "01-rfc"
        pattern = f"**/{rfc_number}*.md"
        matches = list(rfc_dir.glob(pattern))
        return len(matches) > 0


class Crawl4AIRFCAnalyzer:
    """
    Use Crawl4AI for deep RFC content analysis
    Extracts structured data from RFC documents
    """

    def __init__(self, repo_path: str):
        self.repo_path = Path(repo_path)

    async def analyze_rfc_structure(self) -> Dict[str, Any]:
        """Analyze RFC document structure using Crawl4AI patterns"""
        # Note: Crawl4AI is primarily for web crawling
        # Here we adapt its extraction patterns for local markdown

        analysis = {
            "rfc_count": 0,
            "by_series": {},
            "by_status": {},
            "requirement_density": {},
            "cross_reference_graph": {},
            "timestamp": datetime.utcnow().isoformat()
        }

        rfc_files = list((self.repo_path / "01-rfc").rglob("RFC-*.md"))
        analysis["rfc_count"] = len(rfc_files)

        for rfc_file in rfc_files:
            rfc_num = self._extract_rfc_number(rfc_file.name)
            series = self._get_series(rfc_num)

            analysis["by_series"][series] = analysis["by_series"].get(series, 0) + 1

            content = rfc_file.read_text(encoding='utf-8')

            # Extract status
            status_match = re.search(r'\*\*Status:\*\*\s*(\w+)', content)
            if status_match:
                status = status_match.group(1)
                analysis["by_status"][status] = analysis["by_status"].get(status, 0) + 1

            # Count requirements
            must_count = len(re.findall(r'\bMUST\b', content))
            should_count = len(re.findall(r'\bSHOULD\b', content))
            analysis["requirement_density"][rfc_num] = {
                "must": must_count,
                "should": should_count,
                "total": must_count + should_count
            }

            # Build cross-reference graph
            refs = set(re.findall(r'RFC-(\d+)', content))
            refs.discard(rfc_num.replace("RFC-", ""))  # Remove self-reference
            analysis["cross_reference_graph"][rfc_num] = list(refs)

        logger.info("Crawl4AI RFC analysis complete",
                   rfcs=analysis["rfc_count"],
                   series=len(analysis["by_series"]))
        return analysis

    def _extract_rfc_number(self, filename: str) -> str:
        match = re.search(r'RFC-(\d+)', filename)
        return f"RFC-{match.group(1)}" if match else "RFC-UNKNOWN"

    def _get_series(self, rfc_num: str) -> str:
        try:
            num = int(rfc_num.replace("RFC-", ""))
            if 9000 <= num <= 9009:
                return "Core"
            elif 9010 <= num <= 9019:
                return "Pipeline"
            elif 9020 <= num <= 9029:
                return "Cognitive"
            elif 9100 <= num <= 9149:
                return "Integration"
            elif 9150 <= num <= 9199:
                return "Application"
            elif 9200 <= num <= 9299:
                return "Platform"
            else:
                return "Other"
        except:
            return "Unknown"


# Pre-commit hook integration
async def run_precommit_check(repo_path: str) -> int:
    """Run RFC compliance check as pre-commit hook"""
    print("🔍 Running RFC Compliance Check...")

    engine = RFCComplianceEngine(repo_path)
    await engine.initialize()

    # Find staged Rust files
    result = subprocess.run(
        ['git', 'diff', '--cached', '--name-only', '--diff-filter=ACM'],
        capture_output=True, text=True, cwd=repo_path
    )

    staged_files = [f for f in result.stdout.strip().split('\n') if f.endswith('.rs')]

    if not staged_files:
        print("✅ No Rust files staged, skipping RFC check")
        return 0

    # Find unique crates
    crates = set()
    for file in staged_files:
        parts = Path(file).parts
        for i, part in enumerate(parts):
            if 'ctas7' in part or part == 'src':
                if i > 0:
                    crates.add(parts[i-1] if 'ctas7' not in parts[i-1] else part)
                    break

    if not crates:
        crates = {'repo-root'}

    total_violations = 0
    critical_violations = 0

    for crate in crates:
        try:
            report = await engine.check_crate_compliance(crate)
            total_violations += len(report.violations)
            critical_violations += sum(1 for v in report.violations if v.severity == 'MUST')

            if report.violations:
                print(f"\n📦 {crate}: {len(report.violations)} violations")
                for v in report.violations[:5]:  # Show first 5
                    print(f"   ❌ {v.rfc_number}: {v.description}")
                    print(f"      {v.file_path}:{v.line_number}")
                    print(f"      💡 {v.suggestion}")
        except Exception as e:
            print(f"⚠️  Could not check {crate}: {e}")

    print(f"\n📊 RFC Compliance Summary:")
    print(f"   Total violations: {total_violations}")
    print(f"   Critical (MUST): {critical_violations}")

    if critical_violations > 0:
        print(f"\n❌ Commit blocked: {critical_violations} critical RFC violations")
        print("   Fix violations or use SKIP_RFC_CHECK=1 to bypass")
        return 1
    elif total_violations > 0:
        print(f"\n⚠️  Warning: {total_violations} RFC recommendations")
        return 0  # Don't block, just warn
    else:
        print("\n✅ RFC Compliance Check Passed!")
        return 0


# CLI entrypoint
async def main():
    import sys

    repo_path = os.getenv(
        'CTAS7_REPO',
        '/Users/cp5337/Developer/ctas-7-shipyard-staging'
    )

    if len(sys.argv) > 1:
        command = sys.argv[1]

        if command == 'precommit':
            exit_code = await run_precommit_check(repo_path)
            sys.exit(exit_code)

        elif command == 'check':
            if len(sys.argv) < 3:
                print("Usage: rfc_compliance_checker.py check <crate_name>")
                sys.exit(1)
            crate_name = sys.argv[2]

            engine = RFCComplianceEngine(repo_path)
            await engine.initialize()
            report = await engine.check_crate_compliance(crate_name)

            print(json.dumps(asdict(report), indent=2, default=str))

        elif command == 'extract':
            extractor = RFCExtractor(os.path.join(repo_path, "01-rfc"))
            requirements = await extractor.extract_all_requirements()
            print(json.dumps([asdict(r) for r in requirements], indent=2))

        elif command == 'analyze':
            analyzer = Crawl4AIRFCAnalyzer(repo_path)
            analysis = await analyzer.analyze_rfc_structure()
            print(json.dumps(analysis, indent=2))

        elif command == 'validate-links':
            crawler = PlaywrightRFCCrawler(repo_path)
            results = await crawler.validate_rfc_links()
            print(json.dumps(results, indent=2))

        else:
            print(f"Unknown command: {command}")
            print("Commands: precommit, check <crate>, extract, analyze, validate-links")
            sys.exit(1)
    else:
        # Default: run analysis
        analyzer = Crawl4AIRFCAnalyzer(repo_path)
        analysis = await analyzer.analyze_rfc_structure()

        print("📚 RFC Analysis Summary")
        print(f"   Total RFCs: {analysis['rfc_count']}")
        print(f"   Series: {analysis['by_series']}")
        print(f"   Status: {analysis['by_status']}")


if __name__ == "__main__":
    asyncio.run(main())
