# RFC-9302: Nonagon Analytic Node (NAN)

**Version:** 0.1.0  
**Status:** DRAFT  
**Date:** 2025-12-06  
**Author:** CTAS Architecture Team  
**Depends-On:** RFC-9001 (Trivariate Hashing), RFC-9300 (HD4), RFC-9301 (TCR)  

---

## Abstract

This RFC defines the **Nonagon Analytic Node (NAN)** — a 9-aspect graph node structure for multi-dimensional threat analysis, intelligence fusion, and elastic analytical processes.

The nonagon (9-gon) is the **geometric expression of Synaptix9 (SX9)** — the "9" is the architectural constant that unifies the entire cognitive framework:

```
SYNAPTIX9 (SX9)
     │
     ▼
  NONAGON
     │
     ├── 9 vertices
     ├── 3 trivariates × 3 axes = 9 dimensions
     ├── 9 intelligence disciplines
     ├── 9 analytical lenses
     └── 9 edges (circular connectivity)
```

The nonagon provides a natural structure for:
- **3 × 3 Trivariate Alignment** — Three trivariates with three axes each
- **Fusion** — Combining multiple analytical perspectives into unified assessment
- **Elastic** — Scaling analysis depth based on available data/time
- **Graph Nodes** — Novel 9-aspect vertices for cognitive graphs

---

## 0. The SX9 Constant

### 0.1 Nine as Architectural Foundation

The number **9** is not arbitrary — it is the foundational constant of Synaptix9:

| Domain | 9-Expression | Components |
|--------|--------------|------------|
| **Geometry** | Nonagon | 9 vertices, 9 edges, 140° interior angle |
| **Trivariates** | 3 × 3 | 3 trivariates (α, β, γ) × 3 axes (X, Y, Z) |
| **RFC-9000 Core** | 9 Active Primitives | Actor, Object, Event, Concept, Attribute, Function, Module, Header, Footer |
| **Intelligence** | 9 INTs | CYBINT, SIGINT, HUMINT, IMINT, MASINT, OSINT, GEOINT, FININT, TECHINT |
| **Analysis** | 9 Lenses | What, Who, Where, When, Why, How, Impact, Confidence, Action |
| **Numerology** | 3 × 3 | Trinity of trinities, completeness |

### 0.2 Mathematical Properties of 9

```
9 = 3²                          (Square of trinity)
9 = 1 + 2 + 3 + ... + n where sum digits = 9
Interior angle = 140° = 180° - (360°/9)
Central angle = 40° = 360°/9
Diagonals = 27 = 3³             (Cube of trinity)
```

### 0.3 SX9 System Mapping

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SYNAPTIX9 ARCHITECTURE                              │
│                                                                             │
│                              NONAGON                                        │
│                            (9-gon NAN)                                      │
│                                 │                                           │
│         ┌───────────────────────┼───────────────────────┐                  │
│         │                       │                       │                  │
│         ▼                       ▼                       ▼                  │
│   ┌───────────┐           ┌───────────┐           ┌───────────┐           │
│   │TRIVARIATE α│           │TRIVARIATE β│           │TRIVARIATE γ│           │
│   │ (Semantic) │           │(Operational)│           │ (Temporal) │           │
│   ├───────────┤           ├───────────┤           ├───────────┤           │
│   │ X: Context │           │ X: Phase   │           │ X: History │           │
│   │ Y: Meaning │           │ Y: Intensity│           │ Y: Current │           │
│   │ Z: Intent  │           │ Z: Duration│           │ Z: Predict │           │
│   └───────────┘           └───────────┘           └───────────┘           │
│         │                       │                       │                  │
│         └───────────────────────┼───────────────────────┘                  │
│                                 │                                           │
│                                 ▼                                           │
│                          ┌───────────┐                                     │
│                          │  CENTER   │                                     │
│                          │  (Fusion) │                                     │
│                          └───────────┘                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 1. Mathematical Foundation

### 1.1 Nonagon Geometry

