# SX9 OPTIMAL CLAUDE USAGE GUIDE
## Leveraging Claude's Full Capability Stack

```
═══════════════════════════════════════════════════════════════════════════════
                      CLAUDE CAPABILITY ARCHITECTURE
═══════════════════════════════════════════════════════════════════════════════

                              ┌─────────────────┐
                              │   USER PROMPT   │
                              └────────┬────────┘
                                       │
                    ┌──────────────────┼──────────────────┐
                    │                  │                  │
           ┌────────▼────────┐ ┌───────▼───────┐ ┌───────▼───────┐
           │  CONTEXT LAYER  │ │ TOOL LAYER    │ │ OUTPUT LAYER  │
           │                 │ │               │ │               │
           │ • Memory        │ │ • Web Search  │ │ • Artifacts   │
           │ • Past Chats    │ │ • Web Fetch   │ │ • Files       │
           │ • User Prefs    │ │ • Computer    │ │ • Linear      │
           │                 │ │ • MCP Servers │ │ • Canva       │
           └────────┬────────┘ └───────┬───────┘ └───────┬───────┘
                    │                  │                  │
                    └──────────────────┼──────────────────┘
                                       │
                              ┌────────▼────────┐
                              │  CLAUDE CORE    │
                              │  (Reasoning)    │
                              └────────┬────────┘
                                       │
                              ┌────────▼────────┐
                              │    RESPONSE     │
                              └─────────────────┘
```

---

## 1. AVAILABLE TOOLS & WHEN TO USE THEM

### Core Tools (Always Available)

| Tool | Purpose | Best For | Example |
|------|---------|----------|---------|
| **web_search** | Search current information | News, CVEs, API docs, recent changes | "Search for latest MITRE ATT&CK techniques" |
| **web_fetch** | Get full page content | Documentation pages, specifications | "Fetch the full RFC 9110 specification" |
| **computer_use** | Linux environment | Creating files, running code, testing | "Create a Python script and test it" |
| **memory** | Recall past context | Continuing work, user preferences | "What did we decide about the hash algorithm?" |

### MCP Servers (Connected)

| Server | Tools | Best For |
|--------|-------|----------|
| **Linear** | list_issues, create_issue, update_issue, list_projects, create_comment | Mission tracking, task breakdown, status updates |
| **Canva** | Design creation | Architecture diagrams, threat models, presentations |
| **Figma** | get_design_context, get_screenshot | UI implementation, design system reference |
| **Google Drive** | search, fetch | Internal documentation, historical context |
| **Hugging Face** | model_search, paper_search | ML research, model selection |
| **Vercel** | deploy, list_deployments | Frontend deployment, preview environments |
| **Filesystem** | read, write, edit, search | Local file operations |

---

## 2. OPTIMAL WORKFLOW PATTERNS

### Pattern A: Research Task

```yaml
workflow:
  1_context:
    - conversation_search: "Prior research on {topic}"
    - google_drive_search: "Internal docs about {topic}"
    
  2_gather:
    - web_search: "Current state of {topic}"
    - web_fetch: "Key sources for deep reading"
    - huggingface:paper_search: "Academic papers if ML-related"
    
  3_synthesize:
    - computer_use: "Create research report in markdown"
    
  4_track:
    - linear:create_issue: "Document findings, next steps"
    
  5_deliver:
    - present_files: "Output report to user"
```

### Pattern B: Build Task

```yaml
workflow:
  1_context:
    - linear:get_issue: "Understand requirements, acceptance criteria"
    - filesystem:list_directory: "Review existing code structure"
    - memory: "Recall prior decisions about this module"
    
  2_reference:
    - web_search: "API documentation, best practices"
    - figma:get_design_context: "If UI-related"
    
  3_implement:
    - computer_use: 
        - "Create files in /home/claude"
        - "Run tests"
        - "Iterate until passing"
        
  4_track:
    - linear:update_issue: "Progress update"
    - linear:create_comment: "Implementation notes"
    
  5_deliver:
    - present_files: "Output to /mnt/user-data/outputs"
    - linear:update_issue: "Mark complete, link artifacts"
```

### Pattern C: Security Audit

```yaml
workflow:
  1_scope:
    - linear:get_issue: "Audit scope and boundaries"
    - filesystem:list_directory: "Target structure"
    
  2_research:
    - web_search: "Known vulnerabilities for {tech stack}"
    - web_search: "CVEs for {dependencies}"
    
  3_analyze:
    - filesystem:read_file: "Review code (READ-ONLY)"
    - filesystem:search_files: "Find patterns of interest"
    
  4_document:
    - computer_use: "Create findings report"
    - canva: "Create threat model diagram"
    
  5_report:
    - linear:create_issue: "Finding per vulnerability"
    - linear:create_comment: "Summary on parent issue"
    - present_files: "Audit report"
```

---

## 3. CONTEXT LOADING BEST PRACTICES

### Always Start With Context

