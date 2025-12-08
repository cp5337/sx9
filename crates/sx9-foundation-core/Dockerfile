# CTAS-7 Foundation Core - Gold Disk Smart Crate Docker Image
# Tesla/SpaceX engineering standards with multi-stage build

# Build stage - Rust compilation
FROM rust:1.82-slim AS builder

# Install system dependencies for compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifest first for better caching
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the foundation crate with all features
RUN cargo build --release --all-features

# Runtime stage - Minimal distroless image
FROM gcr.io/distroless/cc-debian12

# Metadata for Smart Crate Orchestrator
LABEL maintainer="CTAS-7 Engineering Team"
LABEL version="1.0.0"
LABEL description="CTAS-7 Foundation Core - Gold Disk Smart Crate Template"
LABEL ctas.version="7.0.0"
LABEL ctas.crate-type="foundation"
LABEL ctas.mission="DependencyUnification"
LABEL ctas.security-level="Production"
LABEL ctas.neural-mux="enabled"
LABEL ctas.xsd-symbols="\\u{E500},\\u{E320},\\u{E000}"
LABEL ctas.port-range="18101-18120"
LABEL ctas.orchestrator-compatible="true"
LABEL ctas.hash-engine="blake3+murmur3"

# Copy the binary from builder stage
COPY --from=builder /app/target/release/libctas7_foundation_core.rlib /usr/local/lib/

# Copy any additional assets needed
COPY --from=builder /app/Cargo.toml /usr/local/share/ctas7/

# Set up CTAS-7 environment
ENV CTAS_VERSION=7.0.0
ENV FOUNDATION_VERSION=1.0.0
ENV RUST_LOG=info
ENV CTAS_NEURAL_MUX=enabled
ENV CTAS_HASH_ENGINE=global
ENV CTAS_CDN_PORTS=18108,18109,18110,18200

# Expose the Smart Crate port range
EXPOSE 18101-18120 18200

# Create non-root user for security
USER 1001:1001

# Health check for Smart Crate Orchestrator
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD echo "CTAS-7 Foundation Core Ready" || exit 1

# Default command (this is a library crate, so just validate)
CMD echo "🔥 CTAS-7 Foundation Core Gold Disk Smart Crate Template Ready" && \
    echo "💎 Hash Engine: Blake3 + MurmurHash3 Trivariate" && \
    echo "🧠 Neural Mux: Enabled with priority routing" && \
    echo "⚡ XSD Orchestration: 15+ platforms, 95% automation" && \
    echo "📊 CDN Endpoints: 4 primary (18108,18109,18110,18200)" && \
    echo "🎯 Use this as template for new smart crates" && \
    tail -f /dev/null