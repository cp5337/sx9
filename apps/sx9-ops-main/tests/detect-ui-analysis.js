/**
 * Playwright UI Analysis for /detect page
 * Examines the HD4 Detect phase interface
 */

import { chromium } from 'playwright';
import fs from 'fs';

async function analyzeDetectUI() {
  console.log('Starting Playwright UI Analysis for /detect...\n');

  const browser = await chromium.launch({
    headless: process.env.HEADLESS !== 'false'
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 }
  });

  const page = await context.newPage();

  const results = {
    timestamp: new Date().toISOString(),
    url: 'http://localhost:18601/detect',
    elements: {},
    tabs: [],
    agents: [],
    errors: []
  };

  try {
    console.log('Navigating to http://localhost:18601/detect...');
    await page.goto('http://localhost:18601/detect', {
      waitUntil: 'domcontentloaded',
      timeout: 60000
    });

    // Give app time to render
    await page.waitForTimeout(3000);

    // Wait for initial render
    await page.waitForTimeout(2000);

    // Take initial screenshot
    await page.screenshot({
      path: 'test-results/detect-initial.png',
      fullPage: true
    });
    console.log('Screenshot saved: test-results/detect-initial.png');

    // Analyze main tabs
    console.log('\n=== Analyzing Main Tabs ===');
    const tabButtons = await page.$$('button');
    for (const btn of tabButtons) {
      const text = await btn.textContent();
      const isVisible = await btn.isVisible();
      if (text && isVisible && text.trim().length > 0 && text.trim().length < 30) {
        results.tabs.push(text.trim());
      }
    }
    console.log('Found tabs:', results.tabs.slice(0, 10));

    // Check for HD4PhaseContent specific elements
    console.log('\n=== Checking HD4 Phase Elements ===');

    // Look for Overview tab content
    const overviewTab = await page.$('button:has-text("Overview")');
    if (overviewTab) {
      console.log('Found Overview tab');
      results.elements.overviewTab = true;
    }

    // Look for Kali Tools tab
    const kaliTab = await page.$('button:has-text("Kali Tools")');
    if (kaliTab) {
      console.log('Found Kali Tools tab');
      results.elements.kaliToolsTab = true;

      // Click to analyze Kali Tools section
      await kaliTab.click();
      await page.waitForTimeout(1500);
      await page.screenshot({
        path: 'test-results/detect-kali-tools.png',
        fullPage: true
      });
      console.log('Screenshot saved: test-results/detect-kali-tools.png');
    }

    // Look for Playbooks tab
    const playbooksTab = await page.$('button:has-text("Playbooks")');
    if (playbooksTab) {
      console.log('Found Playbooks tab');
      results.elements.playbooksTab = true;

      await playbooksTab.click();
      await page.waitForTimeout(1500);
      await page.screenshot({
        path: 'test-results/detect-playbooks.png',
        fullPage: true
      });
      console.log('Screenshot saved: test-results/detect-playbooks.png');
    }

    // Look for Red Team tab
    const redTeamTab = await page.$('button:has-text("Red Team")');
    if (redTeamTab) {
      console.log('Found Red Team tab');
      results.elements.redTeamTab = true;

      await redTeamTab.click();
      await page.waitForTimeout(1500);
      await page.screenshot({
        path: 'test-results/detect-redteam.png',
        fullPage: true
      });
      console.log('Screenshot saved: test-results/detect-redteam.png');
    }

    // Go back to Overview for agent analysis
    if (overviewTab) {
      await overviewTab.click();
      await page.waitForTimeout(1000);
    }

    // Check for agent tabs
    console.log('\n=== Checking Agent Tabs ===');
    const agentNames = ['Natasha', 'Marcus', 'Elena', 'Cove', 'Kali'];
    for (const agent of agentNames) {
      const agentBtn = await page.$(`button:has-text("${agent}")`);
      if (agentBtn) {
        console.log(`Found agent tab: ${agent}`);
        results.agents.push(agent);
      }
    }

    // Check for CLI toggle
    const cliToggle = await page.$('button:has-text("Show CLI")');
    if (cliToggle) {
      console.log('Found CLI toggle');
      results.elements.cliToggle = true;

      await cliToggle.click();
      await page.waitForTimeout(1500);
      await page.screenshot({
        path: 'test-results/detect-with-cli.png',
        fullPage: true
      });
      console.log('Screenshot saved: test-results/detect-with-cli.png');
    }

    // Check for map/graph visualization
    const mapSection = await page.$('#hd4-overview-container');
    if (mapSection) {
      console.log('Found HD4 overview container');
      results.elements.overviewContainer = true;
    }

    // Check for visualization manager
    const visManager = await page.$('text=Visualization');
    if (visManager) {
      console.log('Found Visualization Manager');
      results.elements.visualizationManager = true;
    }

    // Final summary screenshot
    await page.screenshot({
      path: 'test-results/detect-final.png',
      fullPage: true
    });

    console.log('\n=== UI Analysis Complete ===');
    console.log('Elements found:', Object.keys(results.elements).length);
    console.log('Tabs found:', results.tabs.length);
    console.log('Agents found:', results.agents.length);

  } catch (error) {
    console.error('Error during analysis:', error.message);
    results.errors.push(error.message);

    await page.screenshot({
      path: 'test-results/detect-error.png',
      fullPage: true
    });
  }

  // Save results
  fs.mkdirSync('test-results', { recursive: true });
  fs.writeFileSync(
    'test-results/detect-analysis.json',
    JSON.stringify(results, null, 2)
  );
  console.log('\nResults saved to test-results/detect-analysis.json');

  await browser.close();
  return results;
}

analyzeDetectUI()
  .then(results => {
    console.log('\n=== SUMMARY ===');
    console.log('URL:', results.url);
    console.log('Main Tabs:', results.tabs.slice(0, 8).join(', '));
    console.log('Agent Tabs:', results.agents.join(', '));
    console.log('Key Elements:', Object.keys(results.elements).join(', '));
    if (results.errors.length > 0) {
      console.log('Errors:', results.errors.join(', '));
    }
  })
  .catch(err => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
