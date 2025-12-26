# GLAF Code Quality / Canonical Pattern Model (v1)

## Node labels
- `LoadSet`     : one analysis run (commit-scoped)
- `Repo`        : repository identity
- `File`        : file path node
- `Function`    : function/method/symbol node
- `CanonicalPattern` : canonical block registry item
- `Finding`     : fused signal (severity/score/evidence)
- `ToolRun`     : static tool execution record
- `SemanticArtifact` : semantic TOML artifact reference
- `Match`       : canonical match instance (classification/confidence)

## Relationship types
- `(Repo)-[:HAS_LOADSET]->(LoadSet)`
- `(LoadSet)-[:SCOPED_TO]->(File)`
- `(File)-[:DECLARES]->(Function)`
- `(Function)-[:MATCHES_PATTERN]->(CanonicalPattern)` with properties: classification, confidence, structural_score, semantic_score, violations
- `(File)-[:HAS_FINDING]->(Finding)`
- `(Finding)-[:DERIVED_FROM]->(ToolRun|SemanticArtifact|Match)`
- `(LoadSet)-[:USED_TOOL]->(ToolRun)`
- `(LoadSet)-[:EMITTED]->(SemanticArtifact)`

## Id strategy (recommended)
- `Repo.id` = stable slug (e.g., "sx9-core")
- `LoadSet.id` = "glaf-<commit>-<utc>"
- `File.id` = "file:<path>"
- `Function.id` = "sym:<path>#<symbol>"
- `CanonicalPattern.id` = N-V-N-N header ID
- `Finding.id` = "finding:<loadset_id>:<signal_id>"
