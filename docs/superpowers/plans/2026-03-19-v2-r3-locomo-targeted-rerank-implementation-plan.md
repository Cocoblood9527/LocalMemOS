# V2-R3 LoCoMo Targeted Re-rank Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve weak LoCoMo categories with feature-based retrieval re-rank while enforcing a stronger default regression gate.

**Architecture:** Keep current FTS candidate generation, then compute blended ranking features (token coverage, bigram coverage, phrase bonus, bm25 stabilizer) in `memory-core`. Upgrade gate defaults in `tools/locomo`, and add a failure-sample exporter plus checked-in sample artifact.

**Tech Stack:** Rust (`memory-core`), Bash/Python tooling in `tools/locomo`, markdown reporting.

---

### Task 1: Add Re-rank Tests First (TDD RED)

**Files:**
- Modify: `crates/memory-core/tests/fts_fallback.rs`

- [ ] **Step 1: Add failing tests for phrase and multi-clue ranking**
- [ ] **Step 2: Run focused test command**

Run: `cargo test -p memory-core --test fts_fallback -- --nocapture`
Expected: FAIL on new ranking assertions.

### Task 2: Implement Feature-based Re-rank (TDD GREEN)

**Files:**
- Modify: `crates/memory-core/src/store.rs`
- Modify (if needed): `crates/memory-core/src/fts.rs`

- [ ] **Step 1: Implement feature extraction helpers (token/bigram/phrase)**
- [ ] **Step 2: Blend features into final relevance score**
- [ ] **Step 3: Update ranking order to use blended score**
- [ ] **Step 4: Re-run focused tests**

Run: `cargo test -p memory-core --test fts_fallback -- --nocapture`
Expected: PASS for old and new tests.

### Task 3: Upgrade Gate + Failure Sample Tooling

**Files:**
- Modify: `tools/locomo/run-regression-gate.sh`
- Modify: `tools/locomo/README.md`
- Modify: `README.md`
- Create: `tools/locomo/export-failure-samples.py`
- Create: `tools/locomo/refresh-failure-samples.sh`
- Create: `tools/locomo/failure-samples-k5.json`

- [ ] **Step 1: Raise default threshold to `0.55`**
- [ ] **Step 2: Add failure sample exporter + refresh wrapper**
- [ ] **Step 3: Generate and commit sample artifact (top misses)**

### Task 4: Metrics + Report

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r3-locomo-targeted-rerank-results.md`

- [ ] **Step 1: Run LoCoMo baseline/gate/official-like scripts**
- [ ] **Step 2: Compare V2-R2 vs V2-R3 overall and by-category**
- [ ] **Step 3: Document command lines and artifact paths**

### Task 5: Required Verification Matrix

**Files:**
- No code changes (verification only)

- [ ] **Step 1: `cargo test --workspace`**
- [ ] **Step 2: `./.venv/bin/pytest python/tests -q`**
- [ ] **Step 3: `corepack pnpm --dir packages/node test`**
- [ ] **Step 4: `corepack pnpm --dir packages/mcp test`**

### Task 6: Branch Completion

**Files:**
- No file edits

- [ ] **Step 1: Commit with conventional messages**
- [ ] **Step 2: Push and create PR**
- [ ] **Step 3: Merge PR and fast-forward local `main`**
