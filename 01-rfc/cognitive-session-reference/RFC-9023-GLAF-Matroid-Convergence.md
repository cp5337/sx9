# RFC-9023 — GLAF Matroid Convergence Mathematics

**Version:** 1.0  
**Status:** Draft  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9012, RFC-9021

---

## 1. Overview

This RFC defines the **GLAF Matroid Convergence Mathematics** framework for measuring information independence and convergence quality in intelligence fragment collections.

### 1.1 Purpose

The matroid framework provides mathematical foundations for:

1. **Information Independence** (H2 Score) - Measuring non-redundancy of intelligence fragments
2. **Convergence Quality** (H1 Score) - Measuring agreement/coherence across sources  
3. **Rank-Based Selection** - Optimal fragment subset selection for maximum information value

### 1.2 Mathematical Foundation

A **matroid** M = (E, I) consists of:
- **Ground set E**: Collection of all intelligence fragments
- **Independent sets I**: Subsets of E satisfying independence axioms

The **rank function** r(S) for subset S ⊆ E gives the maximum number of linearly independent vectors.

---

## 2. Core Data Structures

### 2.1 Fragment

```rust
/// Represents a single intelligence fragment (Vector/Embedding)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fragment {
    /// Corresponds to the HASH of the fragment (trivariate hash)
    pub id: u64,
    
    /// Embedding vector (768-dim per RFC-9012, simplified to 3D for demo)
    pub vector: Vector3<f64>,
    
    /// Source confidence score [0.0, 1.0]
    pub confidence: f64,
}
```

### 2.2 LatentMatroid

```rust
/// Implements the Matroid Rank function based on linear independence.
/// The ground set E is the collection of all fragments.
pub struct LatentMatroid {
    pub ground_set: Vec<Fragment>,
}
```

---

## 3. Rank Calculation

### 3.1 Algorithm

The rank of a subset is computed via matrix rank analysis:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    RANK CALCULATION PIPELINE                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  INPUT: Fragment subset indices [i₁, i₂, ..., iₖ]                          │
│                                                                             │
│  STEP 1: Collect Vectors                                                   │
│  ────────────────────────                                                  │
│  vectors = [fragment[i₁].vector, fragment[i₂].vector, ...]                 │
│                                                                             │
│  STEP 2: Build Matrix                                                      │
│  ────────────────────                                                      │
│  M = [v₁ | v₂ | ... | vₖ]   (Dimension × k matrix)                        │
│                                                                             │
│  ┌─────────────────────────────────────┐                                   │
│  │  v₁[0]  v₂[0]  ...  vₖ[0]          │                                   │
│  │  v₁[1]  v₂[1]  ...  vₖ[1]          │  Dimension = 768 (RFC-9012)       │
│  │  ...    ...    ...  ...            │  or 3 (demo mode)                  │
│  │  v₁[d]  v₂[d]  ...  vₖ[d]          │                                   │
│  └─────────────────────────────────────┘                                   │
│                                                                             │
│  STEP 3: Compute Matrix Rank                                               │
│  ───────────────────────────                                               │
│  rank(M) = number of linearly independent columns                          │
│  tolerance = 1e-6 (numerical stability)                                    │
│                                                                             │
│  OUTPUT: rank ∈ [0, min(dimension, k)]                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Implementation

```rust
impl LatentMatroid {
    /// Calculates the rank of a subset of fragments.
    /// Rank r(S) is the maximum number of linearly independent vectors in S.
    /// This directly measures Information Independence (RFC-9021 §3.3).
    /// 
    /// Note: This is computationally intensive and runs in the 
    /// Zone C (Analytical) time window per RFC-9004.
    pub fn calculate_rank(&self, subset_indices: &[usize]) -> usize {
        if subset_indices.is_empty() {
            return 0;
        }

        // 1. Collect the vectors for the subset
        let vectors: Vec<Vector3<f64>> = subset_indices.iter()
            .filter_map(|&i| self.ground_set.get(i).map(|f| f.vector))
            .collect();

        // 2. Build a matrix where each column is a fragment vector
        let num_cols = vectors.len();
        if num_cols == 0 { return 0; }
        
        let matrix_data: Vec<f64> = vectors.into_iter()
            .flat_map(|v| v.iter().copied())
            .collect();
            
        let matrix = DMatrix::from_column_slice(3, num_cols, &matrix_data);

        // 3. Compute the rank of the matrix
        matrix.rank(1e-6)
    }
}
```

---

## 4. Rank Delta (H2 Contribution)

### 4.1 Definition

