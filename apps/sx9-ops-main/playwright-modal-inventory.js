#!/usr/bin/env node
/**
 * CTAS v7.3.1 Modal Inventory System
 * Executable Document: Crawls entire UI and catalogs all modals, forms, and interactions
 * Uses Playwright to create a comprehensive UI map
 */

const { chromium } = require('playwright');
const fs = require('fs').promises;
const path = require('path');

const BASE_URL = 'http://localhost:15174';
const OUTPUT_DIR = path.join(__dirname, 'modal-inventory');
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
  pages: [],
  modals: [],
  forms: [],
  interactions: [],
  summary: {
    totalPages: 0,
    totalModals: 0,
    totalForms: 0,
    totalButtons: 0,
    totalInputs: 0,
  }
};

async function createDirectories() {
  await fs.mkdir(OUTPUT_DIR, { recursive: true });
  await fs.mkdir(SCREENSHOT_DIR, { recursive: true });
}

async function inventoryPage(browser, pageConfig) {
  const page = await browser.newPage();
  const pageInventory = {
    name: pageConfig.name,
    path: pageConfig.path,
    url: `${BASE_URL}${pageConfig.path}`,
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
          pageInventory.buttons.push({
            text: text.trim(),
            enabled: isEnabled,
            selector: await getSelector(button),
          });
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
              };

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

async function generateReport() {
  // Calculate summary
  inventory.summary.totalPages = inventory.pages.length;
  inventory.summary.totalModals = inventory.pages.reduce((sum, p) => sum + p.modals.length, 0);
  inventory.summary.totalForms = inventory.pages.reduce((sum, p) => sum + p.forms.length, 0);
  inventory.summary.totalButtons = inventory.pages.reduce((sum, p) => sum + p.buttons.length, 0);

  // Generate JSON report
  await fs.writeFile(
    path.join(OUTPUT_DIR, 'inventory.json'),
    JSON.stringify(inventory, null, 2)
  );

  // Generate Markdown report
  let markdown = `# CTAS v7.3.1 Modal Inventory Report\n\n`;
  markdown += `**Generated:** ${inventory.timestamp}\n\n`;
  markdown += `## Summary\n\n`;
  markdown += `- **Total Pages:** ${inventory.summary.totalPages}\n`;
  markdown += `- **Total Modals:** ${inventory.summary.totalModals}\n`;
  markdown += `- **Total Forms:** ${inventory.summary.totalForms}\n`;
  markdown += `- **Total Buttons:** ${inventory.summary.totalButtons}\n\n`;

  markdown += `## Pages\n\n`;

  for (const page of inventory.pages) {
    markdown += `### ${page.name}\n\n`;
    markdown += `**URL:** ${page.url}\n\n`;
    
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
      }
      markdown += `\n`;
    }

    markdown += `---\n\n`;
  }

  await fs.writeFile(
    path.join(OUTPUT_DIR, 'INVENTORY_REPORT.md'),
    markdown
  );

  console.log(`\n📊 Reports generated:`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'inventory.json')}`);
  console.log(`   - ${path.join(OUTPUT_DIR, 'INVENTORY_REPORT.md')}`);
}

async function main() {
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('🎯 CTAS v7.3.1 Modal Inventory System');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log(`\n📍 Base URL: ${BASE_URL}`);
  console.log(`📁 Output: ${OUTPUT_DIR}\n`);

  await createDirectories();

  const browser = await chromium.launch({ 
    headless: true,
    args: ['--no-sandbox']
  });

  // Inventory all pages
  for (const pageConfig of PAGES) {
    const pageInventory = await inventoryPage(browser, pageConfig);
    inventory.pages.push(pageInventory);
  }

  await browser.close();

  // Generate reports
  await generateReport();

  console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('✅ INVENTORY COMPLETE!');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log(`\n📊 Summary:`);
  console.log(`   • Pages inventoried: ${inventory.summary.totalPages}`);
  console.log(`   • Modals found: ${inventory.summary.totalModals}`);
  console.log(`   • Forms found: ${inventory.summary.totalForms}`);
  console.log(`   • Buttons found: ${inventory.summary.totalButtons}`);
  console.log(`\n📁 View reports:`);
  console.log(`   • JSON: ${path.join(OUTPUT_DIR, 'inventory.json')}`);
  console.log(`   • Markdown: ${path.join(OUTPUT_DIR, 'INVENTORY_REPORT.md')}`);
  console.log(`   • Screenshots: ${SCREENSHOT_DIR}\n`);
}

main().catch(console.error);

