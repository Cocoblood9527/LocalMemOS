# V2-R7 LoCoMo Hard-case Query Filter Results

## Summary

This round refreshed failure samples and tested one low-risk filler token candidate (`seen`) in query stopword normalization.

Decision: keep the candidate, because thresholds stayed green and no regression occurred.

## Changes

- `memory-core`:
  - add `seen` to query stopwords in `fts`
  - add TDD test `query_tokens_drop_seen_filler`
- tooling artifact refresh:
  - `tools/locomo/failure-samples-k5.json` regenerated with limit `120`
- superpowers docs:
  - V2-R7 spec/plan/results

## LoCoMo Evidence (K=5)

### Baseline before V2-R7 (V2-R6)

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

### V2-R7

Command:

```bash
tools/locomo/run-regression-gate.sh 5
```

Result:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

Command:

```bash
tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32
```

Result:

- overall: PASS
- multi-hop: PASS
- open-domain: PASS

## Failure Sample Refresh

Command:

```bash
tools/locomo/refresh-failure-samples.sh 5 120
```

Result metadata:

- selected_misses: `120`
- total_misses: `828`

## Required Verification Matrix

All required commands passed:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
