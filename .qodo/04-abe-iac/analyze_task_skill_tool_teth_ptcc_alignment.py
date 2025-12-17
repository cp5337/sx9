#!/usr/bin/env python3
"""
CTAS Task → Skill → Tool → TETH → PTCC Alignment Analysis
==========================================================

Analyzes alignment between:
- CTAS Tasks (166 tasks)
- Skills (from CTAS_SKILLS_MATRIX.md and CTAS_TASK_SKILL_MAPPING.md)
- Tools (Kali tools, threat intelligence tools)
- TETH (Topological Entropy Threat Heuristics)
- PTCC (Primitive Type Classification Code - 32 primitives)

Generates gap analysis and alignment recommendations.
"""

import os
import json
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
from collections import defaultdict
from dataclasses import dataclass, asdict
import csv

try:
    from supabase import create_client, Client
    SUPABASE_AVAILABLE = True
except ImportError:
    SUPABASE_AVAILABLE = False
    print("⚠️  supabase-py not installed. Install with: pip install supabase")

# PTCC 32 Primitives (RFC-9100)
PTCC_PRIMITIVES = {
    0: "CREATE", 1: "READ", 2: "UPDATE", 3: "DELETE",
    4: "CONNECT", 5: "DISCONNECT", 6: "SEND", 7: "RECEIVE",
    8: "TRANSFORM", 9: "VALIDATE", 10: "ROUTE", 11: "FILTER",
    12: "AUTHENTICATE", 13: "AUTHORIZE", 14: "ENCRYPT", 15: "DECRYPT",
    16: "LOCK", 17: "UNLOCK", 18: "ALLOCATE", 19: "DEALLOCATE",
    20: "BRANCH", 21: "LOOP", 22: "CALL", 23: "RETURN",
    24: "COORDINATE", 25: "SYNCHRONIZE", 26: "SIGNAL", 27: "WAIT",
    28: "CHECKPOINT", 29: "SAVE", 30: "TERMINATE", 31: "RESUME"
}

# Primitive Type → PTCC Category Mapping
PRIMITIVE_TYPE_TO_PTCC = {
    "Concept": [20, 21, 22, 23],  # BRANCH, LOOP, CALL, RETURN
    "Actor": [24, 25, 26, 27],   # COORDINATE, SYNCHRONIZE, SIGNAL, WAIT
    "Object": [8, 9, 4, 10, 11], # TRANSFORM, VALIDATE, CONNECT, ROUTE, FILTER
    "Event": [28, 29, 12, 13],   # CHECKPOINT, SAVE, AUTHENTICATE, AUTHORIZE
    "Attribute": [14, 15, 16, 17], # ENCRYPT, DECRYPT, LOCK, UNLOCK
    "Unclassified": [0, 1, 2, 3, 18, 19]  # CREATE, READ, UPDATE, DELETE, ALLOCATE, DEALLOCATE
}

# HD4 Phase → PTCC Affinity
HD4_PHASE_TO_PTCC = {
    "Hunt": [1, 4, 7, 20, 22],      # READ, CONNECT, RECEIVE, BRANCH, CALL
    "Detect": [9, 11, 12, 28],      # VALIDATE, FILTER, AUTHENTICATE, CHECKPOINT
    "Disrupt": [8, 10, 14, 16],     # TRANSFORM, ROUTE, ENCRYPT, LOCK
    "Disable": [3, 5, 19, 30],      # DELETE, DISCONNECT, DEALLOCATE, TERMINATE
    "Dominate": [2, 6, 24, 25]      # UPDATE, SEND, COORDINATE, SYNCHRONIZE
}

