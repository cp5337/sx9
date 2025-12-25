# RFC-9121: Lightning QA — Deterministic Code Quality Grading

**Status:** Active  
**Author:** CTAS Core Engineering Group  
**Date:** 2025-12-24  
**Dependencies:** RFC-9120

---

## 1. Purpose

Lightning QA provides deterministic, reproducible code quality assessment for the Forge pipeline. It:
- Parses Rust AST using `syn`
- Calculates complexity metrics
- Detects anti-patterns (TETH-based)
- Assigns letter grades (A-F)
- Generates refactor directives

---

## 2. Grading Scale

| Grade | Score | Meaning |
|-------|-------|---------|
| **A** | 85-100 | Production ready |
| **B** | 70-84 | Minor issues, acceptable |
| **C** | 55-69 | Needs refactoring |
| **D** | 40-54 | Significant issues |
| **F** | 0-39 | Reject, manual review |

---

## 3. Dimensions

### 3.1 Structure (25%)

| Metric | Weight | Target |
|--------|--------|--------|
| File organization | 30% | Follows N-V-N-N |
| Module coherence | 30% | Single responsibility |
| Import cleanliness | 20% | No circular deps |
| Naming conventions | 20% | snake_case, descriptive |

### 3.2 Complexity (25%)

| Metric | Weight | Target |
|--------|--------|--------|
| Cyclomatic complexity | 40% | < 10 per function |
| Nesting depth | 30% | < 4 levels |
| Function length | 30% | < 50 lines |

### 3.3 Pattern Compliance (25%)

| Metric | Weight | Target |
|--------|--------|--------|
| Canonical pattern match | 50% | > 0.7 confidence |
| N-V-N-N header present | 30% | Required |
| TETH anti-pattern score | 20% | < 0.3 |

### 3.4 Architecture (25%)

| Metric | Weight | Target |
|--------|--------|--------|
| ECS backend correct | 40% | Legion, not Bevy |
| TCR types used | 30% | Foundation types |
| Layer boundaries | 30% | No violations |

---

## 4. Anti-Patterns (TETH)

| Pattern | Penalty | Description |
|---------|---------|-------------|
| `use bevy::` | -50 | Forbidden ECS |
| Local `Rune` type | -30 | Must use foundation |
| `unwrap()` chains | -10 | Error handling |
| Magic numbers | -5 | Use constants |
| Dead code | -5 | Remove unused |
| TODO/FIXME | -2 | Address before ship |

---

## 5. Report Schema

```json
{
  "$schema": "https://sx9.synaptix.io/schemas/qa-report.json",
  "crate": "sx9-example",
  "timestamp": "2025-12-24T12:00:00Z",
  "grade": "B",
  "score": 78,
  "dimensions": {
    "structure": { "score": 82, "issues": [] },
    "complexity": { "score": 75, "issues": ["fn_too_long:process_data"] },
    "pattern": { "score": 80, "issues": [] },
    "architecture": { "score": 74, "issues": ["missing_tcr:CustomType"] }
  },
  "refactor_directives": [
    {
      "file": "src/processor.rs",
      "line": 45,
      "issue": "function_too_long",
      "directive": "Split process_data into smaller functions"
    }
  ]
}
```

---

## 6. Python Implementation

```python
#!/usr/bin/env python3
"""Lightning QA - Static Analysis Gate"""

import json
import subprocess
from pathlib import Path
from dataclasses import dataclass, asdict

@dataclass
class QAResult:
    crate: str
    grade: str
    score: int
    dimensions: dict
    refactor_directives: list

def analyze_crate(crate_path: Path) -> QAResult:
    """Run full QA analysis on a Rust crate."""
    
    # Run cargo check first
    result = subprocess.run(
        ["cargo", "check", "--message-format=json"],
        cwd=crate_path,
        capture_output=True,
        text=True
    )
    
    # Parse for errors
    errors = [
        json.loads(line) 
        for line in result.stdout.splitlines() 
        if line.strip()
    ]
    
    # Calculate dimensions
    structure_score = analyze_structure(crate_path)
    complexity_score = analyze_complexity(crate_path)
    pattern_score = analyze_patterns(crate_path)
    arch_score = analyze_architecture(crate_path)
    
    # Weighted average
    total_score = (
        structure_score * 0.25 +
        complexity_score * 0.25 +
        pattern_score * 0.25 +
        arch_score * 0.25
    )
    
    # Grade assignment
    if total_score >= 85: grade = "A"
    elif total_score >= 70: grade = "B"
    elif total_score >= 55: grade = "C"
    elif total_score >= 40: grade = "D"
    else: grade = "F"
    
    return QAResult(
        crate=crate_path.name,
        grade=grade,
        score=int(total_score),
        dimensions={
            "structure": {"score": structure_score, "issues": []},
            "complexity": {"score": complexity_score, "issues": []},
            "pattern": {"score": pattern_score, "issues": []},
            "architecture": {"score": arch_score, "issues": []},
        },
        refactor_directives=[]
    )

def analyze_structure(path: Path) -> float:
    """Analyze code structure."""
    score = 100
    
    # Check for proper module organization
    src_dir = path / "src"
    if not (src_dir / "lib.rs").exists() and not (src_dir / "main.rs").exists():
        score -= 20
    
    # Check naming conventions
    for rs_file in src_dir.rglob("*.rs"):
        if not rs_file.stem.islower():
            score -= 5
    
    return max(0, score)

def analyze_complexity(path: Path) -> float:
    """Analyze code complexity."""
    score = 100
    
    for rs_file in (path / "src").rglob("*.rs"):
        content = rs_file.read_text()
        lines = content.splitlines()
        
        # Check function length (rough heuristic)
        in_fn = False
        fn_lines = 0
        for line in lines:
            if "fn " in line and "{" in line:
                in_fn = True
                fn_lines = 0
            elif in_fn:
                fn_lines += 1
                if "}" in line and fn_lines > 50:
                    score -= 10
                    in_fn = False
    
    return max(0, score)

def analyze_patterns(path: Path) -> float:
    """Check for canonical pattern compliance."""
    score = 100
    
    # Check for N-V-N-N header
    for rs_file in (path / "src").rglob("*.rs"):
        content = rs_file.read_text()
        if "// N-V-N-N:" not in content and "/// N-V-N-N:" not in content:
            score -= 5
    
    return max(0, score)

def analyze_architecture(path: Path) -> float:
    """Check architecture compliance."""
    score = 100
    
    for rs_file in (path / "src").rglob("*.rs"):
        content = rs_file.read_text()
        
        # Check for forbidden Bevy
        if "use bevy::" in content:
            score -= 50
        
        # Check for local type shadowing
        if "struct Rune" in content or "struct Slot" in content:
            score -= 30
    
    return max(0, score)

if __name__ == "__main__":
    import sys
    crate_path = Path(sys.argv[1])
    result = analyze_crate(crate_path)
    print(json.dumps(asdict(result), indent=2))
```

---

## 7. Integration Points

| Tool | Input | Output |
|------|-------|--------|
| **static_gate.py** | Crate path | Structure + Complexity scores |
| **arch_gate.py** | Crate path | Architecture score |
| **pattern_gate.py** | Crate path + Canonical registry | Pattern score |
| **aggregator.py** | All gate outputs | Final QA report |

---

## 8. References

- RFC-9120: Prompt Forge v4
- RFC-9127: Architecture Compliance
- RFC-9130: Unified Forge Pipeline

---

**End of RFC-9121**
