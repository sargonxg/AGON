# AGON — Honest State of Things (2026-05-13)

Brutally honest audit of what is real and what is scaffold. No marketing.

---

## "Are we using Rust for real?"

**Yes. Definitively.**

- Entire chassis is Rust (Cargo workspace, 16 crates).
- Production binary is a single statically-linked Rust executable in a distroless container (~250 MB image).
- HTTP server: Axum (Rust). DB: sqlx → Cloud SQL Postgres (Rust). Auth: tower (Rust). Embedded UI: rust-embed (Rust).
- `forbid(unsafe_code)` on every new crate.
- 56 unit + integration tests passing.

The only non-Rust pieces are:
- Vertex AI service (cloud-hosted, Google's stack)
- The Postgres engine itself
- The Docker daemon at build time
- ONNX model export (will be Python `optimum-cli`, one-time, output is consumed by Rust at runtime)

---

## "Are we using ML for real?"

**Partially.** Here is the honest accounting.

### What is real ML (today)

1. **Gemini 2.5 Flash via Vertex AI** — a real, large neural network. Schema-constrained JSON output. Pinned model version. This is the primary extractor. It is doing the heavy semantic lifting and producing the typed ACO primitives you see.
2. **Gemini text-embedding-005** — wired in `aco-llm::vertex::embed()` but **NOT yet called from the perception flow**. Available, unused.

### What looks like ML but is NOT

In the response JSON you see today:
```json
"neural_signals": {
  "mode": "local_sparse",
  "model": "local-sparse-conflict-sensor",
  "signals": [{ "kind": "claim_relatedness", "score": 0.63, ... }]
}
```

That field is **misleading by name**. It is **not a neural network**. It is a **TF-IDF-style cosine similarity over a weighted bag-of-words with a stoplist + boosted weights on conflict-keywords** (`crates/aco-embed/src/lib.rs:143-179`). Specifically:

- Tokenize on non-alphanumeric. Drop stop-words. Length ≥ 3.
- Hand-tuned weights: `never`, `denied`, `agreed`, `cancelled`, etc. get 1.8×. Everything else 1.0×.
- Cosine over the weighted vector.

That is information retrieval, not ML. The `fastembed` crate is gated behind a `neural` feature flag that is **off in the production binary**. The field name `neural_signals` was inherited from MVP scaffolding and is overstated.

**Action:** rename to `lexical_similarity_signals` in the next deploy, or actually wire fastembed/ONNX behind it. Honest naming matters.

### What is NOT in production yet (per build plan)

- **Local ONNX encoders** (BGE-M3 embeddings, DeBERTa-v3-large-mnli, fastcoref) — **NOT YET**. This is PROMPT 05 of the perception sprint. Models exist on HuggingFace, HF token is in Secret Manager, but no ONNX export, no `ort` 2.x runtime wiring, no `aco-encode` crate yet. Estimated 2–3 days of focused work.
- **Local LLM** (`mistral.rs` Q4_K_M) — not in plan until ≥1k folders/mo justifies it.

---

## "Are we really using encoders?"

**No. Not yet in any meaningful sense.**

The word "encoders" in this codebase currently refers to:
- The `aco-encode` crate **which does not exist** in `crates/`. Listed in `BUILD_PLAN_PERCEPTION.md` PROMPT 05 — not built.
- The TF-IDF-style `aco-embed` (above) — not an encoder in the modern sense.
- A `Vertex text-embedding-005` call that is **wired but unused** in the perception flow.

When PROMPT 05 lands, we will actually have:
- BGE-M3 dense + sparse + ColBERT embeddings, in-process Rust via `ort` 2.x.
- DeBERTa-v3-large-mnli for natural language inference (entailment / contradiction / neutral).
- fastcoref for coreference resolution.
- Three models, mmap-loaded, batched, sub-second on CPU and sub-100ms on L4.

Until then, the only model doing real ML work in the perception path is **Gemini 2.5 Flash, remote, via Vertex AI**.

---

## What perception capability genuinely works right now

This is what the live `https://agon-dev-tbryoen6qa-uc.a.run.app` does and does well:

1. **Deterministic pre-processing** (Rust, sub-millisecond):
   - speaker-turn detection (dialog/email/Slack/deposition formats)
   - canonical actor IDs from speaker labels
   - temporal marker extraction
   - modality marker extraction (`never`, `agreed`, `promised`, …)
   - conflict-density score (lexical heuristic, 0.0–1.0)
   - segment-level char offsets for evidence verification
   - **NEW (v0.1.1, just deployed):** hedge / modal / passive / pronoun counts, coalition signal, agency-hiding flag, direct/reported speech span counts via `aco-lex` + `aco-text`

2. **LLM extraction** (Gemini 2.5 Flash, ~18s per case):
   - 8 ACO primitives: actors, claims, interests, constraints, leverages, commitments, events, narrative frames
   - relations: ASSERTED, DENIED, ACKNOWLEDGED, CONTRADICTS, COMMITS_TO, …
   - schema-constrained JSON output (`responseMimeType=application/json` + `responseSchema`)
   - last-resort salvage of truncated JSON (`vertex.rs::salvage_json`)
   - **F-03 fix shipped:** bounded retry (502/503/504, 250ms→4s exp backoff)

3. **Post-processing** (Rust, sub-millisecond):
   - evidence-quote verification — every primitive's quote must appear in the source (27/27 verified on Q4 demo)
   - friction matrix per actor pair, with reasons
   - quality gates: evidence_coverage, actor_ambiguity, conflict_signal
   - review questions surfaced for the user

4. **Persistence**:
   - Cloud SQL stores every perception. `/api/sessions` exposes history. UI reloads past extractions.

5. **UI**:
   - Embedded dark-mode dashboard.
   - SVG force-directed graph of actors + claims + commitments.
   - Friction matrix card with heat scores + reasons.
   - History panel.

That is a working, useful conflict-perception system **today**. It just isn't yet using the local ONNX neural models that are in the plan.

---

## The honest path to "really powerful neuro AI"

In order of impact, given everything else equal:

### Tier 1 — biggest leap per day of effort
1. **PROMPT 09 — pattern library**. The actual moat. DARVO, anchoring, scope creep, conspicuous absence, coalition. These named patterns are what generic LLMs miss. The Q4 demo *contains* DARVO ("you're putting words in my mouth") and we only see it as a generic "escalation loop". 5–10 days.
2. **PROMPT 05 — local ONNX encoders**. Adds independent neural signal (NLI contradiction detection, BGE-M3 retrieval). Means we can cross-check Gemini's output against a different model class. 4–7 days. Needs L4 quota for prod; CPU works for dev.

### Tier 2 — quality multipliers
3. **PROMPT 10 — calibration**. Right now every confidence is uncalibrated. We have no idea if 0.76 means 76% accurate or 50% accurate. Calibration plus conformal prediction makes the abstention story honest. 3–5 days.
4. **PROMPT 11 — provenance + audit export**. Litigation-grade evidence chain. Merkle audit log. Signed records. JSON-LD + Markdown export. This is what makes AGON sellable, not a toy. 3–5 days.

### Tier 3 — production hardening
5. **PROMPT 12 — split CPU/GPU services**. `agon-batch` on L4 for local ML; `agon-api` stays CPU. Cost-optimized at scale.
6. **PROMPT 15 — adversarial robustness**. 80-case adversarial pack in CI. Prompt-injection defense.

### Tier 4 — credibility instrument
7. **PROMPT 14 — TCGC v0.2 eval harness**. Without this, "AGON is accurate" is a claim with no instrument behind it. 7–10 days.

---

## What you should believe, what you should question

| Claim | Status |
|---|---|
| "AGON extracts typed primitives with evidence" | **True** — see live demo |
| "AGON detects contradictions" | **True** but via Gemini + lexical similarity, not via NLI yet |
| "AGON uses ML" | **True** — Gemini 2.5 Flash |
| "AGON uses neural encoders" | **Misleading today**. True after PROMPT 05 |
| "AGON detects patterns like DARVO, anchoring, conspicuous absence" | **Aspirational** — PROMPT 09 |
| "AGON is calibrated" | **No** — PROMPT 10 |
| "AGON is litigation-grade" | **No** — PROMPT 11 + 15 |
| "AGON is Rust" | **True** |
| "AGON is on GCP" | **True** — Cloud Run + Cloud SQL + Vertex |

---

## Next move (recommended, prioritized)

1. **Today**: deploy v0.1.1 (just built). Verify lex+speech-span signals reach Gemini in production. Re-run Q4 demo and observe whether the prompt enrichment changes the extraction quality.
2. **This week**: PROMPT 09 — patterns library. Real value, no external blockers. DARVO + anchoring + conspicuous absence first (the three most distinctive). This makes AGON visibly smarter than "another LLM wrapper".
3. **Next week**: PROMPT 05 — aco-encode. Real local ONNX encoders. We have the HF token. We have CPU capacity in Cloud Run. Build it, even before splitting CPU/GPU services.
4. **After that**: PROMPT 10 (calibration) and PROMPT 11 (provenance), in that order.

PROMPTS 06 (Anthropic/OpenAI backends) is dropped — Gemini-only.
PROMPT 12 (GPU split) deferred until aco-encode lands and we measure throughput.
PROMPT 14 (eval harness) interleaves with 09/10/11.

---

*Auditor: Claude Opus 4.7 · No glossing. If you read marketing copy that contradicts this file, this file wins.*
