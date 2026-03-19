# V1.0.0 Release Closure Design

## Overview

This document defines the formal `v1.0.0` release-closure round for `localmemos`.

The round converts the current stable documentation and verification state into an explicit public release milestone.

## Goal

Complete `v1` formal release closure by delivering:

- a release-readiness evidence document
- a public GitHub release notes artifact
- a formal `v1.0.0` Git tag and GitHub Release

## Problem Statement

The repository has completed iterative `v1` stabilization rounds (`v1.1` through `v1.8`) with stable docs flow, verification discipline, and explicit scope boundaries. However, release closure is not yet formalized as one explicit public milestone.

Current gap:

- readiness evidence is distributed across commits and README sections
- no formal `v1.0.0` tag exists
- no consolidated public release notes entry exists on GitHub

## Scope

This round should:

- add one release-readiness document mapping completion criteria to current evidence
- add one release notes document used for GitHub release publication
- run required validation commands
- create and push `v1.0.0` tag
- publish GitHub release for `v1.0.0`

This round should not:

- add runtime features
- expand beyond `v1` scope boundaries
- introduce CI automation
- change API semantics

## Constraints

- `memory-core` remains the semantic source of truth.
- `v1` boundaries remain unchanged.
- Required verification commands must pass before release publication:
  - `cargo test --workspace`
  - `./.venv/bin/pytest python/tests -q`
  - `corepack pnpm --dir packages/node test`
  - `corepack pnpm --dir packages/mcp test`
- Release artifacts must be reproducible from repository history.

## Option Analysis

### Option 1: Tag-only release

Pros:

- fastest execution

Cons:

- weak audit trail
- no structured release communication

### Option 2: Readiness doc + notes + formal tag/release (recommended)

Pros:

- clear evidence mapping
- clear external communication
- reproducible closure workflow

Cons:

- slightly more documentation work

### Option 3: Extended release package (guides/changelogs refactor)

Pros:

- comprehensive packaging

Cons:

- beyond current closure scope
- slower to complete

## Recommended Design

Use Option 2.

### 1. Release Readiness Document

Create a single document that maps `V1 Done Definition` to concrete evidence:

- scope boundaries
- semantic ownership
- adapter consistency
- verification command outcomes
- milestone tags and relevant commit chain

### 2. Release Notes Document

Create a concise public-facing release notes file that includes:

- what `v1.0.0` delivers
- what remains out of scope
- verification baseline commands
- highlight progression from `v1.1` to `v1.8`

### 3. Publish Release

After verification and commits:

- create/push `v1.0.0` tag on current stable HEAD
- publish GitHub release using prepared release notes

## Success Criteria

This round is successful when:

- readiness evidence document exists and is committed
- release notes document exists and is committed
- all four required verification commands pass
- `v1.0.0` tag exists on remote
- GitHub release for `v1.0.0` is published

## Non-Goals

This round does not aim to:

- add v2 planning
- alter README command truth
- introduce new tests beyond release-proof necessity
- reorganize repository structure

## Follow-On Work

Possible next phase after release closure:

1. open `v2` discovery/spec track as a separate scoped initiative.