```
                        A₀ (0°)
                       /    \
                     /        \
                   /            \
               A₈ /              \ A₁
              (320°)            (40°)
                |                  |
                |                  |
            A₇  |                  |  A₂
           (280°)                (80°)
                |                  |
                |                  |
               A₆ \              / A₃
              (240°)            (120°)
                   \            /
                     \        /
                       \    /
                        A₅ (200°)
                         |
                        A₄ (160°)

Interior Angle: 140°
Central Angle: 40° (360° / 9)
Vertices: 9
Edges: 9 (adjacent connections)
Diagonals: 27 (non-adjacent connections)
```

### 1.2 Vertex Positions (Unit Circle)

```
Aₖ = (cos(2πk/9), sin(2πk/9))  for k ∈ {0, 1, ..., 8}

A₀ = (1.000000, 0.000000)       // 0°
A₁ = (0.766044, 0.642788)       // 40°
A₂ = (0.173648, 0.984808)       // 80°
A₃ = (-0.500000, 0.866025)      // 120°
A₄ = (-0.939693, 0.342020)      // 160°
A₅ = (-0.939693, -0.342020)     // 200°
A₆ = (-0.500000, -0.866025)     // 240°
A₇ = (0.173648, -0.984808)      // 280°
A₈ = (0.766044, -0.642788)      // 320°
```

---

## 2. 9-Aspect Framework

### 2.1 Three Trivariate Model (3×3=9)

The nonagon naturally decomposes into **three trivariates** occupying three vertices each:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      TRIVARIATE DECOMPOSITION                               │
│                                                                             │
│   TRIVARIATE α (SEMANTIC)        TRIVARIATE β (OPERATIONAL)                │
│   Vertices: A₀, A₁, A₂           Vertices: A₃, A₄, A₅                      │
│   ─────────────────────          ─────────────────────────                  │
│   A₀: X_α (Context)              A₃: X_β (Phase)                           │
│   A₁: Y_α (Meaning)              A₄: Y_β (Intensity)                       │
│   A₂: Z_α (Intent)               A₅: Z_β (Duration)                        │
│                                                                             │
│   TRIVARIATE γ (TEMPORAL)                                                  │
│   Vertices: A₆, A₇, A₈                                                     │
│   ─────────────────────                                                     │
│   A₆: X_γ (Historical)                                                     │
│   A₇: Y_γ (Current)                                                        │
│   A₈: Z_γ (Predictive)                                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Alternative: 9 Intelligence Disciplines

For intelligence fusion applications:

| Vertex | Aspect | Description |
|--------|--------|-------------|
| A₀ | **CYBINT** | Cyber Intelligence (network, malware, intrusion) |
| A₁ | **SIGINT** | Signals Intelligence (communications, ELINT) |
| A₂ | **HUMINT** | Human Intelligence (agents, interviews, defectors) |
| A₃ | **IMINT** | Imagery Intelligence (satellite, aerial, ground) |
| A₄ | **MASINT** | Measurement & Signature Intelligence |
| A₅ | **OSINT** | Open Source Intelligence |
| A₆ | **GEOINT** | Geospatial Intelligence |
| A₇ | **FININT** | Financial Intelligence |
| A₈ | **TECHINT** | Technical Intelligence (weapons, systems) |

### 2.3 Alternative: 9 Analytical Lenses

For cognitive analysis:

| Vertex | Lens | Question |
|--------|------|----------|
| A₀ | **WHAT** | What is happening? (Observation) |
| A₁ | **WHO** | Who is involved? (Attribution) |
| A₂ | **WHERE** | Where is it occurring? (Geolocation) |
| A₃ | **WHEN** | When did/will it occur? (Temporal) |
| A₄ | **WHY** | Why is it happening? (Motivation) |
| A₅ | **HOW** | How is it being done? (Methodology) |
| A₆ | **IMPACT** | What are the effects? (Consequence) |
| A₇ | **CONFIDENCE** | How certain are we? (Epistemic) |
| A₈ | **ACTION** | What should we do? (Response) |

