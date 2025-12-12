/**
 * Playwright Full UI Analysis
 * Examines all HD4 Kill Chain phases and sidebar pages
 */

import { chromium } from 'playwright';
import fs from 'fs';

const BASE_URL = 'http://localhost:18601';

// Pages to analyze from sidebar
const PAGES = {
  // HD4 Kill Chain phases
  killChain: [
    { name: 'Hunt', path: '/hunt' },
    { name: 'Detect', path: '/detect' },
    { name: 'Disrupt', path: '/disrupt' },
    { name: 'Disable', path: '/disable' },
    { name: 'Dominate', path: '/dominate' },
  ],
  // Other sidebar pages
  sidebar: [
    { name: 'Dashboard', path: '/' },
    { name: 'Tasks', path: '/tasks' },
    { name: 'Streams', path: '/streams' },
    { name: 'Plasma', path: '/plasma' },
    { name: 'Raptor', path: '/raptor' },
    { name: 'vKali', path: '/vkali' },
    { name: 'Containers', path: '/containers' },
    { name: 'Exploit DB', path: '/exploit-db' },
    { name: 'AI CLI', path: '/ai-cli' },
    { name: 'Firefly IAC', path: '/firefly' },
    { name: 'DVM', path: '/dvm' },
    { name: 'Databases', path: '/databases' },
    { name: 'Scripts', path: '/scripts' },
    { name: 'Gallery', path: '/gallery' },
    { name: 'Components', path: '/components' },
    { name: 'Settings', path: '/settings' },
  ]
};

async function analyzePage(page, pageInfo, results) {
  const { name, path } = pageInfo;
  console.log(`\n=== Analyzing ${name} (${path}) ===`);

  const pageResult = {
    name,
    path,
    url: `${BASE_URL}${path}`,
    timestamp: new Date().toISOString(),
    elements: {},
    tabs: [],
    buttons: [],
    sections: [],
    errors: []
  };

  try {
    await page.goto(`${BASE_URL}${path}`, {
      waitUntil: 'domcontentloaded',
      timeout: 30000
    });

    // Wait for render
    await page.waitForTimeout(2000);

    // Take screenshot
    const screenshotPath = `test-results/pages/${name.toLowerCase().replace(/\s+/g, '-')}.png`;
    await page.screenshot({ path: screenshotPath, fullPage: true });
    pageResult.screenshot = screenshotPath;
    console.log(`  Screenshot: ${screenshotPath}`);

    // Analyze page title/header
    const h1 = await page.$('h1');
    if (h1) {
      pageResult.title = await h1.textContent();
    }

    // Find all visible buttons
    const buttons = await page.$$('button');
    for (const btn of buttons) {
      const text = await btn.textContent();
      const isVisible = await btn.isVisible();
      if (text && isVisible && text.trim().length > 0 && text.trim().length < 40) {
        pageResult.buttons.push(text.trim());
      }
    }
    pageResult.buttonCount = pageResult.buttons.length;
    console.log(`  Buttons found: ${pageResult.buttonCount}`);

    // Find tabs (common pattern in HD4 pages)
    const tabPatterns = ['Overview', 'Kali Tools', 'Playbooks', 'Red Team', 'Phase Mapping', 'Tasks'];
    for (const tabName of tabPatterns) {
      const tab = await page.$(`button:has-text("${tabName}")`);
      if (tab && await tab.isVisible()) {
        pageResult.tabs.push(tabName);
      }
    }
    console.log(`  Tabs: ${pageResult.tabs.join(', ') || 'none'}`);

    // Check for agent tabs
    const agents = ['Natasha', 'Marcus', 'Elena', 'Cove', 'Kali'];
    pageResult.agents = [];
    for (const agent of agents) {
      const agentBtn = await page.$(`button:has-text("${agent}")`);
      if (agentBtn && await agentBtn.isVisible()) {
        pageResult.agents.push(agent);
      }
    }
    if (pageResult.agents.length > 0) {
      console.log(`  Agents: ${pageResult.agents.join(', ')}`);
    }

    // Check for map/visualization
    pageResult.hasMap = !!(await page.$('[class*="mapbox"], [class*="leaflet"], #map, .map-container'));
    pageResult.hasGraph = !!(await page.$('[class*="graph"], [class*="chart"], canvas, svg'));

    // Check for CLI/Terminal
    pageResult.hasCLI = !!(await page.$('button:has-text("Show CLI"), button:has-text("Hide CLI"), [class*="terminal"]'));

    // Check for data tables
    pageResult.hasTable = !!(await page.$('table, [class*="table"], [class*="grid"]'));

    // Check for forms
    const inputs = await page.$$('input, textarea, select');
    pageResult.inputCount = inputs.length;

    // Look for specific UI patterns
    pageResult.patterns = {
      hasToolCards: !!(await page.$('[class*="tool-card"], [class*="ToolCard"]')),
      hasPlaybooks: !!(await page.$('button:has-text("Playbooks")')),
      hasFilters: !!(await page.$('button:has-text("Filters")')),
      hasLayers: !!(await page.$('[class*="layer"], button:has-text("Layers")')),
      hasSidebar: !!(await page.$('[class*="sidebar"], nav')),
    };

    console.log(`  Map: ${pageResult.hasMap}, Graph: ${pageResult.hasGraph}, CLI: ${pageResult.hasCLI}`);

  } catch (error) {
    console.error(`  Error: ${error.message}`);
    pageResult.errors.push(error.message);

    await page.screenshot({
      path: `test-results/pages/${name.toLowerCase().replace(/\s+/g, '-')}-error.png`,
      fullPage: true
    });
  }

  results.pages.push(pageResult);
  return pageResult;
}