**Rank Delta** measures the incremental information value when adding a new fragment to an existing set:

```
Δr(S, f) = r(S ∪ {f}) - r(S)
```

- **Δr = 1**: Fragment f adds new independent information
- **Δr = 0**: Fragment f is linearly dependent (redundant)

### 4.2 Implementation

```rust
impl LatentMatroid {
    /// Measures the change in information independence when adding a new fragment.
    /// High Rank Delta = High H2 Contribution.
    pub fn rank_delta(&self, existing_indices: &[usize], new_index: usize) -> usize {
        let old_rank = self.calculate_rank(existing_indices);
        
        let mut new_indices = existing_indices.to_vec();
        if !new_indices.contains(&new_index) {
            new_indices.push(new_index);
        }
        
        let new_rank = self.calculate_rank(&new_indices);
        
        new_rank.saturating_sub(old_rank)
    }
}
```

---

## 5. H1/H2 Score Integration

### 5.1 Score Definitions

| Score | Metric | Formula | Range |
|-------|--------|---------|-------|
| **H1** | Convergence | Agreement across sources | [0, 1] |
| **H2** | Independence | Non-redundancy of information | [0, 1] |
| **Combined** | Quality | α·H1 + (1-α)·H2 | [0, 1] |

### 5.2 H2 from Matroid Rank

```rust
/// Calculate H2 score for a fragment collection
pub fn h2_score(matroid: &LatentMatroid, indices: &[usize]) -> f64 {
    let rank = matroid.calculate_rank(indices);
    let count = indices.len();
    
    if count == 0 {
        return 0.0;
    }
    
    // H2 = rank / count
    // Perfect H2 (1.0) when all fragments are independent
    // Low H2 when fragments are redundant
    rank as f64 / count as f64
}
```

### 5.3 Integration Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    H1/H2 CONVERGENCE SCORING                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Fragment Collection                                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  f₁ (source: MISP, confidence: 0.9)                                  │  │
│  │  f₂ (source: OSINT, confidence: 0.7)                                 │  │
│  │  f₃ (source: Internal, confidence: 0.95)                             │  │
│  │  f₄ (source: ATT&CK, confidence: 1.0)                                │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                         │                                                   │
│           ┌─────────────┴─────────────┐                                    │
│           ▼                           ▼                                    │
│  ┌─────────────────┐         ┌─────────────────┐                          │
│  │ H1: Convergence │         │ H2: Independence│                          │
│  │ ───────────────  │         │ ─────────────── │                          │
│  │                  │         │                 │                          │
│  │ Cosine similarity│         │ Matroid Rank    │                          │
│  │ between vectors  │         │ calculation     │                          │
│  │                  │         │                 │                          │
│  │ H1 = avg(sim)   │         │ H2 = rank/count │                          │
│  │                  │         │                 │                          │
│  └────────┬─────────┘         └────────┬────────┘                          │
│           │                            │                                    │
│           └────────────┬───────────────┘                                   │
│                        ▼                                                    │
│           ┌─────────────────────────┐                                      │
│           │ Combined Score          │                                      │
│           │ ────────────────        │                                      │
│           │                         │                                      │
│           │ Q = α·H1 + (1-α)·H2    │                                      │
│           │ where α = 0.6 (default) │                                      │
│           │                         │                                      │
│           │ Example:                │                                      │
│           │ H1 = 0.85, H2 = 0.75   │                                      │
│           │ Q = 0.6(0.85) + 0.4(0.75)│                                     │
│           │ Q = 0.81               │                                      │
│           │                         │                                      │
│           └─────────────────────────┘                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Greedy Basis Selection

### 6.1 Algorithm

Select optimal subset using greedy matroid basis algorithm:

```rust
/// Greedy selection of maximally independent fragment subset
pub fn greedy_basis_selection(
    matroid: &LatentMatroid, 
    max_size: usize
) -> Vec<usize> {
    let mut selected: Vec<usize> = Vec::new();
    let mut candidates: Vec<usize> = (0..matroid.ground_set.len()).collect();
    
    // Sort by confidence (descending) for tie-breaking
    candidates.sort_by(|&a, &b| {
        matroid.ground_set[b].confidence
            .partial_cmp(&matroid.ground_set[a].confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    for candidate in candidates {
        if selected.len() >= max_size {
            break;
        }
        
        // Only add if it increases rank (adds independent information)
        if matroid.rank_delta(&selected, candidate) > 0 {
            selected.push(candidate);
        }
    }
    
    selected
}
```

### 6.2 Complexity

