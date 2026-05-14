# AGON — Operator's Guide

**Audience:** anyone who needs to start, stop, understand, or build on AGON.
**Status (2026-05-13):** MVP live; perception stack PROMPTS 01–04 partially implemented (contracts + text + time + lex).

---

## 1. What AGON actually does

AGON reads text — emails, transcripts, messages, depositions, board minutes — and produces a **typed, evidence-backed picture of the conflict inside it**: who said what, what was promised, what changed, where the contradictions are, what patterns are present (DARVO, anchoring, scope creep, conspicuous absence, coalition-forming), and what the next reviewer should ask.

It is not a chatbot. It is not a generic NLP pipeline. It is a **conflict perception engine** with the same shape as a self-driving stack:

```
RAW TEXT
   │
   ▼
L1  SENSORS       Rust deterministic — normalize, segment, detect quotes, lexical features, time
L2  ENCODERS      ONNX runtime — embeddings (BGE-M3), NLI (DeBERTa), coreference (fastcoref)
L3  EXTRACTION    Vertex Gemini + Claude — schema-constrained ACO primitives
L4  TRACKING      Rust — cross-doc actor resolution, commitment state machines, Allen-13 time
L5  SCENE         Hybrid — friction matrix + pattern library (5 patterns)
L6  CALIBRATION   Rust — per-detector temp/isotonic, conformal prediction for abstention
L7  PROVENANCE    Rust — typed lineage DAG, Merkle audit log, JSON-LD/Markdown export
L8  DECISION      Axum + SSE — quality gates, review questions, streaming UI
```

Every layer is independently testable. No single model is asked to do everything. **The chassis is Rust. ML models are interchangeable passengers behind typed traits.**

---

## 2. The 30-second start/stop

```bash
bash scripts/agon-up.sh        # turn it on  — ~30s
bash scripts/agon-down.sh      # turn it off — ~10s
bash scripts/agon-status.sh    # see what's running
bash scripts/agon-nuke.sh      # terraform destroy — DATA LOSS
```

PowerShell wrappers: `.\scripts\agon-up.ps1`, etc.

**Cost while UP:** ~$3–8/day active, mostly Vertex AI calls if you exercise the pipeline.
**Cost while DOWN:** ~$0.20–0.50/day (storage at rest, no compute).

**Typical "few days of testing":**

```bash
bash scripts/agon-up.sh
# … work, demo, evaluate …
bash scripts/agon-down.sh
```

Total: under **$30** for the active window. Your GCP startup credit covers this many times over.

---

## 3. What's already live (MVP v0.1.0)

Per `PROJECT_LEDGER/AGON_LEDGER.md`:

- **Cloud Run** `agon-dev` at `https://agon-dev-tbryoen6qa-uc.a.run.app` (Basic Auth `AGON / AGON`)
- **Cloud SQL Postgres 16** + `pgvector` extension
- **Vertex AI Gemini 2.5 Flash** for extraction (single-pass, schema-constrained JSON)
- **Workbench UI** with friction matrix, force-directed graph, history panel
- **Endpoints:** `/`, `/healthz`, `/readyz`, `/api/info`, `/api/perceive`, `/api/sessions`
- **Terraform-managed infra:** 32 resources in `tacitus-agon-dev` project
- **Cloud Build → Artifact Registry → Cloud Run** deploy pipeline (manual trigger working; GitHub auto-trigger pending)

What you can do **right now**:
1. `bash scripts/agon-up.sh`
2. Open the URL, log in
3. Paste a multi-turn dispute (Slack thread, email reply chain, board minutes)
4. See 8 ACO primitives extracted with verifiable spans + a friction score

---

## 4. The perception stack — what's been built since

| Crate | PROMPT | Status | What it does |
|---|---|---|---|
| `tacitus-contracts` | 01 | ✅ scaffolded + 8 schemas + Rust types + validation tests | Single source of truth for typed primitives |
| `aco-text` | 02 | ✅ v0.1: NFC + bidi/ZW strip + whitespace canon + segmenter + span verification | Canonical text foundation |
| `aco-time` | 03 | ✅ v0.1: Allen-13 calculator | Time intervals + temporal logic (regex DFA full impl pending) |
| `aco-lex` | 04 | ✅ v0.1: EN hedge/modality/passive/pronoun | Deterministic lexical features (5 langs full impl pending) |
| `aco-encode` | 05 | ☐ | ort 2.x + BGE-M3 + DeBERTa-NLI + fastcoref ONNX |
| `aco-llm` | 06 | ◐ partial (Vertex Gemini live; needs Claude + GPT-5 + routing.toml) | Vendor-portable LLM layer |
| `aco-extract` | 07 | ☐ | L1+L2+L3 perception pipeline |
| `aco-fuse`, `aco-temporal` | 08 | ☐ | Cross-doc actor resolution + commitment state machines |
| `aco-patterns` | 09 | ☐ | 5 patterns: DARVO, anchoring, scope creep, conspicuous absence, coalition |
| `aco-score` | 10 | ☐ | Calibration registry + conformal prediction |
| `aco-prov` | 11 | ☐ | Provenance DAG + Merkle audit + JSON-LD export |
| infra split | 12 | ☐ | Cloud Run CPU `agon-api` + GPU L4 `agon-batch` |
| workbench | 13 | ☐ | Pattern panels + correction capture |
| eval harness | 14 | ☐ | TCGC v0.2 + Inspect-AI + Argilla |
| adversarial | 15 | ☐ | Canonicalization + spotlighting + 80 cases |

