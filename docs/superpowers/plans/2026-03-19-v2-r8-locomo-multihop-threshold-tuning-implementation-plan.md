# V2-R8 LoCoMo Multi-hop Threshold Tuning Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Run bounded retrieval parameter experiments and keep only threshold-safe improvements.

**Architecture:** Change one parameter at a time, run focused tests + LoCoMo gates, and revert any regression.

**Tech Stack:** Rust (`memory-core`), LoCoMo scripts (`tools/locomo`).

---

### Task 1: Run Parameter Experiments

**Files:**
- Modify and revert as needed: `crates/memory-core/src/store.rs`
- Modify and revert as needed: `crates/memory-core/src/fts.rs`

- [ ] Test long-query threshold `>=7`
- [ ] Test long-query threshold `>=5`
- [ ] Test bm25 evidence weight `2.0`
- [ ] Test query token cap `10`
- [ ] Revert non-qualifying candidates

### Task 2: Verify Final State

**Files:**
- No source edits required

- [ ] `tools/locomo/run-regression-gate.sh 5`
- [ ] `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
- [ ] required 4-command verification matrix

### Task 3: Record and Finish

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r8-locomo-multihop-threshold-tuning-results.md`

- [ ] Document experiment matrix and keep/revert decisions
- [ ] Commit docs-only conclusion if no safe uplift found
- [ ] Merge to `main`
