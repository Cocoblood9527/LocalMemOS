# V1.6 Quick Path Picker Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a 30-second README quick path picker that maps change type to role path and minimum command action while preserving existing v1 command and semantic truth.

**Architecture:** Update only `README.md` by inserting a concise quick-picker section after `Role-Based Operation Paths`. Reuse existing policy sections (`Verification Path`, `Minimum Regression Order`, `Failure Triage Rule`) and the unchanged four-command sequence. Avoid command drift by copying command lines verbatim from current README truth.

**Tech Stack:** Markdown documentation, existing Rust/Python/Node/MCP verification commands.

---

### Task 1: Prepare Context and Insertion Point

**Files:**
- Modify: `README.md`
- Reference: `docs/superpowers/specs/2026-03-19-v1-6-quick-path-picker-design.md`

- [ ] **Step 1: Inspect current role-path section and neighboring headers**

Run: `rg -n "^## (Role-Based Operation Paths|Repository Layout|Verification Path|Minimum Regression Order|Failure Triage Rule)$" README.md && sed -n '1,220p' README.md`
Expected: Stable insertion point after role paths and before repository layout.

- [ ] **Step 2: Confirm current command truth lines**

Run: `sed -n '/## Verification Path/,/## When To Re-Validate Commands/p' README.md && sed -n '/## Minimum Regression Order/,/## Failure Triage Rule/p' README.md`
Expected: Four-command sequence visible and identical across command-truth sections.

### Task 2: Implement 30-Second Quick Path Picker

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add `## 30-Second Path Picker` section**

Insert section after `Role-Based Operation Paths` and before `Repository Layout`.

- [ ] **Step 2: Add quick table mapping**

Add one compact table with columns:
- `If your change is...`
- `Use role path`
- `Start here`
- `Minimum command action`

Cover four role categories:
- docs guidance changes
- core semantics changes
- adapter surface changes
- clean-run/environment validation

- [ ] **Step 3: Add minimum command copy/run block**

Add one four-command code block exactly matching existing command truth:
```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```

- [ ] **Step 4: Add escalation line**

Include explicit line: unclear path or failure classification should use `Failure Triage Rule`.

- [ ] **Step 5: Validate placement and concision**

Run: `rg -n "^## 30-Second Path Picker$|^## Repository Layout$" README.md && sed -n '1,250p' README.md`
Expected: quick picker appears before repository layout and does not introduce new command families.

### Task 3: Run Required Verification Suite

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Rust workspace checks**

Run: `cargo test --workspace`
Expected: Exit 0.

- [ ] **Step 2: Python checks**

Run: `./.venv/bin/pytest python/tests -q`
Expected: Exit 0.

- [ ] **Step 3: Node checks**

Run: `corepack pnpm --dir packages/node test`
Expected: Exit 0.

- [ ] **Step 4: MCP checks**

Run: `corepack pnpm --dir packages/mcp test`
Expected: Exit 0.

- [ ] **Step 5: Confirm diff scope**

Run: `git status --short --branch`
Expected: only v1.6 docs files and README changes.

### Task 4: Commit Ordered Documentation Milestones

**Files:**
- Create: `docs/superpowers/specs/2026-03-19-v1-6-quick-path-picker-design.md`
- Create: `docs/superpowers/plans/2026-03-19-v1-6-quick-path-picker-implementation-plan.md`
- Modify: `README.md`

- [ ] **Step 1: Commit spec**

Run:
```bash
git add docs/superpowers/specs/2026-03-19-v1-6-quick-path-picker-design.md
git commit -m "docs(spec): add v1.6 quick path picker design"
```
Expected: spec commit created.

- [ ] **Step 2: Commit plan**

Run:
```bash
git add docs/superpowers/plans/2026-03-19-v1-6-quick-path-picker-implementation-plan.md
git commit -m "docs(plan): add v1.6 quick path picker implementation plan"
```
Expected: plan commit created.

- [ ] **Step 3: Commit README implementation**

Run:
```bash
git add README.md
git commit -m "docs(readme): add v1.6 quick path picker"
```
Expected: README commit created.

- [ ] **Step 4: Push and tag**

Run:
```bash
git push origin main
git tag v1.6-docs-stable HEAD
git push origin v1.6-docs-stable
```
Expected: remote main updated and new stable tag available.

- [ ] **Step 5: Capture completion history**

Run: `git log --oneline --decorate -10`
Expected: v1.6 spec/plan/readme commits and stable tag visible.
