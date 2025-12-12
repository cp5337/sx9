#!/bin/bash
# Container execution for: Insider Recruitment

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Insider Recruitment"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
