#!/bin/bash
# Container execution for: OSINT Target Analysis

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: OSINT Target Analysis"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
