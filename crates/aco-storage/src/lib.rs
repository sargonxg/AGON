//! `aco-storage` — Cloud SQL Postgres persistence for AGON.
//!
//! Stores the backwards-compatible `sessions` history plus typed MVP primitives
//! grounded in evidence spans.
#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("db: {0}")]
    Db(#[from] sqlx::Error),
    #[error("migrate: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

#[derive(Clone, Debug)]
pub struct Store {
    pool: PgPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub model: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub elapsed_ms: i64,
    pub source_text: String,
    pub friction_score: i32,
    pub n_actors: i32,
    pub n_claims: i32,
    pub n_events: i32,
    pub n_patterns: i32,
    pub n_contradictions: i32,
    pub extraction: Value,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedPerception {
    pub session_id: Uuid,
    pub document_id: String,
    pub chunk_id: String,
    pub source_hash: String,
    pub n_evidence_spans: i64,
    pub n_claims: i64,
    pub n_contradictions: i64,
}

impl Store {
    /// Connect using a libpq-style DSN.
    pub async fn connect(dsn: &str) -> Result<Self, StoreError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(60))
            .connect(dsn)
            .await?;
        Ok(Self { pool })
    }

    pub fn dsn_from_parts(host: &str, user: &str, password: &str, dbname: &str) -> String {
        format!("postgres://{user}:{password}@{host}:5432/{dbname}?sslmode=disable")
    }

    pub async fn migrate(&self) -> Result<(), StoreError> {
        sqlx::migrate!("../../migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn insert_session(&self, s: &Session) -> Result<(), StoreError> {
        let mut tx = self.pool.begin().await?;
        insert_session_tx(&mut tx, s).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn insert_perception(&self, s: &Session) -> Result<PersistedPerception, StoreError> {
        let mut tx = self.pool.begin().await?;
        insert_session_tx(&mut tx, s).await?;
        let persisted = insert_typed_perception_tx(&mut tx, s).await?;
        tx.commit().await?;
        Ok(persisted)
    }

    pub async fn recent_sessions(&self, limit: i64) -> Result<Vec<SessionSummary>, StoreError> {
        let rows = sqlx::query(
            r#"
            SELECT id, created_at, model, friction_score,
                   n_actors, n_claims, n_events, n_patterns, n_contradictions,
                   summary, input_tokens, output_tokens, elapsed_ms
            FROM sessions ORDER BY created_at DESC LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| SessionSummary {
                id: r.get("id"),
                created_at: r.get("created_at"),
                model: r.get("model"),
                friction_score: r.get("friction_score"),
                n_actors: r.get("n_actors"),
                n_claims: r.get("n_claims"),
                n_events: r.get("n_events"),
                n_patterns: r.get("n_patterns"),
                n_contradictions: r.get("n_contradictions"),
                summary: r.get("summary"),
                input_tokens: r.get("input_tokens"),
                output_tokens: r.get("output_tokens"),
                elapsed_ms: r.get("elapsed_ms"),
            })
            .collect())
    }

    pub async fn get_session(&self, id: Uuid) -> Result<Option<Session>, StoreError> {
        let row = sqlx::query(
            r#"
            SELECT id, created_at, model, input_tokens, output_tokens, elapsed_ms,
                   source_text, friction_score, n_actors, n_claims, n_events,
                   n_patterns, n_contradictions, extraction, summary
            FROM sessions WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| Session {
            id: r.get("id"),
            created_at: r.get("created_at"),
            model: r.get("model"),
            input_tokens: r.get("input_tokens"),
            output_tokens: r.get("output_tokens"),
            elapsed_ms: r.get("elapsed_ms"),
            source_text: r.get("source_text"),
            friction_score: r.get("friction_score"),
            n_actors: r.get("n_actors"),
            n_claims: r.get("n_claims"),
            n_events: r.get("n_events"),
            n_patterns: r.get("n_patterns"),
            n_contradictions: r.get("n_contradictions"),
            extraction: r.get("extraction"),
            summary: r.get("summary"),
        }))
    }
}

