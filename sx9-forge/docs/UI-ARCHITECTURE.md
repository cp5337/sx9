# SX9 Prompt Forge - UI Architecture

## Current Layout (PromptForgeScreen.tsx)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ HEADER                                                                       │
│ ┌────────────┐  ┌──────────────┐ ┌────────┐ ┌──────────────┐   [Copy] [Run] │
│ │ SX9 Forge  │  │ Title Input  │ │RFC-    │ │ Phase ▼     │                 │
│ └────────────┘  └──────────────┘ └────────┘ └──────────────┘                │
├─────────┬───────────────────────────────────────────────────┬───────────────┤
│ LEFT    │                    CENTER                          │    RIGHT     │
│ RAIL    │                                                    │    RAIL      │
│         │  ┌─────────────────────────────────────────────┐  │              │
│ [+] New │  │ 1  # SX9-PROMPT v4.0                        │  │ [Brain]      │
│ [E] Edit│  │ 2  # Generated: 2025-12-26...               │  │  Intel       │
│ ─────── │  │ 3                                           │  │              │
│ [C] Harn│  │ 4  header:                                  │  │ [Wrench]     │
│ [Z] Pers│  │ 5    title: "..."                           │  │  Tools       │
│ [T] Miss│  │ 6    rfc: RFC-                              │  │              │
│ [S] Depl│  │ 7    phase: IMPLEMENT                       │  │ [Shield]     │
│         │  │ 8                                           │  │  Threats     │
│ (flex)  │  │ 9  utilization:                             │  │ ─────────    │
│         │  │10    harness: Build & Implement             │  │ [Check]      │
│ ─────── │  │11    persona: FORGE                         │  │  QA/Gov      │
│ [S] Sett│  │12    model: Claude Opus 4.5                 │  │              │
│ [D] Data│  │13    temperature: 0.2                       │  │              │
│         │  │...                                          │  │              │
│         │  └─────────────────────────────────────────────┘  │              │
│         │  ┌─────────────────────────────────────────────┐  │              │
│         │  │ MISSION OBJECTIVE                           │  │              │
│         │  │ ┌─────────────────────────────────────────┐ │  │              │
│         │  │ │ Describe the mission...                 │ │  │              │
│         │  │ └─────────────────────────────────────────┘ │  │              │
│         │  └─────────────────────────────────────────────┘  │              │
├─────────┴───────────────────────────────────────────────────┴───────────────┤
│ STATUS BAR                                                                   │
│ [●] Inference  │  [●] Vector  │  RFC-  │  (file.yaml)  │  READY  [Refresh] │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Left Rail Tabs (Expanded View)

### Tab Bar
```
┌────────┬────────┬────────┬────────┬────────┬─────────┐
│HARNESS │PERSONA │ AGENTS │ LINEAR │ SLACK  │ CONTEXT │
└────────┴────────┴────────┴────────┴────────┴─────────┘
```

### Harness Tab
```
┌─────────────────────────────┐
│ HARNESS MODE                │
├─────────────────────────────┤
│ ┌─────────────────────────┐ │
│ │ Build        [selected] │ │
│ ├─────────────────────────┤ │
│ │ Research                │ │
│ ├─────────────────────────┤ │
│ │ Security                │ │
│ ├─────────────────────────┤ │
│ │ Planning                │ │
│ └─────────────────────────┘ │
│                             │
│ [x] Use Agent Harness       │
└─────────────────────────────┘
```

### Persona Tab (10 CLSGS Agents)
```
┌─────────────────────────────┐
│ SX9 AGENTS (CLSGS Annex A.2)│
├─────────────────────────────┤
│ ┌──────────┬──────────────┐ │
│ │  FORGE   │   AXIOM      │ │
│ ├──────────┼──────────────┤ │
│ │  VECTOR  │   SENTINEL   │ │
│ ├──────────┼──────────────┤ │
│ │ GUARDIAN │   ORACLE     │ │
│ ├──────────┼──────────────┤ │
│ │  SCRIBE  │   RELAY      │ │
│ ├──────────┼──────────────┤ │
│ │  ARBITER │   WEAVER     │ │
│ └──────────┴──────────────┘ │
│                             │
│ Code Generation • Filesystem│
│ CI/CD, MCP tools            │
└─────────────────────────────┘
```

### Agents Tab (AI Providers)
```
┌─────────────────────────────┐
│ AI PROVIDER                 │
├─────────────────────────────┤
│ ┌─────────────────────────┐ │
│ │ Claude Opus 4.5      ▼  │ │
│ └─────────────────────────┘ │
│                             │
│ TEMPERATURE: 0.2            │
│ ──────●───────────────────  │
│                             │
│ Provider routes through SX9 │
│ harness for governance.     │
└─────────────────────────────┘
```

