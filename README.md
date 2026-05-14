# AGON

[![CI](https://github.com/sargonxg/AGON/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sargonxg/AGON/actions/workflows/ci.yml)
[![Docker](https://github.com/sargonxg/AGON/actions/workflows/docker.yml/badge.svg?branch=main)](https://github.com/sargonxg/AGON/actions/workflows/docker.yml)
[![Audit](https://github.com/sargonxg/AGON/actions/workflows/audit.yml/badge.svg?branch=main)](https://github.com/sargonxg/AGON/actions/workflows/audit.yml)

```text
   █████╗  ██████╗  ██████╗ ███╗   ██╗
  ██╔══██╗██╔════╝ ██╔═══██╗████╗  ██║
  ███████║██║  ███╗██║   ██║██╔██╗ ██║
  ██╔══██║██║   ██║██║   ██║██║╚██╗██║
  ██║  ██║╚██████╔╝╚██████╔╝██║ ╚████║
  ╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝

  conflict is legible.
  perception is sovereign.
```

> A perception engine for human conflict. AGON reads messy human text — emails, transcripts, depositions, board minutes, chat logs — and returns a typed, evidence-backed picture of the conflict inside it: **who said what, what was promised, what changed, where the contradictions are, what patterns are present**. Same shape as a self-driving stack — sensors, encoders, extraction, tracking, scene, calibration, provenance — applied to language.

**Status:** v0.1.1 live · 16-crate Rust workspace · Cloud Run + Vertex AI · 56 tests green · MIT/Apache-2.0

Built by [TACITUS](https://www.tacitus.me).

---

## Why AGON exists

Generic LLMs are good at summarizing text. They are bad at:

- **Naming the move**. They tell you "there's tension"; they don't tell you "this is a textbook DARVO".
- **Anchoring claims to source**. They paraphrase. AGON requires every primitive to cite an exact verbatim span — and verifies it.
- **Tracking commitments through time**. They forget what was promised three turns ago. AGON keeps a state machine: `made → confirmed → contested → broken`.
- **Knowing when to abstain**. They are confidently wrong. AGON has calibrated confidence and conformal-prediction abstention (on the roadmap).
- **Being auditable**. They are a black box. AGON's output is a typed DAG with content-hash provenance for every node.

AGON is not a chatbot. It is **infrastructure for conflict vision** — built like a perception stack, not a prompt template.

---

## The self-driving-car analogy (taken literally)

```
                            RAW TEXT
                               │
                               ▼
┌───────────────────────────────────────────────────────────────┐
│  L1  SENSORS          deterministic Rust                       │
│      canonical text · segmentation · quoted-speech FSM ·       │
│      speaker turns · time expressions · lexical features       │
│      (aco-text · aco-time · aco-lex)                           │
├───────────────────────────────────────────────────────────────┤
│  L2  ENCODERS         ort 2.x (ONNX Runtime)                   │
│      BGE-M3 embeddings · DeBERTa-v3-large NLI · fastcoref      │
│      (aco-encode)                       — PROMPT 05            │
├───────────────────────────────────────────────────────────────┤
│  L3  EXTRACTION       Vertex Gemini 2.5 Flash + Pro            │
│      schema-constrained ACO primitives:                        │
│      Actor · Claim · Interest · Constraint · Leverage ·        │
│      Commitment · Event · Narrative · Contradiction            │
│      (aco-extract · aco-llm)                                   │
├───────────────────────────────────────────────────────────────┤
│  L4  TRACKING         deterministic Rust                       │
│      cross-doc actor resolution · commitment state machine ·   │
│      Allen-13 temporal logic · evidence-span verification      │
│      (aco-fuse · aco-temporal)                                 │
├───────────────────────────────────────────────────────────────┤
│  L5  SCENE            hybrid                                   │
│      friction matrix · pattern library                         │
│      DARVO · anchoring · scope creep · conspicuous absence ·   │
│      coalition · power dynamics                                │
│      (aco-patterns)                     — PROMPT 09            │
├───────────────────────────────────────────────────────────────┤
│  L6  CALIBRATION      deterministic Rust                       │
│      per-detector temperature/isotonic · stacked LR ·          │
│      conformal prediction for abstention                       │
│      (aco-score)                        — PROMPT 10            │
├───────────────────────────────────────────────────────────────┤
│  L7  PROVENANCE       deterministic Rust                       │
│      typed lineage DAG · Merkle audit log · signed records ·   │
│      JSON-LD + Markdown export                                 │
│      (aco-prov)                         — PROMPT 11            │
├───────────────────────────────────────────────────────────────┤
│  L8  DECISION         Axum + SSE                               │
│      quality gates · review questions · streaming workbench    │
│      (aco-server)                                              │
└───────────────────────────────────────────────────────────────┘
```

Each layer has a typed contract. Each layer is independently testable. **No single model is asked to do everything.** The chassis is Rust. ML models are interchangeable passengers behind typed traits.

---

## What it does today (live demo)

Paste a multi-turn dispute. Get back a structured perception.

### Input

```
Sam (Mon 09:14): So we're agreed — you own the Q4 launch deck content,
                  I handle design. Lock it in by Thursday?
Alex (Mon 09:47): Sounds good. I'll pick it up after the Jenkins pitch.
Alex (Thu 09:02): I never said I'd own it. Just help.
Sam (Thu 09:15):  That's not what we discussed. We don't have time to
                  relitigate this — the launch is Monday.
Alex (Thu 09:18): You're putting words in my mouth. You said you'd own
                  the content if I helped with design.
```

### Output (real, from the live service, 22 s)

- **2 actors** — `actor_sam`, `actor_alex`
- **1 contested commitment** — *"own the Q4 launch deck content"* · state=`contested` · confidence 0.76
- **1 escalation loop** around `actor_alex` · confidence 0.71
- **10 contradictions** with evidence spans
- **5 speaker turns** detected pre-extraction
- **33/33 evidence quotes verified** against canonical source
- **friction matrix**: Sam ↔ Alex heat **100/100**, reasons include `commitment_contested`, `pattern: defensiveness`, `pattern: criticism`, `pattern: stonewalling`, `escalation_signal`
- **review questions** surfaced: *"What exact words created or limited the alleged commitment?"*, *"Which contradiction is material to the decision?"*

The friction matrix and force-directed actor/claim graph render in a dark-mode workbench at `https://agon-dev-tbryoen6qa-uc.a.run.app` (user `AGON` / pass `AGON`).

---

## Turn it on, turn it off

```bash
bash scripts/agon-up.sh        # start  (~30 s, then ~$3–8/day active)
bash scripts/agon-down.sh      # stop   (~10 s, then ~$0.20–0.50/day idle)
bash scripts/agon-status.sh    # status
bash scripts/agon-nuke.sh      # terraform destroy (DATA LOSS — use with care)
```

PowerShell wrappers: `scripts/*.ps1`. Full operator's guide: **[docs/AGON_GUIDE.md](docs/AGON_GUIDE.md)**.

A typical **few-days-of-testing** cycle costs **under $30** total. Your GCP startup credit covers it many times over.

---

## Architecture decisions that won't change

These are locked. If something downstream conflicts, it loses.

1. **Rust chassis, ML passengers.** No Python sidecar. Every model behind a Rust trait.
2. **JSON Schema is the source of truth.** [`tacitus-contracts`](crates/tacitus-contracts/) is the only place primitives are defined. Rust types live alongside; Python and TypeScript regenerate from the same schemas.
3. **Evidence-span quad form.** Every claim-bearing primitive carries `(segment_id, canonical_offsets, raw_offsets, verbatim_quote, quote_hash, normalization_version)`. Non-negotiable. This is what makes a primitive auditable instead of plausible.
4. **Calibration is mandatory.** Every detector emits raw signal; the calibration registry converts to probability. LLM verbalized confidence is a feature, never a probability.
5. **Per-doc perception, then cross-doc fusion.** Long-context Gemini is an adjudication tool, not the primary architecture.
6. **Pattern names are clinical internally, neutral publicly.** DARVO → "possible role-reversal pattern" in the UI. Ethics + legal.
7. **No training.** AGON is inference-only. Corrections corpus accumulates; training is deliberate future work, gated on the corrections corpus reaching critical mass.

---

## The ACO ontology (Agentic Conflict Ontology) — locked for v0.1

### 8 primitives

| Primitive | Definition |
|---|---|
| **Actor** | Any party capable of holding an interest or making a claim |
| **Claim** | An asserted fact, evaluation, or normative statement attributed to an actor |
| **Interest** | An underlying goal or need (Fisher/Ury distinction from "position") |
| **Constraint** | A rule, norm, or structural limit |
| **Leverage** | A resource, dependency, or capability that shifts bargaining power |
| **Commitment** | A promised future action, with subject and deadline |
| **Event** | A dated or orderable occurrence |
| **Narrative** | A coherent framing across multiple claims |

### 18 typed edges (closed set)

`ASSERTED · DENIED · ACKNOWLEDGED · ACKNOWLEDGED_AMBIGUOUSLY · DENIES_SCOPE · COMMITS_TO · REVOKES · BLOCKS · ENABLES · CAUSES · PRECEDES · CONTRADICTS · SUPPORTS · CITES · HOLDS_INTEREST · FRAMES · LEVERAGES · CONSTRAINED_BY`

Every edge carries a `provenance` field. Missing provenance fails validation.

### Partial-credit type similarity

When a predicted edge is close to but not identical to the gold edge, partial credit is awarded — `ACKNOWLEDGED ↔ ACKNOWLEDGED_AMBIGUOUSLY = 0.75`, `BLOCKS ↔ CONSTRAINED_BY = 0.40`, etc. See [`crates/tacitus-contracts/`](crates/tacitus-contracts/) for the full matrix.

---

## Repository map

```
AGON/
├── README.md                      ← you are here
├── docs/
│   ├── AGON_GUIDE.md              ← operator's guide (start/stop, costs, day-by-day)
│   ├── BUILD_PLAN_PERCEPTION.md   ← 15-prompt build plan (~90 days)
│   ├── DEPLOYMENT_GCP.md          ← target Cloud Run + Vertex topology
│   ├── EXTERNALS.md               ← what you provide (Gemini-only, no Anthropic/OpenAI)
│   ├── HONEST_STATE.md            ← brutally honest accounting of what is real
│   └── AUDIT_2026-05-13.md        ← 15-finding code audit
├── PROJECT_LEDGER/
│   ├── AGON_LEDGER.md             ← MVP v0.1.0 sprint (shipped)
│   ├── PERCEPTION_LEDGER.md       ← 15-prompt perception sprint tracker
│   └── STATE.json                 ← current state, next prompt, open externals
├── crates/
│   ├── tacitus-contracts/         ← typed primitives + JSON Schemas (PROMPT 01) ✓
│   ├── aco-text/                  ← canonical text + segmenter + quoted-speech FSM (PROMPT 02) ✓
│   ├── aco-time/                  ← Allen-13 temporal algebra (PROMPT 03) ◐
│   ├── aco-lex/                   ← hedge/modality/passive/pronoun extractors (PROMPT 04) ◐
│   ├── aco-encode/                ← BGE-M3 + DeBERTa-NLI + fastcoref (PROMPT 05) ☐
│   ├── aco-llm/                   ← Vertex Gemini backend (live)
│   ├── aco-extract/               ← L1+L2+L3 perception pipeline (PROMPT 07) ☐
│   ├── aco-fuse/                  ← cross-doc actor resolution (PROMPT 08) ☐
│   ├── aco-temporal/              ← commitment state machine (PROMPT 08) ☐
│   ├── aco-patterns/              ← DARVO + anchoring + scope creep + coalition + conspicuous absence (PROMPT 09) ☐
│   ├── aco-score/                 ← calibration + conformal prediction (PROMPT 10) ☐
│   ├── aco-prov/                  ← lineage DAG + Merkle audit (PROMPT 11) ☐
│   ├── aco-storage/               ← Cloud SQL via sqlx (live)
│   ├── aco-server/                ← Axum + workbench UI (live)
│   ├── aco-cli/                   ← agon-cli
│   ├── aco-core/                  ← shared types + provenance
│   ├── aco-perceive/              ← MVP perception (refactored at PROMPT 07)
│   ├── aco-fuse/, aco-infer/, aco-embed/, aco-learn/, aco-bench/   ← MVP scaffold
├── infra/terraform/               ← VPC + Cloud SQL + Cloud Run + GCS + Eventarc + IAM
├── scripts/
│   ├── agon-up.sh / .ps1          ← turn ON
│   ├── agon-down.sh / .ps1        ← turn OFF
│   ├── agon-status.sh / .ps1
│   └── agon-nuke.sh               ← terraform destroy
├── migrations/                    ← Postgres schema
├── corpora/                       ← test inputs
├── Cargo.toml                     ← workspace + deps
├── Dockerfile, compose.yaml
├── Makefile
└── .env.example
```

Legend: ✓ done · ◐ in flight · ☐ planned (see `PROJECT_LEDGER/PERCEPTION_LEDGER.md`)

---

## Roadmap (15 prompts, ~90 days)

| Phase | Days | Prompts | Deliverable |
|---|---|---|---|
| **Foundations** | 1–10 | 01–03 | Doc round-trip: normalize → segment → time extract → evidence spans verify |
| **Encoders + LLM** | 11–25 | 04–06 | Local ONNX (BGE-M3 / DeBERTa / fastcoref) + Vertex Gemini routing |
| **Perception + patterns** | 26–45 | 07–09 | Full pipeline emits ACO primitives + 5 named patterns with golden fixtures |
| **Calibration + provenance** | 46–60 | 10–11 | Calibrated confidence on every primitive · litigation-grade audit export |
| **Prod deploy + UI** | 61–75 | 12–13 | Split CPU/GPU services · corrections capture in workbench |
| **Eval + adversarial** | 76–90 | 14–15 | TCGC v0.2 + Inspect-AI + 80-case adversarial pack |

Hard sequencing: 01 → all · 02 → 03/04/05/07 · 05+06 → 07 · 07+08 → 09 · 11 → 14 → 15.

Full spec: **[docs/BUILD_PLAN_PERCEPTION.md](docs/BUILD_PLAN_PERCEPTION.md)** (1246 lines, every prompt self-contained).

---

## Where to start reading

| If you want to… | Read |
|---|---|
| **Full doc map** | [`docs/INDEX.md`](docs/INDEX.md) |
| See where AGON is going (standalone + trinity) | [`ROADMAP.md`](ROADMAP.md) |
| Trinity integration (AGON ↔ DIALECTICA ↔ KAIROS) | [`docs/INTEROP.md`](docs/INTEROP.md) |
| Run it for a few days then stop | [`docs/AGON_GUIDE.md`](docs/AGON_GUIDE.md) §2 + §9 |
| Understand the architecture | [`docs/AGON_GUIDE.md`](docs/AGON_GUIDE.md) §1 + [`docs/BUILD_PLAN_PERCEPTION.md`](docs/BUILD_PLAN_PERCEPTION.md) |
| Know what AGON depends on externally | [`docs/EXTERNALS.md`](docs/EXTERNALS.md) |
| Know what's deployed where | [`docs/DEPLOYMENT_GCP.md`](docs/DEPLOYMENT_GCP.md) |
| See the typed primitive contracts | [`crates/tacitus-contracts/README.md`](crates/tacitus-contracts/README.md) |
| See what's done vs in-flight | [`PROJECT_LEDGER/PERCEPTION_LEDGER.md`](PROJECT_LEDGER/PERCEPTION_LEDGER.md) |
| **Honest accounting of what's real** | [`docs/HONEST_STATE.md`](docs/HONEST_STATE.md) |

---

## Open-source choices and why

| Component | Choice | License | Why |
|---|---|---|---|
| Embeddings | BGE-M3 | Apache-2.0 | Dense + sparse + ColBERT in one model · multilingual · ONNX-exportable |
| NLI | DeBERTa-v3-large-mnli (MoritzLaurer) | MIT | Best open NLI checkpoint · INT8 quantizable |
| Coreference | fastcoref | MIT | License-clean · 78.5 F1 |
| ONNX Runtime | `ort` 2.x | MIT/Apache | Production-proven · pure Rust |
| Time extraction | hand-rolled Rust DFA | — | HeidelTime/SUTime are GPL — can't use |
| Segmenter | hand-rolled SRX-style | — | pragmatic-segmenter is MIT but we own the impl |
| Postgres | self-hosted on Cloud SQL | OSS | $25/mo at dev tier |
| Vector store | `pgvector` extension | OSS | No managed vector DB |
| Annotation | Argilla | Apache-2.0 | Self-host on Cloud Run |
| Eval orchestrator | Inspect-AI (UK AISI) | Apache-2.0 | Principled |
| LLM observability | Langfuse self-hosted | MIT | Self-host vs $100/mo SaaS |
| Remote LLM | Vertex Gemini 2.5 Flash + Pro | paid | Schema-constrained, $0.30/$2.50 per M tokens |

**Vendor strategy: Gemini-only.** Cross-validation done with Flash vs Pro at different temperatures / prompt versions. Anthropic + OpenAI backends in the original plan were dropped 2026-05-13 (see [`docs/AUDIT_2026-05-13.md`](docs/AUDIT_2026-05-13.md) §F-10).

---

## Live demo

```text
URL:      https://agon-dev-tbryoen6qa-uc.a.run.app
User:     AGON
Password: AGON
Status:   https://agon-dev-tbryoen6qa-uc.a.run.app/api/info
```

Paste a multi-turn conflict (Slack thread, email reply chain, deposition snippet, board minutes). Click *Perceive*. Watch the friction matrix, the actor/claim graph, and the structured ACO primitives appear with verifiable evidence quotes.

API:

```bash
curl -u AGON:AGON -X POST https://agon-dev-tbryoen6qa-uc.a.run.app/api/perceive \
  -H "Content-Type: application/json" \
  -d '{"text": "Sam: We agreed Thursday. Alex: I never agreed.", "title": "demo"}'
```

---

## Contributing

This is built in public by Giulio Catanzariti for [TACITUS](https://www.tacitus.me). The 15-prompt build plan is designed for `Claude Code Opus 4.7` to execute one prompt per session, one PR each. If you want to participate:

- Pick the next prompt in [`PROJECT_LEDGER/PERCEPTION_LEDGER.md`](PROJECT_LEDGER/PERCEPTION_LEDGER.md)
- Branch `sprint/<NN>-<name>`
- Implement against the verification block in [`docs/BUILD_PLAN_PERCEPTION.md`](docs/BUILD_PLAN_PERCEPTION.md)
- Open PR · the ledger row turns ✓ on merge

Issues with the spec? Open one tagged `spec-drift`.

---

## License

MIT OR Apache-2.0, at your option. See [`LICENSE`](LICENSE).

---

## Cite

```bibtex
@software{agon2026,
  author = {Catanzariti, Giulio},
  title  = {AGON: A Perception Engine for Human Conflict},
  year   = {2026},
  url    = {https://github.com/sargonxg/AGON},
  note   = {TACITUS},
}
```

---

*Maintainer: Giulio Catanzariti · [`giuliocatanzariti@gmail.com`](mailto:giuliocatanzariti@gmail.com) · TACITUS — making conflict legible.*
