# Local Personal AI Agent Memory Design

## Overview

This document defines the `v1` design for a local-first memory system intended primarily for personal AI agents running on a single machine.

The design optimizes for:

- fast local access
- high factual accuracy
- lightweight deployment
- explicit and controllable writes
- compatibility with SDK, MCP, and HTTP API access patterns

The system is intentionally scoped as a fact memory kernel, not a general-purpose agent operating system or full autonomous memory platform.

## Goals

- Provide a shared memory core that supports embedded SDK access and local service access.
- Store durable facts for local personal agents with clear namespace isolation.
- Prioritize exact retrieval and filtered lookup over semantic-first retrieval.
- Preserve history and time validity for changing facts.
- Keep the `v1` dependency footprint minimal enough for local single-user use.
- Expose a consistent memory model across Rust core, Python SDK, Node SDK, MCP server, and HTTP API.

## Non-Goals

`v1` does not include:

- automatic memory extraction from conversations or logs
- vector search on the primary retrieval path
- heavy graph reasoning or graph database dependency
- episodic, emotional, reflective, or procedural memory as first-class domains
- multi-user or cloud-first architecture
- complex reranking pipelines
- mandatory always-on service deployment

## Design Inputs From Existing Projects

This design borrows selectively from several open-source systems while intentionally avoiding their heavier assumptions.

- `qmd`: local-first retrieval mindset, staged retrieval, and context-oriented access patterns
- `mem0`: namespace awareness and memory primitives exposed through SDK and API layers
- `Letta`: separation between actively used memory and persistent archival memory
- `OpenMemory`: time-aware facts, history preservation, and evidence-oriented design
- `memU`: light vs deep retrieval framing and hierarchical memory organization ideas
- `EverMemOS`: evaluation mindset and retrieval mode stratification
- `AutoMem`: treating MCP as a protocol adapter instead of a memory core

The following ideas are intentionally deferred beyond `v1`:

- automatic extraction and agentic memory curation
- vector-first retrieval
- service-heavy storage stacks
- rich graph-native dependency as a core requirement

## Product Positioning

The system is defined as:

> a local, structured, time-aware, explainable fact memory kernel for personal AI agents

This positioning is meant to keep implementation and future evolution focused. The system is not trying to remember everything. It is trying to remember stable and useful facts correctly.

## Core Constraints Confirmed

- deployment model: same core, multiple access forms
- memory type priority: fact memory first
- write mode: explicit writes first
- retrieval mode: structured exact retrieval first
- scope model: layered namespaces
- storage dependency: SQLite-class lightweight storage
- language ecosystem: Python and Node both matter
- core implementation direction: Rust
- conflict policy: overwrite current value while preserving history
- external interface priority: minimal primitives first
- history model: time validity and version history included in `v1`

## Architecture

The architecture is split into four layers.

### 1. `memory-core`

The Rust core is the only source of truth for:

- fact schema
- namespace rules
- write semantics
- conflict handling
- version history
- retrieval logic
- SQLite transactions and indexing

The core must not depend on MCP or HTTP semantics.

### 2. SDK Layers

Two SDKs sit above the core:

- `memory-sdk-python`
- `memory-sdk-node`

Their job is to:

- map language-native types to the core request and response model
- provide ergonomic wrappers over the core primitives
- avoid introducing behavior that does not exist in the core

SDK helpers may improve usability, but they must be reducible to core primitives.

### 3. `memory-mcp-server`

The MCP server is a protocol adapter for local agent tool ecosystems.

It:

- exposes a narrow set of structured tools
- does not implement its own storage logic
- does not become a second memory system
- forwards requests into the same core semantics used by SDKs and HTTP

### 4. `memory-http-api`

The HTTP layer is an optional local service mode for process-external access.

It exists for:

- local desktop tools
- agent runners outside the host process
- process boundaries where SDK embedding is not convenient

It is not the required default runtime.

## Runtime Modes

The design supports two operating modes from the same core:

### Embedded Mode

- preferred for speed-sensitive local agents
- SDK calls go directly into the core
- no always-on server required

### Local Service Mode

- optional mode for HTTP and MCP access
- useful when multiple local clients need shared access
- still backed by the same SQLite store and core semantics

The default priority is:

