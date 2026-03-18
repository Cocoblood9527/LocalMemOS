use memory_core::{MemoryStore, RecallRequest, UpsertFactRequest};

#[test]
fn workspace_recall_does_not_return_user_facts() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "user",
            "default",
            "preference",
            "language",
            "zh-CN",
        ))
        .unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "language",
            "English docs",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::exact(
            "workspace",
            "localmemos",
            "project",
            "language",
        ))
        .unwrap();
    assert_eq!(result.facts.len(), 1);
    assert_eq!(result.facts[0].namespace, "workspace");
}
