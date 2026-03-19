# V1.0.0 Release Closure Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Produce formal `v1.0.0` release closure artifacts, verify repository health, and publish a stable tag plus GitHub release.

**Architecture:** Keep runtime unchanged. Add two release documents under `docs/superpowers/releases/` (readiness evidence and release notes), run the required verification suite, commit in docs-first order, then publish `v1.0.0` tag and GitHub release from current stable main.

**Tech Stack:** Markdown docs, git tags, GitHub CLI (`gh`), Rust/Python/Node/MCP test commands.

---

### Task 1: Baseline and Release Preconditions

**Files:**
- Reference: `docs/superpowers/specs/2026-03-19-v1-0-0-release-closure-design.md`

- [ ] **Step 1: Confirm clean branch and remote sync**

Run: `git status --short --branch`
Expected: clean working tree on `main`.

- [ ] **Step 2: Confirm tag precondition**

Run: `git tag --list | sort`
Expected: no existing `v1.0.0` tag conflict.

- [ ] **Step 3: Confirm GitHub publishing capability**

Run: `gh auth status`
Expected: authenticated account with `repo` scope.

### Task 2: Create Release Documents

**Files:**
- Create: `docs/superpowers/releases/2026-03-19-v1-0-0-release-readiness.md`
- Create: `docs/superpowers/releases/v1.0.0-release-notes.md`

- [ ] **Step 1: Write release-readiness evidence document**

Include:
- `V1 Done Definition` checklist mapping
- scope/non-scope confirmation
- verification evidence section
- milestone/tag chain summary (`v1.1` to `v1.8`)

- [ ] **Step 2: Write release notes document**

Include:
- key delivered capabilities in `v1.0.0`
- explicit out-of-scope list
- verification baseline commands
- progression summary from stabilization rounds

- [ ] **Step 3: Validate docs presence**

Run: `ls -la docs/superpowers/releases && sed -n '1,260p' docs/superpowers/releases/2026-03-19-v1-0-0-release-readiness.md && sed -n '1,260p' docs/superpowers/releases/v1.0.0-release-notes.md`
Expected: both docs exist and are complete.

### Task 3: Run Required Verification Suite

**Files:**
- Release docs and spec/plan files

- [ ] **Step 1: Rust verification**

Run: `cargo test --workspace`
Expected: Exit 0.

- [ ] **Step 2: Python verification**

Run: `./.venv/bin/pytest python/tests -q`
Expected: Exit 0.

- [ ] **Step 3: Node verification**

Run: `corepack pnpm --dir packages/node test`
Expected: Exit 0.

- [ ] **Step 4: MCP verification**

Run: `corepack pnpm --dir packages/mcp test`
Expected: Exit 0.

### Task 4: Commit Release Artifacts

**Files:**
- Create: `docs/superpowers/specs/2026-03-19-v1-0-0-release-closure-design.md`
- Create: `docs/superpowers/plans/2026-03-19-v1-0-0-release-closure-implementation-plan.md`
- Create: `docs/superpowers/releases/2026-03-19-v1-0-0-release-readiness.md`
- Create: `docs/superpowers/releases/v1.0.0-release-notes.md`

- [ ] **Step 1: Commit spec**

Run:
```bash
git add docs/superpowers/specs/2026-03-19-v1-0-0-release-closure-design.md
git commit -m "docs(spec): add v1.0.0 release closure design"
```

- [ ] **Step 2: Commit plan**

Run:
```bash
git add docs/superpowers/plans/2026-03-19-v1-0-0-release-closure-implementation-plan.md
git commit -m "docs(plan): add v1.0.0 release closure implementation plan"
```

- [ ] **Step 3: Commit release docs**

Run:
```bash
git add docs/superpowers/releases/2026-03-19-v1-0-0-release-readiness.md docs/superpowers/releases/v1.0.0-release-notes.md
git commit -m "docs(release): add v1.0.0 readiness and release notes"
```

### Task 5: Tag and Publish Release

**Files:**
- Uses committed release notes file

- [ ] **Step 1: Push main**

Run: `git push origin main`
Expected: remote main updated.

- [ ] **Step 2: Create and push tag**

Run:
```bash
git tag v1.0.0 HEAD
git push origin v1.0.0
```
Expected: remote tag created.

- [ ] **Step 3: Publish GitHub release**

Run:
```bash
gh release create v1.0.0 \
  --title "v1.0.0" \
  --notes-file docs/superpowers/releases/v1.0.0-release-notes.md
```
Expected: public release entry created from committed notes.

- [ ] **Step 4: Capture closure summary**

Run: `git status --short --branch && git log --oneline --decorate -12 && gh release view v1.0.0`
Expected: clean branch, visible tag, and release metadata available.
