# V1 README Polish Design

## Overview

This document defines a small `v1 polish` project focused only on improving the repository README for `localmemos`.

The README in this phase is not intended to become a full architecture document, public launch page, or complete API reference. It is intended to become a practical entry point for future development work.

## Goal

Turn the current README into a compact operational guide that helps a future maintainer quickly understand:

- what this project is
- what `v1` currently includes
- what `v1` explicitly does not include
- how the repository is organized
- what tools are needed locally
- how to run verification
- how to access the system through its current runtime paths
- what semantic caveats matter during continued development

## Target Reader

The primary audience is the future maintainer of the repository, especially the same developer returning after some time away.

This README should optimize for re-entry speed and maintenance clarity rather than public marketing or contributor onboarding depth.

## Scope

This polish phase only covers the root `README.md`.

It should improve:

- project summary
- current status communication
- repository layout explanation
- prerequisites
- build and test commands
- runtime access paths
- important development notes

It should not add:

- CI configuration
- release automation
- new features
- major documentation structure changes outside the root README
- a full contribution guide
- a full API reference

## Constraints

The README should remain:

- compact
- high-signal
- easy to keep up to date
- consistent with the approved `v1` design and implementation

It must not duplicate large sections from the main architecture spec. When deeper architectural context is needed, it should point readers to existing documents under `docs/superpowers/`.

## Recommended README Structure

The README should use the following section structure.

### 1. Project Summary

Briefly explain:

- `localmemos` is a local-first fact memory kernel
- current maturity is `v1 MVP`
- supported access paths are Python SDK, Node SDK, HTTP, and MCP

### 2. Current Status

Split this into two short lists:

- what `v1` currently supports
- what is intentionally out of scope for `v1`

This section should make the boundary between implemented `v1` and future work explicit.

### 3. Repository Layout

Explain the purpose of the main directories:

- `crates/memory-core`
- `crates/memory-http`
- `crates/memory-node`
- `packages/node`
- `packages/mcp`
- `python`
- `docs/superpowers`

Each description should be short and responsibility-focused.

### 4. Prerequisites

List the tools needed for local development and verification, such as:

- Rust toolchain
- Python
- Node.js
- Corepack

Keep this section practical rather than exhaustive.

### 5. Build and Test

Provide concrete commands for:

- Rust workspace tests
- Python tests
- Node tests
- MCP tests

These commands should reflect the current working repository reality.

### 6. Runtime Paths

Explain the purpose of the four current access modes:

- Python SDK
- Node SDK
- HTTP
- MCP

This section should clarify their role without expanding into a full usage tutorial.

### 7. Development Notes

Include the important semantic reminders that are easy to forget, including:

- `memory-core` is the single source of truth for semantics
- `RecallRequest.include_history` is reserved in `v1`
- `/facts/{id}/history` uses a stored `facts.id` and resolves to the full logical version chain
- `v1` does not include vector retrieval, automatic extraction, or graph-native memory features

## Writing Style

The README should follow a tight operational style:

- short sections
- short bullets
- explicit commands
- minimal repetition
- direct wording over promotional wording

It should read more like a maintenance entry page than a product brochure.

## Success Criteria

This README polish is successful if a future maintainer can open the repository and quickly answer:

- what is this
- what has already been implemented
- how do I run the tests
- where does each runtime surface live
- what should I avoid changing casually

## Non-Goals

This phase does not aim to:

- redesign project documentation globally
- replace the architecture spec
- document every API in detail
- describe `v2`
- provide contributor workflow policies

## Follow-On Work

After this README polish is complete, the next likely documentation/quality tracks are:

1. CI setup
2. developer workflow cleanup
3. broader `v1.1` usability polish

Those are separate efforts and should not be folded into this README-focused scope.
