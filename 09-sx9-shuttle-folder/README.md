# SX9 Shuttle Folder

Multi-model context handoff system for AI orchestration.

## Structure

```
09-sx9-shuttle-folder/
├── inbox/              # Incoming missions/tasks from any model
├── outbox/             # Completed handoffs ready for pickup
├── breadcrumbs/        # Activity logs per model
├── context/            # Shared state files
│   ├── active.json     # Current task state
│   ├── files.json      # Known important file locations
│   └── models.json     # Model status & last actions
└── archive/            # Completed missions (timestamped)
```

## Usage

### Drop a mission (any model):
```bash
cp mission.yaml ~/Developer/sx9/09-sx9-shuttle-folder/inbox/
```

### Pick up a mission (receiving model):
```bash
ls ~/Developer/sx9/09-sx9-shuttle-folder/inbox/
cat ~/Developer/sx9/09-sx9-shuttle-folder/inbox/mission.yaml
# After reading, move to outbox or archive
```

### Log breadcrumb:
```bash
echo "$(date -Iseconds) | Completed T1-T3" >> breadcrumbs/claude.log
```

### Update context:
```bash
# Models read/write context/active.json to share state
```

## File Naming Convention

**Inbox/Outbox:**
```
{DATE}-{FROM}-{TO}-{SUBJECT}.yaml
2024-12-18-claude-antigravity-forge-wiring.yaml
```

**Breadcrumbs:**
```
{model}.log
claude.log
antigravity.log
cursor.log
gemini.log
gpt.log
```

## Integration

Works with:
- Claude (claude.ai, Claude Code)
- Antigravity
- Cursor
- VS Code + Copilot
- Gemini
- GPT/ChatGPT
- Any CLI tool

All models can read/write to filesystem = universal protocol.
