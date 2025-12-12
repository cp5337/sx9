#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Common missing imports to add
const missingImports = {
  'AlertTriangle': 'lucide-react',
  'Play': 'lucide-react',
  'Trash2': 'lucide-react',
  'Plus': 'lucide-react',
  'Shield': 'lucide-react',
  'Zap': 'lucide-react',
  'Globe': 'lucide-react',
  'Database': 'lucide-react',
  'Activity': 'lucide-react',
  'RefreshCw': 'lucide-react',
  'Clock': 'lucide-react',
  'DollarSign': 'lucide-react',
  'Save': 'lucide-react',
  'Eye': 'lucide-react'
};

function addMissingImports(content, filePath) {
  const lines = content.split('\n');
  const importLines = [];
  const otherLines = [];
  let inImportSection = false;
  
  lines.forEach(line => {
    if (line.trim().startsWith('import ')) {
      importLines.push(line);
      inImportSection = true;
    } else if (inImportSection && line.trim() === '') {
      importLines.push(line);
    } else {
      otherLines.push(line);
      inImportSection = false;
    }
  });

  // Find which imports are missing
  const missingIcons = [];
  Object.keys(missingImports).forEach(icon => {
    if (content.includes(icon) && !content.includes(`import.*${icon}`)) {
      missingIcons.push(icon);
    }
  });

  if (missingIcons.length > 0) {
    // Find existing lucide-react import
    let lucideImport = importLines.find(line => line.includes('lucide-react'));
    if (lucideImport) {
      // Add to existing import
      const importMatch = lucideImport.match(/import\s*{([^}]+)}\s*from\s*['"]lucide-react['"]/);
      if (importMatch) {
        const existingIcons = importMatch[1].split(',').map(i => i.trim());
        const newIcons = [...existingIcons, ...missingIcons];
        const newImport = `import { ${newIcons.join(', ')} } from 'lucide-react';`;
        const lucideIndex = importLines.findIndex(line => line.includes('lucide-react'));
        importLines[lucideIndex] = newImport;
      }
    } else {
      // Create new import
      importLines.push(`import { ${missingIcons.join(', ')} } from 'lucide-react';`);
    }
  }

  return [...importLines, ...otherLines].join('\n');
}

function processFile(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    const newContent = addMissingImports(content, filePath);
    
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

console.log('🔧 Fixing remaining TypeScript issues...');
walkDir('./src');
console.log('✅ Import fixes complete!');
