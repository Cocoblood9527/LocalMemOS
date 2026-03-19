# V2-R5 LoCoMo Multi-hop Long-query Re-rank Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Raise LoCoMo multi-hop hit@5 to at least `0.42` with a minimal-risk long-query ranking adjustment.

**Architecture:** Keep existing FTS and bm25 pipeline. In long-query ranking, retain bm25 as primary comparator but use existing rerank score as secondary comparator before recency fallback.

**Tech Stack:** Rust (`memory-core`), existing LoCoMo tooling scripts in `tools/locomo`.

---

### Task 1: TDD Red Test for Long-query Tie-break

**Files:**
- Modify: `crates/memory-core/tests/fts_fallback.rs`

- [ ] **Step 1: Add long-query ranking behavior test**
- [ ] **Step 2: Run focused test and confirm failure (RED)**

Run: `cargo test -p memory-core --test fts_fallback long_query_prefers_rerank_signal_over_recency -- --nocapture`
Expected: FAIL before implementation

### Task 2: Implement Minimal Long-query Comparator Change

**Files:**
- Modify: `crates/memory-core/src/store.rs`

- [ ] **Step 1: Update long-query sort to use rerank score as secondary comparator**
- [ ] **Step 2: Re-run focused test and confirm pass (GREEN)**

Run: `cargo test -p memory-core --test fts_fallback long_query_prefers_rerank_signal_over_recency -- --nocapture`
Expected: PASS

### Task 3: Rebuild binding + LoCoMo Metrics

**Files:**
- No source edits required

- [ ] **Step 1: Rebuild python binding**
Run: `./.venv/bin/python -m pip install -e python`

- [ ] **Step 2: Run regression gate**
Run: `tools/locomo/run-regression-gate.sh 5`

- [ ] **Step 3: Run category gate**
Run: `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`

### Task 4: Required Verification Matrix

**Files:**
- No source edits required

- [ ] **Step 1:** `cargo test --workspace`
- [ ] **Step 2:** `./.venv/bin/pytest python/tests -q`
- [ ] **Step 3:** `corepack pnpm --dir packages/node test`
- [ ] **Step 4:** `corepack pnpm --dir packages/mcp test`

### Task 5: Finishing

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r5-locomo-multihop-longquery-rerank-results.md`

- [ ] **Step 1: Document metric deltas and gate status**
- [ ] **Step 2: Commit and open PR**
- [ ] **Step 3: Merge to main and fast-forward local main**
