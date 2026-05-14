# AGON — GCP Deployment Architecture

**Status:** MVP live on a single Cloud Run service (`agon-dev-tbryoen6qa-uc.a.run.app`). This doc is the **target topology** built up through PROMPT 12 of the perception sprint.

---

## 1. Topology (target, post-PROMPT 12)

```
                            ┌──────────────────────────┐
                            │      Cloud DNS           │
                            │  agon.tacitus.me         │
                            └────────────┬─────────────┘
                                         │
                            ┌────────────▼─────────────┐
                            │  HTTPS Load Balancer +   │
                            │  Cloud Armor (WAF)       │
                            │  (PROMPT 12+)            │
                            └────────────┬─────────────┘
                                         │
       ┌─────────────────────────────────┼───────────────────────────────┐
       │                                 │                               │
┌──────▼──────────┐              ┌───────▼────────┐            ┌─────────▼────────┐
│ Cloud Run CPU    │              │ Cloud Run GPU  │            │  Cloud Run       │
│ agon-api         │  gRPC ───▶  │ agon-batch     │            │  argilla         │
│ 4 vCPU / 8 GB    │              │ L4 GPU         │            │  (annotation)    │
│ min=1 biz hrs    │              │ scale-to-zero  │            │  PROMPT 14       │
│ scale-to-zero    │              │ + warm window  │            └──────────────────┘
│ off-hours        │              └───────┬────────┘
└──┬───────────┬───┘                      │
   │           │                          │
   │           │      ┌───────────────────▼─────────────┐
   │           │      │  Vertex AI Model Garden         │
   │           │      │  gemini-2.5-flash / pro         │
   │           │      │  claude-haiku-4-5 / sonnet-4-6  │
   │           │      │  (gpt-5 via OpenAI fallback)    │
   │           │      └─────────────────────────────────┘
   │           │
   │           │      ┌───────────────────────────────┐
   │           └─────▶│  Cloud SQL Postgres 16        │
   │                  │  + pgvector ext               │
   │                  │  Private IP (VPC peering)     │
   │                  │  Secret Manager creds         │
   │                  └───────────────────────────────┘
   │
   │                  ┌───────────────────────────────┐
   ├─────────────────▶│  GCS buckets                  │
   │                  │  - tacitus-agon-documents-dev │
   │                  │  - tacitus-agon-exports-dev   │
   │                  │  - tacitus-agon-audit-dev     │
   │                  │    (retention lock, vers.)    │
   │                  └───────────────────────────────┘
   │
   │                  ┌───────────────────────────────┐
   └─────────────────▶│  Secret Manager               │
                      │  Eventarc (GCS finalized →    │
                      │   /api/eventarc/upload)       │
                      └───────────────────────────────┘

                      ┌───────────────────────────────┐
                      │ Cloud Logging + Cloud Trace   │
                      │ Langfuse (self-hosted on      │
                      │   Cloud Run, PROMPT 12)       │
                      │ Cost dashboard (PROMPT 12)    │
                      └───────────────────────────────┘
```

---

## 2. Service inventory

### 2.1 `agon-api` (live since MVP)
- **Runtime:** Cloud Run v2, region `us-central1`
- **Image:** `us-central1-docker.pkg.dev/tacitus-agon-dev/agon/agon-server:vX.Y.Z`
- **Resources:** 4 vCPU / 8 GB, max-instances 10, concurrency 80
- **Scaling:** min=1 (Mon–Fri 08–20 UTC), min=0 off-hours
- **Auth:** allUsers invoker (public-facing); per-tenant Basic Auth at app layer until OIDC lands
- **SA:** `agon-api-runtime@tacitus-agon-dev.iam.gserviceaccount.com`
- **VPC:** egress via Serverless VPC connector → Cloud SQL private IP
- **Endpoints:** `/`, `/healthz`, `/readyz`, `/api/info`, `/api/perceive`, `/api/sessions`, `/api/corrections` (PROMPT 13), `/api/sessions/{id}/audit` (PROMPT 11)

