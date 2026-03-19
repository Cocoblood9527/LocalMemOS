#!/usr/bin/env bash
set -euo pipefail

# LocalMemOS V1 -> LoCoMo official-like QA evaluation

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

BASE_DIR="${BASE_DIR:-$(mktemp -d /tmp/localmemos-locomo-official-like.XXXXXX)}"
LOCOMO_REPO="$BASE_DIR/locomo"
DB_PATH="$BASE_DIR/locomo_v1_eval.db"
PRED_JSON="$BASE_DIR/locomo10_localmemos_v1_top${K}_predictions.json"
SCORE_JSON="$BASE_DIR/locomo10_localmemos_v1_top${K}_scores.json"

if [[ ! -d "$LOCOMO_REPO/.git" ]]; then
  git clone --depth 1 https://github.com/snap-research/locomo "$LOCOMO_REPO" >/dev/null 2>&1
fi

"$ROOT/.venv/bin/python" - <<'PY' "$LOCOMO_REPO/data/locomo10.json" "$DB_PATH" "$K" "$PRED_JSON" "$SCORE_JSON"
import json
import re
import string
import sys
from collections import Counter, defaultdict
from memory_sdk.client import MemoryClient

def safe_query(q: str) -> str:
    q = re.sub(r"[^\w\s]", " ", q)
    q = re.sub(r"\s+", " ", q).strip()
    return q

def normalize_answer(s: str) -> str:
    s = s.replace(',', '')
    s = s.lower()
    exclude = set(string.punctuation)
    s = ''.join(ch for ch in s if ch not in exclude)
    s = re.sub(r'\b(a|an|the|and)\b', ' ', s)
    s = ' '.join(s.split())
    return s

def f1_score(prediction: str, ground_truth: str) -> float:
    p = normalize_answer(prediction).split()
    g = normalize_answer(ground_truth).split()
    if not p or not g:
        return 0.0
    common = Counter(p) & Counter(g)
    num_same = sum(common.values())
    if num_same == 0:
        return 0.0
    precision = num_same / len(p)
    recall = num_same / len(g)
    return (2 * precision * recall) / (precision + recall)

def f1_multi(prediction: str, ground_truth: str) -> float:
    preds = [x.strip() for x in prediction.split(',') if x.strip()]
    gts = [x.strip() for x in ground_truth.split(',') if x.strip()]
    if not preds or not gts:
        return 0.0
    return sum(max(f1_score(p, gt) for p in preds) for gt in gts) / len(gts)

def locomo_like_score(category: int, prediction: str, answer: str) -> float:
    if category in (2, 3, 4):
        if category == 3:
            answer = answer.split(';')[0].strip()
        return f1_score(prediction, answer)
    if category == 1:
        return f1_multi(prediction, answer)
    if category == 5:
        low = prediction.lower()
        return 1.0 if ('no information available' in low or 'not mentioned' in low) else 0.0
    return 0.0

def first_sentence(text: str) -> str:
    text = text.strip()
    if not text:
        return ''
    m = re.split(r'(?<=[.!?])\s+', text, maxsplit=1)
    return m[0].strip()

def extract_date_like(text: str) -> str:
    patterns = [
        r'\b(?:jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|sept|september|oct|october|nov|november|dec|december)\s+\d{1,2}(?:,\s*\d{4})?\b',
        r'\b\d{1,2}/\d{1,2}/\d{2,4}\b',
        r'\b\d{4}-\d{1,2}-\d{1,2}\b',
        r'\b(19|20)\d{2}\b',
    ]
    low = text.lower()
    for p in patterns:
        m = re.search(p, low)
        if m:
            return m.group(0)
    return ''

def naive_answer(category: int, question: str, top_texts: list[str]) -> str:
    if category == 5:
        return 'Not mentioned in the conversation'
    if not top_texts:
        return 'Not mentioned in the conversation'
    t1 = top_texts[0].strip()
    t2 = top_texts[1].strip() if len(top_texts) > 1 else ''
    joined = (t1 + ' ' + t2).strip()
    if category == 2:
        d = extract_date_like(joined)
        if d:
            return d
        return first_sentence(t1)[:120] or 'Not mentioned in the conversation'
    if category == 1:
        out = []
        if t1:
            out.append(first_sentence(t1)[:140])
        if t2:
            out.append(first_sentence(t2)[:140])
        if out:
            return ', '.join(out)
        return 'Not mentioned in the conversation'
    return first_sentence(t1)[:180] or 'Not mentioned in the conversation'

def answer_text(qa: dict) -> str:
    if 'answer' in qa and qa['answer'] is not None:
        return str(qa['answer'])
    if 'adversarial_answer' in qa and qa['adversarial_answer'] is not None:
        return str(qa['adversarial_answer'])
    return ''

