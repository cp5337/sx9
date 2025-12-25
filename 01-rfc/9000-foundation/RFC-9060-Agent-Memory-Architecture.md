# RFC-9060: Agent Memory Architecture

**Status:** DRAFT
**Version:** 1.0.0
**Date:** 2025-12-24
**References:** Anthropic "Effective Harnesses for Long-Running Agents", RFC-9130, RFC-9050

---

## 1. Overview

This RFC defines the memory architecture for long-running FORGE agents, implementing patterns from Anthropic's research on effective agent harnesses.

### Core Principle
> "Human-inspired practices—where engineers document handoffs between shifts—directly informed this architecture's effectiveness."

---

## 2. Memory Hierarchy

### 2.1 Folder Structure

```
.forge/
├── memory/
│   ├── long-term/           # Persistent across all sessions
│   │   ├── knowledge.json   # Learned patterns, decisions, rationale
│   │   ├── features.json    # Feature list with pass/fail status
│   │   └── architecture.md  # System understanding, invariants
│   │
│   ├── short-term/          # Current session context
│   │   ├── progress.txt     # What's been done this session
│   │   ├── focus.json       # Current task, files, state
│   │   └── scratch/         # Working notes, drafts
│   │
│   └── artifacts/           # Produced outputs
│       ├── code/            # Generated code snippets
│       ├── docs/            # Generated documentation
│       └── reports/         # QA reports, analysis
│
├── linear/                  # Linear issue tracking
│   ├── active.json          # Currently active issues
│   ├── history/             # Completed issue summaries
│   └── templates/           # Atomic component templates
│
├── git/                     # Git backtrace
│   ├── commits.log          # Recent commit summaries
│   ├── branches.json        # Active branch state
│   └── checkpoints/         # Known-good state markers
│
└── init.sh                  # Environment bootstrap script
```

---

## 3. Memory Components

### 3.1 Long-Term Memory

**knowledge.json** - Accumulated learnings:
```json
{
  "schema_version": "1.0",
  "patterns": {
    "successful": [
      {
        "context": "Adding new NATS subject",
        "approach": "Add to sx9-harness/src/nats/subjects.rs, run tests",
        "learned_at": "2025-12-24T10:30:00Z"
      }
    ],
    "failed": [
      {
        "context": "Direct chrono import with foundation crate",
        "issue": "Derive macros need serde in scope",
        "resolution": "Add serde directly, use foundation for types",
        "learned_at": "2025-12-24T11:00:00Z"
      }
    ]
  },
  "invariants": [
    "Never use Blake3 - only Murmur3 trivariate",
    "Always use foundation crates for common deps",
    "NATS subjects must mirror Redux action types"
  ]
}
```

**features.json** - Feature tracking (prevents premature completion):
```json
{
  "schema_version": "1.0",
  "features": [
    {
      "id": "FORGE-001",
      "name": "NATS Subject Hierarchy",
      "status": "passing",
      "tests": ["nats::subjects::tests::*"],
      "verified_at": "2025-12-24T11:30:00Z"
    },
    {
      "id": "FORGE-002",
      "name": "Thalmic Filter",
      "status": "not_implemented",
      "blocked_by": null
    }
  ]
}
```

### 3.2 Short-Term Memory

**progress.txt** - Session log (human-readable):
```
=== FORGE Session 2025-12-24 ===

[11:00] Started session - reading git log and features.json
[11:05] Continuing work on NATS integration
[11:15] Fixed chrono import issue - added serde to Cargo.toml
[11:20] All 8 NATS tests passing
[11:25] Merged RFC-9400 subjects into sx9-harness
[11:30] CHECKPOINT: sx9-harness compiles, tests pass

NEXT: Wire ATLAS daemon to NATS (see Linear SX9-xxx)
```

**focus.json** - Current working state:
```json
{
  "task": "Wire ATLAS daemon to NATS",
  "linear_issue": "SX9-123",
  "files": [
    "crates/sx9-atlas-daemon/src/main.rs",
    "crates/sx9-harness/src/nats/subjects.rs"
  ],
  "phase": "PLAN",
  "blockers": [],
  "started_at": "2025-12-24T11:35:00Z"
}
```

### 3.3 Artifacts

Generated outputs stored by type with trivariate hashes for tracking:

```
artifacts/
├── code/
│   └── triv_abc123_feature.rs     # Generated code
├── docs/
│   └── triv_def456_readme.md      # Generated docs
└── reports/
    └── triv_ghi789_qa_report.json # QA output
```

---

## 4. Skills Architecture

