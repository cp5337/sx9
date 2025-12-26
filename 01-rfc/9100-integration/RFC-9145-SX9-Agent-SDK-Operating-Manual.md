# RFC-9145: SX9 Autonomous Agent SDK — Operating Manual

**Status:** DRAFT
**Author:** Charlie E. Payne / Claude
**Date:** 2025-12-25
**Supersedes:** N/A
**Integrates:** RFC-9030, RFC-9141, RFC-9142, CLSGS Annex A

---

## Abstract

RFC-9145 defines the complete operating manual for the SX9 Autonomous Agent SDK — an AI-first Software Development Lifecycle (SDLC) that transforms voice-initiated ideation into production-grade software through a deterministic, auditable pipeline. The system extends the Anthropic Claude API patterns for enterprise-grade agentic long runs with Linear integration, Slack notifications, Zotero scholarly references, and DoD DevSecOps 2024 compliance.

---

## 1. Governing Doctrine

This RFC is bound to and extends:

| RFC | Title | Binding |
|-----|-------|---------|
| RFC-9141 | FORGE Assembly Line & Dual-Heartbeat QA Doctrine | CANONICAL |
| RFC-9142 | Semantic Drift Scoring & Gates | NORMATIVE |
| RFC-9030 | Unified Linear Agent Infrastructure | NORMATIVE |
| CLSGS Annex A | Agentic Structure, Linear Task Integration | NORMATIVE |

**Core Principle (RFC-9141):**
> "Prompts are assembled, not authored. Variable selection precedes generation."

---

## 2. System Overview

### 2.1 Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 AGENT SDK PIPELINE                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  IDEATION (BNE)                                                  │
│  ─────────────                                                   │
│  Voice Input → Transcription → Intent Classification            │
│       ↓                                                          │
│  Zotero Query → 2 Scholarly References                          │
│       ↓                                                          │
│  PoC Test Recommendation                                         │
│       ↓                                                          │
│  SDLC Gate Decision (approve/reject)                             │
│                                                                  │
│  STRUCTURED WORK                                                 │
│  ───────────────                                                 │
│  Linear Issue (Atomic Prompt Unit)                               │
│       ↓                                                          │
│  Agent Assignment (Forge, Axiom, Vector, Sentinel, Guardian)     │
│       ↓                                                          │
│  Claude SDK → Code Generation                                    │
│       ↓                                                          │
│  QA Gates (Static → Arch → Pattern → Semantic)                   │
│       ↓                                                          │
│  Git Workflow (Branch → Commit → PR → Merge)                     │
│       ↓                                                          │
│  Slack Notification                                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Component Registry

| Component | Location | Purpose |
|-----------|----------|---------|
| sx9-claude-sdk | crates/sx9-claude-sdk/ | Native Rust Claude API client |
| sx9-linear-agent | sx9-linear-agent/ | Autonomous coding agent |
| sx9-harness | crates/sx9-harness/ | QA gates and testing |
| sx9-docs-public | sx9-docs-public/ | Docusaurus documentation |
| sx9-workbench | sx9-workbench/ | Dioxus iPad PWA |
| SX9 API Vault | tools/vault/SX9_API_VAULT.json | Single source of truth for keys |

---

## 3. Phase 0: Bar Napkin Engineering (BNE)

### 3.1 Purpose

BNE transforms unstructured ideation into structured, research-backed Linear issues before entering the SDLC.

### 3.2 Input Sources

| Source | Transport | Handler |
|--------|-----------|---------|
| Voice | ElevenLabs STT | VoiceTranscriber |
| Slack | Webhook | SlackMentionHandler |
| Linear Comment | Webhook | LinearCommentHandler |
| CLI | Direct | BneCli |

### 3.3 Thalmic Filter Integration (RFC-9108)

**All voice and text input MUST pass through the Thalmic Filter before processing.**

```
Voice Input → Transcription → THALMIC FILTER → Intent Classification
                                    │
                              ┌─────┴─────┐
                              │           │
                        Clarity ≥ 0.7   Clarity < 0.7
                              │           │
                              ▼           ▼
                         Continue    Ask Clarifying
                                     Questions
```

