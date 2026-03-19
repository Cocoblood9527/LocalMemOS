# V2-R12 LoCoMo Single-Knob `like` Stopword Design

## Goal

Apply one minimal retrieval tweak to improve LoCoMo hit stability without broad redesign.

Primary rule for this round: one knob only.

## Baseline

Current K=5 baseline (from V2-R10/V2-R11):

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Brainstorming Summary

After refreshing `failure-samples-k5.json` with limit 120 and replaying candidate single-word removals:

- removing `games`: multi-hop improves but single-hop drops, overall unchanged
- removing `like`: small net gain (`overall 0.5822 -> 0.5827`, adversarial improves), no threshold-category degradation

Chosen option: add `like` into query stopwords.

## Chosen Design

1. Add one token to `STOPWORDS` in `crates/memory-core/src/fts.rs`:
- `like`

2. Add one focused unit test verifying `like` is removed while anchor tokens stay.

3. Keep all other ranking/tokenization behavior unchanged.

## Test Strategy

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
5. `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

## Success Criteria

1. Single-knob change only (`like` stopword + minimal test)
2. required 4-command matrix passes
3. LoCoMo full gate passes (thresholds + drift)
