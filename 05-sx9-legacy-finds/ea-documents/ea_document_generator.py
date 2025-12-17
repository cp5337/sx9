#!/usr/bin/env python3
"""
EA Document Generator with LaTeX + Zotero + Overleaf Integration
Implements RFC-9010 Prior-Art Check (Zotero/GNN) and RFC-9105 SPIRES Extraction

Generates DoD DevSecOps and Enterprise Architecture documents from code.
"""

import asyncio
import json
import os
import re
import subprocess
from datetime import datetime
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Dict, List, Optional, Any
from string import Template
import structlog

logger = structlog.get_logger()

# ═══════════════════════════════════════════════════════════════════════════
# ZOTERO INTEGRATION (RFC-9010 Prior-Art Check)
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class ZoteroReference:
    """Citation reference from Zotero library"""
    key: str
    item_type: str
    title: str
    authors: List[str]
    date: str
    doi: Optional[str] = None
    url: Optional[str] = None
    abstract: Optional[str] = None
    tags: List[str] = field(default_factory=list)
    bibtex_key: Optional[str] = None


class ZoteroClient:
    """
    Zotero API client using pyzotero
    Supports both local Zotero 7 server and remote API

    References:
    - https://pyzotero.readthedocs.io/
    - https://github.com/urschrei/pyzotero
    """

    def __init__(self, library_id: str = None, api_key: str = None, use_local: bool = True):
        self.library_id = library_id or os.getenv('ZOTERO_LIBRARY_ID')
        self.api_key = api_key or os.getenv('ZOTERO_API_KEY')
        self.use_local = use_local
        self.zot = None

    async def connect(self):
        """Initialize Zotero connection"""
        try:
            from pyzotero import zotero

            if self.use_local:
                # Use local Zotero 7 server (requires Zotero running)
                self.zot = zotero.Zotero(
                    library_id=self.library_id or 'local',
                    library_type='user',
                    local=True  # Uses http://localhost:23119/api
                )
                logger.info("Connected to local Zotero server")
            else:
                # Use remote API
                self.zot = zotero.Zotero(
                    library_id=self.library_id,
                    library_type='user',
                    api_key=self.api_key
                )
                logger.info("Connected to Zotero remote API")

        except ImportError:
            logger.warning("pyzotero not installed: pip install pyzotero")
            self.zot = None
        except Exception as e:
            logger.error(f"Zotero connection failed: {e}")
            self.zot = None

    async def search_references(self, query: str, limit: int = 20) -> List[ZoteroReference]:
        """Search Zotero library for references"""
        if not self.zot:
            return []

        try:
            items = self.zot.everything(self.zot.items(q=query, limit=limit))

            references = []
            for item in items:
                data = item.get('data', {})
                refs = ZoteroReference(
                    key=data.get('key', ''),
                    item_type=data.get('itemType', ''),
                    title=data.get('title', ''),
                    authors=[c.get('lastName', '') for c in data.get('creators', [])],
                    date=data.get('date', ''),
                    doi=data.get('DOI'),
                    url=data.get('url'),
                    abstract=data.get('abstractNote'),
                    tags=[t.get('tag', '') for t in data.get('tags', [])]
                )
                references.append(refs)

            logger.info(f"Found {len(references)} references for '{query}'")
            return references

        except Exception as e:
            logger.error(f"Zotero search failed: {e}")
            return []

    async def get_collection(self, collection_name: str) -> List[ZoteroReference]:
        """Get all items from a Zotero collection"""
        if not self.zot:
            return []

        try:
            collections = self.zot.collections()
            target = None
            for col in collections:
                if col['data']['name'] == collection_name:
                    target = col['key']
                    break

            if target:
                items = self.zot.collection_items(target)
                return [self._item_to_ref(item) for item in items]
            return []

        except Exception as e:
            logger.error(f"Failed to get collection: {e}")
            return []

    async def export_bibtex(self, keys: List[str] = None) -> str:
        """Export references as BibTeX for LaTeX"""
        if not self.zot:
            return ""

        try:
            if keys:
                items = [self.zot.item(k) for k in keys]
            else:
                items = self.zot.items(limit=100)

            # Export as BibTeX
            bibtex = self.zot.items(format='bibtex')
            return bibtex

        except Exception as e:
            logger.error(f"BibTeX export failed: {e}")
            return ""

    def _item_to_ref(self, item: dict) -> ZoteroReference:
        """Convert Zotero API item to ZoteroReference"""
        data = item.get('data', {})
        return ZoteroReference(
            key=data.get('key', ''),
            item_type=data.get('itemType', ''),
            title=data.get('title', ''),
            authors=[c.get('lastName', '') for c in data.get('creators', [])],
            date=data.get('date', ''),
            doi=data.get('DOI'),
            url=data.get('url'),
            abstract=data.get('abstractNote'),
            tags=[t.get('tag', '') for t in data.get('tags', [])]
        )


