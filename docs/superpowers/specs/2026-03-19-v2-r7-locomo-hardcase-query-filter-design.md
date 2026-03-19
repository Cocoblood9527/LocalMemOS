# V2-R7 LoCoMo Hard-case Query Filter Design

## Goal

Improve V2 retrieval robustness on remaining hard multi-hop misses while preserving all gate thresholds.

Targets (K=5):

- keep `overall >= 0.55`
- keep `multi-hop >= 0.42` and try to improve above V2-R6 (`0.4255`)
- keep `open-domain >= 0.32`

## Baseline (after V2-R6)

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Problem

Remaining misses in failure samples still include conversational filler verbs/adverbs in questions. These tokens can widen OR-style FTS retrieval without adding much evidence precision.

## Approaches

1. Ranking formula retune
- Higher regression risk, broader blast radius.

2. Phrase/anchor query rewrite
- Higher complexity and previously unstable.

3. Targeted filler-token filtering (chosen)
- Minimal-risk stopword refinement with TDD and direct gate-based A/B validation.

## Chosen Design

### 1) Add one hard-case filler token

Test adding `seen` as a stopword candidate in query normalization.

### 2) Keep strict rollback boundary

- If metrics improve or remain neutral with stable gates, keep change.
- If metrics regress materially, revert candidate and keep baseline.

### 3) Refresh failure corpus for traceability

Keep refreshed `tools/locomo/failure-samples-k5.json` in this round to align next iterations with latest retriever behavior.

## Test Strategy

1. TDD unit test for dropping `seen` as query filler
2. `cargo test -p memory-core --test fts_fallback -- --nocapture`
3. LoCoMo gates:
- `tools/locomo/run-regression-gate.sh 5`
- `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
4. Required verification matrix:
- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

## Success Criteria

1. category gate remains PASS
2. no threshold regression
3. if `multi-hop` improves over `0.4255`, keep change; otherwise drop the candidate and finish with refreshed artifacts only