**Thalmic Filter Checks:**
- Ambiguous pronouns ("it", "this" without antecedent)
- Vague quantifiers ("some", "many", "a few")
- Missing domain context
- Colloquial phrases with multiple interpretations

**Clarity Threshold:** ≥ 0.7 (per RFC-9120)

```rust
pub struct ThalmicResult {
    pub clarity_score: f32,           // 0.0-1.0, must be ≥ 0.7
    pub ambiguities: Vec<Ambiguity>,  // Detected unclear elements
    pub enriched_text: String,        // Clarified version
    pub unicode_op: char,             // U+E54E (THALMIC_VOICE_FILTER)
}
```

### 3.4 High-Level Questions (Forge Portal Alignment)

Instead of modal-by-modal interrogation, ask **three high-level questions** that align with the Forge Portal structure:

**Question 1: What are you building?**
```yaml
purpose: Establish the artifact type and domain
maps_to:
  - initiative.vision_statement
  - project.rfc_reference
  - issue.behavioral_scope.object
example_answers:
  - "A Rust crate for Linear API integration"
  - "A React component for voice transcription"
  - "A security audit workflow"
```

**Question 2: Why does this matter?**
```yaml
purpose: Establish value proposition and success criteria
maps_to:
  - initiative.success_criteria
  - project.poc_test
  - issue.acceptance_criteria
example_answers:
  - "Enables autonomous coding agents to track work"
  - "Reduces manual transcription by 90%"
  - "Achieves C-ATO compliance"
```

**Question 3: What constraints apply?**
```yaml
purpose: Establish boundaries, dependencies, and quality gates
maps_to:
  - issue.behavioral_scope.constraint
  - issue.qa_gates
  - project.sdlc_gate_decision
example_answers:
  - "Must use async-graphql, max 500 LOC"
  - "Must pass all QA gates including semantic"
  - "Must integrate with existing NATS subjects"
```

### 3.5 YAML Output Template

After the three questions, generate a properly annotated YAML/Linear issue:

```yaml
# Auto-generated from BNE flow
# Thalmic clarity score: 0.85
# Generated: 2025-12-25T10:30:00Z

linear_issue:
  type: Issue
  title: "Implement Linear GraphQL client for sx9-claude-sdk"

  # N-V-N-N Behavioral Scope (CLSGS Annex A.2)
  behavioral_scope:
    role: Factory           # N: What role does this play?
    action: generate        # V: What action does it perform?
    constraint: rust_crate  # N: What constraints apply?
    object: api_client      # N: What is being produced?

  # From Question 1: What are you building?
  description: |
    Create a GraphQL client for Linear API integration within the
    sx9-claude-sdk crate. This enables autonomous agents to create,
    update, and query Linear issues programmatically.

  # From Question 2: Why does this matter?
  acceptance_criteria:
    - "Agent can create issues via GraphQL"
    - "Agent can update issue status (Todo → In Progress → Done)"
    - "Agent can add comments for handoff"
    - "Agent can query assigned issues"

  # From Question 3: What constraints apply?
  constraints:
    max_loc: 500
    dependencies:
      - async-graphql
      - reqwest
    patterns:
      - async-http-client

  # QA Gates
  qa_gates:
    - static    # Compile check, clippy
    - arch      # Layer compliance
    - pattern   # N-V-N-N validation
    - semantic  # Intent alignment

  # Agent Assignment
  agent: forge  # Factory agent for code generation

  # Scholarly References (auto-fetched from Zotero)
  scholarly_refs:
    - title: "GraphQL: A Query Language for APIs"
      doi: "10.1145/3183713.3190662"
    - title: "REST vs GraphQL: A Performance Comparison"
      doi: "10.1109/SANER.2019.8667986"

  # PoC Test Recommendation
  poc_test: |
    Create a minimal test that:
    1. Authenticates with Linear API
    2. Creates a test issue
    3. Updates its status
    4. Deletes the test issue
    Expected time: 2 hours

  # Metadata
  labels:
    - bne
    - atomic-prompt
    - forge-portal
  priority: 2  # High
  sdlc_gate: pending
```