---

## 3. Nonagon Node Structure

### 3.1 Core Definition

```rust
/// Nonagon Analytic Node (NAN)
/// RFC-9302 §3
#[derive(Debug, Clone)]
pub struct NonagonNode {
    /// Unique identifier
    pub id: Uuid,
    
    /// SCH hash for semantic addressing
    pub sch: String,
    
    /// 9 vertex values (all 6-decimal precision)
    pub vertices: [f64; 9],
    
    /// Vertex labels (configurable aspect framework)
    pub aspect_labels: [String; 9],
    
    /// Edge weights (9 edges between adjacent vertices)
    pub edges: [f64; 9],
    
    /// Diagonal weights (27 non-adjacent connections)
    pub diagonals: [f64; 27],
    
    /// Center value (fused/converged assessment)
    pub center: f64,
    
    /// Delta angle for position in larger graph
    pub delta_angle: DeltaAngle,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Confidence (overall node confidence)
    pub confidence: f64,
}

impl NonagonNode {
    pub const NUM_VERTICES: usize = 9;
    pub const NUM_EDGES: usize = 9;
    pub const NUM_DIAGONALS: usize = 27;
    pub const INTERIOR_ANGLE: f64 = 140.0;
    pub const CENTRAL_ANGLE: f64 = 40.0;
    
    /// Create new node with default aspect labels
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            sch: String::new(),
            vertices: [0.0; 9],
            aspect_labels: Self::default_aspect_labels(),
            edges: [1.0; 9],  // Default: all edges equal weight
            diagonals: [0.0; 27],  // Default: no diagonal connections
            center: 0.0,
            delta_angle: DeltaAngle::zero(),
            timestamp: 0,
            confidence: 0.0,
        }
    }
    
    /// Default labels: 3×3 Trivariate model
    fn default_aspect_labels() -> [String; 9] {
        [
            "α.X (Context)".into(),
            "α.Y (Meaning)".into(),
            "α.Z (Intent)".into(),
            "β.X (Phase)".into(),
            "β.Y (Intensity)".into(),
            "β.Z (Duration)".into(),
            "γ.X (Historical)".into(),
            "γ.Y (Current)".into(),
            "γ.Z (Predictive)".into(),
        ]
    }
    
    /// Intelligence discipline labels
    pub fn intel_aspect_labels() -> [String; 9] {
        [
            "CYBINT".into(),
            "SIGINT".into(),
            "HUMINT".into(),
            "IMINT".into(),
            "MASINT".into(),
            "OSINT".into(),
            "GEOINT".into(),
            "FININT".into(),
            "TECHINT".into(),
        ]
    }
    
    /// Analytical lens labels
    pub fn lens_aspect_labels() -> [String; 9] {
        [
            "WHAT".into(),
            "WHO".into(),
            "WHERE".into(),
            "WHEN".into(),
            "WHY".into(),
            "HOW".into(),
            "IMPACT".into(),
            "CONFIDENCE".into(),
            "ACTION".into(),
        ]
    }
}
```

### 3.2 Vertex Accessor Methods