# ═══════════════════════════════════════════════════════════════════════════
# LATEX DOCUMENT GENERATION (Overleaf-compatible)
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class DocumentSection:
    """A section of the generated document"""
    title: str
    content: str
    subsections: List['DocumentSection'] = field(default_factory=list)
    citations: List[str] = field(default_factory=list)


@dataclass
class EADocument:
    """Enterprise Architecture Document"""
    doc_id: str
    title: str
    doc_type: str  # 'dod-devsecops', 'enterprise-arch', 'rfc', 'sdd', 'srs'
    version: str
    authors: List[str]
    date: str
    abstract: str
    sections: List[DocumentSection]
    references: List[ZoteroReference]
    metadata: Dict[str, Any] = field(default_factory=dict)


class LaTeXGenerator:
    """
    Generate LaTeX documents for Overleaf

    Templates follow DoD DevSecOps and IEEE standards
    """

    # DoD DevSecOps document template
    DOD_DEVSECOPS_TEMPLATE = r'''
\documentclass[11pt,letterpaper]{article}
\usepackage[utf8]{inputenc}
\usepackage{geometry}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{listings}
\usepackage{xcolor}
\usepackage{fancyhdr}
\usepackage{titlesec}
\usepackage{natbib}

\geometry{margin=1in}
\pagestyle{fancy}
\fancyhf{}
\rhead{$doc_type}
\lhead{$title}
\rfoot{Page \thepage}
\lfoot{$classification}

% Code listing style
\lstset{
    basicstyle=\ttfamily\footnotesize,
    breaklines=true,
    frame=single,
    backgroundcolor=\color{gray!10}
}

\title{$title \\ \large $subtitle}
\author{$authors}
\date{$date \\ Version $version}

\begin{document}

\maketitle
\thispagestyle{fancy}

\begin{abstract}
$abstract
\end{abstract}

\tableofcontents
\newpage

$body

\newpage
\bibliographystyle{ieeetr}
\bibliography{references}

\end{document}
'''

    # RFC Document Template
    RFC_TEMPLATE = r'''
\documentclass[11pt]{article}
\usepackage[utf8]{inputenc}
\usepackage{geometry}
\usepackage{hyperref}
\usepackage{listings}
\usepackage{xcolor}
\usepackage{fancyvrb}
\usepackage{longtable}

\geometry{margin=1in}

\title{$rfc_number: $title}
\author{CTAS Core Engineering Group}
\date{$date}

\begin{document}

\maketitle

\section*{Document Info}
\begin{tabular}{ll}
\textbf{RFC Number:} & $rfc_number \\
\textbf{Status:} & $status \\
\textbf{Version:} & $version \\
\textbf{Dependencies:} & $dependencies \\
\end{tabular}

\hrule
\vspace{1em}

$body

\end{document}
'''

    # SDD (Software Design Document) Template
    SDD_TEMPLATE = r'''
\documentclass[11pt,letterpaper]{article}
\usepackage[utf8]{inputenc}
\usepackage{geometry}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{listings}
\usepackage{tikz}
\usepackage{natbib}

\geometry{margin=1in}

\title{Software Design Document \\ \large $system_name}
\author{$authors}
\date{$date}

\begin{document}

\maketitle

\tableofcontents
\newpage

\section{Introduction}
\subsection{Purpose}
$purpose

\subsection{Scope}
$scope

\subsection{Definitions and Acronyms}
$definitions

\section{System Overview}
$system_overview

\section{Design Considerations}
\subsection{Assumptions and Dependencies}
$assumptions

\subsection{Constraints}
$constraints

\section{Architectural Design}
$architecture

\section{Data Design}
$data_design

\section{Component Design}
$component_design

\section{Interface Design}
$interface_design

\section{Security Design}
$security_design

\bibliographystyle{ieeetr}
\bibliography{references}

\end{document}
'''

    def __init__(self, output_dir: str):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)

    async def generate_document(self, doc: EADocument) -> Path:
        """Generate LaTeX document from EADocument"""
        template = self._get_template(doc.doc_type)

        # Build body from sections
        body = self._build_body(doc.sections)

        # Generate BibTeX file
        bib_content = self._generate_bibtex(doc.references)
        bib_path = self.output_dir / f"{doc.doc_id}.bib"
        bib_path.write_text(bib_content)

        # Substitute template
        latex_content = Template(template).safe_substitute(
            title=self._escape_latex(doc.title),
            subtitle=doc.metadata.get('subtitle', ''),
            doc_type=doc.doc_type.upper().replace('-', ' '),
            authors=', '.join(doc.authors),
            date=doc.date,
            version=doc.version,
            abstract=self._escape_latex(doc.abstract),
            body=body,
            classification=doc.metadata.get('classification', 'UNCLASSIFIED'),
            rfc_number=doc.metadata.get('rfc_number', ''),
            status=doc.metadata.get('status', 'Draft'),
            dependencies=doc.metadata.get('dependencies', ''),
            system_name=doc.metadata.get('system_name', doc.title),
            purpose=doc.metadata.get('purpose', ''),
            scope=doc.metadata.get('scope', ''),
            definitions=doc.metadata.get('definitions', ''),
            system_overview=doc.metadata.get('system_overview', ''),
            assumptions=doc.metadata.get('assumptions', ''),
            constraints=doc.metadata.get('constraints', ''),
            architecture=doc.metadata.get('architecture', ''),
            data_design=doc.metadata.get('data_design', ''),
            component_design=doc.metadata.get('component_design', ''),
            interface_design=doc.metadata.get('interface_design', ''),
            security_design=doc.metadata.get('security_design', '')
        )

        # Write LaTeX file
        tex_path = self.output_dir / f"{doc.doc_id}.tex"
        tex_path.write_text(latex_content)

        logger.info(f"Generated LaTeX document: {tex_path}")
        return tex_path

    def _get_template(self, doc_type: str) -> str:
        """Get appropriate template for document type"""
        if doc_type == 'rfc':
            return self.RFC_TEMPLATE
        elif doc_type in ('sdd', 'software-design'):
            return self.SDD_TEMPLATE
        else:
            return self.DOD_DEVSECOPS_TEMPLATE

    def _build_body(self, sections: List[DocumentSection], level: int = 0) -> str:
        """Build LaTeX body from sections"""
        body = []
        section_cmd = ['section', 'subsection', 'subsubsection', 'paragraph'][min(level, 3)]

        for section in sections:
            body.append(f"\\{section_cmd}{{{self._escape_latex(section.title)}}}")
            body.append(self._escape_latex(section.content))

            # Add citations
            if section.citations:
                cite_keys = ', '.join(section.citations)
                body.append(f"\\cite{{{cite_keys}}}")

            # Recurse into subsections
            if section.subsections:
                body.append(self._build_body(section.subsections, level + 1))

        return '\n\n'.join(body)

    def _generate_bibtex(self, references: List[ZoteroReference]) -> str:
        """Generate BibTeX file from references"""
        entries = []
        for ref in references:
            key = ref.bibtex_key or ref.key
            entry_type = self._map_item_type(ref.item_type)

            entry = f"@{entry_type}{{{key},\n"
            entry += f"  title = {{{ref.title}}},\n"
            entry += f"  author = {{{' and '.join(ref.authors)}}},\n"
            entry += f"  year = {{{ref.date[:4] if ref.date else ''}}},\n"

            if ref.doi:
                entry += f"  doi = {{{ref.doi}}},\n"
            if ref.url:
                entry += f"  url = {{{ref.url}}},\n"

            entry += "}\n"
            entries.append(entry)

        return '\n'.join(entries)

    def _map_item_type(self, zotero_type: str) -> str:
        """Map Zotero item type to BibTeX entry type"""
        mapping = {
            'journalArticle': 'article',
            'book': 'book',
            'bookSection': 'incollection',
            'conferencePaper': 'inproceedings',
            'thesis': 'phdthesis',
            'report': 'techreport',
            'webpage': 'misc',
        }
        return mapping.get(zotero_type, 'misc')

    def _escape_latex(self, text: str) -> str:
        """Escape special LaTeX characters"""
        if not text:
            return ''

        # Escape special characters
        replacements = [
            ('\\', r'\textbackslash{}'),
            ('&', r'\&'),
            ('%', r'\%'),
            ('$', r'\$'),
            ('#', r'\#'),
            ('_', r'\_'),
            ('{', r'\{'),
            ('}', r'\}'),
            ('~', r'\textasciitilde{}'),
            ('^', r'\textasciicircum{}'),
        ]

        for old, new in replacements:
            text = text.replace(old, new)

        return text

    async def compile_pdf(self, tex_path: Path) -> Optional[Path]:
        """Compile LaTeX to PDF using pdflatex"""
        try:
            # Run pdflatex twice for references
            for _ in range(2):
                result = subprocess.run(
                    ['pdflatex', '-interaction=nonstopmode', str(tex_path)],
                    cwd=tex_path.parent,
                    capture_output=True,
                    text=True
                )

            pdf_path = tex_path.with_suffix('.pdf')
            if pdf_path.exists():
                logger.info(f"Compiled PDF: {pdf_path}")
                return pdf_path
            else:
                logger.error("PDF compilation failed")
                return None

        except FileNotFoundError:
            logger.warning("pdflatex not installed - use Overleaf for compilation")
            return None