1. SDK direct access
2. MCP for tool ecosystem integration
3. HTTP for local inter-process integration

## Namespace Model

Namespaces are first-class and required on writes and retrievals.

`v1` includes three namespaces:

- `user`
- `runtime`
- `workspace`

Each namespace also includes a `scope_id` to identify the concrete target inside that namespace.

Examples:

- `user/default`
- `runtime/macbook-pro`
- `workspace/localmemos`

The system should require explicit namespace and `scope_id` whenever possible to prevent memory pollution across contexts.

## Data Model

`v1` uses four primary entities.

### 1. `Fact`

Represents the current effective version of a fact for normal recall.

Suggested fields:

- `id`
- `namespace`
- `scope_id`
- `entity`
- `attribute`
- `value_json`
- `value_text`
- `confidence`
- `status`
- `tags`
- `valid_from`
- `valid_to`
- `created_at`
- `updated_at`

The key shape is conceptually:

`namespace + scope_id + entity + attribute`

Examples:

- `workspace/localmemos :: project.preferred_package_manager = bun`
- `user/default :: preference.language = zh-CN`

### 2. `FactVersion`

Represents the historical versions of a fact over time.

Each change:

- closes the previous version by setting `valid_to`
- writes a new version entry
- preserves a traceable history of what changed and when

The default read path should not need to scan full history.

### 3. `Evidence`

Represents where a fact came from.

Suggested fields:

- `id`
- `fact_id` or `fact_version_id`
- `source_kind`
- `source_ref`
- `summary`
- `created_at`

Example source kinds:

- `manual`
- `agent`
- `file`
- `mcp`
- `http`

Evidence is important for explainability and trust, especially when agents later consume the memory.

### 4. `Namespace`

Namespace semantics are critical even if they are implemented as lightweight metadata rather than a heavy relational object.

The namespace layer defines:

- isolation boundaries
- default query scope
- permission or policy expansion points for later versions

## Write Semantics

`v1` uses explicit write operations.

The primary write primitive is `upsert_fact`.

Write flow:

1. validate request shape and namespace
2. look up current effective fact using `namespace + scope_id + entity + attribute`
3. if no current fact exists, insert a new one
4. if current value matches, refresh timestamps and optionally append evidence
5. if current value differs, close the old version and insert a new effective version

This preserves a single current truth per fact key while retaining history.

## Conflict Handling

The confirmed default conflict rule is:

- one current effective value per fact key
- changed values replace the current value
- historical values remain queryable through version history

This keeps default recall simple while preserving traceability.

## Time Validity Model

Time awareness is included in `v1`.

Each fact version supports:

- `valid_from`
- `valid_to`

Normal retrieval returns the currently effective version.

Optional historical retrieval can answer:

- what was true at time `T`
- when did a fact change
- what value was superseded

This model is especially useful for:

- project configuration changes
- preference updates
- runtime environment changes

## Retrieval Model

The `v1` retrieval philosophy is:

> exact first, filtered second, text-assisted last

### Stage 1: Exact Retrieval

Primary path for speed and accuracy.

Use explicit keys and filters:

- `namespace`
- `scope_id`
- `entity`
- `attribute`

This path should serve the majority of production lookups.

### Stage 2: Structured Filtering

When exact keys are incomplete, retrieval can filter by:

- tags
- source kind
- updated time
- confidence
- namespace and scope

This remains fully structured and lightweight.

### Stage 3: Full-Text Compensation

If the caller only has natural language or partial wording, use SQLite FTS as a fallback over:

- `attribute`
- `value_text`
- evidence summaries

FTS is a support layer, not the primary retrieval system.

## Access Primitives

The core semantic interface should stay intentionally small.

Recommended primitive set:

- `upsert_fact`
- `recall`
- `forget`
- `list`
- `history`

Notes:

- SDKs may expose `remember` as a friendlier alias for `upsert_fact`
- the core should keep the more explicit verb
- `forget` should logically retire or invalidate facts by default, not hard-delete them

## Unified Request Shape

All access paths should converge on the same logical request model.

Example:

```json
{
  "namespace": "workspace",
  "scope_id": "localmemos",
  "entity": "project",
  "attribute": "preferred_package_manager",
  "value": "bun",
  "confidence": 0.95,
  "source": {
    "kind": "manual",
    "ref": "user-confirmed"
  },
  "tags": ["tooling", "preference"],
  "valid_from": "2026-03-18T00:00:00Z"
}
```

