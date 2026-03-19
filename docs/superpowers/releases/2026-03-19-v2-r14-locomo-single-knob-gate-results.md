# V2-R14 LoCoMo Single-Knob Gate Results

## Summary

This round adds a minimal auto-stop evaluator and keeps retrieval logic unchanged.

Added:

- `tools/locomo/evaluate-single-knob.py`
- docs for running the gate before any new single-knob retrieval tweak

## Gate Output

Command:

```bash
./.venv/bin/python tools/locomo/evaluate-single-knob.py --output /tmp/localmemos-v2-r14-single-knob-result.json
```

Decision:

- action: `NO-OP`
- selected candidate: `null`
- rule: apply only when both `overall` and `multi-hop` strictly improve

Candidate summary:

- `games`: overall `0.5822` (no gain), multi-hop `0.4291`
- `kind`: unchanged
- `names`: regresses overall and multi-hop

## Verification

Passed in this round:

1. `cargo test --workspace`
2. `./.venv/bin/pytest python/tests -q`
3. `corepack pnpm --dir packages/node test`
4. `corepack pnpm --dir packages/mcp test`
5. `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

LoCoMo K=5 remains:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`

## Outcome

- Auto-stop gate is now repository-owned and deterministic.
- V2 iteration can stop automatically when evidence does not justify another tweak.