### 2.2 `agon-batch` (new at PROMPT 12)
- **Runtime:** Cloud Run v2 with NVIDIA L4 GPU
- **Image:** `agon-batch:vX.Y.Z` — bundles ONNX Runtime CUDA + BGE-M3 + DeBERTa-NLI + fastcoref weights (~2.5 GB)
- **Resources:** 8 vCPU / 32 GB / 1× L4
- **Scaling:** scale-to-zero; min=1 during business hours
- **Cold-start:** warm-up handler runs synthetic batch at startup; `/readyz` blocks until warm
- **Auth:** internal-only; only `agon-api` SA can invoke
- **Contract:** gRPC (`crates/aco-batch-rpc/proto/*.proto`)
  - `BatchEmbed`, `BatchNli`, `BatchCoref`

### 2.3 `argilla` (PROMPT 14)
- **Runtime:** Cloud Run v2, small CPU instance
- **Purpose:** human annotation UI for TCGC v0.2 + correction rounds
- **Auth:** Basic Auth; admin token in Secret Manager

### 2.4 Vertex AI
- **Models in use:**
  - `gemini-2.5-flash-002` — primary extraction
  - `gemini-2.5-pro` — adjudication on calibrated p < 0.7
  - `claude-haiku-4-5` (Model Garden) — cross-validation
  - `claude-sonnet-4-6` (Model Garden) — narrative frame, contradiction adjudication
  - `gpt-5` (direct OpenAI fallback) — strict-schema escalation
- **Pinning:** model_id always pinned to a specific version; recorded in every `ProvenanceRecord`
- **Cost guards:** per-folder budget enforced in `aco-llm`; daily aggregate alert at $X (configurable)

### 2.5 Cloud SQL Postgres 16
- **Tier:** db-f1-micro for dev; upgrade at first design-partner pilot
- **Extensions:** `pgvector`, `pg_trgm`, `uuid-ossp`
- **Schemas/tables:**
  - `sessions`, `extractions` (MVP)
  - `canonical_actors` (with embedding column) — PROMPT 08
  - `commitment_state_transitions` — PROMPT 08
  - `event_intervals` — PROMPT 08
  - `provenance_records`, `merkle_checkpoints` — PROMPT 11
  - `calibration_curves` — PROMPT 10
  - `corrections` — PROMPT 13
- **Backup:** automated daily, 7-day retention dev / 30-day prod
- **Access:** private IP only; secret-managed password

### 2.6 GCS
| Bucket | Purpose | Lifecycle |
|---|---|---|
| `tacitus-agon-documents-dev` | uploaded source documents | Standard → Nearline at 30d |
| `tacitus-agon-exports-dev` | JSON-LD + MD audit exports | Standard; pinned |
| `tacitus-agon-audit-dev` | provenance append-only + Merkle | **retention lock** 90d dev / 7y prod opt-in; object versioning ON |
| `tacitus-agon-models` (PROMPT 05) | ONNX weights + SHA-256 manifests | versioned |
| `tacitus-agon-calibration` (PROMPT 10) | large calibration curves | Standard |

### 2.7 Eventarc
- GCS `finalized` event on `*-documents-*` → POST `agon-api:/api/eventarc/upload` (MVP wired, untested at volume)

### 2.8 Cloud Build
- **Triggers:** push to `main` → build both `agon-api` and `agon-batch`; canary 10% traffic → 100% after `/readyz` + smoke
- **PR triggers:** build only, no deploy; run CI gates (cargo test, clippy, deny, eval pack from PROMPT 14)

---

## 3. Network

- Single VPC `agon-vpc` per environment
- Subnets: `us-central1-app`, `us-central1-db`
- **Private Service Connection** for Cloud SQL (no public IP)
- **Serverless VPC Access connector** for Cloud Run → Cloud SQL
- Cloud NAT for egress to Vertex / Anthropic / OpenAI APIs
- **Cloud Armor** (PROMPT 12+): WAF rules, rate limiting, IP allow-list for admin endpoints

