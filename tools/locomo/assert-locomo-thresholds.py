#!/usr/bin/env python3
import argparse
import json
import sys


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Assert LoCoMo retrieval thresholds for overall and categories."
    )
    parser.add_argument("--result", required=True, help="Path to baseline result JSON")
    parser.add_argument("--overall", type=float, default=0.55, help="Overall hit@K threshold")
    parser.add_argument(
        "--multi_hop",
        type=float,
        default=0.42,
        help="Multi-hop category threshold",
    )
    parser.add_argument(
        "--open_domain",
        type=float,
        default=0.32,
        help="Open-domain category threshold",
    )
    return parser.parse_args()


def category_score(payload: dict, category_name: str) -> float | None:
    by_category = payload.get("by_category") or {}
    row = by_category.get(category_name) or {}
    return row.get("score")


def main() -> int:
    args = parse_args()
    try:
        with open(args.result, "r", encoding="utf-8") as fh:
            payload = json.load(fh)
    except FileNotFoundError:
        print(
            json.dumps(
                {
                    "ok": False,
                    "reason": "result_file_not_found",
                    "result": args.result,
                },
                ensure_ascii=False,
            )
        )
        return 2

    overall = payload.get("overall")
    multi_hop = category_score(payload, "multi-hop")
    open_domain = category_score(payload, "open-domain")

    checks = {
        "overall": {"actual": overall, "threshold": args.overall},
        "multi-hop": {"actual": multi_hop, "threshold": args.multi_hop},
        "open-domain": {"actual": open_domain, "threshold": args.open_domain},
    }

    ok = True
    for item in checks.values():
        if item["actual"] is None or item["actual"] < item["threshold"]:
            ok = False

    print(json.dumps({"ok": ok, "checks": checks, "result": args.result}, ensure_ascii=False))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
