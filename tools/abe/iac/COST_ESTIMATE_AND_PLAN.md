# CTAS-7 IP Extraction & Research Paper Generation
## Cost Estimate and Execution Plan

**Date:** December 4, 2025  
**Status:** 📊 **COST ESTIMATE - REVIEW BEFORE RUNNING**

---

## 🔐 API Key Source: ABE Vault Federation

```
✅ Using CTAS7 Vault (Tier 2): google_gemini key loaded
   Path: /Users/cp5337/Developer/ctas-7-shipyard-staging/CTAS7_API_VAULT.json
   Active Keys: 5 (elevenlabs, linear, mapbox, openai, google_gemini)
```

**No manual API key setup required** - keys are loaded from the vault automatically.

---

## 📊 Data Volume (Actual)

| Metric | Count |
|--------|-------|
| **RFC Files** | 50 |
| **RFC Content Size** | 1.03 MB |
| **RFC Tokens** | ~257K |
| **Sample MD Files** | 100 (configurable) |

---

## 💰 ACTUAL Cost Estimate - Gemini API

### Option A: Gemini 1.5 Flash (Fast, Cheapest) ⭐ RECOMMENDED
| Task | Tokens | Cost |
|------|--------|------|
| RFC Processing (50 RFCs) | 257K | **$0.03** |
| Sample MDs (100 files) | 9K | **$0.00** |
| **TOTAL FLASH** | | **~$0.04** |

*Actual estimate from pipeline - much cheaper than initial projection!*

### Option B: Gemini 1.5 Pro (Better Quality)
| Task | Tokens | Cost |
|------|--------|------|
| RFC Processing (50 RFCs) | 257K | **$0.58** |
| Sample MDs (100 files) | 9K | **$0.02** |
| **TOTAL PRO** | | **~$0.60** |

### Full Pipeline Cost (All Features)
| Run Type | Model | Estimated Cost |
|----------|-------|----------------|
| RFCs Only | Flash | **$0.04** |
| Hybrid (RFCs + Refs + Tests) | Flash | **$0.12** |
| Full (+ LaTeX + Diagrams) | Flash | **$0.20** |
| Full | Pro | **$1.80** |

---

## 📋 What You Get

### 1. **IP Extraction Results**
- JSON files with extracted entities, concepts, claims
- Dual trivariate hashes (Murmur3-64) for each document
- IP scoring and categorization

### 2. **Scholarly References**
- 5-10 academic citations per RFC
- BibTeX format for LaTeX
- Relevance explanations

### 3. **Test Harnesses**
- Rust test modules for each RFC
- Property-based tests
- Performance benchmarks

### 4. **LaTeX Research Papers** (5 selected)
- RFC-9001: Trivariate Hashing
- RFC-9026: Hourglass-Bernoulli Architecture
- RFC-9100: PTCC 32 Primitives
- RFC-9016: Dual Trivariate Integration
- RFC-9021: Graph Convergence Theory

### 5. **EA Artifacts**
- Cypher queries for Neo4j knowledge graph
- SurrealQL for SurrealDB
- Mermaid diagrams
- DoDAF-style views (OV-1, SV-1)

---

## ⚠️ Prerequisites

```bash
# 1. API Key - ALREADY CONFIGURED via ABE Vault ✅
#    No manual setup needed!

# 2. Install dependencies (if not installed)
pip install google-generativeai

# 3. Verify
python -c "import google.generativeai; print('OK')"
```

---

## 🚀 Execution Options

### Option 1: Minimal Run (~$0.04) ⭐
```bash
# Just RFCs with Flash model - CHEAPEST
python real_ip_extraction_pipeline.py --rfcs-only --model flash
```

### Option 2: Standard Run (~$0.12)
```bash
# RFCs + Scholarly Refs + Test Harnesses
python real_ip_extraction_pipeline.py --hybrid --model flash
```

### Option 3: Full Run (~$0.20)
```bash
# Everything including LaTeX papers and diagrams
python real_ip_extraction_pipeline.py --all --model flash
```

### Option 4: High Quality (~$1.80)
```bash
# Full run with Pro model for best quality
python real_ip_extraction_pipeline.py --all --model pro
```

---

## 🎯 Recommended Approach

1. **Start with Option 1** (~$0.04) to verify pipeline works
2. **Review outputs** in `04-abe-iac/output/real_extraction/`
3. **Run Option 3** (~$0.20) for full extraction
4. **Run Option 4** (~$1.80) if higher quality needed

---

## ❌ What This Does NOT Do

- Does NOT modify any existing code
- Does NOT commit to git
- Does NOT deploy anything
- Does NOT access external services (except Gemini API)
- Does NOT store data in cloud (local only)
- Does NOT create fake outputs

---

## ✅ Approval Checklist

- [x] API key configured via ABE Vault (CTAS7_API_VAULT.json)
- [ ] Budget approved: $_____ (choose option above)
- [ ] Output directory: `04-abe-iac/output/real_extraction/`
- [ ] Ready to run

---

## 📊 To Run:

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac

# Verify estimate first (no API calls)
python real_ip_extraction_pipeline.py --estimate

# Run minimal extraction (~$0.04)
python real_ip_extraction_pipeline.py --rfcs-only --model flash

# Run full extraction (~$0.20)
python real_ip_extraction_pipeline.py --all --model flash
```

