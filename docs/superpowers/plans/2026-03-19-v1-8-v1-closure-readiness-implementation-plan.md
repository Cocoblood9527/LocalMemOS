# V1.8 V1 Closure Readiness Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Finalize v1 closure readiness by adding explicit completion/closure sections in README and strengthening cross-adapter `forget` semantic consistency tests.

**Architecture:** Keep runtime behavior unchanged. Add two concise README policy sections and focused adapter-level tests that verify `forget` retires facts from active recall/list paths consistently across HTTP, Python, Node, and MCP surfaces. Preserve existing command truth and v1 boundaries.

**Tech Stack:** Markdown docs, Rust tests (`cargo test`), Python pytest, Node/MCP vitest.

---

### Task 1: Prepare Context and Section Anchors

**Files:**
- Modify: `README.md`
- Reference: `docs/superpowers/specs/2026-03-19-v1-8-v1-closure-readiness-design.md`

- [ ] **Step 1: Locate insertion points**

Run: `rg -n "^## (Current Status|Role-Based Operation Paths|Verification Path|Minimum Regression Order|Failure Triage Rule|Current Dev Workflow Notes)$" README.md`
Expected: heading anchors found for concise insertion without restructuring.

- [ ] **Step 2: Confirm command truth remains canonical**

Run: `sed -n '/## Verification Path/,/## When To Re-Validate Commands/p' README.md`
Expected: four-command canonical block present and unchanged.

### Task 2: Implement README Closure Sections

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add `## V1 Done Definition`**

Include explicit closure criteria:
- v1 scope boundaries unchanged
- memory-core semantic authority unchanged
- adapter behavior aligned to core primitives
- required four-command verification passes

- [ ] **Step 2: Add `## Release Closure Checklist`**

Include ordered checklist:
1. run four required verification commands in order
2. apply `Failure Triage Rule` if any fail
3. commit ordering discipline (`spec -> plan -> implementation`)
4. push `main`
5. create and push stable tag

- [ ] **Step 3: Validate placement and readability**

Run: `rg -n "^## V1 Done Definition$|^## Release Closure Checklist$|^## Repository Layout$" README.md && sed -n '1,320p' README.md`
Expected: both sections present and concise, without command drift.

### Task 3: Add Cross-Adapter Forget Consistency Tests

**Files:**
- Modify: `python/tests/test_sdk_core.py`
- Modify: `packages/node/test/sdk.test.ts`
- Modify: `packages/mcp/test/tools.test.ts`
- Modify: `crates/memory-http/tests/http_smoke.rs`

- [ ] **Step 1: Python adapter forget behavior test**

Add test case:
- upsert fact
- call `forget`
- assert `recall` for same key returns no active facts

- [ ] **Step 2: Node adapter forget behavior assertion**

Extend or add node test case:
- upsert fact
- call `forget`
- assert follow-up `recall` for same key returns empty facts

- [ ] **Step 3: MCP adapter forget behavior assertion**

Extend MCP tools test:
- upsert fact
- call `memory_forget`
- assert `memory_recall` for same key returns empty facts

- [ ] **Step 4: HTTP adapter forget behavior assertion**

Extend HTTP smoke test:
- upsert fact
- call `/facts:forget`
- assert subsequent `/facts:recall` for same key returns empty facts

### Task 4: Run Required Verification Suite

**Files:**
- Modify: README and test files above

- [ ] **Step 1: Rust workspace verification**

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

- [ ] **Step 5: Confirm change scope**

Run: `git status --short --branch`
Expected: only intended v1.8 docs/tests and README changes.

### Task 5: Commit, Push, Tag, and Record

**Files:**
- Create: `docs/superpowers/specs/2026-03-19-v1-8-v1-closure-readiness-design.md`
- Create: `docs/superpowers/plans/2026-03-19-v1-8-v1-closure-readiness-implementation-plan.md`
- Modify: `README.md`
- Modify: test files listed in Task 3

- [ ] **Step 1: Commit spec**

Run:
```bash
git add docs/superpowers/specs/2026-03-19-v1-8-v1-closure-readiness-design.md
git commit -m "docs(spec): add v1.8 v1 closure readiness design"
```

- [ ] **Step 2: Commit plan**

Run:
```bash
git add docs/superpowers/plans/2026-03-19-v1-8-v1-closure-readiness-implementation-plan.md
git commit -m "docs(plan): add v1.8 v1 closure readiness implementation plan"
```

- [ ] **Step 3: Commit README closure updates**

Run:
```bash
git add README.md
git commit -m "docs(readme): add v1 done definition and release closure checklist"
```

- [ ] **Step 4: Commit forget consistency tests**

Run:
```bash
git add python/tests/test_sdk_core.py packages/node/test/sdk.test.ts packages/mcp/test/tools.test.ts crates/memory-http/tests/http_smoke.rs
git commit -m "test: add forget consistency checks across adapters"
```

- [ ] **Step 5: Push and tag stable milestone**

Run:
```bash
git push origin main
git tag v1.8-docs-stable HEAD
git push origin v1.8-docs-stable
```

- [ ] **Step 6: Capture final history**

Run: `git log --oneline --decorate -14`
Expected: v1.8 milestone commits and new stable tag visible.
