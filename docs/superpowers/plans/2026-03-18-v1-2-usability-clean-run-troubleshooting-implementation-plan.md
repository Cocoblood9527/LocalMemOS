# V1.2 Usability Clean-Run Troubleshooting Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update `README.md` with a macOS/Linux clean-run checklist and a compact troubleshooting section so maintainers can unblock local setup quickly.

**Architecture:** Keep this round documentation-only and scoped to the root `README.md`. Preserve existing `v1` structure, insert the new sections at the spec-defined locations, and verify that all commands and failure-recovery notes match repository reality.

**Tech Stack:** Markdown, existing README content, Python `pyproject.toml`, package-level Node metadata, current verification commands

---

## Preconditions

- Follow spec: `docs/superpowers/specs/2026-03-18-v1-2-usability-clean-run-troubleshooting-design.md`
- Only modify `README.md`
- Do not add scripts, CI, extra docs, or runtime code changes
- Keep `v1 MVP` scope boundaries explicit

## Proposed File Structure

- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-18-v1-2-usability-clean-run-troubleshooting-design.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

## Milestones

1. Confirm current README structure and command assumptions
2. Add `Clean-Run Checklist (macOS + Linux)` after `First-Time Setup`
3. Add `Common Setup Failures` after `Verification Path`
4. Re-verify command accuracy and keep README concise

## Task 1: Audit Current README and Failure Surface

**Files:**
- Modify: `README.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

- [ ] **Step 1: Write a local failing checklist for this round**

```md
- README lacks a dedicated clean-run checklist section
- README lacks a troubleshooting section for common setup failures
- README needs explicit reminder that commands assume repository root
- README needs quick recovery guidance before deeper debugging
```

- [ ] **Step 2: Confirm current README section anchors for insertion**

Run: `rg -n "## First-Time Setup|## Verification Path|## Runtime Paths" README.md`
Expected: `First-Time Setup` and `Verification Path` exist and can be used as stable insertion points

- [ ] **Step 3: Verify dependency/tooling facts used by troubleshooting entries**

Run: `sed -n '1,200p' python/pyproject.toml`
Run: `sed -n '1,200p' packages/node/package.json`
Run: `sed -n '1,200p' packages/mcp/package.json`
Expected: Python package requires local install and Node/MCP rely on package-level pnpm scripts

## Task 2: Add Clean-Run Checklist Section

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Insert `Clean-Run Checklist (macOS + Linux)` after `First-Time Setup`**

```md
## Clean-Run Checklist (macOS + Linux)
```

- [ ] **Step 2: Add linear checklist items for local reset and verification**

```md
1. Run commands from repository root.
2. Ensure `.venv` exists, use explicit `./.venv/bin/...` invocation, and confirm Python package is installed in editable mode.
3. Run `corepack enable`.
4. Install Node dependencies for `packages/node` and `packages/mcp`.
5. Run the four verification commands in order.
```

- [ ] **Step 3: Add short context reminders**

```md
- All commands in this README assume repository root as working directory.
- If local state looks inconsistent, rerun setup commands before deeper debugging.
```

- [ ] **Step 4: Re-read updated area for brevity**

Run: `sed -n '40,180p' README.md`
Expected: New checklist section is short, operational, and does not repeat large blocks from setup sections

## Task 3: Add Common Setup Failures Section

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Insert `Common Setup Failures` after `Verification Path`**

```md
## Common Setup Failures
```

- [ ] **Step 2: Add a compact markdown table (`Symptom | Likely Cause | Minimal Fix`)**

```md
| Symptom | Likely Cause | Minimal Fix |
```

- [ ] **Step 3: Cover required failure cases from spec**

```md
- `ModuleNotFoundError: memory_sdk`
- `pnpm: command not found` or corepack-related failure
- tests run from wrong directory
- native build failure due to missing Rust toolchain in PATH
```

- [ ] **Step 4: Verify troubleshooting commands are consistent with setup path**

Run: `rg -n "memory_sdk|corepack enable|python3 -m venv|pip install -e python|cargo test --workspace" README.md`
Expected: Troubleshooting fixes point back to already documented setup and verification commands

## Task 4: Final Consistency Pass and Verification

**Files:**
- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-18-v1-2-usability-clean-run-troubleshooting-design.md`

- [ ] **Step 1: Re-read full README for structure and insertion correctness**

Run: `sed -n '1,360p' README.md`
Expected: `Clean-Run Checklist (macOS + Linux)` appears immediately after `First-Time Setup`; `Common Setup Failures` appears immediately after `Verification Path`

- [ ] **Step 2: Validate no scope creep**

Run: `rg -n "Windows|CI|workflow automation|v2" README.md`
Expected: No newly introduced Windows setup promises, CI automation instructions, or `v2` expansion content

- [ ] **Step 3: Run documented verification commands**

Run: `cargo test --workspace`
Run: `./.venv/bin/pytest python/tests -q`
Run: `corepack pnpm --dir packages/node test`
Run: `corepack pnpm --dir packages/mcp test`
Expected: All commands pass and confirm README remains operationally accurate

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): add v1.2 clean-run troubleshooting guidance"
```

## Acceptance Checklist

- [ ] only `README.md` is modified during implementation
- [ ] `Clean-Run Checklist (macOS + Linux)` is present after `First-Time Setup`
- [ ] `Common Setup Failures` is present after `Verification Path`
- [ ] troubleshooting is in markdown table format
- [ ] required failure scenarios are covered with minimal fixes
- [ ] repository-root working-directory assumption is explicit
- [ ] `v1 MVP` boundaries remain intact
- [ ] no scripts, CI, or extra docs are introduced
- [ ] all four verification commands pass

## Suggested Execution Order

1. Audit current anchors and command surface.
2. Add clean-run checklist and context reminders.
3. Add troubleshooting table with minimal fixes.
4. Re-run verification and close with a final concise pass.

## Handoff Notes

- If a troubleshooting fix duplicates setup instructions, prefer linking back to existing commands instead of creating alternate flows.
- If README starts to exceed compact operational style, trim wording rather than splitting docs in this round.
- If a new failure category requires non-trivial debugging flow, defer it to a separate future spec.
