CREATE TABLE IF NOT EXISTS documents (
    id            TEXT PRIMARY KEY,
    session_id    UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    source_hash   TEXT NOT NULL,
    title         TEXT NOT NULL DEFAULT 'perception input',
    source_kind   TEXT NOT NULL DEFAULT 'text',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    raw_metadata  JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS documents_session_source_hash_idx
    ON documents (session_id, source_hash);

CREATE TABLE IF NOT EXISTS chunks (
    id          TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    ordinal     INTEGER NOT NULL,
    text        TEXT NOT NULL,
    char_start  INTEGER NOT NULL DEFAULT 0,
    char_end    INTEGER NOT NULL,
    source_hash TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS chunks_document_ordinal_idx
    ON chunks (document_id, ordinal);

CREATE TABLE IF NOT EXISTS actors (
    id              TEXT PRIMARY KEY,
    session_id      UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    canonical_name  TEXT NOT NULL,
    kind            TEXT NOT NULL DEFAULT 'unknown',
    evidence_span_id TEXT,
    raw_json        JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS actor_aliases (
    actor_id          TEXT NOT NULL REFERENCES actors(id) ON DELETE CASCADE,
    alias             TEXT NOT NULL,
    normalized_alias  TEXT NOT NULL,
    source            TEXT NOT NULL DEFAULT 'model',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (actor_id, normalized_alias)
);

CREATE TABLE IF NOT EXISTS evidence_spans (
    id              TEXT PRIMARY KEY,
    session_id      UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    document_id     TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    chunk_id        TEXT NOT NULL REFERENCES chunks(id) ON DELETE CASCADE,
    primitive_kind  TEXT NOT NULL,
    primitive_id    TEXT NOT NULL,
    quote           TEXT NOT NULL,
    char_start      INTEGER,
    char_end        INTEGER,
    span_status     TEXT NOT NULL CHECK (span_status IN ('verified', 'unresolved')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS evidence_spans_primitive_idx
    ON evidence_spans (primitive_kind, primitive_id);

ALTER TABLE actors
    ADD CONSTRAINT actors_evidence_span_id_fkey
    FOREIGN KEY (evidence_span_id) REFERENCES evidence_spans(id);

CREATE TABLE IF NOT EXISTS claims (
    id               TEXT PRIMARY KEY,
    session_id       UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    actor_id         TEXT REFERENCES actors(id) ON DELETE SET NULL,
    text             TEXT NOT NULL,
    polarity         TEXT NOT NULL DEFAULT 'ambiguous',
    evidence_span_id TEXT NOT NULL REFERENCES evidence_spans(id),
    raw_json         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS events (
    id               TEXT PRIMARY KEY,
    session_id       UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    label            TEXT NOT NULL,
    event_time       TEXT,
    evidence_span_id TEXT NOT NULL REFERENCES evidence_spans(id),
    raw_json         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS commitments (
    id               TEXT PRIMARY KEY,
    session_id       UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    by_actor_id      TEXT REFERENCES actors(id) ON DELETE SET NULL,
    subject          TEXT NOT NULL,
    deadline         TEXT,
    status           TEXT NOT NULL DEFAULT 'proposed',
    evidence_span_id TEXT NOT NULL REFERENCES evidence_spans(id),
    raw_json         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS patterns (
    id               TEXT PRIMARY KEY,
    session_id       UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    kind             TEXT NOT NULL,
    actor_id         TEXT REFERENCES actors(id) ON DELETE SET NULL,
    confidence       DOUBLE PRECISION NOT NULL DEFAULT 0,
    evidence_span_id TEXT NOT NULL REFERENCES evidence_spans(id),
    raw_json         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS contradictions (
    id           TEXT PRIMARY KEY,
    session_id   UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    claim_a      TEXT NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
    claim_b      TEXT NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
    materiality  TEXT NOT NULL DEFAULT 'cosmetic',
    source       TEXT NOT NULL DEFAULT 'model_suggested',
    confidence   DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    rationale    TEXT NOT NULL DEFAULT '',
    raw_json     JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS graph_edges (
    id          TEXT PRIMARY KEY,
    session_id  UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    source_id   TEXT NOT NULL,
    target_id   TEXT NOT NULL,
    edge_type   TEXT NOT NULL,
    weight      DOUBLE PRECISION NOT NULL DEFAULT 1,
    raw_json    JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS graph_edges_session_type_idx
    ON graph_edges (session_id, edge_type);
