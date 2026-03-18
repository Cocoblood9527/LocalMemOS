# Local Agent Memory Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a local-first, Rust-backed fact memory kernel with SQLite storage, time-aware history, thin Python and Node SDKs, and matching HTTP and MCP adapters.

**Architecture:** Use a Rust workspace as the source of truth. Keep `memory-core` responsible for schema, validation, transactions, and retrieval semantics. Add thin adapters for HTTP, Python, Node, and MCP only after the core write and recall behavior is stable and fully tested.

**Tech Stack:** Rust, Cargo workspace, SQLite, `rusqlite`, `serde`, `thiserror`, `axum`, `tokio`, `pyo3` + `maturin`, `napi-rs`, TypeScript, `@modelcontextprotocol/sdk`, `pytest`, Vitest

---

## Preconditions

- This workspace is not currently a Git repository. Initialize one before execution if you want to follow the commit steps exactly.
- The implementation should follow the approved spec at `docs/superpowers/specs/2026-03-18-local-agent-memory-design.md`.
- Do not add automatic extraction, embeddings, vector storage, graph storage, or cloud sync during `v1`.

## Proposed File Structure

### Root

- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.gitignore`
- Create: `README.md`

### Rust Core

- Create: `crates/memory-core/Cargo.toml`
- Create: `crates/memory-core/src/lib.rs`
- Create: `crates/memory-core/src/error.rs`
- Create: `crates/memory-core/src/model.rs`
- Create: `crates/memory-core/src/request.rs`
- Create: `crates/memory-core/src/schema.rs`
- Create: `crates/memory-core/src/store.rs`
- Create: `crates/memory-core/src/fts.rs`
- Create: `crates/memory-core/tests/upsert_fact.rs`
- Create: `crates/memory-core/tests/recall_fact.rs`
- Create: `crates/memory-core/tests/history_fact.rs`
- Create: `crates/memory-core/tests/namespace_isolation.rs`
- Create: `crates/memory-core/tests/fts_fallback.rs`

### HTTP Adapter

- Create: `crates/memory-http/Cargo.toml`
- Create: `crates/memory-http/src/main.rs`
- Create: `crates/memory-http/src/routes.rs`
- Create: `crates/memory-http/src/state.rs`
- Create: `crates/memory-http/tests/http_smoke.rs`

### Python SDK

- Create: `python/pyproject.toml`
- Create: `python/memory_sdk/__init__.py`
- Create: `python/memory_sdk/client.py`
- Create: `python/memory_sdk/models.py`
- Create: `python/tests/test_sdk_core.py`

### Node SDK

- Create: `crates/memory-node/Cargo.toml`
- Create: `crates/memory-node/src/lib.rs`
- Create: `packages/node/package.json`
- Create: `packages/node/tsconfig.json`
- Create: `packages/node/src/index.ts`
- Create: `packages/node/src/models.ts`
- Create: `packages/node/test/sdk.test.ts`

### MCP Adapter

- Create: `packages/mcp/package.json`
- Create: `packages/mcp/tsconfig.json`
- Create: `packages/mcp/src/server.ts`
- Create: `packages/mcp/src/tools.ts`
- Create: `packages/mcp/test/tools.test.ts`

## Milestones

1. Foundation and workspace bootstrap
2. Core schema, validation, and write path
3. Core recall, history, forget, and FTS fallback
4. HTTP adapter
5. Python and Node SDKs
6. MCP adapter
7. Cross-adapter verification and docs polish

## Task 1: Bootstrap the Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.gitignore`
- Create: `README.md`
- Create: `crates/memory-core/Cargo.toml`
- Create: `crates/memory-http/Cargo.toml`
- Create: `crates/memory-node/Cargo.toml`
- Create: `python/pyproject.toml`
- Create: `packages/node/package.json`
- Create: `packages/mcp/package.json`

- [ ] **Step 1: Write the failing smoke test for workspace presence**

```rust
// crates/memory-core/tests/upsert_fact.rs
#[test]
fn workspace_bootstrapped() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(manifest.exists());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-core workspace_bootstrapped -- --exact`
Expected: FAIL because the workspace and package manifests do not exist yet

- [ ] **Step 3: Create the minimal workspace files**

