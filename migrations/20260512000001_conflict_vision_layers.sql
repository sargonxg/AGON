CREATE TABLE IF NOT EXISTS document_segments (
    id          TEXT PRIMARY KEY,
    session_id  UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    kind        TEXT NOT NULL,
    label       TEXT NOT NULL,
    char_start  INTEGER NOT NULL,
    char_end    INTEGER NOT NULL,
    raw_json    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS document_segments_session_idx
    ON document_segments (session_id, kind);

CREATE TABLE IF NOT EXISTS neural_signals (
    id          TEXT PRIMARY KEY,
    session_id  UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    kind        TEXT NOT NULL,
    source_id   TEXT NOT NULL,
    target_id   TEXT NOT NULL,
    score       DOUBLE PRECISION NOT NULL DEFAULT 0,
    model       TEXT NOT NULL DEFAULT '',
    rationale   TEXT NOT NULL DEFAULT '',
    raw_json    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS neural_signals_session_kind_idx
    ON neural_signals (session_id, kind);

CREATE TABLE IF NOT EXISTS inference_findings (
    id          TEXT PRIMARY KEY,
    session_id  UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    kind        TEXT NOT NULL,
    severity    TEXT NOT NULL DEFAULT 'medium',
    confidence  DOUBLE PRECISION NOT NULL DEFAULT 0,
    source      TEXT NOT NULL DEFAULT 'deterministic',
    rationale   TEXT NOT NULL DEFAULT '',
    raw_json    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS inference_findings_session_kind_idx
    ON inference_findings (session_id, kind);

CREATE TABLE IF NOT EXISTS quality_gates (
    id          TEXT PRIMARY KEY,
    session_id  UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    label       TEXT NOT NULL,
    status      TEXT NOT NULL,
    score       DOUBLE PRECISION NOT NULL DEFAULT 0,
    detail      TEXT NOT NULL DEFAULT '',
    raw_json    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS quality_gates_session_status_idx
    ON quality_gates (session_id, status);