# ═══════════════════════════════════════════════════════════════════════════
# OVERLEAF INTEGRATION
# ═══════════════════════════════════════════════════════════════════════════

class OverleafClient:
    """
    Overleaf Git integration
    Uses Overleaf's Git bridge for pushing/pulling documents
    """

    def __init__(self, project_url: str = None):
        self.project_url = project_url or os.getenv('OVERLEAF_PROJECT_URL')
        self.local_path = None

    async def clone_project(self, local_path: str) -> bool:
        """Clone Overleaf project via Git"""
        if not self.project_url:
            logger.error("No Overleaf project URL configured")
            return False

        try:
            self.local_path = Path(local_path)
            result = subprocess.run(
                ['git', 'clone', self.project_url, str(self.local_path)],
                capture_output=True,
                text=True
            )
            return result.returncode == 0
        except Exception as e:
            logger.error(f"Overleaf clone failed: {e}")
            return False

    async def push_document(self, tex_path: Path, bib_path: Path = None) -> bool:
        """Push document to Overleaf via Git"""
        if not self.local_path:
            logger.error("No local Overleaf path configured")
            return False

        try:
            # Copy files to Overleaf project
            import shutil
            shutil.copy(tex_path, self.local_path)
            if bib_path and bib_path.exists():
                shutil.copy(bib_path, self.local_path)

            # Git add, commit, push
            subprocess.run(['git', 'add', '.'], cwd=self.local_path)
            subprocess.run(
                ['git', 'commit', '-m', f'Auto-update: {tex_path.name}'],
                cwd=self.local_path
            )
            result = subprocess.run(['git', 'push'], cwd=self.local_path, capture_output=True)

            return result.returncode == 0

        except Exception as e:
            logger.error(f"Overleaf push failed: {e}")
            return False


