# V2-R15 LoCoMo Closure Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Finalize LoCoMo V2 with evidence-backed closure and a stable tag.

**Architecture:** Docs and release-management only; no retrieval code changes.

**Tech Stack:** markdown docs + git tag + existing validation commands.

---

### Task 1: Closure Documentation

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r15-locomo-closure-results.md`

- [ ] Record closure criteria and evidence references.

### Task 2: Final Verification Evidence

**Files:**
- No source edits

- [ ] `cargo test --workspace`
- [ ] `./.venv/bin/pytest python/tests -q`
- [ ] `corepack pnpm --dir packages/node test`
- [ ] `corepack pnpm --dir packages/mcp test`
- [ ] `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

### Task 3: Stable Tag

**Files:**
- No source edits

- [ ] Create stable tag from merged `main`.
- [ ] Push stable tag to remote.
