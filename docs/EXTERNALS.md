# AGON — External Dependencies

What we need from outside the codebase to ship the perception stack. Action items are mine (Claude); items marked **[You]** require your hands.

---

## 1. Cloud accounts & quotas

### 1.1 GCP (primary)
- **[You]** Promote `tacitus-agon-dev` from sandbox to billed (already done — startup credit `011452-1C91EA-384484` attached).
- **[You]** Create `tacitus-agon-prod` project, link same billing, repeat APIs (deferred to PROMPT 12).
- **[You]** Vertex AI Gemini 2.5 quota uplift:
  - `gemini-2.5-flash` → 600 RPM, 2 M TPM
  - `gemini-2.5-pro` → 60 RPM, 300 K TPM
  - Console: IAM & Admin → Quotas → filter "aiplatform.googleapis.com"
- **[You]** GPU quota: `NVIDIA_L4_GPUS` ≥ 4 in `us-central1` (Cloud Run GPU service comes online at PROMPT 12).
- **[You]** Cloud Build ↔ GitHub 2nd-gen connection (still open from MVP ledger).

### 1.2 ~~Anthropic~~ — dropped 2026-05-13
- Decision: **Gemini-only**. Cross-validation done with Flash vs Pro at different temperatures / prompt versions.
- See `crates/aco-llm/config/routing.toml` and `docs/AUDIT_2026-05-13.md` §F-10.

### 1.3 ~~OpenAI~~ — dropped 2026-05-13
- Same decision. GPT-5 escalation path removed; if Vertex Gemini schema validity drops below 99% on a primitive, we add a deterministic repair pass (one retry with stricter prompt) before declaring failure.

### 1.4 Optional (defer until needed)
- **Sigstore / Rekor** — only if signed transparency log opt-in is requested by a design partner (PROMPT 11).
- **Langfuse Cloud** — vs. self-hosted on Cloud Run. Self-hosted is cheaper at MVP volume.

---

## 2. Domains & DNS

| Domain | Purpose | Status |
|---|---|---|
| `agon.tacitus.me` | Public API + workbench | **[You]** point CNAME to Cloud Run after PROMPT 12 |
| `tcgc.tacitus.me` | Argilla annotation UI | **[You]** point CNAME after PROMPT 14 |
| `audit.tacitus.me` | Public audit-export viewer (later) | deferred |

---

## 3. Model weights & exports

These are the manual one-time exports. Tracked in `crates/aco-encode/exports/README.md` (created in PROMPT 05).

| Model | Source | Target | Owner | Notes |
|---|---|---|---|---|
| BGE-M3 | `BAAI/bge-m3` (Apache-2.0) | ONNX FP16, opset 17 | Claude (PROMPT 05) | Validate cos-sim ≥ 0.999 vs HF Python |
| DeBERTa-v3-large-mnli | `MoritzLaurer/...mnli-fever-anli-ling-wanli` (MIT) | ONNX INT8 | Claude (PROMPT 05) | ≥ 99% label agreement vs FP32 on 1k MNLI dev |
| fastcoref | `pie/fastcoref` (MIT) | ONNX, mention head | Claude (PROMPT 05) | Budget 1–2 days; sidecar fallback if export fails |
| Arctic-Embed-L 2.0 | `Snowflake/snowflake-arctic-embed-l-v2.0` | ONNX | Claude (PROMPT 05, optional) | Fallback if BGE-M3 storage-constrained |

Storage: Artifact Registry generic repository `us-central1-docker.pkg.dev/tacitus-agon-dev/agon-models/`. Pinned by SHA-256, version label, and `MODEL_CARD.md` per model.

---

## 4. Licenses to verify before shipping

| Component | License | Verdict | Action |
|---|---|---|---|
| BGE-M3 | Apache-2.0 / MIT family | OK | ship |
| DeBERTa-v3-large MoritzLaurer | MIT | OK | ship |
| fastcoref | MIT | OK | ship |
| Maverick coref | CC-BY-NC-SA | **conditional** | only if SapienzaNLS clears commercial use |
| HeidelTime / SUTime | GPL | **forbidden** | reimplement rules in `aco-time` |
| pragmatic-segmenter | MIT | OK to study | rules reimplemented in `aco-text`, not code-ported |
| BioScope hedge cues | freely redistributable | OK | embed lexicon |
| Stanford Politeness corpus | check | **[You]** verify before PROMPT 04 | rebuild seeds from primary lit if restrictive |
| LEDGAR | mixed | partial | per-piece review before adding to TCGC v0.2 |