### 4.1 Skills as Atomic Knowledge Units

Following Anthropic's skills model, FORGE uses a **progressive disclosure architecture**:

| Layer | Tokens | Purpose |
|-------|--------|---------|
| **Metadata** | ~100 | Skill descriptions, triggers |
| **Instructions** | <5k | Full procedural details |
| **Resources** | On-demand | Scripts, templates, assets |

### 4.2 Skill Structure

```
.forge/skills/
├── code-generation/
│   ├── skill.toml           # Metadata + triggers
│   ├── instructions.md      # Procedural guidance
│   └── templates/           # Code templates
│
├── qa-review/
│   ├── skill.toml
│   ├── instructions.md
│   └── checklists/
│
├── linear-integration/
│   ├── skill.toml
│   ├── instructions.md
│   └── forms/               # Issue templates
│
└── git-workflow/
    ├── skill.toml
    ├── instructions.md
    └── hooks/               # Commit/PR templates
```

### 4.3 Skill Manifest (skill.toml)

```toml
[skill]
name = "code-generation"
description = "Generate Rust code following sx9 patterns"
version = "1.0.0"

[triggers]
# When to activate this skill
keywords = ["generate", "create", "implement", "add function"]
file_patterns = ["*.rs", "Cargo.toml"]
linear_labels = ["feature", "enhancement"]

[resources]
instructions = "instructions.md"
templates = "templates/"

[dependencies]
requires = ["qa-review"]  # Chain to QA after generation
```

### 4.4 Skill ↔ Memory Integration

| Component | Skill Role |
|-----------|------------|
| **Long-term** | Skills are the "how" to long-term's "what" |
| **Short-term** | Active skill tracked in focus.json |
| **Artifacts** | Skills produce artifacts |
| **Linear** | Issues trigger specific skills |
| **Git** | Skills define commit patterns |

---

## 5. Linear Atomic Components

### 5.1 Issue as Memory Unit

Each Linear issue becomes an atomic memory unit that triggers skills:

```json
{
  "linear_id": "SX9-123",
  "title": "Wire ATLAS daemon to NATS",
  "type": "feature",

  "memory_trace": {
    "created_from": "RFC-9400 implementation",
    "parent_issue": "SX9-100",
    "context_snapshot": "triv:abc_def_ghi"
  },

  "agent_state": {
    "assigned_persona": "Forge",
    "harness_mode": "Build",
    "estimated_complexity": "medium"
  },

  "acceptance_criteria": [
    "ATLAS daemon connects to NATS on startup",
    "Publishes to sx9.atlas.* subjects",
    "Subscribes to sx9.atlas.cmd.* subjects"
  ],

  "git_refs": {
    "branch": "feature/SX9-123-atlas-nats",
    "commits": []
  }
}
```

### 4.2 Linear Form Templates

**Feature Request Template:**
```yaml
template: feature
fields:
  - name: title
    type: text
    required: true

  - name: description
    type: markdown
    required: true

  - name: parent_rfc
    type: reference
    pattern: "RFC-\\d{4}"

  - name: affected_crates
    type: multi_select
    options: [from: workspace_crates]

  - name: acceptance_criteria
    type: checklist
    min_items: 1

  - name: qa_requirements
    type: select
    options: [unit_tests, integration_tests, manual_qa]
```

**Bug Report Template:**
```yaml
template: bug
fields:
  - name: title
    type: text
    pattern: "[BUG] *"

  - name: reproduction_steps
    type: markdown
    required: true

  - name: expected_behavior
    type: text

  - name: actual_behavior
    type: text

  - name: git_bisect_range
    type: text
    pattern: "[a-f0-9]{7}\\.\\.[a-f0-9]{7}"
```

---

## 5. Git Backtrace

### 5.1 Commit Protocol

Every agent commit follows this format:

```
[SX9-123] Brief description

## What
- Specific change 1
- Specific change 2

## Why
Rationale for the change

## Memory
- Learned: [key insight]
- Updated: features.json (FORGE-002 now passing)

## Backtrace
- Parent: abc1234 (previous working state)
- Linear: SX9-123
- Session: 2025-12-24-session-001
```

### 5.2 Checkpoint System

**Creating Checkpoints:**
```bash
# Agent creates checkpoint after verified working state
git tag -a checkpoint/SX9-123-v1 -m "Working ATLAS-NATS connection"
```

