# AGON Build Plan

*This document is for Claude Code (or any senior Rust coding agent). It is a day-by-day, task-by-task action plan. Each task has acceptance criteria. Each day produces a verifiable deliverable. The order is the order.*

**Read [ARCHITECTURE.md](./ARCHITECTURE.md) first.** That document is the technical contract. This document is the schedule.

This v2 supersedes the previous build plan. The major changes vs v1: Postgres is set up on Day 2 (not deferred), perception and fusion are explicit days each, scoring and learning are first-class.

---

## How to use this document

- **You are building MVP v1.0.** That is Phase 1. Phases 2–4 are roadmap.
- **Three sprints of 7 days each.** 21 working days total.
- **Each task lists files to create, dependencies, and acceptance criteria.**
- **Do not skip acceptance criteria.** They are the contract.
- **Commit at the end of every task** using [Conventional Commits](https://www.conventionalcommits.org/).
- **CI must stay green.** If a task breaks CI, fix CI before moving on.
- **Use `MockLlmBackend` for tests that would burn Gemini quota.** Live API tests are gated behind `--features live-api`.
- **Use a local Postgres** (Docker compose ships with the repo) for tests that need a database. Tests run against a per-test schema.
- **Ask questions when ambiguous.** Better to surface a design question than fit a wrong assumption.

---

## Pre-Sprint: Repo bootstrap (Day 0)

| # | Task | Files | Acceptance |
|---|---|---|---|
| 0.1 | Cargo workspace init | `Cargo.toml`, `rust-toolchain.toml` | `cargo build` succeeds on empty workspace |
| 0.2 | Workspace shared deps | `Cargo.toml` `[workspace.dependencies]` | See Sprint 1 Day 1 for the canonical list |
| 0.3 | `.gitignore`, `.env.example`, `LICENSE` (placeholder), `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` | as named | All present |
| 0.4 | CI workflows | `.github/workflows/{ci,audit,bench,docker}.yml` | fmt, clippy `-D warnings`, test, audit, deny, doc all green on empty workspace |
| 0.5 | `rustfmt.toml`, `clippy.toml`, `deny.toml` | as named | `cargo fmt --check` and `cargo clippy -- -D warnings` pass |
| 0.6 | Empty crate skeletons (all 11 crates) | `crates/*/Cargo.toml`, `src/lib.rs` or `main.rs` | `cargo build` succeeds |
| 0.7 | Local dev compose | `compose.yaml` | `docker compose up` brings Postgres 16 + pgvector + (optional) Apache AGE |
| 0.8 | Tracing/logging setup | each crate's `lib.rs` | Each crate emits a `tracing` span on init |

### Canonical `Cargo.toml` workspace dependencies

```toml
[workspace.dependencies]
# Core
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-opentelemetry = "0.27"
thiserror = "1"
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
blake3 = "1"
dashmap = "6"
async-trait = "0.1"
futures = "0.3"
parking_lot = "0.12"
once_cell = "1"
uuid = { version = "1", features = ["v4", "serde"] }
secrecy = { version = "0.8", features = ["serde"] }
governor = "0.6"
bytes = "1"

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono", "json", "macros", "migrate"] }
pgvector = { version = "0.4", features = ["sqlx"] }

# Inference
ascent = "0.8"
z3 = { version = "0.20" }
good_lp = { version = "1", features = ["microlp"] }

# Storage
petgraph = "0.8"
arrow = "53"
parquet = "53"

# LLM
google-ai-rs = { version = "0.3" }
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# Embeddings
fastembed = "5"
hnsw_rs = "0.3"

# Server
axum = { version = "0.8", features = ["ws", "macros"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["trace", "cors", "compression-br", "fs"] }
rust-embed = "8"
jsonwebtoken = "9"
tera = "1"

# CLI
clap = { version = "4", features = ["derive", "env"] }
indicatif = "0.17"
console = "0.15"
dialoguer = "0.11"

# Document parsing
lopdf = "0.34"
docx-rs = "0.4"
pulldown-cmark = "0.12"

# Testing
proptest = "1"
insta = { version = "1", features = ["json", "yaml"] }
criterion = "0.5"
mockall = "0.13"
testcontainers = "0.23"
testcontainers-modules = { version = "0.11", features = ["postgres"] }

[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 1

[profile.bench]
inherits = "release"
debug = true
```

### `compose.yaml` (development Postgres)

```yaml
services:
  postgres:
    image: pgvector/pgvector:pg16
    environment:
      POSTGRES_USER: agon
      POSTGRES_PASSWORD: agon
      POSTGRES_DB: agon
    ports:
      - "5432:5432"
    volumes:
      - agon-pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U agon"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  agon-pgdata:
```

---

## Sprint 1 — Foundations (Days 1–7)

**Sprint goal:** A single document goes end-to-end through ingest → perceive → fuse → store → infer → query. Postgres is the source of truth. The CLI emits a friction score for at least one dyad on at least one fixture. By end of sprint, the demo skeleton exists.

### Day 1 — `aco-core`: types and provenance

| # | Task | Files | Acceptance |
|---|---|---|---|
| 1.1 | `Id` and canonical hashing | `crates/aco-core/src/id.rs` | `Id::from_canonical(&primitive)` is stable across runs |
| 1.2 | Common types: `EvidenceSpan`, `Provenance`, `Defeasibility`, `Derivation`, `TemporalInterval`, `Place` | `crates/aco-core/src/common.rs` | All serde-roundtrip in `proptest` |
| 1.3 | Eight ACO primitive structs | `crates/aco-core/src/{actor,claim,interest,constraint,leverage,commitment,event,narrative}.rs` | All serde-roundtrip; full field specs from ARCHITECTURE §4.1 |
| 1.4 | Interpersonal extensions | `crates/aco-core/src/patterns.rs` | `PatternFinding`, `PatternKind`, `AffectMarker`, `Emotion` complete |
| 1.5 | Supporting enums (`Modality`, `SpeechAct`, `LeverageKind`, `Deontic`, …) | `crates/aco-core/src/enums.rs` | All variants from ARCHITECTURE |
| 1.6 | FOL logical form (optional, populated later) | `crates/aco-core/src/fol.rs` | Compiles; tests for variant constructors |
| 1.7 | Error types | `crates/aco-core/src/error.rs` | `thiserror`-derived |
| 1.8 | `lib.rs` re-exports | `crates/aco-core/src/lib.rs` | All public; `cargo doc` no missing-doc warnings |
| 1.9 | Property tests | `crates/aco-core/tests/proptest_roundtrip.rs` | 1000 iters per primitive; pass |

**End-of-day:** `cargo test -p aco-core` passes; `cargo doc -p aco-core` builds without warnings.

### Day 2 — `aco-storage`: Postgres schema and connection

| # | Task | Files | Acceptance |
|---|---|---|---|
| 2.1 | Initial migration | `migrations/001_init.sql` (down + up) | Applies cleanly; rolls back cleanly |
| 2.2 | `Pool` and connection mgmt | `crates/aco-storage/src/pool.rs` | `sqlx` pool; configurable via `AGON_DATABASE_URL` |
| 2.3 | Repository per primitive | `crates/aco-storage/src/repo/{actor,claim,event,...}.rs` | CRUD per type; uses `sqlx::query!` with compile-time check (sqlx-cli prepared) |
| 2.4 | Provenance table writes | `crates/aco-storage/src/repo/provenance.rs` | Every primitive insert atomically writes provenance |
| 2.5 | Evidence span storage | `crates/aco-storage/src/repo/spans.rs` | Polymorphic FK; index by primitive_id |
| 2.6 | Edges table | `crates/aco-storage/src/repo/edges.rs` | Insert, query by from/to/kind |
| 2.7 | Audit log | `crates/aco-storage/src/audit.rs` | Append-only; every mutation logged |
| 2.8 | In-memory graph (petgraph) | `crates/aco-storage/src/memory.rs` | Hydrate from Postgres; sync on write |
| 2.9 | `LISTEN`/`NOTIFY` channel | `crates/aco-storage/src/notify.rs` | External changes trigger memory refresh |
| 2.10 | DB-level integration tests | `crates/aco-storage/tests/db.rs` | Uses `testcontainers::Postgres`; CRUD covered for every primitive |

**End-of-day:** `docker compose up` → `cargo test -p aco-storage` passes. Postgres has all tables and indices.

### Day 3 — `aco-llm`: Gemini + Mock

| # | Task | Files | Acceptance |
|---|---|---|---|
| 3.1 | `LlmBackend` trait | `crates/aco-llm/src/backend.rs` | Async-trait bounds correct |
| 3.2 | `GeminiBackend` | `crates/aco-llm/src/gemini.rs` | `--features live-api` integration test passes against real Gemini |
| 3.3 | `MockLlmBackend` with fixture replay | `crates/aco-llm/src/mock.rs` | Loads fixtures from `tests/fixtures/`; deterministic |
| 3.4 | Retry with `tokio-retry` or hand-rolled | `crates/aco-llm/src/retry.rs` | Retries transient; respects max attempts |
| 3.5 | Rate limiter via `governor` | `crates/aco-llm/src/rate.rs` | Configurable RPS shared across all extractors |
| 3.6 | Cost ledger | `crates/aco-llm/src/cost.rs` | Tracks tokens by model; computes USD |
| 3.7 | Response cache in Postgres | `crates/aco-llm/src/cache.rs` | Content-addressed by `(prompt_fp, schema_fp, content_fp)`; cache hit avoids API call |
| 3.8 | Embedding via fastembed | `crates/aco-embed/src/local.rs` | BAAI/bge-small-en-v1.5 default; configurable model |
| 3.9 | Tests | `crates/aco-{llm,embed}/tests/` | Unit + integration with mock and Postgres |

**End-of-day:** Mock-driven extraction returns structured JSON; embedding produces 384-dim vectors.

### Day 4 — `aco-perceive`: parallel extractors

| # | Task | Files | Acceptance |
|---|---|---|---|
| 4.1 | `Extractor` trait | `crates/aco-perceive/src/lib.rs` | Trait compiles; orchestrator can run a list of extractors in parallel |
| 4.2 | `EntityExtractor` + schema + prompt | `crates/aco-perceive/src/entity.rs`, `prompts/entity_v1.md`, `schemas/entity_v1.json` | Snapshot test on a fixture chunk |
| 4.3 | `EventExtractor` + schema + prompt | `crates/aco-perceive/src/event.rs`, `prompts/event_v1.md`, `schemas/event_v1.json` | Snapshot test |
| 4.4 | `ClaimExtractor` + schema + prompt | `crates/aco-perceive/src/claim.rs`, ... | Snapshot test |
| 4.5 | `AffectExtractor` + schema + prompt | `crates/aco-perceive/src/affect.rs`, ... | Snapshot test |
| 4.6 | `PatternExtractor` + schema + prompt | `crates/aco-perceive/src/pattern.rs`, ... | Snapshot test for Four Horsemen on a workplace-dispute fixture |
| 4.7 | `TemporalExtractor` + schema + prompt | `crates/aco-perceive/src/temporal.rs`, ... | Snapshot test |
| 4.8 | Parallel orchestrator | `crates/aco-perceive/src/orchestrator.rs` | Runs all extractors concurrently with shared rate limit |
| 4.9 | Verify-and-repair loop | `crates/aco-perceive/src/repair.rs` | On validation failure, builds repair prompt; max 2 repairs |
| 4.10 | Document loaders | `crates/aco-perceive/src/loaders/{txt,pdf,docx,md}.rs` | Load a 50-page PDF, returns chunks |

**End-of-day:** A 5-page text file produces non-empty `RawPerception` bundles from all six extractors.

### Day 5 — `aco-fuse`: the canonicalization layer

| # | Task | Files | Acceptance |
|---|---|---|---|
| 5.1 | Canonical hash signature | `crates/aco-fuse/src/signature.rs` | Per-primitive normalisation rules; deterministic |
| 5.2 | Entity normalisation rules | `crates/aco-fuse/src/normalize/entity.rs` | Strip honorifics, diacritics, whitespace; tested on edge cases |
| 5.3 | Event and claim normalisation rules | `crates/aco-fuse/src/normalize/{event,claim}.rs` | Snapshot tests |
| 5.4 | HNSW ANN index over embeddings | `crates/aco-fuse/src/ann.rs` | `hnsw_rs`; top-k cosine search |
| 5.5 | Entity resolver | `crates/aco-fuse/src/resolve/entity.rs` | Exact-hash + ANN + LLM tiebreaker; merges into canonical actor |
| 5.6 | Event coreference | `crates/aco-fuse/src/resolve/event.rs` | Temporal overlap + participant overlap + verb similarity |
| 5.7 | Claim dedup | `crates/aco-fuse/src/resolve/claim.rs` | Embedding similarity + speaker + interval |
| 5.8 | Temporal alignment | `crates/aco-fuse/src/temporal.rs` | Resolves relative times where doc has anchor |
| 5.9 | Confidence reconciliation | `crates/aco-fuse/src/confidence.rs` | Combines via `1 - prod(1-c_i)` |
| 5.10 | Alias graph storage | `crates/aco-storage/src/repo/aliases.rs` (extend) | Every merge writes to `actor_aliases` |
| 5.11 | End-to-end fusion test | `crates/aco-fuse/tests/e2e.rs` | 47 raw actors → ≤ 15 canonical actors on workplace-dispute fixture |

**End-of-day:** Running perception + fusion on a fixture produces canonical primitives stored in Postgres, with alias graph populated and dedup metrics emitted.

### Day 6 — `aco-infer` (first pass) + `aco-score` (first pass)

| # | Task | Files | Acceptance |
|---|---|---|---|
| 6.1 | Infer engine entry point | `crates/aco-infer/src/lib.rs` | `infer(graph) -> InferenceReport` compiles |
| 6.2 | First Datalog rules: leverage chains, gap detection | `crates/aco-infer/src/rules/{leverage,gaps}.rs` | Snapshot tests |
| 6.3 | Coalition graph rule | `crates/aco-infer/src/rules/coalition.rs` | Snapshot |
| 6.4 | Temporal rules (Allen relations) | `crates/aco-infer/src/rules/temporal.rs` | Snapshot |
| 6.5 | Score: friction (composite) | `crates/aco-score/src/friction.rs` | Returns score 0-100 with feature attribution |
| 6.6 | Score: power asymmetry | `crates/aco-score/src/power.rs` | Returns score [-1, +1] with leverage breakdown |
| 6.7 | Score: trust trajectory | `crates/aco-score/src/trust.rs` | Beta-distribution update; time-series in Postgres |
| 6.8 | Score writes to Postgres | `crates/aco-storage/src/repo/scores.rs` | Every score persisted with features and derivation |
| 6.9 | Tests | `crates/aco-{infer,score}/tests/` | Snapshot per rule and per score type |

**End-of-day:** A canonicalized graph produces inference facts and at least three scores per dyad.

### Day 7 — `aco-cli` MVP + first scenario

| # | Task | Files | Acceptance |
|---|---|---|---|
| 7.1 | CLI structure with `clap` | `crates/aco-cli/src/main.rs`, `cli.rs` | `agon --help` lists all v1 commands |
| 7.2 | `agon db init` | `crates/aco-cli/src/cmd/db.rs` | Creates database, applies migrations, optionally installs AGE |
| 7.3 | `agon ingest <PATH>` | `crates/aco-cli/src/cmd/ingest.rs` | Folder or file; stores documents and chunks |
| 7.4 | `agon perceive` | `crates/aco-cli/src/cmd/perceive.rs` | Runs all extractors with progress bars |
| 7.5 | `agon fuse` | `crates/aco-cli/src/cmd/fuse.rs` | Reports merge stats |
| 7.6 | `agon think` | `crates/aco-cli/src/cmd/think.rs` | Inference + scoring |
| 7.7 | `agon stats` | `crates/aco-cli/src/cmd/stats.rs` | Counts primitives, edges, scores, costs |
| 7.8 | `agon ingest+perceive+fuse+think` mega-command (`agon run`) | `crates/aco-cli/src/cmd/run.rs` | Full pipeline in one command |
| 7.9 | First scenario corpus | `corpora/workplace_dispute/` | Synthetic but realistic; 30 pages |
| 7.10 | Snapshot expected outputs | `corpora/workplace_dispute/expected_*.json` | Diff-able |
| 7.11 | End-to-end integration test | `crates/aco-cli/tests/workplace_dispute.rs` | Mocked Gemini; asserts snapshots |

**End-of-sprint check:**
- `cargo test --all` green
- `agon run corpora/workplace_dispute/` produces a canonicalized world model with at least one Friction Score
- The first internal demo recording (90-second clip) exists

---

## Sprint 2 — Deep inference, scoring, dashboard (Days 8–14)

**Sprint goal:** All five wow moments work end-to-end. The dashboard renders the live graph, the friction heatmap, the contradiction view, the DARVO sequence drill-down, and the brief generator. By end of sprint the demo video is recordable.

### Day 8 — Defeasible reasoning (ASPIC+)

| # | Task | Files | Acceptance |
|---|---|---|---|
| 8.1 | Defeasible rule registry | `crates/aco-infer/src/rules/defeasible/mod.rs` | Per-rule priority lattice |
| 8.2 | Rebut, undercut, undermine | `crates/aco-infer/src/rules/defeasible/attacks.rs` | Snapshot tests on toy theories |
| 8.3 | Grounded extension | `crates/aco-infer/src/rules/defeasible/grounded.rs` | Equivalent to ASPIC+ on Lam et al. 2016 examples |
| 8.4 | Integration with gap detection | `crates/aco-infer/src/rules/defeasible/integrate.rs` | Defeasible gaps can be overridden by stronger rules |

### Day 9 — Z3 contradiction detection

| # | Task | Files | Acceptance |
|---|---|---|---|
| 9.1 | FOL → Z3 encoder | `crates/aco-infer/src/contradict/encode.rs` | All FOL variants encoded |
| 9.2 | Per-actor consistency | `crates/aco-infer/src/contradict/actor.rs` | Returns unsat-core with correct claim IDs |
| 9.3 | Deontic conflict | `crates/aco-infer/src/contradict/deontic.rs` | Constraint(Prohibition, P) + Commitment(P) → conflict |
| 9.4 | Optional FOL extraction prompt | `prompts/fol_v1.md`, `crates/aco-perceive/src/fol.rs` | `--extract-fol` populates `Claim.logical_form` |
| 9.5 | CLI command | `crates/aco-cli/src/cmd/contradict.rs` | `agon contradict --actor "<name>"` |
| 9.6 | Snapshot tests | `crates/aco-infer/tests/contradict.rs` | 2 unsat, 1 sat |

### Day 10 — Patterns + remaining scores

| # | Task | Files | Acceptance |
|---|---|---|---|
| 10.1 | Rule-based DARVO detector | `crates/aco-infer/src/rules/patterns/darvo.rs` | Snapshot on a DARVO fixture |
| 10.2 | Rule-based Four Horsemen detector | `crates/aco-infer/src/rules/patterns/horsemen.rs` | Snapshot per horseman |
| 10.3 | Pattern reconciliation (extractor vs rule) | `crates/aco-infer/src/rules/patterns/reconcile.rs` | Cross-check produces high-confidence findings |
| 10.4 | Score: repair capacity | `crates/aco-score/src/repair.rs` | Ratio of repair attempts to escalations |
| 10.5 | Score: bid-turn ratio | `crates/aco-score/src/bid_turn.rs` | Snapshot |
| 10.6 | Score: toxicity index | `crates/aco-score/src/toxicity.rs` | Per-actor counts |
| 10.7 | Score: risk (logistic on friction features) | `crates/aco-score/src/risk.rs` | Coefficients seeded from literature; produces calibrated 0-100 |

### Day 11 — BATNA/ZOPA + abduction loop

| # | Task | Files | Acceptance |
|---|---|---|---|
| 11.1 | `NegotiationModel` and `Issue` types | `crates/aco-infer/src/negotiation/model.rs` | Constructable from a dyad |
| 11.2 | Utility-proxy estimator | `crates/aco-infer/src/negotiation/utility.rs` | Ridge regression on synthetic data |
| 11.3 | ZOPA feasibility LP | `crates/aco-infer/src/negotiation/zopa.rs` | `good_lp` + SCIP; feasibility + infeasibility cases |
| 11.4 | Mediation move (max-min) | `crates/aco-infer/src/negotiation/mediation.rs` | Returns highest-impact axis |
| 11.5 | `Gap` enum + typed prompts | `crates/aco-infer/src/abduction/{gap,prompt}.rs`, `prompts/abduction/*.md` | All 6 gap types covered |
| 11.6 | Candidate ranking | `crates/aco-infer/src/abduction/rank.rs` | Confidence + Z3-consistency + embedding-coherence |
| 11.7 | Defeasible re-injection + cycle guard | `crates/aco-infer/src/abduction/orchestrator.rs` | Stops at `AGON_ABDUCTION_MAX_CYCLES` |
| 11.8 | CLI: `agon mediate --dyad` | `crates/aco-cli/src/cmd/mediate.rs` | Outputs ZOPA result |

### Day 12 — Server + WebSocket + dashboard skeleton

| # | Task | Files | Acceptance |
|---|---|---|---|
| 12.1 | Axum router and state | `crates/aco-server/src/lib.rs`, `state.rs` | Server starts; GET `/` returns embedded HTML |
| 12.2 | REST endpoints | `crates/aco-server/src/api/{graph,query,run,brief}.rs` | Each returns JSON; integration tests via `reqwest` |
| 12.3 | WebSocket endpoint | `crates/aco-server/src/ws.rs` | Streams `WorldEvent`s |
| 12.4 | Static front-end shell | `crates/aco-server/assets/{index.html,app.js,styles.css}` | Cytoscape graph + d3 timeline + heatmap grid + score cards |
| 12.5 | Embed via `rust-embed` | `crates/aco-server/src/embed.rs` | Assets in the binary |
| 12.6 | CORS, JWT, OTel | `crates/aco-server/src/middleware.rs` | Allowlist; JWT validates; tracing exports |
| 12.7 | `agon serve` and `agon viz --open` | `crates/aco-cli/src/cmd/{serve,viz}.rs` | Opens browser at the right URL |

### Day 13 — Dashboard polish + brief generator

| # | Task | Files | Acceptance |
|---|---|---|---|
| 13.1 | Friction heatmap rendering | `crates/aco-server/assets/heatmap.js` | Actor × actor matrix; click cells |
| 13.2 | Score cards with drill-down | `crates/aco-server/assets/cards.js` | Click any score → feature attribution + primitives |
| 13.3 | Evidence pane | `crates/aco-server/assets/evidence.js` | Verbatim quotes from all sources for any selected primitive |
| 13.4 | Pattern timeline overlay | `crates/aco-server/assets/timeline.js` | DARVO/horsemen episodes highlighted |
| 13.5 | Tera brief templates | `prompts/briefs/{mediator_prep,legal,hr,therapist_prep,exec}.tera` | Render from graph |
| 13.6 | Brief renderer | `crates/aco-cli/src/cmd/brief.rs` | Generates markdown brief with inline citations |
| 13.7 | First demo recording (internal) | `RECORDINGS.md` | All five wow moments captured |

### Day 14 — Ask mode + remaining pragmatics

| # | Task | Files | Acceptance |
|---|---|---|---|
| 14.1 | Query AST | `crates/aco-cli/src/ask/ast.rs` | Typed query representation |
| 14.2 | Gemini → AST translator | `crates/aco-cli/src/ask/translate.rs` | Constrained-output Gemini call |
| 14.3 | AST → SQL compiler | `crates/aco-cli/src/ask/compile.rs` | Safe parameterised SQL |
| 14.4 | Result renderer with citations | `crates/aco-cli/src/ask/render.rs` | Click-through to spans |
| 14.5 | CLI `agon ask "<question>"` | `crates/aco-cli/src/cmd/ask.rs` | Works end-to-end |
| 14.6 | Pragmatics: scalar implicature table | `crates/aco-infer/src/rules/pragmatics/scalar.rs` | ≥80 entries |
| 14.7 | Pragmatics: evasion, non-sequitur, frame conflict | `crates/aco-infer/src/rules/pragmatics/*.rs` | Snapshots |
| 14.8 | Silence analysis | `crates/aco-infer/src/rules/silence.rs` | Snapshot |

**End-of-sprint check:**
- Dashboard renders all views
- Five wow moments work
- Brief generation produces a valid mediator-prep markdown
- A 5–10 minute internal demo recording exists

---

## Sprint 3 — Learning, hardening, release (Days 15–21)

### Day 15 — Learning loop

| # | Task | Files | Acceptance |
|---|---|---|---|
| 15.1 | Correction log schema and writes | `migrations/002_corrections.sql`, `crates/aco-storage/src/repo/corrections.rs` | Postgres table; writes via `aco-learn` |
| 15.2 | Active learning queue | `crates/aco-learn/src/queue.rs` | Sorted by potential impact |
| 15.3 | `agon review` CLI | `crates/aco-cli/src/cmd/review.rs` | Surfaces low-confidence items for labeling |
| 15.4 | Dashboard correction UI | `crates/aco-server/assets/review.js` | User can mark wrong-merge, wrong-pattern, wrong-extraction; correction logged via API |
| 15.5 | Retraining trigger | `crates/aco-learn/src/trigger.rs` | Emits `RetrainingTrigger` when threshold crossed |
| 15.6 | Prompt-version pinning | `prompts/manifest.json` | Tracks deployed versions for every extractor |
| 15.7 | Few-shot example bank | `crates/aco-learn/src/few_shot.rs` | Aggregates corrections into examples; pins into next prompt |

### Day 16 — Performance

| # | Task | Files | Acceptance |
|---|---|---|---|
| 16.1 | Parallelise hot Datalog rules with `ascent_par!` | `crates/aco-infer/src/rules/*.rs` | 50k-primitive closure ≤ 600 ms |
| 16.2 | Profile extraction wall time | `crates/aco-bench/benches/extract.rs` | Fix 3 slowest hot paths |
| 16.3 | Tune Gemini context caching | `crates/aco-llm/src/cache.rs` | 90% cache hit on second run |
| 16.4 | Reduce allocations in fusion | `crates/aco-fuse/src/*.rs` | criterion ≥ 10% improvement |
| 16.5 | Postgres index review | `migrations/` | All hot queries use indices; `EXPLAIN ANALYZE` documented |
| 16.6 | Connection pool tuning | `crates/aco-storage/src/pool.rs` | Right size for typical workload |
| 16.7 | Document numbers | `docs/src/performance.md` | All targets from ARCHITECTURE §12 measured |

### Day 17 — Robustness

| # | Task | Files | Acceptance |
|---|---|---|---|
| 17.1 | Property tests across rules | `crates/aco-infer/tests/prop_*.rs` | Monotonicity, termination |
| 17.2 | Fuzz JSON-schema repair | `crates/aco-perceive/fuzz/` | No panics in 10M iters |
| 17.3 | Fuzz PDF loader | `crates/aco-perceive/fuzz/pdf.rs` | No panics on malformed PDFs |
| 17.4 | Timeout + cancellation audit | all async paths | Every long-running task respects cancellation |
| 17.5 | Prompt-injection defence | system prompts, validators | Doc text explicitly framed as data, not instructions |
| 17.6 | Migration roundtrip tests | `migrations/` | Up + down + up returns to same schema |

### Day 18 — Five golden corpora

| # | Task | Files | Acceptance |
|---|---|---|---|
| 18.1 | Workplace dispute (done Day 7) | `corpora/workplace_dispute/` | snapshot stable |
| 18.2 | Co-parenting / divorce dyad | `corpora/coparenting/` | Friction trajectory + Four Horsemen |
| 18.3 | HR investigation case | `corpora/hr_investigation/` | DARVO + power asymmetry |
| 18.4 | Mediation case (multi-party) | `corpora/mediation_treaty/` | BATNA/ZOPA + coalition |
| 18.5 | Diplomatic transcript | `corpora/diplomatic/` | Implicature + contradiction |
| 18.6 | Per-corpus READMEs | `corpora/*/README.md` | What to look for and how to record |

### Day 19 — Documentation

| # | Task | Files | Acceptance |
|---|---|---|---|
| 19.1 | mdbook setup | `docs/book.toml`, `docs/src/SUMMARY.md` | `mdbook build` produces site |
| 19.2 | Conceptual guide | `docs/src/concepts/{aco,inference,provenance,scoring,patterns}.md` | Each ≥800 words |
| 19.3 | Tutorial | `docs/src/tutorial.md` | Quickstart walkthrough with screenshots |
| 19.4 | Cookbook per scenario | `docs/src/cookbook/*.md` | One per corpus |
| 19.5 | API docs polish | `cargo doc` | 100% public-item coverage |
| 19.6 | Runnable examples | `examples/*.rs` | ≥3 examples; `cargo run --example` works |

### Day 20 — Deployment + security

| # | Task | Files | Acceptance |
|---|---|---|---|
| 20.1 | Dockerfile multi-stage | `Dockerfile` | Image ≤ 250 MB |
| 20.2 | Production compose (with managed Postgres reference) | `deploy/compose.yaml`, `deploy/README.md` | Documented |
| 20.3 | Fly.io recipe | `deploy/fly.toml`, README | Step-by-step |
| 20.4 | Cloud Run recipe | `deploy/cloudrun/` | Step-by-step |
| 20.5 | OTel collector config | `deploy/otel-collector.yaml` | Configurable endpoint |
| 20.6 | Production checklist | `deploy/CHECKLIST.md` | Secrets, scaling, health checks |
| 20.7 | `cargo audit` + `cargo deny check` | CI | No criticals |
| 20.8 | Secret-handling audit | all crates | `secrecy::SecretString` everywhere; no secrets logged |
| 20.9 | JWT validation tests | `crates/aco-server/tests/auth.rs` | Reject expired/invalid/missing |
| 20.10 | Threat model | `docs/src/security/threat-model.md` | Prompt injection, data exfiltration, DoS |

### Day 21 — Release and final demo

| # | Task | Files | Acceptance |
|---|---|---|---|
| 21.1 | Tag `v0.1.0` | git | Tag pushed |
| 21.2 | `CHANGELOG.md` + GitHub release notes | as named | Lists capabilities and limits |
| 21.3 | Final demo video | recording | 5–15 minutes; full pipeline + 5 wow moments + brief + learning |
| 21.4 | tacitus.me embed snippet | `docs/src/embed.md` | iframe or `<script>` |
| 21.5 | Public README badges | `README.md` | Build status, version, docs |
| 21.6 | Internal launch note | `LAUNCH.md` | Investor/advisor summary |

**End-of-sprint check:**
- `cargo test --all-features` green
- `cargo bench` within performance targets
- 0.1.0 tagged
- Final demo video exists
- Repo is presentable

---

## Demo recording protocol — the production workflow

This is the exact sequence to record for the launch video. See [README.md §MVP Demo Recording](./README.md#the-mvp-demo-recording--the-production-workflow) for the narrated version.

### Pre-flight
1. `cargo build --release`
2. `docker compose up -d` (Postgres ready)
3. `agon db init` (fresh schema)
4. `corpora/demo_workplace_dispute/` ready
5. `agon serve --port 7878` in background
6. Browser at `http://localhost:7878`
7. Gemini calls pre-cached: run scenario once before recording
8. OBS Studio 1920×1080@60fps, scenes for terminal / browser / split
9. Terminal font ≥ 16pt, dark theme

### Take 1 — full pipeline (90 seconds of compute)
1. `agon ingest corpora/demo_workplace_dispute/`
2. `agon perceive --concurrency 8 --verbose`
3. (cut to browser; graph grows live)
4. `agon fuse --explain`
5. `agon think --explain`
6. (browser dashboard pulses as scores compute)
7. **Wow 1 — Friction heatmap drill-down.** Click hottest dyad → score breakdown → click feature → spans
8. **Wow 2 — Contradiction across time.** Click contradiction badge → two quotes side by side
9. **Wow 3 — DARVO sequence.** Pattern card → 4-turn sequence in source docs
10. **Wow 4 — Unbacked commitment + abduction.** Gap card → three abduced candidate interests
11. **Wow 5 — Risk trajectory.** Forward 30-day projection with feature attribution
12. `agon brief --style mediator-prep --dyad "<a>" "<b>" --out brief.md`
13. `bat brief.md`
14. Dashboard correction: mark a low-confidence pattern as wrong → `agon learn report`
15. `agon stats`

### Take 2 — cinematic shots
- Graph growing live (60 s slow zoom)
- Heatmap colour transitioning (30 s)
- Contradiction quotes side by side (15 s)

### Post
- Voice-over recorded separately, synced in DaVinci Resolve
- Cut to 5–15 minutes
- Export 1080p H.264 for tacitus.me; 4K master archived

---

## Definition of done — MVP v0.1.0

All of these must be true:

- [ ] `cargo test --all-features` green on Linux, macOS, Windows CI
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` green
- [ ] `cargo audit` + `cargo deny check` green
- [ ] `cargo doc --no-deps` builds with no missing-doc warnings
- [ ] All five golden corpora produce stable, snapshot-equal outputs
- [ ] All performance targets from ARCHITECTURE §12 either met or documented as deferred
- [ ] `agon` installable via `cargo install --path crates/aco-cli`
- [ ] `agon db init` automates Postgres bootstrap
- [ ] `agon run <folder>` works end-to-end on any of the corpora
- [ ] Dashboard renders all five views, updates over WebSocket
- [ ] Brief generation works for all four templates
- [ ] `agon ask` works on at least 10 reference questions
- [ ] Correction logging and active learning queue work
- [ ] Docker image builds and runs
- [ ] At least one cloud deploy recipe verified end-to-end
- [ ] mdbook documentation complete and published
- [ ] 5–15 minute demo video exists and is embeddable on tacitus.me
- [ ] LICENSE committed (or decision documented to delay)
- [ ] `CHANGELOG.md` records 0.1.0
- [ ] `LAUNCH.md` drafted

---

## Post-MVP

After v0.1.0, the roadmap continues with Phases 2–4 from [README.md](./README.md#roadmap--beyond-the-mvp). Each phase will get its own BUILDPLAN. The architecture is designed so that:

- **Phase 2** adds `aco-predict`, `aco-counterfactual` crates without modifying core
- **Phase 3** adds `aco-watch`, `aco-strategist`, `aco-anomaly` crates
- **Phase 4** adds multi-tenancy at the Postgres schema layer, GraphQL API at server layer, differential dataflow under inference, Scallop FFI for probabilistic reasoning, multi-lingual prompts

None of these require breaking changes to `aco-core` or `aco-storage`. The ontology and the schema are stable. That stability is the moat.

---

## Notes for the building agent

- **When in doubt, ask.** Posting "this is ambiguous in §X, here is what I assume, OK?" is faster than rebuilding.
- **Commit small.** One concept per commit. Git history is part of the deliverable.
- **Don't optimise prematurely.** Make it correct, then make it fast. Day 16 is for performance.
- **Use the type system.** If the compiler can prove an invariant, do not write a runtime check.
- **Every public function gets a doc comment** with an example where possible.
- **`unsafe` is forbidden.** If you need it, raise a design question first.
- **Cite when you encode literature.** Comments like `// Gottman & Levenson 2000, the "magic ratio"` are mandatory in `aco-score/src/`, `aco-infer/src/rules/patterns/`, `aco-infer/src/rules/defeasible/`, `aco-infer/src/rules/temporal/`.
- **Treat Postgres as a first-class concern.** Every migration is reviewed. Every query goes through `sqlx::query!` or `query_as!` for compile-time checking.
- **Treat the dashboard as a first-class concern.** It is the demo. It is what people see. The codebase should reflect that.

Build the substrate. Drive through text. Ship it.

— end of plan —
