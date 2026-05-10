# Contributing to AGON

Thanks for your interest. Until v0.1.0 ships, AGON is in **active solo build**.

## Process
- Read `ARCHITECTURE.md` first, then `BUILDPLAN.md`.
- Each commit follows [Conventional Commits](https://www.conventionalcommits.org/).
- `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test --workspace` must pass.
- `unsafe` is forbidden by crate attribute. Raise a design issue if you need it.

## Dev loop
```
docker compose up -d                     # Postgres + pgvector
cargo build
cargo test --workspace
cargo install --path crates/aco-cli      # `agon` on PATH
agon db init
```

## Reporting issues
File an issue at <https://github.com/sargonxg/AGON/issues> with a minimal repro.