### 3.6 Processing Steps

**Step 1: Thalmic Filter**
```rust
// Voice/text passes through Thalmic Filter first
let thalmic_result = thalmic_filter.process(input).await?;

if thalmic_result.clarity_score < 0.7 {
    // Ask clarifying questions based on ambiguities
    return Err(BneError::ClarificationNeeded(thalmic_result.ambiguities));
}
```

**Step 2: High-Level Questions**
```rust
pub struct ForgeQuestions {
    pub what_building: String,      // Question 1
    pub why_matters: String,        // Question 2
    pub constraints: String,        // Question 3
}
```

**Step 3: Intent Classification**
```rust
pub struct Intent {
    pub category: IntentCategory,    // Feature, Bug, Refactor, Research
    pub keywords: Vec<String>,       // Extracted domain terms
    pub urgency: Urgency,            // Normal, High, Critical
    pub complexity: Complexity,      // Simple, Moderate, Complex
}

pub enum IntentCategory {
    Feature,
    Bug,
    Refactor,
    Research,
    Documentation,
    Infrastructure,
}
```

**Step 4: Scholarly Reference Fetch**
```rust
pub struct ScholarlyRef {
    pub title: String,
    pub authors: Vec<String>,
    pub year: u16,
    pub doi: Option<String>,
    pub abstract_text: String,
    pub relevance_score: f32,
}

// Query local Zotero first, fallback to web search
pub async fn fetch_references(keywords: &[String]) -> Result<[ScholarlyRef; 2]>;
```

**Step 5: PoC Test Generation**
```rust
pub struct PocTest {
    pub hypothesis: String,          // What we're testing
    pub success_criteria: Vec<String>,
    pub estimated_effort: Effort,    // Hours, not days
    pub risk_factors: Vec<String>,
}
```

**Step 6: SDLC Gate Decision**
```rust
pub enum SdlcGateDecision {
    Approved { project_id: String },
    Deferred { reason: String, revisit_date: Date },
    Rejected { reason: String },
}
```

### 3.7 Linear Form Templates

**Initiative (Epic-level)**
```yaml
type: Initiative
required_fields:
  - vision_statement: text
  - success_criteria: checklist
  - stakeholders: user_list
  - target_quarter: enum [Q1, Q2, Q3, Q4]
labels: [initiative, planning]
```

**Project (Feature-level)**
```yaml
type: Project
required_fields:
  - parent_initiative: relation
  - rfc_reference: text
  - scholarly_refs: array[2]
  - poc_test: text
  - sdlc_gate_decision: enum [pending, approved, rejected]
labels: [project, bne]
```

**Issue (Task-level — Atomic Prompt Unit)**
```yaml
type: Issue
required_fields:
  - parent_project: relation
  - behavioral_scope:
      role: string      # N: Factory, Analyst, etc.
      action: string    # V: generate, analyze, etc.
      constraint: string # N: rust_crate, algorithm
      object: string    # N: source_code, computation
  - acceptance_criteria: checklist
  - qa_gates: enum[] [static, arch, pattern, semantic]
  - agent_assignment: enum [forge, axiom, vector, sentinel, guardian]
labels: [issue, atomic-prompt]
```

---

## 4. Phase 1: Native Rust Claude SDK

### 4.1 Crate Structure

```
crates/sx9-claude-sdk/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API exports
│   ├── client.rs           # ClaudeClient implementation
│   ├── messages.rs         # Message types
│   ├── tools.rs            # Tool definitions
│   ├── streaming.rs        # SSE response handler
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── context.rs      # Conversation window
│   │   ├── persistent.rs   # Sled-backed storage
│   │   └── semantic.rs     # Vector search
│   ├── mcp/
│   │   ├── mod.rs
│   │   ├── transport.rs    # HTTP and stdio
│   │   └── protocol.rs     # MCP message types
│   └── providers/
│       ├── mod.rs
│       ├── anthropic.rs    # Claude API
│       ├── openai.rs       # GPT API (future)
│       └── gemini.rs       # Gemini API (future)
```

