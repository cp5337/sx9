#!/bin/bash
# Setup all API keys from SX9_API_VAULT.json
# SINGLE SOURCE OF TRUTH - sx9/tools/vault/

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VAULT_PATH="$SCRIPT_DIR/SX9_API_VAULT.json"

if [ ! -f "$VAULT_PATH" ]; then
    echo "❌ Vault not found at: $VAULT_PATH"
    exit 1
fi

echo "🔐 Loading API keys from sx9 vault..."

# Extract and export keys
eval $(python3 << PYEOF
import json
import os
from pathlib import Path

vault_path = Path("$VAULT_PATH")

with open(vault_path) as f:
    vault = json.load(f)

api_keys = vault.get("api_keys", {})
db_connections = vault.get("database_connections", {})

# API key mappings
key_mappings = {
    "google_gemini": ["GEMINI_API_KEY", "GOOGLE_API_KEY"],
    "openai": ["OPENAI_API_KEY"],
    "anthropic": ["ANTHROPIC_API_KEY"],
    "elevenlabs": ["ELEVENLABS_API_KEY"],
    "linear": ["LINEAR_API_KEY"],
    "mapbox": ["MAPBOX_TOKEN"],
    "grok": ["GROK_API_KEY", "XAI_API_KEY"],
}

exported = []
for service, env_vars in key_mappings.items():
    service_config = api_keys.get(service, {})
    key = (service_config.get("api_key") or
           service_config.get("access_token") or
           service_config.get("subscription_key"))

    if key and key not in ["NEEDS_REPLACEMENT", "NEEDS_SETUP", "CONFIGURE_IN_APP"]:
        for env_var in env_vars:
            print(f"export {env_var}='{key}'")
            exported.append(env_var)

# Database connections
if "supabase" in db_connections:
    supabase = db_connections["supabase"]
    if supabase.get("url"):
        print(f"export SUPABASE_URL='{supabase['url']}'")
        exported.append("SUPABASE_URL")
    if supabase.get("anon_key"):
        print(f"export SUPABASE_ANON_KEY='{supabase['anon_key']}'")
        exported.append("SUPABASE_ANON_KEY")

if "neon" in db_connections:
    neon = db_connections["neon"]
    if neon.get("connection_string"):
        print(f"export DATABASE_URL='{neon['connection_string']}'")
        exported.append("DATABASE_URL")

print(f"# Exported {len(exported)} environment variables", file=os.sys.stderr)
PYEOF
)

echo ""
echo "✅ API keys loaded into current shell"
echo ""
echo "To persist, add to ~/.zshrc:"
echo "  source $SCRIPT_DIR/setup-api-keys.sh"
