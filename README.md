# localmemos

Local-first fact memory kernel workspace (`v1`).

## Project Summary

`localmemos` is a local-first fact memory kernel for personal AI agents. The current repository represents a `v1 MVP` focused on explicit fact writes, exact retrieval, time-aware history, and thin access adapters rather than a heavy memory operating system.

Deeper design and implementation context lives under `docs/superpowers/`. This README is the short operational entry point for continuing work on the repository.

## Current Status

Supported in `v1`:

- Rust `memory-core` as the single source of truth for memory semantics
- SQLite-backed fact storage
- explicit fact writes via `upsert_fact`
- exact recall and structured filtering
- FTS fallback for text recall
- time-aware history and `as_of` recall
- HTTP, Python, Node, and MCP access paths

Out of scope for `v1`:

- automatic memory extraction
- vector retrieval and vector databases
- graph-native memory and graph reasoning
- cloud sync
- multi-user or multi-tenant support
- reflective, episodic, emotional, or procedural memory systems

## V1 Done Definition

`v1` is considered complete only when all of the following remain true:

- scope stays within current `v1` boundaries (no automatic extraction, vector-first retrieval, graph-native memory, or cloud/multi-tenant expansion)
- `memory-core` remains the single semantic source of truth for write/recall/history behavior
- HTTP/Python/Node/MCP adapters remain thin and semantically aligned with core primitives
- required verification commands pass in order:
  - `cargo test --workspace`
  - `./.venv/bin/pytest python/tests -q`
  - `corepack pnpm --dir packages/node test`
  - `corepack pnpm --dir packages/mcp test`

## Role-Based Operation Paths

Use the role path that best matches your current change. If unsure, start with `Clean-Run Verifier`, then apply `Failure Triage Rule`.

### Docs Maintainer

- Use this path when: editing README commands, setup wording, or workflow guidance without code behavior changes.
- Read first: `Verification Path`, `When To Re-Validate Commands`, `Minimum Regression Order`, `Failure Triage Rule`.
- Run at minimum: `Minimum Regression Order` after command-bearing doc edits.
- Escalate when: a README command no longer matches repository reality (documentation drift).

### Core Semantics Maintainer

- Use this path when: changing `crates/memory-core` write/recall/history semantics.
- Read first: `Current Dev Workflow Notes` and `Repository Layout` (`memory-core` remains semantic source of truth).
- Run at minimum: full `Minimum Regression Order`, keeping command order unchanged.
- Escalate when: tests fail with valid commands and stable environment (code regression).

### Adapter Surface Maintainer

- Use this path when: changing HTTP/Python/Node/MCP adapter layers without redefining `memory-core` semantics.
- Read first: `Repository Layout`, `Verification Path`, and `Failure Triage Rule`.
- Run at minimum: full `Minimum Regression Order`; adapters must stay aligned with core semantics.
- Escalate when: adapter behavior diverges from core semantics or verification fails after environment recovery.

### Clean-Run Verifier

- Use this path when: validating local machine state, onboarding, or recovering from inconsistent setup.
- Read first: `First-Time Setup`, `Clean-Run Checklist (macOS + Linux)`, `Clean-Run Validation Matrix (macOS + Linux)`.
- Run at minimum: `Verification Path` in listed order from repository root.
- Escalate when: matrix recovery cannot restore a clean run, then classify via `Failure Triage Rule`.

## 30-Second Path Picker

Pick one row, start from the linked section, then run the minimum command action.

| If your change is... | Use role path | Start here | Minimum command action |
| --- | --- | --- | --- |
| README command or workflow wording updates | `Docs Maintainer` | `When To Re-Validate Commands` + `Minimum Regression Order` | Run `Minimum Regression Order` after command-bearing edits |
| `memory-core` write/recall/history semantics | `Core Semantics Maintainer` | `Current Dev Workflow Notes` + `Verification Path` | Run full four-command sequence in listed order |
| HTTP/Python/Node/MCP adapter-layer updates | `Adapter Surface Maintainer` | `Repository Layout` + `Verification Path` | Run full four-command sequence in listed order |
| Local setup/onboarding/recovery checks | `Clean-Run Verifier` | `First-Time Setup` + `Clean-Run Validation Matrix (macOS + Linux)` | Run `Verification Path` from repository root |

Minimum command set (same as `Verification Path` and `Minimum Regression Order`):

```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```

If role selection or failure classification is unclear, apply `Failure Triage Rule`.

## Change-to-Validation Mapping

