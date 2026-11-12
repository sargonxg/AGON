# syntax=docker/dockerfile:1.6
FROM rust:1.95.0-slim-bookworm AS builder

WORKDIR /app
ARG CARGO_BUILD_JOBS=2

RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config libssl-dev ca-certificates g++ \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace + only the crates the binary needs (minimal closure).
COPY Cargo.toml Cargo.lock rust-toolchain.toml LICENSE NOTICE THIRD_PARTY_LICENSES.txt ./
COPY crates ./crates
COPY migrations ./migrations

# Cache deps then build agon-server release.
RUN --mount=type=secret,id=proxy_ca \
    if [ -f /run/secrets/proxy_ca ]; then \
      export CARGO_HTTP_CAINFO=/run/secrets/proxy_ca SSL_CERT_FILE=/run/secrets/proxy_ca CURL_CA_BUNDLE=/run/secrets/proxy_ca; \
    fi; \
    cargo build --locked --release --bin agon-server --package aco-server

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /app/target/release/agon-server /usr/local/bin/agon-server

COPY --from=builder /app/LICENSE /usr/share/licenses/agon/LICENSE
COPY --from=builder /app/NOTICE /usr/share/licenses/agon/NOTICE
COPY --from=builder /app/THIRD_PARTY_LICENSES.txt /usr/share/licenses/agon/THIRD_PARTY_LICENSES.txt

USER nonroot
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/agon-server"]
