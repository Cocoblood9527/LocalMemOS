# V2-R3 LoCoMo Targeted Re-rank Design

## Goal

Improve LoCoMo weak categories (`multi-hop`, `open-domain`) while preserving overall retrieval strength achieved in V2-R1/R2.

Primary targets (K=5):

- keep `overall hit@5 >= 0.55`
- raise `multi-hop` and `open-domain` hit@5 vs current baseline

Current reference (V2-R2):

- overall `hit@5 = 0.5822`
- `multi-hop = 0.4149`
- `open-domain = 0.3152`

## Scope

In scope:

- retrieval re-rank improvement in `memory-core` text recall path
- regression gate default threshold raise from `0.05` to `0.55`
- failure sample export tool + committed sample file for quick miss tracking

Out of scope:

- vector DB / embedding retrieval
- full answer model changes
- schema/API contract changes

## Problem

Current ranking is `bm25` first and lexical overlap tie-break. This works overall but is less robust when a question expresses intent across multiple clues (multi-hop) or less direct wording (open-domain).

## Options

1. Increase bm25 weights only
- Fast but brittle, low interpretability.

2. Add feature-based re-rank (recommended)
- Keep FTS candidate generation, then rank by blended relevance features:
  - token coverage ratio
  - bigram coverage ratio
  - exact phrase bonus for longer spans
  - bm25 as stabilizer
- Better control over multi-clue question behavior.

3. Add external reranker model
- Higher complexity and dependencies, not aligned with current V2 constraints.

## Chosen Design

### 1) Feature-based Relevance Score

For each candidate fact, compute:

- `token_coverage`: fraction of query tokens present in candidate text
- `bigram_coverage`: fraction of query bigrams present
- `phrase_bonus`: 1 when query contains a long contiguous phrase present in candidate text, else 0
- `bm25_signal`: transformed bm25 score as secondary stabilizer

Sort by blended score descending, then by recent update time.

### 2) Candidate Text Surface

Reuse existing candidate fields:

- `entity`
- `attribute`
- `value_text`
- `evidence_summary`

No schema change required.

### 3) Regression Gate Upgrade

Update `tools/locomo/run-regression-gate.sh` default threshold:

- from `0.05` to `0.55`

CLI override remains supported.

### 4) Failure Sample Set

Add failure sample exporter that records missed questions at K=5 with:

- `sample_id`, `question`, `category`, `evidence`, retrieved top attrs
- stored under `tools/locomo/failure-samples-k5.json`

This serves as quick regression triage input for later rounds.

## Test Strategy

- add unit/integration tests validating phrase and multi-clue ranking behavior
- run LoCoMo scripts from `tools/locomo` for metric evidence
- run required 4-command matrix:
  - `cargo test --workspace`
  - `./.venv/bin/pytest python/tests -q`
  - `corepack pnpm --dir packages/node test`
  - `corepack pnpm --dir packages/mcp test`

## Success Criteria

1. all tests remain green across Rust/Python/Node/MCP
2. regression gate passes with default threshold `0.55`
3. failure sample file is generated and committed
4. LoCoMo metrics show no overall regression and preferably improve weak categories
