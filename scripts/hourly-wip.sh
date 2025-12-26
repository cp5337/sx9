#!/bin/bash
# SX9 Hourly WIP Commit Script
# Location: /Users/cp5337/Developer/sx9/scripts/hourly-wip.sh
# Purpose: Automatically commit work-in-progress every hour for recovery safety
#
# Commits on ANY branch (including feature branches) to prevent work loss.

set -e

REPO_PATH="/Users/cp5337/Developer/sx9"
LOG_FILE="/Users/cp5337/sx9-wip-commits.log"

cd "$REPO_PATH"

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)

# Check if there are changes (staged, unstaged, or untracked)
if [[ -z $(git status --porcelain) ]]; then
  echo "[$(date '+%Y-%m-%d %H:%M:%S')] No changes on branch: $CURRENT_BRANCH" >> "$LOG_FILE"
  exit 0
fi

TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
CHANGED_FILES=$(git status --porcelain | wc -l | tr -d ' ')

# Add all changes including untracked
git add -A

# Commit with --no-verify for speed
git commit --no-verify -m "WIP: Hourly auto-commit at ${TIMESTAMP}

Branch: ${CURRENT_BRANCH}
Files: ${CHANGED_FILES}

$(git status --short | head -20)
"

# Push to current branch (not hardcoded)
if git push origin "$CURRENT_BRANCH" 2>&1; then
  echo "[${TIMESTAMP}] ✅ Pushed to ${CURRENT_BRANCH} (${CHANGED_FILES} files)" >> "$LOG_FILE"
else
  echo "[${TIMESTAMP}] ⚠️ Committed locally, push failed for ${CURRENT_BRANCH}" >> "$LOG_FILE"
fi
