#!/bin/zsh
# setup_antigravity.sh
# Automates the migration of Antigravity to sx9-development-center

SOURCE_ROOT="/Users/cp5337/Developer/ctas-7-shipyard-staging"
SX9_ROOT="/Users/cp5337/Developer/sx9"
TARGET_ROOT="/Users/cp5337/Developer/sx9-development-center"
ANTIGRAVITY_DIR="$TARGET_ROOT/antigravity"
CAPABILITIES_DIR="$ANTIGRAVITY_DIR/capabilities"

echo "🚀 Starting Antigravity Setup in Development Center..."

# 1. Create Directory Structure
echo "📂 Creating directory structure..."
mkdir -p "$CAPABILITIES_DIR"

# 2. Migrate Tools & configurations
echo "🛠️  Migrating Bootloader and Tools..."
cp "$SX9_ROOT/antigravity/bootloader.py" "$ANTIGRAVITY_DIR/"
cp "$SX9_ROOT/sx9-conda/src/atomic_tools.py" "$ANTIGRAVITY_DIR/"
cp "$SX9_ROOT/antigravity/boot_manifest.toml" "$ANTIGRAVITY_DIR/"
cp "$SX9_ROOT/antigravity/"*.toml "$ANTIGRAVITY_DIR/" 2>/dev/null
cp "$SX9_ROOT/antigravity/"*.yaml "$ANTIGRAVITY_DIR/" 2>/dev/null
cp "$SX9_ROOT/antigravity/"*.json "$ANTIGRAVITY_DIR/" 2>/dev/null
cp "$SX9_ROOT/antigravity/# ANTIGRAVITY AGENT BOOT SEQUENCE" "$ANTIGRAVITY_DIR/"

# 3. Migrate Capabilities (Key Vault, ABE, IAC)
echo "📦 Migrating Capabilities..."

# ABE/IAC
if [ -d "$SOURCE_ROOT/04-abe-iac" ]; then
    echo "   -> Copying ABE/IAC..."
    cp -r "$SOURCE_ROOT/04-abe-iac" "$CAPABILITIES_DIR/abe-iac"
else
    echo "   ⚠️  ABE/IAC not found in source!"
fi

# Key Vault
if [ -d "$SOURCE_ROOT/06 grok/key_vault" ]; then
    echo "   -> Copying Key Vault..."
    cp -r "$SOURCE_ROOT/06 grok/key_vault" "$CAPABILITIES_DIR/key_vault"
else
    echo "   ⚠️  Key Vault not found in source!"
fi

# 4. Integrate Conda Environment
echo "🐍 Integrating sx9-conda..."
# User requested "clone the directory". We will copy the entire environment.
# This takes time, so we'll be verbose.
if [ -d "$SX9_ROOT/sx9-conda" ]; then
    echo "   -> Cloning sx9-conda (This may take a moment)..."
    cp -r "$SX9_ROOT/sx9-conda" "$TARGET_ROOT/"
else
    echo "   ⚠️  sx9-conda source not found!"
fi

# 5. Create .gitignore
echo "📝 Creating .gitignore..."
cat > "$TARGET_ROOT/.gitignore" <<EOF
# Antigravity GitIgnore

# AI Models (Large Files)
models/
*.bin
*.safetensors
*.pt
*.gguf

# Python
__pycache__/
*.pyc
.venv/
.conda/
env/

# Key Vault (Secrets)
antigravity/capabilities/key_vault/secrets/
keys/
*.pem
*.key

# System
.DS_Store
EOF

echo "✅ Antigravity Setup Complete!"
echo "   Target: $ANTIGRAVITY_DIR"
