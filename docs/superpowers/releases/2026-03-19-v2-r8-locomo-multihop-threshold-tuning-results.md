# V2-R8 LoCoMo Multi-hop Threshold Tuning Results

## Summary

This round was a bounded parameter-search cycle. We tested multiple low-risk retrieval parameter tweaks and kept none, because each candidate either regressed a gate metric or failed local ranking tests.

Final state remains V2-R7 retrieval configuration.

## Experiment Matrix

### Candidate A: `long_query >= 7`

- Result: overall `0.5782`, multi-hop `0.4184`, open-domain `0.3261`
- Category gate: FAIL (`multi-hop < 0.42`)
- Decision: REVERT

### Candidate B: `long_query >= 5`

- Result: local ranking tests failed (`fts_fallback` regressions)
- Decision: REVERT before LoCoMo gate

### Candidate C: bm25 evidence weight `1.2 -> 2.0`

- Result: overall `0.5817`, multi-hop `0.4184`, open-domain `0.3261`
- Category gate: FAIL (`multi-hop < 0.42`)
- Decision: REVERT

### Candidate D: query token cap `12 -> 10`

- Result: overall `0.5812`, multi-hop `0.4255`, open-domain `0.3152`
- Category gate: FAIL (`open-domain < 0.32`)
- Decision: REVERT

## Final Metrics (kept configuration)

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`
- category gate: PASS

## Required Verification Matrix

All required commands pass on final kept state:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
