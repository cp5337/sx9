import { chromium } from 'playwright';

(async () => {
  const browser = await chromium.launch({ headless: false });
  const page = await browser.newPage();
  
  console.log('🚀 Opening HD4 Hunt page...');
  await page.goto('http://localhost:15174/hunt');
  
  // Wait for page to load
  await page.waitForTimeout(3000);
  
  console.log('📸 Taking screenshot of initial state...');
  await page.screenshot({ path: 'hd4-hunt-initial.png', fullPage: true });
  
  // Get viewport dimensions
  const viewport = page.viewportSize();
  console.log(`📐 Viewport: ${viewport.width}x${viewport.height}`);
  
  // Check for black space by analyzing layout
  console.log('\n🔍 Analyzing layout...');
  
  // Check tab bar
  const tabBar = await page.locator('div.flex.border-b.border-gray-700').first();
  if (await tabBar.isVisible()) {
    const tabBarBox = await tabBar.boundingBox();
    console.log(`📊 Tab Bar: x=${tabBarBox.x}, y=${tabBarBox.y}, width=${tabBarBox.width}, height=${tabBarBox.height}`);
  }
  
  // Check map container
  const mapContainer = await page.locator('[id^="enhanced-map-container"]').first();
  if (await mapContainer.isVisible()) {
    const mapBox = await mapContainer.boundingBox();
    console.log(`🗺️  Map Container: x=${mapBox.x}, y=${mapBox.y}, width=${mapBox.width}, height=${mapBox.height}`);
  }
  
  // Check layer controls
  const layerControls = await page.locator('div.absolute.top-2.right-2').first();
  if (await layerControls.isVisible()) {
    const layerBox = await layerControls.boundingBox();
    console.log(`🎛️  Layer Controls: x=${layerBox.x}, y=${layerBox.y}, width=${layerBox.width}, height=${layerBox.height}`);
  }
  
  // Check zoom controls
  const zoomControls = await page.locator('div.absolute.bottom-2.right-2').first();
  if (await zoomControls.isVisible()) {
    const zoomBox = await zoomControls.boundingBox();
    console.log(`🔍 Zoom Controls: x=${zoomBox.x}, y=${zoomBox.y}, width=${zoomBox.width}, height=${zoomBox.height}`);
  }
  
  // Check chevron resize handle
  const chevronHandle = await page.locator('div.cursor-ns-resize').first();
  if (await chevronHandle.isVisible()) {
    const chevronBox = await chevronHandle.boundingBox();
    console.log(`🔼🔽 Chevron Handle: x=${chevronBox.x}, y=${chevronBox.y}, width=${chevronBox.width}, height=${chevronBox.height}`);
  }
  
  // Check task panel
  const taskPanel = await page.locator('div.bg-gray-800.rounded-lg.shadow-lg').first();
  if (await taskPanel.isVisible()) {
    const taskBox = await taskPanel.boundingBox();
    console.log(`📋 Task Panel: x=${taskBox.x}, y=${taskBox.y}, width=${taskBox.width}, height=${taskBox.height}`);
  }
  
  // Check for black space
  console.log('\n⚠️  Checking for black space...');
  const body = await page.locator('body').first();
  const bodyBox = await body.boundingBox();
  console.log(`📄 Body: x=${bodyBox.x}, y=${bodyBox.y}, width=${bodyBox.width}, height=${bodyBox.height}`);
  
  // Test resizing
  console.log('\n🖱️  Testing chevron resize...');
  if (await chevronHandle.isVisible()) {
    const chevronBox = await chevronHandle.boundingBox();
    const startY = chevronBox.y + chevronBox.height / 2;
    
    // Drag down 100px
    await page.mouse.move(chevronBox.x + chevronBox.width / 2, startY);
    await page.mouse.down();
    await page.mouse.move(chevronBox.x + chevronBox.width / 2, startY + 100);
    await page.mouse.up();
    
    await page.waitForTimeout(1000);
    console.log('📸 Taking screenshot after resize...');
    await page.screenshot({ path: 'hd4-hunt-after-resize.png', fullPage: true });
  }
  
  // Check if controls are accessible
  console.log('\n🎯 Testing control accessibility...');
  
  // Try clicking layer controls
  try {
    await layerControls.click();
    console.log('✅ Layer controls clickable');
  } catch (e) {
    console.log('❌ Layer controls NOT clickable:', e.message);
  }
  
  // Try clicking zoom controls
  try {
    const zoomInBtn = await page.locator('button[title="Zoom In"]').first();
    if (await zoomInBtn.isVisible()) {
      await zoomInBtn.click();
      console.log('✅ Zoom controls clickable');
    }
  } catch (e) {
    console.log('❌ Zoom controls NOT clickable:', e.message);
  }
  
  console.log('\n✅ Analysis complete! Check screenshots:');
  console.log('   - hd4-hunt-initial.png');
  console.log('   - hd4-hunt-after-resize.png');
  
  // Keep browser open for manual inspection
  console.log('\n👀 Browser will stay open for 30 seconds for manual inspection...');
  await page.waitForTimeout(30000);
  
  await browser.close();
})();

