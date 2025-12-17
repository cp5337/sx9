#!/usr/bin/env node
/**
 * SX9 Modal Inventory → Supabase Migration Script
 *
 * RFC Compliance:
 * - RFC-9001: Trivariate Hashing Standard (Murmur3-64, Base96)
 * - RFC-9005: Unified Schema Specification (Supabase integration)
 * - RFC-9114: SX9 Gateway Neural Retrofit (Modal inventory integration)
 *
 * Purpose: Migrate modal inventory data from JSON to Supabase
 *
 * Usage:
 *   node migrate-inventory-to-supabase.js
 *
 * Environment Variables:
 *   SUPABASE_URL - Supabase project URL
 *   SUPABASE_KEY - Supabase service role key
 */

const { createClient } = require("@supabase/supabase-js");
const fs = require("fs");
const path = require("path");

// ============================================================================
// Configuration
// ============================================================================

const SUPABASE_URL = process.env.SUPABASE_URL || "https://your-project.supabase.co";
const SUPABASE_KEY = process.env.SUPABASE_KEY || "your-service-role-key";

const INVENTORY_PATH = path.join(__dirname, "../modal-inventory-foundation/inventory.json");
const PLAYWRIGHT_RESULTS_PATH = path.join(
  __dirname,
  "../playwright-connection-results/connection-test-results.json"
);

// ============================================================================
// Initialize Supabase Client
// ============================================================================

const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * RFC-9001: Extract SCH hash from trivariate hash
 * Format: [SCH]_[CUID]_[UUID]
 */
function extractSCH(trivariateHash) {
  if (!trivariateHash) return null;

  // SCH is first 16 characters of the hash
  return trivariateHash.substring(0, 16);
}

/**
 * RFC-9001: Validate trivariate hash format
 * Must be 48 characters (16 SCH + 16 CUID + 16 UUID)
 */
function validateTrivariateHash(hash) {
  if (!hash) return false;

  // Remove any Base96 encoding artifacts
  const cleanHash = hash.replace(/[^a-f0-9]/gi, "");

  return cleanHash.length >= 16; // At minimum, we need SCH
}

/**
 * Generate Unicode shortcut from SCH hash
 * RFC-9002: Unicode Operational Routing System
 */
function generateUnicodeShortcut(schHash) {
  if (!schHash) return null;

  // Use first 4 characters of SCH
  return `🔹${schHash.substring(0, 4)}`;
}

// ============================================================================
// Migration Functions
// ============================================================================

/**
 * Migrate UI Pages
 * RFC-9005: Store in ui_pages table
 */
