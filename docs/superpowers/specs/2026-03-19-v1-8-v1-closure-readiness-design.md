# V1.8 V1 Closure Readiness Design

## Overview

This document defines a focused `v1.8` project for `localmemos`.

The round closes remaining release-readiness gaps for `v1` through documentation finalization and cross-adapter behavior verification, without expanding runtime scope.

## Goal

Prepare `v1` for a formal completion announcement by adding:

- explicit `V1 Done Definition` in README
- explicit `Release Closure Checklist` in README
- stronger cross-adapter verification evidence for `forget` behavior

This round does not add new memory capabilities.

## Problem Statement

The repository already has strong `v1` docs and stable validation workflows (`v1.1` to `v1.7`). Core behavior and adapter paths are implemented and tested, but closure criteria are still implicit:

- no single README section states what conditions qualify `v1` as complete
- release closure operations (verify, triage, commit order, push, stable tag) are practiced but not formalized as a stable checklist
- `forget` behavior exists across core/HTTP/Python/Node/MCP but consistency evidence is lighter than upsert/recall/history coverage

The missing piece is explicit closure policy plus targeted adapter-consistency evidence.

## Scope

Implementation output in this round updates:

- `README.md`
- adapter-level test files for `forget` consistency evidence

It should add:

- `V1 Done Definition` section in README
- `Release Closure Checklist` section in README
- cross-adapter tests confirming `forget` retires facts consistently from default recall/list paths

It should not add:

- new runtime features
- schema changes
- API shape changes
- CI workflows
- automation scripts
- v2 scope content

## Constraints

The round must preserve:

- `memory-core` as the single semantic source of truth
- existing command truth (`Verification Path` and `Minimum Regression Order`)
- current `v1` scope boundaries (no extraction/vector/graph/cloud/multi-tenant expansion)

All verification in this round must still include:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

## Option Analysis

### Option 1: Docs-only closure language (no test additions)

Pros:

- smallest change set
- very low execution cost

Cons:

- leaves cross-adapter `forget` evidence weaker than other primitives
- closure claim still relies on inferred behavior

### Option 2: Docs closure sections + targeted forget consistency tests (recommended)

Pros:

- explicit completion criteria
- explicit release checklist
- stronger confidence for adapter semantic parity

Cons:

- modest additional test maintenance

### Option 3: Broader test matrix refactor

Pros:

- maximal coverage

Cons:

- scope expansion beyond closure intent
- slows completion

## Recommended Design

Use Option 2.

### 1. README: V1 Done Definition

Add a compact section that defines closure criteria for `v1`, including:

- scope boundaries remain unchanged
- `memory-core` semantic authority remains unchanged
- adapters stay behaviorally consistent with core primitives
- required four-command verification passes

### 2. README: Release Closure Checklist

Add a compact per-round closure checklist that codifies existing practice:

- run required verification commands in order
- apply `Failure Triage Rule` for any failure
- commit sequence discipline (`spec -> plan -> implementation`)
- push `main`
- create and push stable tag

### 3. Tests: Forget Consistency Evidence

Add focused tests at adapter surfaces to verify:

- after `forget`, exact recall for the same key returns no active fact
- behavior is consistent with core retirement semantics

Target files:

- `python/tests/test_sdk_core.py`
- `packages/node/test/sdk.test.ts`
- `packages/mcp/test/tools.test.ts`
- `crates/memory-http/tests/http_smoke.rs`

Tests should remain minimal, behavior-focused, and avoid adding new product semantics.

## Writing Style

README additions should be:

- operational and concise
- checklist-first
- explicit about required order and escalation

## Success Criteria

This round is successful if:

- maintainers can point to one explicit README definition of `v1 complete`
- release closure actions are standardized and repeatable
- adapter-level tests provide direct evidence that `forget` behavior is semantically aligned
- full required verification suite passes

## Non-Goals

This round does not aim to:

- introduce new memory primitives
- redesign test architecture
- automate release workflows
- change any v1/v2 boundary decisions

## Follow-On Work

Possible finalization step after this round:

1. run final release summary pass and create formal `v1.0.0` milestone/tagging decision document.
