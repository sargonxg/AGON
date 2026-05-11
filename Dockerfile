# syntax=docker/dockerfile:1.6
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace + only the crates the binary needs (minimal closure).
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY migrations ./migrations

# Cache deps then build agon-server release.
RUN cargo build --release --bin agon-server \
    --package aco-server

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /app/target/release/agon-server /usr/local/bin/agon-server

USER nonroot
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/agon-server"]
