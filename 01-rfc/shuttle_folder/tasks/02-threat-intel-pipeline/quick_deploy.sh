#!/bin/bash
# Quick Deploy: Neon Schema + Neo4j Gut Check
# Run: bash quick_deploy.sh

set -e

echo "🚀 Quick Deploy: Neon + Neo4j"
echo ""

# Config
NEON_URL="postgresql://neondb_owner:npg_MrhLF4bcngd8@ep-withered-breeze-a4k4oc6n-pooler.us-east-1.aws.neon.tech/neondb?sslmode=require"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# 1. Check psql
if ! command -v psql &> /dev/null; then
    echo "❌ psql not found. Installing..."
    brew install postgresql
fi

# 2. Deploy Neon schema
echo "📦 Deploying Neon schema..."
psql "$NEON_URL" -f "$SCRIPT_DIR/schema_rfc9005.sql" 2>&1 | grep -E "(CREATE|ERROR)" || true

# 3. Insert test data to Neon
echo "📝 Inserting test data..."
psql "$NEON_URL" << 'EOF'
-- Insert 5 techniques
INSERT INTO entities (trivariate_hash, name, entity_type, source_id, unicode_address, type_extensions)
VALUES 
  ('a1b2c3d4-e5f6g7h8-uuid1', 'Command and Scripting Interpreter', 'technique', 'T1059', 'U+E100', '{"tactic": "Execution"}'::jsonb),
  ('a1b2c3d5-e5f6g7h9-uuid2', 'Application Layer Protocol', 'technique', 'T1071', 'U+E101', '{"tactic": "Command and Control"}'::jsonb),
  ('a1b2c3d6-e5f6g7i0-uuid3', 'OS Credential Dumping', 'technique', 'T1003', 'U+E102', '{"tactic": "Credential Access"}'::jsonb),
  ('a1b2c3d7-e5f6g7i1-uuid4', 'Network Service Discovery', 'technique', 'T1046', 'U+E103', '{"tactic": "Discovery"}'::jsonb),
  ('a1b2c3d8-e5f6g7i2-uuid5', 'Exploit Public-Facing Application', 'technique', 'T1190', 'U+E104', '{"tactic": "Initial Access"}'::jsonb)
ON CONFLICT (trivariate_hash) DO NOTHING;

-- Insert 5 tools
INSERT INTO entities (trivariate_hash, name, entity_type, source, unicode_address, type_extensions)
VALUES 
  ('x9y8z7w6-v5u4t3s2-uuid6', 'PowerShell', 'tool', 'kali', 'U+E200', '{"command": "powershell"}'::jsonb),
  ('x9y8z7w7-v5u4t3s3-uuid7', 'Nmap', 'tool', 'kali', 'U+E201', '{"command": "nmap"}'::jsonb),
  ('x9y8z7w8-v5u4t3s4-uuid8', 'Mimikatz', 'tool', 'atomic', 'U+E202', '{"command": "mimikatz"}'::jsonb),
  ('x9y8z7w9-v5u4t3s5-uuid9', 'Metasploit', 'tool', 'kali', 'U+E203', '{"command": "msfconsole"}'::jsonb),
  ('x9y8z7x0-v5u4t3s6-uuid10', 'Nuclei', 'tool', 'nuclei', 'U+E204', '{"command": "nuclei"}'::jsonb)
ON CONFLICT (trivariate_hash) DO NOTHING;

-- Insert relationships
INSERT INTO relationships (source_entity_id, target_entity_id, relationship_type, confidence)
SELECT 
  (SELECT id FROM entities WHERE name = 'PowerShell'),
  (SELECT id FROM entities WHERE source_id = 'T1059'),
  'implements', 0.98
WHERE NOT EXISTS (
  SELECT 1 FROM relationships 
  WHERE source_entity_id = (SELECT id FROM entities WHERE name = 'PowerShell')
    AND target_entity_id = (SELECT id FROM entities WHERE source_id = 'T1059')
);

INSERT INTO relationships (source_entity_id, target_entity_id, relationship_type, confidence)
SELECT 
  (SELECT id FROM entities WHERE name = 'Nmap'),
  (SELECT id FROM entities WHERE source_id = 'T1046'),
  'implements', 0.95
WHERE NOT EXISTS (
  SELECT 1 FROM relationships 
  WHERE source_entity_id = (SELECT id FROM entities WHERE name = 'Nmap')
    AND target_entity_id = (SELECT id FROM entities WHERE source_id = 'T1046')
);

INSERT INTO relationships (source_entity_id, target_entity_id, relationship_type, confidence)
SELECT 
  (SELECT id FROM entities WHERE name = 'Mimikatz'),
  (SELECT id FROM entities WHERE source_id = 'T1003'),
  'implements', 0.99
WHERE NOT EXISTS (
  SELECT 1 FROM relationships 
  WHERE source_entity_id = (SELECT id FROM entities WHERE name = 'Mimikatz')
    AND target_entity_id = (SELECT id FROM entities WHERE source_id = 'T1003')
);

-- Show stats
SELECT 
  entity_type, 
  count(*) as count
FROM entities 
GROUP BY entity_type;

SELECT count(*) as relationship_count FROM relationships;
EOF

echo "✅ Neon data loaded"

# 4. Install Node deps
echo "📦 Installing Node dependencies..."
cd "$SCRIPT_DIR"
npm install --silent neo4j-driver @neondatabase/serverless 2>&1 | grep -v "npm WARN" || true

# 5. Sync to Neo4j
echo "🔄 Syncing to Neo4j..."
node sync_neon_to_neo4j.js

echo ""
echo "✅ DONE!"
echo ""
echo "🌐 Open Neo4j Browser: http://localhost:7474"
echo "   Username: neo4j"
echo "   Password: password"
echo ""
echo "📊 Run this query to see the graph:"
echo "   MATCH (n)-[r]->(m) RETURN n, r, m LIMIT 50"
echo ""
