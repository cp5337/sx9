# RFC-9140: FORGE Unified Architecture

**Status:** CANONICAL
**Author:** Charles E. Payne / Claude
**Date:** 2025-12-24
**Supersedes:** RFC-9120 (Prompt Forge v4)
**Integrates:** RFC-9100, RFC-9107, RFC-9130, RFC-9060, RFC-9025

---

## Abstract

RFC-9140 defines the canonical architecture for FORGE, the AI-first cognitive computing factory. This document serves as the semantic and doctrinal foundation for all FORGE implementations.

---

## 1. System Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              FORGE COGNITIVE FACTORY                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  ┌────────────────────────────────────────────────────────────────────────────┐ │
│  │                           MISSION INGRESS                                   │ │
│  │                                                                             │ │
│  │    Linear Issue ──┬── Slack @mention ──┬── NATS Request ──┬── Voice       │ │
│  │         │         │         │          │        │         │      │        │ │
│  │         └─────────┴─────────┴──────────┴────────┴─────────┴──────┘        │ │
│  │                                   │                                         │ │
│  │                          ┌────────▼────────┐                               │ │
│  │                          │  GATEWAY:18120  │                               │ │
│  │                          │  (RFC-9030)     │                               │ │
│  │                          └────────┬────────┘                               │ │
│  └───────────────────────────────────┼────────────────────────────────────────┘ │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                              NATS FABRIC                                   │  │
│  │                           (Core: ~50μs)                                    │  │
│  │                                                                            │  │
│  │   sx9.forge.*  sx9.agent.*  sx9.linear.*  sx9.slack.*  sx9.memory.*      │  │
│  │                                                                            │  │
│  │   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │  │
│  │   │ Mission │  │  Agent  │  │  Issue  │  │ Mention │  │ Memory  │        │  │
│  │   │ Events  │  │ Routing │  │  Sync   │  │ Routing │  │  Events │        │  │
│  │   └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                           AGENT REGISTRY                                   │  │
│  │                                                                            │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐  │  │
│  │  │                    PERSONA AGENTS (Internal)                         │  │  │
│  │  │                                                                      │  │  │
│  │  │   @forge      @axiom      @vector     @sentinel    @guardian        │  │  │
│  │  │   CodeGen     Analysis    Planning    Security     QA               │  │  │
│  │  │                                                                      │  │  │
│  │  └─────────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                            │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐  │  │
│  │  │                    PROVIDER AGENTS (External)                        │  │  │
│  │  │                                                                      │  │  │
│  │  │   @claude     @gpt        @gemini     @grok        @cursor          │  │  │
│  │  │   Anthropic   OpenAI      Google      xAI          IDE              │  │  │
│  │  │   :50055      :50057      :50056      :50051       :50059           │  │  │
│  │  │                                                                      │  │  │
│  │  └─────────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                            │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐  │  │
│  │  │                    SPECIALIST AGENTS (Domain)                        │  │  │
│  │  │                                                                      │  │  │
│  │  │   @natasha    @zoe        @elena      @altair      @cove            │  │  │
│  │  │   RedTeam     Orbital     LATAM       SDA          DevOps           │  │  │
│  │  │   :50052      :50056      :50058      :50054       :50053           │  │  │
│  │  │                                                                      │  │  │
│  │  └─────────────────────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                         COGNITIVE ENGINE (ATLAS)                          │  │
│  │                                                                            │  │
│  │  ┌──────────────────────────────────────────────────────────────────────┐ │  │
│  │  │                     CONCURRENT OODA FIELD                             │ │  │
│  │  │                                                                       │ │  │
│  │  │   STRATEGIC ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▶  │ │  │
│  │  │       │                                                               │ │  │
│  │  │       ├── OPERATIONAL ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▶             │ │  │
│  │  │       │       │                                                       │ │  │
│  │  │       │       ├── TACTICAL ━━━━━━━━━━━━━━━━━━━━━▶                    │ │  │
│  │  │       │       │       │                                               │ │  │
│  │  │       │       │       └── ACTION ━━━━▶ ACTION ━━━━▶ ACTION ━━━━▶    │ │  │
│  │  │       │       │                                                       │ │  │
│  │  │       │       └── TACTICAL ━━━━━━━━━━━━━━━━━━━━━▶                    │ │  │
│  │  │       │                                                               │ │  │
│  │  │       └── OPERATIONAL ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▶             │ │  │
│  │  │                                                                       │ │  │
│  │  │   All loops concurrent. All tempos simultaneous. Field, not chain.   │ │  │
│  │  └──────────────────────────────────────────────────────────────────────┘ │  │
│  │                                      │                                     │  │
│  │  ┌──────────────────────────────────▼───────────────────────────────────┐ │  │
│  │  │                      DETECTION LAYER                                  │ │  │
│  │  │                                                                       │ │  │
│  │  │   Δ-Angle ──────► SDT ──────► Thyristor ──────► Matroid Detection   │ │  │
│  │  │   (drift)        (signal)     (threshold)       (pattern emerging)   │ │  │
│  │  │                                                                       │ │  │
│  │  └──────────────────────────────────────────────────────────────────────┘ │  │
│  │                                      │                                     │  │
│  │  ┌──────────────────────────────────▼───────────────────────────────────┐ │  │
│  │  │                      THALAMIC FILTER                                  │ │  │
│  │  │                                                                       │ │  │
│  │  │   Concurrent sensory streams → Integration → Prioritized awareness   │ │  │
│  │  │                                                                       │ │  │
│  │  └──────────────────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                         FACTORY PIPELINE                                   │  │
│  │                                                                            │  │
│  │   INGEST ────► REASON ────► PRODUCE ────► INTEGRATE ────► LEARN          │  │
│  │      │            │            │              │              │             │  │
│  │   receive      select       generate        apply         record          │  │
│  │   mission      model        artifact        changes       outcome         │  │
│  │      │            │            │              │              │             │  │
│  │   parse        build        validate        verify        update          │  │
│  │   intent       prompt       output          integration   knowledge       │  │
│  │      │            │            │              │              │             │  │
│  │   load         execute      refine          publish       emit            │  │
│  │   context      inference    result          result        telemetry       │  │
│  │                                                                            │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                         SDC LIFECYCLE                                      │  │
│  │                                                                            │  │
│  │   1.Design ─► 2.Model ─► 3.Reason ─► 4.Build ─► 5.Integrate              │  │
│  │       │                                              │                     │  │
│  │       └──────────────────────────────────────────────┘                     │  │
│  │                           │                                                │  │
│  │   6.Validate ─► 7.Operate ─► 8.Learn ─► 9.Govern ─► 10.Retire            │  │
│  │                                                                            │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                              QA GATES                                      │  │
│  │                                                                            │  │
│  │   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐           │  │
│  │   │  STATIC  │    │   ARCH   │    │ PATTERN  │    │AGGREGATE │           │  │
│  │   │   GATE   │───►│   GATE   │───►│   GATE   │───►│  GRADE   │           │  │
│  │   │          │    │          │    │          │    │   A-F    │           │  │
│  │   └──────────┘    └──────────┘    └──────────┘    └──────────┘           │  │
│  │                                                                            │  │
│  │   Weights: Structure(25%) + Complexity(25%) + Pattern(25%) + Arch(25%)   │  │
│  │                                                                            │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                           MEMORY SYSTEM                                    │  │
│  │                          (RFC-9060)                                        │  │
│  │                                                                            │  │
│  │   .forge/                                                                  │  │
│  │   ├── memory/                                                              │  │
│  │   │   ├── long-term/          # Persistent across all sessions            │  │
│  │   │   │   ├── knowledge.json  # Learned patterns, decisions               │  │
│  │   │   │   ├── features.json   # Feature list with pass/fail               │  │
│  │   │   │   └── architecture.md # System understanding                      │  │
│  │   │   │                                                                    │  │
│  │   │   ├── short-term/         # Current session context                   │  │
│  │   │   │   ├── progress.txt    # What's been done this session             │  │
│  │   │   │   ├── focus.json      # Current task, files, state                │  │
│  │   │   │   └── scratch/        # Working notes                             │  │
│  │   │   │                                                                    │  │
│  │   │   └── artifacts/          # Produced outputs                          │  │
│  │   │                                                                        │  │
│  │   ├── linear/                 # Linear issue tracking                     │  │
│  │   └── git/                    # Git backtrace + checkpoints               │  │
│  │                                                                            │  │
│  └───────────────────────────────────┬───────────────────────────────────────┘  │
│                                      │                                           │
│  ┌───────────────────────────────────▼───────────────────────────────────────┐  │
│  │                         MISSION EGRESS                                     │  │
│  │                                                                            │  │
│  │   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐           │  │
│  │   │   Git    │    │  Linear  │    │  Slack   │    │   NATS   │           │  │
│  │   │  Commit  │    │  Update  │    │  Notify  │    │  Publish │           │  │
│  │   └──────────┘    └──────────┘    └──────────┘    └──────────┘           │  │
│  │                                                                            │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                  │
└──────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Primitive Foundation

