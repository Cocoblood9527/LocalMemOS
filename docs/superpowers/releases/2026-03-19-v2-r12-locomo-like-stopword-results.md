# V2-R12 LoCoMo `like` Stopword Results

## Summary

This round intentionally applies one single-knob retrieval change:

- add `like` to query stopwords in `memory-core` FTS normalization
- add one unit test for the new token rule

No ranking formula or rerank logic changes were made.

## Files

- `crates/memory-core/src/fts.rs`
- `docs/superpowers/specs/2026-03-19-v2-r12-locomo-like-stopword-design.md`
- `docs/superpowers/plans/2026-03-19-v2-r12-locomo-like-stopword-implementation-plan.md`
- `docs/superpowers/releases/2026-03-19-v2-r12-locomo-like-stopword-results.md`

## LoCoMo Gate Result (K=5)

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
- full gate: PASS

## Required Verification Matrix

All required commands passed:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`

## Outcome

- This minimal change is safe (no regression) under current guardrails.
- Measured K=5 gate metrics are unchanged in this run.