### 4.2 Core Types

```rust
/// Claude API Client
pub struct ClaudeClient {
    api_key: SecretString,
    model: String,
    max_tokens: u32,
    http_client: reqwest::Client,
    memory: Box<dyn MemoryProvider>,
}

impl ClaudeClient {
    /// Send a message and get a response
    pub async fn message(&self, request: MessageRequest) -> Result<MessageResponse>;

    /// Stream a response via SSE
    pub async fn stream(&self, request: MessageRequest) -> Result<MessageStream>;

    /// Add tools to the client
    pub fn with_tools(self, tools: Vec<Tool>) -> Self;
}

/// Message request
pub struct MessageRequest {
    pub messages: Vec<Message>,
    pub system: Option<String>,
    pub max_tokens: Option<u32>,
    pub tools: Vec<Tool>,
    pub tool_choice: Option<ToolChoice>,
}

/// Message response
pub struct MessageResponse {
    pub id: String,
    pub content: Vec<ContentBlock>,
    pub stop_reason: StopReason,
    pub usage: Usage,
}

/// Tool definition
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Memory provider trait
pub trait MemoryProvider: Send + Sync {
    async fn store(&self, key: &str, content: &str) -> Result<()>;
    async fn retrieve(&self, key: &str) -> Result<Option<String>>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>>;
}
```

### 4.3 Configuration

```toml
# Load from environment via tools/vault/setup-api-keys.sh
[claude]
api_key = "${ANTHROPIC_API_KEY}"
model = "claude-opus-4-5-20251101"
max_tokens = 8192

[memory]
provider = "sled"
path = "~/.sx9/memory"
```

---

## 5. Phase 2: Linear Gateway Enhancement

### 5.1 BNE Workflow Module

```rust
// crates/sx9-harness/src/linear/bne.rs

pub struct BneWorkflow {
    claude: ClaudeClient,
    linear: LinearClient,
    zotero: ZoteroClient,
    slack: SlackClient,
}

impl BneWorkflow {
    /// Transform voice ideation into structured Linear issue
    pub async fn process_ideation(&self, input: &str) -> Result<LinearIssue> {
        // 1. Classify intent
        let intent = self.classify_intent(input).await?;

        // 2. Fetch scholarly references
        let refs = self.zotero.search(&intent.keywords, 2).await?;

        // 3. Generate PoC test
        let poc = self.generate_poc_test(&intent).await?;

        // 4. Create Linear issue
        let issue = self.linear.create_issue(LinearIssueCreate {
            title: intent.title,
            description: self.format_bne_description(&intent, &refs, &poc),
            labels: vec!["bne", "pending-sdlc-gate"],
            ..Default::default()
        }).await?;

        // 5. Notify Slack
        self.slack.notify_bne_created(&issue).await?;

        Ok(issue)
    }
}
```

### 5.2 Zotero Integration

```rust
// crates/sx9-harness/src/linear/zotero.rs

pub struct ZoteroClient {
    db_path: PathBuf,  // ~/Zotero/zotero.sqlite
}

impl ZoteroClient {
    /// Search local Zotero library
    pub async fn search(&self, keywords: &[String], limit: usize) -> Result<Vec<ScholarlyRef>> {
        let conn = Connection::open(&self.db_path)?;

        let query = format!(
            r#"
            SELECT items.key, itemDataValues.value AS title
            FROM items
            JOIN itemData ON items.itemID = itemData.itemID
            JOIN itemDataValues ON itemData.valueID = itemDataValues.valueID
            WHERE itemData.fieldID = 1
            AND ({})
            LIMIT ?
            "#,
            keywords.iter()
                .map(|k| format!("itemDataValues.value LIKE '%{}%'", k))
                .collect::<Vec<_>>()
                .join(" OR ")
        );

        // Execute and map results
        // ...
    }
}
```

---

## 6. Phase 3: Agent Loop

