# V2-R3 LoCoMo Targeted Re-rank Results (K=5)

## Run Commands

Executed from repository root:

```bash
./.venv/bin/python -m pip install -e python
BASE_DIR=/tmp/localmemos-locomo-r3f-gate tools/locomo/run-regression-gate.sh 5
BASE_DIR=/tmp/localmemos-locomo-r3f-official tools/locomo/official-like-eval.sh 5
BASE_DIR=/tmp/localmemos-locomo-r3f-qa-proxy tools/locomo/qa-proxy.sh 5
BASE_DIR=/tmp/localmemos-locomo-r3f-failures tools/locomo/refresh-failure-samples.sh 5 80
```

Artifacts:

- `/tmp/localmemos-locomo-r3f-gate/result_hit_at_5.json`
- `/tmp/localmemos-locomo-r3f-official/locomo10_localmemos_v1_top5_scores.json`
- `/tmp/localmemos-locomo-r3f-qa-proxy/qa_proxy_hit_and_f1_at_5.json`
- `tools/locomo/failure-samples-k5.json`

## Summary

- Regression gate default threshold is now `0.55`.
- Gate result: pass (`hit@5 = 0.5767`).
- V2-R3 keeps a large gain over original R0 baseline (`0.001 -> 0.5767`) but is below V2-R2 peak (`0.5822`).

## Metric Comparison

| Metric | R0 | R2 | R3 |
| --- | ---: | ---: | ---: |
| evidence hit@5 | 0.0010 | 0.5822 | 0.5767 |
| naive overall accuracy | 0.2329 | 0.2477 | 0.2472 |
| oracle overall accuracy | 0.2338 | 0.6722 | 0.6703 |

## Retrieval hit@5 by Category

| Category | R2 | R3 |
| --- | ---: | ---: |
| multi-hop | 0.4149 | 0.4078 |
| temporal | 0.6480 | 0.6480 |
| open-domain | 0.3152 | 0.3261 |
| single-hop | 0.6302 | 0.6266 |
| adversarial | 0.6054 | 0.5897 |

## Failure Sample Snapshot

`tools/locomo/failure-samples-k5.json` was refreshed with:

- `total_misses = 839`
- `selected_misses = 80`
- prioritized ordering: `multi-hop`, `open-domain`, then others