def aggregate(qas: list[dict], metric_key: str) -> dict:
    counts = defaultdict(int)
    sums = defaultdict(float)
    for qa in qas:
        c = int(qa['category'])
        counts[c] += 1
        sums[c] += float(qa.get(metric_key, 0.0))
    ordered = [4, 1, 2, 3, 5]
    by_category = {}
    total_n = 0
    total_sum = 0.0
    for c in ordered:
        n = counts[c]
        s = sums[c]
        by_category[str(c)] = {
            'count': n,
            'sum_score': round(s, 4),
            'accuracy': round((s / n), 4) if n else None,
        }
        total_n += n
        total_sum += s
    return {
        'category_counts': {str(k): v for k, v in counts.items()},
        'cum_accuracy_by_category': {str(k): round(v, 4) for k, v in sums.items()},
        'by_category_ordered': by_category,
        'overall_accuracy': round(total_sum / total_n, 4) if total_n else None,
    }

DATA_PATH, DB_PATH, K_RAW, PRED_PATH, SCORE_PATH = sys.argv[1:6]
K = int(K_RAW)
model_naive = f'localmemos_v1_top{K}_naive'
model_oracle = f'localmemos_v1_top{K}_oracle'
pred_key_naive = model_naive + '_prediction'
pred_key_oracle = model_oracle + '_prediction'
ctx_key_naive = model_naive + '_context'
ctx_key_oracle = model_oracle + '_context'
metric_key_naive = model_naive + '_f1'
metric_key_oracle = model_oracle + '_f1'

samples = json.load(open(DATA_PATH))
client = MemoryClient(DB_PATH)
ingested_turns = 0
for sample in samples:
    scope = sample['sample_id']
    for key, session in sample['conversation'].items():
        if not re.fullmatch(r'session_\d+', key):
            continue
        for turn in session:
            dia_id = turn.get('dia_id')
            text = turn.get('text')
            if not dia_id or not text:
                continue
            client.upsert_fact(namespace='benchmark', scope_id=scope, entity='dialog', attribute=str(dia_id), value=text, source_kind='locomo_ingest', source_ref='locomo10', evidence_summary=turn.get('speaker'))
            ingested_turns += 1

out_samples = []
all_qas = []
hit_n = 0
hit_total = 0
for sample in samples:
    scope = sample['sample_id']
    out = {'sample_id': scope, 'qa': []}
    for qa in sample['qa']:
        category = int(qa['category'])
        gold = answer_text(qa)
        q = safe_query(str(qa.get('question', '')))
        rec = {'facts': []}
        if q:
            rec = client.recall(namespace='benchmark', scope_id=scope, text_query=q)
        facts = rec.get('facts', [])[:K]
        top_attrs = [f.get('attribute', '') for f in facts]
        top_texts = [f.get('value_text', '') for f in facts]
        evidence = qa.get('evidence') or []
        has_evidence_hit = bool(evidence) and any(ev in top_attrs for ev in evidence)
        if evidence:
            hit_n += 1
            if has_evidence_hit:
                hit_total += 1
        pred_naive = naive_answer(category, qa.get('question', ''), top_texts)
        if category == 5:
            pred_oracle = 'Not mentioned in the conversation'
        else:
            pred_oracle = gold if has_evidence_hit else 'Not mentioned in the conversation'
        qa_out = dict(qa)
        qa_out[pred_key_naive] = pred_naive
        qa_out[ctx_key_naive] = top_attrs
        qa_out[metric_key_naive] = round(locomo_like_score(category, pred_naive, gold), 4)
        qa_out[pred_key_oracle] = pred_oracle
        qa_out[ctx_key_oracle] = top_attrs
        qa_out[metric_key_oracle] = round(locomo_like_score(category, pred_oracle, gold), 4)
        out['qa'].append(qa_out)
        all_qas.append(qa_out)
    out_samples.append(out)

score = {
    model_naive: aggregate(all_qas, metric_key_naive),
    model_oracle: aggregate(all_qas, metric_key_oracle),
    'retrieval': {f'hit@{K}': round(hit_total / hit_n, 4) if hit_n else None, 'questions_with_evidence': hit_n},
    'meta': {'ingested_turns': ingested_turns, 'questions_total': len(all_qas), 'dataset': DATA_PATH},
}

with open(PRED_PATH, 'w', encoding='utf-8') as f:
    json.dump(out_samples, f, ensure_ascii=False, indent=2)
with open(SCORE_PATH, 'w', encoding='utf-8') as f:
    json.dump(score, f, ensure_ascii=False, indent=2)

print(json.dumps({'prediction_json': PRED_PATH, 'score_json': SCORE_PATH, 'retrieval_hit_at_k': score['retrieval'], 'naive_overall_accuracy': score[model_naive]['overall_accuracy'], 'oracle_overall_accuracy': score[model_oracle]['overall_accuracy']}, ensure_ascii=False, indent=2))
PY

echo ""
echo "Artifacts:"
echo "  BASE_DIR=$BASE_DIR"
echo "  PRED_JSON=$PRED_JSON"
echo "  SCORE_JSON=$SCORE_JSON"
echo ""
echo "Cleanup (manual):"
echo "  rm -rf '$BASE_DIR'"