async fn insert_session_tx(
    tx: &mut Transaction<'_, Postgres>,
    s: &Session,
) -> Result<(), StoreError> {
    sqlx::query(
        r#"
        INSERT INTO sessions
            (id, created_at, model, input_tokens, output_tokens, elapsed_ms,
             source_text, friction_score, n_actors, n_claims, n_events,
             n_patterns, n_contradictions, extraction, summary)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
        ON CONFLICT (id) DO UPDATE SET
            model = EXCLUDED.model,
            input_tokens = EXCLUDED.input_tokens,
            output_tokens = EXCLUDED.output_tokens,
            elapsed_ms = EXCLUDED.elapsed_ms,
            source_text = EXCLUDED.source_text,
            friction_score = EXCLUDED.friction_score,
            n_actors = EXCLUDED.n_actors,
            n_claims = EXCLUDED.n_claims,
            n_events = EXCLUDED.n_events,
            n_patterns = EXCLUDED.n_patterns,
            n_contradictions = EXCLUDED.n_contradictions,
            extraction = EXCLUDED.extraction,
            summary = EXCLUDED.summary
        "#,
    )
    .bind(s.id)
    .bind(s.created_at)
    .bind(&s.model)
    .bind(s.input_tokens)
    .bind(s.output_tokens)
    .bind(s.elapsed_ms)
    .bind(&s.source_text)
    .bind(s.friction_score)
    .bind(s.n_actors)
    .bind(s.n_claims)
    .bind(s.n_events)
    .bind(s.n_patterns)
    .bind(s.n_contradictions)
    .bind(&s.extraction)
    .bind(&s.summary)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_typed_perception_tx(
    tx: &mut Transaction<'_, Postgres>,
    s: &Session,
) -> Result<PersistedPerception, StoreError> {
    let source_hash = hash_text(&s.source_text);
    let document_id = deterministic_id("doc", &format!("{}:{source_hash}", s.id));
    let chunk_id = deterministic_id("chunk", &format!("{document_id}:0:{source_hash}"));
    let char_end = s.source_text.chars().count() as i32;

    sqlx::query(
        r#"
        INSERT INTO documents (id, session_id, source_hash, title, source_kind, raw_metadata)
        VALUES ($1, $2, $3, 'perception input', 'text', $4)
        ON CONFLICT (id) DO UPDATE SET raw_metadata = EXCLUDED.raw_metadata
        "#,
    )
    .bind(&document_id)
    .bind(s.id)
    .bind(&source_hash)
    .bind(serde_json::json!({ "model": s.model }))
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO chunks (id, document_id, ordinal, text, char_start, char_end, source_hash)
        VALUES ($1, $2, 0, $3, 0, $4, $5)
        ON CONFLICT (id) DO UPDATE SET text = EXCLUDED.text, char_end = EXCLUDED.char_end
        "#,
    )
    .bind(&chunk_id)
    .bind(&document_id)
    .bind(&s.source_text)
    .bind(char_end)
    .bind(&source_hash)
    .execute(&mut **tx)
    .await?;

    insert_document_segments(tx, s, &document_id).await?;

    let mut n_evidence_spans = 0;
    let mut n_claims = 0;
    let mut n_contradictions = 0;
    let mut claim_lookup = HashMap::new();

    if let Some(actors) = array(&s.extraction, "actors") {
        for actor in actors {
            let raw_id = str_field(actor, "id")
                .unwrap_or_else(|| str_field(actor, "label").unwrap_or("actor"));
            let label = str_field(actor, "label").unwrap_or(raw_id);
            let actor_id = stable_external_id("actor", raw_id, label);
            let evidence_id = insert_evidence_span(
                tx,
                s.id,
                &document_id,
                &chunk_id,
                "actor",
                &actor_id,
                str_field(actor, "evidence").unwrap_or(label),
                &s.source_text,
            )
            .await?;
            n_evidence_spans += 1;

            sqlx::query(
                r#"
                INSERT INTO actors (id, session_id, canonical_name, kind, evidence_span_id, raw_json)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (id) DO UPDATE SET
                    canonical_name = EXCLUDED.canonical_name,
                    kind = EXCLUDED.kind,
                    evidence_span_id = EXCLUDED.evidence_span_id,
                    raw_json = EXCLUDED.raw_json
                "#,
            )
            .bind(&actor_id)
            .bind(s.id)
            .bind(label)
            .bind(str_field(actor, "kind").unwrap_or("unknown"))
            .bind(&evidence_id)
            .bind(actor)
            .execute(&mut **tx)
            .await?;

            insert_actor_alias(tx, &actor_id, label, "canonical").await?;
            if let Some(aliases) = actor.get("aliases").and_then(Value::as_array) {
                for alias in aliases.iter().filter_map(Value::as_str) {
                    insert_actor_alias(tx, &actor_id, alias, "model").await?;
                }
            }
        }
    }

    if let Some(claims) = array(&s.extraction, "claims") {
        for claim in claims {
            let raw_id = str_field(claim, "id")
                .unwrap_or_else(|| str_field(claim, "text").unwrap_or("claim"));
            let text = str_field(claim, "text").unwrap_or("");
            let claim_id = stable_external_id("claim", raw_id, text);
            remember_claim_ref(&mut claim_lookup, raw_id, &claim_id);
            remember_claim_ref(&mut claim_lookup, text, &claim_id);
            let evidence_id = insert_evidence_span(
                tx,
                s.id,
                &document_id,
                &chunk_id,
                "claim",
                &claim_id,
                str_field(claim, "evidence").unwrap_or(text),
                &s.source_text,
            )
            .await?;
            n_evidence_spans += 1;

            let actor_id =
                str_field(claim, "actor_id").map(|id| stable_external_id("actor", id, id));
            sqlx::query(
                r#"
                INSERT INTO claims (id, session_id, actor_id, text, polarity, evidence_span_id, raw_json)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO UPDATE SET
                    actor_id = EXCLUDED.actor_id,
                    text = EXCLUDED.text,
                    polarity = EXCLUDED.polarity,
                    evidence_span_id = EXCLUDED.evidence_span_id,
                    raw_json = EXCLUDED.raw_json
                "#,
            )
            .bind(&claim_id)
            .bind(s.id)
            .bind(actor_id.as_deref())
            .bind(text)
            .bind(str_field(claim, "polarity").unwrap_or("ambiguous"))
            .bind(&evidence_id)
            .bind(claim)
            .execute(&mut **tx)
            .await?;
            n_claims += 1;

            if let Some(actor_id) = actor_id {
                insert_graph_edge(tx, s.id, &actor_id, &claim_id, "ASSERTS", 1.0, claim).await?;
            }
        }
    }

    if let Some(events) = array(&s.extraction, "events") {
        for event in events {
            let raw_id = str_field(event, "id")
                .unwrap_or_else(|| str_field(event, "label").unwrap_or("event"));
            let label = str_field(event, "label").unwrap_or("");
            let event_id = stable_external_id("event", raw_id, label);
            let evidence_id = insert_evidence_span(
                tx,
                s.id,
                &document_id,
                &chunk_id,
                "event",
                &event_id,
                str_field(event, "evidence").unwrap_or(label),
                &s.source_text,
            )
            .await?;
            n_evidence_spans += 1;

            sqlx::query(
                r#"
                INSERT INTO events (id, session_id, label, event_time, evidence_span_id, raw_json)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (id) DO UPDATE SET
                    label = EXCLUDED.label,
                    event_time = EXCLUDED.event_time,
                    evidence_span_id = EXCLUDED.evidence_span_id,
                    raw_json = EXCLUDED.raw_json
                "#,
            )
            .bind(&event_id)
            .bind(s.id)
            .bind(label)
            .bind(str_field(event, "when"))
            .bind(&evidence_id)
            .bind(event)
            .execute(&mut **tx)
            .await?;
        }
    }

    if let Some(commitments) = array(&s.extraction, "commitments") {
        for commitment in commitments {
            let raw_id = str_field(commitment, "id")
                .unwrap_or_else(|| str_field(commitment, "subject").unwrap_or("commitment"));
            let subject = str_field(commitment, "subject").unwrap_or("");
            let commitment_id = stable_external_id("commitment", raw_id, subject);
            let evidence_id = insert_evidence_span(
                tx,
                s.id,
                &document_id,
                &chunk_id,
                "commitment",
                &commitment_id,
                str_field(commitment, "evidence").unwrap_or(subject),
                &s.source_text,
            )
            .await?;
            n_evidence_spans += 1;

            let actor_id =
                str_field(commitment, "by_actor").map(|id| stable_external_id("actor", id, id));
            sqlx::query(
                r#"
                INSERT INTO commitments
                    (id, session_id, by_actor_id, subject, deadline, status, evidence_span_id, raw_json)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (id) DO UPDATE SET
                    by_actor_id = EXCLUDED.by_actor_id,
                    subject = EXCLUDED.subject,
                    deadline = EXCLUDED.deadline,
                    status = EXCLUDED.status,
                    evidence_span_id = EXCLUDED.evidence_span_id,
                    raw_json = EXCLUDED.raw_json
                "#,
            )
            .bind(&commitment_id)
            .bind(s.id)
            .bind(actor_id.as_deref())
            .bind(subject)
            .bind(str_field(commitment, "deadline"))
            .bind(str_field(commitment, "status").unwrap_or("proposed"))
            .bind(&evidence_id)
            .bind(commitment)
            .execute(&mut **tx)
            .await?;
        }
    }

    if let Some(patterns) = array(&s.extraction, "patterns") {
        for (idx, pattern) in patterns.iter().enumerate() {
            let kind = str_field(pattern, "kind").unwrap_or("other");
            let pattern_id = stable_external_id("pattern", &idx.to_string(), kind);
            let evidence_id = insert_evidence_span(
                tx,
                s.id,
                &document_id,
                &chunk_id,
                "pattern",
                &pattern_id,
                str_field(pattern, "evidence").unwrap_or(kind),
                &s.source_text,
            )
            .await?;
            n_evidence_spans += 1;

            let actor_id =
                str_field(pattern, "actor_id").map(|id| stable_external_id("actor", id, id));
            sqlx::query(
                r#"
                INSERT INTO patterns
                    (id, session_id, kind, actor_id, confidence, evidence_span_id, raw_json)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO UPDATE SET
                    kind = EXCLUDED.kind,
                    actor_id = EXCLUDED.actor_id,
                    confidence = EXCLUDED.confidence,
                    evidence_span_id = EXCLUDED.evidence_span_id,
                    raw_json = EXCLUDED.raw_json
                "#,
            )
            .bind(&pattern_id)
            .bind(s.id)
            .bind(kind)
            .bind(actor_id.as_deref())
            .bind(pattern.get("confidence").and_then(Value::as_f64).unwrap_or(0.0))
            .bind(&evidence_id)
            .bind(pattern)
            .execute(&mut **tx)
            .await?;
        }
    }

    if let Some(contradictions) = array(&s.extraction, "contradictions") {
        for contradiction in contradictions {
            let Some(claim_a_raw) = str_field(contradiction, "claim_a") else {
                continue;
            };
            let Some(claim_b_raw) = str_field(contradiction, "claim_b") else {
                continue;
            };
            let Some(claim_a) = resolve_claim_ref(&claim_lookup, claim_a_raw) else {
                continue;
            };
            let Some(claim_b) = resolve_claim_ref(&claim_lookup, claim_b_raw) else {
                continue;
            };
            let contradiction_id =
                deterministic_id("contradiction", &format!("{claim_a}:{claim_b}"));
            let source = str_field(contradiction, "source").unwrap_or("model_suggested");
            let confidence = contradiction.get("confidence").and_then(Value::as_f64).unwrap_or(0.5);
            sqlx::query(
                r#"
                INSERT INTO contradictions
                    (id, session_id, claim_a, claim_b, materiality, source, confidence, rationale, raw_json)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (id) DO UPDATE SET
                    materiality = EXCLUDED.materiality,
                    source = EXCLUDED.source,
                    confidence = EXCLUDED.confidence,
                    rationale = EXCLUDED.rationale,
                    raw_json = EXCLUDED.raw_json
                "#,
            )
            .bind(&contradiction_id)
            .bind(s.id)
            .bind(&claim_a)
            .bind(&claim_b)
            .bind(str_field(contradiction, "materiality").unwrap_or("cosmetic"))
            .bind(source)
            .bind(confidence)
            .bind(str_field(contradiction, "rationale").unwrap_or(""))
            .bind(contradiction)
            .execute(&mut **tx)
            .await?;
            insert_graph_edge(tx, s.id, &claim_a, &claim_b, "CONTRADICTS", 1.0, contradiction)
                .await?;
            n_contradictions += 1;
        }
    }

    insert_neural_signals(tx, s).await?;
    insert_inference_findings(tx, s).await?;
    insert_quality_gates(tx, s).await?;

    Ok(PersistedPerception {
        session_id: s.id,
        document_id,
        chunk_id,
        source_hash,
        n_evidence_spans,
        n_claims,
        n_contradictions,
    })
}

