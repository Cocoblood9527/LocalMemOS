# V2-R14 LoCoMo Single-Knob Auto-Stop Gate Design

## Goal

Turn the anti-overdesign rule into a small repository-owned tool:

- evaluate a fixed small set of single-knob candidates
- only allow a code-change recommendation when both `overall` and `multi-hop` improve
- otherwise output `NO-OP`

## Baseline

Current K=5 baseline:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Options Considered

1. Keep manual spreadsheet/eyeballing
- Low implementation cost but decision drift risk.

2. Add one small deterministic evaluator script (chosen)
- Minimal maintenance, explicit stop condition, no retrieval logic changes.

3. Build a larger hyperparameter search framework
- Overkill for current stage.

## Chosen Design

1. Add `tools/locomo/evaluate-single-knob.py`:
- runs baseline + candidate simulations on LoCoMo K=5 retrieval
- default candidates: `games`, `kind`, `names`
- hardcoded pass rule: candidate must satisfy
  - `candidate.overall > baseline.overall`
  - `candidate.multi-hop > baseline.multi-hop`
- output includes:
  - baseline metrics
  - candidate metrics
  - decision: `APPLY` or `NO-OP`
  - selected candidate when applicable

2. Keep behavior read-only for repository code:
- script only reports decision; it does not modify retrieval code.

3. Document usage in LoCoMo README + root README.

## Test Strategy

1. run the new script once:
- `./.venv/bin/python tools/locomo/evaluate-single-knob.py`

2. required validation matrix:
- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

3. retrieval gate sanity:
- `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

## Success Criteria

1. evaluator script exists and runs end-to-end
2. decision rule is deterministic and hardcoded
3. docs clearly state the NO-OP gate behavior
4. full verification passes
