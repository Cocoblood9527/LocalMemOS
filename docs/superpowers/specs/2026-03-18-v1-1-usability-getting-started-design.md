# V1.1 Usability Getting Started Design

## Overview

This document defines a small `v1.1 usability` documentation project for `localmemos`.

The purpose of this round is to improve first-time local setup and re-entry usability for the repository without expanding product scope, adding new runtime features, or introducing automation work that belongs to a later phase.

## Goal

Turn the root `README.md` into a practical first-stop guide for a developer who wants to:

- prepare the local environment
- perform first-time dependency setup
- run the current verification commands successfully
- understand the expected day-to-day development entry path

This round is documentation-only.

## Problem Statement

The current README already explains what the project is, what `v1` includes, what is out of scope, and which verification commands exist.

The remaining usability gap is that a first-time developer still has to infer some setup steps, especially around local environment preparation and Python test execution. For example, the README references `./.venv/bin/pytest` but does not currently explain how `.venv` is created and populated.

That makes the README good as a maintenance summary, but still incomplete as a minimal onboarding path.

## Target Reader

The primary reader is a developer working on the repository locally, especially:

- the project owner returning after some time away
- a development model or collaborator asked to continue work
- a technically capable maintainer who needs the shortest path from clone to verified local state

This is not a public quickstart or end-user installation guide.

## Scope

This project only updates the root `README.md`.

It should add or strengthen guidance for:

- environment setup expectations
- first-time setup steps
- dependency installation flow
- minimal verification path
- current local development workflow notes

It should not add:

- code changes
- helper scripts
- CI workflows
- release instructions
- packaging or distribution guidance
- expanded API documentation
- `v2` feature discussion

## Constraints

The documentation must remain:

- concise
- operational
- easy to maintain
- consistent with the current repository reality
- aligned with the approved `v1 MVP` scope

It must not turn the README into a long-form contributor handbook. The root README should stay as the single entry page, not split into additional documents in this round.

## Recommended Approach

Recommended approach: enhance the root README with a small, linear "from zero to running locally" path.

Why this approach:

- it fixes the immediate usability gap at the actual entry point
- it avoids documentation sprawl
- it preserves the lightweight maintenance style established in the previous README polish round

Alternative approaches considered but not recommended in this round:

### 1. Add a separate setup guide

Pros:

- keeps README shorter
- allows more detail

Cons:

- adds navigation overhead
- premature document splitting for a still-small repository

### 2. Add helper scripts instead of more docs

Pros:

- can reduce setup friction further

Cons:

- introduces implementation and maintenance scope
- moves this round out of documentation-only territory

## Recommended README Additions

The README should keep its current high-level project summary sections and add a clearer setup path with the following content.

### 1. Environment Setup

State the baseline local tools and what they are needed for.

This section should stay short and practical, covering:

- Rust toolchain
- Python 3
- Node.js
- Corepack

If appropriate, mention that Python virtual environment support is expected because tests are run from `./.venv`.

### 2. First-Time Setup

Add a short ordered sequence that a first-time developer can follow after cloning:

1. create a Python virtual environment
2. install Python test dependencies
3. enable Corepack if needed
4. install Node workspace dependencies if required by the current package layout

This section should be explicit enough that the reader does not need to guess missing bootstrap steps.

### 3. Verification Path

Present the current verification commands as a practical check sequence rather than as an isolated command dump.

This section should help the reader answer:

- what should I run first
- what does a healthy local setup look like
- which commands cover Rust, Python, Node, and MCP surfaces

### 4. Current Dev Workflow Notes

Add a compact note section for local contributors that captures the most important operating assumptions, such as:

- `memory-core` remains the semantic source of truth
- thin adapters should follow core semantics rather than redefine them
- this repository is still intentionally `v1 MVP`

This section should stay short and avoid turning into a contribution policy document.

## Writing Style

The README changes should use a direct operational style:

- short paragraphs
- short command blocks
- minimal theory
- explicit local commands
- no marketing wording

The goal is for a reader to act immediately, not interpret prose.

## Success Criteria

This `v1.1 usability` documentation round is successful if a first-time local developer can open the README and quickly answer:

- what tools do I need installed
- how do I create the expected local Python environment
- how do I install what is needed before testing
- which commands should I run to verify the workspace
- what semantic boundaries should I preserve while developing

## Non-Goals

This round does not aim to:

- redesign the repository workflow
- automate setup
- add contributor governance documents
- add CI
- document `v2`
- change implementation behavior

## Follow-On Work

After this small usability documentation round, the next likely follow-on tracks are:

1. lightweight local workflow cleanup if setup pain still remains
2. CI and automation when the project is ready for it
3. `v2` design work only after `v1` polish is considered stable
