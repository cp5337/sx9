#!/usr/bin/env python3
"""
Document Fingerprint System - IP Extraction from Markdown Corpus
RFC-9001 Trivariate Hash + RFC-9200 §8 IP Harvester Integration

Integrates with existing ctas7_needle_extractor.py for entity extraction.

Processes 7,000+ markdown files to:
1. Generate trivariate fingerprints (Murmur3-128)
2. Extract novel IP claims via NeedleExtractor
3. Detect duplicate/derivative content
4. Build patent-ready documentation
5. Create Zotero-compatible bibliography
"""

import asyncio
import hashlib
import json
import os
import re
import struct
import sys
from collections import defaultdict
from dataclasses import dataclass, field, asdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple, Any
import structlog

# Import the existing needle extractor
sys.path.insert(0, str(Path(__file__).parent.parent.parent.parent.parent))
try:
    from ctas7_needle_extractor import NeedleExtractor, Needle
    HAS_NEEDLE_EXTRACTOR = True
except ImportError:
    HAS_NEEDLE_EXTRACTOR = False
    Needle = None

# Try to import murmur3, fall back to simpler hash if not available
try:
    import mmh3
    HAS_MURMUR3 = True
except ImportError:
    HAS_MURMUR3 = False

logger = structlog.get_logger()

# ═══════════════════════════════════════════════════════════════════════════
# TRIVARIATE FINGERPRINT (RFC-9001 Aligned)
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class TrivariateFingerprint:
    """
    RFC-9001 Trivariate Hash: prefix:domain:sequence

    prefix (16-bit): Document type classification
    domain (8-bit): Content domain/category
    sequence (104-bit): Content-derived hash
    """
    prefix: int      # 0x0000-0xFFFF
    domain: int      # 0x00-0xFF
    sequence: bytes  # 13 bytes (104 bits)

    # Document type prefixes
    PREFIX_RFC = 0x1000
    PREFIX_ARCHITECTURE = 0x2000
    PREFIX_RESEARCH = 0x3000
    PREFIX_PATENT = 0x4000
    PREFIX_CODE_DOC = 0x5000
    PREFIX_MEETING = 0x6000
    PREFIX_ANALYSIS = 0x7000
    PREFIX_UNKNOWN = 0x0000

    # Domain classifications
    DOMAIN_CORE = 0x01
    DOMAIN_PIPELINE = 0x02
    DOMAIN_COGNITIVE = 0x03
    DOMAIN_INTEGRATION = 0x04
    DOMAIN_APPLICATION = 0x05
    DOMAIN_SECURITY = 0x10
    DOMAIN_GIS = 0x20
    DOMAIN_VOICE = 0x30
    DOMAIN_GRAPH = 0x40

    def to_hex(self) -> str:
        """SCH format: 0xPPPP:0xDD:SSSSSSSSSSSSSSSSSSSSSSSSSS"""
        return f"0x{self.prefix:04X}:0x{self.domain:02X}:{self.sequence.hex().upper()}"

    def to_bytes(self) -> bytes:
        """Full 128-bit representation"""
        return struct.pack('>HB', self.prefix, self.domain) + self.sequence

    @classmethod
    def from_content(cls, content: str, doc_type: str = None, domain: str = None) -> 'TrivariateFingerprint':
        """Generate fingerprint from document content"""
        # Determine prefix from document type
        prefix = cls._classify_prefix(doc_type, content)

        # Determine domain from content analysis
        domain_id = cls._classify_domain(domain, content)

        # Generate 128-bit hash using Murmur3 (or fallback)
        if HAS_MURMUR3:
            hash_128 = mmh3.hash128(content.encode('utf-8'), signed=False)
            hash_bytes = hash_128.to_bytes(16, 'big')
        else:
            # Fallback: use MD5 (not for production, just development)
            hash_bytes = hashlib.md5(content.encode('utf-8')).digest()

        # Extract 104-bit sequence (13 bytes)
        sequence = hash_bytes[:13]

        return cls(prefix=prefix, domain=domain_id, sequence=sequence)

    @classmethod
    def _classify_prefix(cls, doc_type: str, content: str) -> int:
        """Classify document type prefix"""
        if doc_type:
            mapping = {
                'rfc': cls.PREFIX_RFC,
                'architecture': cls.PREFIX_ARCHITECTURE,
                'research': cls.PREFIX_RESEARCH,
                'patent': cls.PREFIX_PATENT,
                'code': cls.PREFIX_CODE_DOC,
                'meeting': cls.PREFIX_MEETING,
                'analysis': cls.PREFIX_ANALYSIS,
            }
            for key, prefix in mapping.items():
                if key in doc_type.lower():
                    return prefix

        # Auto-detect from content
        content_lower = content.lower()
        if 'rfc-' in content_lower or '**rfc' in content_lower:
            return cls.PREFIX_RFC
        elif 'patent' in content_lower or 'claim' in content_lower:
            return cls.PREFIX_PATENT
        elif 'architecture' in content_lower or 'design' in content_lower:
            return cls.PREFIX_ARCHITECTURE
        elif 'meeting' in content_lower or 'agenda' in content_lower:
            return cls.PREFIX_MEETING

        return cls.PREFIX_UNKNOWN

    @classmethod
    def _classify_domain(cls, domain: str, content: str) -> int:
        """Classify content domain"""
        if domain:
            mapping = {
                'core': cls.DOMAIN_CORE,
                'pipeline': cls.DOMAIN_PIPELINE,
                'cognitive': cls.DOMAIN_COGNITIVE,
                'integration': cls.DOMAIN_INTEGRATION,
                'application': cls.DOMAIN_APPLICATION,
                'security': cls.DOMAIN_SECURITY,
                'gis': cls.DOMAIN_GIS,
                'voice': cls.DOMAIN_VOICE,
                'graph': cls.DOMAIN_GRAPH,
            }
            for key, dom in mapping.items():
                if key in domain.lower():
                    return dom

        # Auto-detect from content
        content_lower = content.lower()
        if 'trivariate' in content_lower or 'hash' in content_lower:
            return cls.DOMAIN_CORE
        elif 'neural' in content_lower or 'cognitive' in content_lower:
            return cls.DOMAIN_COGNITIVE
        elif 'gis' in content_lower or 'geospatial' in content_lower:
            return cls.DOMAIN_GIS
        elif 'security' in content_lower or 'encrypt' in content_lower:
            return cls.DOMAIN_SECURITY
        elif 'voice' in content_lower or 'audio' in content_lower:
            return cls.DOMAIN_VOICE

        return cls.DOMAIN_CORE


