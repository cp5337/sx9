// SX9 / GLAF Neo4j Schema (Constraints + Indexes) — v1
// Apply with: cat neo4j/schema.cypher | cypher-shell -u neo4j -p <password>

// Uniqueness constraints
CREATE CONSTRAINT repo_id_unique IF NOT EXISTS
FOR (n:Repo) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT loadset_id_unique IF NOT EXISTS
FOR (n:LoadSet) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT file_id_unique IF NOT EXISTS
FOR (n:File) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT function_id_unique IF NOT EXISTS
FOR (n:Function) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT pattern_id_unique IF NOT EXISTS
FOR (n:CanonicalPattern) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT finding_id_unique IF NOT EXISTS
FOR (n:Finding) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT toolrun_id_unique IF NOT EXISTS
FOR (n:ToolRun) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT semantic_id_unique IF NOT EXISTS
FOR (n:SemanticArtifact) REQUIRE n.id IS UNIQUE;

CREATE CONSTRAINT match_id_unique IF NOT EXISTS
FOR (n:Match) REQUIRE n.id IS UNIQUE;

// Helpful indexes
CREATE INDEX file_path_index IF NOT EXISTS
FOR (n:File) ON (n.path);

CREATE INDEX function_name_index IF NOT EXISTS
FOR (n:Function) ON (n.name);

CREATE INDEX finding_severity_index IF NOT EXISTS
FOR (n:Finding) ON (n.severity);

CREATE INDEX pattern_category_index IF NOT EXISTS
FOR (n:CanonicalPattern) ON (n.category);

// Recommended relationship property conventions:
// (Function)-[:MATCHES_PATTERN {classification, confidence, structural_score, semantic_score, violations[]}]->(CanonicalPattern)
// (File)-[:HAS_FINDING {score, severity}]->(Finding)
