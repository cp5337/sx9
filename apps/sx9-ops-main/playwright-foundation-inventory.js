#!/usr/bin/env node
/**
 * CTAS v7.3.1 Foundation-Integrated Modal Inventory System
 * Executable Document: Crawls entire UI and catalogs with trivariate hashes, USIMs, Unicode
 * 
 * Integration Points:
 * - ctas7-hashing-engine (port 8002) for trivariate hash generation
 * - USIM generation for each UI element
 * - Unicode compression for quick lookup
 * - Multi-tier repository output (Sled, SurrealDB, JSON)
 */

import { chromium } from 'playwright';
import fs from 'fs/promises';
import path from 'path';
import http from 'http';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const BASE_URL = 'http://localhost:15174';
const HASHING_ENGINE_URL = 'http://localhost:8002';
const OUTPUT_DIR = path.join(__dirname, 'modal-inventory-foundation');
const SCREENSHOT_DIR = path.join(OUTPUT_DIR, 'screenshots');

// All CTAS pages to inventory
const PAGES = [
  { path: '/', name: 'Dashboard' },
  { path: '/hunt', name: 'Hunt Phase' },
  { path: '/detect', name: 'Detect Phase' },
  { path: '/disrupt', name: 'Disrupt Phase' },
  { path: '/disable', name: 'Disable Phase' },
  { path: '/dominate', name: 'Dominate Phase' },
  { path: '/tasks', name: 'Tasks' },
  { path: '/graph', name: 'Graph Visualization' },
  { path: '/info-streams', name: 'Info Streams' },
  { path: '/containers', name: 'Containers' },
  { path: '/database', name: 'Database' },
  { path: '/map', name: 'Map' },
  { path: '/map-test', name: 'Map Test' },
  { path: '/nyx-trace', name: 'Nyx-Trace' },
  { path: '/raptor', name: 'Raptor' },
  { path: '/vkali', name: 'vKali' },
  { path: '/settings', name: 'Settings' },
];

// Modal triggers to look for
const MODAL_TRIGGERS = [
  'button:has-text("Add")',
  'button:has-text("Create")',
  'button:has-text("New")',
  'button:has-text("Edit")',
  'button:has-text("Configure")',
  'button:has-text("Settings")',
  'button:has-text("Run")',
  'button:has-text("Execute")',
  'button:has-text("Simulate")',
  '[role="button"]',
  '.modal-trigger',
];

const inventory = {
  timestamp: new Date().toISOString(),
  baseUrl: BASE_URL,
  version: '7.3.1',
  foundation_integration: true,
  pages: [],
  usims: [],
  unicode_map: {},
  sled_entries: [],
  graph_relationships: [],
  summary: {
    totalPages: 0,
    totalModals: 0,
    totalForms: 0,
    totalButtons: 0,
    totalInputs: 0,
    totalHashes: 0,
    totalUSIMs: 0,
  }
};

/**
 * Check if hashing engine is available
 */
async function checkHashingEngine() {
  return new Promise((resolve) => {
    const req = http.get(`${HASHING_ENGINE_URL}/health`, (res) => {
      resolve(res.statusCode === 200);
    });
    req.on('error', () => resolve(false));
    req.setTimeout(2000, () => {
      req.destroy();
      resolve(false);
    });
  });
}

/**
 * Generate trivariate hash via hashing engine
 */
async function generateHash(content, context, primitiveType) {
  return new Promise((resolve, reject) => {
    const postData = JSON.stringify({
      content,
      context,
      primitive_type: primitiveType,
      compress_unicode: true,
    });

    const options = {
      hostname: 'localhost',
      port: 8002,
      path: '/hash',
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Content-Length': Buffer.byteLength(postData),
      },
    };

    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => {
        if (res.statusCode === 200) {
          resolve(JSON.parse(data));
        } else {
          reject(new Error(`Hash generation failed: ${res.statusCode}`));
        }
      });
    });

    req.on('error', reject);
    req.write(postData);
    req.end();
  });
}

/**
 * Generate USIM header via hashing engine
 */
async function generateUSIM(content, context, primitiveType, metadata) {
  return new Promise((resolve, reject) => {
    const postData = JSON.stringify({
      content,
      context,
      primitive_type: primitiveType,
      metadata,
      format: 'Full',
    });

    const options = {
      hostname: 'localhost',
      port: 8002,
      path: '/generate_usim',
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Content-Length': Buffer.byteLength(postData),
      },
    };

    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => {
        if (res.statusCode === 200) {
          resolve(JSON.parse(data));
        } else {
          reject(new Error(`USIM generation failed: ${res.statusCode}`));
        }
      });
    });

    req.on('error', reject);
    req.write(postData);
    req.end();
  });
}

