# AGON Perception Stack Ledger

**Plan source:** `docs/BUILD_PLAN_PERCEPTION.md` (15-prompt sprint, ~90 days)
**Started tracking:** 2026-05-13
**Builds on:** v0.1.0 MVP (already live on Cloud Run, see `AGON_LEDGER.md`)
**Target:** layered perception stack L1–L8 (sensors → encoders → extraction → tracking → scene → calibration → provenance → decision)
**Architecture principle:** typed primitive layer is the moat, not the model layer.

Legend: ☐ todo · ◐ in-progress · ✓ done · ✗ blocked · ⏸ deferred

---

## Phase map (90-day pacing)

| Days | Prompts | Theme | Demo at end |
|---|---|---|---|
| 1–10  | 01, 02, 03 | Foundations | Doc round-trip: normalize + time extract + evidence spans verify |
| 11–25 | 04, 05, 06 | Encoders + LLM portability | BGE-M3 / DeBERTa-NLI / fastcoref local; multi-vendor LLM swap |
| 26–45 | 07, 08, 09 | Perception + patterns | Full pipeline emits ACO primitives + 5 patterns w/ golden fixtures |
| 46–60 | 10, 11 | Calibration + provenance | Calibrated confidence on every primitive; full audit export |
| 61–75 | 12, 13 | Prod deployment + UI | Two-service GCP topology; corrections capture |
| 76–90 | 14, 15 | Eval + adversarial | TCGC v0.2 published; 80-case adversarial passes |

Hard sequencing:
- 01 → all (contracts)
- 02 → 03, 04, 05, 07 (canonical text)
- 05 + 06 → 07
- 07 + 08 → 09
- 09 + 11 → 13
- 11 → 14 → 15

Parallelizable: {03, 04} alongside 05 · {12} alongside {13}

---

## Prompt tracker