### 6.1 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    AGENT ORCHESTRATOR                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │ Initializer  │───▶│    Coder     │───▶│   Handoff    │      │
│  │    Agent     │    │    Agent     │    │    Agent     │      │
│  └──────────────┘    └──────────────┘    └──────────────┘      │
│                                                                  │
│  Initializer: Creates Linear project + issues from spec         │
│  Coder: Picks up issues, implements, runs QA                    │
│  Handoff: Saves session state in Linear comments                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 Agent Loop Implementation

```rust
// sx9-linear-agent/src/agent/loop.rs

pub struct AgentLoop {
    claude: ClaudeClient,
    linear: LinearGateway,
    harness: QaHarness,
    slack: SlackClient,
    nats: NatsClient,
    config: AgentConfig,
}

impl AgentLoop {
    pub async fn run(&self) -> Result<()> {
        let mut iterations = 0;

        loop {
            // Check iteration limit
            if let Some(max) = self.config.max_iterations {
                if iterations >= max {
                    tracing::info!("Max iterations reached, stopping");
                    break;
                }
            }

            // 1. Poll Linear for assigned issues
            let issues = self.linear.get_todo_issues(&self.config.team_id).await?;

            for issue in issues {
                // 2. Claim issue
                self.linear.update_status(&issue.id, LinearState::InProgress).await?;
                self.slack.notify_issue_claimed(&issue).await?;

                // 3. Build context from issue + prior comments
                let context = self.build_context(&issue).await?;

                // 4. Generate code with Claude
                let result = self.claude.message(MessageRequest {
                    messages: context.messages,
                    system: Some(self.get_coding_system_prompt()),
                    tools: self.get_coding_tools(),
                    ..Default::default()
                }).await?;

                // 5. Apply code changes
                self.apply_changes(&result).await?;

                // 6. Run QA gates
                let qa_result = self.harness.run_all_gates(&issue.crate_path).await?;

                if qa_result.all_passed() {
                    // 7. Create git commit + PR
                    let pr = self.create_pr(&issue, &result).await?;

                    // 8. Update Linear
                    self.linear.add_comment(&issue.id, &self.format_pr_comment(&pr)).await?;
                    self.linear.update_status(&issue.id, LinearState::Done).await?;

                    // 9. Notify Slack
                    self.slack.notify_pr_created(&issue, &pr).await?;
                } else {
                    // Handle QA failure
                    self.linear.add_comment(&issue.id, &self.format_qa_failure(&qa_result)).await?;
                    self.slack.notify_qa_failure(&issue, &qa_result).await?;
                }

                iterations += 1;
            }

            // Sleep before next poll
            tokio::time::sleep(Duration::from_secs(self.config.poll_interval)).await;
        }

        Ok(())
    }
}
```

### 6.3 Security Sandbox

```rust
// sx9-linear-agent/src/security/sandbox.rs

pub const ALLOWED_COMMANDS: &[&str] = &[
    "cargo",
    "git",
    "npm",
    "pnpm",
    "rustc",
    "rustfmt",
    "clippy",
    "ls",
    "cat",
    "head",
    "tail",
    "wc",
    "grep",
    "mkdir",
];

pub const FORBIDDEN_PATTERNS: &[&str] = &[
    "rm -rf",
    "sudo",
    "/etc/",
    "/var/",
    "~/.ssh",
    "curl | sh",
    "wget | sh",
];

pub fn validate_command(cmd: &str) -> Result<(), SecurityError> {
    // Check forbidden patterns
    for pattern in FORBIDDEN_PATTERNS {
        if cmd.contains(pattern) {
            return Err(SecurityError::ForbiddenPattern(pattern.to_string()));
        }
    }

    // Extract base command
    let base_cmd = cmd.split_whitespace().next().unwrap_or("");

    // Check allowlist
    if !ALLOWED_COMMANDS.contains(&base_cmd) {
        return Err(SecurityError::CommandNotAllowed(base_cmd.to_string()));
    }

    Ok(())
}
```

---

## 7. Phase 4: DoD DevSecOps Integration

### 7.1 Pipeline Alignment