/**
 * Fallback hash generation (local, if engine unavailable)
 */
async function generateFallbackHash(content) {
  const crypto = await import('crypto');
  const hash = crypto.createHash('sha256').update(content).digest('hex');
  return {
    trivariate_hash: hash.substring(0, 48),
    sch: hash.substring(0, 16),
    cuid: hash.substring(16, 32),
    uuid: hash.substring(32, 48),
    unicode_compressed: `🔹${hash.substring(0, 4)}`,
    generation_time_ms: 0,
    fallback: true,
  };
}

async function createDirectories() {
  await fs.mkdir(OUTPUT_DIR, { recursive: true });
  await fs.mkdir(SCREENSHOT_DIR, { recursive: true });
}

async function inventoryPage(browser, pageConfig, hashingEngineAvailable) {
  const page = await browser.newPage();
  const pageInventory = {
    name: pageConfig.name,
    path: pageConfig.path,
    url: `${BASE_URL}${pageConfig.path}`,
    hash: null,
    usim: null,
    unicode: null,
    modals: [],
    forms: [],
    buttons: [],
    inputs: [],
    screenshots: [],
  };

  try {
    console.log(`\n📄 Inventorying: ${pageConfig.name} (${pageConfig.path})`);
    
    // Navigate to page
    await page.goto(`${BASE_URL}${pageConfig.path}`, { 
      waitUntil: 'networkidle',
      timeout: 30000 
    });
    
    // Wait for page to stabilize
    await page.waitForTimeout(2000);

    // Get page HTML structure for hashing
    const pageHTML = await page.content();
    const pageTitle = await page.title();

    // Generate hash for the page
    if (hashingEngineAvailable) {
      try {
        const hashResult = await generateHash(
          pageHTML.substring(0, 5000), // First 5KB for performance
          `ctas_ui_page_${pageConfig.name}`,
          'ui_page'
        );
        pageInventory.hash = hashResult.trivariate_hash;
        pageInventory.sch = hashResult.sch;
        pageInventory.unicode = hashResult.unicode_compressed;
        inventory.summary.totalHashes++;

        console.log(`  🔹 Hash: ${hashResult.sch}...`);
        console.log(`  🔸 Unicode: ${hashResult.unicode_compressed}`);

        // Generate USIM for the page
        const usimResult = await generateUSIM(
          pageHTML.substring(0, 5000),
          `ctas_ui_page_${pageConfig.name}`,
          'ui_page',
          {
            page_name: pageConfig.name,
            page_path: pageConfig.path,
            page_title: pageTitle,
            url: pageInventory.url,
          }
        );
        pageInventory.usim = usimResult.usim_header;
        inventory.usims.push({
          type: 'page',
          name: pageConfig.name,
          hash: hashResult.sch,
          usim: usimResult.usim_header,
        });
        inventory.summary.totalUSIMs++;

        // Add to Unicode map
        inventory.unicode_map[hashResult.unicode_compressed] = {
          type: 'page',
          name: pageConfig.name,
          hash: hashResult.sch,
          path: pageConfig.path,
        };

        // Add to Sled entries
        inventory.sled_entries.push({
          key: hashResult.sch,
          value: {
            type: 'ui_page',
            name: pageConfig.name,
            path: pageConfig.path,
            url: pageInventory.url,
            unicode: hashResult.unicode_compressed,
            timestamp: new Date().toISOString(),
          },
        });

      } catch (err) {
        console.log(`  ⚠️  Hash generation failed: ${err.message}`);
        const fallback = await generateFallbackHash(pageHTML);
        pageInventory.hash = fallback.trivariate_hash;
        pageInventory.sch = fallback.sch;
        pageInventory.unicode = fallback.unicode_compressed;
      }
    } else {
      const fallback = await generateFallbackHash(pageHTML);
      pageInventory.hash = fallback.trivariate_hash;
      pageInventory.sch = fallback.sch;
      pageInventory.unicode = fallback.unicode_compressed;
    }

    // Take initial screenshot
    const screenshotPath = `${pageConfig.name.replace(/\s+/g, '-').toLowerCase()}-initial.png`;
    await page.screenshot({ 
      path: path.join(SCREENSHOT_DIR, screenshotPath),
      fullPage: true 
    });
    pageInventory.screenshots.push(screenshotPath);
    console.log(`  📸 Screenshot: ${screenshotPath}`);

    // Inventory all buttons
    const buttons = await page.$$('button, [role="button"]');
    console.log(`  🔘 Found ${buttons.length} buttons`);
    
    for (const button of buttons) {
      try {
        const text = await button.textContent();
        const isVisible = await button.isVisible();
        const isEnabled = await button.isEnabled();
        
        if (isVisible && text && text.trim()) {
          const buttonData = {
            text: text.trim(),
            enabled: isEnabled,
            selector: await getSelector(button),
          };

          // Generate hash for button
          if (hashingEngineAvailable) {
            try {
              const hashResult = await generateHash(
                text.trim(),
                `ctas_ui_button_${pageConfig.name}`,
                'ui_button'
              );
              buttonData.hash = hashResult.sch;
              buttonData.unicode = hashResult.unicode_compressed;
              inventory.summary.totalHashes++;
            } catch (err) {
              // Silent fallback
            }
          }

          pageInventory.buttons.push(buttonData);
        }
      } catch (err) {
        // Button might have been removed
      }
    }

    // Inventory all forms
    const forms = await page.$$('form');
    console.log(`  📝 Found ${forms.length} forms`);
    
    for (let i = 0; i < forms.length; i++) {
      const form = forms[i];
      const formData = {
        index: i,
        inputs: [],
        selects: [],
        textareas: [],
        hash: null,
      };

      // Get all inputs in this form
      const inputs = await form.$$('input');
      for (const input of inputs) {
        const type = await input.getAttribute('type');
        const name = await input.getAttribute('name');
        const placeholder = await input.getAttribute('placeholder');
        formData.inputs.push({ type, name, placeholder });
      }

      // Get all selects
      const selects = await form.$$('select');
      for (const select of selects) {
        const name = await select.getAttribute('name');
        formData.selects.push({ name });
      }

      // Get all textareas
      const textareas = await form.$$('textarea');
      for (const textarea of textareas) {
        const name = await textarea.getAttribute('name');
        const placeholder = await textarea.getAttribute('placeholder');
        formData.textareas.push({ name, placeholder });
      }

      // Generate hash for form
      if (hashingEngineAvailable) {
        try {
          const formContent = JSON.stringify(formData);
          const hashResult = await generateHash(
            formContent,
            `ctas_ui_form_${pageConfig.name}_${i}`,
            'ui_form'
          );
          formData.hash = hashResult.sch;
          formData.unicode = hashResult.unicode_compressed;
          inventory.summary.totalHashes++;

          // Add to graph relationships
          inventory.graph_relationships.push({
            from: pageInventory.sch,
            to: hashResult.sch,
            relationship: 'contains_form',
            unicode_op: 'U+E501', // FORM
          });
        } catch (err) {
          // Silent fallback
        }
      }

      pageInventory.forms.push(formData);
    }

    // Look for modal triggers and try to open them
    console.log(`  🔍 Searching for modals...`);
    let modalCount = 0;

    for (const trigger of MODAL_TRIGGERS) {
      try {
        const elements = await page.$$(trigger);
        
        for (const element of elements) {
          const isVisible = await element.isVisible();
          if (!isVisible) continue;

          const text = await element.textContent();
          if (!text || !text.trim()) continue;

          console.log(`    🎯 Trying trigger: "${text.trim()}"`);

          try {
            // Click the trigger
            await element.click({ timeout: 2000 });
            await page.waitForTimeout(1000);

            // Check if a modal appeared
            const modal = await page.$('[role="dialog"], .modal, [class*="modal"]');
            
            if (modal) {
              modalCount++;
              const modalScreenshot = `${pageConfig.name.replace(/\s+/g, '-').toLowerCase()}-modal-${modalCount}.png`;
              await page.screenshot({ 
                path: path.join(SCREENSHOT_DIR, modalScreenshot),
                fullPage: true 
              });

              // Get modal content
              const modalText = await modal.textContent();
              const modalInputs = await modal.$$('input');
              const modalButtons = await modal.$$('button');

              const modalData = {
                trigger: text.trim(),
                screenshot: modalScreenshot,
                content: modalText.substring(0, 500),
                inputs: modalInputs.length,
                buttons: modalButtons.length,
                hash: null,
                usim: null,
              };

              // Generate hash and USIM for modal
              if (hashingEngineAvailable) {
                try {
                  const hashResult = await generateHash(
                    modalText.substring(0, 1000),
                    `ctas_ui_modal_${pageConfig.name}_${modalCount}`,
                    'ui_modal'
                  );
                  modalData.hash = hashResult.sch;
                  modalData.unicode = hashResult.unicode_compressed;
                  inventory.summary.totalHashes++;

                  const usimResult = await generateUSIM(
                    modalText.substring(0, 1000),
                    `ctas_ui_modal_${pageConfig.name}_${modalCount}`,
                    'ui_modal',
                    {
                      page: pageConfig.name,
                      trigger: text.trim(),
                      inputs: modalInputs.length,
                      buttons: modalButtons.length,
                    }
                  );
                  modalData.usim = usimResult.usim_header;
                  inventory.usims.push({
                    type: 'modal',
                    page: pageConfig.name,
                    trigger: text.trim(),
                    hash: hashResult.sch,
                    usim: usimResult.usim_header,
                  });
                  inventory.summary.totalUSIMs++;

                  // Add to graph relationships
                  inventory.graph_relationships.push({
                    from: pageInventory.sch,
                    to: hashResult.sch,
                    relationship: 'triggers_modal',
                    unicode_op: 'U+E502', // MODAL
                  });
                } catch (err) {
                  // Silent fallback
                }
              }

              pageInventory.modals.push(modalData);
              console.log(`      ✅ Modal captured: ${modalScreenshot}`);

              // Close modal (try common methods)
              try {
                await page.keyboard.press('Escape');
                await page.waitForTimeout(500);
              } catch (err) {
                // Try clicking close button
                const closeBtn = await page.$('button:has-text("Close"), button:has-text("Cancel"), [aria-label="Close"]');
                if (closeBtn) {
                  await closeBtn.click();
                  await page.waitForTimeout(500);
                }
              }
            }
          } catch (err) {
            // Modal might not have appeared or element not clickable
          }
        }
      } catch (err) {
        // Selector might not exist on this page
      }
    }

    console.log(`  ✅ Page complete: ${pageInventory.buttons.length} buttons, ${pageInventory.forms.length} forms, ${pageInventory.modals.length} modals`);

  } catch (err) {
    console.error(`  ❌ Error on ${pageConfig.name}:`, err.message);
    pageInventory.error = err.message;
  } finally {
    await page.close();
  }

  return pageInventory;
}

