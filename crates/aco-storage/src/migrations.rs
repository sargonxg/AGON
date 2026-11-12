//! The same immutable SQL is embedded without enabling SQLx's multi-driver macros.
use futures::future::BoxFuture;
use sqlx_core::error::BoxDynError;
use sqlx_core::migrate::{Migration, MigrationSource, MigrationType};
use std::borrow::Cow;

const SOURCES: &[(i64, &str, &str)] = &[
    (20260511000001, "init", include_str!("../../../migrations/20260511000001_init.sql")),
    (
        20260511000002,
        "typed mvp storage",
        include_str!("../../../migrations/20260511000002_typed_mvp_storage.sql"),
    ),
    (
        20260512000001,
        "conflict vision layers",
        include_str!("../../../migrations/20260512000001_conflict_vision_layers.sql"),
    ),
];

#[derive(Debug, Clone, Copy)]
pub(crate) struct EmbeddedMigrations;

impl MigrationSource<'static> for EmbeddedMigrations {
    fn resolve(self) -> BoxFuture<'static, Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async {
            Ok(SOURCES
                .iter()
                .map(|(version, description, sql)| {
                    Migration::new(
                        *version,
                        Cow::Borrowed(*description),
                        MigrationType::Simple,
                        Cow::Borrowed(*sql),
                        sql.starts_with("-- no-transaction"),
                    )
                })
                .collect())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn embedded_migrations_match_sqlx_directory_resolution() {
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
        let original = folder.as_path().resolve().await.unwrap();
        let embedded = EmbeddedMigrations.resolve().await.unwrap();
        assert_eq!(original.len(), embedded.len(), "A new SQL file must be explicitly embedded");
        for (disk, built) in original.iter().zip(&embedded) {
            assert_eq!(disk.version, built.version);
            assert_eq!(disk.description, built.description);
            assert_eq!(disk.migration_type, built.migration_type);
            assert_eq!(disk.sql, built.sql);
            assert_eq!(
                disk.checksum, built.checksum,
                "Already applied migrations must keep their SQLx SHA-384 checksum"
            );
            assert_eq!(disk.no_tx, built.no_tx);
        }
    }
}
