# CTAS-7 AXON (Plasma Processing Engine) v7.3.1
# Processes Wazuh alerts, applies trivariate hashing, generates USIMs

FROM rust:1.83-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy AXON source (build context: workspace root)
COPY ctas6-reference/docker-builds/axon/Cargo.toml ./Cargo.toml
COPY ctas6-reference/docker-builds/axon/src ./src

# Build release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /build/target/release/ctas7-axon /app/

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:15176/health || exit 1

EXPOSE 15176

CMD ["/app/ctas7-axon"]

