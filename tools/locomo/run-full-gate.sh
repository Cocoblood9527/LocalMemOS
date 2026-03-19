#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
K="${1:-5}"
OVERALL="${2:-0.55}"
MULTI_HOP="${3:-0.42}"
OPEN_DOMAIN="${4:-0.32}"
GUARDRAILS="${5:-$ROOT/tools/locomo/baseline-guardrails-k5.json}"
REBUILD_PYTHON="${REBUILD_PYTHON:-1}"
BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-full-gate.XXXXXX)}"
RESULT_JSON="$BASE_DIR/result_hit_at_${K}.json"

cd "$ROOT"

if [[ "$REBUILD_PYTHON" == "1" ]]; then
  "$ROOT/.venv/bin/python" -m pip install -e python
fi

BASE_DIR="$BASE_DIR" "$ROOT/tools/locomo/baseline.sh" "$K"

"$ROOT/.venv/bin/python" "$ROOT/tools/locomo/assert-hit-threshold.py" \
  --result "$RESULT_JSON" \
  --threshold "$OVERALL"

"$ROOT/.venv/bin/python" "$ROOT/tools/locomo/assert-locomo-thresholds.py" \
  --result "$RESULT_JSON" \
  --overall "$OVERALL" \
  --multi_hop "$MULTI_HOP" \
  --open_domain "$OPEN_DOMAIN"

"$ROOT/.venv/bin/python" "$ROOT/tools/locomo/assert-locomo-drift.py" \
  --result "$RESULT_JSON" \
  --guardrails "$GUARDRAILS"

cargo test --workspace
"$ROOT/.venv/bin/pytest" python/tests -q
corepack pnpm --dir packages/node test
corepack pnpm --dir packages/mcp test

printf '\n[full-gate] PASS\n'
printf '[full-gate] BASE_DIR=%s\n' "$BASE_DIR"
printf '[full-gate] RESULT_JSON=%s\n' "$RESULT_JSON"
