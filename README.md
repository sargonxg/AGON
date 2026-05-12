# AGON

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

AGON is a Rust conflict-intelligence engine by TACITUS.

It is not a chatbot. It is a primitive engine for messy human conflict text: HR complaints, workplace investigations, mediation notes, negotiation logs, depositions, email threads, board minutes, and contradictory narratives.

Paste text. AGON extracts the conflict structure:

- actors and aliases
- claims and denials
- events and timelines
- commitments and contested obligations
- relationship pressure
- power dynamics
- escalation signals
- resolution openings
- contradictions
- evidence spans
- actor x actor friction
- document pre-reading profile
- local neural/sparse claim relatedness signals
- deterministic inference findings
- quality gates and review questions
- Markdown reports

Every serious conclusion is tied back to source text. If a quote cannot be verified against the source, AGON marks it unresolved instead of pretending.

## Why AGON Exists

Conflict work is usually trapped in prose: complaints, replies, interviews, chats, emails, meeting notes, deposition fragments, and partial memories. Standard AI systems smooth that material into summaries. That is useful for reading, but dangerous for judgment.

AGON does the opposite. It keeps disagreement structured.

It turns narrative conflict into typed, inspectable primitives that can be searched, scored, challenged, exported, and audited. The intended user is not asking "write me a nice summary." The intended user is asking:

- who is in conflict with whom?
- what exactly is being claimed?
- what is denied?
- what commitment is contested?
- where do timelines diverge?
- which evidence quotes are verified?
- what should a mediator, investigator, or legal team inspect first?

## Live Dev App

```text
URL:      https://agon-dev-tbryoen6qa-uc.a.run.app
User:     AGON
Password: AGON
Status:   /readyz
```

The public health endpoints stay open. The workbench and API are protected with Basic Auth for demos.

Last verified deployment:

```text
Revision: agon-dev-00009-xvd
Image:    us-central1-docker.pkg.dev/tacitus-agon-dev/agon/agon-server:b8d2808ef26e
State:    Vertex backend online, Cloud SQL connected, report export working
```

## Try The API

PowerShell:

```powershell
$base = "https://agon-dev-tbryoen6qa-uc.a.run.app"
$headers = @{
  Authorization = "Basic QUdPTjpBR09O"
  "content-type" = "application/json"
}
$body = @{
  model = "flash-lite"
  text = "Sam says Alex agreed to own the deck. Alex says he never agreed to own the deck. Sam produced a message from Monday saying Alex accepted ownership. Alex says the Monday message was about a different deck."
} | ConvertTo-Json

$result = Invoke-WebRequest "$base/api/perceive" -Headers $headers -Method POST -Body $body -UseBasicParsing
$json = $result.Content | ConvertFrom-Json
$json.session_id

Invoke-WebRequest "$base/api/sessions/$($json.session_id)/report.md" -Headers @{ Authorization = "Basic QUdPTjpBR09O" } -UseBasicParsing
```

Expected shape:

```text
persisted:      true
actors:         Sam, Alex
claims:         4
contradictions: deterministic and model-suggested conflict pairs
evidence:       verified quote ledger
inferences:     deterministic findings and quality gates
report:         Markdown conflict intelligence report
```

## What Makes It Different

Most AI tools summarize conflict away. AGON preserves conflict.

It does not average two incompatible accounts into a smooth paragraph. It separates who said what, what they deny, what they promised, where the stories diverge, and what evidence supports each primitive.

The core product bet:

```text
unstructured text
  -> typed conflict primitives
  -> evidence spans
  -> contradiction graph
  -> friction matrix
  -> auditable report
```

The first wedge is enterprise HR/workplace investigations and internal mediation: high-volume, high-risk human conflict where speed matters, but auditability matters more.

## Library First, App Second

The browser workbench is deliberately small. It exists to showcase the engine.

The durable value is the Rust library stack:

- strong primitive types in `aco-core`
- local neural/sparse conflict signals in `aco-embed`
- deterministic conflict inference in `aco-infer`
- deterministic alias fusion in `aco-fuse`
- model-constrained extraction in `aco-llm`
- evidence-backed persistence in `aco-storage`
- thin API/workbench shell in `aco-server`

The app should stay simple enough that a user understands the result in one page. The library should become powerful enough to process large volumes of messy human text into a defensible conflict graph.

## Current Architecture

```text
browser
  -> Cloud Run / Axum / Rust
      -> deterministic pre-reading / document profile
      -> Vertex AI Gemini schema extraction
      -> local sparse neural-signal layer
      -> Rust deterministic enrichment
      -> inference findings + quality gates
      -> Cloud SQL Postgres typed persistence
      -> embedded one-page workbench
```

Current live capabilities:

- `/api/perceive`
- `/api/perceive/stream`
- `/api/sessions`
- `/api/sessions/{id}`
- `/api/sessions/{id}/report.md`
- `/api/schema`
- `/readyz`
- `/healthz`

The `/api/perceive` response now includes these conflict-vision layers in addition to the original extraction:

