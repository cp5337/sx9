RFC-9012: SX9 Embeddings & GNN Training Fabric

Governs how embeddings, tails, angles, and GNNs interact.

Sections

Purpose

Standard for embedding generation and GNN use in SX9

Ensure all embeddings respect ontology + hash + DSL structure

Embedding Spaces

Code embeddings

DSL/playbook embeddings

Ontology embeddings

Tool/technique embeddings (Nuclei, Caldera, etc.)

Tail/angle embeddings

Tail & Angular Semantics

Tail fields contributing to embedding space

Angle-based attraction/repulsion patterns

Suppression/optionality tiers (more than three; define ladder here)

Representing haptic/voice tone hints as vectors

Unicode as Embedding Substrate

Unicode rune space as structured, not arbitrary token IDs

Mapping from rune → semantic vector → GNN node features

Collision avoidance and audit

GNN Topologies

Graph types (scenario graph, ontology graph, tool graph)

Node/edge features from ontology + DSL + tails

Use cases: similarity, recommendation, completion, anomaly detection

Training Pipelines

Data sources: ontology JSON, TTL, YAMLs, CTAS scenarios

Preprocessing requirements

Split strategy: train/val/test across domains, not just examples

Incremental retraining and versioning

Model Serving & Integration

How SX9 systems call embeddings/GNNs

Expected latency targets

Version pinning / model registry requirements

Safety & Stability

Guardrails around automated semantic filling

Preventing “semantic drift” in core primitives

Monitoring embedding space over time

Conformance

Requirements for any new embedding backend to be SX9-compliant