# Skill Categories from CTAS_TASK_SKILL_MAPPING.md
SKILL_CATEGORIES = {
    "reconnaissance": {
        "skills": ["skill-recon-001", "skill-recon-002", "skill-recon-003", "skill-recon-004"],
        "ptcc_primitives": [1, 4, 7, 9, 11],  # READ, CONNECT, RECEIVE, VALIDATE, FILTER
        "tools": ["nmap", "masscan", "netcat", "Shodan", "Maltego", "theHarvester", "Nuclei", "Nessus", "OpenVAS"],
        "hd4_phases": ["Hunt", "Detect"]
    },
    "exploitation": {
        "skills": ["skill-exploit-001", "skill-exploit-002", "skill-exploit-003", "skill-exploit-004"],
        "ptcc_primitives": [12, 1, 4, 10, 8, 9, 15],  # AUTHENTICATE, READ, CONNECT, ROUTE, TRANSFORM, VALIDATE, DECRYPT
        "tools": ["Mimikatz", "hashcat", "Hydra", "Metasploit", "Cobalt Strike", "Burp Suite", "sqlmap", "Aircrack-ng", "Wifite"],
        "hd4_phases": ["Disrupt"]
    },
    "humint": {
        "skills": ["skill-humint-001", "skill-humint-002", "skill-humint-003"],
        "ptcc_primitives": [24, 26, 6, 25],  # COORDINATE, SIGNAL, SEND, SYNCHRONIZE
        "tools": ["SET", "Gophish"],
        "hd4_phases": ["Hunt", "Detect"]
    },
    "evasion": {
        "skills": ["skill-evasion-001", "skill-evasion-002", "skill-evasion-003"],
        "ptcc_primitives": [14, 10, 3, 2],  # ENCRYPT, ROUTE, DELETE, UPDATE
        "tools": ["Tor", "proxychains", "shred", "wipe", "BleachBit"],
        "hd4_phases": ["Detect", "Dominate"]
    },
    "execution": {
        "skills": ["skill-exec-001", "skill-exec-002", "skill-exec-003"],
        "ptcc_primitives": [0, 6, 4, 24, 14],  # CREATE, SEND, CONNECT, COORDINATE, ENCRYPT
        "tools": ["msfvenom", "Cobalt Strike", "Empire", "DNS tunneling", "steganography"],
        "hd4_phases": ["Disable"]
    },
    "cyber_physical": {
        "skills": ["skill-cyberphys-001", "skill-cyberphys-002", "skill-cyberphys-003"],
        "ptcc_primitives": [4, 8, 6, 5],  # CONNECT, TRANSFORM, SEND, DISCONNECT
        "tools": ["Modbus tools", "PLCScan", "SDR tools", "HackRF"],
        "hd4_phases": ["Disable"]
    }
}

# TETH Algorithms
TETH_ALGORITHMS = {
    "TETH-Topological": {
        "algorithm_type": "topological_entropy_analysis",
        "entropy_calculation_method": "shannon_entropy",
        "complexity_threshold": 0.75
    },
    "TETH-Heuristic": {
        "algorithm_type": "threat_heuristic_scoring",
        "scoring_factors": ["skill_level", "ai_force_multiplier", "region_shielding", "entropy_h"]
    },
    "TETH-Behavioral": {
        "algorithm_type": "behavioral_entropy_modeling",
        "pattern_recognition": True,
        "behavioral_learning": True
    },
    "TETH-Predictive": {
        "algorithm_type": "predictive_threat_modeling",
        "prediction_horizon_days": 30,
        "uncertainty_quantification": True
    }
}


@dataclass
class TaskAlignment:
    """Alignment analysis for a single CTAS task."""
    task_id: str
    task_name: str
    hd4_phase: str
    primitive_type: str
    has_skills: bool
    has_tools: bool
    has_ptcc: bool
    has_teth: bool
    skill_ids: List[str]
    tool_names: List[str]
    ptcc_primitives: List[int]
    teth_algorithms: List[str]
    alignment_score: float
    gaps: List[str]


@dataclass
class AlignmentReport:
    """Overall alignment report."""
    total_tasks: int
    tasks_with_skills: int
    tasks_with_tools: int
    tasks_with_ptcc: int
    tasks_with_teth: int
    fully_aligned: int
    partially_aligned: int
    unaligned: int
    task_alignments: List[TaskAlignment]
    skill_coverage: Dict[str, int]
    tool_coverage: Dict[str, int]
    ptcc_coverage: Dict[int, int]
    teth_coverage: Dict[str, int]
    recommendations: List[str]


