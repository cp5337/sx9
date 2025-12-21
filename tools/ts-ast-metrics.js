const fs = require("fs");
const path = require("path");
// dynamic require from CWD or target?
// We'll try to require 'typescript' from the target project's node_modules
// or global. Alternatively, we can use a simpler approach if not found.
// But for now, let's assume valid node environment.

let ts;
try {
  ts = require("typescript");
} catch (e) {
  // Try to find it in the target directory if provided as arg, or current CWD
  try {
    ts = require(path.join(process.cwd(), "node_modules", "typescript"));
  } catch (e2) {
    // Fallback: This script might be run from root, but graph-db is elsewhere
    // We will try to resolve it from the input path's node_modules later.
  }
}

const ignorePatterns = [];
try {
  const ignoreContent = fs.readFileSync(".lightningignore", "utf-8");
  ignoreContent.split("\n").forEach((line) => {
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("#")) {
      ignorePatterns.push(trimmed);
    }
  });
} catch (e) {
  // No ignore file
}
// Default ignores if explicit file doesn't cover them,
// though the ignore file should be the source of truth if strictly following framework.
// But valid safeguards are good.
const defaultIgnores = ["node_modules/", ".git/", "dist/", "build/", ".next/"];
defaultIgnores.forEach((p) => {
  if (!ignorePatterns.includes(p)) ignorePatterns.push(p);
});

function shouldIgnore(filePath) {
  // Normalize path for check
  const relPath = path.isAbsolute(filePath)
    ? path.relative(process.cwd(), filePath)
    : filePath;
  for (const pattern of ignorePatterns) {
    if (relPath.includes(pattern)) return true;
  }
  return false;
}

function getAllFiles(dirPath, arrayOfFiles) {
  // Check directory exclude
  if (shouldIgnore(dirPath + "/")) return arrayOfFiles || [];

  files = fs.readdirSync(dirPath);
  arrayOfFiles = arrayOfFiles || [];
  files.forEach(function (file) {
    const absPath = path.join(dirPath, file);
    if (shouldIgnore(absPath)) return;

    if (fs.statSync(absPath).isDirectory()) {
      arrayOfFiles = getAllFiles(absPath, arrayOfFiles);
    } else {
      if (file.match(/\.(ts|tsx|js|jsx)$/)) {
        arrayOfFiles.push(absPath);
      }
    }
  });
  return arrayOfFiles;
}

function calculateComplexity(sourceFile) {
  let complexity = 1; // Base

  function visit(node) {
    switch (node.kind) {
      case ts.SyntaxKind.IfStatement:
      case ts.SyntaxKind.ForStatement:
      case ts.SyntaxKind.ForInStatement:
      case ts.SyntaxKind.ForOfStatement:
      case ts.SyntaxKind.WhileStatement:
      case ts.SyntaxKind.DoStatement:
      case ts.SyntaxKind.CatchClause:
      case ts.SyntaxKind.ConditionalExpression: // Ternary
        complexity++;
        break;
      case ts.SyntaxKind.SwitchStatement:
        // Switch itself usually doesn't count, CaseClauses do?
        // McCabe: each branch.
        break;
      case ts.SyntaxKind.CaseClause:
        complexity++;
        break;
      case ts.SyntaxKind.BinaryExpression:
        if (
          node.operatorToken.kind === ts.SyntaxKind.AmpersandAmpersandToken ||
          node.operatorToken.kind === ts.SyntaxKind.BarBarToken
        ) {
          complexity++;
        }
        break;
    }
    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  return complexity;
}

function analyze(targetPath) {
  // Attempt to load typescript from target if not loaded
  if (!ts) {
    try {
      ts = require(path.join(targetPath, "node_modules", "typescript"));
    } catch (e) {
      console.error(
        "Error: 'typescript' module not found. Please run 'npm install' in target."
      );
      process.exit(1);
    }
  }

  const files = getAllFiles(targetPath);
  const results = [];

  files.forEach((filePath) => {
    const content = fs.readFileSync(filePath, "utf-8");
    const sourceFile = ts.createSourceFile(
      filePath,
      content,
      ts.ScriptTarget.Latest,
      true
    );

    const loc = content.split(/\r\n|\r|\n/).length;
    const cyclo = calculateComplexity(sourceFile);

    // Approx grade
    let grade = "A";
    if (cyclo > 10) grade = "B";
    if (cyclo > 20) grade = "C";
    if (cyclo > 50) grade = "F";

    results.push({
      path: path.relative(targetPath, filePath),
      loc: loc,
      complexity: cyclo,
      functions: 0, // TODO: count functions
      grade: grade,
      mi: 100, // Placeholder
    });
  });

  console.log(JSON.stringify(results, null, 2));
}

const target = process.argv[2] || ".";
analyze(target);
