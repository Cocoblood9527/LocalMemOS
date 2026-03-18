use memory_core::{MemoryStore, RecallRequest, UpsertFactRequest};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn text_query_falls_back_to_fts() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "summary",
            "Uses bun and Rust core",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::text(
            "workspace",
            "localmemos",
            "Rust core",
        ))
        .unwrap();
    assert_eq!(result.facts.len(), 1);
}

#[test]
fn text_query_honors_as_of_time() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    let first = store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "summary",
            "Uses bun and Rust core",
        ))
        .unwrap();
    sleep(Duration::from_millis(10));
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "summary",
            "Uses npm and Rust core",
        ))
        .unwrap();

    let current = store
        .recall(RecallRequest::text("workspace", "localmemos", "bun"))
        .unwrap();
    assert_eq!(current.facts.len(), 0);

    let mut historical_req = RecallRequest::text("workspace", "localmemos", "bun");
    historical_req.as_of = Some(first.updated_at);
    let historical = store.recall(historical_req).unwrap();
    assert_eq!(historical.facts.len(), 1);
    assert_eq!(
        historical.facts[0].value_text.as_deref(),
        Some("Uses bun and Rust core")
    );
}
