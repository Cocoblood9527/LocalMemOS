# V2-R4 LoCoMo Retrieval Stabilization + Category Gate Design

## Goal

Recover LoCoMo retrieval quality from the unstable experimental state and ship a safe V2 checkpoint that keeps overall retrieval strong while enforcing category visibility.

Primary target (K=5):

- restore `overall hit@5 >= 0.55`
- keep `open-domain >= 0.32`
- preserve category-aware gate tooling for future iterations

## Context

Current uncommitted retrieval experiments reduced gate result to roughly `overall=0.496` in recent runs. The regression is in ranking behavior, not in tooling. We need a low-risk stabilization first, then continue iterative retrieval optimization in follow-up rounds.

## Scope

In scope:

- remove unstable retrieval ranking tweaks from `memory-core`
- keep and validate category gate scripts and docs
- add/keep deterministic regression test coverage where safe
- provide fresh LoCoMo evidence and required test matrix output

Out of scope:

- larger retrieval algorithm redesign
- full LLM answer-side evaluation overhaul
- schema/API changes

## Options Considered

1. Tooling-only merge and postpone code rollback
- Risk: keeps current retrieval regression active.

2. Continue aggressive retrieval tuning now
- Risk: high churn, likely more regression before stability.

3. Stabilize retrieval first, keep category gate, then iterate with small deltas (chosen)
- Best risk/reward for current phase and user priority.

## Chosen Design

### 1) Retrieval Stability First

Revert the newly introduced ranking tie-break behavior and anchor-token extraction additions that are not yet validated. Keep previously merged V2-R3 ranking logic as stable reference.

### 2) Category-aware Gate as Safety Net

Keep `assert-locomo-thresholds.py` and `run-category-gate.sh`, plus README docs. This allows monitoring `overall`, `multi-hop`, and `open-domain` at once.

### 3) Verification Discipline

Run LoCoMo gate scripts and required four-command matrix each round. Record exact pass/fail with thresholds.

## Test Strategy

- Rust tests including `memory-core` fallback/ranking suite
- LoCoMo regression gate: `tools/locomo/run-regression-gate.sh 5`
- LoCoMo category gate: `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
- required matrix:
  - `cargo test --workspace`
  - `./.venv/bin/pytest python/tests -q`
  - `corepack pnpm --dir packages/node test`
  - `corepack pnpm --dir packages/mcp test`

## Success Criteria

1. `overall hit@5` gate returns to pass (`>=0.55`)
2. category gate tooling executes and reports explicit per-category checks
3. all four required verification commands pass
4. changes are mergeable without destructive history rewriting
