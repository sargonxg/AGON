# Local native/security checkpoint · 12 November 2026

Original TACITUS code is now **Apache-2.0**, as explicitly selected by the repository owner. LICENSE, NOTICE and the preserved third-party copyright/license texts identify the boundary; model weights, evidence and externally authored material retain their own terms.

## Verified local scope

Rust 1.95.0 locked workspace/all-target checking, stable formatting and strict Clippy pass. The normal workspace suite passes 76 tests with 1 explicit database prerequisite skip, including the new embedded-migration parity test; the disposable-database test is explicitly opt-in and also passes against PostgreSQL 16. `cargo audit` reports zero vulnerabilities. `cargo deny --all-features check` passes advisories, bans, licenses and sources. The four previously recorded unmaintained-dependency warnings/exceptions remain visible; no new advisory ignore or license waiver was added.

The PostgreSQL contract applies the legacy directory migrations, stores a Unicode historical session, replays the embedded migrations twice and verifies identical versions/checksums and preserved source/summary/JSON. It uses a local disposable database; no hosted database or production migration is touched.

## What changed and why

SQLx's umbrella/macro resolution brought unused SQLite/MySQL packages into the lockfile audit. The storage crate now uses **exactly matched sqlx-core/sqlx-postgres 0.8.6** directly. The same query runtime and immutable SQL are retained; migration records use SQLx's own SHA-384 calculation. The public MigrationSource interface embeds the existing three SQL files without multi-driver macros. A directory-parity test fails if a future migration is not embedded. Because SQLx core/driver APIs are semver-exempt, any version update must keep their exact versions aligned and rerun this real database acceptance.

Unused PDF-parser dependencies were removed from the still-placeholder perception module; this does not remove working extraction. Parked ONNX modules remain unimplemented and no model download/inference is activated. Existing earlier formatting/Clippy repairs remain.

## Reproduce

```sh
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo audit
cargo deny --all-features check
# Set AGON_TEST_DATABASE_URL only to a disposable PostgreSQL database:
cargo test --locked -p aco-storage --test postgres_migrations -- --ignored
```

CI supplies the disposable PostgreSQL service and explicit integration-test invocation. Security-tool versions are pinned. Docker's Rust builder is aligned with the actual 1.95.0 toolchain and the runtime carries LICENSE, NOTICE and the third-party license text. A passing native suite alone is not a deployed service or a model-quality/customer-benefit result.

This remains independent bounded research, with no new App dependency, cross-application converter, provider activation or production deployment.

The actual release Docker image also builds and starts as `nonroot`, read-only with dropped capabilities and no network. A separate loopback probe in the same isolated namespace returns HTTP 200 `ok` in mock-fixture mode. LICENSE, NOTICE and all gathered dependency notices were copied out and matched to source SHA-256. No hosted deployment, registry publication or provider/model call was made. The source-bind BuildKit layout also avoids excessive copied layers on vfs builders.
