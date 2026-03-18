# V1.6 README Quick Path Picker Design

## Overview

This document defines a small `v1.6` documentation project for `localmemos`.

The round remains documentation-first and adds a fast operator entry point so maintainers can choose a role path and minimum command set in under 30 seconds.

## Goal

Improve `README.md` with a compact `quick path picker` that sits above detailed role guidance and routes maintainers to the right existing sections and command truth without introducing new semantics.

This round is documentation-only.

## Problem Statement

`v1.5` introduced `Role-Based Operation Paths`, but the first decision still requires scanning multiple role subsections before acting.

Current friction:

- readers must parse all role details before selecting a path
- minimum command execution guidance is present but not presented as a one-glance launch step
- maintainers under time pressure can still hesitate on where to start

The missing piece is a compact front-door selector with explicit role mapping and minimum command pointer.

## Scope

Implementation output in this round updates only `README.md`.

It should add:

- one compact `30-second` path picker section near `Role-Based Operation Paths`
- one-glance mapping from change type to role path
- explicit minimum-command reference per role using existing command truth

It may refine:

- minor wording in nearby role-path text for consistency

It should not add:

- code changes
- scripts or automation
- CI workflows
- new API/runtime behavior
- v2 roadmap expansion
- alternate command sequences

## Constraints

The README must remain:

- concise and operational
- aligned with existing repository behavior
- consistent with `memory-core` as single semantic source of truth
- within current `v1 MVP` boundaries

`Verification Path`, `Minimum Regression Order`, and `Failure Triage Rule` remain authoritative and unchanged in meaning.

## Option Analysis

### Option 1: Compact quick-picker table + direct section links (recommended)

Pros:

- fastest scan-to-action workflow
- minimal content overhead
- preserves single README entry point

Cons:

- adds one more section to top half of README

### Option 2: Keep only existing role subsections

Pros:

- no additional content

Cons:

- slower first decision for maintainers
- weaker one-glance usability

### Option 3: ASCII decision tree block

Pros:

- explicit branching

Cons:

- harder to maintain and scan in plain markdown
- more visual noise than value for v1 docs

## Recommended Design

Use Option 1.

Add a new section directly after `Role-Based Operation Paths`:

- `## 30-Second Path Picker`

### Section Structure

Use two compact parts:

1. `Pick your starting path` quick table
2. `Minimum command set` copy/run block (same four commands already used by `Verification Path` and `Minimum Regression Order`)

### Quick Table Requirements

Each row includes:

- `If your change is...`
- `Use role path`
- `Start here`
- `Minimum command action`

Required role coverage:

- Docs maintainer changes
- `memory-core` semantic changes
- Adapter-surface changes
- Clean-run/env validation

### Command Truth Requirements

The section must not redefine commands. It should:

- reuse existing four-command sequence verbatim
- reference `Minimum Regression Order` and `Failure Triage Rule` for escalation and classification
- keep role-based semantics aligned with `v1.5`

## Writing Style

The new section should be:

- compact enough to read in under 30 seconds
- direct and command-oriented
- low prose, high signal

## Success Criteria

This round is successful if a maintainer can:

- identify the starting path in one scan
- find the minimum command set without searching across README
- preserve existing policy flow (`re-validate` triggers, command order, triage rule)
- avoid semantic drift away from `memory-core` authority

## Non-Goals

This round does not aim to:

- change setup or test commands
- replace detailed role-path guidance
- automate role or command selection
- add v2 planning guidance

## Follow-On Work

Possible future rounds:

1. optional navigation polish for long README sections after `v1` docs stabilization
2. optional docs IA audit for further scan-time reduction