# ═══════════════════════════════════════════════════════════════════════════
# RFC-TO-DOCUMENT EXTRACTOR
# ═══════════════════════════════════════════════════════════════════════════

class RFCExtractor:
    """
    Extract structured content from RFC markdown files
    Converts to EADocument for LaTeX generation
    """

    def __init__(self, rfc_dir: str):
        self.rfc_dir = Path(rfc_dir)

    async def extract_rfc(self, rfc_path: Path) -> EADocument:
        """Extract RFC content into EADocument"""
        content = rfc_path.read_text(encoding='utf-8')

        # Extract metadata
        rfc_number = self._extract_pattern(r'RFC[- ]?(\d+)', content) or 'Unknown'
        title = self._extract_pattern(r'^#\s*(.+?)$', content, re.MULTILINE) or rfc_path.stem
        version = self._extract_pattern(r'\*\*Version:\*\*\s*(.+)', content) or '1.0'
        status = self._extract_pattern(r'\*\*Status:\*\*\s*(.+)', content) or 'Draft'
        date = self._extract_pattern(r'\*\*Date:\*\*\s*(.+)', content) or datetime.now().strftime('%B %Y')

        # Extract abstract
        abstract = self._extract_section(content, 'Abstract') or ''

        # Extract sections
        sections = self._extract_all_sections(content)

        return EADocument(
            doc_id=f"rfc-{rfc_number}",
            title=title,
            doc_type='rfc',
            version=version,
            authors=['CTAS Core Engineering Group'],
            date=date,
            abstract=abstract,
            sections=sections,
            references=[],
            metadata={
                'rfc_number': f'RFC-{rfc_number}',
                'status': status,
                'dependencies': self._extract_pattern(r'\*\*Dependencies:\*\*\s*(.+)', content) or ''
            }
        )

    def _extract_pattern(self, pattern: str, content: str, flags: int = 0) -> Optional[str]:
        """Extract first match of pattern"""
        match = re.search(pattern, content, flags)
        return match.group(1).strip() if match else None

    def _extract_section(self, content: str, section_name: str) -> Optional[str]:
        """Extract content of a named section"""
        pattern = rf'##\s*\d*\.?\s*{section_name}\s*\n(.*?)(?=\n##|\Z)'
        match = re.search(pattern, content, re.DOTALL | re.IGNORECASE)
        return match.group(1).strip() if match else None

    def _extract_all_sections(self, content: str) -> List[DocumentSection]:
        """Extract all sections from markdown"""
        sections = []
        current_section = None
        current_content = []

        for line in content.split('\n'):
            if line.startswith('## '):
                if current_section:
                    sections.append(DocumentSection(
                        title=current_section,
                        content='\n'.join(current_content)
                    ))
                current_section = line.lstrip('# ').strip()
                current_content = []
            elif current_section:
                current_content.append(line)

        if current_section:
            sections.append(DocumentSection(
                title=current_section,
                content='\n'.join(current_content)
            ))

        return sections