# ═══════════════════════════════════════════════════════════════════════════
# IP EXTRACTION (RFC-9200 §8 Aligned)
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class NovelClaim:
    """A novel IP claim extracted from document"""
    claim_id: str
    claim_type: str  # 'method', 'system', 'apparatus', 'composition'
    title: str
    description: str
    keywords: List[str]
    source_document: str
    source_fingerprint: str
    confidence: float  # 0.0-1.0
    related_claims: List[str] = field(default_factory=list)
    prior_art_refs: List[str] = field(default_factory=list)


@dataclass
class DocumentIP:
    """Extracted IP from a single document"""
    document_path: str
    fingerprint: TrivariateFingerprint
    title: str
    abstract: str
    novel_claims: List[NovelClaim]
    terminology: Dict[str, str]  # term -> definition
    algorithms: List[Dict[str, Any]]
    data_structures: List[Dict[str, Any]]
    novelty_score: float
    timestamp: str


@dataclass
class IPCorpus:
    """Full IP corpus from all documents"""
    total_documents: int
    unique_fingerprints: int
    duplicate_groups: List[List[str]]  # Groups of similar/duplicate docs
    novel_claims: List[NovelClaim]
    terminology_index: Dict[str, List[str]]  # term -> [doc_paths]
    algorithm_index: Dict[str, List[str]]  # algo_name -> [doc_paths]
    citation_graph: Dict[str, List[str]]  # doc -> [cited_docs]
    generated_at: str