| Operation | Time Complexity | Bernoulli Zone |
|-----------|-----------------|----------------|
| `calculate_rank` | O(d·k²) | Zone C (Analytical) |
| `rank_delta` | O(d·k²) | Zone C |
| `greedy_basis_selection` | O(n·d·k²) | Zone D (Infrastructure) |

Where:
- d = embedding dimension (768)
- k = subset size
- n = ground set size

---

## 7. Bernoulli Zone Classification

Per RFC-9004, matroid operations are classified:

| Operation | Latency | Zone | Use Case |
|-----------|---------|------|----------|
| Single rank query | 1-10ms | C | Ad-hoc analysis |
| Rank delta | 1-10ms | C | Incremental update |
| Full basis selection | 100ms-1s | D | Batch optimization |
| Streaming rank update | <50μs | B | Real-time ingestion |

---

## 8. Integration Points

### 8.1 With RFC-9012 (GNN Embeddings)

```rust
// Fragment vectors come from GNN embeddings
let fragment = Fragment {
    id: trivariate_hash.to_u64(),
    vector: gnn_embedding.to_vec3(),  // Reduced from 768-dim
    confidence: source_confidence,
};
```

### 8.2 With RFC-9021 (Cognitive Inference)

```rust
// Use matroid rank for context selection
let context_fragments = greedy_basis_selection(&matroid, max_context_size);
let assembled_context = context_fragments
    .iter()
    .map(|&i| matroid.ground_set[i].clone())
    .collect();
```

### 8.3 With RFC-9020 (Interview Schema)

Interviews are scored using H1/H2:

```json
{
  "interview_id": "interview-node-001-001-001",
  "convergence_score": {
    "h1": 0.85,
    "h2": 0.75,
    "combined": 0.81,
    "basis_fragments": [0, 2, 5, 7]
  }
}
```

---

## 9. Crate Structure

### 9.1 Cargo.toml

```toml
[package]
name = "ctas7-glaf-matroid-core"
version = "7.3.1"
edition = "2021"
description = "CTAS-7 GLAF Matroid Core - Convergence Math (H1/H2 Score, Matroids)"

[dependencies]
nalgebra = "0.32"
anyhow = "1.0"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ctas7-foundation-core = { path = "../ctas7-foundation-core" }
```

### 9.2 Module Structure

```
ctas7-glaf-matroid-core/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public API exports
│   ├── fragment.rs      # Fragment struct
│   ├── matroid.rs       # LatentMatroid implementation
│   ├── scoring.rs       # H1/H2 score calculations
│   ├── selection.rs     # Greedy basis selection
│   └── streaming.rs     # Streaming rank updates
└── tests/
    ├── rank_tests.rs
    └── integration_tests.rs
```

---

## 10. Example Usage

```rust
use ctas7_glaf_matroid_core::{Fragment, LatentMatroid, h2_score, greedy_basis_selection};
use nalgebra::Vector3;

fn main() -> Result<()> {
    // Create fragments from intelligence sources
    let fragments = vec![
        Fragment { id: 1, vector: Vector3::new(1.0, 0.0, 0.0), confidence: 0.9 },
        Fragment { id: 2, vector: Vector3::new(0.0, 1.0, 0.0), confidence: 0.8 },
        Fragment { id: 3, vector: Vector3::new(1.0, 1.0, 0.0), confidence: 0.7 },  // Dependent!
        Fragment { id: 4, vector: Vector3::new(0.0, 0.0, 1.0), confidence: 0.95 },
    ];
    
    let matroid = LatentMatroid::new(fragments);
    
    // Calculate rank of full set
    let full_rank = matroid.calculate_rank(&[0, 1, 2, 3]);
    println!("Full rank: {}", full_rank);  // Output: 3 (f₃ is dependent)
    
    // Calculate H2 score
    let h2 = h2_score(&matroid, &[0, 1, 2, 3]);
    println!("H2 score: {}", h2);  // Output: 0.75 (3/4)
    
    // Select optimal basis
    let basis = greedy_basis_selection(&matroid, 3);
    println!("Optimal basis: {:?}", basis);  // Output: [3, 0, 1] (by confidence)
    
    Ok(())
}
```

---

## 11. References

- RFC-9012: GNN Embeddings & Training Fabric
- RFC-9021: Cognitive Inference Engine
- Whitney, H. (1935). "On the Abstract Properties of Linear Dependence"
- Oxley, J. (2011). "Matroid Theory" (2nd ed.)
- nalgebra documentation: https://nalgebra.org/

---

**End of RFC-9023**
