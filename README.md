# AGON

**A source-grounded conflict-perception engine from TACITUS.**

[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](Cargo.toml)
![Status](https://img.shields.io/badge/status-experimental-yellow)
![License](https://img.shields.io/badge/license-not%20finalized-lightgrey)

AGON is a Rust research system for turning difficult human text—transcripts, correspondence, minutes, case notes, and policy records—into typed, reviewable conflict objects. It focuses on actors, claims, interests, constraints, leverage, commitments, events, narratives, contradictions, friction signals, source spans, and provenance.

> **Experimental research software.** AGON is not a legal, clinical, human-resources, mediation, or autonomous decision product. Its outputs are analytical proposals for human review, not findings about people or disputes.

## 30-second read

- **Problem:** fluent summaries often blur attribution, evidence, chronology, commitments, contradiction, and uncertainty.
- **Approach:** typed contracts and deterministic validation surround optional model-based extraction.
- **Output:** a graph-shaped conflict scene in which claim-bearing objects point back to exact source spans.
- **Current scope:** an experimental Rust workspace, API/workbench scaffold, fixtures, tests, and deployment tooling. Production validation remains incomplete.

## What AGON is designed to preserve

| Object | Question it helps an analyst ask |
| --- | --- |
| Actor | Who is speaking, acting, affected, or represented? |
| Claim | What was asserted, by whom, and on what evidence? |
| Interest | What underlying objective may explain a stated position? |
| Constraint | What legal, procedural, financial, temporal, or relational limit applies? |
| Leverage | What resource or dependency changes bargaining power? |
| Commitment | Who promised what, to whom, by when, and with what status? |
| Event | What happened, when, and according to which source? |
| Narrative | What broader frame connects multiple claims? |
| Contradiction | Which statements or commitments cannot be jointly sustained? |
| Provenance | Which exact source span supports each object and transformation? |

Inference and observation are not interchangeable. Interests, narratives, patterns, and causal explanations require explicit uncertainty and review.

## Architecture

AGON is organized as a layered perception stack:

```text
raw text
   │
   ▼
segmentation, speaker turns, time expressions, lexical features
   │
   ▼
structured extraction proposals behind typed contracts
   │
   ▼
actor resolution, commitment state, chronology, span verification
   │
   ▼
friction, contradiction, and pattern proposals
   │
   ▼
calibration, abstention, provenance, review questions
   │
   ▼
API, graph output, and analyst-facing workbench
```

The workspace separates contracts, text processing, temporal analysis, extraction, embeddings, storage, fusion, inference, scoring, CLI, server, and benchmarks into independently testable crates. See [Cargo.toml](Cargo.toml) and [ARCHITECTURE.md](ARCHITECTURE.md) for the current structure.

Core design principles:

- **Evidence before fluency:** claim-bearing objects should cite exact source text.
- **Typed contracts:** models propose data; schemas and deterministic code define admissible structure.
- **Human review:** pattern labels and hypotheses remain contestable and should expose their basis.
- **Temporal state:** commitments and claims can be made, confirmed, contested, superseded, or broken.
- **Provider separation:** model choice should not define the public data contract.
- **Abstention over forced certainty:** insufficient evidence should remain visible as insufficient evidence.

## Build and inspect locally

Requirements vary by feature. A recent Rust toolchain is sufficient to inspect the workspace and run components that do not require external services.

```bash
git clone https://github.com/sargonxg/AGON.git
cd AGON
cargo test --workspace
cargo run -p aco-server --bin agon-server
```

Model-backed extraction, database features, and cloud deployment require additional configuration. Review [.env.example](.env.example), [BUILDPLAN.md](BUILDPLAN.md), and the operator documentation before supplying credentials or data. Do not upload confidential material until the deployment, retention, provider, and access-control path has been independently reviewed.

## Current evidence boundary

The repository contains substantial implementation work, but architecture, test counts, fixtures, and demonstrations do not by themselves establish analyst usefulness or real-world accuracy. Public claims about extraction quality, contradiction detection, calibration, pattern detection, or downstream decision improvement require published evaluation against appropriate baselines and human review.

Named conflict patterns are particularly sensitive. A textual resemblance to a pattern is not a diagnosis, intent determination, or conclusion about a person. Public interfaces should use neutral language, expose supporting spans, and make alternative explanations easy to record.

## Place in the TACITUS stack

- **AGON** explores evidence-linked conflict perception.
- [KAIROS](https://github.com/sargonxg/KAIROS-temporal-vision-TACITUS) explores temporal structure: events, episodes, overlap, delay, and commitment status.
- [DIALECTICA](https://github.com/sargonxg/A3_DIALECTICAbyTACITUS_v3) builds reviewable, portable context capsules for PRAXIS.
- [TCGC](https://github.com/sargonxg/TCGC_TACITUS-Conflict-Grammar-Corpus_BENCHMARK) provides a benchmark surface for structural conflict reasoning.

These repositories share a research direction but should be judged by their individual code, tests, status statements, and published evidence.

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) — system design and contracts
- [BUILDPLAN.md](BUILDPLAN.md) — implementation plan and dependencies
- [RESEARCH_QUESTIONS.md](RESEARCH_QUESTIONS.md) — open empirical and design questions
- [CONTRIBUTING.md](CONTRIBUTING.md) — contribution guidance
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — participation expectations

## License status

The licensing posture is not yet finalized. The root [LICENSE](LICENSE) file controls the current repository; do not infer redistribution rights from package metadata or badges. The root license and Cargo package metadata should be aligned before any formal public release.
