#!/bin/bash
# Container execution for: Coercion and Intimidation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Coercion and Intimidation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
