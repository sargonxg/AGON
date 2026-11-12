# syntax=docker/dockerfile:1.6
FROM rust:1.95.0-slim-bookworm AS builder
ARG CARGO_BUILD_JOBS=2
WORKDIR /build

# Read source through BuildKit rather than copying the entire Rust toolchain
# filesystem for every source layer on vfs builders. Source stays read-only;
# the locked build and immutable migration includes use /out for all outputs.
RUN --mount=type=bind,source=.,target=/source,readonly \
    --mount=type=secret,id=proxy_ca \
    set -eu; \
    if [ -f /run/secrets/proxy_ca ]; then \
      export CARGO_HTTP_CAINFO=/run/secrets/proxy_ca SSL_CERT_FILE=/run/secrets/proxy_ca CURL_CA_BUNDLE=/run/secrets/proxy_ca; \
    fi; \
    apt-get update; \
    apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates g++; \
    rm -rf /var/lib/apt/lists/*; \
    cargo build --locked --release --manifest-path /source/Cargo.toml --target-dir /out --bin agon-server --package aco-server; \
    install -m 0644 /source/LICENSE /source/NOTICE /source/THIRD_PARTY_LICENSES.txt /out/

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /out/release/agon-server /usr/local/bin/agon-server
COPY --from=builder /out/LICENSE /usr/share/licenses/agon/LICENSE
COPY --from=builder /out/NOTICE /usr/share/licenses/agon/NOTICE
COPY --from=builder /out/THIRD_PARTY_LICENSES.txt /usr/share/licenses/agon/THIRD_PARTY_LICENSES.txt
USER nonroot
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/agon-server"]
