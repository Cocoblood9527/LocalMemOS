# V2 LoCoMo Retrieval Lift Design

## Goal

Lift LoCoMo evidence retrieval quality first, before any full LLM answer-model benchmarking.

Primary target (K=5):

- `evidence hit@5`: from `0.001` to `>= 0.05`

Known baseline context:

- `naive_overall_accuracy = 0.2329`
- `oracle_overall_accuracy = 0.2338`

Interpretation: current bottleneck is retrieval hit rate, not answer generation.

## Scope

In scope:

- improve `memory-core` text recall ranking behavior for natural-language queries
- keep API compatibility across Rust/HTTP/Python/Node/MCP adapters
- add regression tests for query rewrite + ranking

Out of scope:

- vector database / embedding retrieval
- full LoCoMo LLM answer-model pipeline changes
- schema-breaking API changes

## Problem Analysis

Current `recall_with_fts` behavior uses raw `text_query` directly in `facts_fts MATCH ?`.

This has two major issues:

1. Natural-language questions become strict FTS expressions (implicit hard conjunction), often producing near-empty recall.
2. Results are ordered by recency (`updated_at DESC`) instead of relevance score, which hurts `top5` evidence hit.

## Options Considered

1. Query rewrite only (OR tokenization)
- Minimal change, low risk, but ranking remains weak.

2. Query rewrite + relevance ranking (recommended)
- Add token normalization and OR-prefix query generation.
- Rank by FTS `bm25` and lightweight lexical overlap tie-break.
- Best cost/performance tradeoff for this round.

3. Multi-stage retrieval architecture
- Higher long-term ceiling but larger scope and slower iteration.

## Chosen Design (V2-R1)

### 1) Query Normalization

Build a normalized token list from raw question text:

- lowercase
- keep ASCII letters/digits/underscore
- split on non-word boundaries
- drop short/empty tokens and common stopwords
- deduplicate while preserving order

### 2) FTS Query Rewrite

Translate natural-language input into an FTS5-safe disjunction:

- `token1* OR token2* OR ...`
- prefix search enabled with `*`
- bounded token count to control recall noise

Fallback:

- if normalization yields no valid tokens, return empty result set (not SQL parse error).

### 3) Relevance-First Ranking

Replace recency-first ordering with relevance-aware ordering:

- SQL primary rank: `bm25(facts_fts, ...)` (column-weighted)
- secondary tie-break: lexical overlap score across `entity`, `attribute`, `value_text`, `evidence_summary`
- final tie-break: `updated_at DESC`

### 4) Compatibility

No request/response schema changes in this round.

- `RecallRequest` unchanged
- all adapters keep current JSON contract

## Files / Components

- `crates/memory-core/src/fts.rs`
  - query normalization and FTS query construction helpers
- `crates/memory-core/src/store.rs`
  - V2 recall text path and ranking update
- `crates/memory-core/tests/fts_fallback.rs`
  - regression tests for NL query rewrite and ranking behavior

## Testing Strategy

Unit/integration tests must cover:

- natural-language question now retrieves expected fact
- punctuation-heavy query no longer causes FTS parse failure
- ranking prefers more lexically aligned evidence over weaker matches
- existing time-slice (`as_of`) semantics remain valid

Validation commands per round:

- `cargo test --workspace`
- `./.venv/bin/pytest python/tests -q`
- `corepack pnpm --dir packages/node test`
- `corepack pnpm --dir packages/mcp test`

## Risks and Mitigations

Risk: OR query may increase noise.
Mitigation: token cap + stopword removal + bm25 + lexical tie-break.

Risk: behavior drift for existing consumers.
Mitigation: keep API shape unchanged and preserve existing tests.

Risk: no internet access may block full LoCoMo script execution.
Mitigation: keep `/tmp` scripts intact; validate core behavior with deterministic tests, and run LoCoMo scripts when dataset source is reachable.

## Success Criteria

1. test suite stays green across Rust/Python/Node/MCP
2. new retrieval tests prove V2 behavior changes
3. when LoCoMo dataset is accessible, `hit@5 >= 0.05` on baseline script path
