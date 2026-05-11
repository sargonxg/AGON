//! Cost ledger — tracks token usage per model.

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Per-model token totals.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    pub by_model: HashMap<String, ModelUsage>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsage {
    pub calls: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// Thread-safe cost ledger.
#[derive(Default, Debug, Clone)]
pub struct CostLedger {
    inner: Arc<Mutex<CostReport>>,
}

impl CostLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&self, model: &str, input: u32, output: u32) {
        let mut r = self.inner.lock();
        let e = r.by_model.entry(model.to_string()).or_default();
        e.calls += 1;
        e.input_tokens += input as u64;
        e.output_tokens += output as u64;
    }

    pub fn report(&self) -> CostReport {
        self.inner.lock().clone()
    }
}
