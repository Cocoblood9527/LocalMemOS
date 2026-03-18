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

## Repository Layout

- `crates/memory-core`: core storage, write semantics, recall semantics, and history logic
- `crates/memory-http`: HTTP adapter for local process-external access
- `crates/memory-node`: native Node binding layer
- `packages/node`: Node SDK wrapper over the native binding
- `packages/mcp`: MCP adapter built on top of the Node SDK
- `python`: Python binding and SDK surface
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

## Common Setup Failures

| Symptom | Likely Cause | Minimal Fix |
| --- | --- | --- |
| `ModuleNotFoundError: memory_sdk` when running pytest | Local Python package not installed into `.venv` | `./.venv/bin/python -m pip install -e python` |
| `pnpm: command not found` or Corepack error | Corepack not enabled in current shell | `corepack enable` then rerun `corepack pnpm --dir packages/node install` and `corepack pnpm --dir packages/mcp install` |
| Test commands fail with missing files or wrong paths | Commands executed outside repository root | `cd <your-localmemos-path>` then rerun setup and verification commands from this README |
| Tests fail after dependency updates or environment changes | Stale local artifacts in `.venv`, `target`, or package build outputs | Run `cargo clean`, remove and recreate `.venv` (`rm -rf .venv && python3 -m venv .venv`), then rerun First-Time Setup and the four verification commands |
| Native build fails with Rust toolchain not found | Rust toolchain is missing or not on `PATH` | Install Rust via `rustup`, reopen shell, verify `cargo --version`, then rerun setup and tests |

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
