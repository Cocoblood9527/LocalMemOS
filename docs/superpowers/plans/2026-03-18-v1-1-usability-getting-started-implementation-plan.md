# V1.1 Usability Getting Started Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update the root `README.md` so a first-time local developer can go from clone to verified local workspace without guessing missing setup steps.

**Architecture:** Keep this effort documentation-only and limited to the root `README.md`. Use the current repository files and already-working verification commands as the source of truth, then rewrite the README into a short linear path covering environment setup, first-time setup, verification, and current development notes.

**Tech Stack:** Markdown, Rust workspace metadata, Python `pyproject.toml`, package-level `package.json` files, current local verification commands

---

## Preconditions

- Follow the approved spec at `docs/superpowers/specs/2026-03-18-v1-1-usability-getting-started-design.md`.
- This plan only changes `README.md`.
- Do not add scripts, CI, new documentation files, or implementation changes in this round.
- Keep `README.md` compact and operational rather than turning it into a contributor handbook.

## Proposed File Structure

- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-18-v1-1-usability-getting-started-design.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

## Milestones

1. Confirm the exact first-time setup reality of the repository
2. Add explicit environment and bootstrap instructions to `README.md`
3. Reframe verification commands as a practical local verification path
4. Tighten local development notes without expanding beyond `v1`
5. Re-read the final README for brevity, accuracy, and scope control

## Task 1: Audit the Current Setup Path

**Files:**
- Modify: `README.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

- [ ] **Step 1: Write the failing usability checklist**

```md
- Treat this as a local working checklist for the executor, not as committed content
- README lists prerequisites but not the first-time setup sequence
- README references `./.venv/bin/pytest` without explaining `.venv`
- README does not explain how to install Node package dependencies
- README presents test commands, but not as a simple verification path
- README has development notes, but not a compact "how to continue safely" cue
```

- [ ] **Step 2: Verify the current README fails the checklist**

Run: `sed -n '1,260p' README.md`
Expected: The README shows current project scope and test commands, but does not yet provide a clear first-time bootstrap path

- [ ] **Step 3: Confirm the Python setup facts from the repository**

Run: `sed -n '1,200p' python/pyproject.toml`
Expected: Python packaging metadata exists, but README will still need to explain virtual environment creation and test dependency installation explicitly

- [ ] **Step 4: Confirm the Node and MCP package-manager facts from the repository**

Run: `sed -n '1,200p' packages/node/package.json`
Run: `sed -n '1,200p' packages/mcp/package.json`
Expected: Both packages define their own scripts and rely on `pnpm`, so the README should document package-level dependency installation rather than inventing a root workspace bootstrap

## Task 2: Add Environment Setup and First-Time Setup Sections

**Files:**
- Modify: `README.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

- [ ] **Step 1: Add a short `Environment Setup` section**

```md
## Environment Setup

You will need:
- Rust toolchain
- Python 3.10+
- Node.js
- Corepack
```

- [ ] **Step 2: Add a short `First-Time Setup` section with explicit local bootstrap commands**

```bash
python3 -m venv .venv
./.venv/bin/python -m pip install -U pip pytest maturin
./.venv/bin/python -m pip install -e python
corepack enable
corepack pnpm --dir packages/node install
corepack pnpm --dir packages/mcp install
```

- [ ] **Step 3: Add one short note explaining why these steps exist**

```md
- Python tests are run from `./.venv`
- The local Python package must be installed into the venv before running pytest
- Node and MCP packages manage dependencies independently
```

- [ ] **Step 4: Re-read the README setup sections for brevity**

Run: `sed -n '1,220p' README.md`
Expected: Setup guidance is explicit, short, and does not drift into platform-specific troubleshooting

- [ ] **Step 5: Commit**

```bash
git add README.md
git commit -m "docs(readme): add first-time local setup path"
```

## Task 3: Rewrite Verification as a Minimal Local Check Path

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Replace or extend the current `Build and Test` section so it reads like a check sequence**

