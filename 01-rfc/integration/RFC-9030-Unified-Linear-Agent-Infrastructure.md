# RFC-9030 — Unified Linear Agent Infrastructure

**Version:** 1.0
**Status:** Draft
**Date:** December 2025
**Applies To:** SX9, CTAS-7.3.1, CognetixAlpha
**Author:** CTAS Core Engineering Group
**Dependencies:** RFC-9021, RFC-9004, RFC-9012

---

## 1. Overview

This RFC consolidates the fragmented Linear integration landscape into a unified agent infrastructure that connects:

- **Linear.app** (Project Management)
- **GitHub** (Code Repository + PRs)
- **Claude Code** (IDE Agent)
- **VSCode/Cursor** (Development Environment)
- **Foundation Daemon** (Rust Backend Services)

### 1.1 Current State (Fragmented)

```
CURRENT FRAGMENTATION
─────────────────────

┌─────────────────────────────────────────────────────────────────────────┐
│                     SCATTERED LINEAR INTEGRATIONS                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ctas7-linear-agent-rust/     ← Rust agent (broken deps)               │
│  ├── linear_integration.rs                                              │
│  ├── deep_linear_xsd_integration.rs                                     │
│  └── linear_agent_integration.rs                                        │
│                                                                         │
│  ctas7-linear/                ← TypeScript monorepo                     │
│  ├── packages/linear-webhook-client/                                    │
│  ├── packages/claude-runner/                                            │
│  └── packages/team-coordination/                                        │
│                                                                         │
│  04-abe-iac/abe-qa-system/linear-integration/  ← Python daemon         │
│  ├── linear_atlas_cognitive_node.py                                     │
│  └── linear_qa_daemon.py                                                │
│                                                                         │
│  ctas7-cesium-mcp/src/services/  ← MCP integration                     │
│  └── linear-integration.ts                                              │
│                                                                         │
│  ctas7-repoagent/             ← Multi-LLM agent mesh                   │
│  └── agents/ (altair, claude, gpt, gemini, grok, natasha)              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Target State (Unified)

```
UNIFIED SX9 LINEAR AGENT INFRASTRUCTURE
───────────────────────────────────────

┌─────────────────────────────────────────────────────────────────────────────────┐
│                           LINEAR.APP (SX9 Workspace)                             │
│                           Team: CognetixAlpha → SX9                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                    │                                             │
│                                    │ GraphQL API                                 │
│                                    │ Webhooks                                    │
│                                    ▼                                             │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                    LINEAR GATEWAY SERVICE (Rust)                          │  │
│  │                    Port: 18120                                            │  │
│  │────────────────────────────────────────────────────────────────────────────│  │
│  │                                                                           │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐           │  │
│  │  │ Webhook Handler │  │ GraphQL Client  │  │ OAuth Manager   │           │  │
│  │  │ (Axum Router)   │  │ (async-graphql) │  │ (JWT + Tokens)  │           │  │
│  │  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘           │  │
│  │           │                    │                    │                     │  │
│  │           └────────────────────┼────────────────────┘                     │  │
│  │                                │                                          │  │
│  │                                ▼                                          │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐ │  │
│  │  │                    EVENT BUS (NATS JetStream)                       │ │  │
│  │  │                    Port: 4222                                       │ │  │
│  │  │─────────────────────────────────────────────────────────────────────│ │  │
│  │  │  Subjects:                                                          │ │  │
│  │  │  • linear.issue.created    → Agent dispatch                        │ │  │
│  │  │  • linear.issue.updated    → Status sync                           │ │  │
│  │  │  • linear.comment.created  → AI response trigger                   │ │  │
│  │  │  • linear.pr.linked        → GitHub sync                           │ │  │
│  │  │  • agent.task.completed    → Linear update                         │ │  │
│  │  └─────────────────────────────────────────────────────────────────────┘ │  │
│  │                                │                                          │  │
│  └────────────────────────────────┼──────────────────────────────────────────┘  │
│                                   │                                             │
│         ┌─────────────────────────┼─────────────────────────┐                  │
│         │                         │                         │                  │
│         ▼                         ▼                         ▼                  │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐            │
│  │  CLAUDE CODE    │    │  FOUNDATION     │    │  QA AGENTS      │            │
│  │  AGENT          │    │  DAEMON         │    │  (ABE)          │            │
│  │─────────────────│    │─────────────────│    │─────────────────│            │
│  │                 │    │                 │    │                 │            │
│  │  VSCode/Cursor  │    │  Port: 18100    │    │  Port: 18109    │            │
│  │  Integration    │    │  Rust Services  │    │  Lightning QA   │            │
│  │                 │    │  XSD Orch       │    │                 │            │
│  │  MCP Server     │    │  Playbooks DSL  │    │  Port: 18110    │            │
│  │  Port: 18125    │    │                 │    │  Expert QA      │            │
│  │                 │    │                 │    │                 │            │
│  └────────┬────────┘    └────────┬────────┘    └────────┬────────┘            │
│           │                      │                      │                      │
│           └──────────────────────┼──────────────────────┘                      │
│                                  │                                             │
│                                  ▼                                             │
│  ┌───────────────────────────────────────────────────────────────────────────┐ │
│  │                         GITHUB INTEGRATION                                │ │
│  │────────────────────────────────────────────────────────────────────────────│ │
│  │                                                                           │ │
│  │  • Branch naming: feat/SX9-{issue_id}-{slug}                             │ │
│  │  • PR auto-link to Linear issues                                          │ │
│  │  • Commit message: SX9-{issue_id} prefix                                  │ │
│  │  • PR merge → Linear issue auto-close                                     │ │
│  │  • CI status → Linear sync                                                │ │
│  │                                                                           │ │
│  └───────────────────────────────────────────────────────────────────────────┘ │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Component Architecture

