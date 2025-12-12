#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function cleanupDuplicates(content) {
  const lines = content.split('\n');
  const cleanedLines = [];
  const seenImports = new Set();
  
  lines.forEach(line => {
    if (line.trim().startsWith('import ')) {
      // Check if this import already exists
      const importKey = line.trim();
      if (!seenImports.has(importKey)) {
        seenImports.add(importKey);
        cleanedLines.push(line);
      }
    } else {
      cleanedLines.push(line);
    }
  });
  
  return cleanedLines.join('\n');
}

function processFile(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    const newContent = cleanupDuplicates(content);
    
    if (newContent !== content) {
      fs.writeFileSync(filePath, newContent, 'utf8');
      console.log(`Cleaned duplicates: ${filePath}`);
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

console.log('🧹 Cleaning up duplicate imports...');
walkDir('./src');
console.log('✅ Duplicate cleanup complete!');
