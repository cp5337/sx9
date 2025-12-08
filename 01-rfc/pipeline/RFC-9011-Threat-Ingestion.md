RFC-9011: SX9 Threat Content Ingestion: YAML → DSL → Playbooks

The one that governs Nuclei, Caldera, ATT&CK, Atomic Red Team, Nmap, etc.

Sections

Purpose

Standard for ingesting external threat content into SX9

Domain-agnostic intent (these are “pattern catalogs”)

Supported Source Families

Nuclei

Caldera

MITRE ATT&CK (techniques, matrices, STIX)

Atomic Red Team

Nmap scripts

Future sources (generic YAML/STIX/JSON specification)

Canonical Data Model

SX9 DSL structure

Playbook model

Relationship to SX9 primitives, PTCC, HD4, and ontology

Ingestion Pipeline Architecture

Fetch layer (GitHub, local, registry)

Parser / normalizer

Crosswalk Engine (Nuclei/Caldera/ATT&CK ↔ SX9)

Semantic Conflict Resolver

Semantic Imputer / Filler

Hash & Unicode tail assignment

Storage in BCNF/3NF Supabase + SurrealDB graph

Crosswalk Engine Specification

Mapping rules between source families and:

ATT&CK techniques

PTCC primitives

SX9 primitives

HD4 phases

Conflict resolution / tie-breaking logic

Semantic Conflict Resolver (SCR)

Conditions for conflict

Severity levels

Auto-merge vs auto-reject vs human-review

Logging / audit requirements

Semantic Filler/Imputer (SFE)

When imputation is allowed

Use of GNN + Phi-3 for filling:

primitive type

HD4 phase

severity

missing relationships

tags / DSL hints

Confidence thresholds and override rules

SX9 DSL & Unicode Tail Assignment

DSL opcodes and structure

Mapping semantics to tails (angles, suppression tiers)

How the ingest pipeline calls hash RFCs (9001/9002)

Requirements for collision handling

Playbook Generation

Single-atomic-test → minimal SX9 playbook

Multi-step patterns → composite playbooks

Linking to CTAS scenarios and SX9 ontology

Persistence & Indexing

Where in Supabase each artifact lives (tables & BCNF groups)

SurrealDB graph schemas for fast GNN use

Sled/Sledis caches for runtime usage

Testing & Validation

Round-trip validation (YAML → DSL → YAML-equivalent semantics)

Sampling & sanity checks (e.g., severity distribution, mapping coverage)

Drift detection over time

Conformance

What it means for a new content family to be SX9-ingest-compatible