async function analyzeFullUI() {
  console.log('='.repeat(60));
  console.log('Full UI Pattern Analysis');
  console.log('='.repeat(60));

  const browser = await chromium.launch({
    headless: process.env.HEADLESS !== 'false'
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 }
  });

  const page = await context.newPage();

  const results = {
    timestamp: new Date().toISOString(),
    baseUrl: BASE_URL,
    pages: [],
    patterns: {},
    summary: {}
  };

  // Create output directory
  fs.mkdirSync('test-results/pages', { recursive: true });

  // Analyze Kill Chain pages
  console.log('\n' + '='.repeat(60));
  console.log('HD4 KILL CHAIN PHASES');
  console.log('='.repeat(60));

  for (const pageInfo of PAGES.killChain) {
    await analyzePage(page, pageInfo, results);
  }

  // Analyze other sidebar pages
  console.log('\n' + '='.repeat(60));
  console.log('SIDEBAR PAGES');
  console.log('='.repeat(60));

  for (const pageInfo of PAGES.sidebar) {
    await analyzePage(page, pageInfo, results);
  }

  // Generate pattern summary
  console.log('\n' + '='.repeat(60));
  console.log('PATTERN ANALYSIS');
  console.log('='.repeat(60));

  // Analyze common patterns across Kill Chain pages
  const killChainPages = results.pages.filter(p =>
    PAGES.killChain.some(kc => kc.name === p.name)
  );

  results.patterns.killChain = {
    commonTabs: findCommonElements(killChainPages.map(p => p.tabs)),
    commonAgents: findCommonElements(killChainPages.map(p => p.agents)),
    allHaveMap: killChainPages.every(p => p.hasMap),
    allHaveCLI: killChainPages.every(p => p.hasCLI),
    allHaveGraph: killChainPages.every(p => p.hasGraph),
  };

  console.log('\nKill Chain Common Patterns:');
  console.log(`  Common Tabs: ${results.patterns.killChain.commonTabs.join(', ')}`);
  console.log(`  Common Agents: ${results.patterns.killChain.commonAgents.join(', ')}`);
  console.log(`  All have Map: ${results.patterns.killChain.allHaveMap}`);
  console.log(`  All have CLI: ${results.patterns.killChain.allHaveCLI}`);

  // Summary statistics
  results.summary = {
    totalPages: results.pages.length,
    pagesWithMap: results.pages.filter(p => p.hasMap).length,
    pagesWithCLI: results.pages.filter(p => p.hasCLI).length,
    pagesWithAgents: results.pages.filter(p => p.agents && p.agents.length > 0).length,
    pagesWithTabs: results.pages.filter(p => p.tabs && p.tabs.length > 0).length,
    pagesWithErrors: results.pages.filter(p => p.errors.length > 0).length,
    averageButtons: Math.round(
      results.pages.reduce((sum, p) => sum + (p.buttonCount || 0), 0) / results.pages.length
    ),
  };

  console.log('\nSummary:');
  console.log(`  Total pages analyzed: ${results.summary.totalPages}`);
  console.log(`  Pages with Map: ${results.summary.pagesWithMap}`);
  console.log(`  Pages with CLI: ${results.summary.pagesWithCLI}`);
  console.log(`  Pages with Agents: ${results.summary.pagesWithAgents}`);
  console.log(`  Pages with Errors: ${results.summary.pagesWithErrors}`);

  // Save results
  fs.writeFileSync(
    'test-results/full-ui-analysis.json',
    JSON.stringify(results, null, 2)
  );
  console.log('\nResults saved to test-results/full-ui-analysis.json');

  await browser.close();
  return results;
}

function findCommonElements(arrays) {
  if (arrays.length === 0) return [];
  const filtered = arrays.filter(a => a && a.length > 0);
  if (filtered.length === 0) return [];

  return filtered[0].filter(item =>
    filtered.every(arr => arr.includes(item))
  );
}

// Run analysis
analyzeFullUI()
  .then(results => {
    console.log('\n' + '='.repeat(60));
    console.log('ANALYSIS COMPLETE');
    console.log('='.repeat(60));

    // Print page-by-page summary
    console.log('\nPage Summary:');
    for (const p of results.pages) {
      const status = p.errors.length > 0 ? '❌' : '✓';
      const features = [
        p.hasMap ? 'Map' : '',
        p.hasCLI ? 'CLI' : '',
        p.agents?.length > 0 ? 'Agents' : '',
        p.tabs?.length > 0 ? 'Tabs' : '',
      ].filter(Boolean).join(', ');

      console.log(`  ${status} ${p.name}: ${features || 'basic'}`);
    }
  })
  .catch(err => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
