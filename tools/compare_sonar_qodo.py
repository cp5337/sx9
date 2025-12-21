import json
from pathlib import Path
from dataclasses import dataclass

@dataclass
class Issue:
    tool: str
    file: str
    line: int
    message: str
    severity: str

def load_sonar(path):
    with open(path) as f:
        data = json.load(f)
    return [Issue("Sonar", i['file'], i['line'], i['message'], i['severity']) for i in data]

def load_qodo(path):
    with open(path) as f:
        data = json.load(f)
    return [Issue("Qodo", i['file'], i['line'], i['message'], i['severity']) for i in data]

def main():
    sonar_path = Path("sonar_normalized.json")
    qodo_path = Path("qodo_results.json")
    
    if not sonar_path.exists() or not qodo_path.exists():
        print("Missing input files.")
        return

    sonar_issues = load_sonar(sonar_path)
    qodo_issues = load_qodo(qodo_path)
    
    print(f"Loaded {len(sonar_issues)} Sonar issues and {len(qodo_issues)} Qodo issues.\n")
    
    print("## Overlap Analysis (Sonar findings within +/- 5 lines of Qodo findings)")
    
    for q in qodo_issues:
        print(f"\n🔍 Qodo Issue: {q.file}:{q.line} [{q.severity}]")
        print(f"   \"{q.message}\"")
        
        # Find Sonar matches
        matches = [s for s in sonar_issues if s.file == q.file and abs(s.line - q.line) <= 5]
        
        if matches:
            print("   ✅ Sonar Overlap Found:")
            for m in matches:
                print(f"      - Sonar {m.file}:{m.line} [{m.severity}] -> {m.message}")
        else:
            print("   ❌ No Sonar Overlap (AI caught something unique)")

if __name__ == "__main__":
    main()
