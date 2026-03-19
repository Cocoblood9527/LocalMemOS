# V2-R16 Local Rerank Rare-Anchor Design

## Goal

Implement a pure-local, low-complexity reranker improvement that can lift retrieval quality without adding LLM dependency.

## Baseline

Current LoCoMo K=5 baseline:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Problem

When two candidates have similar token overlap, current scoring can over-favor phrase continuity/common-token matches and miss facts that contain rarer anchor clues.

## Options Considered

1. Keep current reranker
- zero risk, zero gain.

2. Add one rare-anchor coverage feature (chosen)
- small local change; uses only per-query candidate statistics.

3. Introduce external local model reranker
- higher complexity and dependency cost for this stage.

## Chosen Design

1. In `recall_with_fts`, compute per-query token document frequency across candidate rows.
2. Identify rare-anchor query tokens (minimum document frequency among matched query tokens).
3. Add one additional rerank feature:
- `rare_anchor_coverage = matched_rare_anchors / total_rare_anchors`
4. Add this feature into existing rerank score with small query-length-aware weight.

No schema changes, no API changes, no external model dependency.

## Test Strategy

1. Add one regression test in `crates/memory-core/tests/fts_fallback.rs` for tie-like overlap where rare anchor should win.
2. Run required matrix:
- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`
3. Run LoCoMo full gate:
- `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

## Success Criteria

1. Pure-local rerank improvement implemented with one new feature.
2. Regression test passes.
3. Required matrix + full gate pass.
