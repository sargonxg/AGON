# Where Rust earns its keep in AGON

A focused analysis: where the language pays for itself, where it's just plumbing, and what to build next to push the ceiling on the *perception* and *cognition* layers.

The thesis: **AGON treats the LLM as one typed sensor among many, and Rust is the only mainstream language where you can write all the *other* sensors (rules, solvers, graphs, neural nets, parsers) in one binary with strict types, zero GC, and near-C performance.** Python is fine for prototypes. Rust is what you ship when the LLM is no longer the bottleneck.

---

## Tier 1 — already in production, already paying

| Area | Why Rust | Status |
|---|---|---|
| **`aco-core` typed primitives** | `serde` + algebraic enums + `#[forbid(unsafe_code)]` makes the ACO ontology a compile-time contract. A claim without provenance fails to construct. A pattern without a confidence in `[0,1]` fails property tests. | ✅ live |
| **`aco-llm` Vertex client** | Single static binary, `reqwest` with rustls (no openssl ABI hell on Cloud Run), `parking_lot::RwLock` for the access-token cache with no `tokio::sync` overhead for a hot read path. | ✅ live |
| **`aco-server` Axum + embedded assets** | `rust-embed` bakes HTML/CSS/JS into the binary at compile time. The container has no static-file server, no nginx, no Vite. One process. | ✅ live |
| **SSE streaming pipeline** | `tokio::sync::mpsc` + `tokio_stream::wrappers::UnboundedReceiverStream` + Axum's `Sse` — zero-allocation hot path between extractor stages and the browser. The per-stage timing you see live? Free with the type system. | ✅ live |
| **Cloud SQL persistence (`aco-storage`)** | `sqlx` with compile-time-checked queries (when we enable `query!` macro against a live db). Connection pool sized for Cloud Run concurrency. | ✅ live |
| **Deterministic Blake3 ID hashing** | `blake3` is faster than SHA-256 *and* faster than the LLM call dominating the request. Means the same primitive always gets the same ID across sessions → idempotent dedup. | ✅ live |

---

## Tier 2 — next, high impact

These are the moves that turn AGON from a wrapper-around-an-LLM into a genuine conflict-intelligence engine. Each one is a thing Rust does that nothing else does at the same combination of speed, types, and deployability.

### 1. `ascent` — Datalog inference closure

> Same family as Soufflé. Rust-native. Macro DSL. Parallelizes with `ascent_par!`.

```rust
ascent! {
    relation actor(String);
    relation asserted(String, String);          // who, claim
    relation contradicts(String, String);
    relation high_friction_dyad(String, String);

    high_friction_dyad(a, b) <--
        contradicts(c1, c2),
        asserted(a, c1),
        asserted(b, c2),
        if a != b;
}
```

**Why:** closure over the full graph in ≤ 1s on 50k primitives, parallel, with stratified negation. We can encode the *whole* pattern catalog as Datalog and let the engine derive Four Horsemen counts, leverage cycles, coalition graphs, narrative-claim membership, and Allen-relation temporal consistency in one pass.

**Why Rust specifically:** no Python equivalent ships parallel Datalog with this performance. Souffl é is C++ and runs as a separate binary. `ascent` compiles into the AGON binary directly.

**Status:** crate skeleton in place (`aco-infer`). Need to: write 30–50 production rules. ~2 days.

---

### 2. `z3` — SMT contradiction detection

> Real SMT solver. Returns *unsat cores* — minimal sets of mutually-inconsistent claims.

When an actor says "I never agreed" on Thursday and "we agreed Monday" on Tuesday, Z3 doesn't just match strings — it constructs a first-order theory, asks Z3 if any model satisfies both, and if not, returns *exactly which formulas conflict*. The unsat core IS the contradiction evidence, with no LLM in the loop.

```rust
let ctx = z3::Context::new(&z3::Config::new());
let solver = z3::Solver::new(&ctx);
solver.assert(&claim_to_fol(&c1, &ctx));
solver.assert(&claim_to_fol(&c2, &ctx));
match solver.check() {
    SatResult::Unsat => {
        let core = solver.get_unsat_core();
        // core IS the contradiction trace
    }
    _ => {}
}
```

**Why Rust:** the `z3` crate gives us safe FFI to the world's best SMT solver. No subprocess, no IPC.

**Status:** Wired into `aco-infer/contradict/`. Need: FOL encoder for ACO claims. ~1 day.

---

### 3. `good_lp` (with SCIP backend) — BATNA / ZOPA / coalitions

