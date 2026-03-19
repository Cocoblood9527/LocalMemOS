# V2-R11 LoCoMo CI Full Gate Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add CI automation so LoCoMo full-gate is enforced for PR/push events on `main`.

**Architecture:** Keep retrieval logic unchanged; add a GitHub Actions workflow that bootstraps Rust/Python/Node dependencies and executes existing `tools/locomo/run-full-gate.sh` as the single gate entrypoint, then uploads diagnostics artifacts.

**Tech Stack:** GitHub Actions YAML + existing Bash/Python tooling + README docs.

---

### Task 1: Add CI Workflow

**Files:**
- Create: `.github/workflows/locomo-full-gate.yml`

- [ ] Add workflow triggers for `pull_request` to `main`, `push` to `main`, and manual dispatch.
- [ ] Add environment bootstrap steps: Rust, Python `.venv`, Node/Corepack, and package installs.
- [ ] Run `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5` as canonical gate command.
- [ ] Capture and upload gate logs/result artifacts.

### Task 2: Update Documentation

**Files:**
- Modify: `tools/locomo/README.md`
- Modify: `README.md`

- [ ] Document CI workflow behavior and trigger scope.
- [ ] Keep local usage commands unchanged; position CI gate as complement to local validation.

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
- Create: `docs/superpowers/releases/2026-03-19-v2-r11-locomo-ci-full-gate-results.md`

- [ ] Record verification evidence and CI workflow summary.
- [ ] Commit changes on feature branch.
