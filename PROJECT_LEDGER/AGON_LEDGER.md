# AGON Build Ledger

**MVP target:** v0.1.0 — three-sprint GCP-native build per `BUILDPLAN.md`.
**Started:** 2026-05-10
**Spec version:** v3 (GCP-native; supersedes local-dev v2 plan)
**GCP project (dev):** tacitus-agon-dev (to be created)
**GitHub:** github.com/sargonxg/AGON (public, exists, no commits yet on remote)
**Maintainer:** Giulio Catanzariti <giuliocatanzariti@gmail.com>

Legend: ☐ todo · ◐ in-progress · ✓ done · ✗ blocked · ⏸ deferred

---

## Spec reset note (2026-05-10)

The original v2 plan targeted local docker-compose Postgres + direct Gemini API + Day-20 cloud deploy. The new v3 plan (current `ARCHITECTURE.md` + `BUILDPLAN.md`) is **GCP-native from Day 1**: Cloud SQL replaces local Postgres, Vertex AI replaces direct Gemini API, Terraform IaC and Cloud Build CI/CD are foundation work, and no production runtime executes locally.

Day 0 work done under the old plan (Cargo workspace, crate skeletons, basic CI) is preserved. The v2 `compose.yaml` is now superseded — kept on disk for reference until Day 4 is finished, then removed.

---

## Pre-Sprint — GCP bootstrap (Day 0)