- `document_profile`: format, segments, temporal markers, modality markers, pre-reading notes, conflict density, candidate review questions.
- `neural_signals`: local Rust claim relatedness signals. The current deployed path uses a deterministic sparse fallback and exposes fastembed model capability discovery; full model-download mode is gated by environment.
- `inferences`: deterministic findings such as denied obligations, contested/broken commitments, escalation loops, and repair openings.
- `quality_gates`: evidence coverage, actor ambiguity, and conflict signal strength.
- `review_questions`: questions a mediator/investigator should resolve before treating the map as decision-ready.

## One-Page Workbench

The UI is intentionally simple:

- **Overview**: counts, summary, conflict graph
- **Conflict vision lens**: pre-reading profile, quality gates, neural signals, review questions
- **Actors**: canonical actors, aliases, relationships, power dynamics
- **Friction**: actor x actor heat matrix with explainable drivers
- **Contradictions**: side-by-side contradiction cards
- **Evidence**: claims, events, commitments, patterns, evidence ledger
- **Raw**: full JSON for audit/debugging

No sprawling app shell. No generic chat surface. The page is a lens over the engine.

## Rust Crates

```text
crates/
  aco-core/      typed primitives, EvidenceSpan, Provenance, IDs
  aco-embed/     local sparse/neural claim relatedness signals; fastembed-ready
  aco-fuse/      deterministic actor normalization and alias fusion
  aco-infer/     deterministic inference findings and quality gates
  aco-llm/       Vertex/Gemini and mock extraction backends
  aco-storage/   Cloud SQL/Postgres persistence and evidence span recovery
  aco-server/    Axum API, SSE pipeline, auth, embedded dashboard
  aco-infer/     inference scaffolding
  aco-score/     scoring scaffolding
  aco-learn/     correction/learning scaffolding
  aco-cli/       CLI shell
  aco-bench/     benchmarks
```

The current production path is `aco-server` + `aco-llm` + `aco-storage` + `aco-fuse` + `aco-core`.
The v0.2 conflict-vision path also uses `aco-embed` and `aco-infer`.

## Trust Core

AGON uses a strict evidence posture:

- Source text is persisted as a document/chunk with a deterministic hash.
- Extracted primitives carry evidence quotes.
- Storage resolves quote offsets with exact matching first.
- If punctuation/case drift occurs, Rust attempts normalized span recovery.
- If evidence cannot be aligned, it is marked unresolved.
- The UI shows verified vs unresolved evidence.
- Markdown reports preserve the session, summary, contradictions, evidence ledger, and source text.
- Quality gates make uncertainty explicit instead of hiding it in prose.
- Review questions capture what a human should verify next.

## Neural Signal Posture

AGON is designed to become local-neural where that improves conflict understanding, but the live deployment stays operationally conservative.

Current implementation:

- `aco-embed` computes local sparse semantic relatedness over claim pairs.
- It discovers supported `fastembed` embedding/reranker families without forcing model downloads during normal Cloud Run startup.
- Relatedness is never treated as truth by itself. It raises candidate pairs for deterministic contradiction and review logic.

Environment controls:

```text
AGON_NEURAL_MODE=local_sparse        # default
AGON_NEURAL_MAX_PAIRS=250
AGON_NEURAL_MIN_SIM=0.62
AGON_NEURAL_CACHE_DIR=<future model cache path>
```

This keeps the architecture ready for local BGE/fastembed reranking while preserving fast cold starts and predictable costs today.

## Current Storage Model

Typed MVP tables:

- `sessions`
- `documents`
- `chunks`
- `actors`
- `actor_aliases`
- `claims`
- `events`
- `commitments`
- `patterns`
- `contradictions`
- `evidence_spans`
- `graph_edges`
- `document_segments`
- `neural_signals`
- `inference_findings`
- `quality_gates`

The old session-history shape remains compatible with the dashboard.

## Research and Product Direction

The current blueprint lives in:

- `docs/research/AGON_CONFLICT_INTELLIGENCE_BLUEPRINT.md`
- `docs/AGON_CONFLICT_INTELLIGENCE_IMPLEMENTATION_BRIEF.md`
- `docs/AGON_MVP_PLUS_PLAN.md`

The guiding thesis is simple:

> AGON is not AI summarization for disputes. AGON is an evidence-backed conflict primitive engine.

## Local Development

```powershell
git clone https://github.com/sargonxg/AGON.git
cd AGON
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

Run locally with the mock backend:

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

## Deploy

Current dev deployment target:

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
gcloud run deploy agon-dev --project=tacitus-agon-dev --region=us-central1 --image=$image --platform=managed --quiet
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

## Roadmap

Near term:

- stronger local fastembed/BGE reranker execution path behind `AGON_NEURAL_MODE=local`
- NLI-style contradiction classifier evaluation as an optional Rust/ONNX sensor
- deterministic denial/commitment/date contradiction expansion
- richer typed persistence for relationships, power dynamics, escalation, and resolution openings
- reviewed/unreviewed evidence workflow
- stronger report exports
- golden conflict examples and regression metrics

Later:

- multi-document contradiction graphs
- GraphRAG over verified primitives
- temporal reasoning
- human review queues
- local embeddings for suggested alias clusters

Not yet:

- legal advice
- guilt findings
- settlement prediction
- autonomous mediation strategy
- broad enterprise RBAC/multitenancy
- black-box scoring that cannot explain itself

## License

All rights reserved unless otherwise stated.

```text
TACITUS
by Giulio Catanzariti
https://www.tacitus.me
```
