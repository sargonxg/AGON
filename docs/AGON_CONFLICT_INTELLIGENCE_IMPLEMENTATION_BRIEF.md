# AGON Conflict Intelligence Implementation Brief

This brief distills `docs/research/AGON_CONFLICT_INTELLIGENCE_BLUEPRINT.md` into implementation guidance for the Rust app and embedded one-page dashboard.

## Product Thesis

AGON is a conflict primitive engine, not a chatbot. It should ingest messy human text and produce an auditable graph of actors, aliases, claims, events, commitments, contradictions, evidence spans, relationship pressure, escalation signals, and resolution openings.

The first paid wedge is enterprise HR/workplace investigations and internal mediation. That user needs fast, defensible visibility into who said what, where accounts diverge, which commitments are contested, and which source quotes support each conclusion.

## MVP Rules

- Every primitive needs source evidence, preferably a verbatim quote.
- Identity resolution must be conservative. False merges are worse than duplicate actors.
- Contradictions should be shown as model-suggested unless deterministic rules verified them.
- Friction scores must be explainable as additive factors, not black-box judgment.
- The UI should keep human judgment in control: surface evidence, confidence, materiality, and unresolved spans.

## Core Capability Surface

- One-page protected workbench.
- Streaming extraction with pipeline timing.
- Tabs: Overview, Actors, Friction, Contradictions, Evidence, Raw.
- Evidence ledger that marks quotes as verified or unresolved against the pasted source.
- Friction matrix with plain-language reasons.
- Contradiction cards with claim A, claim B, materiality, rationale, and available source quotes.
- Recent-session history backed by Cloud SQL.

## Extraction Fields To Favor

- `actors`: canonical label, aliases, role/kind, evidence.
- `claims`: actor, statement, polarity, optional subject actor, evidence.
- `events`: label, time/order, evidence.
- `commitments`: actor, recipient, subject, deadline, status, evidence.
- `relationships`: source actor, target actor, type, weight, evidence.
- `power_dynamics`: dominant actor, subordinate actor, basis, confidence, evidence.
- `escalation_signals`: actor, trigger, intensity, evidence.
- `resolution_opportunities`: actor, proposed concession or opening, evidence.
- `patterns`: kind, actor, confidence, evidence.
- `contradictions`: claim pair, materiality, source, confidence, rationale.

## Near-Term Engineering Queue

1. Keep the typed storage path stable and expand it gradually from claims/events/commitments/patterns to relationship and escalation tables.
2. Strengthen span recovery in Rust storage: exact quote match first, normalized/fuzzy recovery second, unresolved quarantine last.
3. Add deterministic contradiction helpers for denial pairs, contested commitments, and date/order conflicts.
4. Add report export after the dashboard can reliably show evidence and contradiction cards.
5. Keep advanced GraphRAG, legal issue spotting, settlement advice, and autonomous strategy generation out of the MVP.

## Demo Acceptance

A workplace or HR complaint sample should produce:

- Canonical actors with aliases.
- At least one claim or commitment per major actor.
- Evidence ledger entries that show whether the quote is verified against source text.
- A friction matrix with actor-pair pressure.
- Contradiction cards for material narrative conflicts.
- Raw JSON for audit/debugging.
