#!/usr/bin/env python3
"""
Spec Consistency Pipeline v2 — Batch runner over VeruSage files.

Each task gets its own folder under verusage/workspace_v2/<task_name>/
containing generated test files and a summary.md.

Usage:
  python3 run_pipeline_v2.py --baseline-only [--concurrency N] [--limit N] [--offset N]
  python3 run_pipeline_v2.py --file PATH

The --baseline-only mode checks which files pass Verus verification and saves
the list. The full pipeline (generator + entailment + critic) is driven by
Lem via subagents — this script handles file discovery and Verus invocations.
"""

import argparse
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

BASE = Path.home() / "intent_formalization"
VERUS = BASE / "verus" / "verus"
VERUSAGE = BASE / "verusage" / "source-projects"
WORKSPACE = BASE / "verusage" / "workspace_v2"


def find_verusage_files(limit=None, offset=0):
    files = sorted(VERUSAGE.rglob("verified/**/*.rs"))
    files = files[offset:]
    if limit:
        files = files[:limit]
    return files


def task_name_for(source_path: Path) -> str:
    """Derive a task folder name from the source path."""
    rel = source_path.relative_to(VERUSAGE)
    # e.g. ironkv/verified/marshal_v/marshal_v__impl0__foo.rs -> ironkv__marshal_v__impl0__foo
    parts = list(rel.parts)
    project = parts[0]
    filename = source_path.stem
    return f"{project}__{filename}"


def task_dir_for(source_path: Path) -> Path:
    return WORKSPACE / task_name_for(source_path)


def run_verus(filepath: Path, timeout: int = 120) -> dict:
    try:
        result = subprocess.run(
            [str(VERUS), str(filepath)],
            capture_output=True, text=True, timeout=timeout
        )
        output = result.stdout + result.stderr
        success = "0 errors" in output and "verified" in output
        return {"success": success, "output": output, "timed_out": False}
    except subprocess.TimeoutExpired:
        return {"success": False, "output": "TIMEOUT", "timed_out": True}
    except Exception as e:
        return {"success": False, "output": str(e), "timed_out": False}


def baseline_check(files: list, concurrency: int = 3) -> list:
    """Check which files pass baseline Verus verification. Returns valid files."""
    valid = []
    failed = []
    timeout = []

    def check_one(f):
        r = run_verus(f, timeout=60)
        return (f, r)

    print(f"Baseline check: {len(files)} files, concurrency={concurrency}")
    with ThreadPoolExecutor(max_workers=concurrency) as pool:
        futures = {pool.submit(check_one, f): f for f in files}
        done_count = 0
        for fut in as_completed(futures):
            done_count += 1
            f, r = fut.result()
            rel = f.relative_to(VERUSAGE)
            if r["success"]:
                valid.append(str(f))
                status = "✓"
            elif r["timed_out"]:
                timeout.append(str(f))
                status = "⏱"
            else:
                failed.append(str(f))
                status = "✗"
            print(f"  [{done_count}/{len(files)}] {status} {rel}")

    print(f"\nResults: {len(valid)} pass, {len(failed)} fail, {len(timeout)} timeout")
    return valid


def write_summary(task_dir: Path, source_path: str, phis: list, verdicts: list):
    """Write summary.md for a completed task."""
    summary = f"# Spec Consistency Report\n\n"
    summary += f"**Source:** `{source_path}`\n"
    summary += f"**Date:** {time.strftime('%Y-%m-%d %H:%M UTC', time.gmtime())}\n\n"

    tp_count = sum(1 for v in verdicts if v.get("verdict") == "TRUE_POSITIVE")
    fp_count = sum(1 for v in verdicts if v.get("verdict") == "FALSE_POSITIVE")
    summary += f"## Results\n\n"
    summary += f"- Generated: {len(phis)} candidates\n"
    summary += f"- Verified (entailed): {sum(1 for p in phis if p.get('entailed'))} / {len(phis)}\n"
    summary += f"- True positives: {tp_count}\n"
    summary += f"- False positives: {fp_count}\n\n"

    if tp_count > 0:
        summary += f"## True Positives (Spec Issues)\n\n"
        for v in verdicts:
            if v.get("verdict") == "TRUE_POSITIVE":
                summary += f"### {v['phi']}\n"
                summary += f"- **Confidence:** {v.get('confidence', '?')}\n"
                summary += f"- **Reasoning:** {v.get('reasoning', '?')}\n\n"

    summary += f"## All Candidates\n\n"
    for i, p in enumerate(phis):
        summary += f"### φ{i+1}: {p.get('name', 'unnamed')}\n"
        summary += f"- **Type:** {p.get('type', '?')}\n"
        summary += f"- **Entailed:** {'✅' if p.get('entailed') else '❌'}\n"
        if p.get('reason'):
            summary += f"- **Reason:** {p['reason']}\n"

        # Find matching verdict
        for v in verdicts:
            if v.get("phi") == p.get("name"):
                summary += f"- **Verdict:** {v['verdict']} ({v.get('confidence', '?')})\n"
                break
        summary += "\n"

    (task_dir / "summary.md").write_text(summary)


def main():
    parser = argparse.ArgumentParser(description="Spec Consistency Pipeline v2")
    parser.add_argument("--concurrency", type=int, default=3)
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--file", type=str, default=None)
    parser.add_argument("--baseline-only", action="store_true")
    args = parser.parse_args()

    if args.file:
        files = [Path(args.file)]
    else:
        files = find_verusage_files(limit=args.limit, offset=args.offset)

    print(f"Found {len(files)} VeruSage files")

    if args.baseline_only:
        valid = baseline_check(files, args.concurrency)
        out = WORKSPACE / "baseline_valid.json"
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(valid, indent=2))
        print(f"\nSaved {len(valid)} valid files to {out}")
        return

    # Single file mode: just set up the task directory
    if args.file:
        f = Path(args.file)
        tdir = task_dir_for(f)
        tdir.mkdir(parents=True, exist_ok=True)
        print(f"Task dir: {tdir}")
        # Check baseline
        r = run_verus(f)
        print(f"Baseline: {'✓' if r['success'] else '✗'}")
        if not r['success']:
            print(r['output'][:500])


if __name__ == "__main__":
    main()