CI gate: `cargo deny check licenses` with GPL denylisted, enforced at PROMPT 03.

---

## 5. Secrets & ADC

All secrets live in **Secret Manager** in `tacitus-agon-dev`. Cloud Run service accounts receive `roles/secretmanager.secretAccessor` on the specific secret only — never broad.

| Secret | Consumer | Created at |
|---|---|---|
| `db-password` | aco-server Cloud SQL | MVP (done) |
| `jwt-signing-key` | aco-server auth | MVP (done) |
| `hf-token` | one-time ONNX model export (PROMPT 05) | done 2026-05-13 |
| `ed25519-signing-key` | aco-prov signatures (opt-in) | PROMPT 11 |
| `argilla-admin-token` | Argilla on Cloud Run | PROMPT 14 |

Local development: `gcloud auth application-default login` + `--quota-project=tacitus-agon-dev`. Never commit `.env` (already gitignored).

---

## 6. Human inputs needed

| Item | When | Owner | Notes |
|---|---|---|---|
| Annotation budget — 3 annotators × ~80h | PROMPT 14 | **[You]** | Argilla setup needs 3 raters per primitive for IAA |
| Design-partner pilot data | PROMPT 14 | **[You]** | Synthetic + 1–2 real-but-redacted case folders |
| Prompt versions sign-off | PROMPT 06, 09 | **[You]** | Hash-pinned senior-analyst persona prompts |
| Litigation-grade retention policy choice | PROMPT 11 | **[You]** | 90 days dev / 7 years prod opt-in |
| Pattern public-naming sign-off | PROMPT 09 | **[You]** | "possible role-reversal pattern" vs alternatives |
| Ethics / model card review | PROMPT 15 | **[You]** | Legal/marketing review before "deception" framing rejected |

---

## 7. Local dev tooling (one-time setup, Windows)

Already partially complete from MVP:
- ✓ gcloud SDK, gh, git
- ✓ rustup + Rust 1.83
- ✗ **terraform** — `winget install Hashicorp.Terraform`
- ✗ **make** — `choco install make` or use Git Bash
- ? **docker** — verify with `docker version`
- New for perception stack:
- ✗ **uv** (Python deps for Argilla, Inspect-AI, codegen) — `winget install astral-sh.uv`
- ✗ **typify-cli** — `cargo install typify-cli` (PROMPT 01)
- ✗ **datamodel-code-generator** — `uv tool install datamodel-code-generator` (PROMPT 01)
- ✗ **quicktype** — `npm i -g quicktype` (PROMPT 01)
- ✗ **optimum-cli** — `uv tool install optimum[onnxruntime]` (PROMPT 05, model export)

---

## 8. Budget envelope (steady-state estimate, single dev tenant)

| Component | Monthly | Notes |
|---|---|---|
| Cloud Run CPU `agon-api` min=1 business hrs | ~$45 | scale-to-zero off-hours |
| Cloud Run GPU L4 `agon-batch` warm window | ~$485 | one L4 warm 8h/day × 22 days |
| Cloud SQL Postgres db-f1-micro + pgvector | ~$25 | scale up at design-partner pilot |
| GCS audit bucket + retention | ~$5 | grows ~linearly with case folders |
| Vertex AI Gemini 2.5 Flash | ~$60 | at 1k folders/mo, ~6M in/50M out tokens |
| Vertex AI Gemini 2.5 Pro adjudication | ~$30 | ~10% of folders escalate |
| ~~Claude Haiku~~ | ~$0 | **dropped 2026-05-13** — Gemini-only |
| Argilla on Cloud Run (PROMPT 14+) | ~$15 | small CPU instance |
| **Total** | **~$665/mo** | covered by startup credits in dev |

Cost gates: per-folder budget enforced in `aco-llm` (PROMPT 06). Daily cost dashboard in Cloud Monitoring (PROMPT 12).

---

## 9. What I can do without you

- All Rust crate scaffolding, code, tests, fixtures.
- All Terraform changes (you apply via `terraform apply` since I can't run it; I'll write the plan).
- All Docker builds (locally; you submit via Cloud Build).
- All schema / contract authoring.
- Model export scripts (you run them; ONNX export needs HF download).

## 10. What I genuinely can't do without you

- Auth flows (Vertex / Anthropic / OpenAI keys, GitHub PAT, Cloud Build ↔ GitHub connection).
- Terraform `apply` in your shell (sandboxed).
- Quota & billing changes.
- Human annotation rounds.
- Sign-off on naming, ethics, legal framing.

---

*Maintainer: Giulio Catanzariti · giuliocatanzariti@gmail.com*
