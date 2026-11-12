# Local native checkpoint · 12 November 2026

Rust 1.95.0 locked workspace check, formatting, Clippy with warnings denied and 75 tests pass locally. Removes unreachable parked module declarations that made rustfmt search for nonexistent files, uses stable formatting settings, pins Rust/CI to 1.95.0 and applies Clippy suggestions without suppressing warnings. The parked ONNX modules remain unimplemented; no model download or inference is enabled. The existing LICENSE reserves all rights until a final licensing decision.

The exact accepted source is recorded by the accompanying PR and CI. Run `cargo fmt --all -- --check`, `cargo check --locked --workspace --all-targets`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, then `cargo test --locked --workspace`. Existing fixtures/tests cover the changed code; these checks do not establish live provider, model-quality, hosted authentication or analytical benefit.

This remains bounded research with a distinct contract. It is not promoted to a dependency of the current invited App. No deployment, paid service, automatic extraction or cross-application format conversion is enabled.
