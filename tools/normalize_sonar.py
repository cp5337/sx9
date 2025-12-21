#!/usr/bin/env python3
"""
normalize_sonar.py

A utility script to parse unstructured text dumps from SonarQube (copy-pasted from the Issue UI)
and convert them into a normalized JSON format for analysis or agent ingestion.

Usage:
    python3 normalize_sonar.py <input_text_file> <output_json_file>
"""

import sys
import json
import re
from pathlib import Path
from dataclasses import dataclass, asdict
from typing import List, Dict, Any

@dataclass
class NormalizedIssue:
    file: str
    line: int
    column: int
    message: str
    severity: str
    type: str
    tool: str = "SonarQube"
    rule_id: str = "sonar-issue"

class SonarParser:
    def parse(self, file_path: Path) -> List[NormalizedIssue]:
        issues = []
        try:
            content = file_path.read_text(encoding='utf-8', errors='ignore')
        except Exception as e:
            print(f"Error reading file {file_path}: {e}")
            return []
        
        lines = content.splitlines()
        current_file = "unknown"
        
        # State Machine parsing logic
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            
            # Anchor: "Open" status line often precedes the issue block
            if line == "Open":
                # Look ahead for Line Number "L123"
                # Pattern roughly: Open -> [User/System info] -> L<Num>
                try:
                    line_num_idx = -1
                    # Look ahead up to 6 lines to find "L123"
                    for offset in range(1, 7):
                        if i + offset < len(lines):
                            candidate = lines[i+offset].strip()
                            if re.match(r'^L\d+$', candidate):
                                line_num_idx = i + offset
                                break
                    
                    if line_num_idx != -1:
                        line_str = lines[line_num_idx].strip()
                        line_num = int(line_str[1:])
                        
                        # Layout assumption based on copy-paste:
                        # L61 (line_num_idx)
                        # 1min effort
                        # 6 days ago
                        # Code Smell (type)
                        # Major (severity)
                        # Message content...
                        
                        type_idx = line_num_idx + 3
                        severity_idx = line_num_idx + 4
                        msg_idx = line_num_idx + 5
                        
                        if msg_idx < len(lines):
                            issue_type = lines[type_idx].strip()
                            # Validation: Ensure type is a known Sonar type
                            if issue_type not in ["Code Smell", "Bug", "Vulnerability", "Security Hotspot"]:
                                # Fallback or skip? Sometimes offsets vary.
                                # For now accept, but keep eye on it.
                                pass
                                
                            severity = lines[severity_idx].strip()
                            message = lines[msg_idx].strip()
                            
                            # Handle Filename updates
                            # Sometimes the filename line appears right before the message in the stream if it changed
                            # e.g. "cp5337graph-dbsrc/..."
                            # Heuristic: if 'message' looks like a path, use it as file and take next line as message
                            if "cp5337" in message and ("/" in message or "\\" in message):
                                raw_path = message
                                # Clean path
                                if "cp5337graph-dbsrc/" in raw_path:
                                    current_file = raw_path.replace("cp5337graph-dbsrc/", "src/")
                                elif "cp5337" in raw_path:
                                    # Fallback clean
                                    parts = raw_path.split("cp5337")[-1]
                                    current_file = parts.lstrip("/")
                                
                                # Advance message pointer
                                if msg_idx + 1 < len(lines):
                                    message = lines[msg_idx + 1].strip()
                            
                            issues.append(NormalizedIssue(
                                file=current_file,
                                line=line_num,
                                column=0, # Not available in text dump
                                message=message,
                                severity=severity,
                                type=issue_type
                            ))
                            
                            # Move index past this block to avoid re-matching
                            i = msg_idx 
                except Exception as e:
                    # Continue gracefully on parse error for one item
                    pass
            
            i += 1
            
        return issues

def main():
    if len(sys.argv) < 3:
        print("Usage: python3 normalize_sonar.py <input_txt> <output_json>")
        sys.exit(1)
        
    input_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2])
    
    if not input_file.exists():
        print(f"Input file not found: {input_file}")
        sys.exit(1)
        
    print(f"Normalizing {input_file}...")
    parser = SonarParser()
    issues = parser.parse(input_file)
    
    print(f"Found {len(issues)} issues.")
    
    # Write JSON
    with open(output_file, 'w') as f:
        # Check if we should dump full dicts
        data = [asdict(issue) for issue in issues]
        json.dump(data, f, indent=2)
        
    print(f"Normalized data saved to {output_file}")

if __name__ == "__main__":
    main()