| # | Branch | Crate(s) | Status | PR | Notes |
|---|--------|----------|--------|----|----|
| 01 | sprint/01-tacitus-contracts | crates/tacitus-contracts | ✓ v0.1 (2026-05-13) | — | 8 JSON Schemas + Rust types + validation tests green; codegen→Py/TS deferred to follow-up |
| 02 | sprint/02-aco-text | aco-text | ◐ v0.2 (2026-05-13) | — | + quoted-speech FSM (direct/curly/«»/„"/reported) + speaker-turn detector (Q:/A:/MR./THE/email/Slack); 19 tests green; 200-fixture regression set + free-indirect speech still TODO |
| 03 | sprint/03-aco-time | aco-time | ◐ v0.1 (2026-05-13) | — | Allen-13 calculator + 2 tests; multilingual regex/DFA detection TODO |
| 04 | sprint/04-aco-lex | aco-lex | ◐ v0.1 (2026-05-13) | — | EN hedge/modality/passive/pronoun + 3 tests; FR/ES/IT/PT + face-work/register TODO |
| 05 | sprint/05-aco-encode | aco-encode | ☐ | — | ort 2.x: BGE-M3 + DeBERTa-NLI + fastcoref ONNX |
| 06 | sprint/06-aco-llm | aco-llm | ◐ | — | Extends existing; add Claude + GPT-5 + routing.toml; **VertexGemini live** |
| 07 | sprint/07-aco-extract | aco-extract (new) | ☐ | — | L1+L2+L3 pipeline; Fast/Standard/Deep modes |
| 08 | sprint/08-aco-fuse-temporal | aco-fuse, aco-temporal | ☐ | — | Cross-doc actor IDs + commitment state machine + Allen-13 |
| 09 | sprint/09-aco-patterns | aco-patterns | ◐ v0.2 (2026-05-14) | — | **DARVO + Anchoring + Conspicuous Absence live (3/5)**; 12 unit tests; patterns NOW run pre-LLM and are injected into the Gemini envelope (C-path) — single LLM call, richer extraction; scope creep + coalition remain |
| 10 | sprint/10-aco-score | aco-score | ☐ | — | Temp/iso/beta calibration + stacked LR + conformal prediction |
| 11 | sprint/11-aco-prov | aco-prov | ☐ | — | Lineage DAG + Merkle + Ed25519 + JSON-LD/MD export |
| 12 | sprint/12-cloud-run-gpu | infra/, aco-batch-rpc | ☐ | — | Split agon-api (CPU) + agon-batch (L4 GPU) |
| 13 | sprint/13-workbench-ui | aco-server | ☐ | — | Pattern panels + corrections capture |
| 14 | sprint/14-tcgc-eval | corpus/, evals/ | ☐ | — | TCGC v0.2 + Inspect-AI + Argilla + Krippendorff α |
| 15 | sprint/15-adversarial | aco-text, aco-llm | ☐ | — | Canonicalization + spotlighting + 80-case pack |

---

## Definition of done per prompt

Every prompt ships only when:
1. New crate has ≥ 1 unit test green
2. `cargo build --workspace` + `cargo clippy --workspace -- -D warnings` clean
3. `cargo deny check` clean (license + advisory)
4. Verification block from build plan passes verbatim
5. New CI job added (`.github/workflows/`) if introducing a new crate
6. Ledger row updated to ✓ with PR link
7. STATE.json `current_prompt` advanced

---

## Carry-through risks (re-check at every prompt)

1. Evidence-span drift across normalization → quad-form spans (P02 + P11)
2. Calibration debt → per-sensor curves + nightly refit (P10)
3. Pattern proliferation without versioning → golden fixtures + version-pinning (P09)
4. Schema sprawl → tacitus-contracts SoT + CI codegen check (P01)
5. Prompt injection → canonicalization + spotlighting (P15)
6. Cost runaway on long-context → routing.toml + per-folder budgets (P06 + P12)
7. Model deprecation → LlmBackend trait + version pinning (P06)
11. **Single-vendor LLM (Gemini-only) risk** → Mock backend in CI + trait-isolated swap path; revisit when first design partner signs. See `docs/AUDIT_2026-05-13.md` §"Architecture-level red flags".
8. GPU cold-start → min-instances + warm-up handler (P12)
9. License contamination → cargo-deny + per-prompt license review (P03 + P05)
10. "Deception detector" framing → neutralized public names + ethics doc (P09 + P15)

---

## Sessions

| Date | Prompt | Hours | Outcome |
|------|--------|-------|---------|
| 2026-05-13 | planning | — | Plan absorbed, ledger + externals + GCP deploy docs drafted |
| 2026-05-13 | 01-04 v0.1 | — | tacitus-contracts (8 schemas, types, tests) + aco-text (normalize/segment/verify) + aco-time (Allen-13) + aco-lex (EN features); 24+ tests green; HF token stored in Secret Manager; control buttons shipped (agon-up/down/status/nuke); AGON_GUIDE.md written |
| 2026-05-13 | audit + Gemini-only | — | Audit done (`docs/AUDIT_2026-05-13.md`, 15 findings, 0 critical-in-code, 1 self-introduced bug fixed); Anthropic + OpenAI dropped; `crates/aco-llm/config/routing.toml` written; `forbid(unsafe_code)` added to all new crates; aco-lex regex cache fixed |
| 2026-05-13 | deploy v0.1.1 | — | Cloud Build → revision `agon-dev-00013-rbj`; verified Q4 demo (33/33 evidence, named pattern signals in friction reasons); verified board-minutes test fires coalition + agency-hiding + strong-hedge flags |
| 2026-05-14 | 09 partial — DARVO | — | aco-patterns crate + DARVO detector + wired into /api/perceive; 5 unit tests + 1 integration via server tests; total workspace 61 tests; README rewritten as cover page; merged remote ROADMAP + INTEROP + docs/INDEX |
| 2026-05-14 | 09 #2 + introspection + aco-encode scaffold | — | Anchoring detector live (4 tests, $/€/£/% support); 3 backend introspection endpoints (`/api/system`, `/api/patterns`, `/api/pipeline`); aco-encode crate scaffolded (`Encoder` trait + HF downloader + `--features onnx` gate); workspace 67 tests; deployed v0.1.3 |
| 2026-05-14 | 09 #3 + C-path priming | — | Conspicuous Absence detector live (3 tests, expectations.toml + doc-type inference); patterns now run **pre-LLM** and inject into Gemini envelope; workspace 70 tests; deployed v0.1.4 (revision agon-dev-00016-ls8) |
