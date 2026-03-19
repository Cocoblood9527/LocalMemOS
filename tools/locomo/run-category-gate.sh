#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
K="${1:-5}"
OVERALL="${2:-0.55}"
MULTI_HOP="${3:-0.42}"
OPEN_DOMAIN="${4:-0.32}"
BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-category-gate.XXXXXX)}"
RESULT_JSON="$BASE_DIR/result_hit_at_${K}.json"

cd "$ROOT"
BASE_DIR="$BASE_DIR" "$ROOT/tools/locomo/baseline.sh" "$K"

"$ROOT/.venv/bin/python" "$ROOT/tools/locomo/assert-locomo-thresholds.py" \
  --result "$RESULT_JSON" \
  --overall "$OVERALL" \
  --multi_hop "$MULTI_HOP" \
  --open_domain "$OPEN_DOMAIN"
