#!/bin/bash
# Container execution for: UAVs for ISR and Infiltration

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: UAVs for ISR and Infiltration"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