class CTASAlignmentAnalyzer:
    """Analyzes CTAS task alignment with skills, tools, TETH, and PTCC."""
    
    def __init__(self, supabase_url: Optional[str] = None, supabase_key: Optional[str] = None):
        self.supabase: Optional[Client] = None
        if SUPABASE_AVAILABLE and supabase_url and supabase_key:
            try:
                self.supabase = create_client(supabase_url, supabase_key)
                print("✅ Connected to Supabase")
            except Exception as e:
                print(f"⚠️  Failed to connect to Supabase: {e}")
        
        self.tasks: List[Dict] = []
        self.alignment_report: Optional[AlignmentReport] = None
    
    def load_tasks_from_supabase(self) -> List[Dict]:
        """Load CTAS tasks from Supabase."""
        if not self.supabase:
            print("⚠️  Supabase not available, skipping database load")
            return []
        
        try:
            response = self.supabase.table('ctas_tasks').select('*').execute()
            self.tasks = response.data
            print(f"✅ Loaded {len(self.tasks)} tasks from Supabase")
            return self.tasks
        except Exception as e:
            print(f"❌ Error loading tasks: {e}")
            return []
    
    def load_tasks_from_csv(self, csv_path: str) -> List[Dict]:
        """Load CTAS tasks from CSV file."""
        tasks = []
        try:
            with open(csv_path, 'r', encoding='utf-8') as f:
                reader = csv.DictReader(f)
                for row in reader:
                    tasks.append(row)
            self.tasks = tasks
            print(f"✅ Loaded {len(tasks)} tasks from CSV: {csv_path}")
            return tasks
        except Exception as e:
            print(f"❌ Error loading CSV: {e}")
            return []
    
    def analyze_task_alignment(self, task: Dict) -> TaskAlignment:
        """Analyze alignment for a single task."""
        task_id = task.get('task_id') or task.get('hash_id', 'unknown')
        task_name = task.get('task_name', 'Unknown')
        hd4_phase = task.get('hd4_phase', 'Unknown')
        primitive_type = task.get('primitive_type', 'Unknown')
        
        # Extract skills (from required_skills JSONB or skill_categories)
        skills_json = task.get('required_skills', [])
        if isinstance(skills_json, str):
            try:
                skills_json = json.loads(skills_json)
            except:
                skills_json = []
        skill_ids = skills_json if isinstance(skills_json, list) else []
        
        # Extract tools (from tools JSONB or kali_tools)
        tools_json = task.get('tools', [])
        if isinstance(tools_json, str):
            try:
                tools_json = json.loads(tools_json)
            except:
                tools_json = []
        tool_names = tools_json if isinstance(tools_json, list) else []
        
        # Also check kali_tools field
        kali_tools = task.get('kali_tools', [])
        if isinstance(kali_tools, str):
            try:
                kali_tools = json.loads(kali_tools)
            except:
                kali_tools = []
        if isinstance(kali_tools, list):
            tool_names.extend(kali_tools)
        
        # Extract PTCC (from ptcc_primitive_id or ptcc_primitive_name)
        ptcc_id = task.get('ptcc_primitive_id')
        ptcc_name = task.get('ptcc_primitive_name', '')
        ptcc_primitives = []
        if ptcc_id is not None:
            ptcc_primitives.append(ptcc_id)
        elif ptcc_name:
            # Find PTCC code from name
            for code, name in PTCC_PRIMITIVES.items():
                if name == ptcc_name:
                    ptcc_primitives.append(code)
                    break
        
        # Infer PTCC from primitive_type and hd4_phase if missing
        if not ptcc_primitives:
            if primitive_type in PRIMITIVE_TYPE_TO_PTCC:
                ptcc_primitives = PRIMITIVE_TYPE_TO_PTCC[primitive_type]
            if hd4_phase in HD4_PHASE_TO_PTCC:
                ptcc_primitives.extend(HD4_PHASE_TO_PTCC[hd4_phase])
        
        # Check for TETH (from teth_analysis or entropy_h)
        has_teth = bool(task.get('teth_analysis') or task.get('entropy_h') is not None)
        teth_algorithms = []
        if has_teth:
            # Determine which TETH algorithms apply
            if task.get('entropy_h') is not None:
                teth_algorithms.append("TETH-Topological")
            if task.get('threat_heuristic_score'):
                teth_algorithms.append("TETH-Heuristic")
            if task.get('behavioral_entropy'):
                teth_algorithms.append("TETH-Behavioral")
            if task.get('prediction_confidence'):
                teth_algorithms.append("TETH-Predictive")
        
        # Calculate alignment score
        has_skills = len(skill_ids) > 0
        has_tools = len(tool_names) > 0
        has_ptcc = len(ptcc_primitives) > 0
        
        score = 0.0
        if has_skills:
            score += 0.25
        if has_tools:
            score += 0.25
        if has_ptcc:
            score += 0.25
        if has_teth:
            score += 0.25
        
        # Identify gaps
        gaps = []
        if not has_skills:
            gaps.append("Missing skills")
        if not has_tools:
            gaps.append("Missing tools")
        if not has_ptcc:
            gaps.append("Missing PTCC mapping")
        if not has_teth:
            gaps.append("Missing TETH analysis")
        
        return TaskAlignment(
            task_id=task_id,
            task_name=task_name,
            hd4_phase=hd4_phase,
            primitive_type=primitive_type,
            has_skills=has_skills,
            has_tools=has_tools,
            has_ptcc=has_ptcc,
            has_teth=has_teth,
            skill_ids=skill_ids,
            tool_names=tool_names,
            ptcc_primitives=ptcc_primitives,
            teth_algorithms=teth_algorithms,
            alignment_score=score,
            gaps=gaps
        )
    
    def generate_report(self) -> AlignmentReport:
        """Generate comprehensive alignment report."""
        if not self.tasks:
            print("⚠️  No tasks loaded. Load tasks first.")
            return None
        
        task_alignments = []
        skill_coverage = defaultdict(int)
        tool_coverage = defaultdict(int)
        ptcc_coverage = defaultdict(int)
        teth_coverage = defaultdict(int)
        
        for task in self.tasks:
            alignment = self.analyze_task_alignment(task)
            task_alignments.append(alignment)
            
            # Update coverage metrics
            for skill_id in alignment.skill_ids:
                skill_coverage[skill_id] += 1
            for tool_name in alignment.tool_names:
                tool_coverage[tool_name] += 1
            for ptcc_code in alignment.ptcc_primitives:
                ptcc_coverage[ptcc_code] += 1
            for teth_alg in alignment.teth_algorithms:
                teth_coverage[teth_alg] += 1
        
        # Calculate summary statistics
        total_tasks = len(task_alignments)
        tasks_with_skills = sum(1 for a in task_alignments if a.has_skills)
        tasks_with_tools = sum(1 for a in task_alignments if a.has_tools)
        tasks_with_ptcc = sum(1 for a in task_alignments if a.has_ptcc)
        tasks_with_teth = sum(1 for a in task_alignments if a.has_teth)
        fully_aligned = sum(1 for a in task_alignments if a.alignment_score == 1.0)
        partially_aligned = sum(1 for a in task_alignments if 0 < a.alignment_score < 1.0)
        unaligned = sum(1 for a in task_alignments if a.alignment_score == 0.0)
        
        # Generate recommendations
        recommendations = self._generate_recommendations(
            task_alignments, skill_coverage, tool_coverage, ptcc_coverage, teth_coverage
        )
        
        self.alignment_report = AlignmentReport(
            total_tasks=total_tasks,
            tasks_with_skills=tasks_with_skills,
            tasks_with_tools=tasks_with_tools,
            tasks_with_ptcc=tasks_with_ptcc,
            tasks_with_teth=tasks_with_teth,
            fully_aligned=fully_aligned,
            partially_aligned=partially_aligned,
            unaligned=unaligned,
            task_alignments=task_alignments,
            skill_coverage=dict(skill_coverage),
            tool_coverage=dict(tool_coverage),
            ptcc_coverage=dict(ptcc_coverage),
            teth_coverage=dict(teth_coverage),
            recommendations=recommendations
        )
        
        return self.alignment_report
    
    def _generate_recommendations(
        self,
        task_alignments: List[TaskAlignment],
        skill_coverage: Dict[str, int],
        tool_coverage: Dict[str, int],
        ptcc_coverage: Dict[int, int],
        teth_coverage: Dict[str, int]
    ) -> List[str]:
        """Generate alignment recommendations."""
        recommendations = []
        
        # Skills recommendations
        unaligned_tasks = [a for a in task_alignments if not a.has_skills]
        if unaligned_tasks:
            recommendations.append(
                f"⚠️  {len(unaligned_tasks)} tasks missing skills. "
                f"Map tasks to skill categories based on HD4 phase and primitive type."
            )
        
        # Tools recommendations
        unaligned_tasks = [a for a in task_alignments if not a.has_tools]
        if unaligned_tasks:
            recommendations.append(
                f"⚠️  {len(unaligned_tasks)} tasks missing tools. "
                f"Map Kali tools and threat intelligence tools to tasks based on skill requirements."
            )
        
        # PTCC recommendations
        unaligned_tasks = [a for a in task_alignments if not a.has_ptcc]
        if unaligned_tasks:
            recommendations.append(
                f"⚠️  {len(unaligned_tasks)} tasks missing PTCC mapping. "
                f"Infer PTCC primitives from primitive_type and HD4 phase."
            )
        
        # TETH recommendations
        unaligned_tasks = [a for a in task_alignments if not a.has_teth]
        if unaligned_tasks:
            recommendations.append(
                f"⚠️  {len(unaligned_tasks)} tasks missing TETH analysis. "
                f"Run TETH algorithms (Topological, Heuristic, Behavioral, Predictive) for all tasks."
            )
        
        # Coverage recommendations
        if len(skill_coverage) < 20:
            recommendations.append(
                f"⚠️  Only {len(skill_coverage)} unique skills mapped. "
                f"Expected ~20+ skills from CTAS_SKILLS_MATRIX.md."
            )
        
        if len(ptcc_coverage) < 32:
            recommendations.append(
                f"⚠️  Only {len(ptcc_coverage)} PTCC primitives used. "
                f"Expected all 32 primitives to be mapped."
            )
        
        if len(teth_coverage) < 4:
            recommendations.append(
                f"⚠️  Only {len(teth_coverage)} TETH algorithms applied. "
                f"Expected all 4 TETH algorithms (Topological, Heuristic, Behavioral, Predictive)."
            )
        
        return recommendations
    
    def print_report(self):
        """Print alignment report to console."""
        if not self.alignment_report:
            print("⚠️  No report generated. Run generate_report() first.")
            return
        
        report = self.alignment_report
        
        print("\n" + "="*80)
        print("CTAS TASK → SKILL → TOOL → TETH → PTCC ALIGNMENT REPORT")
        print("="*80)
        
        print(f"\n📊 SUMMARY STATISTICS")
        print(f"   Total Tasks: {report.total_tasks}")
        print(f"   Tasks with Skills: {report.tasks_with_skills} ({report.tasks_with_skills/report.total_tasks*100:.1f}%)")
        print(f"   Tasks with Tools: {report.tasks_with_tools} ({report.tasks_with_tools/report.total_tasks*100:.1f}%)")
        print(f"   Tasks with PTCC: {report.tasks_with_ptcc} ({report.tasks_with_ptcc/report.total_tasks*100:.1f}%)")
        print(f"   Tasks with TETH: {report.tasks_with_teth} ({report.tasks_with_teth/report.total_tasks*100:.1f}%)")
        print(f"\n   Fully Aligned: {report.fully_aligned} ({report.fully_aligned/report.total_tasks*100:.1f}%)")
        print(f"   Partially Aligned: {report.partially_aligned} ({report.partially_aligned/report.total_tasks*100:.1f}%)")
        print(f"   Unaligned: {report.unaligned} ({report.unaligned/report.total_tasks*100:.1f}%)")
        
        print(f"\n📈 COVERAGE METRICS")
        print(f"   Unique Skills: {len(report.skill_coverage)}")
        print(f"   Unique Tools: {len(report.tool_coverage)}")
        print(f"   PTCC Primitives Used: {len(report.ptcc_coverage)}/32")
        print(f"   TETH Algorithms Applied: {len(report.teth_coverage)}/4")
        
        print(f"\n🔧 RECOMMENDATIONS")
        for i, rec in enumerate(report.recommendations, 1):
            print(f"   {i}. {rec}")
        
        # Show top gaps by HD4 phase
        print(f"\n📋 GAPS BY HD4 PHASE")
        gaps_by_phase = defaultdict(lambda: {"skills": 0, "tools": 0, "ptcc": 0, "teth": 0})
        for alignment in report.task_alignments:
            phase = alignment.hd4_phase
            if not alignment.has_skills:
                gaps_by_phase[phase]["skills"] += 1
            if not alignment.has_tools:
                gaps_by_phase[phase]["tools"] += 1
            if not alignment.has_ptcc:
                gaps_by_phase[phase]["ptcc"] += 1
            if not alignment.has_teth:
                gaps_by_phase[phase]["teth"] += 1
        
        for phase, gaps in gaps_by_phase.items():
            print(f"   {phase}:")
            print(f"      Missing Skills: {gaps['skills']}")
            print(f"      Missing Tools: {gaps['tools']}")
            print(f"      Missing PTCC: {gaps['ptcc']}")
            print(f"      Missing TETH: {gaps['teth']}")
        
        print("\n" + "="*80)
    
    def save_report_json(self, output_path: str):
        """Save report as JSON."""
        if not self.alignment_report:
            print("⚠️  No report generated. Run generate_report() first.")
            return
        
        report_dict = {
            "summary": {
                "total_tasks": self.alignment_report.total_tasks,
                "tasks_with_skills": self.alignment_report.tasks_with_skills,
                "tasks_with_tools": self.alignment_report.tasks_with_tools,
                "tasks_with_ptcc": self.alignment_report.tasks_with_ptcc,
                "tasks_with_teth": self.alignment_report.tasks_with_teth,
                "fully_aligned": self.alignment_report.fully_aligned,
                "partially_aligned": self.alignment_report.partially_aligned,
                "unaligned": self.alignment_report.unaligned
            },
            "coverage": {
                "skill_coverage": self.alignment_report.skill_coverage,
                "tool_coverage": self.alignment_report.tool_coverage,
                "ptcc_coverage": {str(k): v for k, v in self.alignment_report.ptcc_coverage.items()},
                "teth_coverage": self.alignment_report.teth_coverage
            },
            "recommendations": self.alignment_report.recommendations,
            "task_alignments": [asdict(ta) for ta in self.alignment_report.task_alignments]
        }
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(report_dict, f, indent=2, ensure_ascii=False)
        
        print(f"✅ Report saved to: {output_path}")