# ═══════════════════════════════════════════════════════════════════════════
# LINEAR INTEGRATION FOR DOCUMENT TRACKING
# ═══════════════════════════════════════════════════════════════════════════

class LinearDocumentTracker:
    """
    Track document generation and review in Linear
    Creates issues for document reviews, approvals
    """

    def __init__(self, api_key: str = None):
        self.api_key = api_key or os.getenv('LINEAR_API_KEY')
        self.base_url = "https://api.linear.app/graphql"

    async def create_document_issue(self, doc: EADocument, project_id: str = None) -> Optional[str]:
        """Create Linear issue for document tracking"""
        if not self.api_key:
            logger.warning("No Linear API key configured")
            return None

        try:
            import httpx

            mutation = """
            mutation CreateIssue($input: IssueCreateInput!) {
                issueCreate(input: $input) {
                    issue {
                        id
                        identifier
                        url
                    }
                }
            }
            """

            variables = {
                "input": {
                    "title": f"[DOC] {doc.title}",
                    "description": f"**Document Type:** {doc.doc_type}\n**Version:** {doc.version}\n\n{doc.abstract}",
                    "teamId": project_id or os.getenv('LINEAR_TEAM_ID'),
                    "labelIds": []  # Add document label
                }
            }

            async with httpx.AsyncClient() as client:
                response = await client.post(
                    self.base_url,
                    json={"query": mutation, "variables": variables},
                    headers={
                        "Authorization": self.api_key,
                        "Content-Type": "application/json"
                    }
                )

                if response.status_code == 200:
                    data = response.json()
                    issue = data.get('data', {}).get('issueCreate', {}).get('issue', {})
                    logger.info(f"Created Linear issue: {issue.get('identifier')}")
                    return issue.get('id')

        except Exception as e:
            logger.error(f"Linear issue creation failed: {e}")

        return None


# ═══════════════════════════════════════════════════════════════════════════
# MAIN DOCUMENT PIPELINE
# ═══════════════════════════════════════════════════════════════════════════

