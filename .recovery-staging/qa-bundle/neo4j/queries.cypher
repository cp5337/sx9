// SX9 / GLAF Neo4j Example Queries

// 1) Files with highest concentration of high/critical findings
MATCH (f:File)-[:HAS_FINDING]->(x:Finding)
WHERE x.severity IN ['high','critical']
RETURN f.path, count(x) AS n
ORDER BY n DESC
LIMIT 25;

// 2) Pattern drift: partial matches with violations
MATCH (fn:Function)-[r:MATCHES_PATTERN]->(p:CanonicalPattern)
WHERE r.classification = 'PARTIAL_MATCH' AND size(r.violations) > 0
RETURN fn.id, p.id, r.confidence, r.violations
ORDER BY r.confidence DESC
LIMIT 50;

// 3) Canonical patterns most frequently implemented
MATCH (:Function)-[r:MATCHES_PATTERN]->(p:CanonicalPattern)
RETURN p.id, count(r) AS uses
ORDER BY uses DESC
LIMIT 25;