All operations compile to 32 PTCC primitives (RFC-9100):

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                         32 PTCC PRIMITIVES (U+E400-E41F)                        │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  CRUD (0x00-0x03)          DATA (0x04-0x07)          NETWORK (0x08-0x0B)       │
│  ┌────┬────┬────┬────┐    ┌────┬────┬────┬────┐    ┌────┬────┬────┬────┐      │
│  │ C  │ R  │ U  │ D  │    │ Tx │ Va │ Co │ En │    │ Ro │ Br │ Su │ Pu │      │
│  └────┴────┴────┴────┘    └────┴────┴────┴────┘    └────┴────┴────┴────┘      │
│                                                                                 │
│  ANALYSIS (0x0C-0x0F)      COGNITIVE (0x10-0x13)     CONTROL (0x14-0x17)       │
│  ┌────┬────┬────┬────┐    ┌────┬────┬────┬────┐    ┌────┬────┬────┬────┐      │
│  │ An │ Co │ Sc │ Pr │    │ Ob │ Or │ De │ Ac │    │ Lk │ Un │ Sp │ Te │      │
│  └────┴────┴────┴────┘    └────┴────┴────┴────┘    └────┴────┴────┴────┘      │
│                            (OODA Loop)                                          │
│                                                                                 │
│  STATE (0x18-0x1B)         META (0x1C-0x1F)                                    │
│  ┌────┬────┬────┬────┐    ┌────┬────┬────┬────┐                               │
│  │ Ck │ Rs │ Sy │ Me │    │ Ms │ Lg │ Al │ No │                               │
│  └────┴────┴────┴────┘    └────┴────┴────┴────┘                               │
│                                                                                 │
│  Primitives are atomic. They compose into Tasks. Tasks compose into Missions. │
│                                                                                 │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Task Composition

