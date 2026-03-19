# V2-R16 Local Rerank Rare-Anchor Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve local retrieval ranking by adding one rare-anchor-aware rerank signal.

**Architecture:** Keep existing FTS and rerank pipeline; add one feature derived from per-query candidate token frequency.

**Tech Stack:** Rust (`memory-core`) + existing LoCoMo scripts.

---

### Task 1: Add Rare-Anchor Rerank Signal

**Files:**
- Modify: `crates/memory-core/src/store.rs`

- [ ] Build candidate sequences first, then compute query token document frequency.
- [ ] Derive rare-anchor token set and per-row rare-anchor coverage.
- [ ] Add rare-anchor coverage term into rerank score with minimal weighting.

### Task 2: Add Regression Test

**Files:**
- Modify: `crates/memory-core/tests/fts_fallback.rs`

- [ ] Add one test for overlap tie where rare anchor should rank first.

### Task 3: Verification

**Files:**
- No source edits

- [ ] `cargo test --workspace`
- [ ] `./.venv/bin/pytest python/tests -q`
- [ ] `corepack pnpm --dir packages/node test`
- [ ] `corepack pnpm --dir packages/mcp test`
- [ ] `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

### Task 4: Finish

**Files:**
- Create: `docs/superpowers/releases/2026-03-20-v2-r16-local-rerank-rare-anchor-results.md`

- [ ] Record score deltas and verification evidence.
- [ ] Commit, PR, merge.
