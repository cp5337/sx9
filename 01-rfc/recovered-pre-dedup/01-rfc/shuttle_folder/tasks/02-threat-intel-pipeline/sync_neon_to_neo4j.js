#!/usr/bin/env node
/**
 * Sync Neon → Neo4j for Graph Gut Check
 *
 * Purpose: Push Neon data to Neo4j to visually verify graph structure
 * Usage: node sync_neon_to_neo4j.js
 */

const { neon } = require("@neondatabase/serverless");
const neo4j = require("neo4j-driver");

// Config
const NEON_URL =
  process.env.NEON_DATABASE_URL ||
  "postgresql://neondb_owner:npg_MrhLF4bcngd8@ep-withered-breeze-a4k4oc6n-pooler.us-east-1.aws.neon.tech/neondb?sslmode=require";

const NEO4J_URI = process.env.NEO4J_URI || "bolt://localhost:7687";
const NEO4J_USER = process.env.NEO4J_USER || "neo4j";
const NEO4J_PASSWORD = process.env.NEO4J_PASSWORD || "password";

async function main() {
  console.log("🔄 Syncing Neon → Neo4j for gut check...\n");

  // Connect to Neon
  const sql = neon(NEON_URL);

  // Connect to Neo4j
  const driver = neo4j.driver(
    NEO4J_URI,
    neo4j.auth.basic(NEO4J_USER, NEO4J_PASSWORD)
  );
  const session = driver.session();

  try {
    // Clear Neo4j (fresh start)
    console.log("🧹 Clearing Neo4j...");
    await session.run("MATCH (n) DETACH DELETE n");

    // Create constraints
    console.log("📐 Creating constraints...");
    await session.run(`
      CREATE CONSTRAINT entity_trivariate IF NOT EXISTS
      FOR (e:Entity) REQUIRE e.trivariate_hash IS UNIQUE
    `);

    // Fetch entities from Neon
    console.log("📥 Fetching entities from Neon...");
    const entities = await sql`
      SELECT id, trivariate_hash, name, entity_type, unicode_address, source, source_id
      FROM entities
      WHERE entity_type IN ('tool', 'technique')
      LIMIT 100
    `;

    console.log(`   Found ${entities.length} entities`);

    // Create Neo4j nodes
    console.log("📝 Creating Neo4j nodes...");
    for (const entity of entities) {
      const labels =
        entity.entity_type === "technique" ? "Technique:Entity" : "Tool:Entity";

      await session.run(
        `
        CREATE (n:${labels} {
          id: $id,
          trivariate_hash: $trivariate_hash,
          name: $name,
          entity_type: $entity_type,
          unicode_address: $unicode_address,
          source: $source,
          source_id: $source_id
        })
      `,
        {
          id: entity.id,
          trivariate_hash: entity.trivariate_hash,
          name: entity.name,
          entity_type: entity.entity_type,
          unicode_address: entity.unicode_address,
          source: entity.source,
          source_id: entity.source_id,
        }
      );
    }

    // Fetch relationships from Neon
    console.log("📥 Fetching relationships from Neon...");
    const relationships = await sql`
      SELECT source_entity_id, target_entity_id, relationship_type, confidence
      FROM relationships
      WHERE relationship_type IN ('covers_technique', 'exploits_technique', 'implements')
      LIMIT 100
    `;

    console.log(`   Found ${relationships.length} relationships`);

    // Create Neo4j relationships
    console.log("🔗 Creating Neo4j relationships...");
    for (const rel of relationships) {
      await session.run(
        `
        MATCH (a:Entity {id: $source})
        MATCH (b:Entity {id: $target})
        CREATE (a)-[r:IMPLEMENTS {
          confidence: $confidence,
          relationship_type: $type
        }]->(b)
      `,
        {
          source: rel.source_entity_id,
          target: rel.target_entity_id,
          confidence: rel.confidence,
          type: rel.relationship_type,
        }
      );
    }

    // Stats
    console.log("\n✅ Sync complete!\n");

    const stats = await session.run(`
      MATCH (n) 
      RETURN labels(n)[0] as label, count(n) as count
      ORDER BY count DESC
    `);

    console.log("📊 Neo4j Stats:");
    stats.records.forEach((record) => {
      console.log(`   ${record.get("label")}: ${record.get("count")}`);
    });

    const relStats = await session.run(`
      MATCH ()-[r]->() 
      RETURN type(r) as type, count(r) as count
    `);

    console.log("\n🔗 Relationships:");
    relStats.records.forEach((record) => {
      console.log(`   ${record.get("type")}: ${record.get("count")}`);
    });

    console.log("\n🌐 Open Neo4j Browser: http://localhost:7474");
    console.log("   Username: neo4j");
    console.log("   Password: password\n");
  } catch (error) {
    console.error("❌ Error:", error.message);
    throw error;
  } finally {
    await session.close();
    await driver.close();
  }
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