async fn insert_evidence_span(
    tx: &mut Transaction<'_, Postgres>,
    session_id: Uuid,
    document_id: &str,
    chunk_id: &str,
    primitive_kind: &str,
    primitive_id: &str,
    quote: &str,
    source_text: &str,
) -> Result<String, StoreError> {
    let span = resolve_quote_span(source_text, quote);
    let evidence_id = deterministic_id(
        "evidence",
        &format!("{session_id}:{primitive_kind}:{primitive_id}:{quote}"),
    );
    let (char_start, char_end, span_status) = match span {
        Some((start, end)) => (Some(start as i32), Some(end as i32), "verified"),
        None => (None, None, "unresolved"),
    };
    sqlx::query(
        r#"
        INSERT INTO evidence_spans
            (id, session_id, document_id, chunk_id, primitive_kind, primitive_id,
             quote, char_start, char_end, span_status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (id) DO UPDATE SET
            quote = EXCLUDED.quote,
            char_start = EXCLUDED.char_start,
            char_end = EXCLUDED.char_end,
            span_status = EXCLUDED.span_status
        "#,
    )
    .bind(&evidence_id)
    .bind(session_id)
    .bind(document_id)
    .bind(chunk_id)
    .bind(primitive_kind)
    .bind(primitive_id)
    .bind(quote)
    .bind(char_start)
    .bind(char_end)
    .bind(span_status)
    .execute(&mut **tx)
    .await?;
    Ok(evidence_id)
}

async fn insert_document_segments(
    tx: &mut Transaction<'_, Postgres>,
    s: &Session,
    document_id: &str,
) -> Result<(), StoreError> {
    let Some(profile) = s.extraction.get("document_profile") else {
        return Ok(());
    };
    let Some(segments) = profile.get("segments").and_then(Value::as_array) else {
        return Ok(());
    };
    for segment in segments {
        let raw_id = str_field(segment, "id").unwrap_or("segment");
        let segment_id = deterministic_id("segment", &format!("{}:{raw_id}", s.id));
        sqlx::query(
            r#"
            INSERT INTO document_segments
                (id, session_id, document_id, kind, label, char_start, char_end, raw_json)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                kind = EXCLUDED.kind,
                label = EXCLUDED.label,
                char_start = EXCLUDED.char_start,
                char_end = EXCLUDED.char_end,
                raw_json = EXCLUDED.raw_json
            "#,
        )
        .bind(&segment_id)
        .bind(s.id)
        .bind(document_id)
        .bind(str_field(segment, "kind").unwrap_or("segment"))
        .bind(str_field(segment, "label").unwrap_or(raw_id))
        .bind(segment.get("char_start").and_then(Value::as_i64).unwrap_or(0) as i32)
        .bind(segment.get("char_end").and_then(Value::as_i64).unwrap_or(0) as i32)
        .bind(segment)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_neural_signals(
    tx: &mut Transaction<'_, Postgres>,
    s: &Session,
) -> Result<(), StoreError> {
    let signals =
        s.extraction.get("neural_signals").and_then(|v| v.get("signals")).and_then(Value::as_array);
    let Some(signals) = signals else {
        return Ok(());
    };
    for signal in signals {
        let a = str_field(signal, "a").unwrap_or("");
        let b = str_field(signal, "b").unwrap_or("");
        let kind = str_field(signal, "kind").unwrap_or("signal");
        let signal_id = deterministic_id("neural", &format!("{}:{kind}:{a}:{b}", s.id));
        sqlx::query(
            r#"
            INSERT INTO neural_signals
                (id, session_id, kind, source_id, target_id, score, model, rationale, raw_json)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                score = EXCLUDED.score,
                model = EXCLUDED.model,
                rationale = EXCLUDED.rationale,
                raw_json = EXCLUDED.raw_json
            "#,
        )
        .bind(&signal_id)
        .bind(s.id)
        .bind(kind)
        .bind(a)
        .bind(b)
        .bind(signal.get("score").and_then(Value::as_f64).unwrap_or(0.0))
        .bind(str_field(signal, "model").unwrap_or(""))
        .bind(str_field(signal, "rationale").unwrap_or(""))
        .bind(signal)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_inference_findings(
    tx: &mut Transaction<'_, Postgres>,
    s: &Session,
) -> Result<(), StoreError> {
    let Some(findings) = array(&s.extraction, "inferences") else {
        return Ok(());
    };
    for finding in findings {
        let raw_id = str_field(finding, "id").unwrap_or("finding");
        let finding_id = deterministic_id("finding", &format!("{}:{raw_id}", s.id));
        sqlx::query(
            r#"
            INSERT INTO inference_findings
                (id, session_id, kind, severity, confidence, source, rationale, raw_json)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                severity = EXCLUDED.severity,
                confidence = EXCLUDED.confidence,
                source = EXCLUDED.source,
                rationale = EXCLUDED.rationale,
                raw_json = EXCLUDED.raw_json
            "#,
        )
        .bind(&finding_id)
        .bind(s.id)
        .bind(str_field(finding, "kind").unwrap_or("inference"))
        .bind(str_field(finding, "severity").unwrap_or("medium"))
        .bind(finding.get("confidence").and_then(Value::as_f64).unwrap_or(0.0))
        .bind(str_field(finding, "source").unwrap_or("deterministic"))
        .bind(str_field(finding, "rationale").unwrap_or(""))
        .bind(finding)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_quality_gates(
    tx: &mut Transaction<'_, Postgres>,
    s: &Session,
) -> Result<(), StoreError> {
    let Some(gates) = array(&s.extraction, "quality_gates") else {
        return Ok(());
    };
    for gate in gates {
        let raw_id = str_field(gate, "id").unwrap_or("gate");
        let gate_id = deterministic_id("gate", &format!("{}:{raw_id}", s.id));
        sqlx::query(
            r#"
            INSERT INTO quality_gates
                (id, session_id, label, status, score, detail, raw_json)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                label = EXCLUDED.label,
                status = EXCLUDED.status,
                score = EXCLUDED.score,
                detail = EXCLUDED.detail,
                raw_json = EXCLUDED.raw_json
            "#,
        )
        .bind(&gate_id)
        .bind(s.id)
        .bind(str_field(gate, "label").unwrap_or(raw_id))
        .bind(str_field(gate, "status").unwrap_or("review"))
        .bind(gate.get("score").and_then(Value::as_f64).unwrap_or(0.0))
        .bind(str_field(gate, "detail").unwrap_or(""))
        .bind(gate)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_actor_alias(
    tx: &mut Transaction<'_, Postgres>,
    actor_id: &str,
    alias: &str,
    source: &str,
) -> Result<(), StoreError> {
    sqlx::query(
        r#"
        INSERT INTO actor_aliases (actor_id, alias, normalized_alias, source)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (actor_id, normalized_alias) DO UPDATE SET alias = EXCLUDED.alias
        "#,
    )
    .bind(actor_id)
    .bind(alias)
    .bind(normalize_alias(alias))
    .bind(source)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_graph_edge(
    tx: &mut Transaction<'_, Postgres>,
    session_id: Uuid,
    source_id: &str,
    target_id: &str,
    edge_type: &str,
    weight: f64,
    raw_json: &Value,
) -> Result<(), StoreError> {
    let edge_id =
        deterministic_id("edge", &format!("{session_id}:{source_id}:{edge_type}:{target_id}"));
    sqlx::query(
        r#"
        INSERT INTO graph_edges (id, session_id, source_id, target_id, edge_type, weight, raw_json)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id) DO UPDATE SET weight = EXCLUDED.weight, raw_json = EXCLUDED.raw_json
        "#,
    )
    .bind(edge_id)
    .bind(session_id)
    .bind(source_id)
    .bind(target_id)
    .bind(edge_type)
    .bind(weight)
    .bind(raw_json)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn array<'a>(value: &'a Value, key: &str) -> Option<&'a Vec<Value>> {
    value.get(key).and_then(Value::as_array)
}

fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str).filter(|s| !s.trim().is_empty())
}

fn remember_claim_ref(claim_lookup: &mut HashMap<String, String>, raw_ref: &str, claim_id: &str) {
    let key = claim_ref_key(raw_ref);
    if !key.is_empty() {
        claim_lookup.entry(key).or_insert_with(|| claim_id.to_string());
    }
}

fn resolve_claim_ref(claim_lookup: &HashMap<String, String>, raw_ref: &str) -> Option<String> {
    claim_lookup.get(&claim_ref_key(raw_ref)).cloned()
}

fn claim_ref_key(raw_ref: &str) -> String {
    raw_ref.trim().to_ascii_lowercase()
}

fn stable_external_id(kind: &str, raw_id: &str, fallback: &str) -> String {
    let normalized = normalize_alias(raw_id);
    if normalized.is_empty() {
        deterministic_id(kind, fallback)
    } else {
        format!("{kind}_{normalized}")
    }
}

fn hash_text(text: &str) -> String {
    blake3::hash(text.as_bytes()).to_hex().to_string()
}

fn deterministic_id(prefix: &str, text: &str) -> String {
    let hash = blake3::hash(text.as_bytes()).to_hex().to_string();
    format!("{prefix}_{}", &hash[..24])
}

fn resolve_quote_span(source_text: &str, quote: &str) -> Option<(usize, usize)> {
    let needle = quote.trim();
    if needle.is_empty() {
        return None;
    }
    if let Some(byte_start) = source_text.find(needle) {
        let char_start = source_text[..byte_start].chars().count();
        let char_len = needle.chars().count();
        return Some((char_start, char_start + char_len));
    }
    resolve_normalized_quote_span(source_text, needle)
}

fn resolve_normalized_quote_span(source_text: &str, quote: &str) -> Option<(usize, usize)> {
    let source_chars: Vec<char> = source_text.chars().collect();
    let source_norm: Vec<(char, usize)> = source_chars
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| normalize_match_char(*ch).map(|c| (c, idx)))
        .collect();
    let quote_norm: Vec<char> = quote.chars().filter_map(normalize_match_char).collect();
    if quote_norm.is_empty() || quote_norm.len() > source_norm.len() {
        return None;
    }

    let start = source_norm
        .windows(quote_norm.len())
        .position(|window| window.iter().map(|(ch, _)| *ch).eq(quote_norm.iter().copied()))?;
    let char_start = source_norm[start].1;
    let char_end = source_norm[start + quote_norm.len() - 1].1 + 1;
    Some((char_start, char_end))
}

fn normalize_match_char(ch: char) -> Option<char> {
    if ch.is_ascii_alphanumeric() {
        Some(ch.to_ascii_lowercase())
    } else {
        None
    }
}

fn normalize_alias(alias: &str) -> String {
    let lower = alias.to_ascii_lowercase();
    let stripped = lower
        .split_whitespace()
        .filter(|part| !matches!(*part, "mr" | "mrs" | "ms" | "miss" | "dr" | "prof" | "the"))
        .collect::<Vec<_>>()
        .join(" ");
    stripped
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("_")
}

/// Emit an init span; called by binaries that want to confirm the crate loaded.
pub fn init() {
    tracing::trace!(crate_name = "aco-storage", version = env!("CARGO_PKG_VERSION"), "loaded");
}

#[cfg(test)]
mod tests {
    use super::{remember_claim_ref, resolve_claim_ref, resolve_quote_span};
    use std::collections::HashMap;

    #[test]
    fn resolves_exact_quote_span() {
        assert_eq!(resolve_quote_span("Sam said yes.", "said yes"), Some((4, 12)));
    }

    #[test]
    fn resolves_punctuation_and_case_drift() {
        let source = "Patel said: \"I postponed, not cancelled, her review.\"";
        assert_eq!(
            resolve_quote_span(source, "i postponed not cancelled her review"),
            Some((13, 51))
        );
    }

    #[test]
    fn resolves_model_claim_text_to_persisted_claim_id() {
        let mut lookup = HashMap::new();
        remember_claim_ref(&mut lookup, "claim_1", "claim_claim_1");
        remember_claim_ref(&mut lookup, "Alex agreed to own the deck.", "claim_claim_1");

        assert_eq!(
            resolve_claim_ref(&lookup, "alex agreed to own the deck."),
            Some("claim_claim_1".to_string())
        );
        assert_eq!(resolve_claim_ref(&lookup, "unknown claim"), None);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub model: String,
    pub friction_score: i32,
    pub n_actors: i32,
    pub n_claims: i32,
    pub n_events: i32,
    pub n_patterns: i32,
    pub n_contradictions: i32,
    pub summary: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub elapsed_ms: i64,
}
