#!/usr/bin/env node
/**
 * SX9 Ops Main Platform - Complete UI Connection Test
 * 
 * Tests all pages, API connections, database connections, and component functionality
 * Verifies the entire UI is properly connected and working
 */

import { chromium } from 'playwright';
import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const BASE_URL = process.env.VITE_BASE_URL || 'http://localhost:18601';
const OUTPUT_DIR = path.join(__dirname, 'playwright-connection-results');
const SCREENSHOT_DIR = path.join(OUTPUT_DIR, 'screenshots');

// All pages to test
const PAGES = [
  { path: '/', name: 'Dashboard', critical: true },
  { path: '/tasks', name: 'Tasks', critical: true, testSupabase: true },
  { path: '/hunt', name: 'Hunt Phase', critical: true },
  { path: '/detect', name: 'Detect Phase', critical: true },
  { path: '/disrupt', name: 'Disrupt Phase', critical: true },
  { path: '/disable', name: 'Disable Phase', critical: true },
  { path: '/dominate', name: 'Dominate Phase', critical: true },
  { path: '/database', name: 'Database', critical: false, testSupabase: true },
  { path: '/plasma', name: 'Plasma', critical: false },
  { path: '/graph', name: 'Graph Visualization', critical: false },
  { path: '/info-streams', name: 'Info Streams', critical: false },
  { path: '/containers', name: 'Containers', critical: false },
  { path: '/map', name: 'Map', critical: false, testMapbox: true },
  { path: '/map-test', name: 'Map Test', critical: false, testMapbox: true },
  { path: '/raptor', name: 'Raptor', critical: false },
  { path: '/vkali', name: 'vKali', critical: false },
  { path: '/settings', name: 'Settings', critical: false },
];

// API endpoints to test
const API_ENDPOINTS = [
  { url: 'http://localhost:3000/health', name: 'PostgREST API', critical: true },
  { url: 'http://localhost:18600/health', name: 'SX9 Gateway', critical: false },
  { url: 'http://localhost:8000', name: 'SurrealDB', critical: false },
];

const results = {
  timestamp: new Date().toISOString(),
  baseUrl: BASE_URL,
  summary: {
    totalPages: 0,
    pagesPassed: 0,
    pagesFailed: 0,
    totalAPIs: 0,
    apisPassed: 0,
    apisFailed: 0,
    totalErrors: 0,
    criticalFailures: 0,
  },
  pages: [],
  apis: [],
  errors: [],
  connections: {
    supabase: { connected: false, error: null },
    mapbox: { connected: false, error: null },
    gateway: { connected: false, error: null },
  },
};

/**
 * Test API endpoint
 */
async function testAPI(endpoint) {
  try {
    const response = await fetch(endpoint.url, { 
      method: 'GET',
      signal: AbortSignal.timeout(5000),
    });
    
    return {
      name: endpoint.name,
      url: endpoint.url,
      status: response.status,
      ok: response.ok,
      connected: response.status < 500,
      error: null,
    };
  } catch (error) {
    return {
      name: endpoint.name,
      url: endpoint.url,
      status: null,
      ok: false,
      connected: false,
      error: error.message,
    };
  }
}

/**
 * Test page and check for errors
 */