### 2.1 Linear Gateway Service (Rust)

The central coordination point for all Linear operations.

```rust
// ctas7-linear-gateway/src/lib.rs

pub struct LinearGateway {
    /// GraphQL client for Linear API
    graphql_client: LinearGraphQLClient,

    /// Webhook receiver (Axum)
    webhook_router: Router,

    /// OAuth token manager
    oauth_manager: OAuthManager,

    /// NATS event publisher
    event_bus: NatsClient,

    /// Agent registry
    agents: AgentRegistry,

    /// Supabase for persistence
    db: SupabaseClient,
}

impl LinearGateway {
    /// Handle incoming webhook from Linear
    pub async fn handle_webhook(&self, payload: LinearWebhook) -> Result<()> {
        match payload.action.as_str() {
            "create" => self.handle_issue_created(payload).await,
            "update" => self.handle_issue_updated(payload).await,
            "comment" => self.handle_comment_created(payload).await,
            _ => Ok(())
        }
    }

    /// Dispatch task to appropriate agent
    pub async fn dispatch_to_agent(&self, issue: LinearIssue) -> Result<AgentTask> {
        // Route based on labels and project
        let agent = self.agents.select_agent(&issue)?;

        // Publish to NATS for agent pickup
        self.event_bus.publish(
            format!("agent.{}.task", agent.id),
            AgentTask::from_issue(issue)
        ).await
    }
}
```

### 2.2 Claude Code MCP Integration

```typescript
// ctas7-linear-mcp/src/linear-server.ts

import { McpServer } from "@anthropic/claude-code-sdk";
import { LinearClient } from "@linear/sdk";

export class LinearMcpServer extends McpServer {
  private linear: LinearClient;
  private nats: NatsConnection;

  tools = {
    // Create Linear issue from Claude Code
    "linear_create_issue": {
      description: "Create a Linear issue linked to current work",
      parameters: {
        title: { type: "string" },
        description: { type: "string" },
        priority: { type: "number", enum: [1, 2, 3, 4] },
        labels: { type: "array", items: { type: "string" } }
      },
      handler: async (params) => {
        const issue = await this.linear.createIssue({
          teamId: process.env.LINEAR_TEAM_ID,
          title: params.title,
          description: params.description,
          priority: params.priority,
          labelIds: await this.resolveLabelIds(params.labels)
        });

        // Create branch if code work
        if (params.labels?.includes("engineering")) {
          await this.createLinkedBranch(issue);
        }

        return { issueId: issue.id, identifier: issue.identifier };
      }
    },

    // Update issue status from Claude Code
    "linear_update_status": {
      description: "Update Linear issue status",
      parameters: {
        issueId: { type: "string" },
        status: { type: "string", enum: ["backlog", "todo", "in_progress", "done", "canceled"] }
      },
      handler: async (params) => {
        const stateId = await this.resolveStateId(params.status);
        await this.linear.updateIssue(params.issueId, { stateId });
        return { success: true };
      }
    },

    // Get current sprint/cycle issues
    "linear_get_current_work": {
      description: "Get issues assigned to current cycle",
      handler: async () => {
        const me = await this.linear.viewer;
        const issues = await me.assignedIssues({
          filter: {
            cycle: { isActive: { eq: true } }
          }
        });
        return issues.nodes.map(i => ({
          id: i.id,
          identifier: i.identifier,
          title: i.title,
          status: i.state?.name,
          priority: i.priority
        }));
      }
    },

    // Link PR to Linear issue
    "linear_link_pr": {
      description: "Link GitHub PR to Linear issue",
      parameters: {
        issueId: { type: "string" },
        prUrl: { type: "string" }
      },
      handler: async (params) => {
        await this.linear.attachmentCreate({
          issueId: params.issueId,
          url: params.prUrl,
          title: `PR: ${params.prUrl.split('/').pop()}`
        });
        return { success: true };
      }
    }
  };
}
```

