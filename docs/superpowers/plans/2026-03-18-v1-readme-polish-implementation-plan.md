# V1 README Polish Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite the root README into a compact `v1` operational guide for future maintenance and continued development.

**Architecture:** Keep this effort scoped to documentation only. Update the root `README.md` so it becomes a high-signal project entry point, while treating the existing design and plan documents as the deeper sources of truth. Avoid turning the README into a second architecture spec or public launch page.

**Tech Stack:** Markdown, existing repository docs, current build/test commands

---

## Preconditions

- Follow the approved spec at `docs/superpowers/specs/2026-03-18-v1-readme-polish-design.md`.
- This plan only changes documentation, primarily `README.md`.
- Do not fold CI setup, release automation, or new feature work into this plan.

## Proposed File Structure

- Modify: `README.md`
- Reference only: `docs/superpowers/specs/2026-03-18-local-agent-memory-design.md`
- Reference only: `docs/superpowers/specs/2026-03-18-v1-readme-polish-design.md`
- Reference only: `docs/superpowers/plans/2026-03-18-local-agent-memory-implementation-plan.md`

## Milestones

1. Restructure README around `v1` operational needs
2. Add concrete prerequisites and verification commands
3. Add current scope, runtime paths, and development notes
4. Verify the README matches the current repository reality

## Task 1: Restructure the README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Write the failing content checklist**

```md
- Missing concise project summary
- Missing current `v1` status section
- Missing repository layout section
- Missing explicit prerequisites section
- Missing concrete build/test commands
- Missing runtime path explanation
- Missing development notes section
```

- [ ] **Step 2: Verify the current README fails the checklist**

Run: `sed -n '1,220p' README.md`
Expected: The README is too short to satisfy the approved section structure

- [ ] **Step 3: Rewrite the README structure**

```md
# localmemos

## Project Summary
## Current Status
## Repository Layout
## Prerequisites
## Build and Test
## Runtime Paths
## Development Notes
```

- [ ] **Step 4: Keep the README compact**

```md
- Prefer short paragraphs and short bullets
- Do not duplicate large sections from the design spec
- Do not add roadmap, CI, contribution guide, or detailed API reference
```

- [ ] **Step 5: Re-read the README for section completeness**

Run: `sed -n '1,260p' README.md`
Expected: All seven target sections exist and remain concise

- [ ] **Step 6: Commit**

```bash
git add README.md
git commit -m "docs(readme): restructure root readme for v1 maintenance"
```

## Task 2: Add Accurate Current Status and Scope Boundaries

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add the `v1` support list**

```md
Supported in `v1`:
- Rust `memory-core`
- SQLite storage
- explicit fact writes
- exact recall
- FTS fallback
- time-aware history
- HTTP, Python, Node, and MCP access paths
```

- [ ] **Step 2: Add the explicit out-of-scope list**

```md
Out of scope for `v1`:
- automatic extraction
- vector retrieval
- vector databases
- graph-native memory
- cloud sync
- multi-user support
```

- [ ] **Step 3: Verify this matches the approved design**

Run: `rg -n "automatic extraction|vector|graph|cloud|multi-user" docs/superpowers/specs/2026-03-18-local-agent-memory-design.md README.md`
Expected: README boundaries align with the main `v1` design

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): clarify v1 scope and non-goals"
```

## Task 3: Add Repository Layout and Prerequisites

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add a short repository layout section**

```md
- `crates/memory-core`: core storage and recall semantics
- `crates/memory-http`: HTTP adapter
- `crates/memory-node`: native Node binding
- `packages/node`: Node SDK
- `packages/mcp`: MCP adapter
- `python`: Python binding and SDK
- `docs/superpowers`: design and planning docs
```

- [ ] **Step 2: Add the prerequisites section**

```md
Requirements:
- Rust toolchain
- Python 3
- Node.js
- Corepack
```

- [ ] **Step 3: Cross-check the listed paths exist**

Run: `find crates packages python docs/superpowers -maxdepth 2 -type d | sort`
Expected: All directories named in the README exist in the repository

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs(readme): document repository layout and prerequisites"
```

## Task 4: Add Concrete Verification Commands and Runtime Notes

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add current verification commands**

```md
```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```
```

- [ ] **Step 2: Add the runtime path explanations**

```md
- Python SDK: embedded local access from Python agents
- Node SDK: embedded local access from Node tools
- HTTP: process-external local integration
- MCP: tool-based integration for agent environments
```

- [ ] **Step 3: Add the development notes**

```md
- `memory-core` is the single source of truth for semantics
- `RecallRequest.include_history` is reserved in `v1`
- `/facts/{id}/history` expects a stored `facts.id`
```

- [ ] **Step 4: Verify commands and notes against current repo reality**

Run: `sed -n '1,260p' README.md`
Expected: Commands are runnable and notes match the implemented behavior

- [ ] **Step 5: Commit**

```bash
git add README.md
git commit -m "docs(readme): add verification commands and runtime notes"
```

## Acceptance Checklist

- [ ] `README.md` is still compact and easy to scan
- [ ] the README clearly states that the project is a `v1 MVP`
- [ ] implemented `v1` scope is explicit
- [ ] non-goals are explicit
- [ ] repository layout is documented
- [ ] prerequisites are documented
- [ ] current verification commands are documented
- [ ] runtime access paths are documented
- [ ] critical `v1` semantic caveats are documented
- [ ] the README does not turn into a second architecture spec

## Suggested Execution Order

1. Rewrite the README structure first.
2. Add current status and scope boundaries.
3. Add repository layout and prerequisites.
4. Add test commands, runtime notes, and semantic caveats.
5. Re-read the full README once at the end for brevity and consistency.

## Handoff Notes

- If the README starts growing beyond a compact maintenance entry page, stop and move deeper material into `docs/superpowers/` references instead.
- If build or test commands drift from the actual repository behavior, update the commands rather than keeping stale instructions.
- Do not introduce CI, release notes, roadmap, or contribution workflow into this README-focused phase.