class IPExtractor:
    """
    Extract IP from markdown documents

    Implements RFC-9200 §8.2 Harvested Artifacts:
    - Patent claims
    - Academic paper sections
    - Copyright excerpts
    - Trade secret documentation

    Integrates with ctas7_needle_extractor.py for entity extraction.
    """

    def __init__(self):
        self.fingerprints: Dict[str, TrivariateFingerprint] = {}
        self.documents: Dict[str, DocumentIP] = {}
        self.similarity_threshold = 0.85

        # Initialize needle extractor if available
        if HAS_NEEDLE_EXTRACTOR:
            self.needle_extractor = NeedleExtractor()
            logger.info("NeedleExtractor initialized from ctas7_needle_extractor.py")
        else:
            self.needle_extractor = None
            logger.warning("NeedleExtractor not available - using fallback patterns")

    # Patterns for extracting novel content
    CLAIM_PATTERNS = [
        (r'(?:novel|unique|innovative)\s+(?:method|approach|technique|system)\s+(?:for|to)\s+([^.]+)', 'method'),
        (r'(?:we|this)\s+(?:propose|introduce|present)\s+([^.]+)', 'method'),
        (r'(?:our|the)\s+(?:algorithm|system|framework)\s+([^.]+)', 'system'),
        (r'(?:invention|contribution)(?:s)?\s+(?:include|are|is)\s*:?\s*([^.]+)', 'apparatus'),
    ]

    ALGORITHM_PATTERNS = [
        r'(?:algorithm|procedure|method)\s*(?:\d+)?[:\s]+([^\n]+)',
        r'```(?:python|rust|pseudo)?\n(.*?)```',
        r'(?:step\s*\d+)[:\s]+([^\n]+)',
    ]

    DEFINITION_PATTERNS = [
        r'\*\*([^*]+)\*\*\s*[-:]\s*([^.\n]+)',
        r'([A-Z][a-zA-Z]+)\s*(?:is|refers to|means)\s+([^.]+)',
        r'(?:Definition|Term)[:\s]+([^:]+)[:\s]+([^.\n]+)',
    ]

    async def extract_document_ip(self, file_path: Path) -> Optional[DocumentIP]:
        """Extract IP from a single markdown document"""
        try:
            content = file_path.read_text(encoding='utf-8', errors='replace')

            # Generate fingerprint
            fingerprint = TrivariateFingerprint.from_content(
                content,
                doc_type=self._infer_doc_type(file_path),
                domain=self._infer_domain(file_path)
            )

            # Use NeedleExtractor if available (from ctas7_needle_extractor.py)
            needles = []
            needle_fingerprint = None
            if self.needle_extractor:
                needles = self.needle_extractor.extract_needles(content)
                needle_fingerprint = self.needle_extractor.generate_semantic_fingerprint(needles)

            # Extract metadata
            title = self._extract_title(content, file_path)
            abstract = self._extract_abstract(content)

            # Extract novel claims (enhanced with needles)
            novel_claims = self._extract_claims(content, file_path, fingerprint, needles)

            # Extract terminology
            terminology = self._extract_terminology(content)

            # Extract algorithms
            algorithms = self._extract_algorithms(content)

            # Extract data structures
            data_structures = self._extract_data_structures(content)

            # Calculate novelty score
            novelty_score = self._calculate_novelty(
                novel_claims, terminology, algorithms
            )

            doc_ip = DocumentIP(
                document_path=str(file_path),
                fingerprint=fingerprint,
                title=title,
                abstract=abstract,
                novel_claims=novel_claims,
                terminology=terminology,
                algorithms=algorithms,
                data_structures=data_structures,
                novelty_score=novelty_score,
                timestamp=datetime.utcnow().isoformat() + 'Z'
            )

            self.fingerprints[str(file_path)] = fingerprint
            self.documents[str(file_path)] = doc_ip

            return doc_ip

        except Exception as e:
            logger.error(f"Failed to extract IP from {file_path}: {e}")
            return None

    def _infer_doc_type(self, path: Path) -> str:
        """Infer document type from path"""
        path_str = str(path).lower()
        if 'rfc' in path_str:
            return 'rfc'
        elif 'patent' in path_str:
            return 'patent'
        elif 'research' in path_str:
            return 'research'
        elif 'meeting' in path_str or 'notes' in path_str:
            return 'meeting'
        elif 'arch' in path_str or 'design' in path_str:
            return 'architecture'
        return 'unknown'

    def _infer_domain(self, path: Path) -> str:
        """Infer domain from path"""
        path_str = str(path).lower()
        if 'core' in path_str or 'foundation' in path_str:
            return 'core'
        elif 'pipeline' in path_str or 'ingestion' in path_str:
            return 'pipeline'
        elif 'cognitive' in path_str or 'neural' in path_str:
            return 'cognitive'
        elif 'gis' in path_str or 'geo' in path_str:
            return 'gis'
        elif 'security' in path_str:
            return 'security'
        elif 'voice' in path_str or 'audio' in path_str:
            return 'voice'
        return 'integration'

    def _extract_title(self, content: str, path: Path) -> str:
        """Extract document title"""
        # Try H1 header
        match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
        if match:
            return match.group(1).strip()

        # Try title metadata
        match = re.search(r'\*\*(?:Title|title):\*\*\s*(.+)', content)
        if match:
            return match.group(1).strip()

        # Fallback to filename
        return path.stem.replace('-', ' ').replace('_', ' ').title()

    def _extract_abstract(self, content: str) -> str:
        """Extract document abstract"""
        # Try explicit abstract section
        match = re.search(
            r'(?:##?\s*)?(?:Abstract|Summary|Overview)\s*\n(.*?)(?=\n##|\n\*\*|\Z)',
            content,
            re.DOTALL | re.IGNORECASE
        )
        if match:
            return match.group(1).strip()[:500]

        # Take first paragraph after title
        match = re.search(r'^#[^\n]+\n+([^#\n][^\n]+)', content, re.MULTILINE)
        if match:
            return match.group(1).strip()[:500]

        return ""

    def _extract_claims(
        self, content: str, path: Path, fingerprint: TrivariateFingerprint,
        needles: List = None
    ) -> List[NovelClaim]:
        """Extract novel claims from content"""
        claims = []

        for pattern, claim_type in self.CLAIM_PATTERNS:
            for match in re.finditer(pattern, content, re.IGNORECASE):
                claim_text = match.group(1).strip()
                if len(claim_text) > 20:  # Filter out short matches
                    claim_id = f"CLM-{fingerprint.to_hex()[:16]}-{len(claims)+1:03d}"

                    # Extract keywords from claim
                    keywords = self._extract_keywords(claim_text)

                    claims.append(NovelClaim(
                        claim_id=claim_id,
                        claim_type=claim_type,
                        title=claim_text[:100],
                        description=claim_text,
                        keywords=keywords,
                        source_document=str(path),
                        source_fingerprint=fingerprint.to_hex(),
                        confidence=0.7 + (0.1 * min(len(keywords), 3))
                    ))

        return claims

    def _extract_keywords(self, text: str) -> List[str]:
        """Extract technical keywords from text"""
        # Technical terms commonly found in patents
        tech_terms = [
            'trivariate', 'hash', 'neural', 'cognitive', 'quantum',
            'encryption', 'routing', 'mux', 'daemon', 'agent',
            'graph', 'embedding', 'vector', 'semantic', 'ontology',
            'extraction', 'pipeline', 'orchestration', 'mesh',
            'geospatial', 'satellite', 'telemetry', 'sensor',
        ]

        text_lower = text.lower()
        found = [term for term in tech_terms if term in text_lower]

        # Also extract capitalized terms
        caps = re.findall(r'\b([A-Z][a-z]+(?:[A-Z][a-z]+)+)\b', text)
        found.extend(caps[:5])

        return list(set(found))

    def _extract_terminology(self, content: str) -> Dict[str, str]:
        """Extract term definitions"""
        terms = {}

        for pattern in self.DEFINITION_PATTERNS:
            for match in re.finditer(pattern, content, re.IGNORECASE):
                term = match.group(1).strip()
                definition = match.group(2).strip()
                if len(term) > 2 and len(definition) > 10:
                    terms[term] = definition

        return terms

    def _extract_algorithms(self, content: str) -> List[Dict[str, Any]]:
        """Extract algorithm descriptions"""
        algorithms = []

        for pattern in self.ALGORITHM_PATTERNS:
            for match in re.finditer(pattern, content, re.DOTALL | re.IGNORECASE):
                algo_content = match.group(1).strip()
                if len(algo_content) > 20:
                    algorithms.append({
                        'name': algo_content[:50],
                        'description': algo_content[:200],
                        'type': 'extracted'
                    })

        return algorithms[:10]  # Limit to 10 algorithms per doc

    def _extract_data_structures(self, content: str) -> List[Dict[str, Any]]:
        """Extract data structure definitions"""
        structures = []

        # Look for struct/class/interface definitions in code blocks
        pattern = r'```(?:rust|typescript|python)?\n.*?(?:struct|class|interface)\s+(\w+)[^`]*```'
        for match in re.finditer(pattern, content, re.DOTALL):
            structures.append({
                'name': match.group(1),
                'source': match.group(0)[:200]
            })

        # Look for table definitions
        pattern = r'\|\s*(?:Field|Column|Attribute)\s*\|'
        if re.search(pattern, content, re.IGNORECASE):
            structures.append({
                'name': 'TableDefinition',
                'type': 'schema'
            })

        return structures

    def _calculate_novelty(
        self,
        claims: List[NovelClaim],
        terminology: Dict[str, str],
        algorithms: List[Dict[str, Any]]
    ) -> float:
        """Calculate novelty score 0.0-1.0"""
        score = 0.0

        # Claims contribute 40%
        score += min(len(claims) * 0.1, 0.4)

        # Terminology contributes 30%
        score += min(len(terminology) * 0.05, 0.3)

        # Algorithms contribute 30%
        score += min(len(algorithms) * 0.1, 0.3)

        return round(score, 2)

    def find_duplicates(self) -> List[List[str]]:
        """Find duplicate/similar documents based on fingerprints"""
        groups = []
        processed = set()

        fingerprint_list = list(self.fingerprints.items())

        for i, (path1, fp1) in enumerate(fingerprint_list):
            if path1 in processed:
                continue

            group = [path1]
            for j, (path2, fp2) in enumerate(fingerprint_list[i+1:], i+1):
                if path2 in processed:
                    continue

                # Compare fingerprints
                if self._fingerprint_similarity(fp1, fp2) >= self.similarity_threshold:
                    group.append(path2)
                    processed.add(path2)

            if len(group) > 1:
                groups.append(group)
            processed.add(path1)

        return groups

    def _fingerprint_similarity(
        self, fp1: TrivariateFingerprint, fp2: TrivariateFingerprint
    ) -> float:
        """Calculate similarity between two fingerprints"""
        # Same prefix and domain is a strong signal
        if fp1.prefix != fp2.prefix or fp1.domain != fp2.domain:
            return 0.0

        # Compare sequence bytes
        matches = sum(a == b for a, b in zip(fp1.sequence, fp2.sequence))
        return matches / len(fp1.sequence)