### 2.3 Agent Dispatch System

```
AGENT DISPATCH FLOW
───────────────────

Linear Issue Created
        │
        ▼
┌───────────────────────────────────────────────────────────────┐
│                    AGENT CLASSIFIER                           │
│───────────────────────────────────────────────────────────────│
│                                                               │
│  Labels/Project Analysis:                                     │
│                                                               │
│  "bug" + "frontend"     → Claude Code Agent                  │
│  "qa-failure"           → ABE Lightning QA Agent             │
│  "documentation"        → Documentation Agent                 │
│  "infrastructure"       → Foundation Daemon                   │
│  "security"             → Security Review Agent               │
│  "design"               → Design Agent (Figma integration)    │
│                                                               │
│  Priority Routing:                                            │
│  P1 (Urgent)   → Immediate dispatch + Slack alert            │
│  P2 (High)     → Next cycle assignment                       │
│  P3 (Medium)   → Backlog with estimate                       │
│  P4 (Low)      → Backlog                                     │
│                                                               │
└───────────────────────────────────────────────────────────────┘
        │
        ▼
┌───────────────────────────────────────────────────────────────┐
│                    NATS EVENT BUS                             │
│───────────────────────────────────────────────────────────────│
│                                                               │
│  linear.issue.SX9-123 → agent.claude-code.task               │
│                                                               │
│  Payload:                                                     │
│  {                                                            │
│    "issue_id": "SX9-123",                                    │
│    "title": "Fix plasma dashboard SSE connection",            │
│    "description": "...",                                      │
│    "agent_type": "claude-code",                              │
│    "context": {                                              │
│      "repository": "sx9",                │
│      "files_mentioned": ["src/hooks/use-plasma-stream.ts"],  │
│      "branch": "feat/SX9-123-fix-plasma-sse"                 │
│    }                                                         │
│  }                                                           │
│                                                               │
└───────────────────────────────────────────────────────────────┘
        │
        ▼
┌───────────────────────────────────────────────────────────────┐
│                    CLAUDE CODE AGENT                          │
│───────────────────────────────────────────────────────────────│
│                                                               │
│  1. Receive task from NATS                                   │
│  2. Clone/checkout branch                                    │
│  3. Read issue context                                       │
│  4. Execute with Claude Code CLI                             │
│  5. Create PR                                                │
│  6. Update Linear issue with PR link                         │
│  7. Post comment with summary                                │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

---

## 3. Database Schema

### 3.1 Supabase Tables

```sql
-- Linear workspace configuration
CREATE TABLE linear_workspaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id TEXT UNIQUE NOT NULL,  -- Linear workspace ID
    workspace_name TEXT NOT NULL,       -- "SX9"
    team_id TEXT NOT NULL,              -- Linear team ID
    api_key_encrypted TEXT,             -- Encrypted API key
    webhook_secret TEXT,                -- Webhook signature secret
    oauth_client_id TEXT,
    oauth_client_secret_encrypted TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Agent registry
CREATE TABLE linear_agents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID REFERENCES linear_workspaces(id),
    agent_type TEXT NOT NULL,           -- "claude-code", "abe-qa", "foundation"
    agent_name TEXT NOT NULL,           -- "Claude Code Agent"
    endpoint_url TEXT,                  -- Agent service URL
    nats_subject TEXT,                  -- NATS subscription subject
    capabilities JSONB,                 -- Agent capabilities
    status TEXT DEFAULT 'active',       -- active, paused, disabled
    last_heartbeat TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Issue-to-agent assignments
