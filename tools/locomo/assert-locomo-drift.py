#!/usr/bin/env python3
import argparse
import json
import sys


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Assert LoCoMo retrieval result does not drift below configured budget."
    )
    parser.add_argument("--result", required=True, help="Path to baseline result JSON")
    parser.add_argument(
        "--guardrails",
        default="tools/locomo/baseline-guardrails-k5.json",
        help="Path to baseline guardrails JSON",
    )
    return parser.parse_args()


def category_score(payload: dict, category_name: str) -> float | None:
    by_category = payload.get("by_category") or {}
    row = by_category.get(category_name) or {}
    return row.get("score")


def read_json(path: str) -> dict:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def main() -> int:
    args = parse_args()
    try:
        result_payload = read_json(args.result)
    except FileNotFoundError:
        print(
            json.dumps(
                {"ok": False, "reason": "result_file_not_found", "result": args.result},
                ensure_ascii=False,
            )
        )
        return 2

    try:
        guardrails = read_json(args.guardrails)
    except FileNotFoundError:
        print(
            json.dumps(
                {
                    "ok": False,
                    "reason": "guardrails_file_not_found",
                    "guardrails": args.guardrails,
                },
                ensure_ascii=False,
            )
        )
        return 2

    baseline = guardrails.get("baseline") or {}
    max_drop = guardrails.get("max_drop") or {}

    actuals = {
        "overall": result_payload.get("overall"),
        "multi-hop": category_score(result_payload, "multi-hop"),
        "open-domain": category_score(result_payload, "open-domain"),
    }

    checks: dict[str, dict] = {}
    ok = True
    for key, actual in actuals.items():
        expected = baseline.get(key)
        budget = max_drop.get(key)
        if expected is None or budget is None or actual is None:
            checks[key] = {
                "actual": actual,
                "expected": expected,
                "max_drop": budget,
                "allowed_floor": None,
                "within_budget": False,
            }
            ok = False
            continue

        allowed_floor = expected - budget
        within_budget = actual >= allowed_floor
        checks[key] = {
            "actual": actual,
            "expected": expected,
            "max_drop": budget,
            "allowed_floor": round(allowed_floor, 6),
            "within_budget": within_budget,
        }
        if not within_budget:
            ok = False

    print(
        json.dumps(
            {
                "ok": ok,
                "result": args.result,
                "guardrails": args.guardrails,
                "checks": checks,
            },
            ensure_ascii=False,
        )
    )
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
