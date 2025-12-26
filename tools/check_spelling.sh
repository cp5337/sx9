#!/bin/bash
# check_spelling.sh
# READ-ONLY wrapper for codespell to safely check for typos.

# Ensure we are in the repo root
cd "$(dirname "$0")/.." || exit 1

if ! command -v codespell &> /dev/null; then
    echo "Error: codespell is not installed."
    echo "Install it with: pip install codespell"
    exit 1
fi

echo "🔍 Running spell check (Read-Only)..."
echo "   Config: .codespellrc"
echo "   Excludes: sensitive directories and build artifacts"
echo ""

# Run codespell in summary/interactive mode (just showing output)
codespell
