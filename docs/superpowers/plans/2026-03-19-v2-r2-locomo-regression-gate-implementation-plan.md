# V2-R2 LoCoMo Regression Gate Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move LoCoMo evaluation from `/tmp` into repository tooling and add a retrieval hit@K regression gate.

**Architecture:** Keep script behavior close to existing `/tmp` baseline, package under `tools/locomo`, and add a thin Python assertion utility to enforce `hit@K` threshold with clear pass/fail output.

**Tech Stack:** Bash, Python 3 (stdlib), existing Memory SDK flow, markdown reporting.

---

### Task 1: Migrate LoCoMo Scripts into Repository

**Files:**
- Create: `tools/locomo/baseline.sh`
- Create: `tools/locomo/qa-proxy.sh`
- Create: `tools/locomo/official-like-eval.sh`
- Create: `tools/locomo/README.md`

- [ ] **Step 1: Copy existing script logic into repo paths**
- [ ] **Step 2: Normalize script headers, usage help, and artifact output paths**
- [ ] **Step 3: Make scripts executable**
- [ ] **Step 4: Smoke-run `tools/locomo/baseline.sh 5`**

### Task 2: Add Threshold Gate

**Files:**
- Create: `tools/locomo/assert-hit-threshold.py`
- Create: `tools/locomo/run-regression-gate.sh`

- [ ] **Step 1: Write failing assertion scenario (manual invocation)**

Run:
`python3 tools/locomo/assert-hit-threshold.py --result /tmp/nonexistent.json --threshold 0.05`
Expected: FAIL with clear error.

- [ ] **Step 2: Implement parser + threshold assertion**
- [ ] **Step 3: Wire gate wrapper script**
- [ ] **Step 4: Run gate against real `baseline.sh` output**

### Task 3: Produce R0 vs Current Comparison Report

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r2-locomo-retrieval-comparison.md`

- [ ] **Step 1: Execute baseline and official-like scripts from `tools/locomo`**
- [ ] **Step 2: Capture overall and by-category metrics**
- [ ] **Step 3: Document commands, artifact paths, and metric table**

### Task 4: Required Verification Matrix

**Files:**
- No code changes (verification only)

- [ ] **Step 1: Run `cargo test --workspace`**
- [ ] **Step 2: Run `./.venv/bin/pytest python/tests -q`**
- [ ] **Step 3: Run `corepack pnpm --dir packages/node test`**
- [ ] **Step 4: Run `corepack pnpm --dir packages/mcp test`**

### Task 5: Branch Completion

**Files:**
- No file edits

- [ ] **Step 1: Stage and commit with conventional message**
- [ ] **Step 2: Push branch**
- [ ] **Step 3: Create/update PR with metrics and verification evidence**
