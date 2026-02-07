"""CLI entry point for DST mod updates."""

from __future__ import annotations

import argparse
import sys

from mods_update.core import ModUpdateError, run_update


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Update DST server mods from Steam workshop/game mods directories.")
    parser.add_argument(
        "--steam-path",
        required=True,
        help="Steam installation root path (e.g., C:\\Program Files (x86)\\Steam)",
    )
    return parser


def print_results(results: list[dict]) -> None:
    if not results:
        print("No server mods found to process.")
        return

    counts = {"updated": 0, "unchanged": 0, "missing": 0, "failed": 0}
    for item in results:
        status = item.get("status", "unknown")
        counts[status] = counts.get(status, 0) + 1
        print(f"{item.get('workshop_id')}: {status} - {item.get('reason')}")

    print("\nSummary")
    for key in ["updated", "unchanged", "missing", "failed"]:
        print(f"{key}: {counts.get(key, 0)}")


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()

    try:
        results = run_update(args.steam_path)
    except ModUpdateError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 1

    print_results(results)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
