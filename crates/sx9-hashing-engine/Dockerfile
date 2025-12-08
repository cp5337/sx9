# CTAS-7 Hashing Engine Microservice
FROM rust:1.75-slim as builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml .
COPY ctas7-foundation-core ../ctas7-foundation-core
COPY ctas7-hashing-engine .

RUN cargo build --release --bin hash-service

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false ctas7

WORKDIR /app

COPY --from=builder /build/target/release/hash-service /app/

RUN chown -R ctas7:ctas7 /app

USER ctas7

ENV RUST_LOG=info
ENV HASH_ENGINE_PORT=8002
ENV GRPC_PORT=50051

HEALTHCHECK --interval=20s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8002/health || exit 1

EXPOSE 8002 50051

CMD ["./hash-service"]

