use memory_core::{MemoryStore, UpsertFactRequest, ValidationError};

#[test]
fn workspace_bootstrapped() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(manifest.exists());
}

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

#[test]
fn initializes_schema_in_memory_sqlite() {
    let store = MemoryStore::open(":memory:").unwrap();
    let count = store.table_count().unwrap();
    assert!(count >= 3);
}

#[test]
fn upsert_replaces_current_value_and_keeps_history() {
    let mut store = MemoryStore::open(":memory:").unwrap();

    let first = UpsertFactRequest::manual(
        "workspace",
        "localmemos",
        "project",
        "preferred_package_manager",
        "npm",
    );
    store.upsert_fact(first).unwrap();

    let second = UpsertFactRequest::manual(
        "workspace",
        "localmemos",
        "project",
        "preferred_package_manager",
        "bun",
    );
    store.upsert_fact(second).unwrap();

    let current = store
        .get_fact(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
        )
        .unwrap()
        .unwrap();
    assert_eq!(current.value_text.as_deref(), Some("bun"));

    let history = store
        .history(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
        )
        .unwrap();
    assert_eq!(history.len(), 2);
}