Detail: `PROJECT_LEDGER/PERCEPTION_LEDGER.md`. Full spec: `docs/BUILD_PLAN_PERCEPTION.md`.

---

## 5. Everything you need externally

### 5.1 What's already done

| Item | Where | Status |
|---|---|---|
| GCP project `tacitus-agon-dev` | project 1086904791123 | ✓ |
| GCP billing + $startup credit | account `011452-1C91EA-384484` | ✓ |
| 17 GCP APIs enabled | via `make bootstrap` | ✓ |
| gcloud authed as `giulio@tacitus.me` | local + ADC | ✓ |
| GitHub `sargonxg/AGON` repo + `gh` auth | local | ✓ |
| Terraform-applied infra | `infra/terraform/envs/dev` | ✓ (32 resources) |
| HuggingFace token | `gcloud secrets versions access latest --secret=hf-token` | ✓ stored 2026-05-13 |

### 5.2 What you still need to give me

Ordered by when blocking:

| When | Item | How to get |
|---|---|---|
| PROMPT 05 (~day 11) | (HF token already done ✓) | needed for ONNX exports |
| PROMPT 06 | **Vertex Gemini quota uplift** | console.cloud.google.com → IAM & Admin → Quotas → `aiplatform.googleapis.com` → request Flash 600 RPM / 2M TPM, Pro 60 RPM / 300K TPM |
| ~~PROMPT 06~~ | ~~Anthropic API key~~ | **dropped 2026-05-13** — Gemini-only decision (see `docs/AUDIT_2026-05-13.md` F-10) |
| ~~PROMPT 06~~ | ~~OpenAI API key~~ | **dropped 2026-05-13** — same |
| PROMPT 12 (~day 60) | **L4 GPU quota** ≥ 4 in `us-central1` | same quotas console → `NVIDIA_L4_GPUS` |
| Any time | **Cloud Build ↔ GitHub** 2nd-gen connection | console.cloud.google.com/cloud-build/repositories/2nd-gen → "Connect repository" (browser OAuth) |
| PROMPT 14 (~day 80) | **Argilla annotation rounds** | 3 annotators × ~80h paid time |

### 5.3 What you don't need to worry about

