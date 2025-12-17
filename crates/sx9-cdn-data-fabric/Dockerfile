# GLAF CDN Data Fabric
FROM rust:1.75-bookworm as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY src ./src

# Copy dependencies
COPY ../ctas7-foundation-core ../ctas7-foundation-core

# Build
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cdn-data-fabric /usr/local/bin/

EXPOSE 18100

CMD ["cdn-data-fabric"]


