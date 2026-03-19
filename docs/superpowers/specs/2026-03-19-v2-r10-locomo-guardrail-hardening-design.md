# V2-R10 LoCoMo Guardrail Hardening Design

## Goal

Harden the current best retrieval baseline with automation guardrails, without changing retrieval behavior.

## Baseline to Protect (K=5)

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Scope

In scope:

- add baseline drift checker against pinned baseline + allowed drop budget
- add one-command full gate that runs LoCoMo gates and required verification matrix
- document usage in repository docs

Out of scope:

- retrieval ranking changes
- LLM answer-side changes

## Design

### 1) Drift Guard Config

Add a repo-owned baseline file in `tools/locomo` with pinned metrics and maximum allowed drops.

### 2) Drift Assert Script

Add a Python checker that compares a baseline result JSON to pinned values and fails when drop exceeds budget.

### 3) Full Gate Script

Add a shell script that runs, in order:

1. optional python binding rebuild
2. LoCoMo regression gate
3. LoCoMo category gate
4. `cargo test --workspace`
5. `./.venv/bin/pytest python/tests -q`
6. `corepack pnpm --dir packages/node test`
7. `corepack pnpm --dir packages/mcp test`

### 4) Documentation

Update `tools/locomo/README.md` and root `README.md` with commands and expectations.

## Success Criteria

1. drift checker catches regressions beyond configured budget
2. full gate runs all required checks end-to-end
3. no retrieval behavior change in this round