class EADocumentPipeline:
    """
    Main pipeline for EA document generation

    Integrates:
    - RFC extraction
    - Zotero citations (RFC-9010)
    - LaTeX generation for Overleaf
    - Linear tracking
    """

    def __init__(self, repo_path: str):
        self.repo_path = Path(repo_path)
        self.output_dir = self.repo_path / "04-abe-iac" / "abe-qa-system" / "ea-documents" / "generated"
        self.output_dir.mkdir(parents=True, exist_ok=True)

        self.zotero = ZoteroClient()
        self.latex = LaTeXGenerator(str(self.output_dir))
        self.rfc_extractor = RFCExtractor(str(self.repo_path / "01-rfc"))
        self.linear = LinearDocumentTracker()

    async def initialize(self):
        """Initialize all clients"""
        await self.zotero.connect()

    async def generate_rfc_document(self, rfc_path: str) -> Path:
        """Generate LaTeX document from RFC"""
        rfc_file = Path(rfc_path)
        if not rfc_file.exists():
            rfc_file = self.repo_path / "01-rfc" / rfc_path

        # Extract RFC
        doc = await self.rfc_extractor.extract_rfc(rfc_file)

        # Search for related Zotero references
        search_terms = doc.title.split()[:3]  # First 3 words
        refs = await self.zotero.search_references(' '.join(search_terms), limit=10)
        doc.references = refs

        # Generate LaTeX
        tex_path = await self.latex.generate_document(doc)

        # Create Linear issue for tracking
        await self.linear.create_document_issue(doc)

        return tex_path

    async def generate_all_rfcs(self) -> List[Path]:
        """Generate LaTeX for all RFCs"""
        rfc_dir = self.repo_path / "01-rfc"
        rfc_files = list(rfc_dir.rglob("RFC-*.md"))

        generated = []
        for rfc_file in rfc_files:
            try:
                tex_path = await self.generate_rfc_document(str(rfc_file))
                generated.append(tex_path)
            except Exception as e:
                logger.error(f"Failed to generate {rfc_file}: {e}")

        logger.info(f"Generated {len(generated)} RFC documents")
        return generated

    async def generate_dod_devsecops_doc(
        self,
        title: str,
        sections: List[DocumentSection],
        classification: str = "UNCLASSIFIED"
    ) -> Path:
        """Generate DoD DevSecOps document"""

        # Search Zotero for relevant references
        refs = await self.zotero.search_references(f"DevSecOps {title}", limit=15)

        doc = EADocument(
            doc_id=f"dod-{title.lower().replace(' ', '-')}",
            title=title,
            doc_type='dod-devsecops',
            version='1.0',
            authors=['CTAS Core Engineering Group'],
            date=datetime.now().strftime('%B %d, %Y'),
            abstract=f"DoD DevSecOps document for {title}",
            sections=sections,
            references=refs,
            metadata={'classification': classification}
        )

        tex_path = await self.latex.generate_document(doc)
        await self.linear.create_document_issue(doc)

        return tex_path


# ═══════════════════════════════════════════════════════════════════════════
# CLI
# ═══════════════════════════════════════════════════════════════════════════

async def main():
    import sys

    repo_path = os.getenv(
        'CTAS7_REPO',
        '/Users/cp5337/Developer/ctas-7-shipyard-staging'
    )

    pipeline = EADocumentPipeline(repo_path)
    await pipeline.initialize()

    if len(sys.argv) > 1:
        command = sys.argv[1]

        if command == 'rfc' and len(sys.argv) > 2:
            rfc_path = sys.argv[2]
            tex_path = await pipeline.generate_rfc_document(rfc_path)
            print(f"Generated: {tex_path}")

        elif command == 'all-rfcs':
            paths = await pipeline.generate_all_rfcs()
            print(f"Generated {len(paths)} documents")

        elif command == 'zotero-search' and len(sys.argv) > 2:
            query = ' '.join(sys.argv[2:])
            refs = await pipeline.zotero.search_references(query)
            for ref in refs:
                print(f"- {ref.title} ({ref.date})")

        else:
            print("Commands:")
            print("  rfc <path>        - Generate LaTeX from RFC")
            print("  all-rfcs          - Generate all RFCs")
            print("  zotero-search <q> - Search Zotero library")
    else:
        # Default: show status
        print("EA Document Generator")
        print(f"  Repo: {repo_path}")
        print(f"  Output: {pipeline.output_dir}")
        print(f"  Zotero: {'Connected' if pipeline.zotero.zot else 'Not connected'}")


if __name__ == "__main__":
    asyncio.run(main())
