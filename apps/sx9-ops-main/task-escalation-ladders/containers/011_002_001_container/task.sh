#!/bin/bash
# Container execution for: Corrupting Law Enforcement

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Corrupting Law Enforcement"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
