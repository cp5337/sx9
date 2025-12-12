#!/bin/bash
# Container execution for: Digital Anonymization

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Digital Anonymization"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
