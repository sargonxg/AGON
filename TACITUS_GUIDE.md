# AGON · purpose and practical use

Represent claims, actors, interests, constraints and commitments as reviewable objects anchored to source spans.

**Role:** Experimental conflict engine.

**Current boundary:** Experimental perception engine; inferred interests and narratives are proposals. Licensing remains unresolved in the current README.

## Start with one useful workflow

Use the Rust/toolchain versions in the existing run guide. Cargo is absent from this audit environment; these native commands were not run.

```sh
cargo test --locked --workspace
```

These application commands are separate from the teaching demo below. Check the recorded results before treating a command as verified. A successful bundle build does not establish provider availability, authenticated ownership or a working hosted deployment.

## How the code fits together

| Entry point | What to inspect |
| --- | --- |
| [crates/aco-core/src/claim.rs](crates/aco-core/src/claim.rs) | Source entry point for the project workflow; inspect its input/output types and failure path before downstream use. |
| [crates/aco-cli/src/main.rs](crates/aco-cli/src/main.rs) | Source entry point for the project workflow; inspect its input/output types and failure path before downstream use. |
| [crates/aco-core/src/commitment.rs](crates/aco-core/src/commitment.rs) | Source entry point for the project workflow; inspect its input/output types and failure path before downstream use. |

## Its place in TACITUS

The common goal is reusable, reviewable policy context: sources → explicit structure → task context → draft/artifact → human review. This repository contributes the role described above. Existing schemas, identity systems and review receipts remain authoritative within each implementation. A similar name such as “graph”, “context” or “capsule” does not establish binary or semantic compatibility.

For a downstream adapter, first pin a source/version and rights notice, state the task and output schema, retain attribution and time qualifications, then test export/re-import and refusal cases against the actual destination. No adapter is represented as working merely because the two projects appear in this portfolio.

## Play the idea before running the stack

[The House of Evidence](tacitus-demo/index.html) is an original four-room point-and-click adventure. Preserve a conditional promise, attribute a claim, inspect its chronology and download a brief with the uncertainty intact. It is a synthetic teaching fixture, not a run of this repository’s engine.

```sh
python -m http.server 8765 --bind 127.0.0.1 --directory tacitus-demo
```

Open `http://127.0.0.1:8765/`. The directory is also a standalone static artifact: copy its contents to a static host to publish only the adventure. No model, credential, database or package installation is needed. Opening `tacitus-demo/index.html` directly works in browsers that allow local files.

The shared teaching engine passed desktop (1440×1080) and mobile (390×844) Chromium checks for wrong answers, incomplete-review refusal, exact quotation export, separate notes, reset and horizontal overflow. [Fixture receipt](tacitus-demo/browser-verification.json). This verifies the shared demo, not the application.

## Research, credit and the contribution to evaluate

[Lewis et al., RAG (2020)](https://arxiv.org/abs/2005.11401), [Liu et al., Lost in the Middle (2023)](https://arxiv.org/abs/2307.03172), and [Edge et al., GraphRAG (2024; revised 2025)](https://arxiv.org/abs/2404.16130) motivate separating retrieval, context use and global synthesis. Read the [methods, evaluation limits and TACITUS design notes](https://github.com/sargonxg/sargonxg/blob/main/docs/RESEARCH_AND_DESIGN.md). Their results are not measurements of this repository.

Retain existing licenses and notices for application dependencies and bundled sources. The new adventure code, synthetic dispatches and generated mansion artwork are credited in [tacitus-demo/CREDITS.md](tacitus-demo/CREDITS.md). The demonstration does not relicense this repository or its external source documents.

The practical improvement to test is whether this workflow reduces a specific failure or repeated task: lost attribution, omitted conditions, stale evidence, untraceable drafting or repeated source preparation. Measure that against a matched baseline with source review, complete costs and held-out tasks before claiming analytical or efficiency gains.

## Verification in this pass

Audited default-branch baseline: `a8f63cc1fd4a31013cb4924aaf58fcb535a8851c`. Changes were prepared from a fresh shallow checkout. The audit covers this branch, not every historical or in-progress branch.

No application runtime acceptance was executed for this repository in this pass. The shared adventure was browser-tested separately.

A failure is a remaining release blocker for the affected workflow. A passing local test is not a new hosted release. Provider/model quality, current external-service setup and production deployment were not certified.
