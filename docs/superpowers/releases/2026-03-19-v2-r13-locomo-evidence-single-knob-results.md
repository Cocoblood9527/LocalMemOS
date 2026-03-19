# V2-R13 LoCoMo Evidence-Only Single-Knob Results

## Summary

This round intentionally avoids retrieval code changes unless one candidate gives clear dual gain.

Candidates evaluated: `games`, `kind`, `names`.

## Offline A/B Evidence

Baseline:

- overall: `0.5822`
- multi-hop: `0.4255`
- open-domain: `0.3370`
- single-hop: `0.6314`

Candidates:

- `games`: overall `0.5822`, multi-hop `0.4291`, single-hop `0.6302`
- `kind`: unchanged vs baseline
- `names`: overall `0.5817`, multi-hop `0.4220`

Decision rule:

- require both overall + multi-hop improvement with no obvious side effect

Decision:

- no candidate passed; no retrieval code change made

## Outcome

- V2-R13 is a docs-only, evidence-driven no-op round.
- over-design avoided by enforcing the single-knob acceptance gate.
