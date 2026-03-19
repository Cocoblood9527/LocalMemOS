# LoCoMo Evaluation Tools

Repository-owned LoCoMo scripts for retrieval-focused V2 validation.

## Scripts

- `baseline.sh`: computes `evidence hit@K`
- `qa-proxy.sh`: computes retrieval and QA proxy scores
- `official-like-eval.sh`: computes LoCoMo official-like outputs
- `assert-hit-threshold.py`: validates `baseline.sh` result against threshold
- `assert-locomo-thresholds.py`: validates overall + category thresholds
- `run-regression-gate.sh`: one-shot baseline + threshold gate (default threshold `0.55`)
- `run-category-gate.sh`: baseline + category-aware gate (defaults: overall `0.55`, multi-hop `0.42`, open-domain `0.32`)
- `export-failure-samples.py`: exports prioritized retrieval misses
- `refresh-failure-samples.sh`: refreshes `failure-samples-k5.json` from current retriever

## Usage

Run from repository root:

```bash
tools/locomo/baseline.sh 5
tools/locomo/qa-proxy.sh 5
tools/locomo/official-like-eval.sh 5
tools/locomo/run-regression-gate.sh 5
tools/locomo/run-category-gate.sh 5
tools/locomo/refresh-failure-samples.sh 5 80
```

All scripts write artifacts to `/tmp` by default (or `BASE_DIR` if provided).

Failure samples output path defaults to:

- `tools/locomo/failure-samples-k5.json`

## Notes

- Requires `.venv` and local editable `memory-sdk` install.
- If `hit@5` unexpectedly stays near old baseline, rebuild Python native binding:

```bash
./.venv/bin/python -m pip install -e python
```
