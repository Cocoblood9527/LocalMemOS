# V2-R11 LoCoMo CI Full Gate Design

## Goal

Turn the current local LoCoMo full gate into an automatic CI guard so every PR can be blocked on retrieval regression risk, while preserving the current validated baseline:

- overall `hit@5 = 0.5822`
- multi-hop `hit@5 = 0.4255`
- open-domain `hit@5 = 0.3370`

## Problem

`tools/locomo/run-full-gate.sh` already gives a strong local quality gate, but it is still manually triggered. This leaves room for accidental merge of retrieval/tooling regressions when contributors skip or partially run checks.

## Options Considered

1. Keep local-only gate and rely on contributor discipline
- Lowest implementation cost, highest process risk.

2. Add a lightweight CI gate with only the 4-command matrix
- Faster CI, but does not protect LoCoMo retrieval baseline/drift.

3. Add GitHub Actions PR gate that runs full LoCoMo gate script (chosen)
- Slightly higher CI time, but directly enforces the retrieval baseline and required matrix in one canonical command.

## Chosen Design

### 1) New GitHub Actions workflow

Create `.github/workflows/locomo-full-gate.yml` with triggers:

- `pull_request` to `main`
- `push` to `main`
- `workflow_dispatch`

### 2) Standardized CI environment bootstrap

In workflow job:

- checkout repo
- setup Rust stable toolchain
- setup Python 3.11 + `.venv`
- install Python deps (`pip`, `pytest`, `maturin`, `pip install -e python`)
- setup Node 20 + Corepack
- install `packages/node` and `packages/mcp` deps with `pnpm`

### 3) Canonical gate execution in CI

Run:

```bash
REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5
```

This preserves one source of truth for gate logic and includes:

- threshold checks
- category checks
- drift checks
- required 4-command matrix

### 4) Artifact retention for troubleshooting

Persist gate log and generated result JSON as workflow artifacts to make CI failures diagnosable without rerunning locally.

## Test Strategy

1. Local command-level verification:
- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

2. Local LoCoMo full gate dry run:
- `REBUILD_PYTHON=0 tools/locomo/run-full-gate.sh 5`

3. CI syntax and path validation via workflow file review and local repo checks.

## Success Criteria

1. Workflow exists and is valid at `.github/workflows/locomo-full-gate.yml`.
2. Workflow runs one-command LoCoMo full gate on PRs to `main`.
3. Gate artifacts (log + result JSON when present) are uploaded.
4. Required local 4-command matrix passes in this round.
