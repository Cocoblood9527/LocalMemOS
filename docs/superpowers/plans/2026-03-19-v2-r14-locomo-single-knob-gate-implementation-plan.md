# V2-R14 LoCoMo Single-Knob Gate Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a minimal evaluator script that enforces NO-OP unless `overall` and `multi-hop` both improve.

**Architecture:** Keep retrieval engine untouched; add one tooling script and docs updates only.

**Tech Stack:** Python script in `tools/locomo` + markdown docs.

---

### Task 1: Add Evaluator Script

**Files:**
- Create: `tools/locomo/evaluate-single-knob.py`

- [ ] Implement baseline and candidate evaluation for K=5 LoCoMo retrieval.
- [ ] Hardcode pass rule (`overall` and `multi-hop` both improve).
- [ ] Emit clear JSON decision payload (`APPLY` or `NO-OP`).

### Task 2: Update Docs

**Files:**
- Modify: `tools/locomo/README.md`
- Modify: `README.md`

- [ ] Document command usage.
- [ ] Document auto-stop decision rule.

### Task 3: Verification

**Files:**
- No source edits

- [ ] `./.venv/bin/python tools/locomo/evaluate-single-knob.py`
- [ ] `cargo test --workspace`
- [ ] `./.venv/bin/pytest python/tests -q`
- [ ] `corepack pnpm --dir packages/node test`
- [ ] `corepack pnpm --dir packages/mcp test`
- [ ] `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

### Task 4: Finish

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r14-locomo-single-knob-gate-results.md`

- [ ] Record decision output and verification evidence.
- [ ] Commit and merge.
