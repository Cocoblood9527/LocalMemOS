# V2-R2 LoCoMo Regression Gate Design

## Goal

Operationalize LoCoMo retrieval evaluation inside repository tooling so V2 retrieval gains are reproducible and guarded against regression.

Primary target:

- ship repository-owned LoCoMo scripts under `tools/locomo/`
- add a threshold gate for retrieval quality (`hit@5 >= 0.05`)
- produce a checked-in R1 vs R2 comparison report

## Scope

In scope:

- migrate temporary `/tmp` LoCoMo scripts into repo tooling
- add script-level threshold assertion for `evidence hit@K`
- add one report document with overall and by-category comparison

Out of scope:

- changing retrieval core behavior in this round
- adding full CI pipeline for LoCoMo runs
- introducing full LLM answer model benchmarking changes

## Problem

Current LoCoMo evaluation scripts live in `/tmp`, which is:

- non-versioned and easy to lose
- hard to share and review
- unable to serve as stable regression guard

## Options

1. Script migration only
- Improves reproducibility, but no pass/fail quality gate.

2. Migration + threshold gate (recommended base)
- Adds explicit quality contract (`hit@5 >= threshold`) with minimal complexity.

3. Migration + gate + comparison report (recommended full)
- Adds decision-ready evidence for progress tracking and release notes.

## Chosen Design

### 1) Tooling Layout

Add `tools/locomo/` with:

- `baseline.sh`
- `qa-proxy.sh`
- `official-like-eval.sh`
- `assert-hit-threshold.py`
- `run-regression-gate.sh`
- `README.md`

Design principle:

- keep original script behavior compatible
- add environment switches for reproducible local runs
- keep artifacts in `/tmp` by default

### 2) Threshold Gate

`run-regression-gate.sh` flow:

1. run `baseline.sh` with configurable `K` and `BASE_DIR`
2. parse result JSON
3. assert `overall >= THRESHOLD` (default `0.05`)
4. fail fast with clear message and metric payload when below threshold

### 3) Comparison Report

Add a report doc under `docs/superpowers/releases/`:

- baseline (`R0`) metrics from initial known result
- R1/R2 current metrics from repo-managed scripts
- overall and by-category table
- commands and artifact paths

### 4) Validation

Keep existing required matrix:

- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

Plus execution checks:

- `tools/locomo/run-regression-gate.sh 5 0.05`
- `tools/locomo/official-like-eval.sh 5`

## Risks and Mitigations

Risk: dataset fetch may fail in restricted network.
Mitigation: support pre-provisioned `LOCOMO_DATA_PATH`/`LOCOMO_REPO` env and emit explicit guidance.

Risk: false confidence from one run.
Mitigation: persist command lines, artifacts, and metric snapshots in report.

## Success Criteria

1. LoCoMo scripts are repository-owned and executable
2. regression gate fails when threshold is violated
3. report captures R0 vs current results with reproducible commands
4. required Rust/Python/Node/MCP verification matrix remains green
