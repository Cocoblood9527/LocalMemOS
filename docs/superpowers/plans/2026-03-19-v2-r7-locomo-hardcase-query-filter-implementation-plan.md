# V2-R7 LoCoMo Hard-case Query Filter Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Run a low-risk hard-case query-filter experiment and keep only metrics-safe improvements.

**Architecture:** Add one candidate filler token (`seen`) to query stopwords with TDD, validate via LoCoMo gates, and retain only if threshold-safe and non-regressive.

**Tech Stack:** Rust (`memory-core`), LoCoMo scripts (`tools/locomo`).

---

### Task 1: TDD RED for `seen` filler token

**Files:**
- Modify: `crates/memory-core/src/fts.rs`

- [ ] **Step 1: Add failing test asserting `seen` is removed from query tokens**
- [ ] **Step 2: Run focused test to confirm RED**

Run: `cargo test -p memory-core fts::tests::query_tokens_drop_seen_filler -- --nocapture`
Expected: FAIL before implementation

### Task 2: Implement stopword candidate + GREEN

**Files:**
- Modify: `crates/memory-core/src/fts.rs`

- [ ] **Step 1: Add `seen` to `STOPWORDS`**
- [ ] **Step 2: Re-run focused tests and ensure GREEN**

Run: `cargo test -p memory-core fts::tests:: -- --nocapture`
Expected: PASS

### Task 3: Retrieval Validation

**Files:**
- Modify: `tools/locomo/failure-samples-k5.json`

- [ ] **Step 1:** `cargo test -p memory-core --test fts_fallback -- --nocapture`
- [ ] **Step 2:** `./.venv/bin/python -m pip install -e python`
- [ ] **Step 3:** `tools/locomo/run-regression-gate.sh 5`
- [ ] **Step 4:** `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
- [ ] **Step 5:** keep/revert `seen` candidate based on gate/metric result

### Task 4: Required Verification Matrix

**Files:**
- No source edits required

- [ ] **Step 1:** `cargo test --workspace`
- [ ] **Step 2:** `./.venv/bin/pytest python/tests -q`
- [ ] **Step 3:** `corepack pnpm --dir packages/node test`
- [ ] **Step 4:** `corepack pnpm --dir packages/mcp test`

### Task 5: Finishing

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r7-locomo-hardcase-query-filter-results.md`

- [ ] **Step 1: Document final keep/revert decision and metrics**
- [ ] **Step 2: Commit + PR + merge to main**
