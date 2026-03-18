use std::thread::sleep;
use std::time::Duration;

use chrono::Utc;
use memory_core::{MemoryStore, RecallRequest, UpsertFactRequest};

#[test]
fn recall_returns_only_current_matching_fact() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
            "bun",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::exact(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
        ))
        .unwrap();
    assert_eq!(result.facts.len(), 1);
    assert_eq!(result.facts[0].value_text.as_deref(), Some("bun"));
}

#[test]
fn recall_returns_fact_effective_at_requested_time() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
            "npm",
        ))
        .unwrap();

    let between_versions = Utc::now();
    sleep(Duration::from_millis(10));

    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
            "bun",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest {
            namespace: "workspace".into(),
            scope_id: "localmemos".into(),
            entity: Some("project".into()),
            attribute: Some("preferred_package_manager".into()),
            text_query: None,
            include_history: false,
            as_of: Some(between_versions),
        })
        .unwrap();

    assert_eq!(result.facts.len(), 1);
    assert_eq!(result.facts[0].value_json, serde_json::json!("npm"));
    assert_eq!(result.facts[0].value_text.as_deref(), Some("npm"));
}