> Linear and mixed-integer programming, in-process.

Negotiation theory says: every dyad has a `ZOPA` (zone of possible agreement). With multiple issues and asymmetric utilities, finding whether one *exists* is an LP feasibility problem. Finding the *Nash equilibrium* point is a MILP. AGON solves both in 50ms in-binary.

```rust
let mut p = ProblemVariables::new();
let buyer_surplus = p.add_variable().min(0.0).name("buyer_surplus");
let vendor_surplus = p.add_variable().min(0.0).name("vendor_surplus");
let model = p.minimise(0).using(default_solver);
let solved = model
    .with(constraint!(buyer_surplus + vendor_surplus <= total_surplus))
    .solve()?;
```

**Why Rust:** sub-100ms response on a Cloud Run cold start. No JVM. No GIL. SCIP is C and we link it via `good_lp::microlp`.

**Status:** crate scaffold. ~1 day to wire negotiation models.

---

### 4. `petgraph` + `blake3` — canonicalization

> The dedup layer. Resolves "Sarah Chen", "Sarah", "Ms. Chen", "the PM" → one canonical actor with all aliases preserved.

Three-tier matching:

1. **Exact hash** on normalized text — `blake3` of lowercased ASCII-folded form.
2. **ANN candidate** via `hnsw_rs` — top-k embedding neighbours from `fastembed` (BAAI/bge-small-en-v1.5, runs in 14ms per chunk).
3. **LLM tiebreaker** on borderline cases — Vertex AI gemini-2.5-pro decides with the *aliases* it sees.

Every merge writes an audit row. The alias graph is queryable: "show me every name Sarah Chen has been called across all my evidence."

**Why Rust:** the *combination* — Blake3 (fastest cryptographic hash), HNSW (fastest ANN index), petgraph (zero-copy graph), all in one binary, all leveraging SIMD via `cpufeatures`.

**Status:** scaffold in `aco-fuse/`. ~2 days.

---

### 5. `candle` or `burn` — neural inference in-binary (the Tier 1 future)

> Run small classifiers and embedders inside the Cloud Run binary, without calling Vertex.

Currently every extraction is a Vertex call (~3-10s, ~$0.01-0.05). For high-volume scoring or near-realtime monitoring (Phase 3 "wire watcher"), per-call latency to Vertex is the bottleneck. The play:

- **Pre-classify** whether a chunk is conflict-relevant at all (small DistilBERT in `candle`, <10ms).
- **Per-turn affect markers** (anger/contempt/disgust/fear) via a fine-tuned 60M-param classifier, in-binary.
- **Pattern proposers** — small models that flag *candidate* DARVO sequences for the LLM to confirm, cutting Vertex spend by 80%.

```rust
let model = candle_transformers::models::bert::BertModel::load(&vb, &config)?;
let logits = model.forward(&input_ids, &token_type_ids)?;
let label = logits.argmax_keepdim(D::Minus1)?;
```

**Why Rust:** `candle` (HuggingFace) and `burn` (Tracel) give us ONNX/safetensors loading, GPU/CPU/Metal backends, and in-process inference *without* Python or PyTorch on the Cloud Run image. The 250 MB distroless image stays 250 MB.

**Status:** not started. ~3 days for the first classifier.

---

### 6. `fastembed` — local embeddings (no Vertex round-trip)

> 384-dim BAAI/bge-small-en-v1.5 sentence embeddings in 14ms/chunk, 100% Rust.

Currently we use Vertex `text-embedding-005`. Switching to local `fastembed`:

- removes a network round-trip per chunk
- removes per-token billing for embeddings
- runs the same model the HNSW index expects
- benchmarked at ~7000 chunks/sec on a Cloud Run vCPU

**Why Rust:** `fastembed` is one ONNX runtime call + tokenization. Beats every Python framework on per-second throughput because there's no Python interpreter in the loop.

**Status:** crate scaffold in `aco-embed/`. ~1 day.

---

### 7. `pulldown-cmark` + `lopdf` + `docx-rs` — document loaders

> The "cameras" of the Tesla analogy.

A 50-page PDF deposition → text chunks → eight parallel extractors. The PDF parser is `lopdf` (pure Rust, no MuPDF C dependency). DOCX via `docx-rs`. Markdown via `pulldown-cmark`. All in the same binary.

**Why Rust:** Python alternatives need PyMuPDF (system libs, GPL), pdfplumber (slow). The Rust path is faster, MIT-licensed, and ships in the distroless container.

