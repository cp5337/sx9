#!/bin/bash
# Container execution for: Cartel Counterintelligence

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Cartel Counterintelligence"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
