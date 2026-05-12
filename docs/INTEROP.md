# AGON Interop

How AGON exposes itself to other TACITUS services (especially DIALECTICA) and how it adopts the shared `tacitus-contracts` types.

> **Status — be honest.** This is a **specification + intent document**. AGON's wire format is stable today via `POST /api/perceive`. The `tacitus-contracts` package referenced here is **not yet published**; AGON-internal types in `aco-core` are still the source of truth. This doc declares how the migration will land.
>
> Paired with DIALECTICA's [`docs/integration/CONTRACTS.md`](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS/blob/main/docs/integration/CONTRACTS.md) — that's the conductor-side spec; this is the AGON-side mirror.

---

## Why this doc

AGON is one of three repos in the [TACITUS trinity](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS/tree/main/docs/integration). It runs standalone (paste-and-perceive demo, `agon-server` binary) and as the **evidence verification layer** inside DIALECTICA's extraction pipeline.

For trinity integration to work without rot, three things must be true:

1. AGON's wire format is **versioned and stable**.
2. AGON's internal types map cleanly to **shared contracts** that DIALECTICA + KAIROS also use.
3. AGON honors **cross-service conventions** (trace propagation, idempotency, contracts version negotiation).

This doc records all three.

---

## Wire surface

### Public endpoints (today)

| Endpoint | Purpose | Stability |
|---|---|---|
| `POST /api/perceive` | Run perception over text → return primitives | Stable; will be aliased to `/api/v1/perceive` |
| `POST /api/perceive/stream` | SSE variant | Stable |
| `GET  /api/sessions` | List sessions | Stable |
| `GET  /api/sessions/{id}` | Session detail | Stable |
| `GET  /api/sessions/{id}/report.md` | Markdown report | Stable |
| `GET  /api/info`, `/api/schema` | Service info, output schema | Stable |
| `GET  /healthz`, `/readyz` | Probes | Stable |

### Versioning plan

- **`/api/perceive`** (current) — kept as alias for one release after v1 lands
- **`/api/v1/perceive`** — first explicitly-versioned endpoint, target Phase B per [`../ROADMAP.md`](../ROADMAP.md)
- **`/api/v1/perceive` will emit `AnalysisEnvelope`** with `contracts_version` field once `tacitus-contracts@1.0.0` is published

### Request envelope (target shape, post-`tacitus-contracts@1.0`)

```json
{
  "envelope": {
    "contracts_version": "1.0.0",
    "trace_id": "<propagated>",
    "workspace_id": "ws_018f...",
    "document_id": "doc_018f...",
    "source_service": "dialectica"
  },
  "text": "...",
  "model": "flash-lite",
  "candidate_claims": [],  // optional: DIALECTICA-extracted claims for verification
  "options": {
    "verify_spans": true,
    "detect_contradictions": true,
    "emit_friction_matrix": true
  }
}
```

### Response envelope (target shape)

```json
{
  "envelope": {
    "contracts_version": "1.0.0",
    "trace_id": "<echoed>",
    "produced_at": "2026-05-12T22:00:00Z",
    "source_service": "agon"
  },
  "actors":          [/* Actor[] */],
  "claims":          [/* Claim[] with VerificationStatus */],
  "contradictions":  [/* Contradiction[] */],
  "commitments":     [/* Commitment[] */],
  "evidence_spans":  [/* SourceSpan[] */],
  "friction_matrix": [/* ... */],
  "quality_gates": {
    "evidence_coverage": 0.78,
    "ambiguity_score": 0.12,
    "unresolved_count": 7,
    "contested_count": 4
  },
  "inferences":      [/* deterministic findings */],
  "review_questions":[/* prompts for human reviewer */]
}
```

---

## Internal types → `tacitus-contracts` mapping

AGON's `aco-core` crate is the source of truth today. The migration to `tacitus-contracts` is **additive**: shared types come in as Cargo path/published dep, `aco-core` keeps internal helpers that don't belong in the contract.

### Identity types

| AGON `aco-core` | `tacitus-contracts` | Notes |
|---|---|---|
| `ActorId(String)` | `tacitus_contracts::ActorId` | Format: `act_<uuid7>`. AGON-local IDs preserved as `Actor.merged_from_agon` after fusion. |
| `ClaimId(String)` | `tacitus_contracts::ClaimId` | Format: `clm_<uuid7>` |
| AGON internal session id | not in contracts | AGON-local concept, not shared |
| AGON's `Provenance` | maps onto `SourceSpan` | See below |

### `EvidenceSpan` → `SourceSpan`

AGON's `EvidenceSpan` becomes `tacitus_contracts::SourceSpan` with all current fields preserved:

| AGON field | Contracts field |
|---|---|
| `document_hash` | `document_id` (via DocumentId lookup) |
| `char_start`, `char_end` | same |
| `quote_text` | `text` |
| `quote_sha256` | `sha256` |
| `match_kind` (Exact / Normalized / Unresolved) | `ConfidenceMarker` enum (EXACT / NORMALIZED / UNRESOLVED) |