```md
## Verification Path

Run the current local verification commands:
```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```
```

- [ ] **Step 2: Add one short line explaining what a healthy result means**

```md
If all four commands pass, the Rust core and the current Python, Node, and MCP access surfaces are in a healthy local state.
```

- [ ] **Step 3: Verify the README still points at the real command surface**

Run: `rg -n "cargo test --workspace|pytest python/tests|packages/node test|packages/mcp test" README.md`
Expected: The README contains the four current verification commands exactly once in the verification section

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): clarify local verification path"
```

## Task 4: Tighten Current Dev Workflow Notes

**Files:**
- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-18-v1-1-usability-getting-started-design.md`

- [ ] **Step 1: Add or refine a compact `Current Dev Workflow Notes` section**

```md
- `memory-core` remains the semantic source of truth
- thin adapters should follow core semantics instead of redefining them
- this repository is still intentionally `v1 MVP`
```

- [ ] **Step 2: Preserve the existing semantic caveats that still matter**

```md
- `RecallRequest.include_history` is reserved in `v1`
- `GET /facts/{id}/history` uses a stored `facts.id`
```

- [ ] **Step 3: Re-read for scope creep**

Run: `sed -n '1,260p' README.md`
Expected: The README gives safe development cues, but does not introduce contribution policy, roadmap detail, automation promises, or `v2` planning

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): add v1.1 workflow notes"
```

## Task 5: Final Accuracy Pass and End-to-End Verification

**Files:**
- Modify: `README.md`
- Reference only: `python/pyproject.toml`
- Reference only: `packages/node/package.json`
- Reference only: `packages/mcp/package.json`

- [ ] **Step 1: Re-read the entire README from top to bottom**

Run: `sed -n '1,320p' README.md`
Expected: The document still reads as one compact entry page with setup, verification, and workflow notes in a sensible order

- [ ] **Step 2: Verify the setup and dependency claims against the repository**

Run: `sed -n '1,200p' python/pyproject.toml`
Run: `sed -n '1,200p' packages/node/package.json`
Run: `sed -n '1,200p' packages/mcp/package.json`
Expected: README setup instructions still match the package surfaces that actually exist in the repo

- [ ] **Step 3: Run the documented verification commands**

Run: `cargo test --workspace`
Run: `./.venv/bin/pytest python/tests -q`
Run: `corepack pnpm --dir packages/node test`
Run: `corepack pnpm --dir packages/mcp test`
Expected: All commands pass, confirming the README documents a working local path

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): finalize v1.1 getting started guide"
```

## Acceptance Checklist

- [ ] `README.md` still acts as the single entry page for the repository
- [ ] a first-time developer can create the expected Python virtual environment without guessing
- [ ] Python dependency bootstrap is documented explicitly enough to run tests
- [ ] the local Python package install step is documented before pytest
- [ ] Node and MCP dependency installation is documented at the package level
- [ ] verification commands are presented as a practical local check path
- [ ] `memory-core` remains clearly described as the semantic source of truth
- [ ] `v1 MVP` scope boundaries remain visible
- [ ] no new scripts, CI, or extra documentation files are introduced
- [ ] no `v2` or roadmap scope is folded into this round

## Suggested Execution Order

1. Confirm the repository facts first so the README does not invent setup steps.
2. Add `Environment Setup` and `First-Time Setup`.
3. Rewrite verification into a simple local check path.
4. Tighten workflow notes while preserving existing semantic caveats.
5. Re-run the documented commands before closing the task.

## Handoff Notes

- If any documented setup command turns out to be inaccurate, fix the README to match the repository reality rather than adding workaround logic in this round.
- If the README starts growing into a full contributor guide, cut it back and keep only the minimum path needed for first-time local success.
- If a missing setup dependency is discovered that cannot be explained cleanly in `README.md`, stop and decide in a separate design round whether helper scripts or workflow automation are justified.
