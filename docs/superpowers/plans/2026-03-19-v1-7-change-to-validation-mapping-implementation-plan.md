# V1.7 Change-to-Validation Mapping Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a compact README table that maps changed areas to first-pass validation checks, with a clear fallback to full regression order.

**Architecture:** Update only `README.md` by inserting `Change-to-Validation Mapping` after the quick picker. Reuse existing commands and sections as single source of command truth. Keep policy explicit: first-pass checks are directional, but full `Minimum Regression Order` remains fallback when unsure and closure baseline.

**Tech Stack:** Markdown documentation, existing Rust/Python/Node/MCP verification commands.

---

### Task 1: Validate Current Anchors and Command Truth

**Files:**
- Modify: `README.md`
- Reference: `docs/superpowers/specs/2026-03-19-v1-7-change-to-validation-mapping-design.md`

- [ ] **Step 1: Inspect existing quick picker and adjacent sections**

Run: `rg -n "^## (30-Second Path Picker|Repository Layout|Verification Path|Minimum Regression Order|Failure Triage Rule)$" README.md && sed -n '1,260p' README.md`
Expected: insertion point after `30-Second Path Picker` and before `Repository Layout` is clear.

- [ ] **Step 2: Confirm command set to be reused**

Run: `sed -n '/## Verification Path/,/## When To Re-Validate Commands/p' README.md`
Expected: canonical four-command sequence present and unchanged.

### Task 2: Implement Mapping Table in README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add `## Change-to-Validation Mapping` heading**

Insert section immediately after `30-Second Path Picker`.

- [ ] **Step 2: Add compact mapping table**

Add rows for:
- docs-only edits
- `crates/memory-core/**`
- `crates/memory-http/**`
- `python/**`
- `packages/node/**`
- `packages/mcp/**`
- mixed/unclear change sets

Use existing commands only.

- [ ] **Step 3: Add fallback and triage rule sentence**

Include explicit line:
- run full `Minimum Regression Order` before closing a round
- if scope is mixed/unclear or failure classification is unclear, use `Failure Triage Rule`

- [ ] **Step 4: Validate placement and non-drift**

Run: `rg -n "^## Change-to-Validation Mapping$|^## Repository Layout$" README.md && sed -n '1,300p' README.md`
Expected: new section appears in intended location and command text is consistent.

### Task 3: Run Required Verification Suite (Sequential)

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

- [ ] **Step 5: Confirm change scope**

Run: `git status --short --branch`
Expected: only v1.7 docs + README changes.

### Task 4: Commit, Push, and Tag

**Files:**
- Create: `docs/superpowers/specs/2026-03-19-v1-7-change-to-validation-mapping-design.md`
- Create: `docs/superpowers/plans/2026-03-19-v1-7-change-to-validation-mapping-implementation-plan.md`
- Modify: `README.md`

- [ ] **Step 1: Commit spec**

Run:
```bash
git add docs/superpowers/specs/2026-03-19-v1-7-change-to-validation-mapping-design.md
git commit -m "docs(spec): add v1.7 change-to-validation mapping design"
```

- [ ] **Step 2: Commit plan**

Run:
```bash
git add docs/superpowers/plans/2026-03-19-v1-7-change-to-validation-mapping-implementation-plan.md
git commit -m "docs(plan): add v1.7 change-to-validation mapping implementation plan"
```

- [ ] **Step 3: Commit README implementation**

Run:
```bash
git add README.md
git commit -m "docs(readme): add v1.7 change-to-validation mapping"
```

- [ ] **Step 4: Push and create stable tag**

Run:
```bash
git push origin main
git tag v1.7-docs-stable HEAD
git push origin v1.7-docs-stable
```

- [ ] **Step 5: Capture final history**

Run: `git log --oneline --decorate -12`
Expected: v1.7 spec/plan/readme commits and stable tag visible.
