# V2-R6 LoCoMo Multi-hop Margin Stabilization Results

## Summary

This round applied low-risk query filler filtering to improve retrieval margin while preserving existing ranking behavior.

Implemented:

- stopword expansion in `fts` for generic question fillers:
  - `activity`, `activities`
  - `way`, `ways`
  - `change`, `changes`
  - `been`
- TDD coverage for new normalization behavior

## LoCoMo Evidence (K=5)

### Before (V2-R5)

- overall: `0.5817`
- multi-hop: `0.4220`
- open-domain: `0.3261`

### After (V2-R6)

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

## Required Verification Matrix

All required verification commands passed:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

## Outcome

- multi-hop moved from threshold-edge (`0.4220`) to a larger safety margin (`0.4255`)
- overall and open-domain both remained above thresholds
- category gate remains fully green
