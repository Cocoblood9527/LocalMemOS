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

## Prerequisites

Requirements:

- Rust toolchain
- Python 3
- Node.js
- Corepack

## Build and Test

This section contains the concrete commands used to verify the current repository state.

## Runtime Paths

- Embedded mode via Python SDK
- Embedded mode via Node SDK
- Local service mode via HTTP
- Tool integration via MCP

## Development Notes

- `RecallRequest.include_history` is reserved for compatibility in `v1` and is currently ignored by core recall logic.
- `GET /facts/{id}/history` expects `{id}` to be a stored `facts.id` row id returned by write/read APIs, and resolves it to the full logical fact version chain.
