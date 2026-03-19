#!/usr/bin/env python3
import argparse
import json
import re
import subprocess
import tempfile
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Optional

from memory_sdk.client import MemoryClient

CATEGORY_NAME = {
    1: "multi-hop",
    2: "temporal",
    3: "open-domain",
    4: "single-hop",
    5: "adversarial",
}

DEFAULT_CANDIDATES = ["games", "kind", "names"]


def safe_query(text: str) -> str:
    text = re.sub(r"[^\w\s]", " ", text)
    return re.sub(r"\s+", " ", text).strip()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Evaluate fixed single-knob candidate stopwords and emit APPLY/NO-OP decision."
    )
    parser.add_argument("--k", type=int, default=5, help="Top-K retrieval cutoff")
    parser.add_argument(
        "--candidates",
        default=",".join(DEFAULT_CANDIDATES),
        help="Comma-separated candidate tokens",
    )
    parser.add_argument(
        "--base-dir",
        default=None,
        help="Optional working directory for dataset/db artifacts (default: mktemp in /tmp)",
    )
    parser.add_argument(
        "--output",
        default=None,
        help="Optional output JSON path",
    )
    return parser.parse_args()


def evaluate(
    samples: List[dict],
    client: MemoryClient,
    k: int,
    drop_token: Optional[str],
) -> Dict[str, float]:
    stats = defaultdict(lambda: {"n": 0, "hit": 0})
    all_n = 0
    all_hit = 0

    for sample in samples:
        scope = sample["sample_id"]
        for qa in sample["qa"]:
            evidence = qa.get("evidence") or []
            if not evidence:
                continue

            query = safe_query(str(qa.get("question", "")))
            if drop_token:
                query = " ".join(t for t in query.split() if t.lower() != drop_token)
            if not query:
                continue

            cat = int(qa["category"])
            stats[cat]["n"] += 1
            all_n += 1

            recall = client.recall(namespace="benchmark", scope_id=scope, text_query=query)
            top_attrs = [f.get("attribute", "") for f in recall.get("facts", [])[:k]]
            if any(ev in top_attrs for ev in evidence):
                stats[cat]["hit"] += 1
                all_hit += 1

    result: Dict[str, float] = {
        "overall": round(all_hit / all_n, 4) if all_n else 0.0,
    }
    for c, v in sorted(stats.items()):
        key = CATEGORY_NAME[c]
        result[key] = round(v["hit"] / v["n"], 4) if v["n"] else 0.0
    return result


def main() -> int:
    args = parse_args()

    candidates = [c.strip().lower() for c in args.candidates.split(",") if c.strip()]
    if not candidates:
        raise SystemExit("No candidate token provided")

    base_dir = Path(args.base_dir) if args.base_dir else Path(tempfile.mkdtemp(prefix="localmemos-single-knob."))
    base_dir.mkdir(parents=True, exist_ok=True)

    locomo_repo = base_dir / "locomo"
    if not (locomo_repo / ".git").exists():
        subprocess.run(
            ["git", "clone", "--depth", "1", "https://github.com/snap-research/locomo", str(locomo_repo)],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )

    data_path = locomo_repo / "data" / "locomo10.json"
    samples = json.loads(data_path.read_text(encoding="utf-8"))

    db_path = base_dir / "single_knob_eval.db"
    client = MemoryClient(str(db_path))

    ingested_turns = 0
    for sample in samples:
        scope = sample["sample_id"]
        for key, session in sample["conversation"].items():
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

    baseline = evaluate(samples, client, args.k, None)

    candidate_results = {}
    passing = []

    for token in candidates:
        metrics = evaluate(samples, client, args.k, token)
        delta_overall = round(metrics.get("overall", 0.0) - baseline.get("overall", 0.0), 4)
        delta_multi = round(metrics.get("multi-hop", 0.0) - baseline.get("multi-hop", 0.0), 4)

        candidate_results[token] = {
            "metrics": metrics,
            "delta": {
                "overall": delta_overall,
                "multi-hop": delta_multi,
            },
            "passes_gate": delta_overall > 0 and delta_multi > 0,
        }

        if delta_overall > 0 and delta_multi > 0:
            passing.append((token, delta_overall, delta_multi, metrics))

    passing.sort(key=lambda x: (x[1], x[2]), reverse=True)

    if passing:
        selected_token = passing[0][0]
        decision = "APPLY"
    else:
        selected_token = None
        decision = "NO-OP"

    result = {
        "meta": {
            "k": args.k,
            "candidates": candidates,
            "ingested_turns": ingested_turns,
            "base_dir": str(base_dir),
            "db_path": str(db_path),
            "source_data": str(data_path),
        },
        "baseline": baseline,
        "candidates": candidate_results,
        "decision": {
            "action": decision,
            "selected_candidate": selected_token,
            "rule": "APPLY only if overall and multi-hop both strictly improve; otherwise NO-OP",
        },
    }

    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")

    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
