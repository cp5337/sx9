#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Migration rules
const migrationRules = [
  // Convert default exports to named exports
  {
    name: 'Convert default exports to named exports',
    pattern: /export default function (\w+)/g,
    replacement: 'export function $1'
  },
  {
    name: 'Convert default const exports to named exports',
    pattern: /export default const (\w+)/g,
    replacement: 'export const $1'
  },
  // Add explicit return types
  {
    name: 'Add explicit return types to functions',
    pattern: /function (\w+)\(([^)]*)\)\s*{/g,
    replacement: 'function $1($2): void {'
  },
  // Remove any types
  {
    name: 'Replace any with unknown',
    pattern: /: any/g,
    replacement: ': unknown'
  },
  // Convert to absolute imports
  {
    name: 'Convert relative imports to absolute',
    pattern: /from ['"]\.\.\/([^'"]+)['"]/g,
    replacement: "from '@/components/$1'"
  }
];

function processFile(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    let modified = false;

    migrationRules.forEach(rule => {
      const newContent = content.replace(rule.pattern, rule.replacement);
      if (newContent !== content) {
        content = newContent;
        modified = true;
        console.log(`  ✓ Applied: ${rule.name}`);
      }
    });

    if (modified) {
      fs.writeFileSync(filePath, content, 'utf8');
      console.log(`Fixed: ${filePath}`);
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

console.log('🚀 Starting migration to TypeScript standards...');
console.log('📁 Processing files...');
walkDir('./src');
console.log('✅ Migration complete!');
console.log('\n📋 Next steps:');
console.log('1. Run: npm run lint');
console.log('2. Fix remaining ESLint errors');
console.log('3. Run: npm run format');
console.log('4. Update imports to use @/ paths');
