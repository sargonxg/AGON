# AGON Roadmap

What's next for AGON as a **standalone evidence engine** and as the **verification layer** of the TACITUS trinity ([DIALECTICA](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS) + AGON + [KAIROS](https://github.com/sargonxg/KAIROS-temporal-vision-TACITUS)).

> **Practical posture.** Standalone-first. Every milestone keeps AGON publishable and runnable on its own. Trinity integration is additive.

---

## North star

AGON is the layer that says, on every claim:

> "Here is the source. Here is the verification status. Here is what contradicts it."

When that's true at scale — across messy real-world dossiers, not curated demos — AGON is the moat of the TACITUS stack.

---

## Phase A — Hardening the standalone product (next 4-6 weeks)

### A1 — Deterministic contradiction rules (biggest signal/effort win)

Current state: contradictions are mostly model-suggested. Deterministic side is thin.

- [ ] **Date inconsistency rule.** When two claims about the same event/commitment carry incompatible dates (resolution-aware), emit `Contradiction { mechanism: DATE_INCONSISTENCY }`.
- [ ] **Obligation denial rule.** Claim "X committed to Y" + claim "X never committed to Y" by same speaker → `OBLIGATION_DENIAL`.
- [ ] **Order inconsistency.** "A happened before B" vs "B happened before A" → contradiction. (Requires temporal anchoring — better with KAIROS.)
- [ ] **Attribution dispute.** Same action attributed to different actors by different sources → `ATTRIBUTION_DISPUTE`.
- [ ] **Polarity flip.** Polarity-bearing predicates ("agreed" / "refused") with conflicting subjects → contradiction.

Acceptance: 10 golden fixtures, each producing the expected contradiction set deterministically.

### A2 — Reviewed / unreviewed evidence workflow

- [ ] Add `evidence.review_status`: `unreviewed | accepted | flagged | rejected`
- [ ] `POST /api/sessions/{id}/evidence/{eid}/review` endpoint
- [ ] UI: review queue in workbench
- [ ] Report.md exports `Reviewed by:` + `Reviewed at:` per evidence span

### A3 — Multi-document case folders

- [ ] `Case` aggregate above `Session`
- [ ] `POST /api/cases` + `POST /api/cases/{id}/documents`
- [ ] Cross-document actor resolution within a case
- [ ] Case-level friction matrix + contradictions
- [ ] Storage: `cases` table, `case_documents` join, FK from sessions

### A4 — Quality + observability

- [ ] Structured JSON logs with `trace_id`, `session_id`, `actor_count`, `verification_rate`
- [ ] Prometheus `/metrics` endpoint
- [ ] Latency budget assertions in CI (p95 < 8s for 5k-word doc)

### A5 — Stronger reports

- [ ] JSON Schema for AGON output (drives downstream type safety)
- [ ] PDF export option
- [ ] Side-by-side contradiction view in report (claim A | claim B | mechanism | evidence)

---

## Phase B — Trinity integration (in parallel with Phase A, weeks 3-8)

These tasks make AGON callable from DIALECTICA's pipeline. See DIALECTICA's [`docs/integration/`](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS/tree/main/docs/integration) for full context.

### B1 — Adopt `tacitus-contracts`

- [ ] Add `tacitus-contracts` Cargo dependency (path or published)
- [ ] Map `aco-core::Actor` → `tacitus_contracts::Actor`
- [ ] Map `aco-core::Claim` → `tacitus_contracts::Claim` (with `VerificationStatus` enum)
- [ ] Map `aco-core::Contradiction` → `tacitus_contracts::Contradiction`
- [ ] Map `aco-core::EvidenceSpan` → `tacitus_contracts::SourceSpan`
- [ ] Emit `AnalysisEnvelope` in `/api/perceive` response
- [ ] Honor `X-Trace-Id` request header (propagate through logs)

### B2 — `POST /api/perceive` contract stability

- [ ] Version the endpoint: `/api/v1/perceive`
- [ ] Document JSON Schema in [`docs/INTEROP.md`](docs/INTEROP.md) (planned)
- [ ] Backward-compat layer: legacy `/api/perceive` proxies to v1 for one release
- [ ] Accept `candidate_claims[]` in request (DIALECTICA passes Gemini-extracted claims for verification)
- [ ] Add idempotency key support: same `document_id` + `text hash` returns cached result

### B3 — Compatibility checks

- [ ] Service emits `contracts_version` in envelope
- [ ] Compatibility CI: AGON test matrix runs against published `tacitus-contracts` versions
- [ ] Add `buf breaking` check if proto-based contracts adopted

