//! `agon` — the AGON command-line.
#![forbid(unsafe_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs::File, io::Read, path::PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "agon", version, about = "AGON — source-linked conflict analysis tools")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print version metadata.
    Version,
    /// Verify an EvidenceSpan against an exact canonical UTF-8 text file.
    VerifyEvidence {
        #[arg(long)]
        text: PathBuf,
        #[arg(long)]
        span: PathBuf,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    aco_core::init();
    aco_llm::init();
    aco_embed::init();
    aco_storage::init();
    aco_perceive::init();
    aco_fuse::init();
    aco_infer::init();
    aco_score::init();
    aco_learn::init();

    let cli = Cli::parse();
    match cli.cmd {
        Cmd::VerifyEvidence { text, span } => {
            let canonical = read_bounded(&text, 2_000_000)?;
            let span: tacitus_contracts::EvidenceSpan =
                serde_json::from_str(&read_bounded(&span, 64_000)?)?;
            let result = aco_text::verify_span(&canonical, &span);
            let exact = matches!(result, aco_text::SpanVerification::Exact);
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "format":"agon-evidence-check/1", "verification":format!("{result:?}"), "exact":exact,
                    "canonical_text_blake3":blake3::hash(canonical.as_bytes()).to_hex().to_string(),
                    "offset_unit":"unicode_codepoints", "scope":"Exact quotation consistency only; does not establish semantic entailment, source trust or current applicability."
                }))?
            );
            if !exact {
                std::process::exit(2);
            }
        }
        Cmd::Version => {
            println!("agon {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}

fn read_bounded(path: &PathBuf, limit: u64) -> Result<String> {
    let mut text = String::new();
    File::open(path)?.take(limit + 1).read_to_string(&mut text)?;
    anyhow::ensure!(text.len() as u64 <= limit, "input exceeds the explicit file limit");
    Ok(text)
}