```rust
impl NonagonNode {
    /// Get vertex by index (0-8)
    pub fn vertex(&self, idx: usize) -> f64 {
        self.vertices[idx.min(8)]
    }
    
    /// Set vertex by index
    pub fn set_vertex(&mut self, idx: usize, value: f64) {
        self.vertices[idx.min(8)] = value;
    }
    
    /// Get trivariate α (vertices 0, 1, 2)
    pub fn trivariate_alpha(&self) -> DeltaAngle {
        DeltaAngle::new(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
        )
    }
    
    /// Get trivariate β (vertices 3, 4, 5)
    pub fn trivariate_beta(&self) -> DeltaAngle {
        DeltaAngle::new(
            self.vertices[3],
            self.vertices[4],
            self.vertices[5],
        )
    }
    
    /// Get trivariate γ (vertices 6, 7, 8)
    pub fn trivariate_gamma(&self) -> DeltaAngle {
        DeltaAngle::new(
            self.vertices[6],
            self.vertices[7],
            self.vertices[8],
        )
    }
    
    /// Set from three trivariates
    pub fn set_from_trivariates(
        &mut self,
        alpha: &DeltaAngle,
        beta: &DeltaAngle,
        gamma: &DeltaAngle,
    ) {
        self.vertices[0] = alpha.x;
        self.vertices[1] = alpha.y;
        self.vertices[2] = alpha.z;
        self.vertices[3] = beta.x;
        self.vertices[4] = beta.y;
        self.vertices[5] = beta.z;
        self.vertices[6] = gamma.x;
        self.vertices[7] = gamma.y;
        self.vertices[8] = gamma.z;
    }
    
    /// Get vertex position on unit circle (for visualization)
    pub fn vertex_position(&self, idx: usize) -> (f64, f64) {
        let angle = 2.0 * std::f64::consts::PI * (idx as f64) / 9.0;
        (angle.cos(), angle.sin())
    }
}
```

---

## 4. Fusion Operations

### 4.1 Center Calculation (Convergence)

The center value represents the **fused assessment** from all 9 aspects:

```rust
impl NonagonNode {
    /// Calculate center as weighted average of vertices
    pub fn calculate_center_weighted(&mut self) {
        let weights = self.calculate_vertex_weights();
        let weighted_sum: f64 = self.vertices.iter()
            .zip(weights.iter())
            .map(|(v, w)| v * w)
            .sum();
        let weight_sum: f64 = weights.iter().sum();
        
        self.center = if weight_sum > 0.0 {
            weighted_sum / weight_sum
        } else {
            0.0
        };
    }
    
    /// Calculate center as geometric mean (multiplicative fusion)
    pub fn calculate_center_geometric(&mut self) {
        let product: f64 = self.vertices.iter()
            .filter(|&&v| v > 0.0)
            .product();
        let count = self.vertices.iter().filter(|&&v| v > 0.0).count();
        
        self.center = if count > 0 {
            product.powf(1.0 / count as f64)
        } else {
            0.0
        };
    }
    
    /// Calculate center using harmonic mean (conservative fusion)
    pub fn calculate_center_harmonic(&mut self) {
        let reciprocal_sum: f64 = self.vertices.iter()
            .filter(|&&v| v > 0.0)
            .map(|v| 1.0 / v)
            .sum();
        let count = self.vertices.iter().filter(|&&v| v > 0.0).count();
        
        self.center = if reciprocal_sum > 0.0 && count > 0 {
            count as f64 / reciprocal_sum
        } else {
            0.0
        };
    }
    
    /// Vertex weights based on edge connectivity
    fn calculate_vertex_weights(&self) -> [f64; 9] {
        let mut weights = [0.0; 9];
        for i in 0..9 {
            // Weight = average of adjacent edges
            let prev = if i == 0 { 8 } else { i - 1 };
            weights[i] = (self.edges[prev] + self.edges[i]) / 2.0;
        }
        weights
    }
}
```

### 4.2 Multi-Node Fusion

Combine multiple nonagon nodes into a single fused assessment:

```rust
/// Fusion modes for combining multiple nonagon nodes
#[derive(Debug, Clone, Copy)]
pub enum FusionMode {
    /// Average all corresponding vertices
    Average,
    /// Take maximum value at each vertex
    Maximum,
    /// Take minimum value at each vertex (conservative)
    Minimum,
    /// Weighted average based on node confidence
    ConfidenceWeighted,
    /// Bayesian fusion (treat as probabilities)
    Bayesian,
    /// Dempster-Shafer evidence combination
    DempsterShafer,
}

impl NonagonNode {
    /// Fuse multiple nodes into one
    pub fn fuse(nodes: &[NonagonNode], mode: FusionMode) -> NonagonNode {
        if nodes.is_empty() {
            return NonagonNode::new(Uuid::new_v4());
        }
        
        let mut result = NonagonNode::new(Uuid::new_v4());
        
        match mode {
            FusionMode::Average => {
                for i in 0..9 {
                    result.vertices[i] = nodes.iter()
                        .map(|n| n.vertices[i])
                        .sum::<f64>() / nodes.len() as f64;
                }
            }
            FusionMode::Maximum => {
                for i in 0..9 {
                    result.vertices[i] = nodes.iter()
                        .map(|n| n.vertices[i])
                        .fold(f64::NEG_INFINITY, f64::max);
                }
            }
            FusionMode::Minimum => {
                for i in 0..9 {
                    result.vertices[i] = nodes.iter()
                        .map(|n| n.vertices[i])
                        .fold(f64::INFINITY, f64::min);
                }
            }
            FusionMode::ConfidenceWeighted => {
                let total_confidence: f64 = nodes.iter()
                    .map(|n| n.confidence)
                    .sum();
                
                for i in 0..9 {
                    result.vertices[i] = if total_confidence > 0.0 {
                        nodes.iter()
                            .map(|n| n.vertices[i] * n.confidence)
                            .sum::<f64>() / total_confidence
                    } else {
                        0.0
                    };
                }
            }
            FusionMode::Bayesian => {
                // Treat vertices as probabilities, combine with Bayes rule
                for i in 0..9 {
                    let mut combined = 0.5; // Prior
                    for n in nodes {
                        let p = n.vertices[i].clamp(0.001, 0.999);
                        // Bayesian update
                        combined = (combined * p) / 
                            ((combined * p) + ((1.0 - combined) * (1.0 - p)));
                    }
                    result.vertices[i] = combined;
                }
            }
            FusionMode::DempsterShafer => {
                // Dempster-Shafer combination rule
                for i in 0..9 {
                    let mut belief = nodes[0].vertices[i];
                    for n in &nodes[1..] {
                        let m1 = belief;
                        let m2 = n.vertices[i];
                        // Dempster's rule of combination
                        let k = m1 * (1.0 - m2) + (1.0 - m1) * m2; // Conflict
                        if k < 1.0 {
                            belief = (m1 * m2) / (1.0 - k);
                        }
                    }
                    result.vertices[i] = belief;
                }
            }
        }
        
        result.calculate_center_weighted();
        result.confidence = nodes.iter()
            .map(|n| n.confidence)
            .sum::<f64>() / nodes.len() as f64;
        
        result
    }
}
```

---

## 5. Elastic Operations

### 5.1 Elastic Scaling

Vertices can "stretch" or "contract" based on data availability:

```rust
/// Elastic scaling mode
#[derive(Debug, Clone, Copy)]
pub enum ElasticMode {
    /// Scale based on data availability at each vertex
    DataDriven,
    /// Scale based on time pressure (less time = fewer vertices)
    TimePressure,
    /// Scale based on confidence requirements
    ConfidenceTarget,
    /// Manual selection of active vertices
    Manual,
}

impl NonagonNode {
    /// Active vertices (those with sufficient data)
    pub fn active_vertices(&self, threshold: f64) -> Vec<usize> {
        self.vertices.iter()
            .enumerate()
            .filter(|(_, &v)| v > threshold)
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Elastic contraction: reduce to N most significant vertices
    pub fn contract(&self, n: usize) -> NonagonNode {
        let mut result = self.clone();
        
        // Find N highest vertices
        let mut indexed: Vec<(usize, f64)> = self.vertices.iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Zero out vertices not in top N
        let top_n: Vec<usize> = indexed.iter()
            .take(n)
            .map(|(i, _)| *i)
            .collect();
        
        for i in 0..9 {
            if !top_n.contains(&i) {
                result.vertices[i] = 0.0;
            }
        }
        
        result.calculate_center_weighted();
        result
    }
    
    /// Elastic expansion: interpolate missing vertices from neighbors
    pub fn expand(&self) -> NonagonNode {
        let mut result = self.clone();
        
        for i in 0..9 {
            if self.vertices[i] == 0.0 {
                // Interpolate from adjacent vertices
                let prev = if i == 0 { 8 } else { i - 1 };
                let next = if i == 8 { 0 } else { i + 1 };
                
                let prev_val = self.vertices[prev];
                let next_val = self.vertices[next];
                
                if prev_val > 0.0 && next_val > 0.0 {
                    result.vertices[i] = (prev_val + next_val) / 2.0;
                } else if prev_val > 0.0 {
                    result.vertices[i] = prev_val * 0.75;
                } else if next_val > 0.0 {
                    result.vertices[i] = next_val * 0.75;
                }
            }
        }
        
        result.calculate_center_weighted();
        result
    }
    
    /// Calculate "coverage" - what fraction of vertices are active
    pub fn coverage(&self, threshold: f64) -> f64 {
        let active = self.vertices.iter()
            .filter(|&&v| v > threshold)
            .count();
        active as f64 / 9.0
    }
    
    /// Calculate "balance" - how evenly distributed are vertex values
    pub fn balance(&self) -> f64 {
        let mean: f64 = self.vertices.iter().sum::<f64>() / 9.0;
        if mean == 0.0 {
            return 0.0;
        }
        
        let variance: f64 = self.vertices.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / 9.0;
        
        let std_dev = variance.sqrt();
        let cv = std_dev / mean; // Coefficient of variation
        
        // Balance = 1 - normalized CV (1.0 = perfectly balanced)
        (1.0 - cv.min(1.0)).max(0.0)
    }
}
```

### 5.2 Trivariate Elastic Grouping

Since vertices group into three trivariates, elastic operations can work at trivariate level:

```rust
impl NonagonNode {
    /// Check if trivariate α has sufficient data
    pub fn alpha_active(&self, threshold: f64) -> bool {
        self.vertices[0] > threshold || 
        self.vertices[1] > threshold || 
        self.vertices[2] > threshold
    }
    
    /// Check if trivariate β has sufficient data
    pub fn beta_active(&self, threshold: f64) -> bool {
        self.vertices[3] > threshold || 
        self.vertices[4] > threshold || 
        self.vertices[5] > threshold
    }
    
    /// Check if trivariate γ has sufficient data
    pub fn gamma_active(&self, threshold: f64) -> bool {
        self.vertices[6] > threshold || 
        self.vertices[7] > threshold || 
        self.vertices[8] > threshold
    }
    
    /// Number of active trivariates (0-3)
    pub fn active_trivariate_count(&self, threshold: f64) -> usize {
        let mut count = 0;
        if self.alpha_active(threshold) { count += 1; }
        if self.beta_active(threshold) { count += 1; }
        if self.gamma_active(threshold) { count += 1; }
        count
    }
    
    /// Contract to single most complete trivariate
    pub fn contract_to_trivariate(&self) -> (DeltaAngle, char) {
        let alpha_sum: f64 = self.vertices[0..3].iter().sum();
        let beta_sum: f64 = self.vertices[3..6].iter().sum();
        let gamma_sum: f64 = self.vertices[6..9].iter().sum();
        
        if alpha_sum >= beta_sum && alpha_sum >= gamma_sum {
            (self.trivariate_alpha(), 'α')
        } else if beta_sum >= gamma_sum {
            (self.trivariate_beta(), 'β')
        } else {
            (self.trivariate_gamma(), 'γ')
        }
    }
}
```

---

## 6. Graph Connectivity

### 6.1 Edge and Diagonal Operations