### B4 — IAM and internal-only mode

- [ ] Cloud Run deploy variant: `agon-internal` accepts traffic only from DIALECTICA service account
- [ ] Drop demo Basic Auth on internal variant
- [ ] Keep `agon-dev` public for demos

### B5 — Optional gRPC server

- [ ] `tonic`-based gRPC server alongside Axum REST
- [ ] Same contracts, lower-latency for DIALECTICA pipeline calls
- [ ] Behind cargo feature `grpc`

---

## Phase C — Capability deepening (weeks 9-20)

### C1 — NLI-style contradiction classifier

- [ ] Evaluate small fine-tunable NLI models (e.g., DeBERTa-v3, BGE-NLI)
- [ ] `aco-embed` feature flag for NLI-as-contradiction-signal
- [ ] A/B against current model-suggested + deterministic baseline
- [ ] Ship as confidence-graded signal, not authoritative

### C2 — Local embedding sensor path

- [ ] `aco-embed` feature `fastembed` for BGE-base local embeddings
- [ ] Cluster-then-verify for alias resolution at scale
- [ ] Re-rank evidence candidates by semantic relevance to claim
- [ ] Runtime image variant with ONNX runtime preloaded

### C3 — Power dynamics + relationship state

Current friction matrix is flat. Add:
- [ ] Bidirectional relationship state (A→B trust, A→B opposition) over time
- [ ] French/Raven power base detection (legitimate, expert, referent, coercive, reward)
- [ ] Output: typed `RelationshipState` records linked to `EvidenceSpan`

### C4 — Reviewer correction loop

- [ ] `corrections` table: human override of model extraction
- [ ] Training-data export for downstream fine-tuning
- [ ] Active-learning surface: "AGON is uncertain about these N claims — please review"

### C5 — GraphRAG over verified primitives

- [ ] Vector index over `(claim, evidence_text)` pairs
- [ ] Cross-document retrieval ("show me all denied commitments across this case folder")
- [ ] Optional: surface via library API or as additional endpoint

---

## Phase D — Enterprise readiness (when there's a customer)

Don't build these speculatively. Build when a real customer is paying for AGON.

- [ ] RBAC with row-level tenancy
- [ ] Secret management for third-party Gemini keys
- [ ] Formal data retention controls (GDPR/CCPA)
- [ ] Audit log API
- [ ] SOC 2 prep
- [ ] SLA tier with min-instances and dedicated egress
- [ ] EU data residency option (GCP europe-west)

---

## What's NOT on the roadmap (intentional)

- **Verdicts.** AGON does not decide. It surfaces structure for human review.
- **Autonomous mediation strategy.** Same.
- **Settlement prediction.** Same.
- **Custom LLM training.** Stick with frontier models via Vertex; train only narrow sensors (NLI, NER) where it's cheap and clearly better.
- **Frontend bloat.** The one-page workbench stays small. Rich UX lives in PRAXIS, not AGON.

---

## Success criteria per phase

| Phase | Done when |
|---|---|
| A | 10 golden fixtures produce expected deterministic contradictions; review workflow shipped; multi-doc cases working |
| B | DIALECTICA can call AGON behind feature flag; integration tests pass end-to-end with mock + real Gemini |
| C | NLI signal improves contradiction F1 by ≥10% on golden set; alias resolution handles 100+ aliases per case |
| D | One enterprise customer in production with audit log + RBAC + data residency |

---

## Open questions

1. **Should AGON own commitment extraction, or defer to KAIROS?**
   Recommendation: AGON keeps its own commitment extraction for standalone use; in trinity mode, prefer KAIROS commitments (temporal precision). Document conflict resolution rule in fusion layer.

2. **Predicate vocabulary.**
   AGON claims use free-string predicates today. Trinity pressure may force a controlled set. Defer until v1.0 of `tacitus-contracts` lands.

3. **gRPC vs REST priority.**
   REST is fine until p99 latency on `/api/perceive` becomes a pipeline bottleneck. Don't optimize prematurely.

4. **Multi-tenant model.**
   Today: shared Cloud SQL + Basic Auth. Real tenancy waits for enterprise (Phase D).

---

## How to contribute

- Pick a milestone above. Open an issue first.
- Conventional commits (`feat:`, `fix:`, `refactor:`, `docs:`, `test:`).
- All PRs must pass: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo audit`, `cargo deny check`.
- Golden fixture tests for any contradiction-rule or extraction change.

---

*Maintained by TACITUS. Updated as phases close.*
