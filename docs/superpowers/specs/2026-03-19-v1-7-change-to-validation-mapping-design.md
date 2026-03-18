# V1.7 README Change-to-Validation Mapping Design

## Overview

This document defines a small `v1.7` documentation project for `localmemos`.

The round remains documentation-first and adds a compact mapping from change type to first-pass validation commands.

## Goal

Improve `README.md` so maintainers can choose an initial validation set based on the files they touched, while preserving the existing full regression fallback and command truth.

This round is documentation-only.

## Problem Statement

`v1.6` introduced a faster role/path entry point, but there is still no explicit path-based command mapping for first-pass validation.

Current friction:

- maintainers can identify role quickly, but not always the smallest safe first check for a concrete change set
- users may jump directly to full regression for every tiny change (slow) or skip relevant checks (risky)
- change-scope-to-validation guidance is implicit across sections, not explicit in one compact table

The missing piece is a lightweight mapping table that keeps command truth centralized and makes first-pass choices obvious.

## Scope

Implementation output in this round updates only `README.md`.

It should add:

- one compact `Change-to-Validation Mapping` section
- explicit mapping from common touched paths/change types to first-pass checks
- explicit fallback rule to run full `Minimum Regression Order` when unsure or before closure

It should not add:

- code changes
- scripts or automation
- CI workflows
- new command families
- v2 scope content

## Constraints

The README must remain:

- concise and operational
- aligned with current repository behavior
- consistent with `memory-core` semantic authority
- within `v1 MVP` boundaries

All commands referenced must be existing commands already present in README.

## Option Analysis

### Option 1: Path-to-first-pass table + mandatory full fallback rule (recommended)

Pros:

- fastest mapping from touched area to first action
- lowers over-testing and under-testing risk
- keeps existing command truth and escalation policy intact

Cons:

- requires concise phrasing to avoid policy ambiguity

### Option 2: Keep role-only guidance

Pros:

- no new section

Cons:

- slower operational decisions for concrete file-level changes

### Option 3: Full decision tree with many branches

Pros:

- high detail

Cons:

- too heavy for current README style
- higher maintenance overhead

## Recommended Design

Use Option 1.

Add a new section after `30-Second Path Picker`:

- `## Change-to-Validation Mapping`

### Section Structure

Use one compact table with columns:

- `If you changed...`
- `First-pass checks`
- `Then`

Recommended row coverage:

1. `README.md` and other docs-only workflow edits
2. `crates/memory-core/**`
3. `crates/memory-http/**`
4. `python/**`
5. `packages/node/**`
6. `packages/mcp/**`
7. mixed or uncertain multi-surface changes

### Command Policy

Allowed commands are existing commands only:

- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

The section must include:

- first-pass subsets where appropriate
- explicit fallback: run full `Minimum Regression Order` before final closure, and immediately when scope is mixed/unclear
- explicit reference to `Failure Triage Rule` for failures and ambiguity

## Writing Style

The section should be:

- compact and operational
- explicit about order when using multi-command rows
- free of theory-heavy narrative

## Success Criteria

This round is successful if a maintainer can:

- map touched files to a practical first-pass check in one scan
- know when to escalate to full `Minimum Regression Order`
- preserve command and triage consistency with existing README policy

## Non-Goals

This round does not aim to:

- replace existing `Verification Path` or `Minimum Regression Order`
- reduce required confidence checks for release/closure
- alter runtime semantics
- introduce automation or CI

## Follow-On Work

Possible future rounds:

1. collect empirical usage feedback and tune row granularity
2. optional lightweight docs consistency pass once v1 docs sequence stabilizes