| # | Task | Status | Notes |
|---|---|---|---|
| 0.1 | Create dev GCP project | ✓ | `tacitus-agon-dev` (#1086904791123) under org 709406008078 |
| 0.2 | Attach billing | ✓ | startup credit `011452-1C91EA-384484` |
| 0.3 | Enable 17 APIs | ✓ | via `make bootstrap` |
| 0.4 | Quota uplift for Vertex AI Gemini | ☐ | [You] console request when ready |
| 0.5 | `gcloud auth login` + ADC | ✓ | giulio@tacitus.me + quota-project=tacitus-agon-dev |
| 0.6 | Repo init | ✓ | pushed to github.com/sargonxg/AGON |
| 0.7 | Makefile + bootstrap.sh + connect-github.sh | ✓ | scaffolded 2026-05-10 |
| 0.8 | `.env.example` for GCP-native vars | ✓ | replaced v2 file |
| 0.9 | First push to `sargonxg/AGON` | ✓ | `main` branch |
| 0.10 | TF state bucket | ✓ | `tacitus-agon-dev-terraform-state` |
| 0.11 | Cloud Build SA IAM | ✓ | 6 roles granted |
| 0.12 | Cloud Build ↔ GitHub connection | ☐ | [You] Console: cloud-build/repositories/2nd-gen |
| 0.13 | Create prod project | ⏸ | deferred to Day 19 |

### Legacy Day 0 (v2) — keep
- ✓ Cargo workspace init
- ✓ 12 crate skeletons (`crates/aco-*/`)
- ✓ rustfmt.toml, clippy.toml, deny.toml
- ✓ CI workflows scaffold under `.github/workflows/`
- ⏸ `compose.yaml` (will be removed after Day 4)

---

## Sprint 1 — Foundations on GCP (Days 1–7)

**Goal:** end-to-end smoke run on deployed dev environment. CI/CD pipeline: `git push` → Cloud Build → Cloud Run revision.

### Day 1 — `aco-core`: types and provenance ✓
- ✓ 1.1 Id + canonical hashing (Blake3)
- ✓ 1.2 Common types (EvidenceSpan, Provenance, Defeasibility, Derivation, TemporalInterval, Place)
- ✓ 1.3 Eight primitives (Actor/Claim/Interest/Constraint/Leverage/Commitment/Event/Narrative)
- ✓ 1.4 Interpersonal extensions (PatternFinding, AffectMarker, Emotion, RepairKind, BidResponse)
- ✓ 1.5 Enums + FOL + errors
- ✓ 1.6 Property tests (1000 iters/primitive serde roundtrip)
- ✓ 1.7 `cargo test -p aco-core` green — **8 passed**
- ✓ 1.8 `cargo doc -p aco-core` clean

### Day 2 — Terraform IaC (provisions dev env) ✓
- ✓ 2.1 main/variables/outputs.tf
- ✓ 2.2 network module (VPC + subnet + private service connection)
- ✓ 2.3 cloud_sql module (Postgres 16, db-f1-micro, private IP, Enterprise edition)
- ✓ 2.4 storage module (docs + exports buckets, lifecycle rules)
- ✓ 2.5 artifact_registry module
- ✓ 2.6 secrets module (db password + jwt key)
- ✓ 2.7 iam module (cloud_run SA + eventarc SA, least privilege)
- ✓ 2.8 cloud_run module (v2 service, placeholder hello image, allUsers invoker, VPC egress)
- ✓ 2.9 eventarc module (GCS finalized → Cloud Run /api/eventarc/upload)
- ✓ 2.10 envs/dev/{backend,main,terraform.tfvars}.tf
- ✓ 2.11 Org policy override: iam.allowedPolicyMemberDomains=allowAll
- ✓ 2.12 `terraform apply` clean — **32 resources, deployed**
- ✓ **Live URL: https://agon-dev-tbryoen6qa-uc.a.run.app (HTTP 200)**

### Day 3 — `aco-llm`: Vertex AI Gemini + Mock ✓
- ✓ 3.1 `LlmBackend` async trait (extract_json + embed)
- ✓ 3.2 `VertexAiBackend` live — talks to Vertex AI from Cloud Run
- ✓ 3.3 Service-account auth via metadata server + ADC fallback for local
- ✓ 3.4 `MockLlmBackend` capitalized-token heuristic
- ✓ 3.5 CostLedger per-model token tracker
- ⏸ 3.6 `aco-embed` fastembed — deferred (Vertex text-embedding-005 used)
- ✓ 3.7 Mock test green

### Day 6 partial — `aco-server` Axum + dashboard ✓
- ✓ Axum 0.8 server with embedded assets via rust-embed
- ✓ Dark-mode dashboard at `/` — textarea + 6-card metrics + 7-panel grid + raw JSON
- ✓ Endpoints: `/`, `/assets/*`, `/healthz`, `/readyz`, `/api/info`, `/api/perceive`
- ✓ Perception prompt + JSON schema for 8 ACO primitives + patterns + contradictions
- ✓ End-to-end live: actors, claims, contested commitments, friction score

### Day 7 partial — first deploy ✓
- ✓ Multi-stage Dockerfile (Rust 1.83 builder → distroless/cc nonroot)
- ✓ `.dockerignore`
- ✓ Cloud Build remote build (5m54s, 250 MB image)
- ✓ Image pushed: `us-central1-docker.pkg.dev/tacitus-agon-dev/agon/agon-server:v0.1.0`
- ✓ Cloud Run revision `agon-dev-00002-56q` serving 100% traffic
- ✓ Live perceive call against real Vertex AI Gemini 2.5 Flash from Cloud Run SA
- ☐ Cloud Build GitHub trigger (manual submit works for now)

### Day 4 — `aco-storage`: Cloud SQL via sqlx
- ☐ 4.1 `migrations/001_init.up.sql` + `.down.sql`
- ☐ 4.2 Pool + Secret Manager password
- ☐ 4.3 Repos per primitive
- ☐ 4.4 Provenance + spans + edges + audit
- ☐ 4.5 `petgraph` mirror + hydration
- ☐ 4.6 `LISTEN`/`NOTIFY`
- ☐ 4.7 Tests

### Day 5 — `aco-perceive` + `aco-fuse`
### Day 6 — `aco-infer` + `aco-score` (first pass) + `aco-server`
### Day 7 — Cloud Build CI/CD + first deploy + workplace-dispute scenario

**Sprint 1 exit:**
- ☐ `cargo test --all` green
- ☐ `git push origin main` → green Cloud Build → Cloud Run revision ≤ 12 min
- ☐ `agon-cli --api $(make url-raw) ingest corpora/workplace_dispute/` works
- ☐ Dashboard at `make url` shows world model + 1 Friction Score

---

## Sprint 2 — Deep inference, scoring, dashboard (Days 8–14)

Defeasible reasoning · Z3 contradiction · patterns + remaining scores · BATNA/ZOPA + abduction · dashboard upgrades + brief generator · live-demo dry-run · Ask mode + pragmatics. Expanded in `BUILDPLAN.md` §Sprint 2.

---

## Sprint 3 — Learning, hardening, release (Days 15–21)

Learning loop · perf on Cloud Run · robustness + security · five golden corpora · prod environment + observability · docs · release v0.1.0 + final demo. Expanded in `BUILDPLAN.md` §Sprint 3.

---

## Blockers / notes

- **2026-05-10** — Plan reset to GCP-native v3. Day 0 (v3) scaffold added. Interactive Day 0 steps in `SETUP.md`.
- **2026-05-10** — Local tooling: `terraform` ✗, `make` ✗, `docker` not verified. `gcloud` ✓, `gh` ✓. See `SETUP.md` §0.
- **2026-05-10** — `compose.yaml`, local-Postgres tests, direct-Gemini API code superseded. Remove in Day 4 commit.
- **Security** — `.env` gitignored. No secrets in source. Vertex AI via Cloud Run SA. Cloud SQL password in Secret Manager.
