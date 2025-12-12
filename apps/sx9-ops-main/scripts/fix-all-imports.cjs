#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function fixImports(content) {
  const lines = content.split('\n');
  const importLines = [];
  const otherLines = [];
  const seenImports = new Map(); // Track imports by source
  
  lines.forEach(line => {
    if (line.trim().startsWith('import ')) {
      // Parse import statement
      const importMatch = line.match(/import\s+(.+?)\s+from\s+['"]([^'"]+)['"]/);
      if (importMatch) {
        const importContent = importMatch[1];
        const source = importMatch[2];
        
        if (!seenImports.has(source)) {
          seenImports.set(source, importContent);
          importLines.push(line);
        } else {
          // Merge with existing import
          const existing = seenImports.get(source);
          if (importContent.startsWith('{') && existing.startsWith('{')) {
            // Merge named imports
            const existingItems = existing.slice(1, -1).split(',').map(i => i.trim());
            const newItems = importContent.slice(1, -1).split(',').map(i => i.trim());
            const mergedItems = [...new Set([...existingItems, ...newItems])];
            const mergedImport = `import { ${mergedItems.join(', ')} } from '${source}';`;
            
            // Replace the existing import
            const existingIndex = importLines.findIndex(l => l.includes(`from '${source}'`));
            if (existingIndex !== -1) {
              importLines[existingIndex] = mergedImport;
            }
          } else {
            // Keep the first one for default imports
            console.log(`Skipping duplicate import: ${line.trim()}`);
          }
        }
      } else {
        importLines.push(line);
      }
    } else {
      otherLines.push(line);
    }
  });
  
  return [...importLines, ...otherLines].join('\n');
}

function processFile(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    const newContent = fixImports(content);
    
    if (newContent !== content) {
      fs.writeFileSync(filePath, newContent, 'utf8');
      console.log(`Fixed imports: ${filePath}`);
    }
  } catch (error) {
    console.error(`Error processing ${filePath}:`, error.message);
  }
}

function walkDir(dir) {
  const files = fs.readdirSync(dir);
  
  files.forEach(file => {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);
    
    if (stat.isDirectory() && !['node_modules', '.git', 'dist'].includes(file)) {
      walkDir(filePath);
    } else if (stat.isFile() && (file.endsWith('.tsx') || file.endsWith('.ts'))) {
      processFile(filePath);
    }
  });
}

console.log('🔧 Fixing all import issues...');
walkDir('./src');
console.log('✅ Import fixes complete!');
