# RFC-9120: Prompt Forge v4 — Plain Language Crate Manufacturing

**Status:** Draft  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-20  
**Dependencies:** RFC-9001, RFC-9005, RFC-9025, RFC-9101, RFC-9112, RFC-9116

---

## Abstract

Prompt Forge v4 eliminates manual pattern selection, constraint specification, and boilerplate generation. Users speak plain language intent; the system derives patterns, populates interviews, generates birth certificates, and produces deterministic canonical prompts that factory agents execute without supervision.

**Core Insight:** The Prompt is the Point. If the canonical prompt is perfect, the factory runs autonomously.

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PROMPT FORGE v4 PIPELINE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │  PLAIN       │    │   THALMIC    │    │   SLEDIS     │                  │
│  │  LANGUAGE    │───▶│   FILTER     │───▶│   PATTERN    │                  │
│  │  INPUT       │    │   (Clarity)  │    │   RESOLVER   │                  │
│  └──────────────┘    └──────────────┘    └──────────────┘                  │
│         │                   │                   │                           │
│         │            [Clarity Score]     [Pattern + Constraints]            │
│         │                   │                   │                           │
│         ▼                   ▼                   ▼                           │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │                  RFC-9025 INTERVIEW POPULATOR               │           │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │           │
│  │  │Identity │ │ Voice   │ │Capabil. │ │Contract │           │           │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘           │           │
│  └─────────────────────────────────────────────────────────────┘           │
│                                    │                                        │
│                            [Birth Certificate]                              │
│                                    │                                        │
│                                    ▼                                        │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │                  CANONICAL PROMPT ASSEMBLY                   │           │
│  │                                                              │           │
│  │  • Constraints from pattern                                  │           │
│  │  • LOC limits from interview                                 │           │
│  │  • PTCC primitives resolved                                  │           │
│  │  • Trivariate hash computed                                  │           │
│  └─────────────────────────────────────────────────────────────┘           │
│                                    │                                        │
│                            [Gold Master Prompt]                             │
│                                    │                                        │
│                                    ▼                                        │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │                     FACTORY AGENT                            │           │
│  │                  (Claude Sonnet 4)                           │           │
│  │                                                              │           │
│  │  Executes autonomously against canonical prompt              │           │
│  └─────────────────────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Thalmic Filter

The Thalmic Filter ensures prompts are **unambiguous and deterministically interpretable**. Think of it like Yoast SEO's smiley face - clarity scoring, not security redaction.

**Clarity Threshold:** ≥ 0.7

**What It Checks:**
- Ambiguous pronouns ("it", "this", "that" without antecedent)
- Colloquial phrases that could mean multiple things
- Missing context for domain-specific terms
- Vague quantifiers ("some", "many", "a few")

---

## 3. Sledis Pattern Resolver

Cached lookup, not LLM inference:

```
Key Pattern: pattern:resolve:{feature_vector_hash}
```

**Example:**
```
Input: "rust http service with postgres"
Features: [rust, http, service, postgres, async]
Lookup: pattern:resolve:a7f3...
Result: {
  pattern_id: "async-http-postgres",
  constraints: { max_loc: 300, must_have: ["axum", "sqlx"] },
  template_ref: "templates/rust-http-pg.toml"
}
```

---

## 4. Interview Auto-Population (RFC-9025)

Once pattern resolves, interview fields populate automatically:

| Field | Source |
|-------|--------|
| identity.name | Derived from intent |
| identity.domain | Pattern category |
| capabilities.primitives | Pattern constraints |
| contract.max_loc | Pattern default |
| contract.dependencies | Pattern must_have |

---

## 5. Birth Certificate Generation

Two outputs:
1. `crate_interview.json` - Full interview data
2. `smartcrate.toml` - Runtime configuration (RFC-9101)

---

## 6. Canonical Prompt Assembly

The Gold Master Prompt includes:
- All constraints from pattern
- LOC limits
- PTCC primitives (resolved to hex codes)
- Trivariate hash for verification
- N-V-N-N grammar enforcement

---

## 7. Factory Agent Execution

Claude Sonnet 4 executes against the canonical prompt:
- No conversation history needed
- Deterministic output from deterministic input
- Continuous EA (structure + contract checks on every commit)

---

## 8. NATS Health Contract

Every manufactured crate registers:

```
Subject: sx9.crate.{crate_id}.health
Payload: {
  "status": "alive",
  "last_check": "2025-12-20T...",
  "contract_hash": "abc123...",
  "qa_grade": "A"
}
```

---

## 9. Storage Architecture

Four-tier:
1. **Sledis L1** - Pattern cache, hot paths (<1ms)
2. **Sled L2** - Local persistent (1-10ms)  
3. **SurrealDB L3** - Graph relationships (10-100ms) [DEPRECATED - use SlotGraph]
4. **Supabase L4** - ACID transactions (100ms+)

---

## 10. LLM Adapters

Different LLMs require different chunking and prompt structures:

| LLM | Context | Optimal Chunk | Notes |
|-----|---------|---------------|-------|
| Claude Sonnet 4 | 200K | 30K | Preferred factory agent |
| GPT-4 | 128K | 20K | Backup |
| Gemini | 1M | 50K | Large context tasks |
| Phi-3 | 4K | 2K | Edge deployment |

---

**Document Status:** RECOVERED  
**Recovery Date:** 2025-12-24