# ═══════════════════════════════════════════════════════════════════════════
# CORPUS PROCESSOR
# ═══════════════════════════════════════════════════════════════════════════

class DocumentCorpusProcessor:
    """
    Process entire markdown corpus for IP extraction
    """

    def __init__(self, repo_path: str):
        self.repo_path = Path(repo_path)
        self.extractor = IPExtractor()
        self.output_dir = self.repo_path / "04-abe-iac" / "abe-qa-system" / "ea-documents" / "ip-corpus"
        self.output_dir.mkdir(parents=True, exist_ok=True)

    async def process_all_markdown(self) -> IPCorpus:
        """Process all markdown files in repository"""
        md_files = list(self.repo_path.rglob("*.md"))

        logger.info(f"Processing {len(md_files)} markdown files...")

        processed = 0
        errors = 0

        for i, md_file in enumerate(md_files):
            # Skip node_modules, .git, etc.
            if any(skip in str(md_file) for skip in ['node_modules', '.git', 'target', 'dist']):
                continue

            result = await self.extractor.extract_document_ip(md_file)
            if result:
                processed += 1
            else:
                errors += 1

            if (i + 1) % 500 == 0:
                logger.info(f"Progress: {i+1}/{len(md_files)} ({processed} extracted, {errors} errors)")

        # Find duplicates
        duplicate_groups = self.extractor.find_duplicates()

        # Build indices
        terminology_index = self._build_terminology_index()
        algorithm_index = self._build_algorithm_index()
        citation_graph = self._build_citation_graph()

        # Collect all novel claims
        all_claims = []
        for doc_ip in self.extractor.documents.values():
            all_claims.extend(doc_ip.novel_claims)

        corpus = IPCorpus(
            total_documents=processed,
            unique_fingerprints=len(set(
                fp.to_hex() for fp in self.extractor.fingerprints.values()
            )),
            duplicate_groups=duplicate_groups,
            novel_claims=all_claims,
            terminology_index=terminology_index,
            algorithm_index=algorithm_index,
            citation_graph=citation_graph,
            generated_at=datetime.utcnow().isoformat() + 'Z'
        )

        # Save results
        await self._save_corpus(corpus)

        logger.info(f"IP Extraction Complete:")
        logger.info(f"  Documents: {corpus.total_documents}")
        logger.info(f"  Unique fingerprints: {corpus.unique_fingerprints}")
        logger.info(f"  Duplicate groups: {len(corpus.duplicate_groups)}")
        logger.info(f"  Novel claims: {len(corpus.novel_claims)}")

        return corpus

    def _build_terminology_index(self) -> Dict[str, List[str]]:
        """Build index of terms to documents"""
        index = defaultdict(list)
        for doc_path, doc_ip in self.extractor.documents.items():
            for term in doc_ip.terminology.keys():
                index[term.lower()].append(doc_path)
        return dict(index)

    def _build_algorithm_index(self) -> Dict[str, List[str]]:
        """Build index of algorithms to documents"""
        index = defaultdict(list)
        for doc_path, doc_ip in self.extractor.documents.items():
            for algo in doc_ip.algorithms:
                index[algo['name'][:30]].append(doc_path)
        return dict(index)

    def _build_citation_graph(self) -> Dict[str, List[str]]:
        """Build citation graph between documents"""
        graph = {}

        for doc_path, doc_ip in self.extractor.documents.items():
            citations = []
            content = Path(doc_path).read_text(encoding='utf-8', errors='replace')

            # Find RFC references
            for match in re.finditer(r'RFC-(\d+)', content):
                rfc_num = match.group(1)
                # Find matching RFC file
                for other_path in self.extractor.documents.keys():
                    if f'RFC-{rfc_num}' in other_path:
                        citations.append(other_path)

            graph[doc_path] = list(set(citations))

        return graph

    async def _save_corpus(self, corpus: IPCorpus):
        """Save corpus results"""
        # Save summary
        summary_path = self.output_dir / "ip-corpus-summary.json"
        with open(summary_path, 'w') as f:
            json.dump({
                'total_documents': corpus.total_documents,
                'unique_fingerprints': corpus.unique_fingerprints,
                'duplicate_groups': len(corpus.duplicate_groups),
                'novel_claims': len(corpus.novel_claims),
                'terminology_terms': len(corpus.terminology_index),
                'algorithms': len(corpus.algorithm_index),
                'generated_at': corpus.generated_at
            }, f, indent=2)

        # Save novel claims
        claims_path = self.output_dir / "novel-claims.json"
        with open(claims_path, 'w') as f:
            json.dump([asdict(c) for c in corpus.novel_claims], f, indent=2)

        # Save fingerprints
        fingerprints_path = self.output_dir / "document-fingerprints.json"
        with open(fingerprints_path, 'w') as f:
            fingerprints = {
                path: {
                    'fingerprint': doc.fingerprint.to_hex(),
                    'title': doc.title,
                    'novelty_score': doc.novelty_score
                }
                for path, doc in self.extractor.documents.items()
            }
            json.dump(fingerprints, f, indent=2)

        # Save duplicates
        if corpus.duplicate_groups:
            dups_path = self.output_dir / "duplicate-groups.json"
            with open(dups_path, 'w') as f:
                json.dump(corpus.duplicate_groups, f, indent=2)

        # Save terminology index
        terms_path = self.output_dir / "terminology-index.json"
        with open(terms_path, 'w') as f:
            json.dump(corpus.terminology_index, f, indent=2)

        logger.info(f"Corpus saved to {self.output_dir}")

    async def generate_patent_claims(self) -> str:
        """Generate patent claims document from extracted IP"""
        claims_by_type = defaultdict(list)

        for claim in self.extractor.documents.values():
            for novel_claim in claim.novel_claims:
                if novel_claim.confidence >= 0.7:
                    claims_by_type[novel_claim.claim_type].append(novel_claim)

        # Generate LaTeX patent claims document
        latex = r"""
\documentclass{article}
\usepackage[margin=1in]{geometry}
\title{Patent Claims - CTAS-7 / SX9 Platform}
\author{Cognetix Intelligence Systems}
\date{\today}

\begin{document}
\maketitle

\section{Independent Claims}
"""
        claim_num = 1
        for claim_type, claims in claims_by_type.items():
            latex += f"\n\\subsection{{{claim_type.title()} Claims}}\n\n"
            for claim in claims[:10]:  # Top 10 per type
                latex += f"""
\\textbf{{Claim {claim_num}:}} {claim.title}

{claim.description}

\\textit{{Source: {Path(claim.source_document).name}}}
\\textit{{Confidence: {claim.confidence:.0%}}}

"""
                claim_num += 1

        latex += r"\end{document}"

        # Save
        patent_path = self.output_dir / "patent-claims.tex"
        patent_path.write_text(latex)

        return str(patent_path)


