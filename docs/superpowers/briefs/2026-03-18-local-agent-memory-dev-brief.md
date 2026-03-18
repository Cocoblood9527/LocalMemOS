# Local Agent Memory Dev Brief

## What You Are Building

Build `v1` of a local-first memory system for personal AI agents.

This is **not** a general memory OS. It is a **structured fact memory kernel** with:

- fast local access
- explicit writes
- exact retrieval first
- time-aware history
- thin SDK, HTTP, and MCP adapters

## Core Product Boundaries

### Must Have

- Rust core as the single source of truth
- SQLite as the default storage engine
- fact memory only
- explicit write path via `upsert_fact`
- exact and filtered recall as the main retrieval path
- FTS only as fallback
- version history with `valid_from` and `valid_to`
- namespace isolation with:
  - `user`
  - `runtime`
  - `workspace`
- thin Python SDK
- thin Node SDK
- thin HTTP API
- thin MCP adapter

### Must Not Add in `v1`

- automatic extraction from conversation or logs
- vector retrieval as a required path
- graph database dependency
- cloud sync
- multi-user auth or tenancy
- reflective, emotional, episodic, or procedural memory systems
- heavy reranking pipelines

## Architecture Rules

- `memory-core` owns schema, validation, transactions, write semantics, recall semantics, history semantics, and namespace rules.
- SDKs are wrappers, not alternate implementations.
- MCP is only a protocol adapter.
- HTTP is only a local process-external adapter.
- No adapter is allowed to invent new memory behavior.

## Data Model

Implement these primary concepts:

- `Fact`
  - current effective fact
  - keyed by `namespace + scope_id + entity + attribute`
- `FactVersion`
  - historical versions for the same fact key
- `Evidence`
  - source metadata for explainability
- namespace semantics
  - required on writes and reads

Recommended key fields:

- `namespace`
- `scope_id`
- `entity`
- `attribute`
- `value_json`
- `value_text`
- `confidence`
- `valid_from`
- `valid_to`
- `updated_at`

## Required Primitives

Implement these core operations:

- `upsert_fact`
- `recall`
- `list`
- `forget`
- `history`

Rules:

- `upsert_fact` replaces the current value for the fact key but preserves old versions
- `forget` should logically retire facts by default, not hard-delete them
- default `recall` returns only current effective facts
- history queries must expose past versions

## Retrieval Rules

Use this order:

1. exact recall by `namespace + scope_id + entity + attribute`
2. structured filtering
3. FTS fallback for text recall

Do not make semantic search the default path.

## Conflict Rule

One fact key has one current effective value.

If a new write changes the value:

- close the old version
- insert the new version
- preserve history
- keep default recall simple

## Execution Order

Build in this order:

1. Rust workspace bootstrap
2. `memory-core` request validation and schema bootstrap
3. `upsert_fact` + history
4. `recall`, `list`, `forget`
5. FTS fallback
6. HTTP adapter
7. Python SDK
8. Node SDK
9. MCP adapter
10. cross-adapter consistency tests

## Testing Requirements

At minimum, cover:

- same-key upsert replaces current value and preserves history
- historical lookup returns the correct version by time
- namespace isolation prevents cross-scope leakage
- exact recall returns current effective facts only
- FTS works only as fallback
- Python, Node, HTTP, and MCP match core semantics

## Practical Constraints

- Prefer embedded mode first: SDK -> Rust core
- HTTP and MCP are secondary access modes
- Keep packaging simple and local-first
- The current workspace is not a Git repo yet, so initialize Git before following commit-based workflow

## Source Documents

Use these as the authoritative references:

- Spec: `docs/superpowers/specs/2026-03-18-local-agent-memory-design.md`
- Plan: `docs/superpowers/plans/2026-03-18-local-agent-memory-implementation-plan.md`

If anything conflicts with your assumptions, follow the spec first, then the implementation plan.
