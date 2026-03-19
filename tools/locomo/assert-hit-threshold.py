#!/usr/bin/env python3
import argparse
import json
import sys


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Assert LoCoMo evidence hit@K result meets threshold."
    )
    parser.add_argument("--result", required=True, help="Path to result JSON file")
    parser.add_argument(
        "--threshold", type=float, required=True, help="Minimum acceptable hit@K value"
    )
    args = parser.parse_args()

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
    if overall is None:
        print(
            json.dumps(
                {"ok": False, "reason": "missing_overall_metric", "result": args.result},
                ensure_ascii=False,
            )
        )
        return 3

    passed = overall >= args.threshold
    print(
        json.dumps(
            {
                "ok": passed,
                "threshold": args.threshold,
                "overall": overall,
                "metric": payload.get("metric"),
                "questions_with_evidence": payload.get("questions_with_evidence"),
                "result": args.result,
            },
            ensure_ascii=False,
        )
    )
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
