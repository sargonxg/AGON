//! `agon` — the AGON command-line.
#![forbid(unsafe_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "agon", version, about = "AGON — a Tesla-style perception engine for human conflict")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print version metadata.
    Version,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with_target(false)
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
        Cmd::Version => {
            println!("agon {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