### 3.1 Core Tasks (39) - Infrastructure Operations

| Domain | Tasks |
|--------|-------|
| SESSION | start, end, checkpoint |
| MEMORY | read/write knowledge, progress, focus |
| CONTEXT | gather, summarize, prune |
| LINEAR | fetch, update, create, close issue |
| GIT | status, branch, stage, commit, checkpoint, push |
| CODE | read, write, edit, delete, generate, refactor |
| QA | run tests, static/arch/pattern gates, generate report |
| NATS | publish, subscribe, request |
| AGENT | dispatch, receive, handoff |

### 3.2 Factory Tasks (15) - Cognitive Workflow

| Phase | Tasks |
|-------|-------|
| INGEST | receive mission, parse intent, load context |
| REASON | select model, build prompt, execute inference |
| PRODUCE | generate artifact, validate output, refine result |
| INTEGRATE | apply changes, verify integration, publish result |
| LEARN | record outcome, update knowledge, emit telemetry |

---

## 4. Node Interview Schema (RFC-9025)

Every task has a node interview - a first-person narrative:

```
I am [Task Name]. I am [role in system].

I [primary action] using [specific methods]. I [secondary action] through
[mechanisms]. I [operational detail].

You have seen me in [example context] where [specific application].

My indicators are [observable patterns]. I try to [optimization], but if
you're [monitoring method], you'll see [what reveals me].

My success means [outcome achieved]. My failure means [outcome blocked].
I feed [downstream tasks] and enable [dependent operations]. Without me,
[consequence of absence].
```

---

## 5. SDC Task Narrative Compilation

Prose compiles to primitives. Primitives are invisible:

> "Reasoning is the continuous process of interpreting modeled reality under uncertainty."

Compiles to: `OBSERVE → ORIENT → DECIDE` without naming them.

| Narrative Phase | Compiles To |
|-----------------|-------------|
| Design | ParseIntent, DesignSolution |
| Model | GatherContext, IdentifyScope |
| Reason | SelectModel, BuildPrompt, ExecuteInference |
| Build | ImplementSolution, GenerateArtifact |
| Integrate | ApplyChanges, VerifyIntegration |
| Validate | RunTests, CheckQuality |
| Operate | PublishResult, EmitTelemetry |
| Learn | RecordOutcome, UpdateKnowledge |
| Govern | CreateCommit, UpdateIssue |
| Retire | (deprecation primitives) |

---

## 6. OODA Field Dynamics

OODA is concurrent and nested, not sequential:

```
All senses active simultaneously:
SEE ─┬─ HEAR ─┬─ SMELL ─┬─ TASTE ─┬─ FEEL ─┬─ ...
     │        │         │         │        │
     └────────┴─────────┴─────────┴────────┴─────► THALAMIC FILTER
                                                          │
                                                          ▼
                                                   SITUATIONAL
                                                    AWARENESS
```

Detection layer approximates biological perception:
- **Δ-Angle:** Context drift measurement (0-180°)
- **SDT:** Signal detection theory - signal vs noise
- **Thyristor:** Threshold trigger - fire when evidence sufficient
- **Matroid:** Latent pattern detection - emerging structures

---

## 7. Port Architecture

| Port | Service | RFC |
|------|---------|-----|
| 4222 | NATS Event Bus | RFC-9400 |
| 18109 | Lightning QA | RFC-9107 |
| 18120 | Linear Gateway | RFC-9030 |
| 18260 | Voice Bridge | RFC-9107 |
| 50051-50058 | Agent Mesh (gRPC) | RFC-9107 |

---

## 8. References

- **RFC-9025:** Node Interview Schema
- **RFC-9030:** Unified Linear Agent Infrastructure
- **RFC-9060:** Agent Memory Architecture
- **RFC-9100:** Dual-Trivariate PTCC Integration
- **RFC-9107:** Unified Agent Infrastructure
- **RFC-9130:** Unified Forge Pipeline

---

## 9. Invariants

1. Never use Blake3 - only Murmur3 trivariate
2. Always use foundation crates for common dependencies
3. NATS subjects must mirror Redux action types
4. Primitives are atomic - no dual operations
5. OODA is a field, not a chain
6. Prose compiles to primitives invisibly

---

**Document Status:** CANONICAL
**Creation Date:** 2025-12-24
**Source:** Session synthesis
