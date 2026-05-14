//! HuggingFace model download helper.
//!
//! Pulls model files from `https://huggingface.co/<repo>/resolve/main/<file>`
//! using the optional `HF_TOKEN` env var. Stores under a content-addressed cache
//! at `<AGON_MODEL_CACHE>/<sha256>/`. Sha256-verifies after download.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use crate::traits::EncodeError;

/// Determine the model cache root. Honors `AGON_MODEL_CACHE`, falls back to
/// `<temp>/agon-models`. Created on first call.
pub fn cache_root() -> PathBuf {
    let root = std::env::var("AGON_MODEL_CACHE")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("agon-models"));
    let _ = std::fs::create_dir_all(&root);
    root
}

/// Read the HF token from env. None if not set.
fn hf_token() -> Option<String> {
    std::env::var("HF_TOKEN").ok().filter(|s| !s.is_empty())
}

/// Download a file from a HuggingFace repo. Returns the local cached path.
/// Skips download if the cached file already passes the expected SHA-256.
///
/// Arguments
/// - `repo`: e.g. `"BAAI/bge-m3"`
/// - `file`: e.g. `"onnx/model.onnx"` or `"tokenizer.json"`
/// - `expected_sha256`: Optional. If provided, verifies after download.
pub async fn ensure_hf_file(
    repo: &str,
    file: &str,
    expected_sha256: Option<&str>,
) -> Result<PathBuf, EncodeError> {
    let safe_repo = repo.replace('/', "__");
    let dest = cache_root().join(&safe_repo).join(file);

    if dest.exists() {
        if let Some(expected) = expected_sha256 {
            if file_sha256(&dest).ok().as_deref() == Some(expected) {
                return Ok(dest);
            }
            warn!(?dest, "cached file failed sha256 — redownloading");
        } else {
            return Ok(dest);
        }
    }

    if let Some(parent) = dest.parent() { std::fs::create_dir_all(parent).ok(); }

    let url = format!("https://huggingface.co/{repo}/resolve/main/{file}");
    let mut req = reqwest::Client::new().get(&url);
    if let Some(t) = hf_token() {
        req = req.bearer_auth(t);
    }
    let resp = req.send().await.map_err(|e| EncodeError::Download(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(EncodeError::Download(format!(
            "{} returned {}",
            url,
            resp.status()
        )));
    }
    let bytes = resp.bytes().await.map_err(|e| EncodeError::Download(e.to_string()))?;
    std::fs::write(&dest, &bytes).map_err(|e| EncodeError::Download(e.to_string()))?;
    info!(?dest, bytes = bytes.len(), "downloaded HF file");

    if let Some(expected) = expected_sha256 {
        let actual = file_sha256(&dest).map_err(|e| EncodeError::Download(e.to_string()))?;
        if actual != expected {
            return Err(EncodeError::Download(format!(
                "sha256 mismatch on {file}: expected {expected}, got {actual}"
            )));
        }
    }
    Ok(dest)
}

fn file_sha256(path: &Path) -> std::io::Result<String> {
    let mut hasher = Sha256::new();
    let mut f = std::fs::File::open(path)?;
    std::io::copy(&mut f, &mut hasher)?;
    Ok(hex::encode(hasher.finalize()))
}

// Tiny hex encoder so we don't pull in the `hex` crate just for this.
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let bytes = bytes.as_ref();
        let mut out = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0xf) as usize] as char);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_root_creates_dir() {
        let root = cache_root();
        assert!(root.exists());
    }

    #[test]
    fn hex_encodes() {
        assert_eq!(hex::encode([0xde, 0xad, 0xbe, 0xef]), "deadbeef");
    }
}
