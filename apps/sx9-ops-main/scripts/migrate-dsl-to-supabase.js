#!/usr/bin/env node
/**
 * SX9 DSL Orchestration → Supabase Migration Script
 *
 * RFC Compliance:
 * - RFC-9001: Trivariate Hashing Standard
 * - RFC-9005: Unified Schema Specification
 * - RFC-9101: Smart Crate System
 * - RFC-9114: SX9 Gateway Neural Retrofit
 *
 * Purpose: Migrate DSL crate grouping data to Supabase
 *
 * Usage:
 *   node migrate-dsl-to-supabase.js
 */

const { createClient } = require("@supabase/supabase-js");
const fs = require("fs");
const path = require("path");
const { parseStringPromise } = require("xml2js");

// ============================================================================
// Configuration
// ============================================================================

const SUPABASE_URL = process.env.SUPABASE_URL || "https://your-project.supabase.co";
const SUPABASE_KEY = process.env.SUPABASE_KEY || "your-service-role-key";

const DSL_PATH = path.join(__dirname, "../DSL-orchestration/dsl-crate-grouping-system.dsl");

// ============================================================================
// Initialize Supabase Client
// ============================================================================

const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);

// ============================================================================
// Migration Functions
// ============================================================================

/**
 * Migrate Crate Groups
 * RFC-9101: Smart Crate System groups
 */
