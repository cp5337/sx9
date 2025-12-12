#!/bin/bash
set -e

# Wait for Neo4j to be ready
until curl -s http://localhost:7474 > /dev/null; do
  echo "Waiting for Neo4j..."
  sleep 1
done

# Initialize Neo4j with CTAS schema
cypher-shell -u neo4j -p "$NEO4J_PASSWORD" << 'EOF'
CREATE CONSTRAINT unique_phase_name IF NOT EXISTS ON (p:Phase) ASSERT p.name IS UNIQUE;
CREATE CONSTRAINT unique_task_id IF NOT EXISTS ON (t:Task) ASSERT t.id IS UNIQUE;
CREATE CONSTRAINT unique_threat_actor_id IF NOT EXISTS ON (a:ThreatActor) ASSERT a.id IS UNIQUE;
EOF