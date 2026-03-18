# V1.4 Command Drift Guard Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add README-level drift-guard guidance so maintainers know when to re-validate commands, what minimum regression order to run, and how to triage failures.

**Architecture:** Keep this round documentation-only and scoped to `README.md` implementation content. Introduce three compact sections (`When To Re-Validate Commands`, `Minimum Regression Order`, `Failure Triage Rule`) near existing verification content, while preserving the existing matrix and troubleshooting sections as operational references.

**Tech Stack:** Markdown, existing README structure, current verification command surface

---

## Preconditions

- Follow spec: `docs/superpowers/specs/2026-03-19-v1-4-command-drift-guard-design.md`
- Modify only `README.md` for implementation content in this plan (workflow artifacts may change separately during design/review loops)
- Do not add scripts, CI, runtime code changes, or extra user-facing docs outside workflow artifacts
- Keep `v1 MVP` boundaries explicit

## Proposed File Structure

- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-19-v1-4-command-drift-guard-design.md`
- Reference only: `Cargo.toml`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

## Milestones

1. Confirm insertion points and command-source consistency
2. Add command re-validation triggers
3. Add minimum regression order and keep it aligned with `Verification Path`
4. Add failure triage rule and verify no scope creep

## Task 1: Audit Anchors and Command Consistency Baseline

**Files:**
- Modify: `README.md`
- Reference only: `Cargo.toml`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

- [ ] **Step 1: Write local failing checklist**

```md
- README lacks explicit trigger conditions for re-validation
- README lacks a minimum regression sequence policy
- README lacks explicit triage rule for doc drift vs environment vs code regression
- README currently has no explicit equality check between `Verification Path` and minimum regression order
```

- [ ] **Step 2: Confirm insertion anchors in README**

Run: `rg -n "## Verification Path|## Clean-Run Validation Matrix|## Common Setup Failures|## Runtime Paths" README.md`
Expected: verification and matrix sections exist and allow concise insertion before `Clean-Run Validation Matrix`

- [ ] **Step 3: Confirm command reality for regression list**

Run: `sed -n '1,200p' python/pyproject.toml`
Run: `sed -n '1,200p' packages/node/package.json`
Run: `sed -n '1,200p' packages/mcp/package.json`
Expected: four command surfaces in `Verification Path` remain valid and unchanged

## Task 2: Add `When To Re-Validate Commands`

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Insert exact heading**

```md
## When To Re-Validate Commands
```

- [ ] **Step 2: Add trigger list with concrete conditions**

```md
- after dependency manifest or lockfile changes
- after Rust/Python/Node runtime or toolchain updates
- after README command edits
- after crate/package changes that affect build/test surfaces
```

- [ ] **Step 3: Tie triggers to minimum regression flow**

```md
- Each trigger requires running `Minimum Regression Order` at least once.
```

- [ ] **Step 4: Re-read section for compactness**

Run: `sed -n '80,210p' README.md`
Expected: trigger guidance is short, concrete, and non-redundant with matrix rows

## Task 3: Add `Minimum Regression Order`

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Insert exact heading**

```md
## Minimum Regression Order
```

- [ ] **Step 2: Add ordered four-command list**

```md
1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
```

- [ ] **Step 3: Add scope note**

```md
- This is the minimum default order; expand as needed for broader change sets.
```

- [ ] **Step 4: Verify the list exactly matches `Verification Path` commands**

Run: `rg -n "cargo test --workspace|pytest python/tests|packages/node test|packages/mcp test" README.md`
Expected: the same four command strings appear in both `Verification Path` and `Minimum Regression Order` without drift
- [ ] **Step 5: Verify `Failure Triage Rule` placement**

Run: `rg -n "## Minimum Regression Order|## Failure Triage Rule|## Clean-Run Validation Matrix" README.md`
Expected: `## Failure Triage Rule` appears directly after `## Minimum Regression Order` and before deeper recovery references

## Task 4: Add `Failure Triage Rule` and Final Validation

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Insert exact heading**

```md
## Failure Triage Rule
```

- [ ] **Step 2: Add concise triage bullets**

```md
- command mismatch with repo reality => documentation drift (update README)
- valid command but inconsistent local machine state => environment issue (apply matrix recovery)
- valid command + stable environment but failing tests => code regression (escalate)
```

- [ ] **Step 3: Validate no scope creep language**

Run: `rg -n "Windows|CI|automation script|v2" README.md`
Expected: no new scope-expanding promises introduced

- [ ] **Step 4: Run verification commands from README**

Run: `cargo test --workspace`
Run: `./.venv/bin/pytest python/tests -q`
Run: `corepack pnpm --dir packages/node test`
Run: `corepack pnpm --dir packages/mcp test`
Expected: all commands pass and README remains operationally accurate
If a failure is clearly unrelated to this README edit, record it as pre-existing/environmental rather than documentation drift

- [ ] **Step 5: Commit**

```bash
git add README.md
git commit -m "docs(readme): add v1.4 command drift guard guidance"
```

## Acceptance Checklist

- [ ] implementation content changes are limited to `README.md`
- [ ] exact headings are present:
- [ ] `## When To Re-Validate Commands`
- [ ] `## Minimum Regression Order`
- [ ] `## Failure Triage Rule`
- [ ] minimum regression list includes exactly four commands
- [ ] four minimum regression commands stay identical to `Verification Path`
- [ ] triage rule clearly separates doc drift, environment issue, and code regression
- [ ] matrix and troubleshooting sections remain complementary references
- [ ] no scripts, CI, or runtime code changes are introduced
- [ ] all four verification commands are run; any failures are triaged and documented via `Failure Triage Rule`

## Suggested Execution Order

1. Confirm anchors and command baseline.
2. Add re-validation triggers.
3. Add minimum regression order and command-equality check.
4. Add triage rule and run full verification.

## Handoff Notes

- Keep new sections short; avoid duplicating detailed matrix explanations.
- If command wording drifts during editing, treat `Verification Path` as the canonical source to realign.
- If one change would require policy-level CI decisions, defer that to a separate spec round.
