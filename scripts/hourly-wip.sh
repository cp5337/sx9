#!/bin/bash
# Hourly WIP Commit Script
# Location: /Users/cp5337/Developer/sx9/scripts/hourly-wip.sh
# Purpose: Automatically commit work-in-progress every hour for recovery safety

set -e

cd /Users/cp5337/Developer/sx9

# Check if there are changes
if [[ -z $(git status --porcelain) ]]; then
  echo "No changes to commit"
  exit 0
fi

# Get current timestamp
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
HOUR=$(date +"%H")

# Add all changes
git add -A

# Create WIP commit
git commit -m "WIP: Hourly auto-commit at ${TIMESTAMP}

Automated hourly backup commit for recovery safety.
Hour: ${HOUR}:00

Changes:
$(git status --short | head -20)
"

# Push to remote
git push origin main

echo "✅ Hourly WIP committed and pushed at ${TIMESTAMP}"
