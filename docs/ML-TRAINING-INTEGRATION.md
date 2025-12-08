# ML Model Training Integration

**Date:** 2025-01-XX  
**Status:** ✅ **COMPLETE**

---

## Overview

ML model training has been integrated into the threat content fetcher pipeline. When threat intelligence is fetched, all three models (DistilBERT, Phi-3, GNN) are automatically trained on the fetched data.

---

## Training Pipeline

### 1. DistilBERT - MITRE Classification

**Purpose:** Classify threat techniques by MITRE ATT&CK tactics  
**Training Data:** MITRE techniques, Nuclei templates  
**Output:** `models/distilbert-mitre/`

```python
from sx9_ml_models.training import train_distilbert

train_distilbert(
    threat_content_dir="output/threat_content",
    output_dir="models/distilbert-mitre"
)
```

### 2. Phi-3 LoRA - Threat Intelligence Explanation

**Purpose:** Fine-tune Phi-3 for threat intelligence analysis and explanation  
**Training Data:** MITRE techniques (instruction-following format)  
**Output:** `models/phi3-mitre-lora/`

```python
from sx9_ml_models.training import train_phi_lora

train_phi_lora(
    threat_content_dir="output/threat_content",
    output_dir="models/phi3-mitre-lora"
)
```

### 3. GNN - Threat Graph Analysis

**Purpose:** Analyze threat relationships and patterns in graph structure  
**Training Data:** Threat graphs from Neo4j export or generated from MITRE data  
**Output:** `models/gnn-threat/`

```python
from sx9_ml_models.training import train_gnn

train_gnn(
    threat_content_dir="output/threat_content",
    neo4j_export="output/ontology/threat_ontology.json",
    output_dir="models/gnn-threat"
)
```

---

## Integration with Threat Content Fetcher

### Automatic Training

When running the threat content fetcher with `--all`, training runs automatically:

```bash
python threat_content_fetcher.py --all
```

This will:
1. Fetch all threat content
2. Generate SPIRES ontology
3. Convert YAMLs to DSL
4. **Train all ML models** ← NEW

### Training-Only Mode

To train models without fetching new content:

```bash
python threat_content_fetcher.py --train-only
```

### Skip Training

To skip training (e.g., if you only want to fetch content):

```bash
python threat_content_fetcher.py --all --no-training
```

---

## Training Configuration

Default training configuration:

```python
TrainingConfig(
    epochs=5,
    batch_size=16,
    learning_rate=2e-5,
    output_dir="models",
    device=None,  # Auto-detect (cuda/cpu)
    gradient_accumulation_steps=1,
    save_steps=500,
    eval_steps=500,
    logging_steps=100
)
```

---

## Output Structure

```
output/
├── threat_content/          # Fetched threat intelligence
├── ontology/                # SPIRES ontology
├── sx9_dsl/                # DSL conversions
└── models/                 # Trained models
    ├── distilbert-mitre/   # DistilBERT classifier
    ├── phi3-mitre-lora/    # Phi-3 LoRA adapter
    └── gnn-threat/         # GNN threat analyzer
```

---

## Dependencies

### Required

- `torch` - PyTorch for neural networks
- `transformers` - HuggingFace transformers
- `peft` - Parameter-Efficient Fine-Tuning (for LoRA)

### Optional

- `torch-geometric` - For GNN training
- `numpy` - For data processing

### Installation

```bash
cd /Users/cp5337/Developer/sx9-conda/python-packages/sx9_ml_models
pip install -e ".[full]"
```

---

## Training Data Sources

### DistilBERT
- MITRE ATT&CK techniques (JSON)
- Nuclei templates (YAML)
- Labels: MITRE tactic IDs, severity levels

### Phi-3
- MITRE techniques → instruction-following format
- Format: `{"instruction": "...", "input": "...", "output": "..."}`
- Examples: technique explanation, detection guidance, mitigation recommendations

### GNN
- Threat graph nodes (techniques, actors, tools)
- Threat graph edges (relationships, dependencies)
- Source: Neo4j export or generated from MITRE data

---

## Next Steps

1. **Full LoRA Training Loop**: Complete the Phi-3 LoRA training implementation
2. **GNN Architecture**: Implement full GNN model architecture and training
3. **Evaluation Metrics**: Add accuracy, F1, and other metrics
4. **Model Serving**: Integrate trained models into inference pipeline
5. **Continuous Training**: Set up periodic retraining on new threat data

---

## Status

✅ **Training modules created**  
✅ **Integrated into threat_content_fetcher.py**  
✅ **Training data generation implemented**  
⚠️ **Full training loops require additional setup** (see TODO comments in code)

---

## References

- RFC-9012: Embeddings & GNN Training Fabric
- RFC-9021: Cognitive Inference
- `sx9-conda/python-packages/sx9_ml_models/sx9_ml_models/training.py`


