RFC-9000-Core: Synaptix9 Agnostic Core & Ontology Standard

This is the anchor everything else MUST reference.

Sections

Purpose and Scope

Why SX9 exists

Relationship to CTAS-7, Cognigraph, PTCC 33 primitives

“This RFC is the parent of all others”

Agnostic Core Principles

Domain-agnostic first

PTCC 33 primitives as core mental model

Separation of:

Ontology

Execution

Storage

Sensory/Haptics

SX9 Primitive System

10 primitive types from ontology:

Actor, Object, Event, Concept, Attribute, Function, Module, Header, Footer, Comment

Mapping table:

PTCC 33 ↔ SX9 primitives ↔ HD4 ↔ CTAS tasks

Ontology & OntoGPT/SPIRES Integration

CTAS_ONTOLOGY_SCHEMA.yaml structure

Extraction templates (CTAS_EXTRACTION_TEMPLATES.yaml)

SPIRES roles: extraction, validation, RDF, KG

How ALL domains must onboard via this pathway

Graph & Knowledge Fabric

CTAS Complete Tactical Ontology

Cognigraph & SX9 Ontology Engine (TS + Rust)

Node and edge types, H/B (Hub/Bridge) classification

HD4, PTCC, and primitive overlays

Hash & Unicode Integration (Reference to RFC-9001/9002/9005)

SCH / CUID / UUID roles (no Blake mention)

Unicode DSL rune layer

Tail semantics, angles, suppression tiers (link, not define in depth here)

Database Layer & Normal Forms

Supabase (BCNF where possible, 3NF fallback)

SurrealDB, Sled, Sledis roles

1n–5n hash structures & their storage expectations

Requirements for any future store to be “SX9-compliant”

Conformance Requirements

What a “conforming vertical” must implement

Required hooks: ontology, hash mapping, HD4/PTCC mapping

Versioning and backward-compatibility

Security & Governance

Ownership of core

RFC evolution rules

Test + validation expectations

References

OntoGPT docs

CTAS ontology files

RFC-9001–9005 pointers