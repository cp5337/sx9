#!/bin/bash
# Container execution for: Document Forgery

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Document Forgery"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
