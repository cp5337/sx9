#!/bin/bash
# Container execution for: Tactical Kinetic Operations

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Tactical Kinetic Operations"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
