<div align="center">

# AGON

**A Tesla-style perception engine for human conflict — in Rust, on Google Cloud.**

[Live demo](https://agon-dev-tbryoen6qa-uc.a.run.app) · [Architecture](./ARCHITECTURE.md) · [Build plan](./BUILDPLAN.md) · [Setup](./SETUP.md) · Built by [TACITUS](https://www.tacitus.me)

[![Rust](https://img.shields.io/badge/rust-1.78%2B-orange?logo=rust)](https://www.rust-lang.org)
[![Cloud Run](https://img.shields.io/badge/runtime-Cloud%20Run-4285F4?logo=googlecloud&logoColor=white)](https://cloud.google.com/run)
[![Cloud SQL](https://img.shields.io/badge/storage-Cloud%20SQL%20pgvector-4285F4?logo=googlecloud&logoColor=white)](https://cloud.google.com/sql)
[![Vertex AI](https://img.shields.io/badge/llm-Vertex%20AI%20Gemini%202.5-4285F4?logo=googlecloud&logoColor=white)](https://cloud.google.com/vertex-ai)

</div>

---

## What it does

Paste a thread, a transcript, a deposition, a message log, a board-minutes excerpt. AGON returns a typed **world model** of the conflict: actors (deduplicated across the text), claims (with attribution + evidence span), events, commitments (with status: proposed/accepted/contested/fulfilled/broken), inferred interests, behavioural patterns (DARVO, gaslighting, stonewalling, Four Horsemen, repair attempts), cross-source contradictions (with materiality), and a calibrated friction score.

Every primitive cites its verbatim source span. Nothing is paraphrased. The reasoning is auditable by clicking.

Try it: paste 2–10 paragraphs into the [live dashboard](https://agon-dev-tbryoen6qa-uc.a.run.app). End-to-end on Vertex AI Gemini 2.5 Flash: ~5–10 seconds for a typical thread.

## How it works

```
browser  ──┬──→  Cloud Run (Axum + embedded dashboard, Rust)  ──→  Vertex AI Gemini 2.5
            └──→  /api/perceive  ──→  schema-constrained extraction  ──→  Cloud SQL (Postgres + pgvector)
```

Sovereign runtime: Rust binary in a distroless container on Cloud Run. The LLM is **the typed sensor**, called over Vertex AI with a JSON schema that constrains output to the ACO (Agentic Conflict Ontology) primitives. Everything downstream — fusion, inference, scoring — is Rust, in-process, deterministic, auditable.

## Architecture

The whole thing follows a Tesla-style pipeline applied to text:

| Tesla layer | AGON layer | Implementation |
|---|---|---|
| Cameras | Document loaders | Cloud Storage (signed URL uploads) |
| Object detection | Parallel extractors | Cloud Run + **Vertex AI Gemini** |
| Sensor fusion | Canonicalization (dedup, alias graph) | Rust in-process |
| HD map | World model | **Cloud SQL Postgres 16 + pgvector** |
| Path planning | Inference (Datalog, Z3, LP) + scoring | Rust in-process (`ascent`, `z3`, `good_lp`) |
| Control output | Dashboard + briefs | Axum + WebSocket |
| Fleet learning | Correction log + active learning | Cloud SQL + Cloud Scheduler |
| Telemetry | Observability | Cloud Logging + Trace + Monitoring |
| Pipeline | CI/CD | Cloud Build + Artifact Registry |

Full spec in [ARCHITECTURE.md](./ARCHITECTURE.md). Day-by-day build plan in [BUILDPLAN.md](./BUILDPLAN.md).

## The ACO ontology

Eight primitives (locked):

- **Actor** — individuals, organizations, states, coalitions
- **Claim** — asserted fact, evaluation, or normative statement attributed to an actor
- **Interest** — underlying goal motivating positions (Fisher/Ury)
- **Constraint** — rule, norm, or structural limit
- **Leverage** — resource or capability shifting bargaining power
- **Commitment** — promised future action with subject + deadline
- **Event** — dated or orderable occurrence
- **Narrative** — coherent framing across multiple claims, per actor

Plus interpersonal extensions: `PatternFinding` (DARVO, gaslighting, Four Horsemen, repair attempts), `AffectMarker`, `BidForConnection`, `RepairAttempt`.

## Repository layout

```
AGON/
├── README.md / ARCHITECTURE.md / BUILDPLAN.md / SETUP.md
├── Makefile                          # bootstrap, infra-apply, deploy, logs, url, rollback
├── Cargo.toml                        # 12-crate Rust workspace
├── Dockerfile / .dockerignore        # multi-stage rust → distroless
├── crates/
│   ├── aco-core/                     # 8 ACO primitives + provenance + Blake3 hashing
│   ├── aco-llm/                      # Vertex AI Gemini client + Mock backend
│   ├── aco-embed/                    # fastembed + HNSW (placeholder)
│   ├── aco-storage/                  # Cloud SQL via sqlx
│   ├── aco-perceive/                 # parallel extractors (next)
│   ├── aco-fuse/                     # canonicalization + dedup (next)
│   ├── aco-infer/                    # Datalog + Z3 + LP (next)
│   ├── aco-score/                    # friction/risk/power/trust scoring (next)
│   ├── aco-learn/                    # correction log + active learning (next)
│   ├── aco-server/                   # Axum HTTP + embedded dashboard ← deployed
│   ├── aco-cli/                      # `agon` thin client (next)
│   └── aco-bench/                    # criterion benches
├── infra/
│   ├── terraform/                    # 8 modules: network/sql/storage/AR/iam/secrets/run/eventarc
│   ├── bootstrap.sh / connect-github.sh
│   └── Dockerfile / cloudbuild.yaml (next)
├── migrations/                       # sqlx Postgres schema migrations
└── PROJECT_LEDGER/                   # day-by-day progress (AGON_LEDGER.md, STATE.json, SESSION_LOG.md)
```

## Build status

| Day | Component | Status |
|---|---|---|
| 0 | GCP bootstrap (project, billing, 17 APIs, TF state bucket, Cloud Build SA IAM) | ✅ |
| 1 | `aco-core` — 8 ACO primitives, Blake3 hashing, provenance, 1000-iter property tests | ✅ |
| 2 | Terraform IaC — 32 GCP resources live (VPC, Cloud SQL, GCS, AR, Cloud Run, Eventarc) | ✅ |
| 3 | `aco-llm` — Vertex AI Gemini 2.5 Flash client + Mock + cost ledger | ✅ |
| 4 | `aco-storage` — Cloud SQL persistence (sessions table + migrations) | ✅ |
| 5 | `aco-perceive` + `aco-fuse` — loaders, chunking, alias graph, HNSW dedup | 🚧 |
| 6 | `aco-server` — Axum + embedded dashboard + graph view + history | ✅ |
| 7 | First deploy — Dockerfile + Cloud Build + Cloud Run revision | ✅ |
| 8 | Defeasible reasoning (ASPIC+ on stratified Datalog) | 🚧 |
| 9 | Z3 contradiction detection (per-actor consistency, deontic conflict) | 🚧 |
| 10 | Pattern detection + remaining scores (repair capacity, bid-turn ratio, toxicity, risk) | 🚧 |
| 11 | BATNA / ZOPA LP + abduction loop | 🚧 |
| 12 | Dashboard upgrades: Cytoscape graph, d3 timeline, friction heatmap, briefs | 🚧 |
| 13–14 | Ask mode + pragmatics + live-demo dry-run | 🚧 |
| 15–21 | Sprint 3 — learning, hardening, prod environment, docs, v0.1.0 release | 🚧 |

## Quickstart

You need a GCP project with billing, `gcloud`, `terraform`, `make`, and Git Bash on Windows. Full step-by-step in [SETUP.md](./SETUP.md).

```bash
# 1. Clone + configure
git clone https://github.com/sargonxg/AGON.git && cd AGON
cp .env.example .env  # GCP_PROJECT_ID, GCP_REGION, ENV

# 2. Bootstrap GCP (enables 17 APIs, creates TF state bucket, grants Cloud Build SA IAM)
make bootstrap

# 3. Provision infrastructure (Cloud SQL, VPC, Cloud Run, Eventarc, IAM, Secrets, AR)
make infra-apply

# 4. Build + push container, deploy revision
gcloud builds submit --tag us-central1-docker.pkg.dev/$GCP_PROJECT_ID/agon/agon-server:v0.1.0
gcloud run deploy agon-dev \
    --image=us-central1-docker.pkg.dev/$GCP_PROJECT_ID/agon/agon-server:v0.1.0 \
    --region=$GCP_REGION

# 5. Open the dashboard
make url
```

## Cost (per month, dev)

Cloud Run scale-to-zero · Cloud SQL `db-f1-micro` · Vertex AI metered.

| Component | Cost |
|---|---|
| Cloud Run service | $0 idle, ~$0.10–0.50/demo-hour |
| Cloud SQL `db-f1-micro` | ~$10–15/mo |
| Cloud Storage 10 GB | ~$0.20/mo |
| Artifact Registry 2 GB | ~$0.20/mo |
| Vertex AI Gemini 2.5 Flash | ~$0.05–0.50/demo |
| **Dev total** | **~$15–25/mo + per-use Gemini** |

## Citations

Conflict and relationship science: Gottman & Levenson (2000), Karpman (1968), Freyd (1996), Bowen (1978). Argumentation: Modgil & Prakken (2014), Dung (1995). Temporal reasoning: Allen (1983). Pragmatics: Searle (1969), Grice (1975). Negotiation: Fisher & Ury (1981). Rust ecosystem: Sahebolamri et al. (Ascent, OOPSLA 2023). Full bibliography in [ARCHITECTURE.md](./ARCHITECTURE.md#20-citations).

## License

To be determined. Until a LICENSE file is committed, all rights reserved by TACITUS / Giulio Catanzariti.

---

<div align="center">

*Built by [TACITUS](https://www.tacitus.me) · Push to main · The cloud deploys*

</div>
