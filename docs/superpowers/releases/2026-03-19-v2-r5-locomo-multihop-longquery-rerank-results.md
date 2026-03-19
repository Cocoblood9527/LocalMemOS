# V2-R5 LoCoMo Multi-hop Long-query Re-rank Results

## Summary

This round targeted `multi-hop` retrieval uplift with low-risk ranking and tokenization changes.

Implemented:

- long-query ranking now uses `bm25` primary + rerank score secondary + recency fallback
- expanded query stopword filtering for common question fillers (`many/has/have/her/their/done` etc.)
- added TDD coverage for token filtering and long-query tie-break behavior

## LoCoMo Evidence (K=5)

### Baseline before V2-R5 (from V2-R4)

- overall: `0.5767`
- multi-hop: `0.4078`
- open-domain: `0.3261`

### V2-R5 after changes

Command:

```bash
tools/locomo/run-regression-gate.sh 5
```

Result:

- overall: `0.5817` (PASS vs `0.55`)
- multi-hop: `0.4220`
- open-domain: `0.3261`

Command:

```bash
tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32
```

Result:

- overall: `0.5817` vs `0.55` (PASS)
- multi-hop: `0.4220` vs `0.42` (PASS)
- open-domain: `0.3261` vs `0.32` (PASS)

## Required Verification Matrix

All required commands passed:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

## Outcome

- V2 category gate is now fully green at default thresholds
- `multi-hop` reached target (`>= 0.42`) while preserving overall retrieval quality
