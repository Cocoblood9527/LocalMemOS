# V1.4 Command Drift Guard Design

## Overview

This document defines a small `v1.4` documentation project for `localmemos`.

The focus is command-drift prevention for the existing README workflows, while keeping the project in a lightweight, documentation-first mode.

## Goal

Improve the root `README.md` so maintainers can quickly decide:

- when they must rerun verification commands
- what minimum regression command order to run first
- how to classify a failure as documentation drift vs environment issue vs real code regression

This round is documentation-only.

## Problem Statement

`v1.3` already provides a clear command matrix and quick recovery per command, but drift risk remains over time:

- repository changes may silently invalidate README assumptions
- maintainers may rerun too many commands (slow) or too few (unsafe)
- failures may be misclassified, causing documentation drift to remain unresolved

The missing piece is a compact policy in README for trigger conditions and minimum regression flow.

## Scope

Implementation output in this round updates only `README.md`.

It should add:

- a compact `When To Re-Validate Commands` section
- a compact `Minimum Regression Order` section
- a compact `Failure Triage Rule` note

It should not add:

- code changes
- scripts or automation
- CI workflows
- extra user-facing documentation files outside workflow artifacts
- Windows-specific guidance
- major-version planning content

## Constraints

The README must remain:

- concise and operational
- aligned with current repository behavior
- consistent with `v1 MVP` boundaries

This round should avoid repeating full command explanations already covered by the validation matrix.

## Option Analysis

### Option 1: README-only trigger + regression checklist (recommended)

Pros:

- smallest scope
- immediate practical value
- no new maintenance surface

Cons:

- still manual execution

### Option 2: Separate drift-check document

Pros:

- more room for detail

Cons:

- splits the entry point
- increases navigation overhead for a small project

### Option 3: Semi-automated check script (no CI)

Pros:

- stronger enforcement

Cons:

- scope creep beyond documentation-first intent
- ongoing script maintenance burden

## Recommended Design

Use Option 1.

Add three small README sections that connect directly to existing `Verification Path` and `Clean-Run Validation Matrix`.

Required heading names:

- `## When To Re-Validate Commands`
- `## Minimum Regression Order`
- `## Failure Triage Rule`

### 1. When To Re-Validate Commands

Add a short trigger list such as:

- after dependency changes (`Cargo.toml`, `pyproject.toml`, package manifests/lockfiles)
- after tooling/runtime changes (Rust/Python/Node updates)
- after touching command-bearing docs in README
- after changes in crates/packages that affect build/test surfaces

Each trigger should point to rerunning at least the minimum regression flow.

### 2. Minimum Regression Order

Add a short ordered fallback flow for fast confidence checks:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

Clarify this is the minimum default order and may be expanded when needed.

### 3. Failure Triage Rule

Add a compact triage rule:

- if command in README no longer matches repository reality, treat as documentation drift and update README
- if command is valid but local machine state is inconsistent, treat as environment issue and apply matrix recovery
- if command and environment are both valid but tests fail, treat as code regression and escalate accordingly

This should be written as a short decision aid, not a long troubleshooting tree.

## Placement

Recommended insertion in README:

- place `When To Re-Validate Commands` and `Minimum Regression Order` after `Verification Path`
- keep `Clean-Run Validation Matrix` and `Common Setup Failures` as execution and recovery references
- place `Failure Triage Rule` near the new trigger/regression sections

## Writing Style

Use short operational language:

- short bullet points
- explicit command names
- no theory-heavy prose

## Success Criteria

This round is successful if a maintainer can:

- identify when command re-validation is required
- run a minimum safe regression sequence without guesswork
- classify failures into doc drift, environment issue, or code regression
- confirm the four commands in `Minimum Regression Order` stay identical to `Verification Path`

## Non-Goals

This round does not aim to:

- automate drift detection
- replace existing matrix/troubleshooting sections
- introduce CI policy
- change runtime behavior

## Follow-On Work

Possible future rounds:

1. optional script-based local drift precheck
2. CI guardrails for README command drift
3. broader future-scope architecture work after `v1` stabilization
