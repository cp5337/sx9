#!/bin/bash
# fix_spelling.sh
# SAFE auto-corrector wrapper for codespell.

# Ensure we are in the repo root
cd "$(dirname "$0")/.." || exit 1

if ! command -v codespell &> /dev/null; then
    echo "Error: codespell is not installed."
    echo "Install it with: pip install codespell"
    exit 1
fi

echo "⚠️  Preparing to auto-correct spelling..."
echo "   Config: .codespellrc"

# Safety check for git status
if [[ -n $(git status -s) ]]; then
    echo "⚠️  WARNING: You have uncommitted changes."
    read -p "    Are you sure you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

# Run codespell with write-changes
echo "✍️  Fixing typos..."
codespell --write-changes

echo "✅ Done. Please review 'git diff' before committing."
