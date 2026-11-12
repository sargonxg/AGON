# Third-party scope and attribution

Apache-2.0 applies to original TACITUS contributions. Dependency code, research, model weights and evidence remain under their original terms. Preserve their copyright/license notices when redistributing compiled dependencies; the root license does not replace them.

| Component | Contribution and limits | Original reference |
| --- | --- | --- |
| SQLx | PostgreSQL connection/query/migration runtime; original migrations and SQLx SHA-384 verification are retained. Compile-time multi-database macros are not required by this application. | https://github.com/launchbadge/sqlx |
| Tokio, Axum, reqwest | Async execution, HTTP service/client libraries; not TACITUS research methods. | https://tokio.rs/ ; https://github.com/tokio-rs/axum ; https://github.com/seanmonstar/reqwest |
| Ascent / good_lp | Declared symbolic/optimization dependencies; experimental engine behavior must be judged against its implemented fixtures. | https://github.com/s-arash/ascent ; https://github.com/rust-or/good_lp |
| fastembed / HNSW | Optional embedding-model catalogue and retrieval research; model weights have separate terms. Default sparse matching does not become neural inference by enabling a catalogue. | https://github.com/Anush008/fastembed-rs ; https://github.com/jean-pierreBoth/hnswlib-rs |
| Parked BGE/DeBERTa skeletons | Unimplemented reference scaffolding; no downloaded model or inference result is claimed. | Existing source skeleton and research documentation |

The resolved dependency licenses are checked by `cargo deny check`. Full gathered upstream license/copyright texts are retained in [THIRD_PARTY_LICENSES.txt](THIRD_PARTY_LICENSES.txt), generated using `cargo-about 0.9.3`, `about.toml` and `licenses.hbs`. Use the locked dependency versions and retain upstream license files for redistributed binaries. The current package remains research software; licensing is not a claim of analytical benefit, production acceptance or permission to reuse external evidence.

Existing research citations in the repository remain primary for the ideas they describe. The testable TACITUS contribution concerns typed conflict/evidence objects and reviewable source relationships, rather than ownership of those methods.

Reproduce the committed notice text with `cargo install cargo-about --version 0.9.3 --locked`, then `python scripts/generate-license-notices.py`. The generator preserves license wording/copyright and normalizes only line endings/trailing whitespace.