Before executing any significant task, load relevant context:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTEXT LOADING SEQUENCE                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. MEMORY FIRST                                                            │
│     └─ conversation_search("relevant keywords")                             │
│     └─ recent_chats(n=5) if continuing recent work                          │
│                                                                             │
│  2. LINEAR FOR STATE                                                        │
│     └─ list_issues(query="related to task")                                 │
│     └─ get_issue(id) if specific issue referenced                           │
│     └─ list_projects() if project context needed                            │
│                                                                             │
│  3. DRIVE FOR KNOWLEDGE                                                     │
│     └─ google_drive_search("internal docs about X")                         │
│     └─ google_drive_fetch(doc_id) for specific docs                         │
│                                                                             │
│  4. FILESYSTEM FOR CODE                                                     │
│     └─ list_directory("/path/to/project")                                   │
│     └─ read_file("key files")                                               │
│                                                                             │
│  5. WEB FOR CURRENT INFO                                                    │
│     └─ web_search("external references")                                    │
│     └─ web_fetch("documentation URLs")                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Why This Order?

1. **Memory** - Cheapest, fastest, most relevant to user
2. **Linear** - Project state, dependencies, acceptance criteria
3. **Drive** - Internal knowledge that won't be on web
4. **Filesystem** - Current code state
5. **Web** - External/current information (most expensive)

---

## 4. LINEAR INTEGRATION PATTERNS

### Mission ↔ Issue Mapping

```yaml
SX9 Concept     →  Linear Concept
─────────────────────────────────
Mission         →  Issue
Prompt          →  Issue Description
Deliverable     →  Sub-issue or Checklist
Checkpoint      →  Comment
Phase           →  Label (phase:IMPLEMENT)
Persona         →  Label (persona:FORGE)
Priority P0-P3  →  Priority 1-4 (Urgent→Low)
```

### Creating Issues from Prompts

```javascript
// When mission starts, create tracking issue:
linear:create_issue({
  team: "SX9",
  title: "${prompt.title}",
  description: `
## Objective
${prompt.objective}

## Constraints
${prompt.constraints.hard.join('\n')}

## Deliverables
${prompt.deliverables.map(d => `- [ ] ${d.name}`).join('\n')}

## Acceptance Criteria
${prompt.acceptance.join('\n')}
  `,
  priority: priorityMap[prompt.priority],
  labels: [
    `phase:${prompt.phase}`,
    `persona:${prompt.persona}`,
    `type:${prompt.type}`
  ]
})
```

### Checkpoint Comments

```javascript
// At each checkpoint:
linear:create_comment({
  issueId: missionIssueId,
  body: `
## Checkpoint: ${timestamp}

### Completed
${completed.join('\n')}

### In Progress
${inProgress.join('\n')}

### Blockers
${blockers.join('\n') || 'None'}

### Next Steps
${nextSteps.join('\n')}
  `
})
```

---

## 5. ARTIFACT CREATION PATTERNS

### File Types & Skills

| Output Type | Extension | Skill to Use |
|------------|-----------|--------------|
| Technical Doc | .md | Direct creation |
| Config | .yaml, .json | Direct creation |
| Code | .py, .rs, .ts | Direct creation |
| Word Doc | .docx | /mnt/skills/public/docx/SKILL.md |
| Presentation | .pptx | /mnt/skills/public/pptx/SKILL.md |
| Spreadsheet | .xlsx | /mnt/skills/public/xlsx/SKILL.md |
| PDF | .pdf | /mnt/skills/public/pdf/SKILL.md |
| Diagram | .jsx (React) | Inline artifact |

### File Creation Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. CREATE in /home/claude                                                   │
│    └─ Working directory, safe for iteration                                 │
│                                                                             │
│ 2. TEST/VALIDATE                                                            │
│    └─ Run scripts, check syntax, verify content                             │
│                                                                             │
│ 3. COPY to /mnt/user-data/outputs                                           │
│    └─ Final artifacts go here                                               │
│                                                                             │
│ 4. PRESENT with present_files tool                                          │
│    └─ Makes files available to user                                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

### React Artifacts for Visuals

For quick diagrams and interactive content:

```jsx
// Architecture diagram as React artifact
export default function ArchDiagram() {
  return (
    <div className="p-4 bg-zinc-900 text-white">
      {/* Mermaid-style diagram in JSX */}
      <svg>...</svg>
    </div>
  );
}
```

---

## 6. HARNESS CONFIGURATIONS

### When to Use Each Harness

| Harness | Mode | Temp | Use When |
|---------|------|------|----------|
| **full_autonomous** | AUTONOMOUS | 0.3 | Complex multi-step, trusted scope |
| **research** | AUTONOMOUS | 0.4 | Information gathering, analysis |
| **build** | SUPERVISED | 0.2 | Code generation, infrastructure |
| **security** | STEP-CONFIRM | 0.1 | Audits, sensitive analysis |
| **planning** | SUPERVISED | 0.3 | Strategy, documentation |

### Harness Selection Logic

```
Is this a security-sensitive operation?
├─ YES → security harness (STEP-CONFIRM, temp 0.1)
└─ NO
   │
   Is this primarily research/gathering?
   ├─ YES → research harness (AUTONOMOUS, temp 0.4)
   └─ NO
      │
      Is this code/build work?
      ├─ YES → build harness (SUPERVISED, temp 0.2)
      └─ NO
         │
         Is this planning/documentation?
         ├─ YES → planning harness (SUPERVISED, temp 0.3)
         └─ NO → full_autonomous (AUTONOMOUS, temp 0.3)
```