- I have full IAM on `tacitus-agon-dev`
- I can run gcloud, terraform, cargo, git, gh
- I can write/run all code, tests, fixtures
- I can deploy via Cloud Build (manual trigger; auto pending #5 above)

---

## 6. Open-source and cheap-first choices (what I picked + why)

| Component | Choice | License | Why |
|---|---|---|---|
| Embeddings | BGE-M3 | Apache-2.0 / MIT | Dense + sparse + ColBERT in one model; multilingual; ONNX-exportable |
| NLI | DeBERTa-v3-large-mnli (MoritzLaurer) | MIT | Best open NLI checkpoint; INT8 quantizable |
| Coreference | fastcoref | MIT | License-clean; 78.5 F1 — good enough |
| Runtime | `ort` 2.x | MIT/Apache | Production-proven (TEI, Magika); pure Rust |
| Local LLM (when scale demands) | `mistral.rs` Q4_K_M | Apache-2.0 | OpenAI-compatible HTTP, paged attention |
| Time extraction | hand-rolled Rust DFA | — | HeidelTime/SUTime are GPL; we can't use them |
| Segmenter | hand-rolled SRX-style | — | pragmatic-segmenter is MIT but we own the impl |
| Lexical lexicons | BioScope + custom | redistributable | Avoid restrictive corpora |
| Postgres | self-hosted on Cloud SQL | OSS | $25/mo at dev tier |
| Vector store | `pgvector` extension | OSS | No managed vector DB needed |
| Annotation UI | Argilla (PROMPT 14) | Apache-2.0 | Self-host on Cloud Run |
| Eval orchestrator | Inspect-AI (UK AISI) | Apache-2.0 | Free, principled |
| LLM observability | Langfuse self-hosted | MIT | Self-host on Cloud Run vs $100/mo SaaS |

**Remote LLM** is the only paid layer. Vertex Gemini Flash at $0.30/$2.50 per million tokens is the cheapest schema-constrained option. Claude Haiku ($1/$5) is second opinion only. GPT-5 is fallback for stubborn schema cases.

---

## 7. Cost shape (real, not theoretical)

### Per "few-days" test cycle (3–5 days of active work)

| Component | Cost |
|---|---|
| Cloud Run CPU (active) | $2–5 |
| Cloud SQL db-f1-micro | ~$4 |
| GCS storage | <$1 |
| Cloud Build | ~$1.50 |
| Vertex Gemini Flash (a few hundred calls) | ~$13 |
| Vertex Gemini Pro (adjudication) | ~$3 |
| Logging / Trace | <$1 |
| **Total** | **~$25–35** |

### Per month (steady-state, single dev tenant, all PROMPTS landed)

| Component | Monthly |
|---|---|
| Cloud Run CPU `agon-api` | ~$45 |
| Cloud Run GPU L4 `agon-batch` (warm window only) | ~$485 |
| Cloud SQL | ~$25 |
| GCS audit + retention | ~$5 |
| Vertex Gemini Flash | ~$60 (1k folders) |
| Vertex Gemini Pro adjudication | ~$30 |
| Claude Haiku cross-val | ~$20 |
| Argilla on Cloud Run | ~$15 |
| **Total** | **~$685** |

Your startup credit covers this for many months. We add cost gates (per-folder LLM budget) in `aco-llm` at PROMPT 06.

---

## 8. Architecture decisions that won't change

These are locked. If something downstream conflicts, it loses.

1. **Rust chassis, ML passengers.** No Python sidecar. Every model behind a Rust trait.
2. **JSON Schema is source of truth.** `tacitus-contracts` is the only place primitives are defined. Rust/Python/TS regenerate from it.
3. **Evidence-span quad form.** Every claim-bearing primitive carries `(segment_id, canonical_offsets, raw_offsets, verbatim_quote, quote_hash, normalization_version)`. Non-negotiable.
4. **Calibration is mandatory.** Every detector emits raw signal; the calibration registry converts to probability. LLM verbalized confidence is a feature, never a probability.
5. **Per-doc perception, then cross-doc fusion.** Long-context Gemini is an adjudication tool, not the primary architecture.
6. **Pattern names are clinical internally, neutral publicly.** DARVO → "possible role-reversal pattern" in the UI. Ethics + legal.
7. **No training.** AGON is inference-only. Corrections corpus accumulates; training is deliberate future work (`aco-learn`).

---

## 9. Day-by-day flow (how to actually use it)

### Day 1 — turn it on, smoke test

```bash
bash scripts/agon-up.sh
curl https://agon-dev-tbryoen6qa-uc.a.run.app/healthz
# open in browser, log in (AGON/AGON), paste a dispute
bash scripts/agon-status.sh    # see what's running
```

### Day 2–4 — exercise the pipeline

- Run `/api/perceive` against varied corpora (slack, email, depositions)
- Watch the friction matrix + graph
- Note failures: missing actors, drifted spans, wrong polarity
- Capture corrections (when PROMPT 13 lands, this writes back to `corrections` table)

### Day 5 — turn it off

```bash
bash scripts/agon-down.sh
```

Cloud SQL stops. Cloud Run scales to zero. Daily cost: cents. Restart with `agon-up.sh` whenever.

### Done with the project entirely?

```bash
bash scripts/agon-nuke.sh
# Type "NUKE" to confirm. terraform destroy runs. Data lost. State preserved.
```

---

## 10. How to keep building

### If you're me (Claude)

PROMPTS 02–04 are stubbed (v0.1 quality). Next moves, in order:

1. **PROMPT 05 — aco-encode.** Needs HF token (have it). Download BGE-M3, DeBERTa-NLI, fastcoref. Export to ONNX. Validate cosine sim. Wire `ort` 2.x. Bench on CPU.
2. **PROMPT 06 — aco-llm extension.** Needs Anthropic + OpenAI keys (don't have yet). For now: harden the existing Vertex Gemini backend; add routing.toml; add MockBackend for tests; defer Claude/OpenAI backends until keys arrive.
3. **PROMPT 07 — aco-extract.** Wire L1+L2+L3 into a typed pipeline. Three modes (Fast/Standard/Deep).
4. Continue down the 15-prompt sequence in `docs/BUILD_PLAN_PERCEPTION.md`.

### If you're you (Giulio)

When you want to advance:

```bash
cd C:\Users\giuli\AGON
# pick the next PROMPT branch from PERCEPTION_LEDGER.md
git checkout -b sprint/05-aco-encode
# tell Claude: "execute PROMPT 05 from docs/BUILD_PLAN_PERCEPTION.md"
```

I'll do the work; you review the PR, merge, advance.

### If you hit a blocker that requires the Cloud Console

`docs/EXTERNALS.md` has the exact actions and where in the Console to take them.

---

## 11. Where things live (map of the repo)

```
AGON/
├── README.md                          ← scannable overview
├── docs/
│   ├── AGON_GUIDE.md                  ← this file (extensive)
│   ├── BUILD_PLAN_PERCEPTION.md       ← the 15-prompt spec (~1250 lines)
│   ├── DEPLOYMENT_GCP.md              ← target GCP topology
│   ├── EXTERNALS.md                   ← what you provide
│   ├── CONTRACTS.md                   ← (TODO: PROMPT 01 follow-up)
│   ├── REPRODUCIBILITY.md             ← (TODO: PROMPT 11)
│   ├── ETHICS.md                      ← (TODO: PROMPT 15)
│   └── MODEL_CARD.md                  ← (TODO: PROMPT 15)
├── PROJECT_LEDGER/
│   ├── AGON_LEDGER.md                 ← MVP v0.1.0 sprint (shipped)
│   ├── PERCEPTION_LEDGER.md           ← 15-prompt sprint tracker
│   ├── SESSION_LOG.md
│   └── STATE.json                     ← current state, blockers, tooling
├── crates/
│   ├── tacitus-contracts/             ← schemas + Rust types (PROMPT 01) ✓
│   ├── aco-text/                      ← normalize + segment + verify (PROMPT 02 v0.1)
│   ├── aco-time/                      ← Allen-13 (PROMPT 03 v0.1)
│   ├── aco-lex/                       ← hedge/modal/passive (PROMPT 04 v0.1)
│   ├── aco-core/                      ← types + provenance (MVP)
│   ├── aco-llm/                       ← Vertex Gemini backend (MVP; extend at PROMPT 06)
│   ├── aco-storage/                   ← Cloud SQL via sqlx (MVP)
│   ├── aco-server/                    ← Axum + workbench UI (MVP)
│   ├── aco-cli/                       ← agon-cli (MVP)
│   └── aco-{fuse,embed,perceive,infer,score,learn,bench}/   ← stubs from MVP scaffold
├── infra/
│   └── terraform/                     ← VPC + Cloud SQL + Cloud Run + GCS + Eventarc + IAM
├── scripts/
│   ├── agon-up.sh / .ps1              ← turn ON
│   ├── agon-down.sh / .ps1            ← turn OFF
│   ├── agon-status.sh / .ps1
│   ├── agon-nuke.sh                   ← terraform destroy
│   └── bootstrap.sh                   ← initial GCP project setup
├── migrations/                        ← Postgres schema
├── corpora/                           ← test inputs
├── Cargo.toml                         ← workspace + deps
├── Dockerfile, compose.yaml
├── Makefile                           ← `make bootstrap`, `make build`, etc.
└── .env.example                       ← copy to .env.local
```

---

## 12. The honest answers

**Q: Can Claude do this all autonomously?**
A: Almost. Interactive logins (Anthropic console, OpenAI console, GitHub App OAuth) need your browser. Quota uplifts need a Cloud Console form. Everything else — yes.

**Q: How risky is this for an hour-of-testing budget?**
A: Negligible. `bash scripts/agon-up.sh` → test → `bash scripts/agon-down.sh` is bounded by Vertex API spend. Set a daily budget alert at $5 if paranoid.

**Q: What's the moat?**
A: Not the models — they commoditize. The moat is the **typed primitive layer + evidence spine + calibration curves + corrections corpus + eval harness**. 70% of engineering goes there. See `BUILD_PLAN_PERCEPTION.md` §6.

**Q: Why Rust and not Python?**
A: Discipline. Typed contracts at the IO boundary. No silent panics. Predictable memory. Cold-start cheap. ML models live behind traits, not in the chassis. Python is for one-off model exports.

**Q: What about HuggingFace? Why do you need a token?**
A: To download model weights (BGE-M3, DeBERTa-NLI, fastcoref) one time, export them to ONNX, then bundle the ONNX into the `agon-batch` container. After that, no HF traffic at runtime. Already stored in Secret Manager as `hf-token`.

**Q: What about prompt injection / safety?**
A: PROMPT 15. Canonicalization (already lands in aco-text v0.1) + spotlighting (delimited untrusted input) + multi-model consensus on high-stakes primitives + 80-case adversarial pack in CI. Not optional.

**Q: When do we have a "fully working" AGON for conflict perception?**
A: PROMPT 07 (~day 45) is the inflection — that's when the full L1→L3 pipeline emits ACO primitives end-to-end with verifiable evidence. Patterns at PROMPT 09 (~day 55). Calibration at PROMPT 10 (~day 60). Production deploy split at PROMPT 12 (~day 70). Adversarial-hardened at PROMPT 15 (~day 90).

---

*Maintainer: Giulio Catanzariti · `<giuliocatanzariti@gmail.com>` · TACITUS · making conflict legible.*
