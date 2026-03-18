# V1.3 Usability Clean-Run Validation Matrix Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a compact clean-run validation matrix to `README.md` so maintainers can evaluate pass/fail and quick recovery per command on macOS/Linux.

**Architecture:** Keep this round documentation-only and scoped to `README.md` implementation content. Preserve existing `First-Time Setup` and `Verification Path` as execution order sources, add a matrix immediately after `Verification Path`, and keep `Common Setup Failures` as complementary guidance with reduced overlap.

**Tech Stack:** Markdown, existing README structure, current local verification command set

---

## Preconditions

- Follow spec: `docs/superpowers/specs/2026-03-19-v1-3-usability-clean-run-validation-matrix-design.md`
- Modify only `README.md` for implementation content in this plan (workflow artifacts may change separately during design/review loops)
- Do not add scripts, CI, runtime code changes, or extra user-facing docs outside workflow artifacts
- Keep `v1 MVP` boundaries explicit

## Proposed File Structure

- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-19-v1-3-usability-clean-run-validation-matrix-design.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

## Milestones

1. Confirm matrix insertion point and existing section roles
2. Add `Clean-Run Validation Matrix (macOS + Linux)` with fixed row schema
3. Reduce overlap with `Common Setup Failures`
4. Re-verify command accuracy and README compactness

## Task 1: Audit Anchors and Command Surface

**Files:**
- Modify: `README.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

- [ ] **Step 1: Write local failing checklist for this round**

```md
- README does not yet include a command-by-command validation matrix
- README pass/fail criteria are not mapped per command
- quick recovery is not attached directly to each command row
- overlap risk exists between matrix and common failures sections
```

- [ ] **Step 2: Confirm insertion anchors**

Run: `rg -n "## Verification Path|## Common Setup Failures|## Runtime Paths" README.md`
Expected: `Verification Path` and `Common Setup Failures` exist and matrix can be inserted between them

- [ ] **Step 3: Confirm command reality for matrix rows**

Run: `sed -n '1,200p' python/pyproject.toml`
Run: `sed -n '1,200p' packages/node/package.json`
Run: `sed -n '1,200p' packages/mcp/package.json`
Expected: Python editable install remains required and Node/MCP commands remain package-level `pnpm` flows

## Task 2: Add Validation Matrix Section

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Insert section header after `Verification Path`**

```md
## Clean-Run Validation Matrix (macOS + Linux)
```

- [ ] **Step 2: Add fixed schema table**

```md
| Command | Expected Result | If Failed | Quick Recovery |
```

- [ ] **Step 3: Populate rows for full clean-run path**

```md
1) `python3 -m venv .venv`
2) `./.venv/bin/python -m pip install -U pip pytest maturin`
3) `./.venv/bin/python -m pip install -e python`
4) `corepack enable`
5) `corepack pnpm --dir packages/node install`
6) `corepack pnpm --dir packages/mcp install`
7) `cargo test --workspace`
8) `./.venv/bin/pytest python/tests -q`
9) `corepack pnpm --dir packages/node test`
10) `corepack pnpm --dir packages/mcp test`
```

- [ ] **Step 4: Keep row content compact and actionable**

Run: `sed -n '70,190p' README.md`
Expected: each row has a short pass signal and one minimal recovery action without long prose

- [ ] **Step 5: Verify root-directory assumption remains explicit**

Run: `rg -n "repository root|working directory" README.md`
Expected: README still explicitly states commands should be run from repository root

## Task 3: Refine Relationship with Common Failures

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Keep setup order source-of-truth explicit**

```md
- `First-Time Setup` remains canonical command order
- matrix is pass/fail + recovery overlay
```

- [ ] **Step 2: Reduce overlap in `Common Setup Failures`**

```md
- keep high-value symptom entries
- reference matrix recovery paths when failures map directly
- avoid duplicating long command explanations
```

- [ ] **Step 3: Verify no section-role conflicts**

Run: `rg -n "First-Time Setup|Verification Path|Clean-Run Validation Matrix|Common Setup Failures" README.md`
Expected: section roles are clear and non-conflicting

## Task 4: Final Verification and Scope Check

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Re-read full README for compactness and placement**

Run: `sed -n '1,360p' README.md`
Expected: matrix appears after `Verification Path` and before `Common Setup Failures`, with concise wording

- [ ] **Step 2: Validate no scope creep language**

Run: `rg -n "Windows|CI|automation script|v2" README.md`
Expected: no new promises outside this round's scope

- [ ] **Step 3: Run verification commands from README**

Run: `cargo test --workspace`
Run: `./.venv/bin/pytest python/tests -q`
Run: `corepack pnpm --dir packages/node test`
Run: `corepack pnpm --dir packages/mcp test`
Expected: all commands pass and README remains operationally accurate
If a failure is clearly unrelated to this README edit, record it as pre-existing/environmental rather than a documentation regression

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): add v1.3 clean-run validation matrix"
```

## Acceptance Checklist

- [ ] implementation content changes are limited to `README.md`
- [ ] matrix section exists at the spec-defined insertion point
- [ ] matrix uses fixed columns: `Command | Expected Result | If Failed | Quick Recovery`
- [ ] matrix covers the full 10-step clean-run path from setup through all test surfaces
- [ ] setup order source-of-truth remains `First-Time Setup`
- [ ] troubleshooting remains complementary rather than duplicated
- [ ] no scripts, CI, or runtime code changes are introduced
- [ ] all four verification commands pass

## Suggested Execution Order

1. Confirm insertion anchors and command reality.
2. Add matrix header + schema + 10 rows.
3. Adjust `Common Setup Failures` for non-duplicated role.
4. Re-run verification and finish with one concise pass.

## Handoff Notes

- If matrix rows become verbose, shorten expected-result text rather than splitting docs in this round.
- If a row needs multiple recovery branches, choose one minimal recovery path and keep deeper detail in `Common Setup Failures`.
- If command behavior differs from README during verification, update README to actual repository behavior before closing.
