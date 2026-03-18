# V1.5 Role-Based Operation Paths Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a concise role-based operation-path section to `README.md` so maintainers can choose the right v1 workflow path quickly without changing semantics or scope.

**Architecture:** Keep `README.md` as the single operator entry point. Add one compact role-first section that routes readers to existing sections (`First-Time Setup`, `Verification Path`, command drift guard, and validation matrix) instead of duplicating instructions. Preserve `memory-core` semantic authority and reuse existing verification command order unchanged.

**Tech Stack:** Markdown documentation, existing Rust/Python/Node/MCP test commands.

---

### Task 1: Prepare README Placement and Content Boundaries

**Files:**
- Modify: `README.md`
- Reference: `docs/superpowers/specs/2026-03-19-v1-5-role-based-operation-paths-design.md`

- [ ] **Step 1: Re-read current README workflow sections**

Run: `sed -n '1,260p' README.md`
Expected: Current `Current Status`, `Repository Layout`, and validation sections are visible for insertion planning.

- [ ] **Step 2: Confirm role-path section insertion point**

Run: `rg -n "^## (Current Status|Repository Layout|Verification Path|When To Re-Validate Commands|Minimum Regression Order|Failure Triage Rule)$" README.md`
Expected: Line anchors found to place `## Role-Based Operation Paths` after `Current Status`.

- [ ] **Step 3: Lock v1 boundaries before editing**

Run: `rg -n "single semantic source of truth|v1 MVP|out of scope|memory-core" README.md`
Expected: Existing v1 scope and semantic-authority wording present and ready to be referenced, not redefined.

### Task 2: Implement Role-Based Operation Paths in README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add `## Role-Based Operation Paths` section**

Edit `README.md` to add a compact role-routing section between `Current Status` and `Repository Layout`.

- [ ] **Step 2: Add four role paths with consistent schema**

For each role (`Docs Maintainer`, `Core Semantics Maintainer`, `Adapter Surface Maintainer`, `Clean-Run Verifier`), include:
- `Use this path when...`
- `Read first...` (link to existing README sections)
- `Run at minimum...` (existing minimum regression order only)
- `Escalate when...` (map to `Failure Triage Rule`)

- [ ] **Step 3: Add explicit safety default**

Include one line: if role is unclear, use `Clean-Run Verifier` path first, then apply `Failure Triage Rule`.

- [ ] **Step 4: Verify README remains concise and non-duplicative**

Run: `rg -n "^## Role-Based Operation Paths$|^## Repository Layout$" README.md && sed -n '1,220p' README.md`
Expected: New section present at intended location, with no new command families or v2 content.

### Task 3: Validate and Regress with Existing Command Truth

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Run Rust workspace verification**

Run: `cargo test --workspace`
Expected: Exit 0 with all workspace tests passing.

- [ ] **Step 2: Run Python verification**

Run: `./.venv/bin/pytest python/tests -q`
Expected: Exit 0 with all Python tests passing.

- [ ] **Step 3: Run Node package verification**

Run: `corepack pnpm --dir packages/node test`
Expected: Exit 0 with vitest passing.

- [ ] **Step 4: Run MCP package verification**

Run: `corepack pnpm --dir packages/mcp test`
Expected: Exit 0 with vitest passing.

- [ ] **Step 5: Confirm only intended docs changes exist**

Run: `git status --short --branch`
Expected: Only v1.5 docs files and README modifications appear.

### Task 4: Commit Documentation Milestones in Order

**Files:**
- Create: `docs/superpowers/specs/2026-03-19-v1-5-role-based-operation-paths-design.md`
- Create: `docs/superpowers/plans/2026-03-19-v1-5-role-based-operation-paths-implementation-plan.md`
- Modify: `README.md`

- [ ] **Step 1: Commit spec document**

Run:
```bash
git add docs/superpowers/specs/2026-03-19-v1-5-role-based-operation-paths-design.md
git commit -m "docs(spec): add v1.5 role-based operation paths design"
```
Expected: Spec commit recorded on `main`.

- [ ] **Step 2: Commit implementation plan document**

Run:
```bash
git add docs/superpowers/plans/2026-03-19-v1-5-role-based-operation-paths-implementation-plan.md
git commit -m "docs(plan): add v1.5 role-based operation paths implementation plan"
```
Expected: Plan commit recorded after spec commit.

- [ ] **Step 3: Commit README implementation**

Run:
```bash
git add README.md
git commit -m "docs(readme): add v1.5 role-based operation paths"
```
Expected: README commit recorded after plan commit.

- [ ] **Step 4: Capture final branch summary**

Run: `git log --oneline --decorate -6`
Expected: New v1.5 spec/plan/readme commits visible at top of history.
