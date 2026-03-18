# localmemos

Local-first fact memory kernel workspace (`v1`).

## Runtime Modes

- Embedded mode via Python SDK
- Embedded mode via Node SDK
- Local service mode via HTTP
- Tool integration via MCP

## v1 API Notes

- `RecallRequest.include_history` is reserved for compatibility in `v1` and is currently ignored by core recall logic.
- `GET /facts/{id}/history` expects `{id}` to be a stored `facts.id` row id returned by write/read APIs, and resolves it to the full logical fact version chain.
