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

**AGON is a Rust conflict-intelligence engine by [TACITUS](https://www.tacitus.me).**

It turns messy human conflict text into typed, evidence-backed primitives: actors, aliases, claims, denials, events, commitments, contradictions, escalation signals, power dynamics, friction maps, review questions, and auditable reports.

**It is not a chatbot. It is infrastructure for conflict vision.**

---

## What AGON does

Given raw text — a complaint, a dossier, a thread, a deposition — AGON produces:

| Output | What it gives you |
|---|---|
| **Canonical actors + aliases** | Deterministic identity resolution |
| **Evidence-backed claims** | Every assertion linked to a verifiable source span |
| **Contradictions** | Both model-suggested and deterministic conflict pairs |
| **Friction matrix** | Actor-by-actor relationship dynamics |
| **Commitments** | Who promised what, to whom, in what state |
| **Quality gates** | Evidence coverage, ambiguity, conflict signal strength |
| **Review questions** | Human prompts before any decision |
| **Auditable report** | Markdown export preserving source + findings + evidence |

AGON **preserves disagreement as data**. Generic LLM summarization erases the structure that matters most: who claims what, who denies what, what commitment is contested, where the timeline diverges, and what remains uncertain. AGON does the opposite.

---

## Live Demo

```text
URL:      https://agon-dev-tbryoen6qa-uc.a.run.app
User:     AGON
Password: AGON
Status:   https://agon-dev-tbryoen6qa-uc.a.run.app/readyz
```

Paste a case → run perception → inspect the friction map, contradictions, evidence, quality gates, and raw JSON. One page. Direct. Demoable.

Backend: `tacitus-agon-dev` GCP project, `agon-dev` Cloud Run, Vertex AI Gemini + Cloud SQL.

---

## Why this exists

Human conflict lives in language before it becomes a case file. The raw material is fragmented: complaints, replies, interview notes, Slack threads, deposition excerpts, mediation memos, timelines reconstructed after the fact.

Generic AI summarizes that material into smoother prose. AGON takes the opposite posture:

> **AGON preserves disagreement as data.**

Every serious conclusion traces to source text. Every claim carries a confidence marker. Every contradiction is explicit, not implicit. The goal is to make tacit interpersonal and institutional friction computable without making it opaque.

AGON does **not** provide:
- Legal advice
- Guilt findings
- Settlement prediction
- Autonomous mediation strategy
- Verdicts of any kind

It creates a structured conflict map for professional review.

---

## The TACITUS Trinity — how AGON fits

AGON is one of three repos that combine into the TACITUS conflict-intelligence stack. **It is fully usable on its own** — you do not need the other repos to run AGON.

| Repo | Role | When you'd use it |
|---|---|---|
| **AGON** (this repo) | Evidence engine | You have messy conflict text and need verified, structured primitives |
| [**KAIROS**](https://github.com/sargonxg/KAIROS-temporal-vision-TACITUS) | Temporal engine | You need a temporal knowledge graph with Allen-13 relations + commitment state |
| [**DIALECTICA**](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS) | Reasoning core + conductor | You want full conflict intelligence: ontology, agents, GraphRAG, multi-tenant API |

```
text → KAIROS (when) → DIALECTICA (structure + ontology) → AGON (evidence verification) → graph
```

In the wired stack, AGON runs as a **post-pass** on DIALECTICA's extraction: it verifies claims, marks contradictions, and produces friction matrices that become typed edges in DIALECTICA's Conflict Grammar graph. The combined output backs [praxis.tacitus.me](https://praxis.tacitus.me).

📖 **Integration contract:** see DIALECTICA's [`docs/integration/`](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS/tree/main/docs/integration) — particularly `CONTRACTS.md`, `ONTOLOGY_MAPPING.md`, and `INTEGRATION_GUIDE.md`.

> AGON's API contract (`POST /api/perceive`) is the integration surface. Keeping that contract stable is a first-class concern; see [`docs/INTEROP.md`](docs/INTEROP.md) (planned) for the AGON-side mirror of the shared `tacitus-contracts` types.

---

## Architecture

```text
browser / DIALECTICA / external caller
  ↓
Axum / Rust / Cloud Run
  ├── document pre-reading (segmentation, density)
  ├── Vertex AI Gemini schema extraction
  ├── Rust evidence verification (spans matched against source)
  ├── deterministic contradiction checks
  ├── local sparse conflict-signal layer
  ├── deterministic inference findings
  ├── quality gates and review questions
  ├── Cloud SQL typed persistence
  └── embedded one-page workbench
```

The product is deliberately split:

- **UI**: small, direct, demoable
- **Rust library stack**: durable capability lives here
- **API**: inspectable extraction + inference surfaces
- **Storage**: typed primitives + raw session compatibility

---

## API

| Endpoint | Purpose |
|---|---|
| `GET  /healthz` | Liveness |
| `GET  /readyz` | Readiness |
| `GET  /api/info` | Service info |
| `GET  /api/schema` | Output schema |
| `POST /api/perceive` | **Primary: extract evidence-backed primitives from text** |
| `POST /api/perceive/stream` | SSE streaming variant |
| `GET  /api/sessions` | List sessions |
| `GET  /api/sessions/{id}` | Session detail |
| `GET  /api/sessions/{id}/report.md` | Auditable Markdown export |

### Quick call (PowerShell)

```powershell
$base = "https://agon-dev-tbryoen6qa-uc.a.run.app"
$headers = @{
  Authorization = "Basic QUdPTjpBR09O"
  "content-type" = "application/json"
}
$body = @{
  model = "flash-lite"
  text = "Sam says Alex agreed to own the board packet. Alex says he never agreed and only promised comments. Sam produced a Slack message saying Alex accepted ownership. Alex says that message referred to another deck."
} | ConvertTo-Json

$result = Invoke-WebRequest "$base/api/perceive" -Headers $headers -Method POST -Body $body -UseBasicParsing
($result.Content | ConvertFrom-Json).session_id
```

### Response surface

```text
persisted:        true when Cloud SQL is connected
document_profile: segments, markers, density, reading notes
actors:           canonical actor records
claims:           evidence-backed assertions
contradictions:   model-suggested + deterministic conflict pairs
neural_signals:   local sparse claim relatedness candidates
inferences:       denied obligations, contested commitments, escalation loops
quality_gates:    evidence coverage, ambiguity, conflict signal strength
review_questions: human review prompts before decision use
report.md:        auditable Markdown export
```

---

## Crate map

```text
crates/
  aco-core/      typed conflict primitives, EvidenceSpan, Provenance, IDs
  aco-llm/       Vertex Gemini and mock extraction backends
  aco-embed/     local sparse claim relatedness; optional fastembed-ready
  aco-fuse/      deterministic actor normalization and alias fusion
  aco-infer/     deterministic findings, quality gates, review questions
  aco-storage/   Postgres persistence, migrations, evidence span recovery
  aco-server/    Axum API, SSE, Basic Auth, embedded workbench
  aco-perceive/  perception pipeline scaffolding
  aco-score/     scoring scaffolding
  aco-learn/     correction and learning scaffolding
  aco-cli/       CLI entrypoint
  aco-bench/     benchmarks
```

---

## Storage model

```text
sessions             documents          chunks
document_segments    actors             actor_aliases
claims               events             commitments
patterns             contradictions     evidence_spans
graph_edges          neural_signals     inference_findings
quality_gates
```

Old `sessions` shape preserved for dashboard compatibility. Newer typed tables enable evidence-backed querying and reports.

---

## Trust posture

AGON is built around auditability:

- Source text persisted with deterministic hashes
- Claims, events, commitments, contradictions linked to evidence
- Evidence quotes checked against source text
- Exact span matches preferred
- Normalized span recovery is best-effort
- Unresolved evidence marked unresolved, not hidden
- Quality gates identify weak coverage and ambiguity
- Reports keep source + findings + contradictions + evidence + review questions together

---

## Local development

```powershell
git clone https://github.com/sargonxg/AGON.git
cd AGON

# Verify
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace

# Run with deterministic mock backend
$env:PORT="18080"
$env:AGON_BACKEND="mock"
$env:AGON_DEMO_USER="AGON"
$env:AGON_DEMO_PASSWORD="AGON"
cargo run -p aco-server --bin agon-server
```

Open `http://127.0.0.1:18080` (user/pass: `AGON` / `AGON`).

### With Vertex AI

```powershell
gcloud auth application-default login
$env:AGON_BACKEND="vertex"
$env:AGON_GCP_PROJECT_ID="tacitus-agon-dev"
$env:AGON_GCP_REGION="us-central1"
cargo run -p aco-server --bin agon-server
```

---

## Deployment

```powershell
$sha = (git rev-parse --short=12 HEAD)
$image = "us-central1-docker.pkg.dev/tacitus-agon-dev/agon/agon-server:$sha"
gcloud builds submit --project=tacitus-agon-dev --tag=$image .
gcloud run deploy agon-dev `
  --project=tacitus-agon-dev `
  --region=us-central1 `
  --image=$image `
  --platform=managed `
  --quiet
```

Verify:
```powershell
Invoke-WebRequest "https://agon-dev-tbryoen6qa-uc.a.run.app/readyz" -UseBasicParsing
```

---

## Verification gates

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo audit
cargo deny check
node --check crates/aco-server/assets/app.js
```

---

## Repository guide

| Document | Purpose |
|---|---|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | System architecture (42KB deep dive) |
| [`BUILDPLAN.md`](BUILDPLAN.md) | Implementation plan |
| [`SETUP.md`](SETUP.md) | Setup notes |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Contribution workflow |
| [`RUST_IMPACT.md`](RUST_IMPACT.md) | Why Rust for this domain |
| [`RESEARCH_QUESTIONS.md`](RESEARCH_QUESTIONS.md) | Open research lines |
| [`docs/AGON_MVP_PLUS_PLAN.md`](docs/AGON_MVP_PLUS_PLAN.md) | MVP+++ plan |
| [`docs/AGON_CONFLICT_INTELLIGENCE_IMPLEMENTATION_BRIEF.md`](docs/AGON_CONFLICT_INTELLIGENCE_IMPLEMENTATION_BRIEF.md) | Implementation brief |
| [`docs/research/AGON_CONFLICT_INTELLIGENCE_BLUEPRINT.md`](docs/research/AGON_CONFLICT_INTELLIGENCE_BLUEPRINT.md) | Research blueprint |
| [`ROADMAP.md`](ROADMAP.md) | Forward plan + trinity integration milestones |

---

## Roadmap

**Near term (standalone AGON):**
- Deeper deterministic contradiction rules (dates, order, obligations)
- Reviewed / unreviewed evidence workflow
- Richer typed persistence for relationship states and power dynamics
- Optional local BGE / fastembed / reranker sensor path
- NLI-style contradiction classifier evaluation
- Multi-document case folders
- Golden conflict examples with regression metrics
- Stronger JSON + Markdown report exports

**Trinity integration (with DIALECTICA + KAIROS):**
- Adopt `tacitus-contracts` shared schemas (ActorID, SourceSpan, Claim, Contradiction)
- Publish `aco-core` ↔ contracts mapping (`docs/INTEROP.md`)
- Stabilize `POST /api/perceive` request/response for DIALECTICA's `evidence_verify` pipeline node
- Optional gRPC server for lower-latency integration
- Cross-service trace propagation (`X-Trace-Id`)

**Later:**
- GraphRAG over verified primitives
- Temporal reasoning across many documents (likely via KAIROS)
- Human review queues
- Reviewer correction loops
- Local embeddings for alias and claim clustering
- Library API for external case systems

📖 Full plan: [`ROADMAP.md`](ROADMAP.md)

---

## Status

AGON is an active MVP. The live demo works with real pasted text and persists typed records when Cloud SQL is connected. Suitable for technical demos, product exploration, and early workflow design.

**Not yet production-ready for regulated enterprise deployment.** Missing: full RBAC, tenancy boundaries, secret management for third-party user keys, formal data retention controls, compliance review.

---

## LLM posture

Live deployment uses Vertex AI Gemini through GCP service-account auth — users don't paste their own key into the hosted demo. Default Cloud Run binary avoids ONNX coupling so service stays predictable. Heavier model-backed work (fastembed/BGE/reranker) is feature-flagged.

```text
AGON_NEURAL_MODE=local_sparse
AGON_NEURAL_MAX_PAIRS=250
AGON_NEURAL_MIN_SIM=0.62
```

---

## License

All rights reserved unless otherwise stated. See [LICENSE](LICENSE).

```text
TACITUS
by Giulio Catanzariti
https://www.tacitus.me
```
