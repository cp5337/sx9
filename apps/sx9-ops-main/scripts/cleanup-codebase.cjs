#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Common unused imports to remove
const unusedImports = [
  'useState', 'useEffect', 'useRef', 'useCallback', 'useMemo',
  'Clock', 'DollarSign', 'Save', 'Eye', 'MessageSquare', 'Calculator', 'Wifi',
  'Terminal', 'Box', 'RefreshCw', 'Play', 'Square', 'AlertTriangle',
  'Server', 'Cpu', 'Activity', 'Users', 'Plus', 'Trash2', 'Edit2',
  'Shield', 'Zap', 'Globe', 'Database', 'Cloud', 'BarChart2'
];

// Files to skip
const skipFiles = [
  'node_modules',
  '.git',
  'dist',
  'build',
  'coverage'
];

function processFile(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    let modified = false;

    // Remove unused imports
    const importRegex = /import\s*{([^}]+)}\s*from\s*['"][^'"]+['"];?/g;
    content = content.replace(importRegex, (match, imports) => {
      const importList = imports.split(',').map(imp => imp.trim());
      const usedImports = importList.filter(imp => {
        const cleanImp = imp.replace(/\s+as\s+\w+/, '').trim();
        return !unusedImports.includes(cleanImp) && content.includes(cleanImp);
      });
      
      if (usedImports.length === 0) {
        modified = true;
        return '';
      } else if (usedImports.length !== importList.length) {
        modified = true;
        return `import { ${usedImports.join(', ')} } from '${match.match(/from\s+['"]([^'"]+)['"]/)[1]}';`;
      }
      return match;
    });

    // Comment out unused variables
    const unusedVars = [
      'selectedNode', 'setSelectedNode', 'selectedTest', 'setSelectedTest',
      'newTask', 'setNewTask', 'dimensions', 'setDimensions', 'hoveredNode', 'setHoveredNode',
      'graphData', 'relationships', 'status', 'setStatus'
    ];

    unusedVars.forEach(varName => {
      const varRegex = new RegExp(`const\\s+\\[${varName}\\s*,\\s*set${varName.charAt(0).toUpperCase() + varName.slice(1)}\\]\\s*=\\s*useState[^;]+;`, 'g');
      content = content.replace(varRegex, `// const [${varName}, set${varName.charAt(0).toUpperCase() + varName.slice(1)}] = useState(...);`);
      modified = true;
    });

    // Fix broken imports
    const brokenImports = [
      { from: '../../utils/redisGraph', to: '// import from redisGraph' },
      { from: './httpClient', to: '../services/api/ApiService' },
      { from: './mongodb', to: '../services/database/MongoDBService' },
      { from: './neo4jDriver', to: '../services/database/Neo4jService' }
    ];

    brokenImports.forEach(({ from, to }) => {
      const importRegex = new RegExp(`import\\s+.*from\\s+['"]${from.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}['"];?`, 'g');
      if (content.match(importRegex)) {
        content = content.replace(importRegex, `// import from ${from}`);
        modified = true;
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
    
    if (stat.isDirectory() && !skipFiles.includes(file)) {
      walkDir(filePath);
    } else if (stat.isFile() && (file.endsWith('.tsx') || file.endsWith('.ts'))) {
      processFile(filePath);
    }
  });
}

console.log('Starting codebase cleanup...');
walkDir('./src');
console.log('Cleanup complete!');
