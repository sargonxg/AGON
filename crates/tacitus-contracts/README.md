# tacitus-contracts

**Single source of truth for typed primitives** across AGON (perception), DIALECTICA (graph reasoning), KAIROS (temporal/causal).

JSON Schema is canonical. Rust / Python / TypeScript types are generated.

## Why this exists

LLMs natively consume JSON Schema. Different downstream consumers want different languages. If we keep separate Rust structs, Python Pydantic, and TS interfaces, they drift. Drift = silent contract violations. Drift = lost evidence.

**Rule:** edit the schema, regen everything.

## Layout

```
schemas/              # canonical JSON Schemas (Draft 2020-12)
  doc.json
  actor.json
  claim.json
  event.json
  commitment.json
  contradiction.json
  pattern_match.json
  provenance.json
  calibration.json
  narrative_frame.json
  interest.json
  leverage.json

src/                  # Rust types
  lib.rs              # public API: re-exports types + loader helpers
  primitives.rs       # hand-written for now; typify-codegen replaces this when wired

python/               # (PROMPT 01 follow-up) pydantic v2 models
typescript/           # (PROMPT 01 follow-up) TS interfaces
```

## Evidence-Span quad form (non-negotiable)

Every primitive that references source carries an `EvidenceSpan`:

```json
{
  "segment_id": "blake3(canonical_text_of_segment)",
  "char_start_canonical": 142,
  "char_end_canonical": 198,
  "char_start_raw": 156,
  "char_end_raw": 212,
  "verbatim_quote": "I never said I'd own the deck.",
  "quote_hash": "blake3(verbatim_quote)",
  "normalization_version": "0.1.0"
}
```

Both canonical and raw offsets travel together so we can:
- recover the **exact bytes from the original document** even after OCR repair / NFC normalization;
- detect **drift** when re-running pipelines on the same source;
- show the user the **literal characters** they wrote, not our reconstruction.

## Adding a primitive

1. Edit/add the schema under `schemas/*.json`. Use `additionalProperties: false`. Use `$defs` for shared sub-types. Add `schema_version` const.
2. Add a `tests/test_<name>.rs` fixture: validate a known-good example against the schema using `jsonschema::Validator`.
3. Run `make contracts` (regens codegen — wired in follow-up commit).
4. Bump `schema_version` if breaking.

## Versioning

Schemas use semver via `schema_version`. Breaking changes: bump major, keep old schema under `schemas/v<N-1>/` for migration.

## License & maintainer

MIT OR Apache-2.0. Maintainer: Giulio Catanzariti `<giuliocatanzariti@gmail.com>`.
