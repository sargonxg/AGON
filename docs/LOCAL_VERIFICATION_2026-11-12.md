# Local native checkpoint · 12 November 2026

Rust 1.95.0 locked workspace check, formatting, Clippy with warnings denied and 75 tests pass locally. Removes unreachable parked module declarations that made rustfmt search for nonexistent files, uses stable formatting settings, pins Rust/CI to 1.95.0 and applies Clippy suggestions without suppressing warnings. The parked ONNX modules remain unimplemented; no model download or inference is enabled. The existing LICENSE reserves all rights until a final licensing decision.

The exact candidate source is recorded by the accompanying PR; local checks are distinct from any remote CI. Run `cargo fmt --all -- --check`, `cargo check --locked --workspace --all-targets`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, then `cargo test --locked --workspace`. Existing fixtures/tests cover the changed code; these checks do not establish live provider, model-quality, hosted authentication or analytical benefit.

This remains bounded research with a distinct contract. It is not promoted to a dependency of the current invited App. No deployment, paid service, automatic extraction or cross-application format conversion is enabled.

## Dependency-security follow-up: release gate remains open

The resolved Cargo lockfile audit was refreshed rather than relying on the older August CI failure. Compatible security updates and removal of unused Google AI, DOCX, SMT and container-test dependencies reduce the audit from 26 advisory entries to two. PostgreSQL SQLx defaults are explicit; parser code is not activated by the updated lopdf dependency. Existing neural inference remains unimplemented/opt-in. Updated workspace compilation, strict Clippy and all 75 tests pass locally.

Remaining lockfile advisories: [libsqlite3-sys / RUSTSEC-2026-0459](https://rustsec.org/advisories/RUSTSEC-2026-0459.html) and [rsa / RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071.html), associated with the resolved SQLx backend graph. Do not treat the default Linux PostgreSQL build as proof that every resolved optional backend is safe. A dependency/DB contract upgrade requires its own tested acceptance.

The all-feature `cargo deny check` still fails on the repository's deliberately undecided, all-rights-reserved license and the unmaintained transitive ttf-parser dependency. Other unmaintained dependency warnings remain recorded. No new advisory exception or third-party licensing waiver was added. MIT-0 is explicitly recognized as a permissive upstream license; the repository's own LICENSE is unchanged and does not grant redistribution.

This PR remains a candidate pending those release decisions. It is not a production-ready service, an App dependency, a verified database deployment or permission to redistribute. Keep the audit gates visible; do not replace them with the passing software-test count.
