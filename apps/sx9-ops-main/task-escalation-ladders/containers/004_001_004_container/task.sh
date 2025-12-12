#!/bin/bash
# Container execution for: Trade-Based Money Laundering

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Trade-Based Money Laundering"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
