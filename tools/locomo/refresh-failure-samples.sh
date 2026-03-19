#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
K="${1:-5}"
LIMIT="${2:-80}"
BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-failure-samples.XXXXXX)}"
LOCOMO_REPO="$BASE_DIR/locomo"
DB_PATH="$BASE_DIR/locomo_failure_samples.db"
OUT_PATH="${3:-$ROOT/tools/locomo/failure-samples-k5.json}"

if [[ ! -x "$ROOT/.venv/bin/python" ]]; then
  echo "Missing .venv at $ROOT/.venv. Run setup first." >&2
  exit 1
fi

if [[ ! -d "$LOCOMO_REPO/.git" ]]; then
  git clone --depth 1 https://github.com/snap-research/locomo "$LOCOMO_REPO" >/dev/null 2>&1
fi

cd "$ROOT"
"$ROOT/.venv/bin/python" "$ROOT/tools/locomo/export-failure-samples.py" \
  --data "$LOCOMO_REPO/data/locomo10.json" \
  --db "$DB_PATH" \
  --k "$K" \
  --limit "$LIMIT" \
  --output "$OUT_PATH"

echo ""
echo "Artifacts:"
echo "  BASE_DIR=$BASE_DIR"
echo "  DB_PATH=$DB_PATH"
echo "  OUT_PATH=$OUT_PATH"
