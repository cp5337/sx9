#!/bin/bash
# Remove External Repositories from Git Tracking
# These repos should NOT be in our git - they're cloned fresh on each run

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "🗑️  Removing External Repos from Git Tracking"
echo "=============================================="
echo ""

# External repos to remove from git (but keep locally)
EXTERNAL_REPOS=(
    "04-abe-iac/node-interview-generator/output/threat_content/car"
    "04-abe-iac/node-interview-generator/output/threat_content/atlas"
    "04-abe-iac/node-interview-generator/output/threat_content/engage"
    "04-abe-iac/node-interview-generator/output/threat_content/atomic-red-team"
    "04-abe-iac/node-interview-generator/output/threat_content/nuclei-templates"
    "04-abe-iac/node-interview-generator/output/threat_content/sigma"
    "04-abe-iac/node-interview-generator/output/threat_content/yara-rules"
    "04-abe-iac/node-interview-generator/output/threat_content/wazuh"
    "04-abe-iac/node-interview-generator/output/threat_content/lolbas"
    "04-abe-iac/node-interview-generator/output/threat_content/gtfobins"
    "04-abe-iac/node-interview-generator/output/threat_content/loldrivers"
    "04-abe-iac/node-interview-generator/output/threat_content/hijacklibs"
    "04-abe-iac/node-interview-generator/output/threat_content/wadcoms"
    "04-abe-iac/node-interview-generator/output/threat_content/nmap"
    "04-abe-iac/node-interview-generator/output/threat_content/awesome-osint"
    "04-abe-iac/node-interview-generator/output/threat_content/osint-framework"
    "04-abe-iac/node-interview-generator/output/threat_content/sherlock"
    "04-abe-iac/node-interview-generator/output/threat_content/caldera"
)

REMOVED=0
SKIPPED=0

for repo_path in "${EXTERNAL_REPOS[@]}"; do
    if git ls-files --error-unmatch "$repo_path" &>/dev/null; then
        echo "🗑️  Removing: $repo_path"
        git rm -r --cached "$repo_path" 2>/dev/null || {
            echo "   ⚠️  Failed to remove (may not be tracked)"
        }
        REMOVED=$((REMOVED + 1))
    else
        echo "⏭️  Skipping: $repo_path (not tracked)"
        SKIPPED=$((SKIPPED + 1))
    fi
done

echo ""
echo "=============================================="
echo "✅ Removal Complete"
echo "=============================================="
echo ""
echo "📊 Summary:"
echo "   • Removed: $REMOVED repos"
echo "   • Skipped: $SKIPPED repos (not tracked)"
echo ""
echo "💡 Next Steps:"
echo "   1. Review changes: git status"
echo "   2. Commit removal: git commit -m 'Remove external repos from tracking'"
echo "   3. Repos will still exist locally (just not in git)"
echo "   4. .gitignore will prevent re-adding them"
echo ""