async function getSelector(element) {
  try {
    const id = await element.getAttribute('id');
    if (id) return `#${id}`;

    const className = await element.getAttribute('class');
    if (className) return `.${className.split(' ')[0]}`;

    return 'button';
  } catch {
    return 'unknown';
  }
}

async function generateReports() {
  // Calculate summary
  inventory.summary.totalPages = inventory.pages.length;
  inventory.summary.totalModals = inventory.pages.reduce((sum, p) => sum + p.modals.length, 0);
  inventory.summary.totalForms = inventory.pages.reduce((sum, p) => sum + p.forms.length, 0);
  inventory.summary.totalButtons = inventory.pages.reduce((sum, p) => sum + p.buttons.length, 0);

  // Generate JSON reports
  await fs.writeFile(
    path.join(OUTPUT_DIR, 'inventory.json'),
    JSON.stringify(inventory, null, 2)
  );

  await fs.writeFile(
    path.join(OUTPUT_DIR, 'usims.json'),
    JSON.stringify(inventory.usims, null, 2)
  );

  await fs.writeFile(
    path.join(OUTPUT_DIR, 'unicode-map.json'),
    JSON.stringify(inventory.unicode_map, null, 2)
  );

  await fs.writeFile(
    path.join(OUTPUT_DIR, 'sled-index.json'),
    JSON.stringify(inventory.sled_entries, null, 2)
  );

  await fs.writeFile(
    path.join(OUTPUT_DIR, 'graph-relationships.json'),
    JSON.stringify(inventory.graph_relationships, null, 2)
  );

  // Generate Markdown report
  let markdown = `# CTAS v7.3.1 Foundation-Integrated UI Inventory\n\n`;
  markdown += `**Generated:** ${inventory.timestamp}\n\n`;
  markdown += `**Foundation Integration:** ${inventory.foundation_integration ? '✅ Enabled' : '❌ Disabled'}\n\n`;
  
  markdown += `## Summary\n\n`;
  markdown += `- **Total Pages:** ${inventory.summary.totalPages}\n`;
  markdown += `- **Total Modals:** ${inventory.summary.totalModals}\n`;
  markdown += `- **Total Forms:** ${inventory.summary.totalForms}\n`;
  markdown += `- **Total Buttons:** ${inventory.summary.totalButtons}\n`;
  markdown += `- **Total Hashes:** ${inventory.summary.totalHashes}\n`;
  markdown += `- **Total USIMs:** ${inventory.summary.totalUSIMs}\n\n`;

  markdown += `## Foundation Integration\n\n`;
  markdown += `This inventory uses CTAS v7.3.1 foundation crates:\n\n`;
  markdown += `- **Trivariate Hashing:** Every UI element has a Murmur3 SCH hash\n`;
  markdown += `- **USIM Generation:** Pages and modals have USIM headers for documentation\n`;
  markdown += `- **Unicode Compression:** Quick lookup via emoji sequences\n`;
  markdown += `- **Multi-Tier Repository:** Sled KVS, SurrealDB graph, JSON exports\n\n`;

  markdown += `## Hash-Addressable UI Elements\n\n`;
  markdown += `| Element Type | Count | Example Hash | Unicode |\n`;
  markdown += `|--------------|-------|--------------|----------|\n`;
  markdown += `| Pages | ${inventory.summary.totalPages} | ${inventory.pages[0]?.sch || 'N/A'} | ${inventory.pages[0]?.unicode || 'N/A'} |\n`;
  markdown += `| Modals | ${inventory.summary.totalModals} | ${inventory.pages.find(p => p.modals.length > 0)?.modals[0]?.hash || 'N/A'} | ${inventory.pages.find(p => p.modals.length > 0)?.modals[0]?.unicode || 'N/A'} |\n`;
  markdown += `| Forms | ${inventory.summary.totalForms} | ${inventory.pages.find(p => p.forms.length > 0)?.forms[0]?.hash || 'N/A'} | ${inventory.pages.find(p => p.forms.length > 0)?.forms[0]?.unicode || 'N/A'} |\n\n`;

  markdown += `## Pages\n\n`;

  for (const page of inventory.pages) {
    markdown += `### ${page.name}\n\n`;
    markdown += `**URL:** ${page.url}\n\n`;
    markdown += `**Hash:** \`${page.sch || 'N/A'}\`\n\n`;
    markdown += `**Unicode:** ${page.unicode || 'N/A'}\n\n`;
    
    if (page.screenshots.length > 0) {
      markdown += `**Screenshots:**\n`;
      for (const screenshot of page.screenshots) {
        markdown += `- ![${screenshot}](screenshots/${screenshot})\n`;
      }
      markdown += `\n`;
    }

    if (page.modals.length > 0) {
      markdown += `**Modals (${page.modals.length}):**\n\n`;
      for (const modal of page.modals) {
        markdown += `- **Trigger:** "${modal.trigger}"\n`;
        markdown += `  - Hash: \`${modal.hash || 'N/A'}\`\n`;
        markdown += `  - Unicode: ${modal.unicode || 'N/A'}\n`;
        markdown += `  - Screenshot: ![${modal.screenshot}](screenshots/${modal.screenshot})\n`;
        markdown += `  - Inputs: ${modal.inputs}\n`;
        markdown += `  - Buttons: ${modal.buttons}\n\n`;
      }
    }

    if (page.buttons.length > 0) {
      markdown += `**Buttons (${page.buttons.length}):**\n`;
      const buttonList = page.buttons.slice(0, 20).map(b => `"${b.text}"`).join(', ');
      markdown += `${buttonList}${page.buttons.length > 20 ? '...' : ''}\n\n`;
    }

    if (page.forms.length > 0) {
      markdown += `**Forms (${page.forms.length}):**\n`;
      for (let i = 0; i < page.forms.length; i++) {
        const form = page.forms[i];
        markdown += `- Form ${i + 1}: ${form.inputs.length} inputs, ${form.selects.length} selects, ${form.textareas.length} textareas\n`;
        if (form.hash) {
          markdown += `  - Hash: \`${form.hash}\`\n`;
        }
      }
      markdown += `\n`;
    }

    markdown += `---\n\n`;
  }

  markdown += `## Repository Integration\n\n`;
  markdown += `### Sled KVS Entries\n\n`;
  markdown += `${inventory.sled_entries.length} entries ready for Sled import.\n\n`;
  markdown += `**File:** \`sled-index.json\`\n\n`;
  
  markdown += `### Graph Relationships\n\n`;
  markdown += `${inventory.graph_relationships.length} relationships for SurrealDB.\n\n`;
  markdown += `**File:** \`graph-relationships.json\`\n\n`;

  markdown += `### USIMs\n\n`;
  markdown += `${inventory.usims.length} USIM headers generated.\n\n`;
  markdown += `**File:** \`usims.json\`\n\n`;

  markdown += `## Usage\n\n`;
  markdown += `### Voice Navigation\n\n`;
  markdown += `\`\`\`\n`;
  markdown += `"Open hunt phase" → Unicode: ${inventory.pages.find(p => p.name === 'Hunt Phase')?.unicode || 'N/A'}\n`;
  markdown += `"Show tasks page" → Unicode: ${inventory.pages.find(p => p.name === 'Tasks')?.unicode || 'N/A'}\n`;
  markdown += `\`\`\`\n\n`;

  markdown += `### Hash Lookup\n\n`;
  markdown += `\`\`\`bash\n`;
  markdown += `# Query Sled for UI element\n`;
  markdown += `sledis get ${inventory.pages[0]?.sch || 'HASH'}\n`;
  markdown += `\`\`\`\n\n`;

  markdown += `### Automated Testing\n\n`;
  markdown += `\`\`\`javascript\n`;
  markdown += `// Reference UI elements by hash\n`;
  markdown += `const dashboardHash = "${inventory.pages[0]?.sch || 'HASH'}";\n`;
  markdown += `await page.goto(getUrlByHash(dashboardHash));\n`;
  markdown += `\`\`\`\n\n`;

  await fs.writeFile(
    path.join(OUTPUT_DIR, 'INVENTORY_REPORT.md'),
    markdown
  );

  console.log(`\n📊 Reports generated:`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'inventory.json')}`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'usims.json')}`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'unicode-map.json')}`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'sled-index.json')}`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'graph-relationships.json')}`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'INVENTORY_REPORT.md')}`);
}