Use this table to pick first-pass checks from touched areas. Before closing a round, run full `Minimum Regression Order`.

| If you changed... | First-pass checks | Then |
| --- | --- | --- |
| `README.md` or docs-only workflow wording | `cargo test --workspace` | Run full `Minimum Regression Order` before closure |
| `crates/memory-core/**` | `cargo test --workspace` | Then run `./.venv/bin/pytest python/tests -q`, `corepack pnpm --dir packages/node test`, `corepack pnpm --dir packages/mcp test` |
| `crates/memory-http/**` | `cargo test --workspace` | Then run `./.venv/bin/pytest python/tests -q` and full `Minimum Regression Order` |
| `python/**` | `./.venv/bin/pytest python/tests -q` | Then run full `Minimum Regression Order` |
| `packages/node/**` | `corepack pnpm --dir packages/node test` | Then run `corepack pnpm --dir packages/mcp test` and full `Minimum Regression Order` |
| `packages/mcp/**` | `corepack pnpm --dir packages/mcp test` | Then run full `Minimum Regression Order` |
| mixed or unclear multi-surface changes | run full `Minimum Regression Order` immediately | Classify failures with `Failure Triage Rule` |

If scope is unclear or failure classification is ambiguous at any point, apply `Failure Triage Rule` and use full `Minimum Regression Order`.

## Release Closure Checklist

Use this checklist for each closure round and for final `v1` release preparation:

1. Run the four required verification commands in order.
2. If any step fails, apply `Failure Triage Rule` before making more changes.
3. Keep commit sequence explicit: `spec -> plan -> implementation`.
4. Push `main` after verification and commit checks pass.
5. Create and push stable tag using `v1.x-docs-stable` naming.

## Repository Layout

- `crates/memory-core`: core storage, write semantics, recall semantics, and history logic
- `crates/memory-http`: HTTP adapter for local process-external access
- `crates/memory-node`: native Node binding layer
- `packages/node`: Node SDK wrapper over the native binding
- `packages/mcp`: MCP adapter built on top of the Node SDK
- `python`: Python binding and SDK surface
- `tools/locomo`: LoCoMo retrieval evaluation scripts and regression gate
- `docs/superpowers`: design, planning, and project workflow documents

## Environment Setup

You will need:

- Rust toolchain
- Python 3.10+ with virtual environment support
- Node.js
- Corepack

## First-Time Setup

Run this once after cloning:

```bash
python3 -m venv .venv
./.venv/bin/python -m pip install -U pip pytest maturin
./.venv/bin/python -m pip install -e python
corepack enable
corepack pnpm --dir packages/node install
corepack pnpm --dir packages/mcp install
```

- Python tests run from `./.venv`.
- Install the local Python package into the venv before running pytest.
- Node and MCP packages manage dependencies independently.

## Clean-Run Checklist (macOS + Linux)

1. Run commands from the repository root (the directory that contains `README.md` and `Cargo.toml`).
2. Ensure `.venv` exists, use explicit `./.venv/bin/...` calls, and confirm editable install with `./.venv/bin/python -m pip install -e python`.
3. Ensure `corepack enable` has been run in your shell.
4. Ensure dependencies are installed for both `packages/node` and `packages/mcp`.
5. Run all four verification commands in order.

- All commands in this README assume the repository root as working directory.
- If local state looks inconsistent, rerun First-Time Setup commands before deeper debugging.

## Verification Path

Run these checks for a healthy local state:

```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```

If all four commands pass, the Rust core and Python/Node/MCP access surfaces are in sync locally.

## LoCoMo Retrieval Path

Use repository-owned scripts for retrieval-focused V2 tracking:

```bash
tools/locomo/baseline.sh 5
tools/locomo/qa-proxy.sh 5
tools/locomo/official-like-eval.sh 5
tools/locomo/run-regression-gate.sh 5
tools/locomo/run-category-gate.sh 5
tools/locomo/refresh-failure-samples.sh 5 80
```

- Scripts write artifacts to `/tmp` by default (or `BASE_DIR` when provided).
- `run-regression-gate.sh` default threshold is `hit@5 >= 0.55` (second arg can override).
- `run-category-gate.sh` also checks default category floors for `multi-hop` and `open-domain`.
- If LoCoMo scores unexpectedly remain old, rebuild local Python binding:
  `./.venv/bin/python -m pip install -e python`

## When To Re-Validate Commands

Run `Minimum Regression Order` at least once after any of the following:

- dependency manifest or lockfile changes (Rust/Python/Node)
- Rust/Python/Node runtime or toolchain updates
- edits to command-bearing sections in this README
- crate or package changes that affect build/test surfaces

## Minimum Regression Order

This is the minimum default order for command drift checks. Expand it for broader change sets.

```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```

## Failure Triage Rule

- If a README command no longer matches repository reality, treat it as documentation drift and update README.
- If commands are valid but local machine state is inconsistent, treat it as an environment issue and apply matrix recovery.
- If commands are valid and environment is stable but tests fail, treat it as a code regression and escalate.

## Clean-Run Validation Matrix (macOS + Linux)

`First-Time Setup` remains the source of truth for command order. This matrix is a pass/fail and quick-recovery overlay for each command.

| Command | Expected Result | If Failed | Quick Recovery |
| --- | --- | --- | --- |
| `python3 -m venv .venv` | `.venv` directory is created with no error output | `.venv` not created or Python venv error | Verify Python 3.10+ is available (`python3 --version`), then rerun from repository root |
| `./.venv/bin/python -m pip install -U pip pytest maturin` | pip exits successfully and packages are installed/updated | network or package install error | Retry command; if pip is broken, recreate `.venv` and rerun First-Time Setup |
| `./.venv/bin/python -m pip install -e python` | `memory-sdk` installs successfully in editable mode | `memory_sdk` remains unavailable in tests | Rerun command from repository root and confirm `python/pyproject.toml` exists |
| `corepack enable` | command exits successfully with no error | corepack not found or permission error | Ensure Node.js includes Corepack, restart shell, rerun command |
| `corepack pnpm --dir packages/node install` | Node package dependencies install without error | install fails or lockfile/dependency resolution issue | Rerun from repository root; if still failing, remove package cache artifacts and rerun install |
| `corepack pnpm --dir packages/mcp install` | MCP package dependencies install without error | install fails or lockfile/dependency resolution issue | Rerun from repository root; if still failing, remove package cache artifacts and rerun install |
| `cargo test --workspace` | all Rust workspace tests pass | compile/test failure in Rust workspace | Run `cargo clean` and rerun; confirm Rust toolchain is installed and on `PATH` |
| `./.venv/bin/pytest python/tests -q` | pytest reports all tests passed | `ModuleNotFoundError` or Python test failure | Rerun `./.venv/bin/python -m pip install -e python`, then rerun pytest |
| `corepack pnpm --dir packages/node test` | vitest reports Node tests passed | native build/test failure in Node package | Rerun `corepack pnpm --dir packages/node install`, then rerun test |
| `corepack pnpm --dir packages/mcp test` | vitest reports MCP tests passed | native build/test failure in MCP package | Rerun `corepack pnpm --dir packages/mcp install`, then rerun test |

## Common Setup Failures

| Symptom | Likely Cause | Minimal Fix |
| --- | --- | --- |
| `ModuleNotFoundError: memory_sdk` when running pytest | Local editable install is missing or stale | Use the matrix row for `./.venv/bin/python -m pip install -e python` and then rerun pytest |
| `pnpm: command not found` or Corepack error | Corepack not enabled in current shell | Use the matrix row for `corepack enable`, then rerun package install rows |
| Test commands fail with missing files or wrong paths | Commands executed outside repository root | `cd <your-localmemos-path>` and rerun from the matrix starting at the failed row |
| Tests fail after dependency updates or environment changes | Stale local artifacts in `.venv`, `target`, or package build outputs | Run `cargo clean`, recreate `.venv`, then rerun matrix rows from venv creation onward |
| Native build fails with Rust toolchain not found | Rust toolchain is missing or not on `PATH` | Install Rust via `rustup`, confirm `cargo --version`, then rerun matrix rows for install + test |

## Runtime Paths

- Python SDK: embedded local access from Python agents and tools
- Node SDK: embedded local access from Node tools
- HTTP: process-external local integration path
- MCP: tool-based integration for agent environments

## Current Dev Workflow Notes

- `memory-core` is the single semantic source of truth for write, recall, and history behavior.
- Thin adapters should follow core semantics rather than redefining them.
- This repository remains intentionally scoped as `v1 MVP`.
- `RecallRequest.include_history` is reserved for compatibility in `v1` and is currently ignored by core recall logic.
- `GET /facts/{id}/history` expects `{id}` to be a stored `facts.id` row id returned by write/read APIs, and resolves it to the full logical fact version chain.
- `v1` does not include automatic extraction, vector retrieval, vector databases, or graph-native memory features.
