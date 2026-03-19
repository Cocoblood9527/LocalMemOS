# V2-R4 LoCoMo Stabilization + Category Gate Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Recover LoCoMo retrieval from the current regression while shipping category-aware gating as a stable guardrail.

**Architecture:** Roll back unvalidated retrieval ranking deltas in `memory-core` to the known-good V2-R3 baseline, retain category gate tooling and docs, then validate with LoCoMo gate scripts and full cross-language test matrix.

**Tech Stack:** Rust (`memory-core`), Bash/Python (`tools/locomo`), repository docs (`README.md`, `tools/locomo/README.md`).

---

### Task 1: Stabilize Retrieval Logic in `memory-core`

**Files:**
- Modify: `crates/memory-core/src/store.rs`
- Modify: `crates/memory-core/src/fts.rs`
- Modify: `crates/memory-core/tests/fts_fallback.rs`

- [ ] **Step 1: Remove experimental tie-break ranking delta in `store.rs`**
- [ ] **Step 2: Remove unvalidated anchor-token helper addition in `fts.rs`**
- [ ] **Step 3: Remove or adjust tests that only validate reverted behavior**
- [ ] **Step 4: Run focused Rust test suite**

Run: `cargo test -p memory-core --test fts_fallback -- --nocapture`
Expected: PASS

### Task 2: Keep Category Gate Tooling + Docs

**Files:**
- Create: `tools/locomo/assert-locomo-thresholds.py`
- Create: `tools/locomo/run-category-gate.sh`
- Modify: `tools/locomo/README.md`
- Modify: `README.md`

- [ ] **Step 1: Ensure scripts are executable and argument defaults are correct**
- [ ] **Step 2: Ensure README docs include category-gate usage and thresholds**

### Task 3: Rebuild Binding and Re-run LoCoMo Gates

**Files:**
- No source edits required

- [ ] **Step 1: Rebuild python extension used by LoCoMo tooling**
Run: `./.venv/bin/python -m pip install -e python`
Expected: install succeeds

- [ ] **Step 2: Run regression gate**
Run: `tools/locomo/run-regression-gate.sh 5`
Expected: `overall >= 0.55`

- [ ] **Step 3: Run category gate**
Run: `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
Expected: explicit JSON pass/fail for overall + categories

### Task 4: Required Verification Matrix

**Files:**
- No source edits required

- [ ] **Step 1:** `cargo test --workspace`
- [ ] **Step 2:** `./.venv/bin/pytest python/tests -q`
- [ ] **Step 3:** `corepack pnpm --dir packages/node test`
- [ ] **Step 4:** `corepack pnpm --dir packages/mcp test`

### Task 5: Finishing

**Files:**
- No source edits required

- [ ] **Step 1: Summarize LoCoMo metric evidence and gate status**
- [ ] **Step 2: Commit with conventional message**
- [ ] **Step 3: Report next retrieval iteration targets (multi-hop/open-domain uplift)**
