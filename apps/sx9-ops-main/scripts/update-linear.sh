#!/bin/bash
# CTAS v7.3.1 Linear Progress Updater
# Automatically creates/updates Linear issues from git commits

set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"
LINEAR_API_KEY="${LINEAR_API_KEY:-$(cat ~/.linear_api_key 2>/dev/null || echo '')}"
LINEAR_TEAM_ID="979acadf-8301-459e-9e51-bf3c1f60e496" # CognetixALPHA/COG
LINEAR_API="https://api.linear.app/graphql"

if [ -z "$LINEAR_API_KEY" ]; then
    echo "❌ LINEAR_API_KEY not found"
    echo "   Set via: export LINEAR_API_KEY=your_key"
    echo "   Or save to: ~/.linear_api_key"
    exit 1
fi

echo "📋 CTAS v7.3.1 Linear Updater"
echo "============================="
echo ""

# Get commit info
COMMIT_HASH=$(git rev-parse --short HEAD)
COMMIT_MSG=$(git log -1 --pretty=%B | head -1)
COMMIT_BODY=$(git log -1 --pretty=%B | tail -n +2)
FILES_CHANGED=$(git diff --stat HEAD~1 HEAD 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")

echo "📝 Commit: $COMMIT_HASH"
echo "💬 Message: $COMMIT_MSG"
echo ""

# Parse commit type (feat/fix/docs/etc)
COMMIT_TYPE=$(echo "$COMMIT_MSG" | grep -oE '^[a-z]+' || echo "chore")
COMMIT_SCOPE=$(echo "$COMMIT_MSG" | grep -oE '\([a-z-]+\)' | tr -d '()' || echo "general")

# Determine priority based on commit type
case "$COMMIT_TYPE" in
    feat)
        PRIORITY=2  # High
        LABEL="feature"
        ;;
    fix)
        PRIORITY=1  # Urgent
        LABEL="bug"
        ;;
    docs)
        PRIORITY=4  # Low
        LABEL="documentation"
        ;;
    refactor)
        PRIORITY=3  # Normal
        LABEL="improvement"
        ;;
    *)
        PRIORITY=3  # Normal
        LABEL="task"
        ;;
esac

# Create Linear issue
echo "📋 Creating Linear issue..."

ISSUE_TITLE="$COMMIT_MSG"
ISSUE_DESCRIPTION=$(cat << DESCEOF
## Commit: $COMMIT_HASH

### Changes
$COMMIT_BODY

### Statistics
- Files changed: $FILES_CHANGED
- Type: $COMMIT_TYPE
- Scope: $COMMIT_SCOPE

### Repository
- Branch: main
- Commit: https://github.com/cp5337/sb1-snwqto-ctas_6/commit/$COMMIT_HASH

---
*Auto-generated from git commit*
DESCEOF
)

# GraphQL mutation to create issue
MUTATION=$(cat << GRAPHQLEOF
mutation {
  issueCreate(input: {
    teamId: "$LINEAR_TEAM_ID"
    title: $(echo "$ISSUE_TITLE" | jq -Rs .)
    description: $(echo "$ISSUE_DESCRIPTION" | jq -Rs .)
    priority: $PRIORITY
    labelIds: []
  }) {
    success
    issue {
      id
      identifier
      url
    }
  }
}
GRAPHQLEOF
)

RESPONSE=$(curl -s -X POST "$LINEAR_API" \
  -H "Authorization: $LINEAR_API_KEY" \
  -H "Content-Type: application/json" \
  -d "{\"query\": $(echo "$MUTATION" | jq -Rs .)}")

# Extract issue info
ISSUE_ID=$(echo "$RESPONSE" | jq -r '.data.issueCreate.issue.identifier' 2>/dev/null || echo "")
ISSUE_URL=$(echo "$RESPONSE" | jq -r '.data.issueCreate.issue.url' 2>/dev/null || echo "")

if [ -n "$ISSUE_ID" ] && [ "$ISSUE_ID" != "null" ]; then
    echo "✅ Linear issue created: $ISSUE_ID"
    echo "🔗 URL: $ISSUE_URL"
    
    # Save to commit message
    git notes add -m "Linear: $ISSUE_ID - $ISSUE_URL" HEAD 2>/dev/null || true
else
    echo "⚠️  Failed to create Linear issue"
    echo "Response: $RESPONSE"
fi

# Update project progress
echo ""
echo "📊 Updating project progress..."

# Get recent issues
QUERY=$(cat << GRAPHQLEOF
query {
  team(id: "$LINEAR_TEAM_ID") {
    issues(first: 50, filter: {state: {name: {in: ["In Progress", "Done"]}}}) {
      nodes {
        identifier
        title
        state {
          name
        }
        priority
        createdAt
      }
    }
  }
}
GRAPHQLEOF
)

ISSUES_RESPONSE=$(curl -s -X POST "$LINEAR_API" \
  -H "Authorization: $LINEAR_API_KEY" \
  -H "Content-Type: application/json" \
  -d "{\"query\": $(echo "$QUERY" | jq -Rs .)}")

TOTAL_ISSUES=$(echo "$ISSUES_RESPONSE" | jq -r '.data.team.issues.nodes | length' 2>/dev/null || echo "0")
DONE_ISSUES=$(echo "$ISSUES_RESPONSE" | jq -r '.data.team.issues.nodes | map(select(.state.name == "Done")) | length' 2>/dev/null || echo "0")

echo "✅ Project Progress:"
echo "   Total Issues: $TOTAL_ISSUES"
echo "   Completed: $DONE_ISSUES"
echo "   In Progress: $((TOTAL_ISSUES - DONE_ISSUES))"

# Create progress summary
PROGRESS_PCT=$((DONE_ISSUES * 100 / TOTAL_ISSUES))
echo "   Progress: $PROGRESS_PCT%"

echo ""
echo "============================="
echo "✅ Linear Update Complete"
echo "============================="