## Right Rail Tabs (Expanded View)

### Tab Bar
```
┌────────┬────────┬──────────┬──────┐
│ INTEL  │ TOOLS  │ THREATS  │  QA  │
└────────┴────────┴──────────┴──────┘
```

### Intel Tab
```
┌─────────────────────────────┐
│ PATTERN INTELLIGENCE        │
├─────────────────────────────┤
│ ● Similar prompts (3)       │
│ ● Related RFC patterns      │
│ ● Behavioral scope (5)      │
│                             │
│ LEPTOSE STATUS              │
│ Vector similarity: Ready    │
└─────────────────────────────┘
```

### QA Tab (Governance - RFC-9141)
```
┌─────────────────────────────┐
│ GOVERNANCE STATUS (RFC-9141)│
├─────────────────────────────┤
│ ┌───────────┬─────────────┐ │
│ │ ✓ Static  │ ○ Semantic  │ │
│ │   QA      │   QA        │ │
│ ├───────────┼─────────────┤ │
│ │ — Lineage │ — Drift     │ │
│ │   Check   │   Score     │ │
│ └───────────┴─────────────┘ │
│                             │
│ GATE STATUS                 │
│ OBSERVE • No blocking gates │
└─────────────────────────────┘
```

## Component Hierarchy

```
PromptForgeScreen
├── Header
│   ├── Logo
│   ├── TitleInput
│   ├── RFCInput
│   ├── PhaseSelect
│   ├── CopyButton
│   └── RunButton
│
├── Main (flex row)
│   ├── LeftRail
│   │   ├── RailHeader (collapse/expand)
│   │   └── RailBody
│   │       ├── LeftIcons (collapsed)
│   │       └── LeftContent (expanded)
│   │           ├── TabBar
│   │           └── TabContent
│   │               ├── NewTab
│   │               ├── EditTab
│   │               ├── HarnessTab
│   │               ├── PersonaTab
│   │               ├── AgentsTab
│   │               ├── LinearTab
│   │               ├── SlackTab
│   │               ├── ContextTab
│   │               ├── MissionTab
│   │               ├── DeployTab
│   │               ├── SettingsTab
│   │               └── DataTab
│   │
│   ├── Center
│   │   ├── Editor (YAML preview)
│   │   │   ├── LineNumbers
│   │   │   └── CodePane
│   │   └── MissionInput
│   │
│   └── RightRail
│       ├── RailHeader
│       └── RailBody
│           ├── RightIcons (collapsed)
│           └── RightContent (expanded)
│               ├── TabBar
│               └── TabContent
│                   ├── IntelTab
│                   ├── ToolsTab
│                   ├── ThreatsTab
│                   └── QATab
│
├── TemplatePickerModal
├── ComponentsPanel (slide-out)
└── StatusBar
    ├── InferenceStatus
    ├── VectorStatus
    ├── RFCCode
    ├── LoadedFilePath
    ├── FeedbackText
    └── RefreshButton
```

## State Management

```
Local State (useState):
├── Rails: leftOpen, rightOpen, leftTab, rightTab
├── Form: title, rfc, phase, objective
├── Harness: harnessMode, persona, model, temperature, useHarness
├── Integration: linearTeam, createLinearIssue, slackChannel, enableSlack
├── Context: contextSources (memory, linear, drive, filesystem, web)
├── UI: feedback, timestamp, showTemplates, showComponents
└── File: templates[], loadedFilePath

Redux State:
├── intelligence.leptose.status
└── intelligence.chroma.status

Derived:
└── output (YAML string via useMemo)
```

## Tauri Commands Used

| Command | Purpose |
|---------|---------|
| `save_prompt` | Save YAML to disk |
| `create_linear_issue_forge` | Create Linear issue |
| `notify_slack` | Send Slack notification |
| `copy_to_clipboard` | Copy to system clipboard |
| `open_file_dialog` | File picker dialog |
| `list_templates` | List .yaml templates |
| `read_file_by_path` | Read template content |

## CLSGS Integration Points

1. **Annex A.2** - 10 Agent personas with N-V-N-N behavioral scopes
2. **Annex A.3** - Linear integration with intent anchors
3. **Annex A.4** - Git lineage tracking (future)
4. **RFC-9141** - Dual-heartbeat QA in right rail QA tab
5. **RFC-9142** - Drift scoring (future integration)
