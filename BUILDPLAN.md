# AGON Build Plan

*This document is for Claude Code (or any senior Rust coding agent). It is a day-by-day, task-by-task action plan for a GCP-native build. Each task has acceptance criteria. Each day produces a verifiable deliverable. The order is the order.*

**Read [ARCHITECTURE.md](./ARCHITECTURE.md) first.** That document is the technical contract. This document is the schedule.

This v3 supersedes earlier build plans. The major shifts vs v2: Cloud SQL replaces local Postgres from Day 1, Vertex AI replaces direct Gemini API, Terraform IaC and Cloud Build CI/CD are part of the foundation (not deferred to release week), and no part of the production runtime executes locally.

---

## How to use this document

- **You are building MVP v1.0.** Phase 1. Phases 2–4 are roadmap.
- **Three sprints of 7 days each.** 21 working days total.
- **Each task lists files to create or modify, dependencies, and acceptance criteria.**
- **Do not skip acceptance criteria.** They are the contract.
- **Commit at the end of every task** using [Conventional Commits](https://www.conventionalcommits.org/).
- **CI must stay green.** Every push to a branch runs `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` via Cloud Build (or GitHub Actions pre-PR; Cloud Build on merge). If a task breaks CI, fix CI before moving on.
- **No code runs in production locally.** Cloud Run runs the binary. Cloud SQL stores the data. Vertex AI extracts. Local Rust runs only for tests and for the `agon-cli` thin client.
- **Use `MockLlmBackend` for tests** that would burn Vertex AI quota. Live integration is gated behind `--features live-vertex` and run nightly.
- **Two GCP projects from Day 1.** One `dev`, one `prod`. Tests run against `dev`.
- **Ask questions when ambiguous.** Better to surface a design question than fit a wrong assumption.

---

## Pre-Sprint: GCP project bootstrap (Day 0)

This is the only day a human must touch GCP outside of `make` commands. After Day 0, everything is `git push` and `make`.

| # | Task | What | Acceptance |
|---|---|---|---|
| 0.1 | Create GCP projects | `tacitus-agon-dev`, `tacitus-agon-prod` (or your names) | Both projects exist with billing attached |
| 0.2 | Enable APIs | `gcloud services enable run.googleapis.com sqladmin.googleapis.com aiplatform.googleapis.com storage.googleapis.com secretmanager.googleapis.com cloudbuild.googleapis.com artifactregistry.googleapis.com eventarc.googleapis.com pubsub.googleapis.com cloudscheduler.googleapis.com servicenetworking.googleapis.com vpcaccess.googleapis.com logging.googleapis.com cloudtrace.googleapis.com monitoring.googleapis.com cloudresourcemanager.googleapis.com iam.googleapis.com` | All return `Operation finished successfully` |
| 0.3 | Quota uplift for Vertex AI Gemini | Console → IAM & Admin → Quotas → search "Gemini 2.5 Flash"; request 60 RPM → 600 RPM | Approved (usually instant) |
| 0.4 | Authenticate `gcloud` locally | `gcloud auth login && gcloud auth application-default login` | `gcloud config get-value project` returns dev project |
| 0.5 | Set ADC for Terraform | `gcloud auth application-default set-quota-project tacitus-agon-dev` | No quota errors when running terraform |
| 0.6 | Repo init | `git clone https://github.com/sargonxg/AGON.git && cd AGON` | Repo cloned |
| 0.7 | `Makefile` skeleton | `Makefile` | `make help` lists all targets |
| 0.8 | `.env.example` and `.env` | as named | All env vars documented in `.env.example`; user has filled `.env` |

### `.env.example`

```
GCP_PROJECT_ID=tacitus-agon-dev
GCP_REGION=us-central1
ENV=dev
GITHUB_OWNER=sargonxg
GITHUB_REPO=AGON
TF_BUCKET_NAME=                          # auto-set by bootstrap
LOG_LEVEL=info
```

### `Makefile` (essential targets)

```makefile
.PHONY: help bootstrap infra-plan infra-apply infra-destroy ci-connect deploy logs url status rollback test-local test-cloud

include .env
export

help:
	@echo "make bootstrap        — one-time GCP project setup"
	@echo "make infra-plan       — terraform plan"
	@echo "make infra-apply      — terraform apply"
	@echo "make infra-destroy    — terraform destroy (with confirm)"
	@echo "make ci-connect       — connect GitHub repo to Cloud Build"
	@echo "make deploy           — push to main (CI builds and deploys)"
	@echo "make logs             — tail Cloud Run logs"
	@echo "make url              — print deployed Cloud Run URL"
	@echo "make status           — show Cloud Run revisions"
	@echo "make rollback REVISION=<rev> — roll back to a previous revision"
	@echo "make test-local       — cargo test against testcontainers Postgres"
	@echo "make test-cloud       — integration tests against deployed dev env"

bootstrap:
	bash infra/bootstrap.sh

infra-plan:
	cd infra/terraform/envs/$(ENV) && terraform init && terraform plan

infra-apply:
	cd infra/terraform/envs/$(ENV) && terraform apply

infra-destroy:
	cd infra/terraform/envs/$(ENV) && terraform destroy

ci-connect:
	bash infra/connect-github.sh

deploy:
	git push origin main

logs:
	gcloud logging tail "resource.type=cloud_run_revision AND resource.labels.service_name=agon-$(ENV)" --project=$(GCP_PROJECT_ID)

url:
	@gcloud run services describe agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID) --format='value(status.url)'

url-raw:
	@gcloud run services describe agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID) --format='value(status.url)' | tr -d '\n'

status:
	gcloud run revisions list --service=agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID)

rollback:
	gcloud run services update-traffic agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID) --to-revisions=$(REVISION)=100

test-local:
	cargo test --workspace --all-features

test-cloud:
	cargo test --workspace --all-features --features live-vertex -- --ignored
```

### `infra/bootstrap.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail
source .env

echo "[bootstrap] Project: $GCP_PROJECT_ID, region: $GCP_REGION, env: $ENV"

# Enable APIs (idempotent)
gcloud services enable run.googleapis.com sqladmin.googleapis.com aiplatform.googleapis.com \
    storage.googleapis.com secretmanager.googleapis.com cloudbuild.googleapis.com \
    artifactregistry.googleapis.com eventarc.googleapis.com pubsub.googleapis.com \
    cloudscheduler.googleapis.com servicenetworking.googleapis.com vpcaccess.googleapis.com \
    logging.googleapis.com cloudtrace.googleapis.com monitoring.googleapis.com \
    cloudresourcemanager.googleapis.com iam.googleapis.com \
    --project=$GCP_PROJECT_ID

# Create Terraform state bucket (idempotent)
TF_BUCKET="${GCP_PROJECT_ID}-terraform-state"
if ! gcloud storage buckets describe "gs://${TF_BUCKET}" --project=$GCP_PROJECT_ID >/dev/null 2>&1; then
    gcloud storage buckets create "gs://${TF_BUCKET}" \
        --location=$GCP_REGION --project=$GCP_PROJECT_ID --uniform-bucket-level-access
    gcloud storage buckets update "gs://${TF_BUCKET}" --versioning --project=$GCP_PROJECT_ID
fi

# Grant Cloud Build SA the roles it needs (initial bootstrap; refined later by Terraform)
PROJECT_NUMBER=$(gcloud projects describe $GCP_PROJECT_ID --format='value(projectNumber)')
CB_SA="${PROJECT_NUMBER}@cloudbuild.gserviceaccount.com"
for role in roles/run.admin roles/iam.serviceAccountUser roles/artifactregistry.writer \
            roles/cloudsql.client roles/secretmanager.secretAccessor roles/storage.objectAdmin; do
    gcloud projects add-iam-policy-binding $GCP_PROJECT_ID \
        --member="serviceAccount:${CB_SA}" --role="${role}" --condition=None --quiet
done

# Update .env with bucket name
if ! grep -q "^TF_BUCKET_NAME=" .env || grep -q "^TF_BUCKET_NAME=$" .env; then
    sed -i.bak "s|^TF_BUCKET_NAME=.*|TF_BUCKET_NAME=${TF_BUCKET}|" .env
fi

echo "[bootstrap] Done. Next: cd infra/terraform/envs/$ENV && terraform init && terraform apply"
```

**End-of-day:** APIs enabled, Terraform state bucket created, Cloud Build SA has IAM, `make bootstrap` is idempotent and rerunnable.

---

## Sprint 1 — Foundations on GCP (Days 1–7)

**Sprint goal:** end-to-end smoke run on the deployed dev environment. A document uploaded via the dashboard is perceived, fused, stored in Cloud SQL, and produces at least one Friction Score visible on the dashboard. CI/CD pipeline works: `git push` → Cloud Build → Cloud Run revision.

### Day 1 — `aco-core`: types and provenance

| # | Task | Files | Acceptance |
|---|---|---|---|
| 1.1 | Cargo workspace + shared deps | `Cargo.toml`, `rust-toolchain.toml` | `cargo build` succeeds on empty workspace |
| 1.2 | `Id` and canonical hashing | `crates/aco-core/src/id.rs` | `Id::from_canonical(&primitive)` stable across runs |
| 1.3 | Common types | `crates/aco-core/src/common.rs` | All serde-roundtrip in proptest |
| 1.4 | Eight primitives | `crates/aco-core/src/{actor,claim,interest,constraint,leverage,commitment,event,narrative}.rs` | All serde-roundtrip; full ARCHITECTURE specs |
| 1.5 | Interpersonal extensions | `crates/aco-core/src/patterns.rs` | `PatternFinding`, `AffectMarker`, etc. complete |
| 1.6 | Enums + FOL + errors | `crates/aco-core/src/{enums,fol,error}.rs` | All variants from ARCHITECTURE |
| 1.7 | Property tests | `crates/aco-core/tests/proptest_roundtrip.rs` | 1000 iters per primitive |

**End-of-day:** `cargo test -p aco-core` passes; `cargo doc -p aco-core` clean.

### Day 2 — Terraform IaC: provision the dev environment

This day creates real GCP resources. Iterate with `terraform plan` until it's clean, then `terraform apply`.

| # | Task | Files | Acceptance |
|---|---|---|---|
| 2.1 | Terraform skeleton | `infra/terraform/{main.tf,variables.tf,outputs.tf}` | `terraform init` succeeds against GCS backend |
| 2.2 | APIs module | `infra/terraform/modules/apis/main.tf` | All required APIs as `google_project_service` |
| 2.3 | Network module | `infra/terraform/modules/network/main.tf` | VPC + subnet + firewall + Private Service Connection for Cloud SQL |
| 2.4 | Cloud SQL module | `infra/terraform/modules/cloud_sql/main.tf` | Postgres 16, private IP, `db-f1-micro` for dev, automated backups, `vector` + `ltree` flags |
| 2.5 | Storage module | `infra/terraform/modules/storage/main.tf` | Documents bucket + exports bucket + Eventarc Pub/Sub topic |
| 2.6 | Artifact Registry module | `infra/terraform/modules/artifact_registry/main.tf` | Docker repo at `${region}-docker.pkg.dev/${project}/agon` |
| 2.7 | IAM module | `infra/terraform/modules/iam/main.tf` | Cloud Run, Cloud Build, Eventarc SAs with least-privilege bindings |
| 2.8 | Secrets module | `infra/terraform/modules/secrets/main.tf` | `agon-db-password`, `agon-jwt-signing-key` |
| 2.9 | Cloud Run module (placeholder) | `infra/terraform/modules/cloud_run/main.tf` | Service + Job; initial image `gcr.io/cloudrun/hello` |
| 2.10 | Eventarc module | `infra/terraform/modules/eventarc/main.tf` | GCS Object Finalized → Cloud Run trigger |
| 2.11 | Env-specific tfvars | `infra/terraform/envs/dev/{terraform.tfvars,backend.tf}` | `terraform plan` clean |
| 2.12 | Apply | `make infra-apply` | All resources created in GCP project |

**End-of-day:** `make url` returns a Cloud Run URL; visiting it shows the placeholder hello page. Cloud SQL instance is RUNNABLE. The buckets exist. IAM is configured.

### Day 3 — `aco-llm`: Vertex AI Gemini + Mock

| # | Task | Files | Acceptance |
|---|---|---|---|
| 3.1 | `LlmBackend` trait | `crates/aco-llm/src/backend.rs` | Async-trait bounds correct |
| 3.2 | `VertexAiBackend` | `crates/aco-llm/src/vertex.rs` | `--features live-vertex` test passes against real Vertex AI |
| 3.3 | Service-account auth | `crates/aco-llm/src/auth.rs` | Uses `gcp_auth` or `google-cloud-auth`; reads from metadata server in Cloud Run, ADC locally |
| 3.4 | `MockLlmBackend` | `crates/aco-llm/src/mock.rs` | Replays fixtures from `tests/fixtures/` |
| 3.5 | Retry with backoff | `crates/aco-llm/src/retry.rs` | Transient errors retried |
| 3.6 | Rate limiter | `crates/aco-llm/src/rate.rs` | Configurable RPM; shared across extractors |
| 3.7 | Cost ledger | `crates/aco-llm/src/cost.rs` | Tracks tokens by model |
| 3.8 | Response cache | `crates/aco-llm/src/cache.rs` | Cached via Postgres in Day 4; in-memory for now |
| 3.9 | Embedding crate | `crates/aco-embed/src/local.rs` | `fastembed` BAAI/bge-small-en-v1.5 |
| 3.10 | Tests | `crates/aco-{llm,embed}/tests/` | Unit + mock-replay; live test gated |

**End-of-day:** Mock-driven structured extraction works. Live Vertex AI call against dev project succeeds.

### Day 4 — `aco-storage`: Cloud SQL via sqlx

| # | Task | Files | Acceptance |
|---|---|---|---|
| 4.1 | Initial migration | `migrations/001_init.up.sql` (+ `.down.sql`) | Full schema from ARCHITECTURE §7.3 |
| 4.2 | `Pool` and connection mgmt | `crates/aco-storage/src/pool.rs` | `sqlx` pool sized for Cloud Run; Secret Manager-sourced password |
| 4.3 | Repos per primitive | `crates/aco-storage/src/repo/*.rs` | CRUD per type via `sqlx::query!` (compile-time checked) |
| 4.4 | Provenance + spans + edges + audit | `crates/aco-storage/src/repo/{provenance,spans,edges,audit}.rs` | Insert atomically; queryable |
| 4.5 | In-memory `petgraph` mirror | `crates/aco-storage/src/memory.rs` | Hydrate from SQL; sync on write |
| 4.6 | `LISTEN`/`NOTIFY` channel | `crates/aco-storage/src/notify.rs` | Cross-instance event propagation |
| 4.7 | DB integration tests | `crates/aco-storage/tests/db.rs` | `testcontainers::Postgres` for local; nightly job tests against Cloud SQL |
| 4.8 | LLM cache table | `migrations/001_init.up.sql` (extend) | `aco-llm` cache uses Postgres |

**End-of-day:** `cargo test -p aco-storage` passes against testcontainers. The dev Cloud SQL instance has the migrations applied. The `aco-llm` cache reads/writes to Postgres.

### Day 5 — `aco-perceive` + `aco-fuse`: perception and fusion

| # | Task | Files | Acceptance |
|---|---|---|---|
| 5.1 | `Extractor` trait + orchestrator | `crates/aco-perceive/src/{lib.rs,orchestrator.rs}` | Runs N extractors concurrently with shared rate limit |
| 5.2 | Entity, event, claim extractors | `crates/aco-perceive/src/{entity,event,claim}.rs` + schemas + prompts | Snapshot tests on fixture chunks |
| 5.3 | Affect, pattern, temporal extractors | `crates/aco-perceive/src/{affect,pattern,temporal}.rs` + schemas + prompts | Snapshot tests |
| 5.4 | Document loaders | `crates/aco-perceive/src/loaders/{txt,md,pdf,docx}.rs` | Load 50-page PDF; returns chunks |
| 5.5 | Chunk planner | `crates/aco-perceive/src/planner.rs` | Splits at heading/paragraph; ≤ 150k tokens/chunk |
| 5.6 | Verify-and-repair loop | `crates/aco-perceive/src/repair.rs` | Span check, ref check, type check; up to 2 repairs |
| 5.7 | Canonical hash signatures | `crates/aco-fuse/src/signature.rs` | Per-primitive normalisation rules |
| 5.8 | Embedding signatures + HNSW | `crates/aco-fuse/src/ann.rs` | `hnsw_rs` index; rebuilds from Cloud SQL on cold start |
| 5.9 | Entity resolver | `crates/aco-fuse/src/resolve/entity.rs` | Exact-hash + ANN + LLM tiebreaker |
| 5.10 | Event coreference + claim dedup | `crates/aco-fuse/src/resolve/{event,claim}.rs` | Snapshot tests |
| 5.11 | Alias graph | `crates/aco-storage/src/repo/aliases.rs` | Every merge writes; query exposed |
| 5.12 | E2E fusion test | `crates/aco-fuse/tests/e2e.rs` | Workplace-dispute fixture: 47 raw actors → ≤ 15 canonical |

**End-of-day:** Run perceive + fuse on a fixture in-process; primitives land in Cloud SQL with alias graph populated.

### Day 6 — `aco-infer` (first pass) + `aco-score` (first pass) + `aco-server` (skeleton)

| # | Task | Files | Acceptance |
|---|---|---|---|
| 6.1 | Engine entry point | `crates/aco-infer/src/lib.rs` | `infer(graph) -> InferenceReport` |
| 6.2 | First Datalog rules | `crates/aco-infer/src/rules/{leverage,gaps,coalition,temporal}.rs` | Snapshot tests |
| 6.3 | Score: friction, power, trust | `crates/aco-score/src/{friction,power,trust}.rs` | Returns scores with feature attribution |
| 6.4 | Score persistence | `crates/aco-storage/src/repo/scores.rs` | Writes to `scores` table |
| 6.5 | Axum server skeleton | `crates/aco-server/src/{main.rs,state.rs,routes.rs}` | Binds `$PORT`; healthz/readyz endpoints |
| 6.6 | `POST /api/upload-url` | `crates/aco-server/src/api/upload.rs` | Returns signed Cloud Storage URL |
| 6.7 | `POST /api/eventarc/upload` | `crates/aco-server/src/api/eventarc.rs` | Verifies OIDC token; ingests the file |
| 6.8 | Embedded dashboard shell | `crates/aco-server/assets/{index.html,app.js,styles.css}` + `embed.rs` | Static page served at `/` |
| 6.9 | Tests | unit + integration | passing |

**End-of-day:** A document POSTed to a signed URL triggers Eventarc → perception → fusion → storage → first scores computed. The dashboard shell loads.

### Day 7 — Cloud Build CI/CD + first deploy + first scenario

| # | Task | Files | Acceptance |
|---|---|---|---|
| 7.1 | Dockerfile | `infra/Dockerfile` | Multi-stage; image ≤ 250 MB |
| 7.2 | `cloudbuild.yaml` | `infra/cloudbuild.yaml` | Test + build + push + migrate + deploy |
| 7.3 | `make ci-connect` script | `infra/connect-github.sh` | Creates Cloud Build GitHub trigger |
| 7.4 | First successful Cloud Build run | git push origin main | Build green; new Cloud Run revision serves |
| 7.5 | `agon-cli` thin client | `crates/aco-cli/src/{main.rs,client.rs}` | Calls deployed API over HTTPS; commands: `ingest`, `status`, `brief` |
| 7.6 | First scenario corpus | `corpora/workplace_dispute/` | 30 pages synthetic but realistic |
| 7.7 | Snapshot expected outputs | `corpora/workplace_dispute/expected_*.json` | Diff-able |
| 7.8 | E2E integration test against dev | `crates/aco-cli/tests/workplace_dispute_cloud.rs` | Gated behind `--features live-cloud`; runs against deployed dev env |
| 7.9 | First demo recording (internal) | `RECORDINGS.md` | 90-second clip; full pipeline on dev environment |

**End-of-sprint check:**
- `cargo test --all` green
- `git push origin main` → Cloud Build green → new Cloud Run revision in ≤ 12 minutes
- `agon-cli --api $(make url-raw) ingest corpora/workplace_dispute/` works
- Dashboard at `make url` shows the world model and at least one Friction Score
- Internal 90-second demo recording exists

---

## Sprint 2 — Deep inference, scoring, dashboard (Days 8–14)

**Sprint goal:** all five wow moments work end-to-end on the deployed dev environment. The dashboard renders the live graph, friction heatmap, contradiction view, DARVO sequence, brief generator. By end of sprint, the demo video is recordable against the deployed instance.

### Day 8 — Defeasible reasoning (ASPIC+)

| # | Task | Files | Acceptance |
|---|---|---|---|
| 8.1 | Defeasible rule registry | `crates/aco-infer/src/rules/defeasible/mod.rs` | Per-rule priority lattice |
| 8.2 | Rebut, undercut, undermine | `crates/aco-infer/src/rules/defeasible/attacks.rs` | Snapshot tests on toy theories |
| 8.3 | Grounded extension | `crates/aco-infer/src/rules/defeasible/grounded.rs` | Equivalent to ASPIC+ on Lam et al. 2016 examples |
| 8.4 | Integration with gaps | `crates/aco-infer/src/rules/defeasible/integrate.rs` | Defeasible gaps can be overridden |

### Day 9 — Z3 contradiction detection

| # | Task | Files | Acceptance |
|---|---|---|---|
| 9.1 | FOL → Z3 encoder | `crates/aco-infer/src/contradict/encode.rs` | All FOL variants encoded |
| 9.2 | Per-actor consistency | `crates/aco-infer/src/contradict/actor.rs` | Unsat-core with correct claim IDs |
| 9.3 | Deontic conflict | `crates/aco-infer/src/contradict/deontic.rs` | Snapshot |
| 9.4 | Optional FOL extraction | `prompts/fol_v1.md`, `crates/aco-perceive/src/fol.rs` | Populates `Claim.logical_form` when `--extract-fol` |
| 9.5 | Snapshot tests | `crates/aco-infer/tests/contradict.rs` | 2 unsat, 1 sat |

### Day 10 — Patterns + remaining scores

| # | Task | Files | Acceptance |
|---|---|---|---|
| 10.1 | Rule-based DARVO detector | `crates/aco-infer/src/rules/patterns/darvo.rs` | Snapshot |
| 10.2 | Rule-based Four Horsemen | `crates/aco-infer/src/rules/patterns/horsemen.rs` | Snapshot per horseman |
| 10.3 | Pattern reconciliation | `crates/aco-infer/src/rules/patterns/reconcile.rs` | Extractor + rule cross-check |
| 10.4 | Repair-capacity score | `crates/aco-score/src/repair.rs` | Snapshot |
| 10.5 | Bid-turn ratio | `crates/aco-score/src/bid_turn.rs` | Snapshot |
| 10.6 | Toxicity index | `crates/aco-score/src/toxicity.rs` | Per-actor |
| 10.7 | Risk (logistic) | `crates/aco-score/src/risk.rs` | Coefficients seeded from literature; calibrated 0-100 |

### Day 11 — BATNA/ZOPA + abduction

| # | Task | Files | Acceptance |
|---|---|---|---|
| 11.1 | `NegotiationModel` types | `crates/aco-infer/src/negotiation/model.rs` | Constructable from dyad |
| 11.2 | Utility-proxy estimator | `crates/aco-infer/src/negotiation/utility.rs` | Snapshot |
| 11.3 | ZOPA LP | `crates/aco-infer/src/negotiation/zopa.rs` | Feasibility + infeasibility cases |
| 11.4 | Mediation move | `crates/aco-infer/src/negotiation/mediation.rs` | Highest-impact axis |
| 11.5 | `Gap` enum + prompts | `crates/aco-infer/src/abduction/{gap,prompt}.rs`, `prompts/abduction/*.md` | 6 gap types |
| 11.6 | Candidate ranking | `crates/aco-infer/src/abduction/rank.rs` | Confidence + Z3 + embedding |
| 11.7 | Re-injection + cycle guard | `crates/aco-infer/src/abduction/orchestrator.rs` | Bounded |

### Day 12 — Dashboard upgrades + brief generator

| # | Task | Files | Acceptance |
|---|---|---|---|
| 12.1 | Cytoscape graph | `crates/aco-server/assets/graph.js` | Force-directed; node shape per type; click → evidence pane |
| 12.2 | d3 timeline | `crates/aco-server/assets/timeline.js` | Events + claims + affect overlay |
| 12.3 | Friction heatmap | `crates/aco-server/assets/heatmap.js` | Actor × actor matrix; click drill-down |
| 12.4 | Score cards | `crates/aco-server/assets/cards.js` | Feature attribution bars |
| 12.5 | Evidence pane | `crates/aco-server/assets/evidence.js` | Verbatim spans for any selected primitive |
| 12.6 | WebSocket live updates | `crates/aco-server/src/ws.rs` + `assets/ws.js` | `WorldEvent`s render incrementally |
| 12.7 | Tera brief templates | `prompts/briefs/{mediator_prep,legal,hr,therapist_prep,exec}.tera` | Render from graph |
| 12.8 | Brief endpoint | `crates/aco-server/src/api/brief.rs` | `POST /api/brief` returns markdown + signed download URL |

### Day 13 — Live demo dry-run on deployed dev environment

| # | Task | Files / Action | Acceptance |
|---|---|---|---|
| 13.1 | Push to main | `git push origin main` | Cloud Build green; new revision serves |
| 13.2 | Upload demo corpus via dashboard | `corpora/demo_workplace_dispute/` | All five extractors run; world model populated |
| 13.3 | Verify all five wow moments | screen capture | Heatmap drill-down, contradiction, DARVO, abduction, risk |
| 13.4 | Brief generation | dashboard | All four templates produce valid markdown with citations |
| 13.5 | First demo recording (internal) | OBS Studio | 5–10 minute recording captured |

### Day 14 — Ask mode + remaining pragmatics

| # | Task | Files | Acceptance |
|---|---|---|---|
| 14.1 | Query AST | `crates/aco-server/src/ask/ast.rs` | Typed query |
| 14.2 | Vertex AI → AST translator | `crates/aco-server/src/ask/translate.rs` | Constrained output |
| 14.3 | AST → SQL compiler | `crates/aco-server/src/ask/compile.rs` | Parameterised |
| 14.4 | Renderer with citations | `crates/aco-server/src/ask/render.rs` | Click-through |
| 14.5 | `POST /api/ask` | `crates/aco-server/src/api/ask.rs` | End-to-end |
| 14.6 | Scalar implicature table | `crates/aco-infer/src/rules/pragmatics/scalar.rs` | ≥ 80 entries |
| 14.7 | Evasion, non-sequitur, frame conflict, silence analysis | `crates/aco-infer/src/rules/pragmatics/*.rs` + `silence.rs` | Snapshots |

**End-of-sprint check:**
- All five wow moments work on the deployed dev environment
- Dashboard renders all views
- A 5–10 minute internal demo recording exists, recorded against the live Cloud Run instance

---

## Sprint 3 — Learning, hardening, release (Days 15–21)

### Day 15 — Learning loop

| # | Task | Files | Acceptance |
|---|---|---|---|
| 15.1 | Correction log schema | `migrations/002_corrections.sql` | Postgres table |
| 15.2 | Correction API + UI | `crates/aco-server/src/api/correction.rs`, `assets/correction.js` | User can mark wrong-extraction, wrong-merge, wrong-pattern |
| 15.3 | Active learning queue | `crates/aco-learn/src/queue.rs` | Sorted by impact |
| 15.4 | `agon-cli review` | `crates/aco-cli/src/cmd/review.rs` | Surfaces low-confidence items |
| 15.5 | Retraining trigger | `crates/aco-learn/src/trigger.rs` | Threshold-based |
| 15.6 | Prompt-version pinning | `prompts/manifest.json` | Deployed versions tracked |
| 15.7 | Few-shot example bank | `crates/aco-learn/src/few_shot.rs` | Pinned into next prompt version |

### Day 16 — Performance on Cloud Run

| # | Task | Files | Acceptance |
|---|---|---|---|
| 16.1 | Parallelise hot rules with `ascent_par!` | `crates/aco-infer/src/rules/*.rs` | Closure ≤ 600 ms on 50k primitives |
| 16.2 | Profile extraction on Cloud Run | `crates/aco-bench/benches/extract.rs` | Fix 3 slowest hot paths |
| 16.3 | Vertex AI context caching | `crates/aco-llm/src/cache.rs` | 90% prefix hit on second run of same dossier |
| 16.4 | Reduce allocations in fusion | `crates/aco-fuse/src/*.rs` | criterion ≥ 10% improvement |
| 16.5 | Postgres index audit | `migrations/` | All hot queries use indices; document `EXPLAIN ANALYZE` |
| 16.6 | Connection pool tuning | `crates/aco-storage/src/pool.rs` | Sized for Cloud Run concurrency × max instances |
| 16.7 | Cloud Run min instances trial | `infra/terraform/envs/dev` | Test min=0 vs min=1 cold-start trade-off |
| 16.8 | Document numbers | `docs/src/performance.md` | All ARCHITECTURE §15 targets validated |

### Day 17 — Robustness and security

| # | Task | Files | Acceptance |
|---|---|---|---|
| 17.1 | Property tests across rules | `crates/aco-infer/tests/prop_*.rs` | Monotonicity, termination |
| 17.2 | Fuzz JSON-schema repair | `crates/aco-perceive/fuzz/` | 10M iters, no panics |
| 17.3 | Fuzz PDF loader | `crates/aco-perceive/fuzz/pdf.rs` | No panics on malformed PDFs |
| 17.4 | Timeout + cancellation audit | all async paths | Every long task respects cancellation |
| 17.5 | Prompt-injection defence | system prompts | Doc text explicitly framed as data |
| 17.6 | `cargo audit` + `cargo deny check` | CI | No critical advisories |
| 17.7 | Secret audit | all crates | `secrecy::SecretString` everywhere; no logs leak |
| 17.8 | IAM review | `infra/terraform/modules/iam/` | Least privilege; no `roles/owner` or `roles/editor` |
| 17.9 | Cloud SQL audit logging | `infra/terraform/modules/cloud_sql/` | Enabled |
| 17.10 | Threat model | `docs/src/security/threat-model.md` | Documented |

### Day 18 — Five golden corpora

| # | Task | Files | Acceptance |
|---|---|---|---|
| 18.1 | Workplace dispute (done) | `corpora/workplace_dispute/` | snapshot stable |
| 18.2 | Co-parenting / divorce | `corpora/coparenting/` | Friction trajectory + Four Horsemen |
| 18.3 | HR investigation | `corpora/hr_investigation/` | DARVO + power asymmetry |
| 18.4 | Multi-party mediation | `corpora/mediation_treaty/` | BATNA/ZOPA + coalition |
| 18.5 | Diplomatic transcript | `corpora/diplomatic/` | Implicature + contradiction |
| 18.6 | Per-corpus READMEs | `corpora/*/README.md` | What to look for, how to record |
| 18.7 | All corpora pass on deployed dev | E2E test | Snapshots stable against live Vertex AI |

### Day 19 — Production environment + observability

| # | Task | Files | Acceptance |
|---|---|---|---|
| 19.1 | Prod tfvars | `infra/terraform/envs/prod/terraform.tfvars` | `db_tier=db-g1-small`, `db_ha=true` |
| 19.2 | Prod apply | `ENV=prod make infra-apply` | Prod env provisioned |
| 19.3 | Prod Cloud Build trigger | Terraform | Triggers on `release/*` tags |
| 19.4 | Custom domain (optional) | Cloud Run domain mapping | `agon.tacitus.me` resolves |
| 19.5 | Cloud Monitoring dashboard | `infra/terraform/modules/observability/dashboard.tf` | Per-pipeline-stage latency, error rate, cost |
| 19.6 | Alerts | `infra/terraform/modules/observability/alerts.tf` | Error rate > 1%, P95 latency > 5s, budget > 80% |
| 19.7 | Budget alerts | `infra/terraform/modules/observability/budget.tf` | Email at 50%, 90%, 100% |
| 19.8 | OpenTelemetry integration | `crates/aco-server/src/telemetry.rs` | Traces export to Cloud Trace |
| 19.9 | Production runbook | `docs/src/runbook.md` | Incidents: cold start, DB exhaustion, Vertex AI quota, rollback |

### Day 20 — Documentation

| # | Task | Files | Acceptance |
|---|---|---|---|
| 20.1 | mdbook setup | `docs/book.toml`, `docs/src/SUMMARY.md` | `mdbook build` produces site |
| 20.2 | Concepts | `docs/src/concepts/{aco,inference,scoring,patterns,fusion}.md` | Each ≥ 800 words |
| 20.3 | Tutorial | `docs/src/tutorial.md` | Quickstart with screenshots from deployed env |
| 20.4 | Cookbook | `docs/src/cookbook/*.md` | One per corpus |
| 20.5 | Operations | `docs/src/ops/{deploy,monitoring,rollback,scaling,cost}.md` | Each topic covered |
| 20.6 | API docs | `cargo doc` | 100% public-item coverage |
| 20.7 | Examples | `examples/*.rs` | ≥ 3 runnable |

### Day 21 — Release and final demo

| # | Task | Files / Action | Acceptance |
|---|---|---|---|
| 21.1 | Tag `v0.1.0` | git tag | Pushed; Cloud Build deploys to prod |
| 21.2 | Release notes | `CHANGELOG.md`, GitHub release | Lists capabilities and limits |
| 21.3 | Final demo recording | OBS, against prod | 5–15 minutes; full pipeline + 5 wow moments + brief + learning |
| 21.4 | tacitus.me embed snippet | `docs/src/embed.md` | iframe or script |
| 21.5 | Public README badges | `README.md` | Build status, version, docs |
| 21.6 | Internal launch note | `LAUNCH.md` | Investor/advisor summary |

**End-of-sprint check:**
- `cargo test --all-features` green
- `cargo bench` within targets
- prod environment serves at custom domain (or `*.run.app`)
- v0.1.0 tagged
- Final demo video against prod exists
- Cost monitoring active

---

## Demo recording protocol

This is the exact sequence for the launch video. See [README.md](./README.md#the-mvp-demo-recording--against-the-deployed-instance) for the narrated version.

### Pre-flight
1. Latest revision deployed to dev or prod (`make status`)
2. `corpora/demo_workplace_dispute/` available
3. Vertex AI context cache pre-warmed (run scenario once)
4. OBS Studio: 1920×1080@60fps; scenes for browser, terminal, split, GCP console
5. Browser bookmarks: dashboard URL, GCP console (Cloud Run, Cloud SQL, Cloud Logging)
6. Voice-over notes ready

### Take 1 — full pipeline (≈90 seconds compute)
1. Open dashboard at `make url`
2. Drag `corpora/demo_workplace_dispute/` onto upload zone
3. Signed-URL upload completes; Eventarc trigger fires
4. Six extractor cards light up; graph populates
5. Fusion panel: alias merges
6. Cognition: friction heatmap fills; pattern findings stream
7. Wow 1 — heatmap drill-down → score breakdown → feature → spans
8. Wow 2 — contradiction badge → two quotes side by side
9. Wow 3 — DARVO sequence with each turn highlighted in source
10. Wow 4 — unbacked-commitment gap with three abduced interests
11. Wow 5 — risk trajectory with feature attribution
12. Brief → mediator-prep → generate → download
13. Correction: mark a low-confidence pattern as wrong → toast confirms
14. Cut to GCP console: Cloud Run autoscale, Cloud SQL connections, Cloud Logging stream
15. Close

### Take 2 — cinematic shots
- Graph growing live (60 s slow zoom)
- Heatmap colour transitioning (30 s)
- Two contradiction quotes side by side (15 s)
- GCP console with Cloud Run scaling (20 s)

### Post
- Voice-over in DaVinci Resolve
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
- [ ] All performance targets met or documented as deferred
- [ ] `make bootstrap` is idempotent and works on a fresh GCP project
- [ ] `make infra-apply` provisions dev and prod environments cleanly
- [ ] `git push origin main` builds and deploys dev in ≤ 12 minutes
- [ ] `git tag v0.x.y && git push --tags` deploys prod
- [ ] Dashboard renders all five views on the deployed instance
- [ ] Brief generation works for all four templates
- [ ] `agon-cli` works against the deployed API
- [ ] Correction logging and active learning queue work
- [ ] Cloud Monitoring dashboards exist; budget alerts configured
- [ ] mdbook documentation complete and published
- [ ] 5–15 minute demo video against the deployed instance exists
- [ ] LICENSE committed (or decision documented to delay)
- [ ] `CHANGELOG.md` records 0.1.0
- [ ] `LAUNCH.md` drafted

---

## Post-MVP

Phases 2–4 from [README.md](./README.md#roadmap--beyond-the-mvp). Each phase will get its own BUILDPLAN. The architecture is designed so that:

- **Phase 2** adds `aco-predict`, `aco-counterfactual` crates without modifying core or infra
- **Phase 3** adds `aco-watch`, `aco-strategist`, `aco-anomaly` crates; Eventarc routes from Gmail/Slack/X connectors
- **Phase 4** adds multi-tenancy via Cloud SQL row-level security, GraphQL API at server layer, optional AlloyDB migration, Firebase Hosting for dashboard CDN, Scallop FFI for probabilistic reasoning, multi-lingual prompts

None of these require breaking changes to `aco-core` or `aco-storage`. The ontology and the schema are stable. That stability is the moat.

---

## Notes for the building agent

- **When in doubt, ask.** Posting "this is ambiguous in §X, here is what I assume, OK?" is faster than rebuilding.
- **Commit small.** One concept per commit. Git history is part of the deliverable.
- **Don't optimise prematurely.** Make it correct on Cloud Run, then make it fast. Day 16 is for performance.
- **Use the type system.** If the compiler can prove an invariant, do not write a runtime check.
- **Every public function gets a doc comment** with an example.
- **`unsafe` is forbidden.** If you need it, raise a design question first.
- **Cite when you encode literature.** Comments like `// Gottman & Levenson 2000, the "magic ratio"` are mandatory in `aco-score/`, `aco-infer/src/rules/patterns/`, `aco-infer/src/rules/defeasible/`, `aco-infer/src/rules/temporal/`.
- **Treat Cloud SQL as a first-class concern.** Every migration is reviewed. Every query goes through `sqlx::query!` for compile-time checks.
- **Treat the dashboard as a first-class concern.** It is the demo. The codebase should reflect that.
- **Treat Terraform as a first-class concern.** Infrastructure changes go through the same review as code changes. Never `gcloud` an ad-hoc resource into existence in production.
- **No secret ever lands in source or in `.env` at rest.** Secret Manager is the only correct place.

Build the substrate. Drive through text. Push to main. Ship it.

— end of plan —
