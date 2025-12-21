#!/usr/bin/env python3
"""
RFC Dependency Validator
Validates RFC-INDEX.toml for consistency and generates dependency reports.
"""

import sys
import tomllib
import json
from pathlib import Path
from dataclasses import dataclass
from typing import Optional
import re


@dataclass
class ValidationError:
    rfc: str
    rule: str
    message: str
    severity: str  # error, warning


@dataclass
class ValidationResult:
    errors: list[ValidationError]
    warnings: list[ValidationError]
    
    @property
    def passed(self) -> bool:
        return len(self.errors) == 0


def load_index(path: Path) -> dict:
    """Load RFC-INDEX.toml"""
    with open(path, "rb") as f:
        return tomllib.load(f)


def validate_file_exists(index: dict, base_path: Path) -> list[ValidationError]:
    """Check that all RFC files exist"""
    errors = []
    
    for rfc_id, rfc in index.get("rfcs", {}).items():
        file_path = base_path / rfc.get("file", "")
        if not file_path.exists():
            errors.append(ValidationError(
                rfc=rfc_id,
                rule="file_exists",
                message=f"File not found: {file_path}",
                severity="error"
            ))
    
    return errors


def validate_dependencies(index: dict) -> list[ValidationError]:
    """Check that all dependencies are resolvable"""
    errors = []
    
    # Collect all known RFCs
    known_rfcs = set(index.get("rfcs", {}).keys())
    known_rfcs.update(index.get("external", {}).keys())
    
    # Check each RFC's dependencies
    for rfc_id, rfc in index.get("rfcs", {}).items():
        depends_on = rfc.get("depends_on", {})
        required = depends_on.get("required", [])
        optional = depends_on.get("optional", [])
        
        for dep in required:
            if dep not in known_rfcs:
                errors.append(ValidationError(
                    rfc=rfc_id,
                    rule="deps_resolvable",
                    message=f"Required dependency not found: {dep}",
                    severity="error"
                ))
        
        for dep in optional:
            if dep not in known_rfcs:
                errors.append(ValidationError(
                    rfc=rfc_id,
                    rule="deps_resolvable",
                    message=f"Optional dependency not found: {dep}",
                    severity="warning"
                ))
    
    return errors


def validate_nats_subjects(index: dict) -> list[ValidationError]:
    """Validate NATS subject naming conventions"""
    errors = []
    pattern = re.compile(index.get("validation", {}).get("rules", {}).get(
        "nats_subject_pattern", r"^sx9\.[a-z]+\..+"
    ))
    
    for rfc_id, rfc in index.get("rfcs", {}).items():
        subjects = rfc.get("nats_subjects", {})
        for name, subject in subjects.items():
            # Subject may have placeholders like {crate}
            test_subject = re.sub(r'\{[^}]+\}', 'placeholder', subject)
            if not pattern.match(test_subject):
                errors.append(ValidationError(
                    rfc=rfc_id,
                    rule="nats_subject_pattern",
                    message=f"Invalid NATS subject pattern: {subject}",
                    severity="warning"
                ))
    
    return errors


def validate_status(index: dict) -> list[ValidationError]:
    """Validate RFC status values"""
    errors = []
    valid_statuses = index.get("validation", {}).get("rules", {}).get(
        "valid_statuses", ["draft", "review", "stable", "deprecated"]
    )
    
    for rfc_id, rfc in index.get("rfcs", {}).items():
        status = rfc.get("status", "")
        if status not in valid_statuses:
            errors.append(ValidationError(
                rfc=rfc_id,
                rule="valid_status",
                message=f"Invalid status '{status}', must be one of: {valid_statuses}",
                severity="error"
            ))
    
    return errors


def detect_circular_dependencies(index: dict) -> list[ValidationError]:
    """Detect circular dependencies in the RFC graph"""
    errors = []
    
    # Build adjacency list
    deps = {}
    for rfc_id, rfc in index.get("rfcs", {}).items():
        required = rfc.get("depends_on", {}).get("required", [])
        deps[rfc_id] = [d for d in required if d.startswith("RFC-91")]  # Only internal deps
    
    # DFS to detect cycles
    visited = set()
    rec_stack = set()
    
    def dfs(node: str, path: list[str]) -> Optional[list[str]]:
        if node in rec_stack:
            # Found cycle
            cycle_start = path.index(node)
            return path[cycle_start:] + [node]
        
        if node in visited:
            return None
        
        visited.add(node)
        rec_stack.add(node)
        
        for neighbor in deps.get(node, []):
            cycle = dfs(neighbor, path + [node])
            if cycle:
                return cycle
        
        rec_stack.remove(node)
        return None
    
    for rfc_id in deps.keys():
        if rfc_id not in visited:
            cycle = dfs(rfc_id, [])
            if cycle:
                errors.append(ValidationError(
                    rfc=cycle[0],
                    rule="no_circular_deps",
                    message=f"Circular dependency detected: {' -> '.join(cycle)}",
                    severity="error"
                ))
    
    return errors


