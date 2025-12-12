#!/bin/bash
# Container execution for: Traffic Violations

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Traffic Violations"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
