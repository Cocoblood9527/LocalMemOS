# V2-R16 Local Rerank Rare-Anchor Results

## Summary

This round completed pure-local reranker design/implementation exploration with strict anti-overdesign gating.

Outcome:

- implemented and validated a minimal rare-anchor rerank feature in working branch
- ran full verification and LoCoMo gate
- measured no LoCoMo aggregate gain
- reverted retrieval code changes and closed as docs-only no-op

## Why No-Op

Measured K=5 remained unchanged after the candidate reranker:

- overall: `0.5822` (no change)
- multi-hop: `0.4255` (no change)
- open-domain: `0.3370` (no change)

Given no benchmark lift, keeping extra ranking logic would add complexity without measurable value.

## Final Decision

- Keep repository retrieval code unchanged in this round.
- Preserve design/plan/results docs for future reference.

## Verification Evidence (final no-op state)

Passed:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
5. `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

LoCoMo final scores:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Artifact

- full-gate result: `/tmp/localmemos-v2-r16-final-gate/result_hit_at_5.json`
