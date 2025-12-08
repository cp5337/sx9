#!/bin/bash
# Setup all API keys from CTAS7_API_VAULT.json
# This script loads keys from vault and exports them to environment

VAULT_PATH="/Users/cp5337/Developer/ctas-7-shipyard-staging/CTAS7_API_VAULT.json"

if [ ! -f "$VAULT_PATH" ]; then
    echo "❌ Vault not found at: $VAULT_PATH"
    exit 1
fi

echo "🔐 Loading API keys from vault..."
echo ""

# Extract and export keys using Python - actually export them
eval $(python3 << 'PYEOF'
import json
import os
from pathlib import Path

vault_path = Path("/Users/cp5337/Developer/ctas-7-shipyard-staging/CTAS7_API_VAULT.json")

with open(vault_path) as f:
    vault = json.load(f)

api_keys = vault.get("api_keys", {})

# Map vault services to environment variable names
key_mappings = {
    "google_gemini": ["GEMINI_API_KEY", "GOOGLE_API_KEY"],
    "openai": ["OPENAI_API_KEY", "VITE_OPENAI_API_KEY"],
    "anthropic": ["ANTHROPIC_API_KEY", "VITE_ANTHROPIC_API_KEY"],
    "elevenlabs": ["ELEVENLABS_API_KEY", "VITE_ELEVENLABS_API_KEY"],
    "linear": ["LINEAR_API_KEY", "VITE_LINEAR_API_KEY"],
    "mapbox": ["MAPBOX_TOKEN", "VITE_MAPBOX_TOKEN"],
    "grok": ["GROK_API_KEY", "VITE_GROK_API_KEY"],
}

exported = []
for service, env_vars in key_mappings.items():
    service_config = api_keys.get(service, {})
    
    # Get the key (try api_key, access_token, subscription_key)
    key = (service_config.get("api_key") or 
           service_config.get("access_token") or 
           service_config.get("subscription_key"))
    
    if key and key not in ["NEEDS_REPLACEMENT", "NEEDS_SETUP", "CONFIGURE_IN_APP"]:
        for env_var in env_vars:
            print(f"export {env_var}='{key}'")
            exported.append(env_var)
    else:
        print(f"# ⚠️  {service} key needs setup: {key}", file=os.sys.stderr)

print("", file=os.sys.stderr)
print(f"✅ Exported {len(exported)} API keys", file=os.sys.stderr)
PYEOF

echo ""
echo "✅ API keys loaded! Run: source setup-api-keys-from-vault.sh"
echo "   Or add to your ~/.zshrc: source $(pwd)/setup-api-keys-from-vault.sh"

