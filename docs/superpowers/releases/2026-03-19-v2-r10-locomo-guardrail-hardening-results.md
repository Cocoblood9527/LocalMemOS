# V2-R10 LoCoMo Guardrail Hardening Results

## Summary

This round adds tooling-only hardening around the current best retrieval baseline. Retrieval logic is unchanged.

Added:

- pinned baseline guardrail file for K=5
- drift assertion script with drop-budget checks
- one-command full gate script (LoCoMo + required 4-command matrix)
- README updates for new guardrail workflow

## New Files

- `tools/locomo/baseline-guardrails-k5.json`
- `tools/locomo/assert-locomo-drift.py`
- `tools/locomo/run-full-gate.sh`

## LoCoMo Evidence (K=5)

### Regression Gate

Command:

```bash
tools/locomo/run-regression-gate.sh 5
```

Result:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

### Category Gate

Command:

```bash
tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32
```

Result:

- PASS

### Full Gate (including drift)

Command:

```bash
REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5
```

Result:

- threshold checks: PASS
- drift checks: PASS
- required 4-command matrix: PASS

## Required Verification Matrix

All required commands passed in this round:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

## Outcome

- Baseline drift can now be caught automatically.
- A single command now runs LoCoMo threshold checks, drift checks, and full repository validation.
