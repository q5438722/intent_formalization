#!/home/chentianyu/miniconda3/bin/python3
"""
Step 2: Generate test candidates (φ) for each file's exec functions.

Two sub-steps:
  2a: Spec-only generation — adversarial properties from requires/ensures
  2b: Body-aware generation — properties the body guarantees but spec doesn't

Reads:  workspace/<task_name>/original.rs + exec_functions.json (or global exec_functions.json)
Writes: workspace/<task_name>/generator_1a_raw.txt
        workspace/<task_name>/generator_1b_raw.txt
        workspace/<task_name>/candidates.json

Usage:
  python3 step2_generate.py [--limit N] [--offset N] [--model MODEL] [--workspace DIR]
"""

import argparse
import json
import shutil
import sys
import time
from pathlib import Path

BASE = Path.home() / "intent_formalization"

sys.path.insert(0, str(BASE / "src" / "utils"))
from llm import LLMClient
from pipeline_common import extract_spec_portion, parse_phi_blocks


# ---------------------------------------------------------------------------
# Prompts
# ---------------------------------------------------------------------------

SPEC_ONLY_PROMPT = """You are a spec consistency query generator for Verus (Rust verification).

You are given a Verus source file and a list of EXECUTABLE functions (not spec/proof functions).
Your job: generate candidate "undesirable properties" targeting ONLY these executable functions' specifications (requires/ensures).

For EACH candidate, output in this EXACT format:

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
SOURCE: spec_only
CODE:
```verus
proof fn phi_<n>_<snake_name>(<params>)
    requires
        <preconditions from the spec>,
    ensures
        <the undesirable property>,
{
}
```
REASON: <one line why this would be undesirable if entailed>
===PHI_END===

RULES:
- Generate AT LEAST 5 candidates (more is better), all targeting the listed exec functions
- Each proof fn will be appended inside the existing verus!{} block
- Use types/functions/traits from the source file
- Do NOT add new `use`/`mod` statements or wrap in verus!{}
- Keep proof bodies SHORT — rely on Verus's SMT solver
- Spread tests across exec functions if there are multiple
"""


BODY_AWARE_PROMPT = """You are a spec incompleteness detector for Verus (Rust verification).

You are given a Verus source file and EXECUTABLE functions with their full body AND specification.
Your job: find properties that the function body guarantees but the specification DOES NOT express.
These are spec *incompleteness* gaps — the function does something useful that callers can't rely on
because the spec doesn't promise it.

Strategy:
- Read the function body carefully. What does it actually compute/guarantee?
- Compare with the ensures clause. What's missing?
- Focus on behavioral intent: if a human reads the body, what would they expect the spec to say?
- Look for comments like "not strictly needed", TODO, FIXME near specs — developer-acknowledged gaps

Generate candidates in three dimensions:
- **Behavioral**: Body guarantees a semantic property (e.g., monotonicity, completeness, forward progress) not in ensures
- **Boundary**: Body handles edge cases (empty input, zero, overflow) but spec doesn't distinguish them
- **Logical**: Body maintains relationships between outputs (e.g., result.start + result.count <= bound) not stated

For EACH candidate, output in this EXACT format:

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
SOURCE: body_aware
CODE:
```verus
proof fn phi_<n>_<snake_name>(<params>)
    requires
        <preconditions from the spec>,
    ensures
        <the property that body guarantees but spec doesn't>,
{
}
```
REASON: <what the body does that the spec doesn't capture>
===PHI_END===

RULES:
- Generate AT LEAST 5 candidates (more is better)
- Each proof fn will be appended inside the existing verus!{} block
- Use types/functions/traits from the source file
- Do NOT add new `use`/`mod` statements or wrap in verus!{}
- Focus on what's MISSING from the spec, not what's wrong with the body
"""


# ---------------------------------------------------------------------------
# Sub-steps
# ---------------------------------------------------------------------------

def generate_spec_only(llm: LLMClient, model: str, spec_text: str, exec_section: str) -> tuple[str, list]:
    """Step 2a: Generate φ from spec alone."""
    user_prompt = (
        f"Source file (spec-relevant portions):\n\n```rust\n{spec_text}\n```\n"
        f"{exec_section}\n"
        f"Generate at least 5 candidate undesirable properties targeting ONLY the exec functions listed above."
    )
    try:
        resp = llm.chat(SPEC_ONLY_PROMPT, user_prompt, model=model)
        raw = resp.content
    except Exception as e:
        raw = f"ERROR: {e}"
    return raw, parse_phi_blocks(raw)


