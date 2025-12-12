#!/bin/bash
# Container execution for: Arrest Avoidance

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Arrest Avoidance"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
