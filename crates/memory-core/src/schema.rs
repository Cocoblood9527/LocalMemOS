pub const FACTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS facts (
    id TEXT PRIMARY KEY,
    namespace TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    entity TEXT NOT NULL,
    attribute TEXT NOT NULL,
    value_json TEXT NOT NULL,
    value_text TEXT,
    confidence REAL,
    status TEXT NOT NULL,
    valid_from TEXT NOT NULL,
    valid_to TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
"#;

pub const FACT_VERSIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS fact_versions (
    id TEXT PRIMARY KEY,
    fact_id TEXT NOT NULL,
    namespace TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    entity TEXT NOT NULL,
    attribute TEXT NOT NULL,
    value_json TEXT NOT NULL,
    value_text TEXT,
    confidence REAL,
    valid_from TEXT NOT NULL,
    valid_to TEXT,
    created_at TEXT NOT NULL
);
"#;

pub const EVIDENCE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS evidence (
    id TEXT PRIMARY KEY,
    fact_id TEXT,
    fact_version_id TEXT,
    source_kind TEXT NOT NULL,
    source_ref TEXT,
    summary TEXT,
    created_at TEXT NOT NULL
);
"#;

pub const FACTS_FTS: &str = r#"
CREATE VIRTUAL TABLE IF NOT EXISTS facts_fts USING fts5(
    fact_id UNINDEXED,
    namespace,
    scope_id,
    entity,
    attribute,
    value_text,
    evidence_summary
);
"#;

pub const ALL_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS facts (
    id TEXT PRIMARY KEY,
    namespace TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    entity TEXT NOT NULL,
    attribute TEXT NOT NULL,
    value_json TEXT NOT NULL,
    value_text TEXT,
    confidence REAL,
    status TEXT NOT NULL,
    valid_from TEXT NOT NULL,
    valid_to TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fact_versions (
    id TEXT PRIMARY KEY,
    fact_id TEXT NOT NULL,
    namespace TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    entity TEXT NOT NULL,
    attribute TEXT NOT NULL,
    value_json TEXT NOT NULL,
    value_text TEXT,
    confidence REAL,
    valid_from TEXT NOT NULL,
    valid_to TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS evidence (
    id TEXT PRIMARY KEY,
    fact_id TEXT,
    fact_version_id TEXT,
    source_kind TEXT NOT NULL,
    source_ref TEXT,
    summary TEXT,
    created_at TEXT NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS facts_fts USING fts5(
    fact_id UNINDEXED,
    namespace,
    scope_id,
    entity,
    attribute,
    value_text,
    evidence_summary
);
"#;