| DoD Phase | SX9 Implementation |
|-----------|-------------------|
| PLAN | BNE → Scholarly Refs → PoC Test → SDLC Gate |
| CODE | Agent Coding → Static Analysis → Secrets Scan |
| BUILD | Cargo Build → Dependency Audit → SBOM |
| TEST | Unit → Integration → Contract → E2E |
| RELEASE | Semantic Version → Changelog → Sign |
| DEPLOY | Container Build → Scan → Push → Verify |
| OPERATE | Health Checks → Metrics → Alerts |
| MONITOR | SIEM → Audit → Compliance → C-ATO |

### 7.2 Security Gates

```rust
// crates/sx9-harness/src/security/mod.rs

pub struct SecurityHarness {
    secrets_scanner: SecretsScanner,
    dependency_auditor: DependencyAuditor,
    sbom_generator: SbomGenerator,
    container_scanner: ContainerScanner,
}

impl SecurityHarness {
    /// Run all security gates
    pub async fn run_all(&self, path: &Path) -> Result<SecurityReport> {
        let secrets = self.secrets_scanner.scan(path).await?;
        let deps = self.dependency_auditor.audit(path).await?;
        let sbom = self.sbom_generator.generate(path).await?;

        SecurityReport {
            secrets_clean: secrets.findings.is_empty(),
            dependencies_clean: deps.vulnerabilities.is_empty(),
            sbom,
            timestamp: Utc::now(),
        }
    }
}
```

---

## 8. Phase 5: Documentation System

### 8.1 Hybrid Architecture

| Platform | Purpose | Stack |
|----------|---------|-------|
| Docusaurus | Public docs, SEO, API reference | Node.js + React |
| Dioxus Workbench | Internal tools, iPad PWA, offline | Rust + WASM |

### 8.2 Docusaurus Configuration

```javascript
// sx9-docs-public/docusaurus.config.js
module.exports = {
  title: 'SX9 Documentation',
  tagline: 'AI-First Engineering Framework',
  url: 'https://docs.synaptix9.io',

  presets: [
    ['@docusaurus/preset-classic', {
      docs: {
        path: '../01-rfc',  // Auto-import RFCs
        routeBasePath: 'rfc',
      },
    }],
  ],

  plugins: [
    './src/plugins/zotero-citations',  // Custom Zotero integration
    './src/plugins/cargo-doc-import',  // Import rustdoc
  ],
};
```

### 8.3 Dioxus Workbench Features

| Feature | Description |
|---------|-------------|
| Health Dashboard | Agent status, QA gates, NATS health |
| RFC Library | Offline-capable RFC browser with search |
| Linear Board | Real-time Linear issue visualization |
| Agent Monitor | Live agent activity feed |

---

## 9. Configuration Reference

### 9.1 Environment Variables

```bash
# Source from: tools/vault/setup-api-keys.sh

# Claude API
export ANTHROPIC_API_KEY="sk-ant-..."

# Linear
export LINEAR_API_KEY="lin_api_..."
export LINEAR_TEAM_ID="..."

# Slack
export SLACK_BOT_TOKEN="xoxb-..."
export SLACK_CHANNEL="#sx9-dev"

# Database
export DATABASE_URL="postgresql://..."
export SUPABASE_URL="https://..."
export SUPABASE_ANON_KEY="..."
```

### 9.2 Agent Configuration

```toml
# sx9-linear-agent/config/linear.toml

# Keys loaded from environment
linear_api_key = "${LINEAR_API_KEY}"
slack_bot_token = "${SLACK_BOT_TOKEN}"
serena_endpoint = "http://localhost:8000"
team_id = "5f2c1e8a-3b4d-4c9a-8e1f-2d3c4b5a6e7f"
slack_channel = "#sx9-dev"

[agent]
poll_interval = 60
max_concurrent_tasks = 3
auto_assign = true
max_iterations = 100  # Safety limit

[security]
allowed_commands = ["cargo", "git", "npm", "pnpm", "rustc", "rustfmt", "clippy"]
allowed_paths = ["src/", "crates/", "tests/", "Cargo.toml", "package.json"]
forbidden_patterns = ["rm -rf", "sudo", "/etc/", "/var/", "~/.ssh"]

[qa]
enable_static_gate = true
enable_arch_gate = true
enable_pattern_gate = true
enable_semantic_gate = true
min_quality_score = 0.7
```

