#!/usr/bin/env bash
set -euo pipefail

# V1 QA proxy baseline for LoCoMo
# Outputs:
# - naive_e2e_f1: pure retrieval+heuristic answer baseline (no label leakage)
# - retrieval_oracle_f1: retrieval-ceiling estimate using gold answer iff evidence is retrieved

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

BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-qa-proxy.XXXXXX)}"
LOCOMO_REPO="$BASE_DIR/locomo"
DB_PATH="$BASE_DIR/locomo_v1_qa_proxy.db"
RESULT_JSON="$BASE_DIR/qa_proxy_hit_and_f1_at_${K}.json"

if [[ ! -d "$LOCOMO_REPO/.git" ]]; then
  git clone --depth 1 https://github.com/snap-research/locomo "$LOCOMO_REPO" >/dev/null 2>&1
fi

"$ROOT/.venv/bin/python" - <<'PY' "$LOCOMO_REPO/data/locomo10.json" "$DB_PATH" "$K" "$RESULT_JSON"
import json
import re
import string
import sys
from collections import Counter, defaultdict

from memory_sdk.client import MemoryClient


def normalize_answer(s: str) -> str:
    s = s.replace(',', '')
    s = s.lower()
    s = ''.join(ch for ch in s if ch not in set(string.punctuation))
    s = re.sub(r'\b(a|an|the|and)\b', ' ', s)
    s = ' '.join(s.split())
    return s


def f1_score(prediction: str, ground_truth: str) -> float:
    pred_tokens = normalize_answer(prediction).split()
    gt_tokens = normalize_answer(ground_truth).split()
    if not pred_tokens or not gt_tokens:
        return 0.0
    common = Counter(pred_tokens) & Counter(gt_tokens)
    num_same = sum(common.values())
    if num_same == 0:
        return 0.0
    precision = num_same / len(pred_tokens)
    recall = num_same / len(gt_tokens)
    return (2 * precision * recall) / (precision + recall)


def f1_multi(prediction: str, ground_truth: str) -> float:
    preds = [p.strip() for p in prediction.split(',') if p.strip()]
    gts = [g.strip() for g in ground_truth.split(',') if g.strip()]
    if not preds or not gts:
        return 0.0
    return sum(max(f1_score(p, gt) for p in preds) for gt in gts) / len(gts)


def score_by_category(category: int, pred: str, answer: str) -> float:
    if category in (2, 3, 4):
        if category == 3:
            answer = answer.split(';')[0].strip()
        return f1_score(pred, answer)
    if category == 1:
        return f1_multi(pred, answer)
    if category == 5:
        low = pred.lower()
        if ('no information available' in low) or ('not mentioned' in low):
            return 1.0
        return 0.0
    return 0.0


def safe_query(q: str) -> str:
    q = re.sub(r"[^\w\s]", " ", q)
    q = re.sub(r"\s+", " ", q).strip()
    return q


def naive_answer(category: int, top_texts: list[str]) -> str:
    if category == 5:
        return 'Not mentioned in the conversation'
    if not top_texts:
        return 'Not mentioned in the conversation'
    txt = top_texts[0].strip()
    if not txt:
        return 'Not mentioned in the conversation'
    return txt[:200]


DATA_PATH, DB_PATH, K_RAW, RESULT_PATH = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
K = int(K_RAW)

cat_name = {
    1: 'multi-hop',
    2: 'temporal',
    3: 'open-domain',
    4: 'single-hop',
    5: 'adversarial',
}

samples = json.load(open(DATA_PATH))
client = MemoryClient(DB_PATH)

ingested_turns = 0
for sample in samples:
    scope = sample['sample_id']
    conv = sample['conversation']
    for key, session in conv.items():
        if not re.fullmatch(r'session_\d+', key):
            continue
        for turn in session:
            dia_id = turn.get('dia_id')
            text = turn.get('text')
            if not dia_id or not text:
                continue
            client.upsert_fact(
                namespace='benchmark',
                scope_id=scope,
                entity='dialog',
                attribute=str(dia_id),
                value=text,
                source_kind='locomo_ingest',
                source_ref='locomo10',
                evidence_summary=turn.get('speaker'),
            )
            ingested_turns += 1

naive_total = 0.0
oracle_total = 0.0
q_count = 0

naive_by_cat = defaultdict(lambda: {'n': 0, 'sum': 0.0})
oracle_by_cat = defaultdict(lambda: {'n': 0, 'sum': 0.0})

hit_by_cat = defaultdict(lambda: {'n': 0, 'hit': 0})
hit_n = 0
hit_total = 0

for sample in samples:
    scope = sample['sample_id']
    for qa in sample['qa']:
        category = int(qa['category'])
        answer = str(qa.get('answer', qa.get('adversarial_answer', '')))
        evidence = qa.get('evidence') or []

        q = safe_query(qa['question'])
        recall = {'facts': []}
        if q:
            recall = client.recall(namespace='benchmark', scope_id=scope, text_query=q)

        facts = recall.get('facts', [])[:K]
        top_attrs = [f.get('attribute', '') for f in facts]
        top_texts = [f.get('value_text', '') for f in facts]

        naive_pred = naive_answer(category, top_texts)
        naive_score = score_by_category(category, naive_pred, answer)

        if category == 5:
            oracle_pred = 'Not mentioned in the conversation'
        else:
            has_ev = bool(evidence) and any(ev in top_attrs for ev in evidence)
            oracle_pred = answer if has_ev else 'Not mentioned in the conversation'
        oracle_score = score_by_category(category, oracle_pred, answer)

        q_count += 1
        naive_total += naive_score
        oracle_total += oracle_score

        naive_by_cat[category]['n'] += 1
        naive_by_cat[category]['sum'] += naive_score
        oracle_by_cat[category]['n'] += 1
        oracle_by_cat[category]['sum'] += oracle_score

        if evidence:
            hit_by_cat[category]['n'] += 1
            hit_n += 1
            has_ev = any(ev in top_attrs for ev in evidence)
            if has_ev:
                hit_by_cat[category]['hit'] += 1
                hit_total += 1

result = {
    'metric_family': {
        'retrieval': f'evidence_hit@{K}',
        'qa_f1_naive': 'locomo_like_f1',
        'qa_f1_oracle': 'locomo_like_f1 (retrieval-conditioned upper bound)',
    },
    'counts': {
        'questions_total': q_count,
        'questions_with_evidence': hit_n,
        'ingested_turns': ingested_turns,
    },
    'overall': {
        f'hit@{K}': round(hit_total / hit_n, 4) if hit_n else None,
        'naive_e2e_f1': round(naive_total / q_count, 4) if q_count else None,
        'retrieval_oracle_f1': round(oracle_total / q_count, 4) if q_count else None,
    },
    'by_category': {},
    'db_path': DB_PATH,
}

for c in sorted(cat_name):
    hn = hit_by_cat[c]['n']
    h = hit_by_cat[c]['hit']
    nn = naive_by_cat[c]['n']
    ns = naive_by_cat[c]['sum']
    on = oracle_by_cat[c]['n']
    os = oracle_by_cat[c]['sum']

    result['by_category'][cat_name[c]] = {
        'questions': nn,
        f'hit@{K}': round(h / hn, 4) if hn else None,
        'naive_e2e_f1': round(ns / nn, 4) if nn else None,
        'retrieval_oracle_f1': round(os / on, 4) if on else None,
    }

with open(RESULT_PATH, 'w', encoding='utf-8') as f:
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