---

## 4. IAM (least-privilege)

| SA | Roles |
|---|---|
| `agon-api-runtime` | `cloudsql.client`, `secretmanager.secretAccessor` (specific secrets), `aiplatform.user`, `storage.objectAdmin` (own buckets only), `run.invoker` on `agon-batch` |
| `agon-batch-runtime` | `aiplatform.user` (none — local ML only), `storage.objectViewer` (models bucket) |
| `cloudbuild-deployer` | `run.admin`, `artifactregistry.writer`, `iam.serviceAccountUser` (on runtime SAs only) |
| `eventarc-trigger` | `run.invoker` on `agon-api` |
| `terraform-apply` | granular per resource type (no project-level owner) |

**No human accounts** receive Cloud Run / Cloud SQL admin in dev. All ops via Terraform + Cloud Build.

---

## 5. Environments

| Env | GCP project | Promotion | Min-instances | Retention |
|---|---|---|---|---|
| `dev` | `tacitus-agon-dev` | every merge to `main` | 1 biz hrs / 0 off | 90 days audit |
| `staging` | `tacitus-agon-stg` (PROMPT 12) | tag `v*-rc*` | 1 biz hrs | 90 days |
| `prod` | `tacitus-agon-prod` (PROMPT 12) | tag `v*` after manual approval | 2 always | 7 years opt-in |

Terraform workspace per env; shared modules at `infra/terraform/modules/`.

---

## 6. Observability

- **Traces:** OpenTelemetry → Cloud Trace; every `/api/perceive` is a root span; LLM calls + encoder calls are child spans
- **Logs:** structured JSON → Cloud Logging; PII redacted at the boundary (LLM input/output bodies redacted by default; toggle with `--redact-llm-logs=false` flag)
- **Metrics:** Cloud Monitoring custom metrics — per-primitive latency, per-pattern detection rate, calibrated ECE rolling avg, Vertex spend
- **LLM observability:** Langfuse self-hosted on Cloud Run (PROMPT 12); captures prompt/response (redacted), token counts, cost
- **Alerting:** Cloud Monitoring → Slack
  - `5xx > 1%/5min` page
  - `Vertex spend > $X/day` email
  - `calibrated_ECE > 0.1` warning
  - `agon-batch cold-start TTFT > 30s` warning

---

## 7. Disaster recovery

- **Cloud SQL:** PITR enabled; backups in a separate region
- **GCS audit bucket:** retention lock + versioning; objects cannot be deleted within retention window (litigation-grade)
- **Terraform state:** versioned GCS bucket `tacitus-agon-dev-terraform-state` with object versioning (already done)
- **Image registry:** Artifact Registry with vulnerability scanning; old images retained per Cloud Run revision history
- **Secrets:** Secret Manager versioning ON; rotation playbook in `docs/RUNBOOK.md` (write at PROMPT 12)

---

## 8. Cost guardrails

- Per-folder LLM budget enforced in `aco-llm` (PROMPT 06): default $0.50/folder, configurable per-tenant
- Daily budget alert at $X (configurable per env) → email
- GPU service scale-to-zero outside business hours
- Cloud SQL right-sized at db-f1-micro until first paying pilot
- Vertex Pro adjudication only on calibrated_p < 0.7 (cost-gated by the calibration layer itself)

---

## 9. Deploy procedure (manual fallback)

When Cloud Build is down or for hotfix:

```bash
make build-api           # local docker build
make push-api            # to Artifact Registry
make deploy-api ENV=dev  # gcloud run deploy with canary
make verify ENV=dev      # hit /readyz + /api/info + a sample /api/perceive
```

Targets live in `Makefile` (already present; extend for `agon-batch` at PROMPT 12).

---

*Source of truth: this document. Terraform code at `infra/terraform/`. Live URL: <https://agon-dev-tbryoen6qa-uc.a.run.app>*
