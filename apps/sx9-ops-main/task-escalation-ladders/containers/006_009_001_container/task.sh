#!/bin/bash
# Container execution for: Covert Data Exfiltration

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Covert Data Exfiltration"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
