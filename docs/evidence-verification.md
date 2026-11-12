# Verify a model-supplied quotation

A proposed claim may carry a plausible quotation and the hash of a real passage. Before reusing it, check both the displayed quotation and its exact source span.

```sh
cargo run --locked -p aco-cli -- verify-evidence \
  --text path/to/canonical-source.txt --span path/to/evidence-span.json
```

`evidence-span.json` uses the existing [EvidenceSpan contract](../crates/tacitus-contracts/schemas/evidence_span.json). Canonical offsets count Unicode code points; raw offsets are bytes. The input text must be the exact canonical text associated with the span. This command does not reconstruct normalization or verify the raw-byte mapping. Text is limited to 2 MB and the span JSON to 64 KB.

The command prints JSON to stdout. It exits zero only for an exact match; normalized/fuzzy diagnostics and failed checks exit 2. Logs go to stderr. The result includes a hash of the supplied canonical text, without publishing the document contents.

The verifier now rejects a displayed quotation whose hash differs from its declared hash, rejects empty spans and checks Unicode bounds by code points. These checks fix a case where a retained hash could label an altered displayed quotation as Exact. Fuzzy matching remains a bounded diagnostic, never an exact certificate.

This establishes quotation consistency, not source trust, semantic entailment, current applicability or expert approval. See the [model-assisted policy pipeline](https://github.com/sargonxg/TACITUS-Knowledge-Pipeline-open/tree/main/reference-pipeline) and [Workbench](https://github.com/sargonxg/tacitus-workbench) for a separate end-to-end drafting example. Those formats are not automatically interchangeable with AGON's contract.

Original TACITUS code is Apache-2.0, as approved by the repository owner on 12 November 2026. Third-party source/evidence/model terms remain separate; preserve LICENSE, NOTICE and upstream credit.

## Run the committed drafting exercise

```sh
cargo run --locked -p aco-cli -- verify-evidence --text examples/evidence-check/canonical.txt --span examples/evidence-check/span.json
# This intentionally altered displayed quotation must exit 2:
cargo run --locked -p aco-cli -- verify-evidence --text examples/evidence-check/canonical.txt --span examples/evidence-check/altered-span.json
```

The wording is a fictional diplomatic drafting exercise, not an attributed real statement. It demonstrates a check on the reservation “subject to instructions”, not a legal-effect determination. Actual command outputs are committed alongside the input.
