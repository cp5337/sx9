import { chromium } from "playwright";
import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import { dirname } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const BASE_URL = "http://localhost:15174";
const OUTPUT_DIR = path.join(__dirname, "capability-audit");

// Pages to audit
const PAGES = [
  { path: "/", name: "Dashboard" },
  { path: "/hunt", name: "Hunt Phase" },
  { path: "/detect", name: "Detect Phase" },
  { path: "/disrupt", name: "Disrupt Phase" },
  { path: "/disable", name: "Disable Phase" },
  { path: "/dominate", name: "Dominate Phase" },
  { path: "/tasks", name: "Tasks" },
  { path: "/graph", name: "Graph Visualization" },
  { path: "/info-streams", name: "Info Streams" },
  { path: "/containers", name: "Containers" },
  { path: "/database", name: "Database" },
  { path: "/map", name: "Map" },
  { path: "/settings", name: "Settings" },
];

async function audit() {
  console.log("🚀 Starting UI Capability Audit (No Hashing)...");
  await fs.mkdir(OUTPUT_DIR, { recursive: true });

  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const inventory = [];

  for (const pageConfig of PAGES) {
    const page = await context.newPage();
    try {
      console.log(`\n📄 Auditing: ${pageConfig.name} (${pageConfig.path})`);
      await page.goto(`${BASE_URL}${pageConfig.path}`, {
        waitUntil: "domcontentloaded",
        timeout: 15000,
      });

      // 1. Identify Buttons (Actions)
      const buttons = await page.$$eval("button", els =>
        els.map(e => e.innerText.trim()).filter(t => t.length > 0)
      );

      // 2. Identify Links (Navigation)
      const links = await page.$$eval("a", els =>
        els
          .map(e => ({ text: e.innerText.trim(), href: e.getAttribute("href") }))
          .filter(l => l.text.length > 0)
      );

      // 3. Identify Inputs (Data Entry)
      const inputs = await page.$$eval("input", els =>
        els.map(e => ({ type: e.type, placeholder: e.placeholder }))
      );

      inventory.push({
        page: pageConfig.name,
        path: pageConfig.path,
        capabilities: {
          actions: buttons,
          navigation: links,
          inputs: inputs,
        },
      });

      console.log(`   ✅ Found ${buttons.length} actions, ${inputs.length} inputs.`);
    } catch (e) {
      console.error(`   ❌ Failed to audit ${pageConfig.name}: ${e.message}`);
    } finally {
      await page.close();
    }
  }

  await browser.close();

  // Generate Report
  const reportPath = path.join(OUTPUT_DIR, "UI_CAPABILITY_MANIFEST.json");
  await fs.writeFile(reportPath, JSON.stringify(inventory, null, 2));

  // Generate Markdown Summary
  let md = "# UI Capability Manifest\n\n";
  inventory.forEach(item => {
    md += `## ${item.page}\n`;
    if (item.capabilities.actions.length > 0) {
      md += `### Actions\n`;
      item.capabilities.actions.forEach(a => (md += `- [ ] ${a}\n`));
    }
    if (item.capabilities.inputs.length > 0) {
      md += `### Inputs\n`;
      item.capabilities.inputs.forEach(i => (md += `- ${i.type}: ${i.placeholder}\n`));
    }
    md += "\n";
  });

  const mdPath = path.join(OUTPUT_DIR, "UI_CAPABILITY_MANIFEST.md");
  await fs.writeFile(mdPath, md);

  console.log(`\n✅ Audit Complete. Reports saved to ${OUTPUT_DIR}`);
}

audit();