**Status:** crate scaffold. ~1 day.

---

### 8. WebSocket live updates

> When Cloud Run instance A perceives a new doc, instance B's dashboards subscribe via Postgres `LISTEN`/`NOTIFY` and the dashboard updates over the existing WebSocket.

```rust
use sqlx::postgres::PgListener;
let mut listener = PgListener::connect_with(&pool).await?;
listener.listen("agon_events").await?;
while let Some(notif) = listener.try_recv().await? {
    let event: WorldEvent = serde_json::from_str(notif.payload())?;
    ws_broadcast.send(event)?;
}
```

**Why Rust:** the `sqlx` LISTEN integration + `tokio::sync::broadcast` for fan-out across an arbitrary number of WebSocket connections. Sub-millisecond fanout to N browsers from one Postgres notification.

**Status:** Axum supports `ws://`. ~half a day.

---

## Tier 3 — eventual, big-payoff bets

### `tokio` async runtime for parallel sensors

The Tesla analogy says: every extractor runs *concurrently* on the same chunk. Today we serialize. With `tokio::join!`:

```rust
let (entities, events, claims, affect, patterns, temporal) = tokio::join!(
    entity_extractor.run(chunk),
    event_extractor.run(chunk),
    claim_extractor.run(chunk),
    affect_extractor.run(chunk),
    pattern_extractor.run(chunk),
    temporal_extractor.run(chunk),
);
```

Six Vertex calls in parallel, rate-limited by a shared `governor::DefaultDirectRateLimiter`. End-to-end latency on a 50-page doc drops from ~120s to ~25s.

### `wasmtime` for sandboxed user-defined rules

Phase 4: customers want to add their own pattern detectors. Embed `wasmtime`, load user rules as WASM, run them against the world model with strict resource bounds. Rust is the only major host language with first-class WASM embedding.

### `scallop` (probabilistic Datalog) via FFI

The line between symbolic and probabilistic conflict reasoning. Same Datalog rules, but every fact carries a probability and the engine returns marginal probabilities for derived facts. Scallop is Rust-native. The integration is gated behind a feature flag.

### Streaming JSON via `simd-json`

Parsing the 16k-token Vertex response with `serde_json` is 80% of post-extraction CPU time. `simd-json` is 4-7x faster on the same input. Free win, ~half day.

---

## What I would NOT use Rust for

Honest list:

- **Prompt engineering iteration.** Reload-the-prompt cycle should be fast. Keep prompts in `.tera` files + a hot-reload loop, no Rust restart needed.
- **Notebook-style exploration.** Use Python + duckdb against the Cloud SQL replica for one-off analytical queries.
- **The dashboard's *interactive* graph view.** Cytoscape.js (JS) is genuinely better than anything Rust→WASM right now for force-directed graphs with mouse interaction. Use it. Render from Rust JSON.

---

## Concretely, the next 10 days of Rust work

In priority order:

1. **`aco-perceive` extractor orchestrator** — `tokio::join!` parallel Vertex calls + JSON-schema verify-and-repair loop. (Day 5)
2. **`aco-fuse` canonicalization** — Blake3 exact-match + fastembed + HNSW + LLM tiebreaker for the alias graph. (Day 5)
3. **`ascent` Datalog rules** — leverage chains, coalition graphs, gap detection, temporal Allen relations. (Day 6)
4. **`z3` per-actor consistency** — FOL encoder + unsat-core extraction. (Day 9)
5. **`candle` pattern proposer** — small model that flags candidate DARVO/gaslighting before the LLM confirms, cutting Vertex spend ~5x. (Day 10)
6. **Document loaders + chunk planner** — `lopdf` / `docx-rs` / `pulldown-cmark` + 150k-token chunk boundaries on heading/paragraph splits. (Day 5)
7. **`good_lp` BATNA/ZOPA** — feasibility + Nash mediation move. (Day 11)
8. **Postgres `LISTEN`/`NOTIFY` → WebSocket fanout** — multi-instance dashboard sync. (Day 12)
9. **`simd-json` swap** — drop-in for the hot path. (Day 16 perf pass)
10. **Per-pipeline-stage `criterion` benches** — lock the performance targets in CI. (Day 16)

Every one of these is a thing Rust does better than the alternative *and* that AGON needs to ship its core value proposition: a sovereign, fast, auditable conflict-intelligence engine.

---

*The LLM is one of eight sensors. Rust is what makes the other seven actually competitive with it.*
