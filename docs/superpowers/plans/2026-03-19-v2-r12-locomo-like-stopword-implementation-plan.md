# V2-R12 LoCoMo `like` Stopword Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve LoCoMo retrieval with one minimal query-tokenization change by treating `like` as a stopword.

**Architecture:** Keep retrieval/rerank pipeline unchanged; update stopword list and add one unit test in `fts.rs` only.

**Tech Stack:** Rust (`memory-core`) + existing LoCoMo scripts.

---

### Task 1: Single-Knob Code Change

**Files:**
- Modify: `crates/memory-core/src/fts.rs`

- [ ] Add `like` to `STOPWORDS`.
- [ ] Add one unit test for `like` removal while preserving semantic anchors.

### Task 2: Verification

**Files:**
- No source edits

- [ ] `cargo test --workspace`
- [ ] `./.venv/bin/pytest python/tests -q`
- [ ] `corepack pnpm --dir packages/node test`
- [ ] `corepack pnpm --dir packages/mcp test`
- [ ] `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

### Task 3: Finish Round

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r12-locomo-like-stopword-results.md`

- [ ] Record LoCoMo and verification outputs.
- [ ] Commit, PR, merge.
