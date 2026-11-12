//! Runs only against an explicitly configured disposable database.
use aco_storage::{Session, Store};
use chrono::Utc;
use sqlx_core::{migrate::Migrator, query::query, row::Row};
use sqlx_postgres::PgPoolOptions;
use uuid::Uuid;

#[tokio::test]
#[ignore = "requires a disposable PostgreSQL database in AGON_TEST_DATABASE_URL"]
async fn legacy_migrations_and_unicode_history_survive_embedded_replay() {
    let dsn = std::env::var("AGON_TEST_DATABASE_URL").expect("Set only a disposable database URL");
    let pool = PgPoolOptions::new().max_connections(2).connect(&dsn).await.unwrap();
    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    Migrator::new(folder.as_path()).await.unwrap().run(&pool).await.unwrap();
    let before = query("SELECT version, checksum FROM _sqlx_migrations ORDER BY version")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|r| (r.get::<i64, _>("version"), r.get::<Vec<u8>, _>("checksum")))
        .collect::<Vec<_>>();
    assert_eq!(before.len(), 3);
    let store = Store::connect(&dsn).await.unwrap();
    let old = Session {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        model: "migration-fixture-no-model-call".into(),
        input_tokens: 0,
        output_tokens: 0,
        elapsed_ms: 0,
        source_text: "A 🧭 minister said: only if approved. Łódź remains qualified.".into(),
        friction_score: 0,
        n_actors: 0,
        n_claims: 0,
        n_events: 0,
        n_patterns: 0,
        n_contradictions: 0,
        extraction: serde_json::json!({}),
        summary: "Historical fixture".into(),
    };
    store.insert_session(&old).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    let restored = store.get_session(old.id).await.unwrap().unwrap();
    assert_eq!(restored.source_text, old.source_text);
    assert_eq!(restored.summary, old.summary);
    assert_eq!(restored.extraction, old.extraction);
    let after = query("SELECT version, checksum FROM _sqlx_migrations ORDER BY version")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|r| (r.get::<i64, _>("version"), r.get::<Vec<u8>, _>("checksum")))
        .collect::<Vec<_>>();
    assert_eq!(before, after, "Migration versions/checksums must remain identical");
    pool.close().await;
}
