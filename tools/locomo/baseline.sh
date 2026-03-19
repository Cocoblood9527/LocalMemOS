#!/usr/bin/env bash
set -euo pipefail

# V1 retrieval baseline for LoCoMo using localmemos MemoryClient
# - Keeps all artifacts under /tmp (unless BASE_DIR is provided)
# - Computes evidence hit@K overall + by category

ROOT="$(pwd)"
if [[ ! -f "$ROOT/README.md" || ! -f "$ROOT/Cargo.toml" ]]; then
  echo "Please run from localmemos repo root." >&2
  exit 1
fi
if [[ ! -x "$ROOT/.venv/bin/python" ]]; then
  echo "Missing .venv at $ROOT/.venv. Run setup first." >&2
  exit 1
fi

K="${1:-5}"
if ! [[ "$K" =~ ^[0-9]+$ ]] || [[ "$K" -lt 1 ]]; then
  echo "Usage: $0 [K>=1]" >&2
  exit 1
fi

BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-baseline.XXXXXX)}"
LOCOMO_REPO="$BASE_DIR/locomo"
DB_PATH="$BASE_DIR/locomo_v1_baseline.db"
RESULT_JSON="$BASE_DIR/result_hit_at_${K}.json"

if [[ ! -d "$LOCOMO_REPO/.git" ]]; then
  git clone --depth 1 https://github.com/snap-research/locomo "$LOCOMO_REPO" >/dev/null 2>&1
fi

"$ROOT/.venv/bin/python" - <<'PY' "$LOCOMO_REPO/data/locomo10.json" "$DB_PATH" "$K" "$RESULT_JSON"
import json, re, sys
from collections import defaultdict
from memory_sdk.client import MemoryClient

data_path, db_path, k_raw, result_path = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
K = int(k_raw)

cat_name = {
    1: "multi-hop",
    2: "temporal",
    3: "open-domain",
    4: "single-hop",
    5: "adversarial",
}

# Avoid sqlite fts parser errors from punctuation/operators in raw questions.
def safe_query(q: str) -> str:
    q = re.sub(r"[^\w\s]", " ", q)
    q = re.sub(r"\s+", " ", q).strip()
    return q

samples = json.load(open(data_path))
client = MemoryClient(db_path)

ingested_turns = 0
for sample in samples:
    scope = sample["sample_id"]
    conv = sample["conversation"]
    for key, session in conv.items():
        if not re.fullmatch(r"session_\d+", key):
            continue
        for turn in session:
            dia_id = turn.get("dia_id")
            text = turn.get("text")
            if not dia_id or not text:
                continue
            client.upsert_fact(
                namespace="benchmark",
                scope_id=scope,
                entity="dialog",
                attribute=str(dia_id),
                value=text,
                source_kind="locomo_ingest",
                source_ref="locomo10",
                evidence_summary=turn.get("speaker"),
            )
            ingested_turns += 1

stats = defaultdict(lambda: {"n": 0, "hit": 0})
all_n, all_hit, skipped = 0, 0, 0

for sample in samples:
    scope = sample["sample_id"]
    for qa in sample["qa"]:
        evidence = qa.get("evidence") or []
        if not evidence:
            continue
        q = safe_query(qa["question"])
        if not q:
            skipped += 1
            continue

        cat = qa["category"]
        stats[cat]["n"] += 1
        all_n += 1

        recall = client.recall(
            namespace="benchmark",
            scope_id=scope,
            text_query=q,
        )
        top_attrs = [f["attribute"] for f in recall.get("facts", [])[:K]]
        hit = any(ev in top_attrs for ev in evidence)
        if hit:
            stats[cat]["hit"] += 1
            all_hit += 1

result = {
    "metric": f"evidence_hit@{K}",
    "questions_with_evidence": all_n,
    "skipped_questions": skipped,
    "ingested_turns": ingested_turns,
    "overall": round(all_hit / all_n, 4) if all_n else None,
    "by_category": {
        cat_name[c]: {
            "n": v["n"],
            "score": round(v["hit"] / v["n"], 4) if v["n"] else None,
        }
        for c, v in sorted(stats.items())
    },
    "db_path": db_path,
}

with open(result_path, "w", encoding="utf-8") as f:
    json.dump(result, f, ensure_ascii=False, indent=2)

print(json.dumps(result, ensure_ascii=False, indent=2))
PY

echo ""
echo "Artifacts:" 
echo "  BASE_DIR=$BASE_DIR"
echo "  RESULT_JSON=$RESULT_JSON"
echo ""
echo "Cleanup (manual):"
echo "  rm -rf '$BASE_DIR'"
