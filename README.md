# localmemos

Local-first fact memory kernel workspace (`v1`).

## Project Summary

`localmemos` is a local-first fact memory kernel for personal AI agents. The current repository represents a `v1 MVP` focused on explicit fact writes, exact retrieval, time-aware history, and thin access adapters rather than a heavy memory operating system.

Deeper design and implementation context lives under `docs/superpowers/`. This README is the short operational entry point for continuing work on the repository.

## Current Status

This section summarizes what `v1` currently includes and what remains intentionally out of scope.

## Repository Layout

This section explains the main repository directories and their responsibilities.

## Prerequisites

This section lists the local tools needed to build, test, and verify the project.

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
