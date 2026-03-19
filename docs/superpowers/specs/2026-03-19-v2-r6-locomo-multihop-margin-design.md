# V2-R6 LoCoMo Multi-hop Margin Stabilization Design

## Goal

Build a small safety margin above the new V2-R5 category threshold pass by improving multi-hop retrieval robustness while preserving:

- overall `hit@5 >= 0.55`
- multi-hop `hit@5 >= 0.42`
- open-domain `hit@5 >= 0.32`

Target for this round: keep category gate green and preferably move multi-hop slightly above `0.422`.

## Baseline (2026-03-19, after V2-R5)

- overall: `0.5817`
- multi-hop: `0.4220`
- open-domain: `0.3261`
- category gate: PASS

## Problem

Some multi-hop questions still contain generic natural-language fillers (e.g., “ways”, “activities”, “changes”, “been”) that contribute little evidence signal but broaden FTS OR matching. This can dilute top-5 precision for clue-dense questions.

## Options Considered

1. Larger ranking formula retune
- Potential gains but wider regression surface.

2. Query rewriting / anchor extraction in retrieval path
- Higher complexity and previously unstable.

3. Minimal stopword expansion for question fillers (chosen)
- Very small change radius, directly targets noisy query terms.

## Chosen Design

### 1) Stopword expansion (low-risk)

Expand `STOPWORDS` in `crates/memory-core/src/fts.rs` with selected question filler terms:

- `activity`, `activities`
- `way`, `ways`
- `change`, `changes`
- `been`

### 2) TDD coverage for token normalization

Add/extend unit tests in `fts.rs` to verify these filler terms are dropped while retaining semantic anchors (names, entities, key nouns).

### 3) Verification-first rollout

After implementation, rebuild python binding and run both LoCoMo gates plus required 4-command matrix before any completion claim.

## Test Strategy

1. Unit tests in `memory-core` (`fts` token normalization)
2. `cargo test -p memory-core --test fts_fallback -- --nocapture`
3. LoCoMo:
- `tools/locomo/run-regression-gate.sh 5`
- `tools/locomo/run-category-gate.sh 5 0.55 0.42 0.32`
4. Required verification matrix:
- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

## Success Criteria

1. category gate remains PASS at default thresholds
2. no regression in overall/open-domain below thresholds
3. all required verification commands pass