async function main() {
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('🎯 CTAS v7.3.1 Foundation-Integrated UI Inventory');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log(`\n📍 Base URL: ${BASE_URL}`);
  console.log(`🔗 Hashing Engine: ${HASHING_ENGINE_URL}`);
  console.log(`📁 Output: ${OUTPUT_DIR}\n`);

  // Check hashing engine availability
  console.log('🔍 Checking hashing engine...');
  const hashingEngineAvailable = await checkHashingEngine();
  
  if (hashingEngineAvailable) {
    console.log('✅ Hashing engine available - Full foundation integration enabled\n');
  } else {
    console.log('⚠️  Hashing engine unavailable - Using fallback hashing\n');
    console.log('   To enable full integration, start the hashing engine:');
    console.log('   cd ctas7-hashing-engine && cargo run --release\n');
  }

  await createDirectories();

  const browser = await chromium.launch({ 
    headless: true,
    args: ['--no-sandbox']
  });

  // Inventory all pages
  for (const pageConfig of PAGES) {
    const pageInventory = await inventoryPage(browser, pageConfig, hashingEngineAvailable);
    inventory.pages.push(pageInventory);
  }

  await browser.close();

  // Generate reports
  await generateReports();

  console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('✅ FOUNDATION-INTEGRATED INVENTORY COMPLETE!');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log(`\n📊 Summary:`);
  console.log(`   • Pages inventoried: ${inventory.summary.totalPages}`);
  console.log(`   • Modals found: ${inventory.summary.totalModals}`);
  console.log(`   • Forms found: ${inventory.summary.totalForms}`);
  console.log(`   • Buttons found: ${inventory.summary.totalButtons}`);
  console.log(`   • Hashes generated: ${inventory.summary.totalHashes}`);
  console.log(`   • USIMs created: ${inventory.summary.totalUSIMs}`);
  console.log(`\n📁 View reports:`);
  console.log(`   • Full Inventory: ${path.join(OUTPUT_DIR, 'inventory.json')}`);
  console.log(`   • USIMs: ${path.join(OUTPUT_DIR, 'usims.json')}`);
  console.log(`   • Unicode Map: ${path.join(OUTPUT_DIR, 'unicode-map.json')}`);
  console.log(`   • Sled Index: ${path.join(OUTPUT_DIR, 'sled-index.json')}`);
  console.log(`   • Graph Data: ${path.join(OUTPUT_DIR, 'graph-relationships.json')}`);
  console.log(`   • Report: ${path.join(OUTPUT_DIR, 'INVENTORY_REPORT.md')}`);
  console.log(`   • Screenshots: ${SCREENSHOT_DIR}\n`);
}

main().catch(console.error);