def generate_body_aware(llm: LLMClient, model: str, spec_text: str, exec_section: str) -> tuple[str, list]:
    """Step 2b: Generate φ from body vs spec comparison."""
    user_prompt = (
        f"Source file (spec-relevant portions):\n\n```rust\n{spec_text}\n```\n"
        f"{exec_section}\n"
        f"Generate at least 5 candidate properties that the body guarantees but the spec does NOT express."
    )
    try:
        resp = llm.chat(BODY_AWARE_PROMPT, user_prompt, model=model)
        raw = resp.content
    except Exception as e:
        raw = f"ERROR: {e}"
    return raw, parse_phi_blocks(raw)


# ---------------------------------------------------------------------------
# Task processing
# ---------------------------------------------------------------------------

def process_one(entry: dict, llm: LLMClient, model: str, workspace: Path) -> dict:
    """Generate candidates for one file."""
    task_name = entry["task_name"]
    fpath = Path(entry["file_path"])
    task_dir = workspace / task_name
    task_dir.mkdir(parents=True, exist_ok=True)

    source_text = fpath.read_text()
    shutil.copy2(fpath, task_dir / "original.rs")
    (task_dir / "exec_functions.json").write_text(
        json.dumps(entry["exec_functions"], indent=2))

    spec_text = extract_spec_portion(source_text)
    exec_section = "\n\n## Executable Functions to Test:\n\n"
    for fn in entry["exec_functions"]:
        exec_section += f"### `{fn['name']}`\n```verus\n{fn['code']}\n```\n\n"

    # Step 2a: Spec-only
    print(f"  [2a] {task_name} ({len(entry['exec_functions'])} exec fns) — spec-only")
    raw_1a, candidates_1a = generate_spec_only(llm, model, spec_text, exec_section)
    (task_dir / "generator_1a_raw.txt").write_text(raw_1a)

    # Step 2b: Body-aware
    print(f"  [2b] {task_name} — body-aware")
    raw_1b, candidates_1b = generate_body_aware(llm, model, spec_text, exec_section)
    (task_dir / "generator_1b_raw.txt").write_text(raw_1b)

    # Merge
    candidates = candidates_1a + candidates_1b
    (task_dir / "candidates.json").write_text(json.dumps(candidates, indent=2))

    status = "ok" if candidates else "no_candidates"
    if not candidates:
        print(f"  [skip] {task_name} — no candidates parsed")

    return {
        "task_name": task_name,
        "candidates": len(candidates),
        "candidates_2a": len(candidates_1a),
        "candidates_2b": len(candidates_1b),
        "status": status,
    }


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Step 2: Generate φ candidates (spec-only + body-aware)")
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--model", type=str, default="claude-opus-4.6")
    parser.add_argument("--workspace", type=str, default=str(BASE / "verusage" / "workspace_v4"))
    args = parser.parse_args()

    workspace = Path(args.workspace)
    entries = json.loads((workspace / "exec_functions.json").read_text())
    entries = entries[args.offset:]
    if args.limit:
        entries = entries[:args.limit]

    print(f"Step 2: Generating candidates for {len(entries)} files (model={args.model})")
    print(f"  Workspace: {workspace}")

    llm = LLMClient(timeout=300)
    results = []

    for i, entry in enumerate(entries):
        print(f"\n[{i+1}/{len(entries)}]")
        try:
            r = process_one(entry, llm, args.model, workspace)
            results.append(r)
        except Exception as e:
            print(f"  [error] {entry['task_name']}: {e}")
            results.append({"task_name": entry["task_name"], "status": "error", "error": str(e)})

    ok = sum(1 for r in results if r["status"] == "ok")
    total_2a = sum(r.get("candidates_2a", 0) for r in results)
    total_2b = sum(r.get("candidates_2b", 0) for r in results)
    print(f"\n=== Done: {ok}/{len(results)} with candidates ({total_2a} spec-only + {total_2b} body-aware) ===")

    progress_file = workspace / "step2_progress.json"
    existing = json.loads(progress_file.read_text()) if progress_file.exists() else []
    existing.extend(results)
    progress_file.write_text(json.dumps(existing, indent=2))


if __name__ == "__main__":
    main()
