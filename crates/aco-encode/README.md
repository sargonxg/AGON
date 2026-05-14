# aco-encode

Local neural encoders for AGON. Pure Rust via [`ort`](https://crates.io/crates/ort) 2.x (ONNX Runtime bindings) + [`tokenizers`](https://crates.io/crates/tokenizers) (HuggingFace).

## Status (2026-05-14)

| Component | Status |
|---|---|
| `Encoder` / `EmbedEncoder` / `NliEncoder` traits | ✅ defined |
| Model download + SHA-256 cache | ✅ done (`download::ensure_hf_file`) |
| BGE-M3 embedder | 🟡 scaffolded (feature `onnx`) |
| DeBERTa-NLI classifier | 🟡 scaffolded (feature `onnx`) |
| fastcoref | ☐ planned |
| Wired into perceive pipeline | ☐ planned |

The crate compiles in the default (no-`onnx`) configuration so the workspace builds anywhere. Real inference is gated behind `--features onnx` so the production container in `agon-batch` (PROMPT 12) can enable it without breaking everyone else's local builds.

## Why this matters

This is the crate that flips `local_encoders.status` from `"scaffolded"` to `"live"` in `/api/system`. Until it ships, AGON's only neural network is remote (Gemini via Vertex). After it ships:

- **Embeddings** become local — no API call latency, no token spend, runs at CPU speed on dev hosts and GPU speed on L4.
- **NLI** runs locally and gives a second-opinion contradiction signal that does not depend on Gemini.
- **Coref** lets multi-turn pattern detection (especially DARVO) handle paraphrased role-reversal — not just regex-pattern hits.

## Files

```
src/
  lib.rs          ← public re-exports, build_info()
  traits.rs       ← Encoder, EmbedEncoder, NliEncoder, EmbedResult, NliResult
  download.rs     ← HuggingFace fetcher with SHA-256 cache, HF_TOKEN auth
  bge_m3.rs       ← BGE-M3 embedder (feature = "onnx")
  deberta_nli.rs  ← DeBERTa-v3-large-mnli classifier (feature = "onnx")
```

## Build modes

```bash
# Default — compiles everywhere, all encoder methods return EncodeError::FeatureDisabled.
cargo build -p aco-encode

# With ort 2.x — needs ONNX Runtime native libs on PATH.
cargo build -p aco-encode --features onnx
```

## Model provenance

| Model | HF repo | License | Size |
|---|---|---|---|
| BGE-M3 | `BAAI/bge-m3` | MIT | ~2.2 GB |
| DeBERTa-v3-large-mnli | `MoritzLaurer/DeBERTa-v3-large-mnli-fever-anli-ling-wanli` | MIT | ~1.6 GB |
| fastcoref | `pie/fastcoref` | MIT | ~500 MB |

All download via `download::ensure_hf_file(repo, file, Some(expected_sha256))`. The SHA-256 pins guarantee reproducibility — if HF rotates a checkpoint we'll know.

## License

MIT OR Apache-2.0.
