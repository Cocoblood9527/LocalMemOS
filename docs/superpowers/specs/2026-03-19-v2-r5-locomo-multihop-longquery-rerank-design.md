# V2-R5 LoCoMo Multi-hop Long-query Re-rank Design

## Goal

Increase LoCoMo `multi-hop` evidence hit@5 from the current `0.4078` to at least `0.42` while preserving:

- `overall hit@5 >= 0.55`
- `open-domain >= 0.32`

## Baseline (2026-03-19)

From `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`:

- overall: `0.5767` (pass)
- multi-hop: `0.4078` (fail vs `0.42`)
- open-domain: `0.3261` (pass)

## Problem

Current recall behavior for long queries (`query_sequence.len() >= 6`) ignores existing feature rerank signals and uses bm25 + recency ordering only. Multi-hop questions are frequently long and clue-dense, so recency tie-breaking can outrank semantically stronger candidates.

## Approaches Considered

1. Aggressive query rewrite / anchor extraction in FTS query
- Potential gain but high regression risk (already caused major drop earlier).

2. Full blended ranking for long queries
- Better relevance but changes ranking surface too broadly in one step.

3. Conservative long-query tie-break with existing rerank score (chosen)
- Keep bm25 as primary signal, replace recency-only tie-break with rerank score as secondary comparator.
- Minimal change radius and directly targets multi-hop failure mode.

## Chosen Design

### 1) Long-query ordering update

In `recall_with_fts` for `long_query` path:

- primary: bm25 ascending (unchanged)
- secondary: rerank score descending (new)
- tertiary: updated_at descending (existing fallback)

No FTS query expansion and no schema changes.

### 2) TDD coverage

Add a long-query ranking test where bm25-near candidates differ in clue continuity/coverage. Expected top hit should prefer coherent clue alignment over pure recency.

## Risk and Mitigation

- Risk: ranking drift affecting overall score.
- Mitigation: keep bm25 primary, run both regression gate and category gate before merge.

## Test Strategy

1. Red/green focused Rust test (`fts_fallback`)
2. LoCoMo checks:
- `tools/locomo/run-regression-gate.sh 5`
- `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
3. Required 4-command matrix:
- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

## Success Criteria

1. category gate reaches pass (`multi-hop >= 0.42`) without breaking other thresholds
2. required verification commands all pass
3. change remains small, reviewable, and rollback-friendly
