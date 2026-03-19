# V1.0.0 Release Readiness

Date: 2026-03-19  
Target release: `v1.0.0`  
Release target commit: `081785f`

## Summary

`localmemos` meets the documented `v1` completion criteria:

- scope remains a local-first fact memory kernel
- `memory-core` remains semantic source of truth
- adapters remain thin and aligned (HTTP/Python/Node/MCP)
- required verification suite passes

This document maps completion criteria to repository evidence for formal release closure.

## Done Definition Mapping

### 1) Scope Boundaries Are Preserved

Evidence:

- README `Current Status` and `Out of scope for v1` explicitly retain boundaries (no automatic extraction/vector-first/graph-cloud/multi-tenant expansion).
- no new feature commits in `v1.5`-`v1.8`; rounds are docs and consistency hardening.

### 2) Semantic Authority Remains in `memory-core`

Evidence:

- README and architecture docs state `memory-core` as single semantic source.
- adapters call core primitives (`upsert`, `recall`, `list`, `forget`, `history`) without redefining memory semantics.

### 3) Adapter Consistency Is Verified

Evidence:

- existing tests already covered upsert/recall/history parity.
- `v1.8` adds explicit cross-adapter `forget` consistency checks:
  - Python: `python/tests/test_sdk_core.py`
  - Node: `packages/node/test/sdk.test.ts`
  - MCP: `packages/mcp/test/tools.test.ts`
  - HTTP: `crates/memory-http/tests/http_smoke.rs`

### 4) Required Verification Suite Passes

Required command sequence:

```bash
cargo test --workspace
./.venv/bin/pytest python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test
```

Latest run status before release publication: PASS (all 4 commands).

## Stabilization Milestone Chain

- `v1.1`: Getting Started usability baseline
- `v1.2`: clean-run troubleshooting
- `v1.3`: clean-run validation matrix (`v1-docs-stable`)
- `v1.4`: command drift guard
- `v1.5`: role-based operation paths (`v1.5-docs-stable`)
- `v1.6`: 30-second path picker (`v1.6-docs-stable`)
- `v1.7`: change-to-validation mapping (`v1.7-docs-stable`)
- `v1.8`: v1 done definition + release checklist + forget parity tests (`v1.8-docs-stable`)

## Release Closure Decision

Decision: proceed with formal `v1.0.0` tag and GitHub release on top of `081785f`.

Rationale:

- no unresolved verification failures
- completion criteria are explicit and satisfied
- stabilization rounds are complete and traceable
