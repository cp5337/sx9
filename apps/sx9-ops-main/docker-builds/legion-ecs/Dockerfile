# CTAS-7 Legion ECS (Entity Component System) v7.3.1
# Tracks entities (threats, assets, agents) with trivariate hash addressing

FROM rust:1.83-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Legion source
COPY docker-builds/legion-ecs/Cargo.toml ./Cargo.toml
COPY docker-builds/legion-ecs/src ./src

# Copy foundation crates
COPY ctas7-foundation-core ./ctas7-foundation-core
COPY ctas7-foundation-math ./ctas7-foundation-math

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
COPY --from=builder /build/target/release/ctas7-legion-ecs /app/

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:15177/health || exit 1

EXPOSE 15177

CMD ["/app/ctas7-legion-ecs"]

