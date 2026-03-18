# V1.2 Usability Clean-Run Troubleshooting Design

## Overview

This document defines a small `v1.2 usability` documentation project for `localmemos`.

This round keeps the same documentation-first direction as `v1.1`, and focuses on making the existing `README.md` more robust for first-time local execution on both macOS and Linux.

## Goal

Improve the root `README.md` so a developer can:

- follow a clean-run path on macOS or Linux
- recognize common setup or test failures quickly
- apply minimal corrective actions without leaving the README

This round remains documentation-only.

## Problem Statement

`v1.1` already added a clear setup and verification path, but the README still assumes a smooth environment.

In real first-time runs, common failures can still occur, for example:

- virtual environment created but package not installed editable
- `corepack` not enabled before `pnpm` commands
- running commands from the wrong directory
- stale local artifacts causing confusing test behavior

Without a compact troubleshooting section, a developer can still get blocked even with correct baseline instructions.

## Target Reader

Primary reader:

- the project owner returning to the repo
- a collaborator or coding model continuing work
- any maintainer who needs a fast local unblock path

This is not an end-user installation guide.

## Scope

Implementation output in this round updates only `README.md`.

It should add:

- a compact clean-run checklist for macOS and Linux
- a compact troubleshooting / FAQ section for common local failures
- explicit notes about command location and expected local context

It should not add:

- code changes
- scripts or automation
- CI workflows
- extra user-facing documentation files outside the normal `docs/superpowers` workflow artifacts
- Windows setup support in this round
- major-version planning content

## Constraints

The resulting README must remain:

- short and operational
- easy to maintain
- aligned with current repository reality
- consistent with `v1 MVP` boundaries

The README must stay as the single entry page in this round.

## Approach Options

### Option 1: README-only clean-run + troubleshooting (recommended)

Add two compact sections in README:

- `Clean-Run Checklist (macOS + Linux)`
- `Common Setup Failures`

Pros:

- fastest path to unblock
- no doc sprawl
- consistent with current documentation strategy

Cons:

- README becomes slightly longer

### Option 2: Split troubleshooting into separate document

Pros:

- keeps README shorter

Cons:

- adds navigation overhead
- not needed yet for current project size

### Option 3: Add helper scripts to auto-repair setup

Pros:

- could reduce manual steps

Cons:

- implementation scope expansion
- maintenance overhead
- violates this round's documentation-only scope

## Recommended Design

Use Option 1.

Keep existing `Environment Setup`, `First-Time Setup`, and `Verification Path`, then add two short sections.

Insertion points in `README.md`:

- insert `Clean-Run Checklist (macOS + Linux)` immediately after `First-Time Setup`
- insert `Common Setup Failures` immediately after `Verification Path`

### 1. Clean-Run Checklist (macOS + Linux)

Add a short, linear checklist that states:

1. run commands from repository root
2. ensure `.venv` exists and is active via explicit path calls
3. ensure `corepack enable` has been run
4. run package installs for both `packages/node` and `packages/mcp`
5. execute the four verification commands in order

This checklist should be short enough that a maintainer can execute it quickly before deeper debugging.

### 2. Common Setup Failures

Add a compact markdown table with:

- symptom
- likely cause
- minimal fix command(s)

Initial target failures:

- `ModuleNotFoundError: memory_sdk`
- `pnpm: command not found` or corepack-related failure
- test command run from wrong directory
- stale local artifacts causing inconsistent local results
- native build failure due to missing Rust toolchain in PATH

Fixes must stay short and repository-consistent.

### 3. Context Notes

Add one short reminder:

- README commands assume repository root as working directory

Add one short reminder:

- if local state looks inconsistent, rerun setup commands before deeper debugging

## Writing Style

Style remains operational:

- short sections
- direct commands
- no long theory
- no platform marketing language

## Success Criteria

This round is successful if a first-time developer on macOS or Linux can:

- complete setup from README alone
- recover from the most common local setup failures without external docs
- run the full verification path successfully

## Non-Goals

This round does not aim to:

- support Windows setup
- automate environment repair
- redesign repository documentation architecture
- change runtime behavior
- introduce major-version planning content

## Follow-On Work

Possible follow-on (separate rounds):

1. lightweight setup validation script (if troubleshooting volume remains high)
2. CI-based docs command verification
3. broader future-scope design work after `v1` polish is stable
