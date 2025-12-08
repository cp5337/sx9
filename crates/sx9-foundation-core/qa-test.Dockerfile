# CTAS-7 Foundation Core - Sterile QA Testing Environment
# Completely isolated environment for testing the gold disk smart crate

FROM rust:1.82-slim AS qa-tester

# Install testing dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    wget \
    procps \
    htop \
    net-tools \
    && rm -rf /var/lib/apt/lists/*

# Create sterile testing directory
WORKDIR /sterile-test

# Copy only the foundation crate for testing
COPY Cargo.toml ./
COPY src/ ./src/

# Create QA test script
RUN cat > qa-test.sh << 'EOF'
#!/bin/bash
set -e

echo "🧪 CTAS-7 Foundation Core - Sterile QA Testing"
echo "================================================"

# Test 1: Basic compilation
echo "1️⃣ Testing basic compilation..."
cargo check
echo "✅ Basic compilation passed"

# Test 2: All features compilation
echo "2️⃣ Testing all features compilation..."
cargo check --all-features
echo "✅ All features compilation passed"

# Test 3: Individual feature testing
echo "3️⃣ Testing individual features..."
cargo check --features "neural-mux"
cargo check --features "database"
cargo check --features "unicode-assembly"
cargo check --features "xsd-integration"
echo "✅ Individual features passed"

# Test 4: Minimal feature set
echo "4️⃣ Testing minimal feature set..."
cargo check --no-default-features --features "foundation-core"
echo "✅ Minimal features passed"

# Test 5: Run all tests
echo "5️⃣ Running all tests..."
cargo test --all-features
echo "✅ All tests passed"

# Test 6: Release build
echo "6️⃣ Testing release build..."
cargo build --release --all-features
echo "✅ Release build passed"

# Test 7: Documentation generation
echo "7️⃣ Testing documentation generation..."
cargo doc --all-features --no-deps
echo "✅ Documentation generation passed"

# Test 8: Hash engine validation
echo "8️⃣ Testing hash engine functionality..."
cargo test --all-features hash_engine
echo "✅ Hash engine tests passed"

# Test 9: Neural Mux validation
echo "9️⃣ Testing Neural Mux functionality..."
cargo test --all-features neural_mux
echo "✅ Neural Mux tests passed"

# Test 10: Unicode Assembly validation
echo "🔟 Testing Unicode Assembly functionality..."
cargo test --all-features unicode_assembly
echo "✅ Unicode Assembly tests passed"

echo ""
echo "🎯 QA TESTING COMPLETE - ALL TESTS PASSED!"
echo "💎 CTAS-7 Foundation Core Gold Disk VALIDATED"
echo "⚡ Ready for production deployment"
EOF

RUN chmod +x qa-test.sh

# Set environment for sterile testing
ENV RUST_LOG=debug
ENV CTAS_QA_MODE=sterile
ENV CTAS_TEST_ISOLATION=true

# Health check for QA environment
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD echo "QA Environment Ready" || exit 1

# Default command runs QA tests
CMD ["./qa-test.sh"]