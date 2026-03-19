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

#[test]
fn natural_language_question_retrieves_relevant_fact() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "preferred_package_manager",
            "We use bun to install workspace dependencies",
        ))
        .unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "workspace",
            "localmemos",
            "project",
            "runtime",
            "Runtime is Rust with Axum",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::text(
            "workspace",
            "localmemos",
            "What package manager do we use to install dependencies for this workspace?",
        ))
        .unwrap();

    assert!(!result.facts.is_empty());
    assert_eq!(result.facts[0].attribute, "preferred_package_manager");
}

#[test]
fn punctuation_heavy_question_does_not_fail_query() {
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
            "bun??? rust!!!",
        ))
        .unwrap();
    assert_eq!(result.facts.len(), 1);
}

#[test]
fn ranking_prefers_facts_with_more_query_overlap() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "benchmark",
            "sample-1",
            "dialog",
            "10",
            "Alex booked a flight to Seattle on March 3",
        ))
        .unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "benchmark",
            "sample-1",
            "dialog",
            "11",
            "Alex likes coffee",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::text(
            "benchmark",
            "sample-1",
            "When did Alex book a flight to Seattle?",
        ))
        .unwrap();

    assert!(!result.facts.is_empty());
    assert_eq!(result.facts[0].attribute, "10");
}

#[test]
fn ranking_prefers_phrase_continuity_over_recency_when_tokens_tie() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "benchmark",
            "sample-phrase-1",
            "dialog",
            "100",
            "Alex booked a flight to Seattle on March 3 before the conference",
        ))
        .unwrap();
    sleep(Duration::from_millis(10));
    store
        .upsert_fact(UpsertFactRequest::manual(
            "benchmark",
            "sample-phrase-1",
            "dialog",
            "101",
            "Alex Seattle March 3 conference before booked a flight to",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::text(
            "benchmark",
            "sample-phrase-1",
            "flight Seattle March before conference",
        ))
        .unwrap();

    assert!(!result.facts.is_empty());
    assert_eq!(result.facts[0].attribute, "100");
}

#[test]
fn ranking_prefers_multiclue_bigram_alignment() {
    let mut store = MemoryStore::open(":memory:").unwrap();
    store
        .upsert_fact(UpsertFactRequest::manual(
            "benchmark",
            "sample-phrase-2",
            "dialog",
            "200",
            "Mina borrowed Leo's camera before hiking in Kyoto at dawn",
        ))
        .unwrap();
    sleep(Duration::from_millis(10));
    store
        .upsert_fact(UpsertFactRequest::manual(
            "benchmark",
            "sample-phrase-2",
            "dialog",
            "201",
            "Mina Kyoto dawn Leo camera borrowed hiking before in",
        ))
        .unwrap();

    let result = store
        .recall(RecallRequest::text(
            "benchmark",
            "sample-phrase-2",
            "borrowed Leo camera hiking Kyoto",
        ))
        .unwrap();

    assert!(!result.facts.is_empty());
    assert_eq!(result.facts[0].attribute, "200");
}
