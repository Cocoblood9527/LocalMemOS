# V2-R2 LoCoMo Retrieval Comparison (K=5)

## Purpose

Track retrieval progress after shipping repository-owned LoCoMo tooling and regression gate.

## Command Set

Executed from repository root:

```bash
./.venv/bin/python -m pip install -e python
BASE_DIR=/tmp/localmemos-locomo-r2-gate tools/locomo/run-regression-gate.sh 5 0.05
BASE_DIR=/tmp/localmemos-locomo-r2-official tools/locomo/official-like-eval.sh 5
BASE_DIR=/tmp/localmemos-locomo-r2-qa-proxy tools/locomo/qa-proxy.sh 5
```

Artifacts:

- `/tmp/localmemos-locomo-r2-gate/result_hit_at_5.json`
- `/tmp/localmemos-locomo-r2-official/locomo10_localmemos_v1_top5_scores.json`
- `/tmp/localmemos-locomo-r2-qa-proxy/qa_proxy_hit_and_f1_at_5.json`

## Overall Comparison

| Metric | R0 (known baseline) | R1 (V2-R1) | R2 (this round) |
| --- | ---: | ---: | ---: |
| evidence hit@5 | 0.0010 | 0.5822 | 0.5822 |
| naive overall accuracy | 0.2329 | 0.2477 | 0.2477 |
| oracle overall accuracy | 0.2338 | 0.6722 | 0.6722 |

## Retrieval hit@5 by Category

| Category | R1 | R2 |
| --- | ---: | ---: |
| multi-hop | 0.4149 | 0.4149 |
| temporal | 0.6480 | 0.6480 |
| open-domain | 0.3152 | 0.3152 |
| single-hop | 0.6302 | 0.6302 |
| adversarial | 0.6054 | 0.6054 |

## Gate Result

Regression gate command:

```bash
BASE_DIR=/tmp/localmemos-locomo-r2-gate tools/locomo/run-regression-gate.sh 5 0.05
```

Observed assertion payload:

```json
{"ok": true, "threshold": 0.05, "overall": 0.5822}
```

## Notes

- This round focuses on reproducibility and regression protection.
- Retrieval core behavior is unchanged from V2-R1 in this round.