def main():
    """Main execution."""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Analyze CTAS task alignment with skills, tools, TETH, and PTCC"
    )
    parser.add_argument(
        '--supabase-url',
        help='Supabase URL',
        default=os.getenv('SUPABASE_URL')
    )
    parser.add_argument(
        '--supabase-key',
        help='Supabase key',
        default=os.getenv('SUPABASE_KEY')
    )
    parser.add_argument(
        '--csv',
        help='Path to CTAS tasks CSV file',
        default='ctas_tasks_with_primitive_type.csv'
    )
    parser.add_argument(
        '--output',
        help='Output JSON report path',
        default='task_skill_tool_teth_ptcc_alignment_report.json'
    )
    
    args = parser.parse_args()
    
    analyzer = CTASAlignmentAnalyzer(args.supabase_url, args.supabase_key)
    
    # Try to load from Supabase first, fallback to CSV
    tasks_loaded = False
    if analyzer.supabase:
        tasks = analyzer.load_tasks_from_supabase()
        if tasks:
            tasks_loaded = True
    
    if not tasks_loaded:
        csv_path = Path(__file__).parent.parent / args.csv
        if csv_path.exists():
            analyzer.load_tasks_from_csv(str(csv_path))
        else:
            print(f"❌ CSV file not found: {csv_path}")
            print("   Please provide --csv path or set SUPABASE_URL and SUPABASE_KEY")
            sys.exit(1)
    
    # Generate and print report
    report = analyzer.generate_report()
    if report:
        analyzer.print_report()
        
        # Save JSON report
        output_path = Path(__file__).parent / args.output
        analyzer.save_report_json(str(output_path))


if __name__ == '__main__':
    main()



