# RFC-9122: Git Workflow, Linear Orchestration & Slack Decision Gates

**Status:** DRAFT  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-20  
**Depends On:** RFC-9120 (Prompt Forge), RFC-9121 (Lightning QA), RFC-9116 (Dev Forge)

---

## Abstract

RFC-9122 specifies the complete Git workflow for the SX9 ecosystem, integrating Linear for issue tracking, Slack for human decision gates, and strict branch isolation to prevent fratricide between human developers and factory agents. The core principle: **nothing touches main without passing all gates, and all work is isolated until proven safe.**

**Core Insight:** Active coding belongs in isolation. The repo is a museum of certified artifacts, not a construction site.

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    GIT WORKFLOW ORCHESTRATION                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                           LINEAR                                      │  │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │  │
│  │  │ Backlog │→│ Ready   │→│ Building│→│ Review  │→│  Done   │        │  │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │              │            │           │           │               │
│         │              │            │           │           │               │
│         ▼              ▼            ▼           ▼           ▼               │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                            GIT                                        │  │
│  │                                                                       │  │
│  │    main ─────────────────────────────────────────────────────────▶   │  │
│  │      │                                              ▲                 │  │
│  │      │                                              │ (merge)         │  │
│  │      ▼                                              │                 │  │
│  │  develop ──────────────────────────────────────────►│                │  │
│  │      │                              ▲               │                 │  │
│  │      │                              │ (PR)          │                 │  │
│  │      ▼                              │               │                 │  │
│  │  feature/* ─────────────────────────┘               │                 │  │
│  │  factory/* ─────────────────────────────────────────┘                │  │
│  │                                                                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │              │            │           │           │               │
│         ▼              ▼            ▼           ▼           ▼               │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                           SLACK                                       │  │
│  │  #sx9-factory    #sx9-review    #sx9-alerts    #sx9-decisions        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Branch Strategy

### 2.1 Branch Hierarchy

| Branch | Purpose | Protection | Who Writes |
|--------|---------|------------|------------|
| `main` | Production-ready, registry source | Full protection, no direct push | Merge only via PR from `develop` |
| `develop` | Integration branch, QA-passed code | Protected, PR required | Merge from feature/factory branches |
| `feature/*` | Human developer work | None | Developers |
| `factory/*` | Factory agent work | None | Factory agents only |
| `hotfix/*` | Emergency production fixes | Expedited review | Senior developers |
| `experiment/*` | R&D, throwaway | None, never merges | Anyone |

### 2.2 Branch Naming Convention

```
# Human development
feature/{linear-id}-{short-description}
feature/SX9-123-add-nats-router

# Factory agent
factory/{linear-id}-{crate-name}
factory/SX9-456-sx9-nats-router

# Hotfix
hotfix/{linear-id}-{issue}
hotfix/SX9-789-fix-auth-bypass

# Experiment (never merges)
experiment/{owner}-{concept}
experiment/cpayne-quantum-routing
```

### 2.3 Branch Lifecycle

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         BRANCH LIFECYCLE                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CREATE ──────────────────────────────────────────────────────────────────▶│
│    │  Linear issue moves to "Building"                                     │
│    │  Branch created from develop                                          │
│    │  Slack: #sx9-factory "Branch created: factory/SX9-456-..."           │
│    │                                                                        │
│  WORK ────────────────────────────────────────────────────────────────────▶│
│    │  Commits pushed to feature/factory branch                             │
│    │  CI runs on every push (lint, test, partial QA)                       │
│    │  No notification unless failure                                       │
│    │                                                                        │
│  READY ───────────────────────────────────────────────────────────────────▶│
│    │  Developer/Agent marks work complete                                  │
│    │  Linear issue moves to "Review"                                       │
│    │  PR created to develop                                                │
│    │  Slack: #sx9-review "PR ready: [title] - [link]"                     │
│    │                                                                        │
│  REVIEW ──────────────────────────────────────────────────────────────────▶│
│    │  Lightning QA runs (full analysis)                                    │
│    │  Birth certificate validated                                          │
│    │  Human review if required                                             │
│    │  Slack: #sx9-decisions if human approval needed                      │
│    │                                                                        │
│  MERGE ───────────────────────────────────────────────────────────────────▶│
│    │  PR merged to develop                                                 │
│    │  Branch deleted                                                       │
│    │  Linear issue moves to "Done"                                         │
│    │  Slack: #sx9-factory "Merged: [crate] - Grade [A]"                   │
│    │                                                                        │
│  RELEASE ─────────────────────────────────────────────────────────────────▶│
│    │  develop merged to main (scheduled or manual)                         │
│    │  Registry updated                                                     │
│    │  Slack: #sx9-alerts "Released: [version]"                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Linear Issue Lifecycle

### 3.1 Issue States

| State | Description | Trigger | Next States |
|-------|-------------|---------|-------------|
| **Backlog** | Idea captured, not prioritized | Manual creation | Ready, Cancelled |
| **Ready** | Prioritized, spec complete | Manual triage | Building |
| **Building** | Active development | Branch created | Review, Blocked |
| **Blocked** | Waiting on dependency | Manual flag | Building |
| **Review** | PR open, awaiting approval | PR created | Done, Building (rework) |
| **Done** | Merged and released | PR merged | - |
| **Cancelled** | Won't do | Manual decision | - |

### 3.2 Issue Labels

```yaml
# Source labels (who/what creates the work)
labels:
  source:
    - forge:prompt      # Created by Prompt Forge
    - forge:factory     # Created by Factory Agent
    - human:feature     # Human-initiated feature
    - human:bugfix      # Human-initiated fix
    - auto:qa           # Created by QA failure
    - auto:security     # Created by security scan

# Type labels
  type:
    - crate:new         # New crate creation
    - crate:refactor    # Refactoring existing crate
    - crate:feature     # Feature addition
    - crate:fix         # Bug fix
    - infra:pipeline    # CI/CD changes
    - docs:rfc          # RFC work

# Priority labels
  priority:
    - P0:critical       # Drop everything
    - P1:high           # This sprint
    - P2:medium         # Next sprint
    - P3:low            # Backlog

# Gate labels (for workflow automation)
  gate:
    - gate:qa-required      # Must pass Lightning QA
    - gate:human-review     # Requires human approval
    - gate:security-review  # Requires security review
    - gate:arch-review      # Requires architecture review
```

### 3.3 Linear Automation Rules

```yaml
# Automation rules in Linear

automations:
  # When branch is created, move to Building
  - trigger: "branch_created"
    action: "move_to_state"
    state: "Building"
    notify:
      slack: "#sx9-factory"
      message: "🔨 Building started: {issue.title}"

  # When PR is opened, move to Review
  - trigger: "pr_opened"
    action: "move_to_state"
    state: "Review"
    add_label: "gate:qa-required"
    notify:
      slack: "#sx9-review"
      message: "👀 Review needed: {issue.title}\n{pr.url}"

  # When PR is merged, move to Done
  - trigger: "pr_merged"
    action: "move_to_state"
    state: "Done"
    notify:
      slack: "#sx9-factory"
      message: "✅ Merged: {issue.title}"

  # When QA fails, add label and notify
  - trigger: "qa_failed"
    action: "add_label"
    label: "auto:qa"
    notify:
      slack: "#sx9-alerts"
      message: "❌ QA Failed: {issue.title}\nGrade: {qa.grade}\n{qa.report_url}"

  # When blocked, notify
  - trigger: "state_changed_to_blocked"
    action: "notify"
    notify:
      slack: "#sx9-alerts"
      message: "🚧 Blocked: {issue.title}\nReason: {issue.blocking_reason}"
```

---

## 4. PR Workflow

### 4.1 PR Creation

```yaml
# PR Template (.github/PULL_REQUEST_TEMPLATE.md)

## Summary
<!-- What does this PR do? -->

## Linear Issue
<!-- Link to Linear issue -->
Closes: SX9-{number}

## Type
- [ ] New crate
- [ ] Feature
- [ ] Refactor
- [ ] Fix
- [ ] Documentation

## Source
- [ ] Human developer
- [ ] Factory agent
- [ ] Prompt Forge

## Checklist
- [ ] Birth certificate present (`crate_interview.json`)
- [ ] Smart manifest present (`smartcrate.toml`)
- [ ] Lightning QA passed (Grade A)
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No secrets committed

## QA Report
<!-- Auto-populated by CI -->
Grade: {pending}
Score: {pending}
Report: {pending}

## Decision Required
- [ ] None (auto-merge eligible)
- [ ] Human review required
- [ ] Architecture review required
- [ ] Security review required
```

### 4.2 PR Checks (CI Pipeline)

```yaml
# .github/workflows/pr-checks.yml

name: PR Checks

on:
  pull_request:
    branches: [develop, main]

jobs:
  # Gate 1: Basic validation
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Check branch naming
        run: |
          BRANCH="${{ github.head_ref }}"
          if [[ ! "$BRANCH" =~ ^(feature|factory|hotfix)/SX9-[0-9]+-.*$ ]]; then
            echo "::error::Invalid branch name: $BRANCH"
            exit 1
          fi
      
      - name: Check Linear issue exists
        run: |
          ISSUE_ID=$(echo "${{ github.head_ref }}" | grep -oP 'SX9-\d+')
          # Call Linear API to verify issue exists and is in correct state
          sx9-linear-check --issue "$ISSUE_ID" --expected-state "Review"

  # Gate 2: Build and test
  build:
    needs: validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Build
        run: cargo build --all
      
      - name: Test
        run: cargo test --all
      
      - name: Clippy
        run: cargo clippy --all -- -D warnings

  # Gate 3: Birth certificate validation
  birth-certificate:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Find new/modified crates
        id: crates
        run: |
          CRATES=$(git diff --name-only origin/develop | grep -oP '^crates/[^/]+' | sort -u)
          echo "crates=$CRATES" >> $GITHUB_OUTPUT
      
      - name: Validate birth certificates
        run: |
          for crate in ${{ steps.crates.outputs.crates }}; do
            if [[ ! -f "$crate/crate_interview.json" ]]; then
              echo "::error::Missing birth certificate: $crate/crate_interview.json"
              exit 1
            fi
            sx9-validate-interview "$crate/crate_interview.json"
          done

  # Gate 4: Lightning QA
  lightning-qa:
    needs: birth-certificate
    runs-on: ubuntu-latest
    outputs:
      grade: ${{ steps.qa.outputs.grade }}
      score: ${{ steps.qa.outputs.score }}
      report_url: ${{ steps.qa.outputs.report_url }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Run Lightning QA
        id: qa
        run: |
          for crate in ${{ needs.birth-certificate.outputs.crates }}; do
            RESULT=$(sx9-lightning-qa analyze "$crate" --format json)
            GRADE=$(echo "$RESULT" | jq -r '.grade')
            SCORE=$(echo "$RESULT" | jq -r '.final_score')
            
            echo "grade=$GRADE" >> $GITHUB_OUTPUT
            echo "score=$SCORE" >> $GITHUB_OUTPUT
            
            # Upload report
            REPORT_URL=$(sx9-upload-report "$RESULT")
            echo "report_url=$REPORT_URL" >> $GITHUB_OUTPUT
            
            if [[ "$GRADE" != "A" ]]; then
              echo "::error::QA Grade $GRADE (need A). Score: $SCORE"
              exit 1
            fi
          done
      
      - name: Update PR description
        uses: actions/github-script@v7
        with:
          script: |
            const body = context.payload.pull_request.body;
            const updated = body
              .replace('Grade: {pending}', 'Grade: ${{ steps.qa.outputs.grade }}')
              .replace('Score: {pending}', 'Score: ${{ steps.qa.outputs.score }}')
              .replace('Report: {pending}', 'Report: ${{ steps.qa.outputs.report_url }}');
            
            github.rest.pulls.update({
              owner: context.repo.owner,
              repo: context.repo.repo,
              pull_number: context.payload.pull_request.number,
              body: updated
            });

  # Gate 5: Security scan
  security:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Cargo audit
        run: cargo audit
      
      - name: Check for secrets
        run: trufflehog --only-verified .

  # Decision gate: Determine if human review needed
  decision:
    needs: [lightning-qa, security]
    runs-on: ubuntu-latest
    outputs:
      needs_human: ${{ steps.decide.outputs.needs_human }}
    steps:
      - name: Determine review requirements
        id: decide
        run: |
          # Factory agents creating new crates need human review
          if [[ "${{ github.head_ref }}" == factory/* ]]; then
            if [[ "${{ github.event.pull_request.additions }}" -gt 500 ]]; then
              echo "needs_human=true" >> $GITHUB_OUTPUT
              exit 0
            fi
          fi
          
          # PRs touching core infrastructure need review
          CORE_TOUCHED=$(git diff --name-only origin/develop | grep -c '^crates/sx9-core/')
          if [[ "$CORE_TOUCHED" -gt 0 ]]; then
            echo "needs_human=true" >> $GITHUB_OUTPUT
            exit 0
          fi
          
          echo "needs_human=false" >> $GITHUB_OUTPUT

  # Notify Slack
  notify:
    needs: [lightning-qa, security, decision]
    runs-on: ubuntu-latest
    steps:
      - name: Notify review channel
        if: needs.decision.outputs.needs_human == 'true'
        run: |
          sx9-slack-notify \
            --channel "#sx9-decisions" \
            --message "🔔 Human Review Required\n\nPR: ${{ github.event.pull_request.html_url }}\nTitle: ${{ github.event.pull_request.title }}\nGrade: ${{ needs.lightning-qa.outputs.grade }}\n\nReact with ✅ to approve or ❌ to request changes."
      
      - name: Notify factory channel
        if: needs.decision.outputs.needs_human == 'false'
        run: |
          sx9-slack-notify \
            --channel "#sx9-factory" \
            --message "🤖 Auto-merge eligible\n\nPR: ${{ github.event.pull_request.html_url }}\nGrade: ${{ needs.lightning-qa.outputs.grade }}"

  # Auto-merge if eligible
  auto-merge:
    needs: [lightning-qa, security, decision]
    if: needs.decision.outputs.needs_human == 'false'
    runs-on: ubuntu-latest
    steps:
      - name: Enable auto-merge
        uses: peter-evans/enable-pull-request-automerge@v3
        with:
          pull-request-number: ${{ github.event.pull_request.number }}
          merge-method: squash
```

### 4.3 PR States

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PR STATE MACHINE                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OPENED ──────────────────────────────────────────────────────────────────▶│
│     │                                                                       │
│     ▼                                                                       │
│  ┌─────────┐    PASS     ┌─────────┐    PASS     ┌─────────┐              │
│  │ Validate│───────────▶ │  Build  │───────────▶ │  Birth  │              │
│  └─────────┘             └─────────┘             │  Cert   │              │
│       │                       │                  └─────────┘              │
│       │ FAIL                  │ FAIL                  │                    │
│       ▼                       ▼                       │ PASS               │
│  ┌─────────┐             ┌─────────┐                 ▼                    │
│  │ BLOCKED │             │ BLOCKED │            ┌─────────┐               │
│  │ (naming)│             │ (build) │            │Lightning│               │
│  └─────────┘             └─────────┘            │   QA    │               │
│                                                  └─────────┘               │
│                                                       │                    │
│                               ┌───────────────────────┼───────────────┐   │
│                               │                       │               │   │
│                               ▼                       ▼               ▼   │
│                          Grade A              Grade B/C          Grade D/F │
│                               │                   │                   │   │
│                               ▼                   ▼                   ▼   │
│                         ┌─────────┐         ┌─────────┐         ┌───────┐ │
│                         │ Decision│         │ REFACTOR│         │ REJECT│ │
│                         │  Gate   │         │  LOOP   │         │       │ │
│                         └─────────┘         └─────────┘         └───────┘ │
│                               │                                           │
│               ┌───────────────┴───────────────┐                          │
│               │                               │                          │
│               ▼                               ▼                          │
│          Auto-Merge                    Human Review                      │
│          Eligible                       Required                         │
│               │                               │                          │
│               │                    ┌──────────┴──────────┐              │
│               │                    │                     │              │
│               │                    ▼                     ▼              │
│               │               ✅ Approved           ❌ Changes          │
│               │                    │                 Requested          │
│               │                    │                     │              │
│               ▼                    ▼                     ▼              │
│          ┌─────────┐          ┌─────────┐          ┌─────────┐         │
│          │  MERGE  │          │  MERGE  │          │ REWORK  │         │
│          └─────────┘          └─────────┘          └─────────┘         │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Slack Integration

### 5.1 Channel Structure

| Channel | Purpose | Who Posts | Who Reads |
|---------|---------|-----------|-----------|
| `#sx9-factory` | Factory agent status, build updates | Bots, CI | Everyone |
| `#sx9-review` | PRs ready for review | CI | Reviewers |
| `#sx9-decisions` | Human approval required | CI | Decision makers |
| `#sx9-alerts` | Failures, security issues, blocks | CI, Monitors | Everyone |
| `#sx9-releases` | Production releases | Release bot | Everyone |

### 5.2 Message Types

```yaml
# Message templates

messages:
  # Factory status
  factory_started:
    channel: "#sx9-factory"
    template: |
      🔨 *Factory Build Started*
      Issue: <{linear_url}|{issue_id}: {issue_title}>
      Branch: `{branch_name}`
      Pattern: {pattern}
      Agent: {agent_id}
    
  factory_progress:
    channel: "#sx9-factory"
    template: |
      ⏳ *Build Progress*
      Issue: <{linear_url}|{issue_id}>
      Step: {current_step}/{total_steps}
      Status: {status}

  factory_success:
    channel: "#sx9-factory"
    template: |
      ✅ *Build Complete*
      Issue: <{linear_url}|{issue_id}: {issue_title}>
      Crate: `{crate_name}`
      QA Grade: *{grade}* ({score}/100)
      PR: <{pr_url}|#{pr_number}>

  # Review requests
  review_needed:
    channel: "#sx9-review"
    template: |
      👀 *Review Requested*
      PR: <{pr_url}|{pr_title}>
      Author: {author}
      Grade: {grade}
      Changes: +{additions}/-{deletions}
      
      <{pr_url}|View PR>

  # Decision gates
  decision_required:
    channel: "#sx9-decisions"
    template: |
      🔔 *Human Approval Required*
      
      PR: <{pr_url}|{pr_title}>
      Reason: {reason}
      
      *QA Summary:*
      • Grade: {grade}
      • Score: {score}/100
      • Violations: {violation_count}
      
      React with:
      ✅ Approve
      ❌ Request Changes
      🔍 Need More Info
      
      _Decision required within 24 hours_
    reactions:
      - emoji: "✅"
        action: "approve_pr"
      - emoji: "❌"
        action: "request_changes"
      - emoji: "🔍"
        action: "add_comment"

  # Alerts
  qa_failed:
    channel: "#sx9-alerts"
    template: |
      ❌ *QA Failed*
      Issue: <{linear_url}|{issue_id}: {issue_title}>
      Grade: *{grade}* ({score}/100)
      
      *Critical Violations:*
      {violations}
      
      *Refactor Directives:*
      {directives}
      
      <{report_url}|Full Report>

  blocked:
    channel: "#sx9-alerts"
    template: |
      🚧 *Issue Blocked*
      Issue: <{linear_url}|{issue_id}: {issue_title}>
      Reason: {blocking_reason}
      Blocked By: {blocking_issues}
      
      _Assign someone to unblock_

  security_alert:
    channel: "#sx9-alerts"
    template: |
      🚨 *Security Alert*
      Issue: <{linear_url}|{issue_id}>
      Severity: *{severity}*
      
      *Vulnerability:*
      {vulnerability_description}
      
      *Affected:*
      {affected_crates}
      
      _Immediate action required_

  # Releases
  release_staged:
    channel: "#sx9-releases"
    template: |
      📦 *Release Staged*
      Version: `{version}`
      Crates: {crate_count}
      
      *Included:*
      {crate_list}
      
      *Changes:*
      {changelog_summary}
      
      React with 🚀 to deploy to production

  release_deployed:
    channel: "#sx9-releases"
    template: |
      🚀 *Released to Production*
      Version: `{version}`
      Deployed by: {deployer}
      Time: {timestamp}
      
      Registry updated.
```

### 5.3 Slack Bot Commands

```yaml
# Slack bot slash commands

commands:
  /sx9-status:
    description: "Get current factory/pipeline status"
    usage: "/sx9-status [issue-id]"
    response: |
      *SX9 Factory Status*
      Active builds: {active_count}
      PRs awaiting review: {review_count}
      Blocked issues: {blocked_count}
      
      {issue_details if issue_id}

  /sx9-approve:
    description: "Approve a pending PR"
    usage: "/sx9-approve SX9-123"
    permissions: ["reviewer", "admin"]
    action: "approve_pr"

  /sx9-reject:
    description: "Reject a pending PR with reason"
    usage: "/sx9-reject SX9-123 \"Needs refactoring\""
    permissions: ["reviewer", "admin"]
    action: "request_changes"

  /sx9-unblock:
    description: "Mark an issue as unblocked"
    usage: "/sx9-unblock SX9-123"
    permissions: ["developer", "admin"]
    action: "unblock_issue"

  /sx9-release:
    description: "Trigger a release"
    usage: "/sx9-release [version]"
    permissions: ["admin"]
    action: "trigger_release"

  /sx9-halt:
    description: "Emergency halt all factory agents"
    usage: "/sx9-halt \"reason\""
    permissions: ["admin"]
    action: "halt_factory"
```

### 5.4 Decision Workflow via Slack

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    SLACK DECISION WORKFLOW                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. CI posts to #sx9-decisions ──────────────────────────────────────────▶ │
│     │                                                                       │
│     │  "🔔 Human Approval Required"                                        │
│     │  [PR details]                                                         │
│     │  React: ✅ ❌ 🔍                                                      │
│     │                                                                       │
│  2. Human reacts ─────────────────────────────────────────────────────────▶│
│     │                                                                       │
│     ├── ✅ Approve ──────────────────────────────────────────────────────▶ │
│     │       │                                                               │
│     │       ▼                                                               │
│     │   Bot calls GitHub API: approve PR                                   │
│     │   Bot updates message: "✅ Approved by @user"                        │
│     │   PR auto-merges                                                     │
│     │   Linear issue → Done                                                │
│     │                                                                       │
│     ├── ❌ Request Changes ──────────────────────────────────────────────▶ │
│     │       │                                                               │
│     │       ▼                                                               │
│     │   Bot opens dialog: "What changes are needed?"                       │
│     │   User provides feedback                                             │
│     │   Bot posts comment to PR                                            │
│     │   Bot updates message: "❌ Changes requested by @user"              │
│     │   Linear issue → Building (rework)                                   │
│     │   Slack: #sx9-factory "Rework needed: [issue]"                      │
│     │                                                                       │
│     └── 🔍 Need More Info ───────────────────────────────────────────────▶ │
│             │                                                               │
│             ▼                                                               │
│         Bot opens dialog: "What info do you need?"                         │
│         User provides question                                             │
│         Bot posts comment to PR                                            │
│         Bot updates message: "🔍 Info requested by @user"                 │
│         Factory agent or developer responds                                │
│         Cycle back to decision                                             │
│                                                                             │
│  3. Timeout (24 hours) ───────────────────────────────────────────────────▶│
│     │                                                                       │
│     ▼                                                                       │
│  Escalation: Post to #sx9-alerts                                           │
│  "⏰ Decision overdue: [PR] - needs attention"                             │
│  Tag: @channel                                                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Factory Agent Isolation

### 6.1 Agent Branch Isolation

```yaml
# Factory agents NEVER touch develop or main directly

agent_rules:
  branch_creation:
    base: "develop"
    prefix: "factory/"
    naming: "factory/{issue_id}-{crate_name}"
    
  commits:
    author: "sx9-factory-agent[bot]"
    email: "factory@sx9.dev"
    sign: true  # GPG signed
    
  push:
    allowed_branches:
      - "factory/*"
    forbidden_branches:
      - "main"
      - "develop"
      - "feature/*"
      - "hotfix/*"
    
  pr:
    target: "develop"
    auto_create: true
    labels:
      - "forge:factory"
      - "gate:qa-required"
```

### 6.2 Agent Credentials

```yaml
# Factory agent authentication

agent_auth:
  git:
    type: "github_app"
    app_id: "sx9-factory-agent"
    permissions:
      - "contents:write"      # Push to factory/* branches
      - "pull_requests:write" # Create PRs
      - "issues:read"         # Read Linear-synced issues
    
  linear:
    type: "api_key"
    scopes:
      - "issues:read"
      - "issues:write"
      - "comments:write"
    
  slack:
    type: "bot_token"
    scopes:
      - "chat:write"
      - "reactions:read"
```

### 6.3 Agent Workspace Isolation

```
# Each factory agent gets isolated workspace

/factory-workspaces/
├── agent-001/
│   ├── .git/           # Isolated git config
│   ├── workspace/      # Current build
│   └── artifacts/      # Build outputs
├── agent-002/
│   └── ...
└── shared/
    ├── cache/          # Shared cargo cache
    └── registry/       # Local crate registry mirror
```

---

## 7. Conflict Prevention

### 7.1 Branch Locking

```yaml
# Prevent multiple agents/developers from working on same issue

locking:
  mechanism: "linear_assignment"
  
  rules:
    # Issue can only have one assignee at a time
    - when: "issue.state == 'Building'"
      lock:
        type: "exclusive"
        holder: "{issue.assignee}"
        scope: "branch"
    
    # Factory agent claims via assignment
    - when: "agent.starts_build"
      action: "assign_issue_to_agent"
    
    # Release lock when PR merged or abandoned
    - when: "pr.merged OR pr.closed"
      action: "unassign_issue"
      action: "delete_branch"
```

### 7.2 Crate Ownership

```toml
# CODEOWNERS file

# Core crates require senior review
/crates/sx9-core/**  @cpayne @senior-devs

# Factory-generated crates auto-assign
/crates/sx9-nats-*   @sx9-factory-agent
/crates/sx9-tool-*   @sx9-factory-agent

# Infrastructure requires admin
/.github/**          @cpayne
/terraform/**        @cpayne
```

### 7.3 Merge Queue

```yaml
# GitHub merge queue configuration

merge_queue:
  enabled: true
  
  rules:
    # Batch merges to reduce conflicts
    batch_size: 5
    wait_time: "5m"
    
    # Require clean rebase
    update_method: "rebase"
    
    # Re-run checks on merge
    checks:
      - "validate"
      - "build"
      - "lightning-qa"
      - "security"
```

---

## 8. Release Workflow

### 8.1 Release Types

| Type | Trigger | Target | Approval |
|------|---------|--------|----------|
| **Continuous** | Every merge to develop | Staging | Automatic |
| **Scheduled** | Daily/Weekly cron | Production | Automatic if all green |
| **Manual** | Slack command | Production | Admin approval |
| **Hotfix** | Emergency | Production | Expedited review |

### 8.2 Release Pipeline

```yaml
# .github/workflows/release.yml

name: Release

on:
  schedule:
    - cron: '0 10 * * 1'  # Weekly Monday 10am
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release'
        required: true

jobs:
  prepare:
    runs-on: ubuntu-latest
    outputs:
      version: ${{ steps.version.outputs.version }}
      changelog: ${{ steps.changelog.outputs.changelog }}
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Calculate version
        id: version
        run: |
          if [[ -n "${{ github.event.inputs.version }}" ]]; then
            echo "version=${{ github.event.inputs.version }}" >> $GITHUB_OUTPUT
          else
            # Auto-increment based on conventional commits
            VERSION=$(conventional-changelog --next-version)
            echo "version=$VERSION" >> $GITHUB_OUTPUT
          fi
      
      - name: Generate changelog
        id: changelog
        run: |
          CHANGELOG=$(conventional-changelog -p angular)
          echo "changelog<<EOF" >> $GITHUB_OUTPUT
          echo "$CHANGELOG" >> $GITHUB_OUTPUT
          echo "EOF" >> $GITHUB_OUTPUT

  notify-staging:
    needs: prepare
    runs-on: ubuntu-latest
    steps:
      - name: Post to Slack
        run: |
          sx9-slack-notify \
            --channel "#sx9-releases" \
            --message "📦 Release Staged\n\nVersion: ${{ needs.prepare.outputs.version }}\n\n${{ needs.prepare.outputs.changelog }}\n\nReact with 🚀 to deploy"

  wait-for-approval:
    needs: [prepare, notify-staging]
    runs-on: ubuntu-latest
    steps:
      - name: Wait for Slack reaction
        id: approval
        run: |
          # Poll Slack for 🚀 reaction (max 24 hours)
          APPROVED=$(sx9-slack-wait-reaction --emoji "🚀" --timeout 86400)
          echo "approved=$APPROVED" >> $GITHUB_OUTPUT
      
      - name: Check approval
        if: steps.approval.outputs.approved != 'true'
        run: |
          echo "::error::Release not approved within 24 hours"
          exit 1

  deploy:
    needs: [prepare, wait-for-approval]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Merge develop to main
        run: |
          git checkout main
          git merge develop --no-ff -m "Release ${{ needs.prepare.outputs.version }}"
          git push origin main
      
      - name: Tag release
        run: |
          git tag -a "v${{ needs.prepare.outputs.version }}" -m "Release ${{ needs.prepare.outputs.version }}"
          git push origin "v${{ needs.prepare.outputs.version }}"
      
      - name: Publish to registry
        run: |
          for crate in $(find crates -name "Cargo.toml" -exec dirname {} \;); do
            cargo publish --manifest-path "$crate/Cargo.toml"
          done
      
      - name: Notify success
        run: |
          sx9-slack-notify \
            --channel "#sx9-releases" \
            --message "🚀 Released to Production\n\nVersion: ${{ needs.prepare.outputs.version }}\nRegistry updated."
```

---

## 9. Integration Summary

### 9.1 Prompt Forge Flow (RFC-9120)

```
User speaks intent
       │
       ▼
┌─────────────┐
│ Prompt Forge│
│  (Thalmic)  │
└──────┬──────┘
       │
       ▼
┌─────────────┐    Linear: Create issue (state: Ready)
│  Pattern    │───▶ Slack: #sx9-factory "Intent captured"
│  Resolution │
└──────┬──────┘
       │
       ▼
┌─────────────┐    Linear: Move to Building
│  Interview  │───▶ Slack: #sx9-factory "Build started"
│  Population │    Git: Create factory/SX9-XXX-crate branch
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Canonical  │
│   Prompt    │
└──────┬──────┘
       │
       ▼
   [Factory Agent]
```

### 9.2 Factory + QA Flow (RFC-9121)

```
   [Canonical Prompt]
       │
       ▼
┌─────────────┐
│  Factory    │    Git: Commits to factory/* branch
│   Agent     │───▶ Slack: #sx9-factory "Building..."
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    Self     │    Generate crate_interview.json
│  Interview  │───▶ Generate smartcrate.toml
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Lightning  │    Grade A ──▶ Continue
│     QA      │───▶ Grade B/C ──▶ Refactor loop
└──────┬──────┘    Grade D/F ──▶ HALT + Slack alert
       │
       ▼
┌─────────────┐    Git: Create PR to develop
│  PR Created │───▶ Linear: Move to Review
└──────┬──────┘    Slack: #sx9-review "PR ready"
       │
       ▼
┌─────────────┐
│  CI Checks  │    All pass ──▶ Decision gate
└──────┬──────┘    Any fail ──▶ Block + Slack alert
       │
       ▼
┌─────────────┐    Auto-eligible ──▶ Auto-merge
│  Decision   │───▶ Human required ──▶ Slack: #sx9-decisions
│    Gate     │    
└──────┬──────┘
       │
       ▼
┌─────────────┐    Git: Merge to develop
│   Merged    │───▶ Git: Delete branch
└──────┬──────┘    Linear: Move to Done
       │           Slack: #sx9-factory "Merged"
       ▼
   [Develop branch updated]
```

### 9.3 Release Flow

```
   [Develop branch]
       │
       ▼
┌─────────────┐    Slack: #sx9-releases "Release staged"
│   Staged    │───▶ Post: React 🚀 to deploy
└──────┬──────┘
       │
       │ (Wait for 🚀 reaction)
       │
       ▼
┌─────────────┐    Git: Merge develop → main
│  Approved   │───▶ Git: Tag version
└──────┬──────┘    Registry: Publish crates
       │
       ▼
┌─────────────┐    Slack: #sx9-releases "Released"
│  Released   │───▶ Linear: Close milestone
└─────────────┘
```

---

## 10. Implementation Checklist

### Phase 1: Git Setup (Week 1)

- [ ] Branch protection rules (main, develop)
- [ ] CODEOWNERS file
- [ ] PR template
- [ ] Branch naming validation

### Phase 2: CI Pipeline (Week 2)

- [ ] PR checks workflow
- [ ] Birth certificate validation
- [ ] Lightning QA integration
- [ ] Security scanning

### Phase 3: Linear Integration (Week 3)

- [ ] Issue state automation
- [ ] Label taxonomy
- [ ] Branch ↔ Issue linking
- [ ] Webhook handlers

### Phase 4: Slack Integration (Week 4)

- [ ] Channel setup
- [ ] Message templates
- [ ] Reaction-based approvals
- [ ] Slash commands

### Phase 5: Release Automation (Week 5)

- [ ] Merge queue configuration
- [ ] Release workflow
- [ ] Changelog generation
- [ ] Registry publishing

---

## 11. References

- RFC-9116: Dev Forge Architecture
- RFC-9120: Prompt Forge v4
- RFC-9121: Lightning QA Engine
- GitHub Branch Protection Documentation
- Linear Webhooks API
- Slack Block Kit Builder

---

## Appendix A: Slack Message Examples

### Factory Build Started
```
🔨 Factory Build Started

Issue: SX9-456: Add NATS router crate
Branch: factory/SX9-456-sx9-nats-router
Pattern: Reactor
Agent: factory-agent-001

View in Linear
```

### Decision Required
```
🔔 Human Approval Required

PR: #123 - Add sx9-nats-router crate
Reason: New crate from factory agent (>500 lines)

QA Summary:
• Grade: A
• Score: 87/100
• Violations: 2 (medium)

React with:
✅ Approve
❌ Request Changes
🔍 Need More Info

Decision required within 24 hours
```

### QA Failed Alert
```
❌ QA Failed

Issue: SX9-456: Add NATS router crate
Grade: C (62/100)

Critical Violations:
• AsyncMutexAbuse in src/router.rs:45
• UnboundedChannel in src/lib.rs:127

Refactor Directives:
• Replace std::sync::Mutex with tokio::sync::Mutex
• Add capacity to channel: channel(1024)

Full Report
```

---

## Appendix B: Linear Webhook Payloads

### Issue State Changed
```json
{
  "type": "Issue",
  "action": "update",
  "data": {
    "id": "issue-uuid",
    "identifier": "SX9-456",
    "title": "Add NATS router crate",
    "state": {
      "id": "state-uuid",
      "name": "Building"
    },
    "assignee": {
      "id": "user-uuid",
      "name": "factory-agent-001"
    },
    "labels": [
      { "name": "forge:factory" },
      { "name": "crate:new" }
    ]
  }
}
```

### Branch Created
```json
{
  "type": "Issue",
  "action": "update",
  "data": {
    "id": "issue-uuid",
    "identifier": "SX9-456",
    "attachments": [
      {
        "type": "github_branch",
        "url": "https://github.com/sx9/sx9/tree/factory/SX9-456-sx9-nats-router"
      }
    ]
  }
}
```

---

*End of RFC-9122*
