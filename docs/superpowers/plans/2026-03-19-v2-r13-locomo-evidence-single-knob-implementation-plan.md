# V2-R13 LoCoMo Evidence-Only Single-Knob Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Run a constrained evidence check for three stopword candidates and avoid code change unless acceptance rule is met.

**Architecture:** A/B evaluation only; repository code remains unchanged when criteria are not met.

**Tech Stack:** Python offline evaluator + existing LoCoMo gate scripts.

---

### Task 1: Candidate Evaluation

**Files:**
- Create: `/tmp/v2_r13_candidate_eval.py` (temporary)

- [ ] Evaluate `games`, `kind`, `names` against current baseline.
- [ ] Compare `overall` + `multi-hop` jointly.

### Task 2: Execute Decision

**Files:**
- No retrieval source edits if acceptance rule fails

- [ ] Apply no-op decision if no candidate satisfies acceptance rule.

### Task 3: Verification

**Files:**
- No source edits

- [ ] `cargo test --workspace`
- [ ] `./.venv/bin/pytest python/tests -q`
- [ ] `corepack pnpm --dir packages/node test`
- [ ] `corepack pnpm --dir packages/mcp test`
- [ ] `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

### Task 4: Finish Round

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r13-locomo-evidence-single-knob-results.md`

- [ ] Record A/B evidence and no-op rationale.
- [ ] Commit docs-only round.