```toml
# Cargo.toml
[workspace]
members = ["crates/memory-core", "crates/memory-http", "crates/memory-node"]
resolver = "2"
```

```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

```gitignore
/target
/.venv
/node_modules
/dist
/python/.maturin
```

- [ ] **Step 4: Add package manifests with no implementation logic**

```toml
# crates/memory-core/Cargo.toml
[package]
name = "memory-core"
version = "0.1.0"
edition = "2021"

[dependencies]
rusqlite = { version = "0.31", features = ["bundled", "chrono"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
```

```toml
# crates/memory-http/Cargo.toml
[package]
name = "memory-http"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
memory-core = { path = "../memory-core" }
```

```toml
# crates/memory-node/Cargo.toml
[package]
name = "memory-node"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
napi = "2"
napi-derive = "2"
memory-core = { path = "../memory-core" }
```

```toml
# python/pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "memory-sdk"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = []
```

```json
// packages/node/package.json
{
  "name": "@localmemos/node",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "build": "tsc -p tsconfig.json",
    "test": "vitest run"
  },
  "devDependencies": {
    "typescript": "^5.6.0",
    "vitest": "^2.1.0"
  }
}
```

```json
// packages/mcp/package.json
{
  "name": "@localmemos/mcp",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "build": "tsc -p tsconfig.json",
    "test": "vitest run"
  },
  "dependencies": {
    "@modelcontextprotocol/sdk": "^1.0.0"
  },
  "devDependencies": {
    "typescript": "^5.6.0",
    "vitest": "^2.1.0"
  }
}
```

- [ ] **Step 5: Run test to verify the bootstrap passes**

Run: `cargo test -p memory-core workspace_bootstrapped -- --exact`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git init
git add Cargo.toml rust-toolchain.toml .gitignore README.md crates python packages
git commit -m "chore: bootstrap workspace"
```

## Task 2: Define Core Types and Validation

**Files:**
- Create: `crates/memory-core/src/lib.rs`
- Create: `crates/memory-core/src/error.rs`
- Create: `crates/memory-core/src/model.rs`
- Create: `crates/memory-core/src/request.rs`
- Test: `crates/memory-core/tests/upsert_fact.rs`

- [ ] **Step 1: Write the failing validation test**

```rust
use memory_core::{UpsertFactRequest, ValidationError};

#[test]
fn rejects_missing_namespace() {
    let req = UpsertFactRequest {
        namespace: "".into(),
        scope_id: "localmemos".into(),
        entity: "project".into(),
        attribute: "preferred_package_manager".into(),
        value: serde_json::json!("bun"),
        confidence: Some(0.95),
        tags: vec![],
        valid_from: None,
        source_kind: "manual".into(),
        source_ref: Some("user-confirmed".into()),
        evidence_summary: None,
    };

    let err = req.validate().unwrap_err();
    assert!(matches!(err, ValidationError::MissingField("namespace")));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-core rejects_missing_namespace -- --exact`
Expected: FAIL because `UpsertFactRequest` and `ValidationError` do not exist yet

- [ ] **Step 3: Add the error and request types**

```rust
// crates/memory-core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("invalid confidence")]
    InvalidConfidence,
}
```

```rust
// crates/memory-core/src/request.rs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpsertFactRequest {
    pub namespace: String,
    pub scope_id: String,
    pub entity: String,
    pub attribute: String,
    pub value: serde_json::Value,
    pub confidence: Option<f32>,
    pub tags: Vec<String>,
    pub valid_from: Option<chrono::DateTime<chrono::Utc>>,
    pub source_kind: String,
    pub source_ref: Option<String>,
    pub evidence_summary: Option<String>,
}
```

- [ ] **Step 4: Implement `validate()` minimally**

```rust
impl UpsertFactRequest {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.namespace.trim().is_empty() {
            return Err(ValidationError::MissingField("namespace"));
        }
        if self.scope_id.trim().is_empty() {
            return Err(ValidationError::MissingField("scope_id"));
        }
        if self.entity.trim().is_empty() {
            return Err(ValidationError::MissingField("entity"));
        }
        if self.attribute.trim().is_empty() {
            return Err(ValidationError::MissingField("attribute"));
        }
        if self.source_kind.trim().is_empty() {
            return Err(ValidationError::MissingField("source_kind"));
        }
        if let Some(confidence) = self.confidence {
            if !(0.0..=1.0).contains(&confidence) {
                return Err(ValidationError::InvalidConfidence);
            }
        }
        Ok(())
    }
}
```