```rust
impl NonagonNode {
    /// Get edge weight between adjacent vertices
    pub fn edge_weight(&self, from: usize, to: usize) -> Option<f64> {
        // Edges only connect adjacent vertices
        let from = from % 9;
        let to = to % 9;
        
        if (from + 1) % 9 == to {
            Some(self.edges[from])
        } else if (to + 1) % 9 == from {
            Some(self.edges[to])
        } else {
            None // Not adjacent
        }
    }
    
    /// Get diagonal weight between non-adjacent vertices
    pub fn diagonal_weight(&self, from: usize, to: usize) -> Option<f64> {
        let from = from % 9;
        let to = to % 9;
        
        // Adjacent vertices don't have diagonals
        if (from + 1) % 9 == to || (to + 1) % 9 == from || from == to {
            return None;
        }
        
        // Calculate diagonal index
        // 27 diagonals total, indexed by (from, to) pair
        let idx = Self::diagonal_index(from, to)?;
        Some(self.diagonals[idx])
    }
    
    /// Calculate diagonal index from vertex pair
    fn diagonal_index(from: usize, to: usize) -> Option<usize> {
        let (a, b) = if from < to { (from, to) } else { (to, from) };
        
        // Skip adjacent pairs
        if (a + 1) % 9 == b || (b + 1) % 9 == a {
            return None;
        }
        
        // Diagonal index calculation
        // Each vertex has 6 diagonals (to 6 non-adjacent vertices)
        // But we only count each diagonal once (a < b)
        let mut idx = 0;
        for i in 0..a {
            for j in (i + 2)..9 {
                if j != (i + 8) % 9 { // Not wrapping adjacent
                    if i == a && j == b {
                        return Some(idx);
                    }
                    idx += 1;
                }
            }
        }
        for j in (a + 2)..9 {
            if j != (a + 8) % 9 {
                if j == b {
                    return Some(idx);
                }
                idx += 1;
            }
        }
        
        Some(idx)
    }
    
    /// Strengthen connection between two aspects
    pub fn strengthen_connection(&mut self, from: usize, to: usize, amount: f64) {
        if let Some(edge_idx) = self.edge_index(from, to) {
            self.edges[edge_idx] = (self.edges[edge_idx] + amount).min(1.0);
        } else if let Some(diag_idx) = Self::diagonal_index(from, to) {
            self.diagonals[diag_idx] = (self.diagonals[diag_idx] + amount).min(1.0);
        }
    }
    
    fn edge_index(&self, from: usize, to: usize) -> Option<usize> {
        let from = from % 9;
        let to = to % 9;
        
        if (from + 1) % 9 == to {
            Some(from)
        } else if (to + 1) % 9 == from {
            Some(to)
        } else {
            None
        }
    }
}
```

### 6.2 Inter-Node Connections

Nonagon nodes connect to form a graph:

```rust
/// Connection between two nonagon nodes
#[derive(Debug, Clone)]
pub struct NonagonEdge {
    /// Source node ID
    pub from_node: Uuid,
    /// Target node ID  
    pub to_node: Uuid,
    /// Which vertex on source connects
    pub from_vertex: usize,
    /// Which vertex on target connects
    pub to_vertex: usize,
    /// Connection strength
    pub weight: f64,
    /// Connection type
    pub edge_type: NonagonEdgeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonagonEdgeType {
    /// Same aspect across nodes (e.g., CYBINT → CYBINT)
    SameAspect,
    /// Different aspects (cross-aspect connection)
    CrossAspect,
    /// Trivariate to trivariate
    TrivariateBridge,
    /// Center to center (fused assessment connection)
    CenterLink,
    /// Temporal sequence (time ordering)
    Temporal,
}

/// Graph of nonagon nodes
#[derive(Debug, Clone)]
pub struct NonagonGraph {
    /// Nodes indexed by UUID
    pub nodes: HashMap<Uuid, NonagonNode>,
    /// Edges between nodes
    pub edges: Vec<NonagonEdge>,
    /// Delta angle for graph position
    pub delta_angle: DeltaAngle,
}

impl NonagonGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            delta_angle: DeltaAngle::zero(),
        }
    }
    
    /// Add a node to the graph
    pub fn add_node(&mut self, node: NonagonNode) {
        self.nodes.insert(node.id, node);
    }
    
    /// Connect two nodes
    pub fn connect(
        &mut self,
        from: Uuid,
        to: Uuid,
        from_vertex: usize,
        to_vertex: usize,
        weight: f64,
    ) {
        let edge_type = if from_vertex == to_vertex {
            NonagonEdgeType::SameAspect
        } else {
            NonagonEdgeType::CrossAspect
        };
        
        self.edges.push(NonagonEdge {
            from_node: from,
            to_node: to,
            from_vertex,
            to_vertex,
            weight,
            edge_type,
        });
    }
    
    /// Global fusion across all nodes
    pub fn global_fusion(&self, mode: FusionMode) -> NonagonNode {
        let nodes: Vec<NonagonNode> = self.nodes.values().cloned().collect();
        NonagonNode::fuse(&nodes, mode)
    }
}
```

