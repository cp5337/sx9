FROM rust:1.75-alpine AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates curl

COPY --from=builder /app/target/release/ctas7-cdn-statistical /usr/local/bin/

EXPOSE 18108

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:18108/health || exit 1

CMD ["ctas7-cdn-statistical"]
