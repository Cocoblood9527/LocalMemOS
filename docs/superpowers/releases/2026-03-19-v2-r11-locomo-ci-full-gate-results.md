# V2-R11 LoCoMo CI Full Gate Results

## Summary

This round adds CI automation for the existing LoCoMo full gate. Retrieval logic is unchanged.

Added:

- GitHub Actions workflow to run full LoCoMo gate on PR/push to `main`
- workflow artifact upload for gate logs and result JSON
- README updates describing CI gate behavior
- spec + implementation plan docs for V2-R11

## New Files

- `.github/workflows/locomo-full-gate.yml`
- `docs/superpowers/specs/2026-03-19-v2-r11-locomo-ci-full-gate-design.md`
- `docs/superpowers/plans/2026-03-19-v2-r11-locomo-ci-full-gate-implementation-plan.md`

## LoCoMo Full Gate Evidence (K=5)

Command:

```bash
REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5
```

Result:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`
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

- LoCoMo retrieval guardrail is now enforceable in CI.
- PRs to `main` can be blocked automatically when retrieval metrics or drift budgets regress.
