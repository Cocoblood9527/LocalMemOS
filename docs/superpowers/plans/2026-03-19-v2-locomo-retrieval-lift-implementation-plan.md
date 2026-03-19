# V2 LoCoMo Retrieval Lift Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve text recall relevance so LoCoMo `evidence hit@5` can move from `0.001` toward `>= 0.05` without introducing vector retrieval.

**Architecture:** Keep adapter contracts unchanged and implement a `memory-core` retrieval upgrade: normalize question text into robust FTS tokens, issue OR-prefix FTS query, and rank matches by BM25 plus lightweight lexical overlap tie-break before recency.

**Tech Stack:** Rust (`rusqlite` + SQLite FTS5), existing workspace tests, Python/Node/MCP adapter tests.

---

### Task 1: Add FTS Query Rewrite Utilities (TDD)

**Files:**
- Modify: `crates/memory-core/src/fts.rs`
- Test: `crates/memory-core/tests/fts_fallback.rs`

- [ ] **Step 1: Write failing tests for normalized query rewrite behavior**

Add tests that prove:
- natural-language questions are tokenized safely
- punctuation-heavy queries do not produce SQL parse errors

- [ ] **Step 2: Run focused test to verify RED**

Run: `cargo test -p memory-core fts_fallback -- --nocapture`
Expected: FAIL because V2 rewrite behavior is not implemented yet.

- [ ] **Step 3: Implement minimal normalization + rewrite helpers**

Implement in `fts.rs`:
- query tokenizer with stopword filtering and dedup
- FTS query builder (`token* OR token* ...`)

- [ ] **Step 4: Re-run focused test to verify GREEN**

Run: `cargo test -p memory-core fts_fallback -- --nocapture`
Expected: PASS for rewrite tests.

### Task 2: Upgrade Recall Ranking Pipeline (TDD)

**Files:**
- Modify: `crates/memory-core/src/store.rs`
- Test: `crates/memory-core/tests/fts_fallback.rs`

- [ ] **Step 1: Add failing ranking test**

Add a test where two rows match one query token, but only one row matches several key tokens; assert relevant row appears first.

- [ ] **Step 2: Run focused test to verify RED**

Run: `cargo test -p memory-core fts_fallback -- --nocapture`
Expected: FAIL on ranking-order assertion.

- [ ] **Step 3: Implement relevance-first ordering**

Update `recall_with_fts`:
- use rewritten FTS query
- use `bm25(facts_fts, ...)` as primary ordering signal
- add lexical overlap tie-break for stable top-k relevance
- keep `as_of` filtering intact

- [ ] **Step 4: Re-run focused test to verify GREEN**

Run: `cargo test -p memory-core fts_fallback -- --nocapture`
Expected: PASS on ranking + existing recall semantics.

### Task 3: Full Verification Matrix (Required Each Round)

**Files:**
- No code changes (verification only)

- [ ] **Step 1: Rust workspace tests**

Run: `cargo test --workspace`
Expected: all tests pass.

- [ ] **Step 2: Python tests**

Run: `./.venv/bin/pytest python/tests -q`
Expected: all tests pass.

- [ ] **Step 3: Node package tests**

Run: `corepack pnpm --dir packages/node test`
Expected: all tests pass.

- [ ] **Step 4: MCP package tests**

Run: `corepack pnpm --dir packages/mcp test`
Expected: all tests pass.

### Task 4: LoCoMo Baseline Re-check

**Files:**
- `/tmp/localmemos-v1-locomo-baseline.sh` (read-only use)

- [ ] **Step 1: Run K=5 retrieval baseline script**

Run: `BASE_DIR=/tmp/localmemos-locomo-v2-round1 /tmp/localmemos-v1-locomo-baseline.sh 5`
Expected: collect new `hit@5` metric.

- [ ] **Step 2: Record blocker if dataset/network unavailable**

If LoCoMo source cannot be fetched in current environment, explicitly record command failure and reason, while preserving all core verification evidence.