The important consistency rule is that all callers must supply enough context to avoid ambiguity, especially:

- `namespace`
- `scope_id`
- `source.kind`

## SDK Design

Python and Node SDKs should remain structurally aligned.

Each SDK may expose:

- low-level core-mapped primitives
- ergonomic helper functions

Helpers must:

- compose the primitive layer
- not introduce new storage semantics
- not fork retrieval rules

This keeps behavior consistent across languages.

## MCP Design

The MCP interface should use narrow, explicit tools.

Suggested `v1` tools:

- `memory_upsert_fact`
- `memory_recall`
- `memory_list`
- `memory_forget`
- `memory_history`

MCP responses should stay structured so that the consuming agent decides how to incorporate results into context.

## HTTP API Design

The HTTP layer should mirror the same primitives.

Suggested routes:

- `POST /facts:upsert`
- `POST /facts:recall`
- `POST /facts:list`
- `POST /facts:forget`
- `GET /facts/{id}/history`

RPC-style routing is acceptable here because the system is an agent-facing memory service rather than a generic human-facing REST backend.

## Error Model

Error semantics should be consistent across all adapters.

Suggested categories:

- `validation_error`
- `conflict_error`
- `storage_error`
- `query_error`

Error responses should be recoverable by callers when possible. The system should prefer explicit failure over silent corruption or silent drops.

## Performance Strategy

`v1` performance comes from scope control rather than heavy optimization.

Recommended approach:

- keep current effective facts on the main query path
- keep history storage separate from default recall paths
- index around exact retrieval and filtered lookup
- use FTS only when exact or filtered retrieval is insufficient
- prefer in-process SDK access for latency-sensitive paths

Priority indexes should likely include:

- `namespace + scope_id + entity + attribute`
- `namespace + scope_id + updated_at`
- `namespace + scope_id + confidence`

Exact index strategy can be finalized during implementation planning.

## Testing Strategy

The first test priority is semantic correctness, not just CRUD coverage.

Minimum test groups:

### 1. Fact Upsert and Replacement

Verify that:

- same fact key updates correctly
- old versions are closed correctly
- new versions become the default effective value

### 2. Time-Aware Retrieval

Verify that:

- current queries return current facts
- historical queries return facts valid at the requested time

### 3. Namespace Isolation

Verify that:

- writes do not bleed across namespaces
- reads respect namespace and `scope_id`

### 4. Cross-Adapter Consistency

Verify that:

- Rust core
- Python SDK
- Node SDK
- MCP layer
- HTTP layer

all preserve the same write and recall semantics.

## Benchmark Suggestions

Early benchmarks should focus on practical local use cases:

- single fact `upsert` latency
- exact `recall` P50 and P95
- filtered `recall` P50 and P95
- history query latency
- correctness rate for conflicting updates

## `v1` Scope Guardrails

To prevent accidental scope creep, implementation planning should reject additions that require:

- automatic extraction pipelines
- embedding generation as a hard dependency
- vector database setup
- graph database setup
- multi-user auth and tenancy
- cloud synchronization requirements
- agent-driven summarization as a required storage path

## `v2` Evolution Path

If `v1` proves stable, recommended expansion order is:

1. candidate memory extraction with human or agent confirmation
2. `light recall` and `deep recall` tiers
3. optional vector-backed retrieval plugins
4. lightweight relation edges or graph views on top of the fact model
5. import/export and local sync tooling

## Open Questions For Planning

These questions do not block the design, but should be resolved during implementation planning:

- how Rust bindings should be delivered to Python and Node
- exact SQLite schema layout and migration strategy
- whether FTS is enabled by default or lazily created
- how to package embedded mode and local service mode for development and release

## Summary

`v1` should be built as a lightweight, local-first, structured fact memory kernel with:

- Rust as the core implementation
- SQLite as the default storage engine
- explicit writes
- exact retrieval first
- time-aware history
- namespace isolation
- thin SDK, MCP, and HTTP adapters

That scope is intentionally smaller than a full memory operating system. The goal is to make local personal agents reliably remember the right facts, quickly and predictably.