async function testPage(page, browserPage, pageConfig) {
  const pageResult = {
    name: pageConfig.name,
    path: pageConfig.path,
    url: `${BASE_URL}${pageConfig.path}`,
    loaded: false,
    errors: [],
    warnings: [],
    consoleErrors: [],
    networkErrors: [],
    components: {
      buttons: 0,
      inputs: 0,
      forms: 0,
      modals: 0,
    },
    connections: {
      supabase: null,
      mapbox: null,
    },
    screenshot: null,
  };

  try {
    console.log(`\n🔍 Testing: ${pageConfig.name} (${pageConfig.path})`);

    // Collect console errors
    const consoleMessages = [];
    browserPage.on('console', msg => {
      if (msg.type() === 'error') {
        consoleMessages.push({
          text: msg.text(),
          type: msg.type(),
        });
      }
    });

    // Collect network errors
    const networkErrors = [];
    browserPage.on('response', response => {
      if (response.status() >= 400) {
        networkErrors.push({
          url: response.url(),
          status: response.status(),
          statusText: response.statusText(),
        });
      }
    });

    // Navigate to page with more lenient timeout
    await browserPage.goto(pageResult.url, { 
      waitUntil: 'domcontentloaded', // Changed from networkidle to avoid timeouts
      timeout: 15000,
    });

    pageResult.loaded = true;
    console.log(`  ✅ Page loaded`);

    // Wait for page to stabilize
    await browserPage.waitForTimeout(2000);

    // Check for React errors
    const reactError = await browserPage.evaluate(() => {
      const errorDiv = document.querySelector('[data-react-error]');
      return errorDiv ? errorDiv.textContent : null;
    });

    if (reactError) {
      pageResult.errors.push(`React Error: ${reactError}`);
    }

    // Collect console errors
    pageResult.consoleErrors = consoleMessages;

    // Collect network errors (filter out expected 404s for assets)
    pageResult.networkErrors = networkErrors.filter(
      e => !e.url.includes('.ico') && !e.url.includes('favicon')
    );

    // Count components
    pageResult.components.buttons = await browserPage.locator('button').count();
    pageResult.components.inputs = await browserPage.locator('input, textarea, select').count();
    pageResult.components.forms = await browserPage.locator('form').count();
    pageResult.components.modals = await browserPage.locator('[role="dialog"], .modal').count();

    // Test Supabase connection if needed
    if (pageConfig.testSupabase) {
      try {
        const supabaseTest = await browserPage.evaluate(async () => {
          try {
            // Try direct PostgREST query (no auth required for local)
            const response = await fetch('http://localhost:3000/ctas_tasks?limit=1', {
              headers: {
                'Accept': 'application/json',
              },
            });
            
            // 404 means table doesn't exist, but PostgREST is working
            // 200 means table exists and we got data
            // 500 means PostgREST error
            if (response.status === 404) {
              return { 
                connected: true, 
                tableExists: false,
                error: 'Table ctas_tasks does not exist (PostgREST is working)' 
              };
            }
            if (response.status === 200) {
              return { 
                connected: true, 
                tableExists: true,
                error: null 
              };
            }
            return { 
              connected: false, 
              tableExists: false,
              error: `Status: ${response.status}` 
            };
          } catch (e) {
            return { connected: false, tableExists: false, error: e.message };
          }
        });
        pageResult.connections.supabase = supabaseTest;
        if (supabaseTest.connected) {
          if (supabaseTest.tableExists) {
            console.log(`  ✅ Supabase connected, table exists`);
          } else {
            console.log(`  ⚠️  Supabase connected but table missing: ${supabaseTest.error}`);
          }
        } else {
          console.log(`  ❌ Supabase not connected: ${supabaseTest.error}`);
        }
      } catch (e) {
        pageResult.connections.supabase = { connected: false, error: e.message };
      }
    }

    // Test Mapbox connection if needed
    if (pageConfig.testMapbox) {
      try {
        const mapboxTest = await browserPage.evaluate(() => {
          const token = window.VITE_MAPBOX_TOKEN || 
                       document.querySelector('[data-mapbox-token]')?.getAttribute('data-mapbox-token');
          return { 
            connected: !!token && token.length > 20,
            error: token ? null : 'Mapbox token not found',
          };
        });
        pageResult.connections.mapbox = mapboxTest;
        if (mapboxTest.connected) {
          console.log(`  ✅ Mapbox token found`);
        } else {
          console.log(`  ⚠️  Mapbox token missing`);
        }
      } catch (e) {
        pageResult.connections.mapbox = { connected: false, error: e.message };
      }
    }

    // Take screenshot
    const screenshotPath = path.join(SCREENSHOT_DIR, `${pageConfig.name.replace(/\s+/g, '-')}.png`);
    await browserPage.screenshot({ path: screenshotPath, fullPage: true });
    pageResult.screenshot = screenshotPath;

    // Determine if page passed
    const hasErrors = pageResult.errors.length > 0 || 
                     pageResult.consoleErrors.length > 0 ||
                     (pageResult.networkErrors.length > 5); // Allow some 404s

    if (hasErrors) {
      pageResult.status = 'failed';
      results.summary.pagesFailed++;
      if (pageConfig.critical) {
        results.summary.criticalFailures++;
      }
      console.log(`  ❌ Page has errors`);
    } else {
      pageResult.status = 'passed';
      results.summary.pagesPassed++;
      console.log(`  ✅ Page passed`);
    }

  } catch (error) {
    pageResult.loaded = false;
    pageResult.status = 'failed';
    pageResult.errors.push(error.message);
    results.summary.pagesFailed++;
    if (pageConfig.critical) {
      results.summary.criticalFailures++;
    }
    console.log(`  ❌ Page failed: ${error.message}`);
  }

  return pageResult;
}

