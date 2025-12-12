#!/bin/bash

# Sync tokens from CTAS7_API_VAULT.json to .env file
# This ensures tokens are always up-to-date from the vault

VAULT_PATH="../../CTAS7_API_VAULT.json"
ENV_PATH=".env"

if [ ! -f "$VAULT_PATH" ]; then
    echo "❌ Vault not found at: $VAULT_PATH"
    exit 1
fi

echo "🔐 Syncing tokens from vault to .env..."

# Extract tokens using Python (more reliable than jq for nested JSON)
python3 << 'PYEOF'
import json
import os

vault_path = "../../CTAS7_API_VAULT.json"
env_path = ".env"

try:
    with open(vault_path, 'r') as f:
        vault = json.load(f)
    
    api_keys = vault.get('api_keys', {})
    
    # Map vault keys to .env variable names
    token_map = {
        'mapbox': {
            'access_token': ['VITE_MAPBOX_TOKEN', 'VITE_MAPBOX_ACCESS_TOKEN']
        },
        'openai': {
            'api_key': ['VITE_OPENAI_API_KEY']
        },
        'gemini': {
            'api_key': ['VITE_GEMINI_API_KEY']
        },
        'anthropic': {
            'api_key': ['VITE_ANTHROPIC_API_KEY']
        },
        'elevenlabs': {
            'api_key': ['VITE_ELEVENLABS_API_KEY']
        },
        'linear': {
            'api_key': ['VITE_LINEAR_API_KEY']
        }
    }
    
    # Read existing .env or create new
    env_vars = {}
    if os.path.exists(env_path):
        with open(env_path, 'r') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#') and '=' in line:
                    key, value = line.split('=', 1)
                    env_vars[key] = value
    
    # Update tokens from vault
    updated = []
    for service, config in token_map.items():
        if service in api_keys:
            service_data = api_keys[service]
            for key_field, env_vars_list in config.items():
                token_value = service_data.get(key_field) or service_data.get('access_token')
                if token_value and token_value not in ['NEEDS_REPLACEMENT', 'NEEDS_SETUP', 'CONFIGURE_IN_APP']:
                    for env_var in env_vars_list:
                        if env_vars.get(env_var) != token_value:
                            env_vars[env_var] = token_value
                            updated.append(env_var)
    
    # Write updated .env
    with open(env_path, 'w') as f:
        f.write("# SX9 Ops Main Platform - Environment Configuration\n")
        f.write("# Auto-synced from CTAS7_API_VAULT.json\n\n")
        
        # Write all vars
        for key, value in sorted(env_vars.items()):
            f.write(f"{key}={value}\n")
    
    if updated:
        print(f"✅ Updated {len(updated)} token variables: {', '.join(updated)}")
    else:
        print("✅ All tokens already up-to-date")
        
except Exception as e:
    print(f"❌ Error syncing tokens: {e}")
    exit(1)
PYEOF

echo ""
echo "✅ Token sync complete!"