---

## 10. Operational Procedures

### 10.1 Starting the Agent

```bash
# 1. Load environment variables
source tools/vault/setup-api-keys.sh

# 2. Verify keys are loaded
echo $LINEAR_API_KEY | head -c 10

# 3. Start the agent
cd sx9-linear-agent
cargo run
```

### 10.2 Monitoring

```bash
# Check agent logs
RUST_LOG=sx9_linear_agent=debug cargo run

# Monitor NATS subjects
nats sub "sx9.agent.>"

# Check Linear API health
curl -H "Authorization: Bearer $LINEAR_API_KEY" \
  https://api.linear.app/graphql \
  -d '{"query": "{ viewer { id name } }"}'
```

### 10.3 Troubleshooting

| Issue | Resolution |
|-------|------------|
| Agent not picking up issues | Check `team_id` and `auto_assign` settings |
| QA gates failing | Review gate output, check `min_quality_score` |
| Slack notifications missing | Verify `SLACK_BOT_TOKEN` and channel permissions |
| Memory overflow | Increase `max_tokens` or enable chunking |

---

## 11. Implementation Roadmap

### Week 1-2: Foundation
- [ ] Create `sx9-claude-sdk` crate with HTTP client
- [ ] Implement message and streaming APIs
- [ ] Add to workspace Cargo.toml
- [ ] Basic memory provider (Sled-backed)

### Week 3-4: Linear Gateway
- [ ] BNE workflow module
- [ ] Zotero integration (local SQLite at ~/Zotero/zotero.sqlite)
- [ ] Scholarly reference fetcher
- [ ] PoC test generator
- [ ] SDLC decision gate

### Week 5-6: Agent Loop
- [ ] Complete sx9-linear-agent modules
- [ ] Initializer agent
- [ ] Coder agent
- [ ] Handoff agent

### Week 7-8: DevSecOps
- [ ] Security gates integration
- [ ] SBOM generation
- [ ] Container scanning
- [ ] Compliance checklist

### Week 9-10: EA Automation
- [ ] Figma MCP connector
- [ ] Architecture diagram generation
- [ ] Living documentation pipeline
- [ ] C-ATO dashboard

### Week 11-12: Documentation System
- [ ] Initialize Docusaurus (sx9-docs-public)
- [ ] Migrate ctas7-dioxus-bootstrap → sx9-workbench
- [ ] Health Dashboard
- [ ] RFC Library with offline support

---

## 12. References

- RFC-9030: Unified Linear Agent Infrastructure
- RFC-9141: FORGE Assembly Line & Dual-Heartbeat QA Doctrine
- RFC-9142: Semantic Drift Scoring & Gates
- CLSGS Annex A: Agentic Structure, Linear Task Integration
- DoD DevSecOps Reference Design v2.0 (2024)
- Anthropic Claude API Documentation
- Linear GraphQL API Documentation

---

## 13. Appendices

### A. NATS Subject Hierarchy

```
sx9.agent.claude.request       # Task assignment
sx9.agent.claude.response      # Task completion
sx9.linear.issue.create        # Issue creation
sx9.linear.issue.update        # Status updates
sx9.linear.comment.create      # Handoff comments
sx9.slack.notify               # Slack notifications
sx9.qa.gate.static             # Static QA results
sx9.qa.gate.semantic           # Semantic QA results
```

### B. Default Agent Roster

| Agent | Provider | Role | Action | Constraint | Object |
|-------|----------|------|--------|------------|--------|
| Forge | Claude | Factory | generate | rust_crate | source_code |
| Axiom | Claude | Analyst | analyze | algorithm | computation |
| Vector | Claude | Strategist | plan | architecture | design |
| Sentinel | Claude | Guardian | audit | security | codebase |
| Guardian | Claude | Validator | validate | qa_gate | artifact |

---

**Document Status:** DRAFT
**Next:** Implementation begins after approval

---

**END OF RFC-9145**
