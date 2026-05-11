//! `aco-storage` — Cloud SQL Postgres persistence for AGON.
//!
//! Minimal v0.1: stores each perception as a `session` row with the source
//! text, raw extraction JSON, friction score, and counts. Full per-primitive
//! schema (with pgvector + alias graph + audit log) lands in v0.2.
#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
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
    pub extraction: serde_json::Value,
    pub summary: String,
}

impl Store {
    /// Connect using a libpq-style DSN.
    pub async fn connect(dsn: &str) -> Result<Self, StoreError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
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
        sqlx::query(
            r#"
            INSERT INTO sessions
                (id, created_at, model, input_tokens, output_tokens, elapsed_ms,
                 source_text, friction_score, n_actors, n_claims, n_events,
                 n_patterns, n_contradictions, extraction, summary)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
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
        .execute(&self.pool)
        .await?;
        Ok(())
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

        Ok(rows.into_iter().map(|r| SessionSummary {
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
        }).collect())
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
