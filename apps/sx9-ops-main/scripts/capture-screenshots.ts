import puppeteer from 'puppeteer';
import fs from 'fs/promises';
// import path from 'path';
import yaml from 'js-yaml';

interface PageInfo {
  title: string;
  path: string;
  purpose: string;
  navigation: string[];
  controls: string[];
  dependencies: string[];
  sources: string[];
  nextSteps: string[];
}

async function captureScreenshots() {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  
  // Load page configurations
  const pagesConfig = yaml.load(await fs.readFile('scripts/pages-config.yml', 'utf8')) as PageInfo[];
  
  // Clear old screenshots
  await fs.rm('docs/public/images', { recursive: true, force: true });
  await fs.mkdir('docs/public/images', { recursive: true });
  
  for (const pageInfo of pagesConfig) {
    // Capture screenshot
    await page.goto(`http://localhost:5173${pageInfo.path}`);
    await page.setViewport({ width: 1920, height: 1080 });
    await page.screenshot({
      path: `docs/public/images/${pageInfo.path.replace(/\//g, '-')}.png`,
      fullPage: true
    });
    
    // Generate documentation
    await generatePageDoc(pageInfo);
  }
  
  await browser.close();
  console.log('Screenshots and documentation updated successfully');
}

async function generatePageDoc(pageInfo: PageInfo) {
  const docContent = `# ${pageInfo.title}

![${pageInfo.title}](/images${pageInfo.path.replace(/\//g, '-')}.png)

## Purpose
${pageInfo.purpose}

## Navigation
${pageInfo.navigation.map(item => `- ${item}`).join('\n')}

## Controls
${pageInfo.controls.map(item => `- ${item}`).join('\n')}

## Dependencies
${pageInfo.dependencies.map(item => `- ${item}`).join('\n')}

## Data Sources
${pageInfo.sources.map(item => `- ${item}`).join('\n')}

## Next Steps
${pageInfo.nextSteps.map(item => `- ${item}`).join('\n')}
`;

  await fs.writeFile(
    `docs/pages${pageInfo.path}.md`,
    docContent,
    'utf8'
  );
}

captureScreenshots().catch(console.error);