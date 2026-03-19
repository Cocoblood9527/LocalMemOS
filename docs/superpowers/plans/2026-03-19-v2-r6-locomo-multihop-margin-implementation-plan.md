# V2-R6 LoCoMo Multi-hop Margin Stabilization Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve multi-hop retrieval margin with minimal-risk query filler filtering while keeping all retrieval/category thresholds green.

**Architecture:** Extend stopword filtering in `fts` token normalization to remove selected low-signal question fillers. Keep ranking formula unchanged except for already-shipped V2-R5 behavior.

**Tech Stack:** Rust (`memory-core`), LoCoMo tooling (`tools/locomo`).

---

### Task 1: TDD for Additional Question Fillers

**Files:**
- Modify: `crates/memory-core/src/fts.rs`

- [ ] **Step 1: Add failing token-normalization tests for `ways/activities/changes/been`**
- [ ] **Step 2: Run focused tests and confirm RED**

Run: `cargo test -p memory-core fts::tests:: -- --nocapture`
Expected: FAIL on new assertions before implementation

### Task 2: Implement Minimal Stopword Expansion

**Files:**
- Modify: `crates/memory-core/src/fts.rs`

- [ ] **Step 1: Extend `STOPWORDS` with selected filler tokens**
- [ ] **Step 2: Re-run focused tests and confirm GREEN**

Run: `cargo test -p memory-core fts::tests:: -- --nocapture`
Expected: PASS

### Task 3: Retrieval and LoCoMo Checks

**Files:**
- No source edits required

- [ ] **Step 1:** `cargo test -p memory-core --test fts_fallback -- --nocapture`
- [ ] **Step 2:** `./.venv/bin/python -m pip install -e python`
- [ ] **Step 3:** `tools/locomo/run-regression-gate.sh 5`
- [ ] **Step 4:** `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`

### Task 4: Required Verification Matrix

**Files:**
- No source edits required

- [ ] **Step 1:** `cargo test --workspace`
- [ ] **Step 2:** `./.venv/bin/pytest python/tests -q`
- [ ] **Step 3:** `corepack pnpm --dir packages/node test`
- [ ] **Step 4:** `corepack pnpm --dir packages/mcp test`

### Task 5: Finishing

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r6-locomo-multihop-margin-results.md`

- [ ] **Step 1: Record metric deltas and gate status**
- [ ] **Step 2: Commit and create PR**
- [ ] **Step 3: Merge and fast-forward local `main`**