### `Claim` → `Claim`

| AGON field | Contracts field | Notes |
|---|---|---|
| `subject_actor_id` | `subject: ActorId` | |
| `predicate` (free string) | `predicate: String` | Vocabulary remains free in v1.0; controlled set TBD in v1.1 |
| `object` | `object: oneof { actor / event / string }` | AGON adapts to discriminated union |
| `evidence_spans[]` | `evidence: SourceSpan[]` | Direct |
| `verification_status` | `verification: VerificationStatus` | `verified` / `unresolved` / `contradicted` / `denied` |
| `confidence: f64` | `confidence: float` | |
| `contradicted_by: Option<ClaimId>` | same | |

### `Contradiction` → `Contradiction`

| AGON | Contracts |
|---|---|
| `claim_a`, `claim_b` | same |
| `mechanism: ContradictionMechanism` (enum) | direct map: `DIRECT_NEGATION` / `DATE_INCONSISTENCY` / `ATTRIBUTION_DISPUTE` / `OBLIGATION_DENIAL` / `SEMANTIC` |
| `severity: 1..=5` | same |
| `confidence: f64` | same |
| `explanation: String` | same |

### `Commitment` → `Commitment`

AGON commitments are simpler than KAIROS commitments (no Allen-13 anchoring). In trinity mode, **KAIROS commitments win on conflict**. AGON commitments still emitted for standalone use; DIALECTICA fusion prefers KAIROS when both present (logs AGON variant as alternative).

| AGON | Contracts |
|---|---|
| `committer`, `target` | `committer: ActorId`, `target: ActorId` |
| `content: String` | same |
| `state` | `state: CommitmentState` (pledged / active / fulfilled / breached / contested / withdrawn) |
| `deadline: Option<DateTime>` | `deadline: Option<Timestamp>` |
| `evidence_spans[]` | `evidence: SourceSpan[]` |

---

## Cross-service conventions

### Trace propagation

AGON honors `X-Trace-Id` request header. If present:
- Echoed in `envelope.trace_id` of response
- Included in all structured logs for that request
- Propagated to downstream Vertex AI calls (where supported)

If absent: AGON mints `trace_id` as `agon_<uuid7>` and includes in response.

### Idempotency

`POST /api/v1/perceive` accepts optional header `Idempotency-Key`. Same key + same body (canonical JSON) within 24h returns cached result.

Internally: cache keyed on `(document_id, sha256(text), model, options)`.

### Compatibility check

Caller declares supported contracts range via `X-Contracts-Range: ">=1.0.0,<2.0.0"`. AGON responds with `envelope.contracts_version`. Caller is responsible for graceful handling on mismatch (DIALECTICA logs warning + falls back to skipping AGON post-pass).

### IAM-internal mode

Deployment variant `agon-internal` (planned):
- Cloud Run service accepting traffic only from DIALECTICA service account (IAM-bound)
- Basic Auth disabled
- Same wire contract
- Used by DIALECTICA's `evidence_verify` pipeline node in production

Public `agon-dev` continues with Basic Auth for demos.

---

## What AGON does NOT promise

These are out of scope for the wire contract:

- **No autonomous verdicts.** AGON returns structure; consumers decide.
- **No silent normalization.** If an evidence span can't be resolved, AGON marks `UNRESOLVED`; never fabricates a match.
- **No persistence guarantees across versions.** Storage schema may evolve; consumers should treat AGON as stateless from a contract perspective. Use `session_id` only within a single AGON version.
- **No real-time ingestion.** AGON is request/response. Streaming variant exists but the unit of work is still one document.

---

## Where this doc evolves

| Trigger | Doc update |
|---|---|
| `tacitus-contracts@0.1.0` published | Fill in concrete type imports + remove "target shape" caveats |
| `/api/v1/perceive` shipped | Move from "planned" to "stable" in endpoint table |
| New contradiction mechanism added | Update mapping table + bump contracts minor |
| Streaming format formalized | Add SSE event schema section |
| First DIALECTICA pipeline call in production | Add "consumer recipes" section with DIALECTICA examples |

---

## See also

- [`../README.md`](../README.md) — AGON product README
- [`../ROADMAP.md`](../ROADMAP.md) — phased plan (B1-B5 are this doc's work)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md) — internal architecture (42KB deep dive)
- DIALECTICA [`docs/integration/CONTRACTS.md`](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS/blob/main/docs/integration/CONTRACTS.md) — full shared-types spec
- DIALECTICA [`docs/integration/ONTOLOGY_MAPPING.md`](https://github.com/sargonxg/A2_DIALECTICAbyTACITUS/blob/main/docs/integration/ONTOLOGY_MAPPING.md) — how AGON outputs land in Conflict Grammar
- KAIROS [`docs/INTEROP.md`](https://github.com/sargonxg/KAIROS-temporal-vision-TACITUS/blob/main/docs/INTEROP.md) — temporal-side mirror

---

*Maintained by TACITUS. Update when wire contracts change.*