def generate_mermaid_graph(index: dict) -> str:
    """Generate Mermaid diagram of RFC dependencies"""
    lines = ["```mermaid", "graph TD"]
    
    # Add subgraphs for layers
    layers = {}
    for rfc_id, node in index.get("graph", {}).get("nodes", {}).items():
        layer = node.get("layer", "unknown")
        if layer not in layers:
            layers[layer] = []
        layers[layer].append(rfc_id)
    
    for layer, nodes in layers.items():
        lines.append(f"    subgraph {layer.title()}")
        for node in nodes:
            lines.append(f"        {node}")
        lines.append("    end")
    
    # Add edges
    for edge_id, edge in index.get("graph", {}).get("edges", {}).items():
        from_node = edge["from"]
        to_node = edge["to"]
        edge_type = edge.get("type", "requires")
        
        if edge_type == "requires":
            lines.append(f"    {from_node} --> {to_node}")
        else:
            lines.append(f"    {from_node} -.-> {to_node}")
    
    lines.append("```")
    return "\n".join(lines)


def generate_status_report(index: dict) -> str:
    """Generate implementation status report"""
    lines = ["# Implementation Status", ""]
    
    for rfc_id, status in index.get("status", {}).items():
        lines.append(f"## {rfc_id}")
        lines.append("")
        lines.append("| Component | Status |")
        lines.append("|-----------|--------|")
        
        for component, state in status.items():
            emoji = "✅" if state == "complete" else "⏳" if state == "in_progress" else "⬜"
            lines.append(f"| {component} | {emoji} {state} |")
        
        lines.append("")
    
    return "\n".join(lines)


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="RFC Dependency Validator")
    parser.add_argument("--index", default="RFC-INDEX.toml", help="Path to RFC-INDEX.toml")
    parser.add_argument("--format", choices=["text", "json"], default="text", help="Output format")
    parser.add_argument("--graph", action="store_true", help="Generate dependency graph")
    parser.add_argument("--status", action="store_true", help="Generate status report")
    
    args = parser.parse_args()
    
    index_path = Path(args.index)
    if not index_path.exists():
        print(f"Error: {index_path} not found", file=sys.stderr)
        sys.exit(1)
    
    index = load_index(index_path)
    base_path = index_path.parent
    
    if args.graph:
        print(generate_mermaid_graph(index))
        sys.exit(0)
    
    if args.status:
        print(generate_status_report(index))
        sys.exit(0)
    
    # Run validations
    all_errors = []
    all_errors.extend(validate_file_exists(index, base_path))
    all_errors.extend(validate_dependencies(index))
    all_errors.extend(validate_nats_subjects(index))
    all_errors.extend(validate_status(index))
    all_errors.extend(detect_circular_dependencies(index))
    
    errors = [e for e in all_errors if e.severity == "error"]
    warnings = [e for e in all_errors if e.severity == "warning"]
    
    result = ValidationResult(errors=errors, warnings=warnings)
    
    if args.format == "json":
        output = {
            "passed": result.passed,
            "errors": [{"rfc": e.rfc, "rule": e.rule, "message": e.message} for e in errors],
            "warnings": [{"rfc": e.rfc, "rule": e.rule, "message": e.message} for e in warnings],
        }
        print(json.dumps(output, indent=2))
    else:
        if errors:
            print("❌ Validation FAILED\n")
            print("Errors:")
            for e in errors:
                print(f"  [{e.rfc}] {e.rule}: {e.message}")
        
        if warnings:
            print("\nWarnings:")
            for w in warnings:
                print(f"  [{w.rfc}] {w.rule}: {w.message}")
        
        if result.passed:
            print("✅ Validation PASSED")
            if warnings:
                print(f"   ({len(warnings)} warnings)")
    
    sys.exit(0 if result.passed else 1)


if __name__ == "__main__":
    main()