---

## 7. PROMPT STRUCTURE FOR OPTIMAL RESULTS

### The Ideal Prompt Structure

```yaml
# SX9-PROMPT v2.0

header:
  id: PRM-YYYYMMDD-HHMM
  title: "Clear, descriptive title"
  phase: IMPLEMENT
  priority: P1

harness:
  base: build
  persona: AXIOM
  mode: SUPERVISED
  
  context_loading:
    sources: [memory, linear, drive, web]
    
linear:
  enabled: true
  team: SX9
  labels: [type:CODE, phase:IMPLEMENT]

objective: |
  Single sentence: what does this accomplish?

context: |
  Why now? What triggered this? (2-3 sentences max)

constraints:
  hard:
    - Absolute limits (violations = halt)
  soft:
    - Preferences (violate with justification)

deliverables:
  - D1: Specific, verifiable output

acceptance:
  - Binary yes/no criteria

task: |
  CONTEXT LOADING:
  1. Search memory for prior work
  2. Check Linear for related issues
  3. Search Drive for internal docs
  
  EXECUTION:
  4. Step-by-step instructions
  5. Clear, actionable items
  
  COMPLETION:
  6. Update Linear
  7. Output artifacts
```

---

## 8. COMMON PATTERNS & EXAMPLES

### Example 1: Threat Research

```yaml
header:
  title: "Research APT29 TTPs"
  phase: RESEARCH
  priority: P1

harness:
  base: research
  persona: SENTINEL
  
  context_loading:
    sources: [memory, drive, web]

objective: |
  Document APT29 tactics, techniques, and procedures with detection opportunities.

task: |
  1. Search memory for prior APT29 research
  2. Search Drive for internal threat intel
  3. Web search for recent APT29 activity
  4. Web fetch MITRE ATT&CK page
  5. Create comprehensive TTP report
  6. Identify detection opportunities
  7. Create Linear issue for follow-up
```

### Example 2: Build Pipeline

```yaml
header:
  title: "Create RFC LaTeX Build Pipeline"
  phase: IMPLEMENT
  priority: P1

harness:
  base: build
  persona: FORGE

linear:
  enabled: true
  team: SX9
  labels: [type:BUILD, phase:IMPLEMENT]

objective: |
  Create automated LaTeX build system for RFC documents.

task: |
  1. Check Linear for related build issues
  2. Review existing 02-sx9-latex/ structure
  3. Search web for LaTeX build best practices
  4. Create Makefile with build targets
  5. Create build script
  6. Test with sample RFC
  7. Update Linear with implementation
  8. Output artifacts
```

### Example 3: Security Audit

```yaml
header:
  title: "Audit Authentication Module"
  phase: ANALYZE
  priority: P0

harness:
  base: security
  persona: VECTOR

constraints:
  hard:
    - READ-ONLY - DO NOT modify any files
    - HALT on credential exposure
    
linear:
  enabled: true
  labels: [type:SECURITY, severity:HIGH]

task: |
  1. Get audit scope from Linear issue
  2. Web search for auth vulnerabilities
  3. Read auth module code (READ-ONLY)
  4. Document findings with severity
  5. Create Canva threat diagram
  6. Create Linear issues per finding
```

---

## 9. ANTI-PATTERNS TO AVOID

### ❌ DON'T

- Start coding without loading context
- Skip Linear tracking for significant work
- Create files directly in /mnt/user-data/outputs (test first)
- Use AUTONOMOUS mode for security-sensitive work
- Ignore acceptance criteria
- Create monolithic files (>300 lines)
- Push to git without explicit approval

### ✅ DO

- Always start with context loading sequence
- Track all significant work in Linear
- Work in /home/claude, copy final to outputs
- Match harness to task sensitivity
- Verify all acceptance criteria before completing
- Decompose into small, focused modules
- Request explicit approval for destructive actions

---

## 10. QUICK REFERENCE

### Tool Selection Cheat Sheet

```
Need current info?        → web_search
Need full page content?   → web_fetch
Need to create files?     → computer_use
Need past conversation?   → conversation_search / recent_chats
Need internal docs?       → google_drive_search
Need task tracking?       → linear:*
Need diagrams?            → canva / React artifact
Need UI reference?        → figma:get_design_context
Need ML research?         → huggingface:paper_search
Need to deploy?           → vercel:deploy
```

### Persona Selection Cheat Sheet

```
Building pipelines?       → FORGE
Writing code?             → AXIOM
Security audit?           → VECTOR
Threat research?          → SENTINEL
Data migration?           → NEXUS
Crypto work?              → CIPHER
Documentation?            → SCRIBE
```

### Phase Reference

```
PULL      → Gathering requirements, inputs
ANALYZE   → Understanding, assessment
RESEARCH  → Investigation, discovery
PLAN      → Strategy, architecture
IMPLEMENT → Building, coding
WALK      → Review, verification
COMMIT    → Finalization, delivery
```