CREATE TABLE linear_agent_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID REFERENCES linear_workspaces(id),
    agent_id UUID REFERENCES linear_agents(id),
    linear_issue_id TEXT NOT NULL,      -- Linear issue ID
    linear_issue_identifier TEXT,       -- "SX9-123"
    task_status TEXT DEFAULT 'pending', -- pending, in_progress, completed, failed
    github_branch TEXT,                 -- Created branch name
    github_pr_url TEXT,                 -- Created PR URL
    agent_output JSONB,                 -- Agent execution result
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Webhook event log
CREATE TABLE linear_webhook_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID REFERENCES linear_workspaces(id),
    event_type TEXT NOT NULL,           -- issue.create, comment.create, etc.
    payload JSONB NOT NULL,             -- Raw webhook payload
    processed BOOLEAN DEFAULT FALSE,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE linear_workspaces ENABLE ROW LEVEL SECURITY;
ALTER TABLE linear_agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE linear_agent_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE linear_webhook_events ENABLE ROW LEVEL SECURITY;
```

---

## 4. Git Workflow Integration

### 4.1 Branch Naming Convention

```
feat/SX9-{issue_number}-{slug}
fix/SX9-{issue_number}-{slug}
docs/SX9-{issue_number}-{slug}
chore/SX9-{issue_number}-{slug}

Examples:
- feat/SX9-123-plasma-cognitive-integration
- fix/SX9-456-sse-reconnection-bug
- docs/SX9-789-api-documentation
```

### 4.2 Commit Message Format

```
SX9-{issue_number}: {description}

{body}

{footer}
```

### 4.3 PR Template

```markdown
## Linear Issue
Closes SX9-{issue_number}

## Summary
{AI-generated summary from agent}

## Changes
- {change 1}
- {change 2}

## Test Plan
- [ ] {test item 1}
- [ ] {test item 2}

## RFC Compliance
- [ ] RFC-{number}: {description}

---
🤖 Generated by {agent_name}
```

---

## 5. Migration Plan

### 5.1 Phase 1: Consolidate (Week 1-2)

1. Create `ctas7-linear-gateway` crate
2. Migrate GraphQL client from `ctas7-linear-agent-rust`
3. Migrate webhook handlers from ABE system
4. Set up NATS JetStream

### 5.2 Phase 2: MCP Integration (Week 3)

1. Create `ctas7-linear-mcp` package
2. Implement Linear tools for Claude Code
3. Test with VSCode/Cursor

### 5.3 Phase 3: Agent Framework (Week 4)

1. Standardize agent interface
2. Connect existing agents to gateway
3. Implement dispatch system

### 5.4 Phase 4: CognetixAlpha → SX9 (Week 5)

1. Rename Linear workspace
2. Update all integrations
3. Migrate existing issues

---

## 6. Port Allocation

| Port | Service | Description |
|------|---------|-------------|
| 18120 | Linear Gateway | Main gateway service |
| 18121 | Linear Webhook | Webhook receiver |
| 18122 | Linear GraphQL Proxy | GraphQL API proxy |
| 18125 | Linear MCP Server | Claude Code MCP |
| 4222 | NATS | Event bus |
| 4223 | NATS JetStream | Persistent events |

---

## 7. Environment Variables

```bash
# Linear API
LINEAR_API_KEY=lin_api_xxxxx
LINEAR_TEAM_ID=xxxxxxxx
LINEAR_WORKSPACE_ID=xxxxxxxx
LINEAR_WEBHOOK_SECRET=whsec_xxxxx

# OAuth (for third-party app)
LINEAR_OAUTH_CLIENT_ID=xxxxx
LINEAR_OAUTH_CLIENT_SECRET=xxxxx
LINEAR_OAUTH_REDIRECT_URI=https://sx9.io/auth/linear/callback

# NATS
NATS_URL=nats://localhost:4222
NATS_CLUSTER_ID=sx9-linear-cluster

# GitHub
GITHUB_TOKEN=ghp_xxxxx
GITHUB_WEBHOOK_SECRET=xxxxx

# Supabase
SUPABASE_URL=https://xxxxx.supabase.co
SUPABASE_ANON_KEY=xxxxx
SUPABASE_SERVICE_KEY=xxxxx
```

---

## 8. References

- Linear API Documentation: https://linear.app/developers
- Linear GraphQL Schema: https://studio.apollographql.com/public/Linear-API
- RFC-9004: Deterministic Routing Architecture
- RFC-9021: Cognitive Inference Engine
- Existing: `ctas7-linear-agent-rust/src/linear_integration.rs`
- Existing: `04-abe-iac/abe-qa-system/linear-integration/`

---

**End of RFC-9030**
