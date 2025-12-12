#!/bin/bash
# Container execution for: Packet Capture Analysis

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Packet Capture Analysis"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
