#!/home/chentianyu/miniconda3/bin/python3
"""
Step 2: Generate test candidates (φ) for each file's exec functions.

Reads:  workspace_v3/exec_functions.json
Writes: workspace_v3/<task_name>/original.rs
        workspace_v3/<task_name>/exec_functions.json
        workspace_v3/<task_name>/generator_raw.txt
        workspace_v3/<task_name>/candidates.json

Usage:
  python3 step2_generate.py [--limit N] [--offset N] [--model MODEL]
"""

import argparse
import json
import os
import re
import shutil
import sys
import time
from pathlib import Path

BASE = Path.home() / "intent_formalization"
WORKSPACE = BASE / "verusage" / "workspace_v3"

sys.path.insert(0, str(BASE / "src" / "utils"))
from llm import LLMClient

GENERATOR_PROMPT = """You are a spec consistency query generator for Verus (Rust verification).

You are given a Verus source file and a list of EXECUTABLE functions (not spec/proof functions).
Your job: generate candidate "undesirable properties" targeting ONLY these executable functions' specifications (requires/ensures).

For EACH candidate, output in this EXACT format:

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
CODE:
```verus
proof fn phi_<n>_<snake_name>(<params>)
    requires
        <preconditions from the spec>,
    ensures
        <the undesirable property>,
{
    // proof body (can be empty)
}
```
REASON: <one line why this would be undesirable if entailed>
===PHI_END===

RULES:
- Generate EXACTLY 5 candidates, all targeting the listed exec functions
- Each proof fn will be appended inside the existing verus!{} block
- Use types/functions/traits from the source file
- Do NOT add new `use`/`mod` statements or wrap in verus!{}
- Keep proof bodies SHORT — rely on Verus's SMT solver
- Spread tests across exec functions if there are multiple
"""


def parse_phi_blocks(text: str) -> list:
    blocks = []
    pattern = r'===PHI_START===(.*?)===PHI_END==='
    for match in re.finditer(pattern, text, re.DOTALL):
        block = match.group(1).strip()
        name_m = re.search(r'NAME:\s*(.+)', block)
        target_m = re.search(r'TARGET_FN:\s*(.+)', block)
        type_m = re.search(r'TYPE:\s*(.+)', block)
        code_m = re.search(r'```(?:verus|rust)?\s*\n(.*?)```', block, re.DOTALL)
        reason_m = re.search(r'REASON:\s*(.+)', block)

        if name_m and code_m:
            blocks.append({
                "name": name_m.group(1).strip(),
                "target_fn": target_m.group(1).strip() if target_m else "",
                "type": type_m.group(1).strip() if type_m else "unknown",
                "code": code_m.group(1).strip(),
                "reason": reason_m.group(1).strip() if reason_m else "",
            })
    return blocks


def extract_spec_portion(source_text: str, max_lines: int = 400) -> str:
    """For large files, extract spec-relevant portions."""
    lines = source_text.split('\n')
    if len(lines) <= max_lines:
        return source_text

    spec_keywords = ['requires', 'ensures', 'invariant', 'recommends', 'spec fn',
                     'proof fn', 'decreases', 'open spec', 'closed spec',
                     'pub proof', 'pub open spec', 'pub closed spec',
                     'pub fn', 'fn ', 'struct ', 'impl ', 'trait ', 'enum ']
    important = set()
    for i, line in enumerate(lines):
        low = line.lower().strip()
        if any(kw in low for kw in spec_keywords):
            for j in range(max(0, i - 3), min(len(lines), i + 8)):
                important.add(j)

    # Always include first 30 and last 5 lines
    for i in range(min(30, len(lines))):
        important.add(i)
    for i in range(max(0, len(lines) - 5), len(lines)):
        important.add(i)

    selected = sorted(important)[:max_lines]
    result = []
    prev = -2
    for i in selected:
        if i > prev + 1:
            result.append(f"// ... (lines {prev+2}-{i-1} omitted) ...")
        result.append(lines[i])
        prev = i
    return '\n'.join(result)


def process_one(entry: dict, llm: LLMClient, model: str) -> dict:
    """Generate candidates for one file."""
    task_name = entry["task_name"]
    fpath = Path(entry["file_path"])
    task_dir = WORKSPACE / task_name
    task_dir.mkdir(parents=True, exist_ok=True)

    source_text = fpath.read_text()

    # Save originals
    shutil.copy2(fpath, task_dir / "original.rs")
    (task_dir / "exec_functions.json").write_text(
        json.dumps(entry["exec_functions"], indent=2))

    # Build prompt
    spec_text = extract_spec_portion(source_text)
    exec_section = "\n\n## Executable Functions to Test:\n\n"
    for fn in entry["exec_functions"]:
        exec_section += f"### `{fn['name']}`\n```verus\n{fn['code']}\n```\n\n"

    user_prompt = (
        f"Source file (spec-relevant portions):\n\n```rust\n{spec_text}\n```\n"
        f"{exec_section}\n"
        f"Generate 5 candidate undesirable properties targeting ONLY the exec functions listed above."
    )

    # Call LLM
    print(f"  [gen] {task_name} ({len(entry['exec_functions'])} exec fns)")
    try:
        resp = llm.chat(GENERATOR_PROMPT, user_prompt, model=model)
        raw_text = resp.content
    except Exception as e:
        raw_text = f"ERROR: {e}"

    (task_dir / "generator_raw.txt").write_text(raw_text)

    # Parse
    candidates = parse_phi_blocks(raw_text)
    (task_dir / "candidates.json").write_text(json.dumps(candidates, indent=2))

    status = "ok" if candidates else "no_candidates"
    if not candidates:
        print(f"  [skip] {task_name} — no candidates parsed")

    return {
        "task_name": task_name,
        "candidates": len(candidates),
        "status": status,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--model", type=str, default="claude-opus-4.6")
    args = parser.parse_args()

    entries = json.loads((WORKSPACE / "exec_functions.json").read_text())
    entries = entries[args.offset:]
    if args.limit:
        entries = entries[:args.limit]

    print(f"Step 2: Generating candidates for {len(entries)} files (model={args.model})")

    llm = LLMClient(timeout=300)
    results = []

    for i, entry in enumerate(entries):
        print(f"\n[{i+1}/{len(entries)}]")
        try:
            r = process_one(entry, llm, args.model)
            results.append(r)
        except Exception as e:
            print(f"  [error] {entry['task_name']}: {e}")
            results.append({"task_name": entry["task_name"], "status": "error", "error": str(e)})

    # Summary
    ok = sum(1 for r in results if r["status"] == "ok")
    print(f"\n=== Done: {ok}/{len(results)} with candidates ===")

    # Save progress
    progress_file = WORKSPACE / "step2_progress.json"
    existing = json.loads(progress_file.read_text()) if progress_file.exists() else []
    existing.extend(results)
    progress_file.write_text(json.dumps(existing, indent=2))


if __name__ == "__main__":
    main()
