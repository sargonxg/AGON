# AGON MVP+++ Plan: Mediation/Legal Conflict Intelligence

## Current Repo Corrections

- `EvidenceSpan` and `Provenance` already exist in `aco-core` and should be reused, tightened, and mapped into persistence rather than replaced.
- Current GitHub Actions CI is red as of May 11, 2026. The latest `ci` and `audit` workflows fail on formatting, missing crate init hooks, and dependency audit policy/advisory findings.

## Summary

Build AGON into a credible mediation/legal MVP+++ focused on one paid workflow: ingest conflict text, extract claims/events/commitments, canonicalize actors, persist typed evidence-backed primitives, show a friction matrix and contradiction/evidence report, and keep every conclusion auditable to source text.

## Key Changes

- Stabilize the repo first:
  - Run `cargo fmt --all`, fix formatting failures, then verify `cargo fmt --all -- --check`.
  - Inspect/fix the latest CI/audit failures from GitHub Actions.
  - Keep the live Cloud Run app working while adding typed internals.

- Convert persistence from raw-session-only to typed MVP storage:
  - Add migrations for `documents`, `chunks`, `actors`, `claims`, `events`, `commitments`, `patterns`, `contradictions`, `evidence_spans`, and `graph_edges`.
  - Keep the existing `sessions` table for backwards-compatible dashboard history.
  - Store original `source_text` as a document/chunk with deterministic IDs and source hash.
  - Require every persisted primitive to reference an `EvidenceSpan`.

- Build the first real "trust core" pipeline:
  - `aco-server` still accepts `/api/perceive` and `/api/perceive/stream`.
  - After Vertex returns schema JSON, map extracted JSON into typed `aco-core` primitives.
  - Validate evidence quotes against source text with best-effort span recovery.
  - Persist typed primitives plus raw extraction JSON.
  - Compute the MVP friction matrix from persisted claims/contradictions, not only transient JSON.

- Build canonical actor MVP:
  - Implement `aco-fuse` with deterministic normalization first: case-folding, punctuation stripping, honorific removal, exact alias matching.
  - Defer `fastembed`/`hnsw_rs` until deterministic canonicalization is tested.
  - Persist aliases so "John Doe", "Doe", "the plaintiff", and "Mr. Doe" can be merged manually or by conservative rules.

- Build MVP contradiction and report surfaces:
  - Keep LLM-extracted contradictions as "model-suggested".
  - Add deterministic checks for simple denial conflicts, contested commitments, and date/order conflicts.
  - UI shows contradiction cards with claim A, claim B, actor labels, source quotes, materiality, and confidence/source.
  - Add a first export endpoint returning Markdown or JSON report; PDF/DOCX can wait.

- Improve the product UI without overbuilding:
  - Keep the existing embedded dashboard.
  - Add tabs: `Overview`, `Actors`, `Friction`, `Contradictions`, `Evidence`, `Raw`.
  - Keep the graph simple for MVP; Cytoscape.js can be a later replacement if the built-in SVG becomes limiting.
  - Make "auditable evidence" the main wow factor, not a generic chatbot.

## Deferred

- Defer `z3`, `good_lp`, Candle/Burn, GraphQL, OCR, RBAC, and full pgvector until typed persistence and evidence-backed workflow are solid.
- Defer enterprise multitenancy; MVP can remain single-deployment/single-tenant for trusted demos.
- Defer "local neural sensors" until the product has golden examples and regression metrics.

## Test Plan

- CI/stability:
  - `cargo fmt --all -- --check`
  - `cargo check --workspace`
  - `cargo test -p aco-core`
  - `cargo test -p aco-server`
  - `cargo test -p aco-storage` where DB-dependent tests are gated or use testcontainers.

- Persistence:
  - Insert a sample workplace/legal conflict and verify documents, chunks, actors, claims, contradictions, and evidence spans persist.
  - Verify old `/api/sessions` still returns usable history.

- Evidence:
  - For every extracted claim/event/commitment, assert quote text is present in source text or marked `span_status = unresolved`.
  - UI should never present unresolved evidence as fully verified.

- Canonicalization:
  - Test 20 alias variants for one actor and verify conservative merge behavior.
  - Test two similar but different people and verify they do not merge.

- Product scenarios:
  - Workplace dispute sample.
  - HR complaint sample.
  - Legal/deposition-style contradiction sample.
  - Commercial negotiation sample.

## First Three PRs

- PR 1: `docs/AGON_MVP_PLUS_PLAN.md` plus CI stabilization.
  - Save the audit and MVP+++ plan.
  - Fix formatting.
  - Document current working/partial/placeholder state.
  - Acceptance: CI formatting passes and the plan is committed.

- PR 2: typed storage foundation.
  - Add normalized MVP migrations.
  - Add `Store` methods for documents, chunks, actors, claims, evidence spans, and contradictions.
  - Keep session history compatibility.
  - Acceptance: typed insert/read tests pass and dashboard still loads prior session shape.

- PR 3: evidence-backed perceive pipeline.
  - Map Vertex JSON into typed primitives.
  - Resolve evidence quotes to source spans.
  - Persist typed primitives after each perception run.
  - Render contradiction/evidence panels from persisted data.
  - Acceptance: one legal/mediation sample produces actors, claims, contradictions, friction matrix, and clickable evidence quotes.

## Assumptions

- Product wedge is mediation/legal.
- First implementation wave prioritizes trust core over wow UI or advanced solvers.
- Save target is `docs/AGON_MVP_PLUS_PLAN.md`.
- Existing Cloud Run/Vertex/Cloud SQL deployment should remain compatible during MVP work.
- Current `aco-core` provenance types should be reused and tightened rather than replaced.