**Checkpoint Manifest (.forge/git/checkpoints/SX9-123-v1.json):**
```json
{
  "tag": "checkpoint/SX9-123-v1",
  "commit": "abc1234def5678",
  "timestamp": "2025-12-24T12:00:00Z",
  "verified_by": ["cargo test", "cargo check"],
  "features_passing": ["FORGE-001", "FORGE-002"],
  "linear_issues_closed": ["SX9-123"],
  "can_rollback_to": true
}
```

### 5.3 Recovery Protocol

When agent encounters issues:

1. **Read last checkpoint** from `.forge/git/checkpoints/`
2. **Diff current state** against checkpoint
3. **Decision matrix:**
   - Minor divergence → continue with fixes
   - Major divergence → rollback to checkpoint
   - Unknown state → alert human, pause

---

## 6. Session Lifecycle

### 6.1 Session Start (Initialization)

```python
def start_session():
    # 1. Read long-term memory
    knowledge = read_json(".forge/memory/long-term/knowledge.json")
    features = read_json(".forge/memory/long-term/features.json")

    # 2. Read git state
    recent_commits = git_log(n=10)
    current_branch = git_branch_current()

    # 3. Read Linear active issues
    active_issues = read_json(".forge/linear/active.json")

    # 4. Initialize short-term memory
    progress = f"=== FORGE Session {datetime.now()} ===\n"
    progress += f"[{time()}] Started - branch: {current_branch}\n"
    progress += f"[{time()}] Active issues: {len(active_issues)}\n"

    # 5. Determine focus
    focus = select_next_task(active_issues, features)

    write_file(".forge/memory/short-term/progress.txt", progress)
    write_json(".forge/memory/short-term/focus.json", focus)
```

### 6.2 During Session

```python
def work_cycle():
    while not session_complete():
        # Read current focus
        focus = read_json(".forge/memory/short-term/focus.json")

        # Execute task
        result = execute_task(focus)

        # Update progress
        append_progress(f"[{time()}] {result.summary}")

        # Update features if applicable
        if result.feature_updated:
            update_feature_status(result.feature_id, result.status)

        # Commit if meaningful progress
        if result.should_commit:
            create_commit(result)

        # Checkpoint if milestone reached
        if result.is_milestone:
            create_checkpoint(result)
```

### 6.3 Session End

```python
def end_session():
    # 1. Final progress entry
    append_progress(f"[{time()}] Session ending")

    # 2. Update long-term memory with learnings
    if new_patterns_learned:
        update_knowledge(new_patterns_learned)

    # 3. Commit session state
    create_commit({
        "type": "session_end",
        "message": "Session checkpoint",
        "include_forge": True
    })

    # 4. Update Linear issues
    for issue in worked_issues:
        update_linear_issue(issue, session_notes)
```

---

## 7. NATS Integration

Memory events publish to NATS for observability:

```rust
// Memory event subjects
pub mod memory {
    pub const SESSION_START: &str = "sx9.forge.memory.session.start";
    pub const SESSION_END: &str = "sx9.forge.memory.session.end";
    pub const CHECKPOINT: &str = "sx9.forge.memory.checkpoint";
    pub const LEARNING: &str = "sx9.forge.memory.learning";
    pub const FOCUS_CHANGE: &str = "sx9.forge.memory.focus";
}
```

---

## 8. Color-Coded Visualization

### 8.1 Memory Health Colors

| Component | Green | Yellow | Red |
|-----------|-------|--------|-----|
| Long-term | knowledge.json valid | >7 days stale | missing/corrupt |
| Short-term | progress.txt current | >1hr stale | missing |
| Features | >80% passing | 50-80% passing | <50% passing |
| Git | clean, checkpointed | uncommitted changes | diverged from main |
| Linear | all issues tracked | orphan commits | sync errors |

### 8.2 Folder Icons (Tauri/UI)

```typescript
const MEMORY_COLORS = {
  "long-term": "#4CAF50",   // Green - persistent
  "short-term": "#2196F3",  // Blue - active
  "artifacts": "#9C27B0",   // Purple - outputs
  "linear": "#FF9800",      // Orange - tasks
  "git": "#607D8B",         // Gray - history
};
```

---

## 9. Implementation Checklist

- [ ] Create `.forge/` directory structure
- [ ] Implement memory read/write in sx9-harness
- [ ] Add memory NATS subjects to subjects.rs
- [ ] Create Linear form templates
- [ ] Implement git commit protocol
- [ ] Add checkpoint system
- [ ] Build Tauri commands for memory access
- [ ] Add memory visualization to FORGE UI

---

## References

- [Anthropic: Effective Harnesses for Long-Running Agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- RFC-9130: Unified Forge Pipeline
- RFC-9050: QA Two-Heartbeat System
