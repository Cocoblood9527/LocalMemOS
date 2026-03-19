# V2-R13 LoCoMo Evidence-Only Single-Knob Design

## Goal

Avoid over-design by allowing at most one stopword tweak only when evidence shows clear gain.

Round policy:

- run offline A/B for exactly three candidates (`games`, `kind`, `names`)
- only change code if `overall` and `multi-hop` both improve
- if not, ship no retrieval code change

## Baseline

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## A/B Results

- `games`: overall `0.5822`, multi-hop `0.4291`, single-hop `0.6302`
- `kind`: unchanged from baseline
- `names`: overall `0.5817`, multi-hop `0.4220`

## Decision

No candidate satisfies the acceptance rule (`overall` + `multi-hop` both up without side effects).

Chosen action: no retrieval logic change in this round.

## Test Strategy

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
5. `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

## Success Criteria

1. no code changes in retrieval path
2. required 4-command matrix passes
3. full-gate remains PASS
