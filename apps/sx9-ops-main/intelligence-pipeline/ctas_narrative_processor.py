#!/usr/bin/env python3
"""
CTAS Complete Narrative Processor
Processes the full adversarial narrative and integrates with task generation
"""

import re
import json
import csv
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class TaskNode:
    """Represents a single task node extracted from narrative"""
    task_id: str
    uuid: str
    task_name: str
    parent_task: Optional[str]
    description: str
    probability: float
    duration: str
    relationships: List[str]
    primitive_type: str
    hd4_phase: str
    supports_tasks: List[str]
    requires_tasks: List[str]
    follows_tasks: List[str]

class CTASNarrativeProcessor:
    """Processes the complete CTAS adversarial narrative"""
    
    def __init__(self, narrative_file: str, output_dir: str = "./processed_narrative"):
        self.narrative_file = Path(narrative_file)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        
        self.tasks: List[TaskNode] = []
        self.parent_tasks: Dict[str, TaskNode] = {}
        self.child_tasks: Dict[str, List[TaskNode]] = {}
        
        # HD4 phase mapping
        self.hd4_mapping = {
            "SCH001": "hunt",
            "SCH002": "hunt", 
            "SCH003": "hunt",
            "SCH004": "detect",
            "SCH005": "hunt",
            "SCH006": "detect",
            "SCH007": "disrupt",
            "SCH008": "disable",
            "SCH009": "disable",
            "SCH010": "dominate",
            "SCH011": "dominate"
        }
        
        # Primitive type mapping based on task patterns
        self.primitive_mapping = {
            "ideological": "ANALYZE",
            "planning": "ANALYZE", 
            "reconnaissance": "SENSE",
            "targeting": "SENSE",
            "security": "ENCODE",
            "resources": "ACT",
            "cyber": "ACT",
            "infiltration": "ACT",
            "execution": "ACT",
            "escape": "ORCHESTRATE",
            "aftermath": "MONITOR"
        }
    
    def load_narrative(self) -> str:
        """Load the complete narrative text"""
        try:
            with open(self.narrative_file, 'r', encoding='utf-8') as f:
                content = f.read()
            logger.info(f"Loaded narrative: {len(content)} characters")
            return content
        except Exception as e:
            logger.error(f"Failed to load narrative: {e}")
            return ""
    
    def extract_parent_tasks(self, narrative: str) -> List[TaskNode]:
        """Extract parent tasks (SCH001.000 through SCH011.000)"""
        parent_pattern = r'SCH(\d{3})\.000:\s*([^—]+)(?:—\s*([^\.]+))?'
        matches = re.findall(parent_pattern, narrative)
        
        parent_tasks = []
        for match in matches:
            sch_num, title, subtitle = match
            task_id = f"SCH{sch_num}.000"
            
            # Clean up title
            title = title.strip()
            if subtitle:
                full_title = f"{title} - {subtitle.strip()}"
            else:
                full_title = title
            
            # Determine HD4 phase
            hd4_phase = self.hd4_mapping.get(f"SCH{sch_num}", "hunt")
            
            # Determine primitive type from title
            primitive_type = self.determine_primitive_type(full_title)
            
            parent_task = TaskNode(
                task_id=task_id,
                uuid=f"uuid-{sch_num}-000-000",
                task_name=full_title,
                parent_task=None,
                description=f"Parent task: {full_title}",
                probability=0.85,  # Default probability
                duration="weeks to months",
                relationships=[],
                primitive_type=primitive_type,
                hd4_phase=hd4_phase,
                supports_tasks=[],
                requires_tasks=[],
                follows_tasks=[]
            )
            
            parent_tasks.append(parent_task)
            self.parent_tasks[task_id] = parent_task
        
        logger.info(f"Extracted {len(parent_tasks)} parent tasks")
        return parent_tasks
    
    def extract_child_tasks(self, narrative: str) -> List[TaskNode]:
        """Extract child tasks from the narrative"""
        # Pattern for child tasks: SCH001.001, SCH001.002, etc.
        child_pattern = r'\*\s*SCH(\d{3})\.(\d{3}):\s*([^(]+)\s*\(([^)]+)\):[^,]*,\s*([^,]*),\s*[^=]*P=([0-9.]+)'
        matches = re.findall(child_pattern, narrative)
        
        child_tasks = []
        for match in matches:
            parent_num, child_num, title, uuid, description, probability = match
            
            task_id = f"SCH{parent_num}.{child_num}"
            parent_id = f"SCH{parent_num}.000"
            
            # Clean up extracted data
            title = title.strip()
            description = description.strip()
            prob = float(probability)
            
            # Determine HD4 phase from parent
            hd4_phase = self.hd4_mapping.get(f"SCH{parent_num}", "hunt")
            
            # Determine primitive type
            primitive_type = self.determine_primitive_type(title)
            
            child_task = TaskNode(
                task_id=task_id,
                uuid=uuid.strip(),
                task_name=title,
                parent_task=parent_id,
                description=description,
                probability=prob,
                duration="days to weeks",
                relationships=[],
                primitive_type=primitive_type,
                hd4_phase=hd4_phase,
                supports_tasks=[],
                requires_tasks=[],
                follows_tasks=[]
            )
            
            child_tasks.append(child_task)
            
            # Group by parent
            if parent_id not in self.child_tasks:
                self.child_tasks[parent_id] = []
            self.child_tasks[parent_id].append(child_task)
        
        logger.info(f"Extracted {len(child_tasks)} child tasks")
        return child_tasks
    
    def determine_primitive_type(self, task_name: str) -> str:
        """Determine CTAS primitive type from task name"""
        name_lower = task_name.lower()
        
        # SENSE - Information gathering
        if any(keyword in name_lower for keyword in [
            'osint', 'reconnaissance', 'surveillance', 'collection', 'analysis',
            'targeting', 'intelligence', 'observation', 'monitoring'
        ]):
            return "SENSE"
        
        # ACT - Direct action  
        elif any(keyword in name_lower for keyword in [
            'execution', 'attack', 'infiltration', 'access', 'exploitation',
            'cyber operations', 'recruitment', 'acquisition', 'deployment'
        ]):
            return "ACT"
        
        # ENCODE - Data transformation/protection
        elif any(keyword in name_lower for keyword in [
            'security', 'encryption', 'anonymization', 'concealment', 
            'camouflage', 'protection', 'encoding'
        ]):
            return "ENCODE"
        
        # ANALYZE - Processing/planning
        elif any(keyword in name_lower for keyword in [
            'planning', 'formation', 'analysis', 'assessment', 'evaluation',
            'selection', 'identification'
        ]):
            return "ANALYZE"
        
        # ORCHESTRATE - Coordination
        elif any(keyword in name_lower for keyword in [
            'coordination', 'orchestration', 'management', 'initiation',
            'multi-actor', 'escape', 'evasion'
        ]):
            return "ORCHESTRATE"
        
        # MONITOR - Observation/tracking
        elif any(keyword in name_lower for keyword in [
            'monitoring', 'tracking', 'aftermath', 'observation'
        ]):
            return "MONITOR"
        
        else:
            return "ANALYZE"  # Default fallback
    
    def extract_relationships(self, narrative: str):
        """Extract task relationships from narrative"""
        # Look for relationship patterns
        supports_pattern = r'SUPPORTING\s+([A-Z0-9.]+)'
        requires_pattern = r'REQUIRES\s+([A-Z0-9.]+)'
        follows_pattern = r'FOLLOWS\s+([A-Z0-9.]+)'
        comes_before_pattern = r'COMES_BEFORE\s+([A-Z0-9.]+)'
        
        for task in self.tasks:
            # Find task context in narrative
            task_context = self.find_task_context(narrative, task.task_id)
            
            if task_context:
                # Extract relationships
                supports = re.findall(supports_pattern, task_context)
                requires = re.findall(requires_pattern, task_context)
                follows = re.findall(follows_pattern, task_context)
                comes_before = re.findall(comes_before_pattern, task_context)
                
                task.supports_tasks = supports
                task.requires_tasks = requires
                task.follows_tasks = follows + comes_before
    
    def find_task_context(self, narrative: str, task_id: str) -> str:
        """Find the context around a specific task ID in the narrative"""
        # Look for the task ID and extract surrounding context
        pattern = rf'{re.escape(task_id)}[^*]*?(?=\*|SCH\d{{3}}\.000:|$)'
        match = re.search(pattern, narrative, re.DOTALL)
        
        if match:
            return match.group(0)
        return ""
    
    def process_complete_narrative(self):
        """Process the complete narrative and extract all tasks"""
        logger.info("Processing complete CTAS narrative...")
        
        # Load narrative
        narrative = self.load_narrative()
        if not narrative:
            return
        
        # Extract parent tasks
        parent_tasks = self.extract_parent_tasks(narrative)
        self.tasks.extend(parent_tasks)
        
        # Extract child tasks
        child_tasks = self.extract_child_tasks(narrative)
        self.tasks.extend(child_tasks)
        
        # Extract relationships
        self.extract_relationships(narrative)
        
        logger.info(f"Processed {len(self.tasks)} total tasks")
    
    def export_to_csv(self):
        """Export processed tasks to CSV format"""
        csv_file = self.output_dir / "ctas_complete_tasks.csv"
        
        with open(csv_file, 'w', newline='', encoding='utf-8') as f:
            fieldnames = [
                'task_id', 'task_name', 'description', 'parent_task', 
                'primitive_type', 'hd4_phase', 'probability', 'duration',
                'supports_tasks', 'requires_tasks', 'follows_tasks'
            ]
            writer = csv.DictWriter(f, fieldnames=fieldnames)
            writer.writeheader()
            
            for task in self.tasks:
                row = {
                    'task_id': task.task_id,
                    'task_name': task.task_name,
                    'description': task.description,
                    'parent_task': task.parent_task or '',
                    'primitive_type': task.primitive_type,
                    'hd4_phase': task.hd4_phase,
                    'probability': task.probability,
                    'duration': task.duration,
                    'supports_tasks': '; '.join(task.supports_tasks),
                    'requires_tasks': '; '.join(task.requires_tasks),
                    'follows_tasks': '; '.join(task.follows_tasks)
                }
                writer.writerow(row)
        
        logger.info(f"Exported tasks to {csv_file}")
    
    def export_to_json(self):
        """Export processed tasks to JSON format"""
        json_file = self.output_dir / "ctas_complete_tasks.json"
        
        data = {
            "metadata": {
                "total_tasks": len(self.tasks),
                "parent_tasks": len(self.parent_tasks),
                "child_tasks": len(self.tasks) - len(self.parent_tasks),
                "primitive_distribution": self.get_primitive_distribution(),
                "hd4_distribution": self.get_hd4_distribution()
            },
            "tasks": [asdict(task) for task in self.tasks]
        }
        
        with open(json_file, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        
        logger.info(f"Exported tasks to {json_file}")
    
    def get_primitive_distribution(self) -> Dict[str, int]:
        """Get distribution of primitive types"""
        distribution = {}
        for task in self.tasks:
            primitive = task.primitive_type
            distribution[primitive] = distribution.get(primitive, 0) + 1
        return distribution
    
    def get_hd4_distribution(self) -> Dict[str, int]:
        """Get distribution of HD4 phases"""
        distribution = {}
        for task in self.tasks:
            phase = task.hd4_phase
            distribution[phase] = distribution.get(phase, 0) + 1
        return distribution
    
    def generate_interview_prompts(self):
        """Generate interview prompts for each task"""
        prompts_file = self.output_dir / "ctas_interview_prompts.txt"
        
        with open(prompts_file, 'w', encoding='utf-8') as f:
            f.write("# CTAS Complete Narrative Interview Prompts\n")
            f.write("# Generated from comprehensive adversarial narrative\n\n")
            
            for i, task in enumerate(self.tasks, 1):
                f.write(f"## PROMPT {i}: {task.task_id} - {task.task_name}\n")
                f.write("=" * 80 + "\n\n")
                
                prompt = self.create_task_prompt(task)
                f.write(prompt)
                f.write("\n\n" + "=" * 80 + "\n\n")
        
        logger.info(f"Generated interview prompts: {prompts_file}")
    
    def create_task_prompt(self, task: TaskNode) -> str:
        """Create interview prompt for a specific task"""
        context = ""
        if task.parent_task:
            context = f"This is a child task under parent task {task.parent_task}."
        
        return f"""
You are creating a first-person adversarial interview for CTAS based on the comprehensive threat narrative.

TASK DETAILS:
- Task ID: {task.task_id}
- Task Name: {task.task_name}
- Description: {task.description}
- Primitive Type: {task.primitive_type}
- HD4 Phase: {task.hd4_phase}
- Probability: {task.probability}
- Duration: {task.duration}
{context}

RELATIONSHIPS:
- Supports: {', '.join(task.supports_tasks) if task.supports_tasks else 'None'}
- Requires: {', '.join(task.requires_tasks) if task.requires_tasks else 'None'}
- Follows: {', '.join(task.follows_tasks) if task.follows_tasks else 'None'}

Create a comprehensive first-person adversarial interview that includes:

1. PRIMARY NARRATIVE (250-350 words):
   - First-person perspective as the threat actor conducting this task
   - Specific methods, tools, and techniques used
   - How this task fits into the broader operational sequence
   - Authentic adversarial mindset and motivation

2. CAPABILITIES (5-7 items):
   - Specific technical and operational capabilities
   - Tools and resources required
   - Sophistication levels and skill requirements

3. LIMITATIONS (3-5 items):
   - Operational constraints and vulnerabilities
   - Detection risks and mitigation challenges
   - Resource dependencies and bottlenecks

4. REAL-WORLD EXAMPLES:
   - Historical incidents demonstrating this task type
   - Current threat patterns and emerging techniques
   - Specific case studies with dates and relevance

Generate a realistic, technically accurate adversarial interview for {task.task_name}.
"""
    
    def generate_summary_report(self):
        """Generate summary report of processed narrative"""
        report_file = self.output_dir / "narrative_processing_report.md"
        
        primitive_dist = self.get_primitive_distribution()
        hd4_dist = self.get_hd4_distribution()
        
        report = f"""# CTAS Complete Narrative Processing Report

## Overview
- **Total Tasks Processed**: {len(self.tasks)}
- **Parent Tasks**: {len(self.parent_tasks)}
- **Child Tasks**: {len(self.tasks) - len(self.parent_tasks)}

## Primitive Type Distribution
"""
        for primitive, count in sorted(primitive_dist.items()):
            report += f"- **{primitive}**: {count} tasks\n"
        
        report += "\n## HD4 Phase Distribution\n"
        for phase, count in sorted(hd4_dist.items()):
            report += f"- **{phase}**: {count} tasks\n"
        
        report += f"""
## Parent Tasks Summary
"""
        for task_id, task in self.parent_tasks.items():
            child_count = len(self.child_tasks.get(task_id, []))
            report += f"- **{task_id}**: {task.task_name} ({child_count} child tasks)\n"
        
        with open(report_file, 'w', encoding='utf-8') as f:
            f.write(report)
        
        logger.info(f"Generated summary report: {report_file}")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Process CTAS complete narrative")
    parser.add_argument("narrative_file", help="Path to narrative text file")
    parser.add_argument("--output-dir", "-o", default="./processed_narrative")
    parser.add_argument("--generate-prompts", action="store_true", 
                       help="Generate interview prompts")
    
    args = parser.parse_args()
    
    processor = CTASNarrativeProcessor(args.narrative_file, args.output_dir)
    processor.process_complete_narrative()
    processor.export_to_csv()
    processor.export_to_json()
    processor.generate_summary_report()
    
    if args.generate_prompts:
        processor.generate_interview_prompts()

if __name__ == "__main__":
    main()
