# AGON Build Ledger

**MVP target:** Sprint 1 (Days 1-7) — first end-to-end pipeline on workplace dispute corpus.
**Started:** 2026-05-10
**Mode:** Full autopilot to MVP done.

Legend: ☐ todo · ◐ in-progress · ✓ done · ✗ blocked

## Day 0 — Pre-sprint bootstrap

- ✓ 0.1 Cargo workspace init
- ✓ 0.2 Workspace shared deps in Cargo.toml
- ✓ 0.3 .gitignore, .env.example, LICENSE placeholder, CODE_OF_CONDUCT, CONTRIBUTING
- ✓ 0.4 CI workflows (ci/audit/bench/docker)
- ✓ 0.5 rustfmt.toml, clippy.toml, deny.toml
- ✓ 0.6 11 empty crate skeletons
- ✓ 0.7 compose.yaml (Postgres 16 + pgvector)
- ✓ 0.8 tracing/logging init per crate
- ◐ Day 0 build validation (cargo check running, bg: b3mk3nt7r)

## Day 1 — aco-core: types + provenance
- ☐ 1.1 Id + canonical hashing
- ☐ 1.2 Common types (EvidenceSpan, Provenance, Defeasibility, Derivation, TemporalInterval, Place)
- ☐ 1.3 8 ACO primitives
- ☐ 1.4 Interpersonal extensions (PatternFinding, AffectMarker, Emotion)
- ☐ 1.5 Supporting enums
- ☐ 1.6 FOL logical form
- ☐ 1.7 Error types
- ☐ 1.8 lib.rs re-exports
- ☐ 1.9 Property tests (1000 iters)

## Day 2 — aco-storage: Postgres
- ☐ 2.1 Initial migration (001_init.sql up+down)
- ☐ 2.2 Pool + connection mgmt
- ☐ 2.3 Repo per primitive
- ☐ 2.4 Provenance table writes
- ☐ 2.5 Evidence span storage
- ☐ 2.6 Edges table
- ☐ 2.7 Audit log
- ☐ 2.8 In-memory petgraph + hydration
- ☐ 2.9 LISTEN/NOTIFY channel
- ☐ 2.10 testcontainers integration tests

## Day 3 — aco-llm + aco-embed
- ☐ 3.1 LlmBackend trait
- ☐ 3.2 GeminiBackend (gated live-api)
- ☐ 3.3 MockLlmBackend (fixture replay)
- ☐ 3.4 Retry
- ☐ 3.5 Rate limiter (governor)
- ☐ 3.6 Cost ledger
- ☐ 3.7 Postgres response cache
- ☐ 3.8 fastembed embedding
- ☐ 3.9 Tests

## Day 4 — aco-perceive: extractors
- ☐ 4.1 Extractor trait
- ☐ 4.2 EntityExtractor + schema + prompt
- ☐ 4.3 EventExtractor
- ☐ 4.4 ClaimExtractor
- ☐ 4.5 AffectExtractor
- ☐ 4.6 PatternExtractor
- ☐ 4.7 TemporalExtractor
- ☐ 4.8 Parallel orchestrator
- ☐ 4.9 Verify-and-repair
- ☐ 4.10 Document loaders (txt/pdf/docx/md)

## Day 5 — aco-fuse: canonicalization
- ☐ 5.1 Canonical hash signature
- ☐ 5.2 Entity normalisation
- ☐ 5.3 Event + claim normalisation
- ☐ 5.4 HNSW ANN
- ☐ 5.5 Entity resolver
- ☐ 5.6 Event coreference
- ☐ 5.7 Claim dedup
- ☐ 5.8 Temporal alignment
- ☐ 5.9 Confidence reconciliation
- ☐ 5.10 Alias graph storage
- ☐ 5.11 E2E fusion test

## Day 6 — aco-infer + aco-score (first pass)
- ☐ 6.1 Infer engine entry point
- ☐ 6.2 Datalog rules: leverage, gaps
- ☐ 6.3 Coalition rule
- ☐ 6.4 Temporal rules (Allen)
- ☐ 6.5 Friction score
- ☐ 6.6 Power asymmetry
- ☐ 6.7 Trust trajectory
- ☐ 6.8 Score persistence
- ☐ 6.9 Tests

## Day 7 — aco-cli + first scenario
- ☐ 7.1 CLI with clap
- ☐ 7.2 `agon db init`
- ☐ 7.3 `agon ingest`
- ☐ 7.4 `agon perceive`
- ☐ 7.5 `agon fuse`
- ☐ 7.6 `agon think`
- ☐ 7.7 `agon stats`
- ☐ 7.8 `agon run` mega-command
- ☐ 7.9 workplace_dispute corpus
- ☐ 7.10 Expected snapshots
- ☐ 7.11 E2E integration test

## Sprint 1 exit criteria
- ☐ cargo test --all green
- ☐ `agon run corpora/workplace_dispute/` produces canonical world model + ≥1 Friction Score
- ☐ Internal 90s demo clip exists

## Blockers / notes
- 2026-05-10: Rust toolchain installing via winget (background task bsacd2soy)
- 2026-05-10: Docker Desktop not installed. Postgres options: (a) winget PostgreSQL.PostgreSQL native, (b) Docker Desktop later, (c) defer Day 2 testcontainers tests
- Gemini key in .env (NOT committed). Rotate after MVP done.