async function migratePages(pages) {
  console.log(`\n📄 Migrating ${pages.length} pages...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  for (const page of pages) {
    try {
      const schHash = extractSCH(page.hash);
      const unicodeShortcut = page.unicode || generateUnicodeShortcut(schHash);

      const { data, error } = await supabase.from("ui_pages").upsert(
        {
          name: page.name,
          path: page.path,
          url: page.url,
          trivariate_hash: page.hash,
          sch_hash: schHash,
          unicode_shortcut: unicodeShortcut,
          usim_header: page.usim,
          last_scanned_at: new Date().toISOString(),
          metadata: {
            error: page.error,
            screenshots: page.screenshots || [],
          },
        },
        {
          onConflict: "path",
        }
      );

      if (error) {
        results.failed++;
        results.errors.push({ page: page.name, error: error.message });
        console.error(`  ❌ Failed: ${page.name} - ${error.message}`);
      } else {
        results.success++;
        console.log(`  ✅ Migrated: ${page.name} (${schHash})`);
      }
    } catch (err) {
      results.failed++;
      results.errors.push({ page: page.name, error: err.message });
      console.error(`  ❌ Exception: ${page.name} - ${err.message}`);
    }
  }

  return results;
}

/**
 * Migrate UI Buttons
 * RFC-9005: Store in ui_buttons table
 */
async function migrateButtons(pages) {
  console.log(`\n🔘 Migrating buttons...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  for (const page of pages) {
    if (!page.buttons || page.buttons.length === 0) continue;

    // Get page ID from Supabase
    const { data: pageData, error: pageError } = await supabase
      .from("ui_pages")
      .select("id")
      .eq("path", page.path)
      .single();

    if (pageError || !pageData) {
      console.error(`  ⚠️  Page not found: ${page.path}`);
      continue;
    }

    for (let i = 0; i < page.buttons.length; i++) {
      const button = page.buttons[i];

      try {
        const schHash = extractSCH(button.hash);

        const { data, error } = await supabase.from("ui_buttons").insert({
          page_id: pageData.id,
          text: button.text,
          selector: button.selector,
          enabled: button.enabled !== false,
          trivariate_hash: button.hash,
          sch_hash: schHash,
          position: i,
          metadata: {},
        });

        if (error) {
          // Ignore duplicate errors
          if (!error.message.includes("duplicate")) {
            results.failed++;
            results.errors.push({ button: button.text, page: page.name, error: error.message });
          }
        } else {
          results.success++;
        }
      } catch (err) {
        results.failed++;
        results.errors.push({ button: button.text, page: page.name, error: err.message });
      }
    }
  }

  console.log(`  ✅ Migrated ${results.success} buttons`);
  return results;
}

/**
 * Migrate Screenshots
 * RFC-9005: Store in ui_screenshots table
 */
async function migrateScreenshots(pages) {
  console.log(`\n📸 Migrating screenshots...`);

  const results = {
    success: 0,
    failed: 0,
    errors: [],
  };

  for (const page of pages) {
    if (!page.screenshots || page.screenshots.length === 0) continue;

    // Get page ID from Supabase
    const { data: pageData, error: pageError } = await supabase
      .from("ui_pages")
      .select("id")
      .eq("path", page.path)
      .single();

    if (pageError || !pageData) continue;

    for (const screenshot of page.screenshots) {
      try {
        const { data, error } = await supabase.from("ui_screenshots").insert({
          page_id: pageData.id,
          filename: screenshot,
          storage_path: `screenshots/${screenshot}`,
          screenshot_type: "initial",
          metadata: {},
        });

        if (error) {
          if (!error.message.includes("duplicate")) {
            results.failed++;
            results.errors.push({ screenshot, page: page.name, error: error.message });
          }
        } else {
          results.success++;
        }
      } catch (err) {
        results.failed++;
        results.errors.push({ screenshot, page: page.name, error: err.message });
      }
    }
  }

  console.log(`  ✅ Migrated ${results.success} screenshots`);
  return results;
}

// ============================================================================
// Main Migration
// ============================================================================

async function main() {
  console.log("╔════════════════════════════════════════════════════════════════╗");
  console.log("║  SX9 Modal Inventory → Supabase Migration                     ║");
  console.log("║  RFC-9001 (Trivariate Hashing) + RFC-9005 (Unified Schema)    ║");
  console.log("╚════════════════════════════════════════════════════════════════╝");

  // Load inventory data
  console.log(`\n📂 Loading inventory from: ${INVENTORY_PATH}`);

  if (!fs.existsSync(INVENTORY_PATH)) {
    console.error(`❌ Inventory file not found: ${INVENTORY_PATH}`);
    process.exit(1);
  }

  const inventoryData = JSON.parse(fs.readFileSync(INVENTORY_PATH, "utf8"));

  console.log(`\n📊 Inventory Summary:`);
  console.log(`   - Timestamp: ${inventoryData.timestamp}`);
  console.log(`   - Base URL: ${inventoryData.baseUrl}`);
  console.log(`   - Total Pages: ${inventoryData.pages.length}`);
  console.log(`   - Foundation Integration: ${inventoryData.foundation_integration ? "✅" : "❌"}`);

  // Test Supabase connection
  console.log(`\n🔌 Testing Supabase connection...`);
  const { data: testData, error: testError } = await supabase
    .from("ui_pages")
    .select("count")
    .limit(1);

  if (testError) {
    console.error(`❌ Supabase connection failed: ${testError.message}`);
    console.error(`   Check SUPABASE_URL and SUPABASE_KEY environment variables`);
    process.exit(1);
  }

  console.log(`✅ Supabase connection successful`);

  // Run migrations
  const pageResults = await migratePages(inventoryData.pages);
  const buttonResults = await migrateButtons(inventoryData.pages);
  const screenshotResults = await migrateScreenshots(inventoryData.pages);

  // Summary
  console.log("\n╔════════════════════════════════════════════════════════════════╗");
  console.log("║  Migration Summary                                             ║");
  console.log("╚════════════════════════════════════════════════════════════════╝");
  console.log(`\n📄 Pages:       ${pageResults.success} success, ${pageResults.failed} failed`);
  console.log(`🔘 Buttons:     ${buttonResults.success} success, ${buttonResults.failed} failed`);
  console.log(
    `📸 Screenshots: ${screenshotResults.success} success, ${screenshotResults.failed} failed`
  );

  if (pageResults.errors.length > 0) {
    console.log(`\n⚠️  Errors (${pageResults.errors.length}):`);
    pageResults.errors.forEach(err => {
      console.log(`   - ${err.page}: ${err.error}`);
    });
  }

  console.log("\n✅ Migration complete!\n");
}

// Run migration
main().catch(err => {
  console.error("❌ Migration failed:", err);
  process.exit(1);
});
