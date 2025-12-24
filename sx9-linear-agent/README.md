# SX9 Linear Agent

Autonomous agent that integrates Linear, Serena MCP, and Slack for automated code generation and project management.

## Architecture

```
sx9-linear-agent/
├── Cargo.toml                    # Rust workspace
├── src/
│   ├── main.rs                   # Entry point
│   ├── agent/
│   │   ├── initializer.rs        # Creates Linear project + issues from spec
│   │   ├── coder.rs              # Picks up issues, implements, marks done
│   │   └── handoff.rs            # Session state via Linear comments
│   ├── linear/
│   │   ├── client.rs             # Linear API wrapper
│   │   ├── issue.rs              # Issue CRUD
│   │   └── project.rs            # Project management
│   ├── security/
│   │   ├── sandbox.rs            # Command allowlist
│   │   └── filesystem.rs         # Path restrictions
│   └── mcp/
│       ├── linear.rs             # Linear MCP integration
│       ├── puppeteer.rs          # Browser testing MCP
│       ├── serena.rs             # Serena AI code generation
│       └── slack.rs              # Slack notifications
├── prompts/
│   ├── spec_template.md          # App specification format
│   ├── initializer.md            # First-run prompt
│   └── coder.md                  # Continuation prompt
└── config/
    └── linear.toml               # Configuration
```

## Features

### 🎯 Linear Integration
- Automatic project and issue creation from specifications
- Issue state management (Todo → In Progress → Done)
- Comment-based handoff between sessions
- GraphQL API integration

### 🤖 Serena MCP
- AI-powered code generation
- Code quality analysis
- Intelligent suggestions
- Multi-language support

### 💬 Slack Integration
- Real-time notifications for issue updates
- Code generation completion alerts
- QA gate result notifications
- Rich message formatting with blocks

### 🔒 Security
- Command whitelist (cargo, git, npm, etc.)
- Filesystem path restrictions
- Forbidden pattern detection
- Sandboxed execution

### ✅ QA Gates
- Static analysis (cargo check, clippy)
- Architecture compliance
- Pattern matching
- Quality score thresholds

## Setup

### 1. Install Dependencies
```bash
cd sx9-linear-agent
cargo build
```

### 2. Configure API Keys
Edit `config/linear.toml`:
```toml
linear_api_key = "lin_api_..."
slack_bot_token = "xoxb-..."
serena_endpoint = "http://localhost:8000"
team_id = "your-team-id"
slack_channel = "#sx9-dev"
```

### 3. Run Agent
```bash
cargo run
```

## Usage

### Initialize Project from Spec
```bash
# Create prompts/spec_template.md with your app specification
cargo run -- init --spec prompts/spec_template.md
```

### Run Coder Loop
```bash
# Agent picks up Linear issues and implements them
cargo run -- code --project-id <project-id>
```

### Manual Issue Assignment
```bash
# Assign specific issue to agent
cargo run -- assign --issue-id <issue-id>
```

## Workflow

1. **Initialization**
   - Read app specification
   - Create Linear project
   - Break down into issues
   - Notify Slack

2. **Coding Loop**
   - Poll Linear for assigned issues
   - Use Serena MCP to generate code
   - Run QA gates
   - Commit to git
   - Update Linear issue status
   - Notify Slack

3. **Handoff**
   - Save session state in Linear comments
   - Next session resumes from last state
   - Continuous progress tracking

## Integration with sx9-harness

The Linear Agent uses `sx9-harness` for QA gates:

```rust
use sx9_harness::{StaticGate, ArchGate, PatternGate};

// Run QA gates before marking issue as done
let static_report = StaticGate::default().run(&crate_path).await?;
let arch_report = ArchGate::default().run(&crate_path).await?;
let pattern_report = PatternGate::default().run(&crate_path).await?;

// Notify Slack with results
slack.notify_qa_results(
    &config.slack_channel,
    &issue_id,
    static_report.passed && arch_report.passed,
    &format_qa_summary(&static_report, &arch_report)
).await?;
```

## Environment Variables

```bash
export LINEAR_API_KEY="lin_api_..."
export SLACK_BOT_TOKEN="xoxb-..."
export SERENA_ENDPOINT="http://localhost:8000"
```

## Development

### Run Tests
```bash
cargo test
```

### Run with Debug Logging
```bash
RUST_LOG=sx9_linear_agent=debug cargo run
```

### Format Code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

## Architecture Alignment

This agent aligns with:
- **RFC-9112**: Deterministic Prompt Engineering
- **RFC-9116**: APECS Legion Bridge ECS
- **RFC-9122**: Git Workflow Linear Slack Integration
- **SYNAPTIX-UNIFIED-ARCHITECTURE**: Three-container OrbStack deployment

## Deployment

### OrbStack Containers
1. **sx9-linear-agent** - Main agent loop
2. **serena-mcp** - AI code generation service
3. **nats-jetstream** - Message bus for inter-service communication

### Docker Compose
```yaml
services:
  linear-agent:
    build: .
    environment:
      - LINEAR_API_KEY=${LINEAR_API_KEY}
      - SLACK_BOT_TOKEN=${SLACK_BOT_TOKEN}
      - SERENA_ENDPOINT=http://serena:8000
    volumes:
      - ./:/workspace
  
  serena:
    image: serena-mcp:latest
    ports:
      - "8000:8000"
  
  nats:
    image: nats:latest
    command: -js
    ports:
      - "4222:4222"
```

## License

Proprietary - SDVOSB
