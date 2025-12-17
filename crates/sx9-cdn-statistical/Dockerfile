# CTAS-7 Statistical Analysis CDN v7.3.1
# Multi-stage build producing a slim runtime image

FROM rust:1.83-slim AS builder
WORKDIR /workspace

# Cache dependencies by copying manifests first
COPY Cargo.toml Cargo.lock ./
COPY ctas7-cdn-statistical/Cargo.toml ctas7-cdn-statistical/Cargo.toml

# Copy the entire workspace (needed for path dependencies)
COPY . .

# Build only the statistical CDN binary
RUN cargo build --release -p ctas7-cdn-statistical

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info
WORKDIR /app
COPY --from=builder /workspace/target/release/ctas7-cdn-statistical /usr/local/bin/ctas7-cdn-statistical

EXPOSE 18108
HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
  CMD curl -fsS http://localhost:18108/health || exit 1

CMD ["/usr/local/bin/ctas7-cdn-statistical"]