---

## 7. HD4/OTL Integration

### 7.1 Phase Mapping

Map nonagon aspects to HD4/OTL phases:

```rust
impl NonagonNode {
    /// Map current state to HD4 phase based on vertex pattern
    pub fn infer_hd4_phase(&self) -> Phase {
        // Weighted center of trivariate β (operational)
        let operational_center = (
            self.vertices[3] * 0.0 +   // Phase → 0.0
            self.vertices[4] * 0.5 +   // Intensity → 0.5
            self.vertices[5] * 1.0     // Duration → 1.0
        ) / (self.vertices[3] + self.vertices[4] + self.vertices[5] + 0.001);
        
        Phase::from_y_axis(operational_center)
    }
    
    /// Set vertices based on HD4 phase
    pub fn set_from_hd4_phase(&mut self, phase: Phase) {
        let y = phase.y_axis();
        
        // Map to trivariate β
        self.vertices[3] = 1.0 - y;      // Phase (decreases as we progress)
        self.vertices[4] = y;            // Intensity (increases)
        self.vertices[5] = y * 0.5;      // Duration (partial correlation)
    }
}
```

---

## 8. Unicode Allocation

| Range | Symbol | Component | Description |
|-------|--------|-----------|-------------|
| U+E740 | 🝀 | NAN-0 | Vertex A₀ indicator |
| U+E741 | 🝁 | NAN-1 | Vertex A₁ indicator |
| U+E742 | 🝂 | NAN-2 | Vertex A₂ indicator |
| U+E743 | 🝃 | NAN-3 | Vertex A₃ indicator |
| U+E744 | 🝄 | NAN-4 | Vertex A₄ indicator |
| U+E745 | 🝅 | NAN-5 | Vertex A₅ indicator |
| U+E746 | 🝆 | NAN-6 | Vertex A₆ indicator |
| U+E747 | 🝇 | NAN-7 | Vertex A₇ indicator |
| U+E748 | 🝈 | NAN-8 | Vertex A₈ indicator |
| U+E749 | 🝉 | NAN-C | Center (fused) indicator |
| U+E74A | 🝊 | NAN-F | Fusion operation |
| U+E74B | 🝋 | NAN-E | Elastic operation |

---

## 9. Implementation Requirements

### 9.1 MUST Requirements

1. All nonagon nodes MUST have exactly 9 vertices
2. All vertex values MUST use 6-decimal precision minimum
3. Edge indices MUST map to adjacent vertex pairs only
4. Fusion operations MUST recalculate center value
5. SCH hashes MUST be generated for addressability

### 9.2 SHOULD Requirements

1. Implementations SHOULD support all three aspect frameworks (trivariate, intel, lens)
2. Elastic operations SHOULD preserve relative vertex relationships
3. Graphs SHOULD track temporal ordering of nodes

### 9.3 MAY Requirements

1. Implementations MAY extend to 18-gon (double nonagon) for expanded analysis
2. Visualization MAY use radar/spider charts for nonagon display
3. Diagonal weights MAY be computed dynamically based on vertex similarity

---

## 10. References

- RFC-9001: Trivariate Hashing System
- RFC-9300: HD4 Canonical Specification
- RFC-9301: Thyristor, Crystal, and Ring Bus Architecture

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2025-12-06 | Initial draft |

---

*End of RFC-9302*
