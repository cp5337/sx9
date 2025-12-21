# SX9 OPS ARCHITECTURE
## IDE Bootloader + Prompt Forge + Mission Control

```
═══════════════════════════════════════════════════════════════════════════════
                            SX9 OPS CONTROL PLANE
═══════════════════════════════════════════════════════════════════════════════

    ┌─────────────────────────────────────────────────────────────────────┐
    │                        OPERATOR INTERFACE                           │
    ├─────────────────────────────────────────────────────────────────────┤
    │                                                                     │
    │   ┌─────────────────┐         ┌─────────────────┐                  │
    │   │  PROMPT FORGE   │         │   SX9 CLI       │                  │
    │   │    (Web UI)     │         │   (Terminal)    │                  │
    │   │                 │         │                 │                  │
    │   │ • Type presets  │         │ • sx9 init      │                  │
    │   │ • File browser  │         │ • sx9 boot      │                  │
    │   │ • Constraint UI │         │ • sx9 mission   │                  │
    │   │ • YAML export   │         │ • sx9 prompt    │                  │
    │   └────────┬────────┘         └────────┬────────┘                  │
    │            │                           │                           │
    │            └───────────┬───────────────┘                           │
    │                        ▼                                           │
    │            ┌─────────────────────┐                                 │
    │            │   .sx9/ DIRECTORY   │                                 │
    │            │   (Control Plane)   │                                 │
    │            └─────────────────────┘                                 │
    │                        │                                           │
    └────────────────────────┼───────────────────────────────────────────┘
                             │
    ┌────────────────────────┼───────────────────────────────────────────┐
    │                        ▼                                           │
    │            ┌─────────────────────┐                                 │
    │            │    IDE BOOTLOADER   │                                 │
    │            │                     │                                 │
    │            │ Reads bootloader.yaml                                 │
    │            │ Generates:          │                                 │
    │            │ • .cursorrules      │                                 │
    │            │ • .vscode/settings  │                                 │
    │            │ • MCP config        │                                 │
    │            └──────────┬──────────┘                                 │
    │                       │                                            │
    │         ┌─────────────┼─────────────┐                              │
    │         ▼             ▼             ▼                              │
    │   ┌──────────┐  ┌──────────┐  ┌──────────┐                        │
    │   │  CURSOR  │  │  VSCODE  │  │  CLAUDE  │                        │
    │   │   IDE    │  │  + Ext   │  │ DESKTOP  │                        │
    │   └────┬─────┘  └────┬─────┘  └────┬─────┘                        │
    │        │             │             │                               │
    │        └─────────────┼─────────────┘                               │
    │                      ▼                                             │
    │            ┌─────────────────────┐                                 │
    │            │   AGENT EXECUTION   │                                 │
    │            │                     │                                 │
    │            │ • Loads mission     │                                 │
    │            │ • Applies persona   │                                 │
    │            │ • Enforces constraints                                │
    │            │ • Logs checkpoints  │                                 │
    │            │ • Writes artifacts  │                                 │
    │            └─────────────────────┘                                 │
    │                                                                    │
    │                      AGENT RUNTIME                                 │
    └────────────────────────────────────────────────────────────────────┘


═══════════════════════════════════════════════════════════════════════════════
                         .sx9/ DIRECTORY STRUCTURE
═══════════════════════════════════════════════════════════════════════════════

    .sx9/
    ├── bootloader.yaml          # Master configuration
    │                            # • Personas & roles
    │                            # • Global constraints
    │                            # • Filesystem rules
    │                            # • IDE integration
    │
    ├── prompts/                 # Generated prompt YAMLs
    │   ├── PRM-20241218-0930.yaml
    │   ├── PRM-20241218-1045.yaml
    │   └── ...
    │
    ├── missions/                # Mission lifecycle
    │   ├── active/              # Currently executing
    │   │   └── MSN-20241218-0932.yaml
    │   ├── completed/           # Successfully finished
    │   └── failed/              # Failed or rolled back
    │
    ├── logs/                    # Audit trail
    │   ├── missions.log         # Mission lifecycle events
    │   └── {mission-id}.log     # Per-mission detailed log
    │
    ├── templates/               # Prompt templates
    │   ├── build-pipeline.yaml
    │   ├── security-audit.yaml
    │   └── ...
    │
    └── scratch/                 # Temporary working space
                                 # (safe for agent to use freely)


═══════════════════════════════════════════════════════════════════════════════
                              WORKFLOW
═══════════════════════════════════════════════════════════════════════════════

    ┌──────────────────────────────────────────────────────────────────────┐
    │ 1. INITIALIZE                                                        │
    │    $ sx9 init                                                        │
    │    Creates .sx9/ structure in project root                           │
    └──────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
    ┌──────────────────────────────────────────────────────────────────────┐
    │ 2. BOOT IDE                                                          │
    │    $ sx9 boot FORGE                                                  │
    │    Generates .cursorrules with persona + constraints                 │
    └──────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
    ┌──────────────────────────────────────────────────────────────────────┐
    │ 3. CREATE PROMPT (Option A: CLI)                                     │
    │    $ sx9 prompt new BUILD_PIPELINE                                   │
    │    Creates .sx9/prompts/PRM-*.yaml from template                     │
    │                                                                      │
    │    (Option B: Web UI)                                                │
    │    Open Prompt Forge → Select type → Fill form → Export YAML         │
    │    Save to .sx9/prompts/                                             │
    └──────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
    ┌──────────────────────────────────────────────────────────────────────┐
    │ 4. START MISSION                                                     │
    │    $ sx9 mission new .sx9/prompts/PRM-20241218-0930.yaml             │
    │    Creates active mission, logs start time                           │
    └──────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
    ┌──────────────────────────────────────────────────────────────────────┐
    │ 5. EXECUTE IN IDE                                                    │
    │    • Open IDE (Cursor/VSCode/Claude)                                 │
    │    • Agent reads .cursorrules (injected context)                     │
    │    • Paste prompt YAML or reference active mission                   │
    │    • Agent executes with constraints enforced                        │
    └──────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
    ┌──────────────────────────────────────────────────────────────────────┐
    │ 6. CHECKPOINT (as needed)                                            │
    │    $ sx9 mission checkpoint "Phase 1 complete"                       │
    │    Logs progress, allows pause/resume                                │
    └──────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
    ┌──────────────────────────────────────────────────────────────────────┐
    │ 7. COMPLETE OR FAIL                                                  │
    │    $ sx9 mission complete    # All acceptance criteria met           │
    │    $ sx9 mission fail "reason"  # If constraints violated            │
    │    Archives mission, generates summary                               │
    └──────────────────────────────────────────────────────────────────────┘


═══════════════════════════════════════════════════════════════════════════════
                           PERSONA MATRIX
═══════════════════════════════════════════════════════════════════════════════

    ┌──────────┬────────────────────┬─────────────┬───────────────────────┐
    │ PERSONA  │ PRIMARY USE        │ TEMP        │ KEY CONSTRAINTS       │
    ├──────────┼────────────────────┼─────────────┼───────────────────────┤
    │ FORGE    │ Build pipelines    │ 0.1         │ No source mods        │
    │          │ CI/CD, packaging   │ (precise)   │ Document everything   │
    ├──────────┼────────────────────┼─────────────┼───────────────────────┤
    │ AXIOM    │ Code generation    │ 0.2         │ Max 300 lines/file    │
    │          │ Architecture       │ (balanced)  │ Include tests         │
    ├──────────┼────────────────────┼─────────────┼───────────────────────┤
    │ VECTOR   │ Security audits    │ 0.1         │ READ-ONLY always      │
    │          │ Vuln scanning      │ (precise)   │ Flag severity         │
    ├──────────┼────────────────────┼─────────────┼───────────────────────┤
    │ SENTINEL │ Threat emulation   │ 0.2         │ No production         │
    │          │ Red team, TTPs     │ (balanced)  │ Map to ATT&CK         │
    ├──────────┼────────────────────┼─────────────┼───────────────────────┤
    │ NEXUS    │ Integrations       │ 0.1         │ Always have rollback  │
    │          │ Data migrations    │ (precise)   │ Validate integrity    │
    ├──────────┼────────────────────┼─────────────┼───────────────────────┤
    │ CIPHER   │ Crypto, privacy    │ 0.1         │ Never log secrets     │
    │          │ Secure patterns    │ (precise)   │ Document decisions    │
    └──────────┴────────────────────┴─────────────┴───────────────────────┘


═══════════════════════════════════════════════════════════════════════════════
                        PROMPT TYPE → PRESET MAPPING
═══════════════════════════════════════════════════════════════════════════════

    Prompt Forge Type        Default Persona    Phase       Tools Enabled
    ─────────────────────────────────────────────────────────────────────
    BUILD_PIPELINE      →    FORGE              IMPLEMENT   fs_rw, shell, git
    SECURITY_AUDIT      →    VECTOR             ANALYZE     fs_r, shell, web
    CODE_GENERATION     →    AXIOM              IMPLEMENT   fs_rw, shell, git
    REFACTOR           →    AXIOM              IMPLEMENT   fs_rwd, shell, git
    MIGRATION          →    NEXUS              PLAN        fs_rw, shell, db
    DOCUMENTATION      →    (default)          WALK        fs_rw, git
    RFC_ALIGNMENT      →    FORGE              WALK        fs_rw, shell, git
    THREAT_EMULATION   →    SENTINEL           IMPLEMENT   fs_rw, shell, db
    RESEARCH           →    SENTINEL           RESEARCH    fs_rw, net, web


═══════════════════════════════════════════════════════════════════════════════
                         INTEGRATION POINTS
═══════════════════════════════════════════════════════════════════════════════

    CURSOR IDE
    ───────────────────────────────────────────────────────────────────────
    • .cursorrules      ← Generated by `sx9 boot`
    • Agent reads rules on every prompt
    • Constraints enforced via system prompt injection

    VS CODE
    ───────────────────────────────────────────────────────────────────────
    • .vscode/settings.json    ← Generated by `sx9 boot`
    • .vscode/tasks.json       ← Quick actions for mission control
    • Works with Claude extension, Continue, etc.

    CLAUDE DESKTOP (MCP)
    ───────────────────────────────────────────────────────────────────────
    • Can configure MCP servers for filesystem access
    • Bootloader can generate claude_desktop_config.json
    • Future: Custom MCP server for mission control

    PROMPT FORGE (Web UI)
    ───────────────────────────────────────────────────────────────────────
    • Standalone React app
    • Exports YAML to clipboard or file
    • Reads .sx9/bootloader.yaml for project-specific presets
    • Future: Direct integration via local API


═══════════════════════════════════════════════════════════════════════════════
                            QUICK START
═══════════════════════════════════════════════════════════════════════════════

    # 1. Install CLI
    chmod +x sx9-cli.sh
    sudo ln -s $(pwd)/sx9-cli.sh /usr/local/bin/sx9

    # 2. Initialize project
    cd /path/to/synaptix9
    sx9 init

    # 3. Boot IDE with persona
    sx9 boot FORGE

    # 4. Create prompt (or use Prompt Forge web UI)
    sx9 prompt new BUILD_PIPELINE

    # 5. Start mission
    sx9 mission new .sx9/prompts/PRM-*.yaml

    # 6. Work in IDE...
    # 7. Checkpoint progress
    sx9 mission checkpoint "Completed phase 1"

    # 8. Complete when done
    sx9 mission complete


═══════════════════════════════════════════════════════════════════════════════
```

## Files Summary

| File | Purpose |
|------|---------|
| `sx9-prompt-forge-v2.jsx` | Web UI for generating prompts |
| `sx9-cli.sh` | CLI for bootloader & mission control |
| `sx9-bootloader.yaml` | Master configuration template |
| `sx9-ops-architecture.md` | This architecture doc |

## Next Steps

1. **Package as npm/cargo tool** - Make `sx9` installable via package manager
2. **Prompt Forge server mode** - Local web server that reads `.sx9/` config
3. **MCP server for missions** - Claude Desktop integration for mission control
4. **Git hooks** - Auto-checkpoint on commit, fail mission on forbidden path modification
5. **Dashboard** - Real-time mission status, log streaming, metrics