- [ ] **Step 5: Export the types from `lib.rs`**

```rust
pub mod error;
pub mod model;
pub mod request;

pub use error::ValidationError;
pub use request::UpsertFactRequest;
```

- [ ] **Step 6: Run the validation test**

Run: `cargo test -p memory-core rejects_missing_namespace -- --exact`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/memory-core/src crates/memory-core/tests/upsert_fact.rs
git commit -m "feat: add core request validation"
```

## Task 3: Add SQLite Schema Bootstrap and Test Harness

**Files:**
- Create: `crates/memory-core/src/schema.rs`
- Create: `crates/memory-core/src/store.rs`
- Test: `crates/memory-core/tests/upsert_fact.rs`
- Test: `crates/memory-core/tests/history_fact.rs`

- [ ] **Step 1: Write the failing schema bootstrap test**

```rust
use memory_core::MemoryStore;

#[test]
fn initializes_schema_in_memory_sqlite() {
    let store = MemoryStore::open(":memory:").unwrap();
    let count = store.table_count().unwrap();
    assert!(count >= 3);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-core initializes_schema_in_memory_sqlite -- --exact`
Expected: FAIL because `MemoryStore` does not exist yet

- [ ] **Step 3: Add schema DDL constants**

```rust
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
```

- [ ] **Step 4: Add the store bootstrap**

```rust
pub struct MemoryStore {
    conn: rusqlite::Connection,
}

impl MemoryStore {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = rusqlite::Connection::open(path)?;
        conn.execute_batch(crate::schema::ALL_SCHEMA)?;
        Ok(Self { conn })
    }
}
```

- [ ] **Step 5: Add a test helper method**

```rust
impl MemoryStore {
    pub fn table_count(&self) -> rusqlite::Result<i64> {
        self.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
    }
}
```

- [ ] **Step 6: Run the schema bootstrap test**

Run: `cargo test -p memory-core initializes_schema_in_memory_sqlite -- --exact`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/memory-core/src/schema.rs crates/memory-core/src/store.rs crates/memory-core/tests
git commit -m "feat: add sqlite schema bootstrap"
```

## Task 4: Implement `upsert_fact` with Version History

**Files:**
- Modify: `crates/memory-core/src/model.rs`
- Modify: `crates/memory-core/src/store.rs`
- Test: `crates/memory-core/tests/upsert_fact.rs`
- Test: `crates/memory-core/tests/history_fact.rs`

- [ ] **Step 1: Write the failing upsert replacement test**

```rust
use memory_core::{MemoryStore, UpsertFactRequest};

#[test]
fn upsert_replaces_current_value_and_keeps_history() {
    let mut store = MemoryStore::open(":memory:").unwrap();

    let first = UpsertFactRequest::manual("workspace", "localmemos", "project", "preferred_package_manager", "npm");
    store.upsert_fact(first).unwrap();

    let second = UpsertFactRequest::manual("workspace", "localmemos", "project", "preferred_package_manager", "bun");
    store.upsert_fact(second).unwrap();

    let current = store.get_fact("workspace", "localmemos", "project", "preferred_package_manager").unwrap().unwrap();
    assert_eq!(current.value_text.as_deref(), Some("bun"));

    let history = store.history("workspace", "localmemos", "project", "preferred_package_manager").unwrap();
    assert_eq!(history.len(), 2);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-core upsert_replaces_current_value_and_keeps_history -- --exact`
Expected: FAIL because store read and history methods are not implemented yet

