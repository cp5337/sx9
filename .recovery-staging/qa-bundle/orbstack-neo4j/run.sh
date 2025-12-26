#!/usr/bin/env bash
set -euo pipefail
mkdir -p data logs import
docker compose up -d
echo "Waiting for Neo4j to be ready..."
sleep 10
echo "Loading schema..."
cat ../neo4j/schema.cypher | docker exec -i sx9_neo4j cypher-shell -u neo4j -p testpassword
echo "Neo4j ready at http://localhost:7474 (neo4j/testpassword)"