async function migrateCrateGroups(grouping) {
  console.log(`\n📦 Migrating crate groups...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  const crateGroups = grouping.ctasCrateGrouping.crateGroup || [];

  for (const group of crateGroups) {
    try {
      const groupId = group.$.groupId;
      const groupName = group.$.groupName;
      const groupType = group.$.groupType;

      // Extract criteria
      const criteria = group.criteria?.[0]?.criterion || [];

      // Extract operational capabilities
      const capabilities = group.operationalCapabilities?.[0]?.capability || [];

      const { data, error } = await supabase.from("crate_groups").upsert(
        {
          group_id: groupId,
          group_name: groupName,
          group_type: groupType,
          description: group.description?.[0] || "",
          criteria: JSON.stringify(criteria),
          operational_capabilities: JSON.stringify(capabilities),
          metadata: {},
        },
        {
          onConflict: "group_id",
        }
      );

      if (error) {
        results.failed++;
        results.errors.push({ group: groupName, error: error.message });
        console.error(`  ❌ Failed: ${groupName} - ${error.message}`);
      } else {
        results.success++;
        console.log(`  ✅ Migrated: ${groupName} (${groupId})`);
      }
    } catch (err) {
      results.failed++;
      results.errors.push({ group: group.$.groupName, error: err.message });
      console.error(`  ❌ Exception: ${group.$.groupName} - ${err.message}`);
    }
  }

  return results;
}

/**
 * Migrate Crates
 * RFC-9101: Individual crate entries
 */
async function migrateCrates(grouping) {
  console.log(`\n📦 Migrating crates...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  const crateGroups = grouping.ctasCrateGrouping.crateGroup || [];

  for (const group of crateGroups) {
    const groupId = group.$.groupId;

    // Get group UUID from Supabase
    const { data: groupData, error: groupError } = await supabase
      .from("crate_groups")
      .select("id")
      .eq("group_id", groupId)
      .single();

    if (groupError || !groupData) {
      console.error(`  ⚠️  Group not found: ${groupId}`);
      continue;
    }

    const crates = group.crates?.[0]?.crate || [];

    for (const crateName of crates) {
      try {
        const { data, error } = await supabase.from("crates").upsert(
          {
            crate_name: crateName,
            group_id: groupData.id,
            description: `${crateName} - Part of ${group.$.groupName} group`,
            smart_crate_version: "1.2.0",
            tesla_grade: false,
            metadata: {
              group_type: group.$.groupType,
            },
          },
          {
            onConflict: "crate_name",
          }
        );

        if (error) {
          if (!error.message.includes("duplicate")) {
            results.failed++;
            results.errors.push({ crate: crateName, error: error.message });
          }
        } else {
          results.success++;
        }
      } catch (err) {
        results.failed++;
        results.errors.push({ crate: crateName, error: err.message });
      }
    }
  }

  console.log(`  ✅ Migrated ${results.success} crates`);
  return results;
}

/**
 * Migrate Operational Capabilities
 * RFC-9114: Operational intelligence mapping
 */
async function migrateOperationalCapabilities(grouping) {
  console.log(`\n🎯 Migrating operational capabilities...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  const mapping = grouping.ctasCrateGrouping.operationalIntelligenceMapping?.[0];
  if (!mapping) {
    console.log(`  ⚠️  No operational intelligence mapping found`);
    return results;
  }

  const capabilities = mapping.intelligenceCapabilities?.[0]?.capability || [];

  for (const capability of capabilities) {
    try {
      const capabilityName = capability.$.name;
      const description = capability.description?.[0] || "";
      const groupMapping = capability.groupMapping?.[0]?.group || [];
      const crateMapping = capability.crateMapping?.[0] || "";
      const assessment = capability.assessment?.[0] || "";

      const { data, error } = await supabase.from("operational_capabilities").upsert(
        {
          capability_name: capabilityName,
          capability_type: mapping.$.mappingType || "threat_emulation_capability",
          description: description,
          group_mapping: JSON.stringify(groupMapping),
          crate_mapping: crateMapping,
          assessment: assessment,
          metadata: {},
        },
        {
          onConflict: "capability_name",
        }
      );

      if (error) {
        results.failed++;
        results.errors.push({ capability: capabilityName, error: error.message });
        console.error(`  ❌ Failed: ${capabilityName} - ${error.message}`);
      } else {
        results.success++;
        console.log(`  ✅ Migrated: ${capabilityName}`);
      }
    } catch (err) {
      results.failed++;
      results.errors.push({ capability: capability.$.name, error: err.message });
      console.error(`  ❌ Exception: ${capability.$.name} - ${err.message}`);
    }
  }

  return results;
}

/**
 * Migrate Component Mappings
 * RFC-9115: Frontend adapter integration
 */
async function migrateComponentMappings(grouping) {
  console.log(`\n🖥️  Migrating component mappings...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  const frontendMapping = grouping.ctasCrateGrouping.frontendComponentMapping?.[0];
  if (!frontendMapping) {
    console.log(`  ⚠️  No frontend component mapping found`);
    return results;
  }

  const mappings = frontendMapping.componentMappings?.[0]?.mapping || [];

  for (const mapping of mappings) {
    try {
      const mappingName = mapping.$.name;
      const groupId = mapping.group?.[0];
      const uiComponent = mapping.uiComponent?.[0];
      const dashboardIntegration = mapping.dashboardIntegration?.[0];
      const realTimeData = mapping.realTimeData?.[0] === "true";
      const description = mapping.description?.[0] || "";

      // Get group UUID from Supabase
      const { data: groupData, error: groupError } = await supabase
        .from("crate_groups")
        .select("id")
        .eq("group_id", groupId)
        .single();

      if (groupError || !groupData) {
        console.error(`  ⚠️  Group not found: ${groupId}`);
        continue;
      }

      const { data, error } = await supabase.from("component_mappings").upsert(
        {
          mapping_name: mappingName,
          group_id: groupData.id,
          ui_component: uiComponent,
          dashboard_integration: dashboardIntegration,
          real_time_data: realTimeData,
          description: description,
          metadata: {},
        },
        {
          onConflict: "mapping_name",
        }
      );

      if (error) {
        results.failed++;
        results.errors.push({ mapping: mappingName, error: error.message });
        console.error(`  ❌ Failed: ${mappingName} - ${error.message}`);
      } else {
        results.success++;
        console.log(`  ✅ Migrated: ${mappingName}`);
      }
    } catch (err) {
      results.failed++;
      results.errors.push({ mapping: mapping.$.name, error: err.message });
      console.error(`  ❌ Exception: ${mapping.$.name} - ${err.message}`);
    }
  }

  return results;
}

// ============================================================================
// Main Migration
// ============================================================================

async function main() {
  console.log("╔════════════════════════════════════════════════════════════════╗");
  console.log("║  SX9 DSL Orchestration → Supabase Migration                   ║");
  console.log("║  RFC-9101 (Smart Crate) + RFC-9114 (Gateway)                  ║");
  console.log("╚════════════════════════════════════════════════════════════════╝");

  // Load DSL data
  console.log(`\n📂 Loading DSL from: ${DSL_PATH}`);

  if (!fs.existsSync(DSL_PATH)) {
    console.error(`❌ DSL file not found: ${DSL_PATH}`);
    process.exit(1);
  }

  const dslContent = fs.readFileSync(DSL_PATH, "utf8");
  const grouping = await parseStringPromise(dslContent);

  const metadata = grouping.ctasCrateGrouping.metadata?.[0];
  console.log(`\n📊 DSL Summary:`);
  console.log(`   - Grouping ID: ${metadata?.groupingId?.[0]}`);
  console.log(`   - Version: ${metadata?.version?.[0]}`);
  console.log(`   - Status: ${metadata?.status?.[0]}`);
  console.log(`   - Author: ${metadata?.author?.[0]}`);

  // Test Supabase connection
  console.log(`\n🔌 Testing Supabase connection...`);
  const { data: testData, error: testError } = await supabase
    .from("crate_groups")
    .select("count")
    .limit(1);

  if (testError) {
    console.error(`❌ Supabase connection failed: ${testError.message}`);
    process.exit(1);
  }

  console.log(`✅ Supabase connection successful`);

  // Run migrations
  const groupResults = await migrateCrateGroups(grouping);
  const crateResults = await migrateCrates(grouping);
  const capabilityResults = await migrateOperationalCapabilities(grouping);
  const componentResults = await migrateComponentMappings(grouping);

  // Summary
  console.log("\n╔════════════════════════════════════════════════════════════════╗");
  console.log("║  Migration Summary                                             ║");
  console.log("╚════════════════════════════════════════════════════════════════╝");
  console.log(`\n📦 Groups:       ${groupResults.success} success, ${groupResults.failed} failed`);
  console.log(`📦 Crates:       ${crateResults.success} success, ${crateResults.failed} failed`);
  console.log(
    `🎯 Capabilities: ${capabilityResults.success} success, ${capabilityResults.failed} failed`
  );
  console.log(
    `🖥️  Components:   ${componentResults.success} success, ${componentResults.failed} failed`
  );

  console.log("\n✅ Migration complete!\n");
}

// Run migration
main().catch(err => {
  console.error("❌ Migration failed:", err);
  process.exit(1);
});
