#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
K="${1:-5}"
THRESHOLD="${2:-0.05}"
BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-regression-gate.XXXXXX)}"
RESULT_JSON="$BASE_DIR/result_hit_at_${K}.json"

cd "$ROOT"
BASE_DIR="$BASE_DIR" "$ROOT/tools/locomo/baseline.sh" "$K"

"$ROOT/.venv/bin/python" "$ROOT/tools/locomo/assert-hit-threshold.py" \
  --result "$RESULT_JSON" \
  --threshold "$THRESHOLD"
