#!/bin/sh
#
# Install pre-commit hook

HOOK_PATH=".git/hooks/pre-commit"

echo "Installing pre-commit hook to $HOOK_PATH..."

cat > "$HOOK_PATH" << 'EOF'
#!/bin/sh
#
# Pre-commit hook to ensure code quality
# Run `cargo fmt --check` and `cargo check` before committing.

echo "Running pre-commit checks..."

# 1. Check formatting
echo "Checking formatting..."
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "Formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# 2. Check compilation (fast check)
echo "Checking compilation..."
cargo check --workspace
if [ $? -ne 0 ]; then
    echo "Compilation check failed. Fix errors before committing."
    exit 1
fi

echo "Pre-commit checks passed!"
exit 0
EOF

chmod +x "$HOOK_PATH"
echo "Pre-commit hook installed successfully!"