- [ ] **Step 3: Add the `FactRecord` and `FactVersionRecord` models**

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FactRecord {
    pub id: String,
    pub namespace: String,
    pub scope_id: String,
    pub entity: String,
    pub attribute: String,
    pub value_json: serde_json::Value,
    pub value_text: Option<String>,
    pub confidence: Option<f32>,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

- [ ] **Step 4: Implement `upsert_fact` in a transaction**

```rust
pub fn upsert_fact(&mut self, req: UpsertFactRequest) -> Result<FactRecord, MemoryError> {
    req.validate()?;
    let tx = self.conn.transaction()?;
    let now = chrono::Utc::now();

    let existing = find_current_fact(&tx, &req)?;
    if let Some(current) = existing {
        if current.value_json == req.value {
            append_evidence(&tx, &current.id, &req, now)?;
            tx.commit()?;
            return Ok(current);
        }
        close_current_fact(&tx, &current.id, now)?;
    }

    let inserted = insert_new_fact(&tx, &req, now)?;
    insert_fact_version(&tx, &inserted, &req, now)?;
    append_evidence(&tx, &inserted.id, &req, now)?;
    tx.commit()?;
    Ok(inserted)
}
```

- [ ] **Step 5: Add a convenience constructor for manual writes**

```rust
impl UpsertFactRequest {
    pub fn manual(namespace: &str, scope_id: &str, entity: &str, attribute: &str, value: &str) -> Self {
        Self {
            namespace: namespace.into(),
            scope_id: scope_id.into(),
            entity: entity.into(),
            attribute: attribute.into(),
            value: serde_json::json!(value),
            confidence: Some(1.0),
            tags: vec![],
            valid_from: None,
            source_kind: "manual".into(),
            source_ref: Some("test".into()),
            evidence_summary: None,
        }
    }
}
```

- [ ] **Step 6: Run the upsert and history tests**

Run: `cargo test -p memory-core upsert_replaces_current_value_and_keeps_history -- --exact`
Expected: PASS

Run: `cargo test -p memory-core --test history_fact`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/memory-core/src crates/memory-core/tests/upsert_fact.rs crates/memory-core/tests/history_fact.rs
git commit -m "feat: implement fact upsert and version history"
```

## Task 5: Implement `recall`, `list`, and `forget`

**Files:**
- Modify: `crates/memory-core/src/request.rs`
- Modify: `crates/memory-core/src/store.rs`
- Test: `crates/memory-core/tests/recall_fact.rs`
- Test: `crates/memory-core/tests/namespace_isolation.rs`

- [ ] **Step 1: Write the failing recall test**

```rust
use memory_core::{MemoryStore, RecallRequest, UpsertFactRequest};

#[test]
fn recall_returns_only_current_matching_fact() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store.upsert_fact(UpsertFactRequest::manual("workspace", "localmemos", "project", "preferred_package_manager", "bun")).unwrap();

    let result = store.recall(RecallRequest::exact("workspace", "localmemos", "project", "preferred_package_manager")).unwrap();
    assert_eq!(result.facts.len(), 1);
    assert_eq!(result.facts[0].value_text.as_deref(), Some("bun"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-core recall_returns_only_current_matching_fact -- --exact`
Expected: FAIL because `RecallRequest` and `recall()` do not exist yet

- [ ] **Step 3: Add `RecallRequest` and `ListRequest` types**

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecallRequest {
    pub namespace: String,
    pub scope_id: String,
    pub entity: Option<String>,
    pub attribute: Option<String>,
    pub text_query: Option<String>,
    pub include_history: bool,
}
```

- [ ] **Step 4: Implement structured recall and list**

```rust
pub fn recall(&self, req: RecallRequest) -> Result<RecallResponse, MemoryError> {
    if let Some(text_query) = req.text_query.as_deref() {
        return self.recall_with_fts(req.namespace, req.scope_id, text_query);
    }
    self.recall_exact(req)
}

pub fn forget(&mut self, req: ForgetRequest) -> Result<(), MemoryError> {
    let now = chrono::Utc::now();
    self.conn.execute(
        "UPDATE facts SET valid_to = ?, status = 'retired' WHERE namespace = ? AND scope_id = ? AND entity = ? AND attribute = ? AND valid_to IS NULL",
        rusqlite::params![now.to_rfc3339(), req.namespace, req.scope_id, req.entity, req.attribute],
    )?;
    Ok(())
}
```

- [ ] **Step 5: Add namespace isolation coverage**

```rust
#[test]
fn workspace_recall_does_not_return_user_facts() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store.upsert_fact(UpsertFactRequest::manual("user", "default", "preference", "language", "zh-CN")).unwrap();
    store.upsert_fact(UpsertFactRequest::manual("workspace", "localmemos", "project", "language", "English docs")).unwrap();

    let result = store.recall(RecallRequest::exact("workspace", "localmemos", "project", "language")).unwrap();
    assert_eq!(result.facts.len(), 1);
    assert_eq!(result.facts[0].namespace, "workspace");
}
```

- [ ] **Step 6: Run recall and namespace tests**

Run: `cargo test -p memory-core --test recall_fact`
Expected: PASS

Run: `cargo test -p memory-core --test namespace_isolation`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/memory-core/src crates/memory-core/tests/recall_fact.rs crates/memory-core/tests/namespace_isolation.rs
git commit -m "feat: add structured recall and forget"
```

## Task 6: Add FTS Fallback for Text Recall

**Files:**
- Create: `crates/memory-core/src/fts.rs`
- Modify: `crates/memory-core/src/schema.rs`
- Modify: `crates/memory-core/src/store.rs`
- Test: `crates/memory-core/tests/fts_fallback.rs`

- [ ] **Step 1: Write the failing FTS fallback test**

```rust
use memory_core::{MemoryStore, RecallRequest, UpsertFactRequest};

#[test]
fn text_query_falls_back_to_fts() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store.upsert_fact(UpsertFactRequest::manual("workspace", "localmemos", "project", "summary", "Uses bun and Rust core")).unwrap();

    let result = store.recall(RecallRequest::text("workspace", "localmemos", "Rust core")).unwrap();
    assert_eq!(result.facts.len(), 1);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-core text_query_falls_back_to_fts -- --exact`
Expected: FAIL because the FTS table and fallback query do not exist yet

- [ ] **Step 3: Add FTS schema**

```rust
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
```

- [ ] **Step 4: Update write flow to maintain the FTS index**

```rust
fn upsert_fts_row(
    tx: &rusqlite::Transaction<'_>,
    fact_id: &str,
    namespace: &str,
    scope_id: &str,
    entity: &str,
    attribute: &str,
    value_text: Option<&str>,
    evidence_summary: Option<&str>,
) -> rusqlite::Result<()> {
    tx.execute(
        "INSERT INTO facts_fts (fact_id, namespace, scope_id, entity, attribute, value_text, evidence_summary) VALUES (?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![fact_id, namespace, scope_id, entity, attribute, value_text, evidence_summary],
    )?;
    Ok(())
}
```

- [ ] **Step 5: Implement `RecallRequest::text()` and the fallback query**

```rust
pub fn text(namespace: &str, scope_id: &str, text_query: &str) -> Self {
    Self {
        namespace: namespace.into(),
        scope_id: scope_id.into(),
        entity: None,
        attribute: None,
        text_query: Some(text_query.into()),
        include_history: false,
    }
}
```

- [ ] **Step 6: Run the FTS test**

Run: `cargo test -p memory-core --test fts_fallback`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/memory-core/src crates/memory-core/tests/fts_fallback.rs
git commit -m "feat: add fts fallback recall"
```

## Task 7: Add the HTTP Adapter

**Files:**
- Create: `crates/memory-http/src/main.rs`
- Create: `crates/memory-http/src/routes.rs`
- Create: `crates/memory-http/src/state.rs`
- Test: `crates/memory-http/tests/http_smoke.rs`

- [ ] **Step 1: Write the failing HTTP smoke test**

```rust
#[tokio::test]
async fn upsert_and_recall_work_over_http() {
    let app = memory_http::app_for_test(":memory:").await;
    let response = app.oneshot(/* upsert request */).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p memory-http upsert_and_recall_work_over_http -- --exact`
Expected: FAIL because the HTTP app and routes do not exist yet

- [ ] **Step 3: Add the application state**

```rust
#[derive(Clone)]
pub struct AppState {
    pub store_path: String,
}
```

- [ ] **Step 4: Implement the primitive routes**

```rust
let app = axum::Router::new()
    .route("/facts:upsert", axum::routing::post(routes::upsert_fact))
    .route("/facts:recall", axum::routing::post(routes::recall))
    .route("/facts:list", axum::routing::post(routes::list))
    .route("/facts:forget", axum::routing::post(routes::forget))
    .route("/facts/:id/history", axum::routing::get(routes::history))
    .with_state(state);
```

- [ ] **Step 5: Return structured JSON errors**

```rust
#[derive(serde::Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}
```

- [ ] **Step 6: Run the HTTP smoke test**

Run: `cargo test -p memory-http --test http_smoke`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/memory-http
git commit -m "feat: add http memory adapter"
```

## Task 8: Add the Python SDK

**Files:**
- Create: `python/memory_sdk/__init__.py`
- Create: `python/memory_sdk/client.py`
- Create: `python/memory_sdk/models.py`
- Test: `python/tests/test_sdk_core.py`

- [ ] **Step 1: Write the failing Python SDK test**

```python
from memory_sdk.client import MemoryClient


def test_python_sdk_upsert_and_recall(tmp_path):
    client = MemoryClient(str(tmp_path / "memory.db"))
    client.upsert_fact(
        namespace="workspace",
        scope_id="localmemos",
        entity="project",
        attribute="preferred_package_manager",
        value="bun",
        source_kind="manual",
    )
    result = client.recall(
        namespace="workspace",
        scope_id="localmemos",
        entity="project",
        attribute="preferred_package_manager",
    )
    assert result["facts"][0]["value_text"] == "bun"
```

- [ ] **Step 2: Run test to verify it fails**

Run: `pytest python/tests/test_sdk_core.py -k python_sdk_upsert_and_recall -q`
Expected: FAIL because the SDK package does not exist yet

- [ ] **Step 3: Add the Python package shell**

```python
# python/memory_sdk/__init__.py
from .client import MemoryClient

__all__ = ["MemoryClient"]
```

- [ ] **Step 4: Implement a thin client API over the Rust binding**

```python
class MemoryClient:
    def __init__(self, path: str) -> None:
        self._path = path
        self._core = _memory_sdk.NativeMemoryClient(path)

    def upsert_fact(self, **kwargs):
        return self._core.upsert_fact(kwargs)

    def recall(self, **kwargs):
        return self._core.recall(kwargs)
```

- [ ] **Step 5: Build and run the SDK test**

Run: `maturin develop --manifest-path python/pyproject.toml`
Expected: builds the Python extension successfully

Run: `pytest python/tests/test_sdk_core.py -q`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add python crates/memory-core
git commit -m "feat: add python sdk"
```

## Task 9: Add the Node SDK

**Files:**
- Create: `crates/memory-node/src/lib.rs`
- Create: `packages/node/src/index.ts`
- Create: `packages/node/src/models.ts`
- Test: `packages/node/test/sdk.test.ts`

- [ ] **Step 1: Write the failing Node SDK test**

```ts
import { MemoryClient } from "../src/index";

test("node sdk upsert and recall", async () => {
  const client = new MemoryClient(":memory:");
  await client.upsertFact({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
    value: "bun",
    sourceKind: "manual",
  });

  const result = await client.recall({
    namespace: "workspace",
    scopeId: "localmemos",
    entity: "project",
    attribute: "preferred_package_manager",
  });

  expect(result.facts[0].valueText).toBe("bun");
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `pnpm --dir packages/node test`
Expected: FAIL because the native addon and TypeScript package do not exist yet

- [ ] **Step 3: Expose Rust functions through `napi-rs`**

```rust
#[napi]
pub struct NativeMemoryClient {
    inner: std::sync::Mutex<memory_core::MemoryStore>,
}

#[napi]
impl NativeMemoryClient {
    #[napi(constructor)]
    pub fn new(path: String) -> napi::Result<Self> {
        let store = memory_core::MemoryStore::open(&path).map_err(to_napi_error)?;
        Ok(Self { inner: std::sync::Mutex::new(store) })
    }
}
```

- [ ] **Step 4: Add the TypeScript wrapper**

```ts
export class MemoryClient {
  private readonly native: NativeMemoryClient;

  constructor(path: string) {
    this.native = new NativeMemoryClient(path);
  }

  upsertFact(input: UpsertFactInput) {
    return this.native.upsertFact(input);
  }

  recall(input: RecallInput) {
    return this.native.recall(input);
  }
}
```

- [ ] **Step 5: Build and run the Node SDK test**

Run: `pnpm --dir packages/node install`
Expected: installs dependencies successfully

Run: `pnpm --dir packages/node test`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/memory-node packages/node
git commit -m "feat: add node sdk"
```

## Task 10: Add the MCP Adapter

**Files:**
- Create: `packages/mcp/src/server.ts`
- Create: `packages/mcp/src/tools.ts`
- Test: `packages/mcp/test/tools.test.ts`

- [ ] **Step 1: Write the failing MCP tool registration test**

```ts
import { createServer } from "../src/server";

test("registers all v1 memory tools", async () => {
  const server = await createServer(":memory:");
  const tools = server.listTools();
  expect(tools.map((tool) => tool.name)).toEqual([
    "memory_upsert_fact",
    "memory_recall",
    "memory_list",
    "memory_forget",
    "memory_history",
  ]);
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `pnpm --dir packages/mcp test`
Expected: FAIL because the MCP server package does not exist yet

- [ ] **Step 3: Implement tool registration using the Node SDK**

```ts
export async function createServer(path: string) {
  const client = new MemoryClient(path);
  const server = new McpServer({ name: "local-agent-memory", version: "0.1.0" });
  registerMemoryTools(server, client);
  return server;
}
```

- [ ] **Step 4: Implement structured tool handlers**

```ts
server.tool("memory_upsert_fact", schema, async (input) => {
  return {
    content: [
      {
        type: "text",
        text: JSON.stringify(await client.upsertFact(input)),
      },
    ],
  };
});
```

- [ ] **Step 5: Run the MCP tests**

Run: `pnpm --dir packages/mcp install`
Expected: installs dependencies successfully

Run: `pnpm --dir packages/mcp test`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add packages/mcp
git commit -m "feat: add mcp memory adapter"
```

## Task 11: Verify Cross-Adapter Consistency

**Files:**
- Modify: `README.md`
- Create: `crates/memory-http/tests/http_smoke.rs`
- Modify: `python/tests/test_sdk_core.py`
- Modify: `packages/node/test/sdk.test.ts`
- Modify: `packages/mcp/test/tools.test.ts`

- [ ] **Step 1: Add one shared fixture scenario**

```text
namespace = workspace
scope_id = localmemos
entity = project
attribute = preferred_package_manager
initial value = npm
updated value = bun
```

- [ ] **Step 2: Assert identical current-value behavior everywhere**

```text
Expected current value: bun
Expected history length: 2
Expected namespace: workspace
Expected no leakage into user/default
```

- [ ] **Step 3: Run the Rust test suites**

Run: `cargo test -p memory-core`
Expected: PASS

Run: `cargo test -p memory-http`
Expected: PASS

- [ ] **Step 4: Run the Python and Node suites**

Run: `pytest python/tests -q`
Expected: PASS

Run: `pnpm --dir packages/node test`
Expected: PASS

Run: `pnpm --dir packages/mcp test`
Expected: PASS

- [ ] **Step 5: Update the README with execution paths**

```md
## Runtime Modes

- Embedded mode via Python SDK
- Embedded mode via Node SDK
- Local service mode via HTTP
- Tool integration via MCP
```

- [ ] **Step 6: Commit**

```bash
git add README.md crates python packages
git commit -m "docs: document adapters and verify consistency"
```

## Acceptance Checklist

- [ ] `memory-core` owns all storage and retrieval semantics
- [ ] SQLite schema supports current facts, versions, and evidence
- [ ] `upsert_fact` preserves history correctly
- [ ] `recall` prefers exact and filtered lookup
- [ ] FTS is only a fallback path
- [ ] Python SDK and Node SDK remain thin wrappers
- [ ] HTTP routes map directly to memory primitives
- [ ] MCP exposes only the approved `v1` tools
- [ ] namespace isolation is covered by tests
- [ ] historical lookup is covered by tests
- [ ] cross-adapter behavior is consistent
- [ ] no vector, graph, auto-extraction, or cloud features were introduced

## Suggested Execution Order

1. Finish Tasks 1 through 6 before touching adapters.
2. Do Task 7 next so there is one process-external integration path.
3. Implement Python and Node SDKs after core semantics stabilize.
4. Build the MCP adapter last because it should be a very thin layer over the Node SDK.
5. Run Task 11 only after every adapter has at least one passing smoke test.

## Handoff Notes

- If binding delivery becomes the main blocker, keep the Rust core stable and continue the HTTP adapter first.
- If the implementation worker wants to add new memory domains, stop and compare the change to the approved `v1` non-goals before proceeding.
- If the implementation worker wants to swap SQLite for a heavier backend, reject the change for `v1`.
