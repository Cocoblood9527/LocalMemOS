# V1.3 Usability Clean-Run Validation Matrix Design

## Overview

This document defines a small `v1.3 usability` documentation project for `localmemos`.

This round keeps the documentation-first direction from `v1.1` and `v1.2`, and tightens execution clarity by introducing a validation matrix in the root README.

## Goal

Improve `README.md` so a maintainer can run a clean local setup on macOS or Linux and quickly determine whether each step is successful.

The core output is a compact `command + expected result` matrix, not new automation.

## Problem Statement

`v1.2` added setup steps, checklist guidance, and common failure recovery. However, the README still relies on interpretation when deciding whether a command has succeeded.

Typical friction points:

- command runs but success criteria are unclear
- recovery actions are present but not mapped tightly to specific command failures
- setup and verification instructions are readable, but not audit-friendly as a single execution view

## Scope

Implementation output in this round updates only `README.md`.

It should add:

- one compact `Clean-Run Validation Matrix (macOS + Linux)` section
- explicit expected-result criteria per command
- direct link between command failure and quick recovery

It should refine:

- relationship between existing setup/checklist content and troubleshooting content

It should not add:

- code changes
- scripts or automation
- CI workflows
- extra user-facing documentation files outside workflow artifacts
- Windows support
- major-version planning content

## Constraints

The README must remain:

- operational and concise
- accurate to current repository behavior
- consistent with existing `v1 MVP` boundaries

This round should not duplicate long command blocks across sections.

## Option Analysis

### Option 1: Human checklist only

Pros:

- shortest content footprint

Cons:

- weaker pass/fail signal
- low auditability

### Option 2: Command + expected-result matrix (recommended)

Pros:

- high execution clarity
- clear pass/fail interpretation
- easy for future maintainers and coding models to follow consistently

Cons:

- slightly longer than checklist-only format

### Option 3: Checklist + matrix

Pros:

- highest completeness

Cons:

- more content duplication risk
- reduced compactness

## Recommended Design

Use Option 2.

Add a new section in README:

- `Clean-Run Validation Matrix (macOS + Linux)`

Insertion point:

- place this section immediately after `Verification Path` and before `Common Setup Failures`

### Matrix Schema

Use a compact markdown table with four columns:

- `Command`
- `Expected Result`
- `If Failed`
- `Quick Recovery`

### Matrix Coverage

The matrix should cover the minimum local path end-to-end:

1. `python3 -m venv .venv`
2. `./.venv/bin/python -m pip install -U pip pytest maturin`
3. `./.venv/bin/python -m pip install -e python`
4. `corepack enable`
5. `corepack pnpm --dir packages/node install`
6. `corepack pnpm --dir packages/mcp install`
7. `cargo test --workspace`
8. `./.venv/bin/pytest python/tests -q`
9. `corepack pnpm --dir packages/node test`
10. `corepack pnpm --dir packages/mcp test`

Each row should define a practical success signal and one minimal recovery path.

### Relationship with Existing Sections

- Keep `First-Time Setup` as the canonical command sequence and source of truth for order
- Keep `Verification Path` as the short execution summary
- Use the validation matrix as a pass/fail and quick-recovery overlay, not as a replacement for setup order
- Keep `Common Setup Failures`, but reduce overlap by pointing readers to matrix recovery rows when applicable
- Keep root-directory assumption explicit

## Writing Style

The added section should:

- use short, concrete wording
- avoid explanatory prose where a success signal is enough
- avoid platform-specific detours beyond macOS/Linux shell assumptions

## Success Criteria

This round is successful if a maintainer can:

- execute clean-run steps in order
- determine pass/fail at each step without guesswork
- find immediate recovery guidance tied to the failed command
- complete the four verification commands successfully

## Non-Goals

This round does not aim to:

- automate clean-run validation
- replace troubleshooting content with exhaustive diagnostics
- change runtime semantics
- expand beyond current `v1` scope

## Follow-On Work

Possible future rounds:

1. optional lightweight script-based validation (separate scope decision)
2. CI verification for README command drift
3. broader future-scope architecture work after `v1` stabilization
