#!/bin/bash
# SX9 Agent Harness Runner
# Starts the synchronized Linear/Git/Slack agent system
#
# Prerequisites:
#   - LINEAR_API_KEY environment variable
#   - SLACK_BOT_TOKEN environment variable
#   - gh CLI authenticated
#   - Git configured with push access

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           SX9 Agent Harness Runner (RFC-9030)               ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check required environment variables
check_env() {
    local var_name=$1
    local var_value=${!var_name}
    if [ -z "$var_value" ]; then
        echo -e "${RED}ERROR: $var_name not set${NC}"
        echo "  Run: source tools/vault/setup-api-keys.sh"
        return 1
    else
        echo -e "${GREEN}✓${NC} $var_name configured"
        return 0
    fi
}

echo -e "${YELLOW}Checking prerequisites...${NC}"
echo ""

# Check environment
ENV_OK=true
check_env "LINEAR_API_KEY" || ENV_OK=false
check_env "SLACK_BOT_TOKEN" || ENV_OK=false

# Check gh CLI
if command -v gh &> /dev/null; then
    if gh auth status &> /dev/null; then
        echo -e "${GREEN}✓${NC} gh CLI authenticated"
    else
        echo -e "${RED}ERROR: gh CLI not authenticated${NC}"
        echo "  Run: gh auth login"
        ENV_OK=false
    fi
else
    echo -e "${RED}ERROR: gh CLI not installed${NC}"
    echo "  Install: brew install gh"
    ENV_OK=false
fi

# Check git
if git remote get-url origin &> /dev/null; then
    echo -e "${GREEN}✓${NC} Git remote configured"
else
    echo -e "${RED}ERROR: Git remote not configured${NC}"
    ENV_OK=false
fi

echo ""

if [ "$ENV_OK" = false ]; then
    echo -e "${RED}Prerequisites not met. Exiting.${NC}"
    exit 1
fi

# Build the agent
echo -e "${YELLOW}Building sx9-linear-agent...${NC}"
cd "$PROJECT_ROOT/sx9-linear-agent"

if cargo build --release 2>&1; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}Build failed. Check errors above.${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Starting agent harness...${NC}"
echo ""

# Export config path
export SX9_CONFIG_PATH="$PROJECT_ROOT/sx9-linear-agent/config/linear.toml"

# Run the agent
echo -e "${BLUE}Agent starting with:${NC}"
echo "  - Linear: Polling every 60s"
echo "  - Slack: #sx9-dev channel"
echo "  - QA Gates: Static, Arch, Pattern, Semantic"
echo "  - Git: Auto-branch and PR creation"
echo ""
echo -e "${GREEN}Press Ctrl+C to stop${NC}"
echo ""

# Start the agent
"$PROJECT_ROOT/sx9-linear-agent/target/release/sx9-linear-agent"
