# V2-R10 LoCoMo Guardrail Hardening Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add drift detection and one-command full validation for the current LoCoMo baseline.

**Architecture:** Keep retrieval logic untouched; add tooling-only guardrails and docs.

**Tech Stack:** Bash/Python scripts under `tools/locomo`, README updates.

---

### Task 1: Add Drift Baseline + Checker

**Files:**
- Create: `tools/locomo/baseline-guardrails-k5.json`
- Create: `tools/locomo/assert-locomo-drift.py`

- [ ] Add pinned baseline and allowed drop budget
- [ ] Implement drift checker JSON output + exit semantics

### Task 2: Add One-command Full Gate

**Files:**
- Create: `tools/locomo/run-full-gate.sh`

- [ ] Add ordered execution for LoCoMo gates + required 4 commands
- [ ] Support optional python binding rebuild toggle

### Task 3: Docs

**Files:**
- Modify: `tools/locomo/README.md`
- Modify: `README.md`

- [ ] Document new guardrail scripts and usage

### Task 4: Verification

**Files:**
- No source edits

- [ ] `tools/locomo/run-regression-gate.sh 5`
- [ ] `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
- [ ] `tools/locomo/run-full-gate.sh 5`
- [ ] required 4-command matrix evidence

### Task 5: Finish

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v2-r10-locomo-guardrail-hardening-results.md`

- [ ] Record final outputs
- [ ] Commit, PR, merge