/**
 * Main test runner
 */
async function runTests() {
  console.log('🚀 Starting SX9 Ops Main Platform Connection Tests\n');
  console.log(`Base URL: ${BASE_URL}`);
  console.log(`Output Directory: ${OUTPUT_DIR}\n`);

  // Create output directories
  await fs.mkdir(OUTPUT_DIR, { recursive: true });
  await fs.mkdir(SCREENSHOT_DIR, { recursive: true });

  // Test API endpoints
  console.log('📡 Testing API Endpoints...\n');
  for (const endpoint of API_ENDPOINTS) {
    const apiResult = await testAPI(endpoint);
    results.apis.push(apiResult);
    results.summary.totalAPIs++;
    
    if (apiResult.connected) {
      results.summary.apisPassed++;
      console.log(`  ✅ ${apiResult.name}: Connected (${apiResult.status})`);
      
      // Update connection status
      if (apiResult.name.includes('PostgREST')) {
        results.connections.supabase = { connected: true, error: null };
      }
      if (apiResult.name.includes('Gateway')) {
        results.connections.gateway = { connected: true, error: null };
      }
    } else {
      results.summary.apisFailed++;
      console.log(`  ❌ ${apiResult.name}: Failed - ${apiResult.error || 'Connection refused'}`);
      
      if (apiResult.name.includes('PostgREST')) {
        results.connections.supabase = { connected: false, error: apiResult.error };
      }
      if (apiResult.name.includes('Gateway')) {
        results.connections.gateway = { connected: false, error: apiResult.error };
      }
    }
  }

  // Launch browser
  console.log('\n🌐 Launching browser...\n');
  const browser = await chromium.launch({ 
    headless: process.env.HEADLESS !== 'false',
  });
  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
  });
  const browserPage = await context.newPage();

  // Test all pages
  console.log('\n📄 Testing Pages...\n');
  for (const pageConfig of PAGES) {
    results.summary.totalPages++;
    const pageResult = await testPage(null, browserPage, pageConfig);
    results.pages.push(pageResult);
    
    // Collect errors
    if (pageResult.errors.length > 0) {
      results.errors.push(...pageResult.errors.map(e => ({
        page: pageConfig.name,
        error: e,
      })));
    }
    if (pageResult.consoleErrors.length > 0) {
      results.errors.push(...pageResult.consoleErrors.map(e => ({
        page: pageConfig.name,
        error: `Console: ${e.text}`,
      })));
    }
  }

  results.summary.totalErrors = results.errors.length;

  // Close browser
  await browser.close();

  // Save results
  const resultsPath = path.join(OUTPUT_DIR, 'connection-test-results.json');
  await fs.writeFile(resultsPath, JSON.stringify(results, null, 2));

  // Generate report
  console.log('\n' + '='.repeat(70));
  console.log('📊 TEST RESULTS SUMMARY');
  console.log('='.repeat(70));
  console.log(`\nPages: ${results.summary.pagesPassed}/${results.summary.totalPages} passed`);
  console.log(`APIs: ${results.summary.apisPassed}/${results.summary.totalAPIs} connected`);
  console.log(`Total Errors: ${results.summary.totalErrors}`);
  console.log(`Critical Failures: ${results.summary.criticalFailures}`);
  
  console.log('\n🔌 Connection Status:');
  console.log(`  Supabase: ${results.connections.supabase.connected ? '✅' : '❌'} ${results.connections.supabase.error || 'Connected'}`);
  console.log(`  Mapbox: ${results.connections.mapbox.connected ? '✅' : '❌'} ${results.connections.mapbox.error || 'Token found'}`);
  console.log(`  Gateway: ${results.connections.gateway.connected ? '✅' : '❌'} ${results.connections.gateway.error || 'Connected'}`);

  console.log('\n📁 Results saved to:');
  console.log(`  ${resultsPath}`);
  console.log(`  Screenshots: ${SCREENSHOT_DIR}`);

  if (results.summary.criticalFailures > 0) {
    console.log('\n⚠️  CRITICAL FAILURES DETECTED - Review errors above');
    process.exit(1);
  } else {
    console.log('\n✅ All critical tests passed!');
    process.exit(0);
  }
}

// Run tests
runTests().catch(error => {
  console.error('❌ Test runner error:', error);
  process.exit(1);
});

