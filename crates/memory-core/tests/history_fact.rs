use memory_core::{MemoryStore, UpsertFactRequest};

#[test]
fn history_contains_versions_for_replaced_fact() {
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
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
            "bun",
        ))
        .unwrap();

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
