# V2-R4 LoCoMo Stabilization Results

## Summary

This round removed unstable retrieval experiments from `memory-core` and kept category-aware LoCoMo gate tooling.

## LoCoMo Evidence (K=5)

### Regression Gate

Command:

```bash
tools/locomo/run-regression-gate.sh 5
```

Result:

- overall: `0.5767` (PASS vs threshold `0.55`)
- multi-hop: `0.4078`
- open-domain: `0.3261`

### Category Gate

Command:

```bash
tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32
```

Result:

- overall: `0.5767` vs `0.55` (PASS)
- multi-hop: `0.4078` vs `0.42` (FAIL)
- open-domain: `0.3261` vs `0.32` (PASS)

## Required Verification Matrix

All required verification commands passed in this round:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

## Outcome

- retrieval path restored to stable V2-R3 behavior
- overall LoCoMo gate recovered and passing
- category-aware gating is now available for targeted next-round optimization (focus: `multi-hop`)
