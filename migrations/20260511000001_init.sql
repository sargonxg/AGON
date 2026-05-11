CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS sessions (
    id                UUID PRIMARY KEY,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    model             TEXT NOT NULL,
    input_tokens      INTEGER NOT NULL DEFAULT 0,
    output_tokens     INTEGER NOT NULL DEFAULT 0,
    elapsed_ms        BIGINT  NOT NULL DEFAULT 0,
    source_text       TEXT NOT NULL,
    friction_score    INTEGER NOT NULL DEFAULT 0,
    n_actors          INTEGER NOT NULL DEFAULT 0,
    n_claims          INTEGER NOT NULL DEFAULT 0,
    n_events          INTEGER NOT NULL DEFAULT 0,
    n_patterns        INTEGER NOT NULL DEFAULT 0,
    n_contradictions  INTEGER NOT NULL DEFAULT 0,
    extraction        JSONB NOT NULL,
    summary           TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS sessions_created_at_idx ON sessions (created_at DESC);
CREATE INDEX IF NOT EXISTS sessions_friction_idx   ON sessions (friction_score DESC);