# ═══════════════════════════════════════════════════════════════════════════
# CLI
# ═══════════════════════════════════════════════════════════════════════════

async def main():
    import sys

    repo_path = os.getenv(
        'CTAS7_REPO',
        '/Users/cp5337/Developer/ctas-7-shipyard-staging'
    )

    processor = DocumentCorpusProcessor(repo_path)

    if len(sys.argv) > 1:
        command = sys.argv[1]

        if command == 'scan':
            corpus = await processor.process_all_markdown()
            print(f"\n✅ Processed {corpus.total_documents} documents")
            print(f"   Novel claims: {len(corpus.novel_claims)}")
            print(f"   Duplicates: {len(corpus.duplicate_groups)} groups")

        elif command == 'patent':
            # Load existing corpus or scan first
            corpus = await processor.process_all_markdown()
            patent_path = await processor.generate_patent_claims()
            print(f"✅ Patent claims generated: {patent_path}")

        elif command == 'fingerprint' and len(sys.argv) > 2:
            file_path = Path(sys.argv[2])
            extractor = IPExtractor()
            doc_ip = await extractor.extract_document_ip(file_path)
            if doc_ip:
                print(f"Fingerprint: {doc_ip.fingerprint.to_hex()}")
                print(f"Title: {doc_ip.title}")
                print(f"Novelty: {doc_ip.novelty_score}")
                print(f"Claims: {len(doc_ip.novel_claims)}")

        else:
            print("Commands:")
            print("  scan              - Scan all markdown files")
            print("  patent            - Generate patent claims LaTeX")
            print("  fingerprint <file> - Fingerprint single file")
    else:
        print("Document Fingerprint System")
        print(f"  Repo: {repo_path}")
        print("\nRun: python document_fingerprint_system.py scan")


if __name__ == "__main__":
    asyncio.run(main())
