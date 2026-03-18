# V1.5 README Role-Based Operation Paths Design

## Overview

This document defines a small `v1.5` documentation project for `localmemos`.

The round stays documentation-first and improves operator flow by adding role-based entry paths in the root `README.md`.

## Goal

Improve `README.md` so different maintainers can quickly select the correct operational path without expanding `v1` scope or changing runtime behavior.

This round is documentation-only.

## Problem Statement

`v1.1` through `v1.4` improved setup, troubleshooting, validation, and command drift guard quality. However, the README is still mostly section-linear.

Typical friction points:

- maintainers can find commands, but first-step selection is still manual
- users with different responsibilities may run either too much (slow) or too little (risky)
- role context (docs-only vs core semantics vs adapter surface) is implicit instead of explicit

The missing piece is a compact role-first decision layer that maps directly to existing sections and existing command policy.

## Scope

Implementation output in this round updates only `README.md`.

It should add:

- one compact role-based operation-path section near the top-level workflow guidance
- explicit path guidance for common maintainer roles in this repository
- direct links from each role path to existing README sections (`First-Time Setup`, `Verification Path`, drift-guard and matrix sections)

It may refine:

- wording and ordering around workflow navigation for readability

It should not add:

- code changes
- scripts or automation
- CI workflows
- new memory features
- v2 architecture expansion
- additional user-facing docs outside existing workflow artifacts

## Constraints

The README must remain:

- concise and operational
- aligned with current repository behavior
- consistent with `memory-core` as the single semantic source of truth
- within current `v1 MVP` boundaries

All verification commands must remain exactly aligned with existing `Verification Path` and `Minimum Regression Order`.

## Assumptions

- `v1.5` focuses on navigation and execution clarity, not new technical capability.
- Role paths should reuse existing README sections instead of duplicating long instructions.
- If a maintainer is unsure which path applies, guidance should prefer the safer broader validation route.

## Option Analysis

### Option 1: Compact role cards + path actions in README (recommended)

Pros:

- fastest path selection for maintainers
- no additional document maintenance surface
- keeps one operational entry point

Cons:

- role descriptions must stay brief to avoid README bloat

### Option 2: Separate per-role docs with README links

Pros:

- more room for detail per role

Cons:

- fragments operational source of truth
- increases navigation and maintenance overhead

### Option 3: Flowchart-first section (ASCII/diagram style)

Pros:

- visually clear branching

Cons:

- higher formatting overhead in markdown
- less friendly for quick copy/run workflows

## Recommended Design

Use Option 1.

Add a compact README section, suggested heading:

- `## Role-Based Operation Paths`

Recommended placement:

- after `Current Status` and before `Repository Layout` (early enough to guide actions)

### Role Set

Define concise paths for:

1. `Docs Maintainer`
2. `Core Semantics Maintainer`
3. `Adapter Surface Maintainer` (HTTP/Python/Node/MCP changes without redefining semantics)
4. `Clean-Run Verifier` (environment and setup validation focus)

### Path Content Pattern

Each role path should use a compact schema:

- `Use this path when...`
- `Read first...` (link to existing README sections)
- `Run at minimum...` (reuse existing commands/order only)
- `Escalate when...` (map to Failure Triage Rule)

### Policy Alignment

Path guidance must explicitly preserve:

- `memory-core` semantic authority
- `When To Re-Validate Commands`
- `Minimum Regression Order`
- `Failure Triage Rule`

No role path may redefine semantics or introduce alternate command truth.

## Writing Style

Use short operational language:

- terse bullets
- explicit role names
- explicit section names and command blocks
- no theory-heavy prose

Keep each role path short enough for quick scanning.

## Success Criteria

This round is successful if a maintainer can:

- identify their role and choose a workflow path in under one minute
- navigate directly to relevant existing README sections with low ambiguity
- run the correct minimum validation order without inventing new command sequences
- keep semantic ownership centered on `memory-core`

## Non-Goals

This round does not aim to:

- change runtime behavior
- add new command families
- automate role detection
- expand into `v2` memory-system design

## Follow-On Work

Possible future rounds:

1. optional concise role-to-command quick table tuning after real usage feedback
2. optional README information-architecture cleanup once v1 docs stabilize
