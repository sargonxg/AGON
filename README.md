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

AGON is a Rust conflict-intelligence engine by [TACITUS](https://www.tacitus.me).

It turns messy human conflict text into typed, evidence-backed primitives: actors, aliases, claims, denials, events, commitments, contradictions, escalation signals, power dynamics, relationship friction, review questions, and auditable reports.

It is not a chatbot. It is infrastructure for conflict vision.

## Live Demo

```text
URL:      https://agon-dev-tbryoen6qa-uc.a.run.app
User:     AGON
Password: AGON
Status:   https://agon-dev-tbryoen6qa-uc.a.run.app/readyz
```

Current deployed target:

```text
Project:  tacitus-agon-dev
Region:   us-central1
Service:  agon-dev
Backend:  Vertex AI Gemini + Cloud SQL
```

The demo app is intentionally one page: paste a case, run perception, inspect the friction map, contradictions, evidence, quality gates, and raw JSON.

## Why This Exists

Human conflict lives in language before it becomes a case file, risk register, mediation packet, lawsuit, HR investigation, board issue, or institutional memory.

The source material is usually fragmented:

- complaint narratives
- replies and rebuttals
- interview notes
- Slack and email threads
- deposition excerpts
- mediation memos
- negotiation logs
- timelines reconstructed after the fact

Generic AI summarizes this material into smoother prose. That can be useful, but it can also erase the structure that matters most: who claims what, who denies what, what commitment is contested, which evidence supports it, where the timeline diverges, and what remains uncertain.

AGON takes the opposite posture.

It preserves disagreement as data.

The goal is to make tacit interpersonal and institutional friction computable without making it opaque. Every serious conclusion should be traceable to source text, marked with confidence, and reviewable by a human.

## TACITUS Context

TACITUS builds tools for institutions that need clearer judgment under pressure.

The broader vision is a stack of small, serious systems for perception, reasoning, and decision support across complex human domains: conflict, policy, diplomacy, governance, organizations, and high-stakes coordination.

AGON is the conflict-vision layer of that stack. It asks:

```text
Can a system read large volumes of messy human text
and construct a defensible map of friction, evidence,
relationships, contested facts, and resolution openings?
```

That map is not a verdict. It is a better substrate for investigation, mediation, legal review, and institutional learning.

## What AGON Does Today

Given raw text, AGON produces:

- canonical actors and aliases
- claims, events, commitments, patterns, and contradictions
- source-backed evidence spans with verified or unresolved status
- actor-by-actor friction matrix
- document pre-reading profile
- local sparse neural-style claim relatedness signals
- deterministic inference findings
- quality gates and review questions
- persistent sessions and typed storage in Postgres
- Markdown export reports
- streaming and non-streaming API routes

Live API routes:

```text
GET  /healthz
GET  /readyz
GET  /api/info
GET  /api/schema
POST /api/perceive
POST /api/perceive/stream
GET  /api/sessions
GET  /api/sessions/{id}
GET  /api/sessions/{id}/report.md
```

## Quick API Call

PowerShell:

```powershell
$base = "https://agon-dev-tbryoen6qa-uc.a.run.app"
$headers = @{
  Authorization = "Basic QUdPTjpBR09O"
  "content-type" = "application/json"
}
$body = @{
  model = "flash-lite"
  text = "Sam says Alex agreed to own the board packet. Alex says he never agreed to own it and only promised comments. Sam produced a Monday Slack message saying Alex accepted ownership. Alex says that message referred to another deck."
} | ConvertTo-Json

$result = Invoke-WebRequest "$base/api/perceive" -Headers $headers -Method POST -Body $body -UseBasicParsing
$json = $result.Content | ConvertFrom-Json
$json.session_id

Invoke-WebRequest "$base/api/sessions/$($json.session_id)/report.md" `
  -Headers @{ Authorization = "Basic QUdPTjpBR09O" } `
  -UseBasicParsing
```

Expected response surface:

```text
persisted:        true when Cloud SQL is connected
document_profile: segments, markers, density, reading notes
actors:           canonical actor records
claims:           evidence-backed assertions
contradictions:   model-suggested and deterministic conflict pairs
neural_signals:   local sparse claim relatedness candidates
inferences:       denied obligations, contested commitments, escalation loops
quality_gates:    evidence coverage, ambiguity, conflict signal strength
review_questions: human review prompts before decision use
report.md:        auditable Markdown export
```

## Architecture

```text
browser
  -> Axum / Rust / Cloud Run
      -> document pre-reading
      -> Vertex AI Gemini schema extraction
      -> Rust evidence verification
      -> deterministic contradiction checks
      -> local sparse conflict-signal layer
      -> deterministic inference findings
      -> quality gates and review questions
      -> Cloud SQL typed persistence
      -> embedded one-page workbench
```

The product is deliberately split:

- The UI is small, direct, and demoable.
- The Rust library stack is where the durable capability lives.
- The API keeps the extraction and inference surfaces inspectable.
- The storage layer keeps raw sessions compatible while adding typed primitives.

## Crate Map

```text
crates/
  aco-core/      typed conflict primitives, EvidenceSpan, Provenance, IDs
  aco-llm/       Vertex Gemini and mock extraction backends
  aco-embed/     local sparse claim relatedness; optional fastembed-ready feature
  aco-fuse/      deterministic actor normalization and alias fusion
  aco-infer/     deterministic findings, quality gates, review questions
  aco-storage/   Postgres persistence, migrations, evidence span recovery
  aco-server/    Axum API, SSE, Basic Auth, embedded one-page workbench
  aco-perceive/  perception pipeline scaffolding
  aco-score/     scoring scaffolding
  aco-learn/     correction and learning scaffolding
  aco-cli/       CLI entrypoint
  aco-bench/     benchmarks
```

## Storage Model

Typed MVP tables:

```text
sessions
documents
chunks
document_segments
actors
actor_aliases
claims
events
commitments
patterns
contradictions
evidence_spans
graph_edges
neural_signals
inference_findings
quality_gates
```

The old `sessions` shape remains available for dashboard compatibility. Newer typed tables make evidence-backed querying and reports possible.

## Trust Posture

AGON is built around auditability:

- Original source text is persisted with deterministic hashes.
- Claims, events, commitments, and contradictions are linked to evidence.
- Evidence quotes are checked against the source text.
- Exact span matches are preferred.
- Normalized span recovery is best-effort.
- Unresolved evidence is marked unresolved, not hidden.
- Quality gates identify weak extraction coverage and ambiguity.
- Reports keep the source, findings, contradiction pairs, evidence, and review questions together.

AGON does not provide legal advice, guilt findings, settlement prediction, or autonomous mediation strategy. It creates a structured conflict map for professional review.

## LLM and Neural Posture

The live deployment uses Vertex AI Gemini through Google Cloud service-account auth. Users do not need to paste their own Gemini key into the hosted demo.

Current local signal layer:

```text
AGON_NEURAL_MODE=local_sparse
AGON_NEURAL_MAX_PAIRS=250
AGON_NEURAL_MIN_SIM=0.62
```

`aco-embed` keeps heavier model-backed work optional. The default Cloud Run binary avoids ONNX runtime coupling so the service stays deployable, fast, and predictable. Future work can enable fastembed/BGE/reranker execution behind explicit features and runtime images built for that path.

## Local Development

Prerequisites:

- Rust toolchain from `rust-toolchain.toml`
- PowerShell or equivalent shell
- optional: `gcloud` for Vertex/Cloud Run work
- optional: Postgres/Cloud SQL for persistence

Clone and check:

```powershell
git clone https://github.com/sargonxg/AGON.git
cd AGON
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

Run locally with deterministic mock extraction:

```powershell
$env:PORT="18080"
$env:AGON_BACKEND="mock"
cargo run -p aco-server --bin agon-server
```

Open:

```text
http://127.0.0.1:18080
User: AGON
Password: AGON
```

Run locally against Vertex AI with ADC:

```powershell
gcloud auth application-default login
$env:PORT="18080"
$env:AGON_BACKEND="vertex"
$env:AGON_GCP_PROJECT_ID="tacitus-agon-dev"
$env:AGON_GCP_REGION="us-central1"
cargo run -p aco-server --bin agon-server
```

## Deployment

Current Cloud Run deployment target:

```text
Project: tacitus-agon-dev
Region:  us-central1
Service: agon-dev
Image:   us-central1-docker.pkg.dev/tacitus-agon-dev/agon/agon-server:<sha>
```

Build and deploy:

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
$base = "https://agon-dev-tbryoen6qa-uc.a.run.app"
Invoke-WebRequest "$base/readyz" -UseBasicParsing
```

## Verification Gates

Use these before shipping:

```powershell
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo audit
cargo deny check
node --check crates/aco-server/assets/app.js
```

Expected audit posture today: `cargo audit` exits successfully with existing allowed warnings for unmaintained transitive crates.

## Repository Guide

Useful entry points:

- [ARCHITECTURE.md](ARCHITECTURE.md): system architecture
- [SETUP.md](SETUP.md): setup notes
- [CONTRIBUTING.md](CONTRIBUTING.md): contribution workflow
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md): conduct expectations
- [docs/AGON_MVP_PLUS_PLAN.md](docs/AGON_MVP_PLUS_PLAN.md): MVP+++ implementation plan
- [docs/AGON_CONFLICT_INTELLIGENCE_IMPLEMENTATION_BRIEF.md](docs/AGON_CONFLICT_INTELLIGENCE_IMPLEMENTATION_BRIEF.md): implementation brief
- [docs/research/AGON_CONFLICT_INTELLIGENCE_BLUEPRINT.md](docs/research/AGON_CONFLICT_INTELLIGENCE_BLUEPRINT.md): research blueprint

## Roadmap

Near term:

- deeper deterministic contradiction rules for dates, order, and obligations
- reviewed/unreviewed evidence workflow
- richer typed persistence for relationship states and power dynamics
- optional local BGE/fastembed/reranker sensor path
- NLI-style contradiction classifier evaluation
- multi-document case folders
- golden conflict examples and regression metrics
- stronger JSON and Markdown report exports

Later:

- GraphRAG over verified primitives
- temporal reasoning across many documents
- human review queues
- reviewer correction loops
- local embeddings for alias and claim clustering
- integration-ready library API for external case systems

## Status

AGON is an active MVP. The live demo works with real pasted text and persists typed records when Cloud SQL is connected. The system is suitable for technical demos, product exploration, and early workflow design.

It is not yet production-ready for regulated enterprise deployment. Missing pieces include full RBAC, tenancy boundaries, secret management for third-party user keys, formal data retention controls, and compliance review.

## License

All rights reserved unless otherwise stated. See [LICENSE](LICENSE).

```text
TACITUS
by Giulio Catanzariti
https://www.tacitus.me
```
