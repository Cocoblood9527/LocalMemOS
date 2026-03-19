# V2-R8 LoCoMo Multi-hop Threshold Tuning Design

## Goal

Attempt to raise `multi-hop` beyond the V2-R7 level (`0.4255`) with low-risk retrieval parameter tuning, while preserving category gate thresholds.

## Baseline (from V2-R7)

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`
- category gate: PASS

## Scope

In scope:

- bounded parameter experiments in retrieval path
- strict rollback for any regression
- evidence-first decision logging

Out of scope:

- major ranking formula redesign
- query rewrite architecture changes
- LLM-side answer pipeline changes

## Candidate Experiments

1. `long_query` threshold from `>=6` to `>=7`
2. `long_query` threshold from `>=6` to `>=5`
3. bm25 `evidence_summary` weight from `1.2` to `2.0`
4. reduce query token cap from `12` to `10`

## Decision Rule

Keep only candidates that satisfy all:

1. `run-category-gate.sh` stays PASS
2. no degradation below thresholds
3. measurable improvement in target metric (`multi-hop`)

Otherwise revert immediately.

## Result Summary

All candidates failed the decision rule:

- `>=7` long-query threshold: multi-hop dropped to `0.4184` (FAIL)
- `>=5` long-query threshold: broke local ranking tests (reverted before gate)
- bm25 evidence weight `2.0`: multi-hop dropped to `0.4184` (FAIL)
- query token cap `10`: open-domain dropped to `0.3152` (FAIL)

Final decision: keep V2-R7 retrieval configuration unchanged.

## Success Criteria

1. final branch contains no retrieval regression
2. category gate remains PASS on final state
3. reproducible experiment record is committed for next rounds
