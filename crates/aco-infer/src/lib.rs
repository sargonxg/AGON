//! `aco-infer` — deterministic conflict inference.
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::trace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceFinding {
    pub id: String,
    pub kind: String,
    pub actors: Vec<String>,
    pub parents: Vec<String>,
    pub confidence: f32,
    pub severity: String,
    pub rationale: String,
    pub source: String,
    pub evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    pub id: String,
    pub label: String,
    pub status: String,
    pub score: f32,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictAnalysis {
    pub inferences: Vec<InferenceFinding>,
    pub quality_gates: Vec<QualityGate>,
    pub review_questions: Vec<String>,
}

pub fn analyze_extraction(x: &Value) -> ConflictAnalysis {
    let mut findings = Vec::new();
    findings.extend(infer_commitment_conflicts(x));
    findings.extend(infer_escalation_loops(x));
    findings.extend(infer_repair_and_openings(x));

    let quality_gates = quality_gates(x, &findings);
    let review_questions = review_questions(x, &findings, &quality_gates);
    ConflictAnalysis { inferences: findings, quality_gates, review_questions }
}

fn infer_commitment_conflicts(x: &Value) -> Vec<InferenceFinding> {
    let mut out = Vec::new();
    let claims = array(x, "claims");
    let commitments = array(x, "commitments");
    for commitment in commitments {
        let status = str_field(commitment, "status").unwrap_or("");
        if !matches!(status, "contested" | "broken") {
            continue;
        }
        let subject = str_field(commitment, "subject").unwrap_or("");
        let by_actor = str_field(commitment, "by_actor").unwrap_or("");
        let to_actor = str_field(commitment, "to_actor").unwrap_or("");
        let parents = vec![str_field(commitment, "id").unwrap_or("commitment").to_string()];
        out.push(InferenceFinding {
            id: stable_id("commitment", &parents.join("|")),
            kind: if status == "broken" { "broken_commitment" } else { "contested_commitment" }
                .into(),
            actors: [by_actor, to_actor]
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect(),
            parents,
            confidence: 0.76,
            severity: if status == "broken" { "high" } else { "medium" }.into(),
            rationale: format!("Commitment status is `{status}` for `{subject}`."),
            source: "deterministic".into(),
            evidence: str_field(commitment, "evidence").map(str::to_string),
        });
    }

    for claim in claims {
        let text = str_field(claim, "text").unwrap_or("").to_ascii_lowercase();
        if text.contains("never agreed") || text.contains("did not agree") {
            out.push(InferenceFinding {
                id: stable_id("denied_obligation", text.as_str()),
                kind: "denied_obligation".into(),
                actors: str_field(claim, "actor_id")
                    .map(|s| vec![s.to_string()])
                    .unwrap_or_default(),
                parents: str_field(claim, "id").map(|s| vec![s.to_string()]).unwrap_or_default(),
                confidence: 0.72,
                severity: "medium".into(),
                rationale: "Actor explicitly denies agreement or obligation language.".into(),
                source: "deterministic".into(),
                evidence: str_field(claim, "evidence").map(str::to_string),
            });
        }
    }
    out
}

fn infer_escalation_loops(x: &Value) -> Vec<InferenceFinding> {
    let mut out = Vec::new();
    let mut by_actor: HashMap<String, usize> = HashMap::new();
    for signal in array(x, "escalation_signals") {
        if let Some(actor) = str_field(signal, "actor_id") {
            *by_actor.entry(actor.to_string()).or_default() += 1;
        }
    }
    for rel in array(x, "relationships") {
        let kind = str_field(rel, "type").unwrap_or("");
        if matches!(kind, "pressures" | "bypasses" | "retaliation_risk" | "accuses") {
            if let Some(actor) = str_field(rel, "from_actor") {
                *by_actor.entry(actor.to_string()).or_default() += 1;
            }
        }
    }
    for (actor, count) in by_actor {
        if count >= 2 {
            out.push(InferenceFinding {
                id: stable_id("escalation_loop", &actor),
                kind: "escalation_loop".into(),
                actors: vec![actor.clone()],
                parents: Vec::new(),
                confidence: (0.55 + count as f32 * 0.08).min(0.9),
                severity: if count >= 4 { "high" } else { "medium" }.into(),
                rationale: format!(
                    "Multiple pressure/escalation signals cluster around `{actor}`."
                ),
                source: "deterministic".into(),
                evidence: None,
            });
        }
    }
    out
}

fn infer_repair_and_openings(x: &Value) -> Vec<InferenceFinding> {
    let mut out = Vec::new();
    for opening in array(x, "resolution_opportunities") {
        let actor = str_field(opening, "actor_id").unwrap_or("");
        out.push(InferenceFinding {
            id: stable_id("repair_opening", str_field(opening, "opening").unwrap_or(actor)),
            kind: "repair_opening".into(),
            actors: if actor.is_empty() { Vec::new() } else { vec![actor.to_string()] },
            parents: Vec::new(),
            confidence: 0.66,
            severity: "opportunity".into(),
            rationale:
                "Resolution language or concession/opening is available for a human reviewer."
                    .into(),
            source: "deterministic".into(),
            evidence: str_field(opening, "evidence").map(str::to_string),
        });
    }
    out
}

fn quality_gates(x: &Value, findings: &[InferenceFinding]) -> Vec<QualityGate> {
    let evidence_rows = array(x, "evidence_audit");
    let verified =
        evidence_rows.iter().filter(|row| str_field(row, "status") == Some("verified")).count();
    let evidence_ratio =
        if evidence_rows.is_empty() { 0.0 } else { verified as f32 / evidence_rows.len() as f32 };
    let actors = array(x, "actors");
    let ambiguous_actors =
        actors.iter().filter(|a| str_field(a, "kind").unwrap_or("unknown") == "unknown").count();
    let contradictions = array(x, "contradictions").len();

    vec![
        gate(
            "evidence_coverage",
            "Verified evidence coverage",
            evidence_ratio,
            if evidence_ratio >= 0.8 {
                "pass"
            } else if evidence_ratio >= 0.5 {
                "review"
            } else {
                "fail"
            },
            format!("{verified}/{} primitive evidence quotes verified", evidence_rows.len()),
        ),
        gate(
            "actor_ambiguity",
            "Actor ambiguity",
            if actors.is_empty() {
                0.0
            } else {
                1.0 - ambiguous_actors as f32 / actors.len() as f32
            },
            if ambiguous_actors <= 1 { "pass" } else { "review" },
            format!("{ambiguous_actors} actors remain kind=unknown"),
        ),
        gate(
            "conflict_signal",
            "Conflict signal strength",
            ((contradictions + findings.len()) as f32 / 6.0).min(1.0),
            if contradictions + findings.len() >= 2 { "pass" } else { "review" },
            format!(
                "{contradictions} contradictions and {} deterministic findings",
                findings.len()
            ),
        ),
    ]
}

fn gate(id: &str, label: &str, score: f32, status: &str, detail: String) -> QualityGate {
    QualityGate { id: id.into(), label: label.into(), status: status.into(), score, detail }
}

fn review_questions(
    x: &Value,
    findings: &[InferenceFinding],
    quality_gates: &[QualityGate],
) -> Vec<String> {
    let mut q = Vec::new();
    if quality_gates.iter().any(|g| g.id == "evidence_coverage" && g.status != "pass") {
        q.push(
            "Which unresolved quotes should be verified manually against the source record?".into(),
        );
    }
    if !array(x, "contradictions").is_empty() {
        q.push(
            "Which contradiction is material to the decision, and which is only contextual?".into(),
        );
    }
    if findings.iter().any(|f| f.kind == "contested_commitment" || f.kind == "denied_obligation") {
        q.push("What exact words created, limited, or denied the contested obligation?".into());
    }
    if findings.iter().any(|f| f.kind == "escalation_loop") {
        q.push("What intervention would interrupt the repeated pressure/escalation loop?".into());
    }
    if q.is_empty() {
        q.push(
            "What additional source text would most reduce uncertainty in this conflict map?"
                .into(),
        );
    }
    q
}

fn array<'a>(value: &'a Value, key: &str) -> Vec<&'a Value> {
    value.get(key).and_then(Value::as_array).map(|v| v.iter().collect()).unwrap_or_default()
}

fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str).filter(|s| !s.trim().is_empty())
}

fn stable_id(prefix: &str, text: &str) -> String {
    let hash = blake3::hash(text.as_bytes()).to_hex().to_string();
    format!("{prefix}_{}", &hash[..16])
}

pub fn init() {
    trace!(crate_name = "aco-infer", "loaded");
}

#[cfg(test)]
mod tests {
    use super::analyze_extraction;

    #[test]
    fn infers_denied_obligation_and_review_question() {
        let x = serde_json::json!({
            "actors": [{"id":"a","label":"Alex","kind":"individual"}],
            "claims": [{"id":"c1","actor_id":"a","text":"I never agreed to own the deck","evidence":"never agreed"}],
            "evidence_audit": [{"status":"verified","kind":"claim"}],
            "contradictions": []
        });
        let result = analyze_extraction(&x);
        assert!(result.inferences.iter().any(|f| f.kind == "denied_obligation"));
        assert!(!result.quality_gates.is_empty());
        assert!(!result.review_questions.is_empty());
    }
}
