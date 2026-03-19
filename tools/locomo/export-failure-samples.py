#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path

from memory_sdk.client import MemoryClient


CATEGORY_NAME = {
    1: "multi-hop",
    2: "temporal",
    3: "open-domain",
    4: "single-hop",
    5: "adversarial",
}

CATEGORY_PRIORITY = {
    1: 0,  # multi-hop first
    3: 1,  # then open-domain
    2: 2,
    4: 3,
    5: 4,
}


def safe_query(q: str) -> str:
    q = re.sub(r"[^\w\s]", " ", q)
    q = re.sub(r"\s+", " ", q).strip()
    return q


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Export top LoCoMo retrieval misses for regression triage."
    )
    parser.add_argument("--data", required=True, help="Path to locomo10.json")
    parser.add_argument("--db", required=True, help="SQLite db path for ingestion/query")
    parser.add_argument("--k", type=int, default=5, help="Top-K for evidence hit check")
    parser.add_argument(
        "--limit",
        type=int,
        default=80,
        help="Max number of misses to export after priority ordering",
    )
    parser.add_argument("--output", required=True, help="Output JSON path")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    data_path = Path(args.data)
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with data_path.open("r", encoding="utf-8") as fh:
        samples = json.load(fh)

    client = MemoryClient(args.db)
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

    misses = []
    for sample in samples:
        scope = sample["sample_id"]
        for qa in sample["qa"]:
            evidence = qa.get("evidence") or []
            if not evidence:
                continue
            category = int(qa.get("category", 4))
            query = safe_query(str(qa.get("question", "")))
            if not query:
                continue

            recall = client.recall(namespace="benchmark", scope_id=scope, text_query=query)
            top_facts = recall.get("facts", [])[: args.k]
            top_attrs = [fact.get("attribute", "") for fact in top_facts]
            hit = any(ev in top_attrs for ev in evidence)
            if hit:
                continue

            misses.append(
                {
                    "sample_id": scope,
                    "category": category,
                    "category_name": CATEGORY_NAME.get(category, "unknown"),
                    "question": qa.get("question", ""),
                    "evidence": evidence,
                    "retrieved_top_attrs": top_attrs,
                    "retrieved_top_texts": [fact.get("value_text", "") for fact in top_facts],
                }
            )

    misses.sort(
        key=lambda item: (
            CATEGORY_PRIORITY.get(item["category"], 99),
            item["sample_id"],
            item["question"],
        )
    )
    selected = misses[: args.limit]

    payload = {
        "meta": {
            "k": args.k,
            "limit": args.limit,
            "ingested_turns": ingested_turns,
            "total_misses": len(misses),
            "selected_misses": len(selected),
            "source_data": str(data_path),
        },
        "samples": selected,
    }
    with output_path.open("w", encoding="utf-8") as fh:
        json.dump(payload, fh, ensure_ascii=False, indent=2)
    print(json.dumps(payload["meta"], ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
