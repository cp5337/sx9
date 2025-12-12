#!/bin/bash
# Container execution for: Detection Avoidance for CBRN

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Detection Avoidance for CBRN"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
