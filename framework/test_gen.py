"""
Generate Verus verified test functions using LLMs via GitHub Copilot CLI.

This is the test-generation counterpart to spec_gen.py.  Instead of
generating specifications, it generates verified test functions that
exercise those specifications.  The generated tests can then be used
by spec_checker.py to evaluate spec quality.

Usage:
    python -m framework.test_gen \
        --case bitmap \
        --project_dir ./bitmap \
        [--variant bitmap_new] \
        --output_dir /path/to/output/ \
        [--models claude-opus-4.6 gpt-5.3-codex] \
        [--num_tests 5] \
        [--max_workers 4] \
        [--timeout 600]

    python -m framework.test_gen \
        --case verusage \
        --tasks_jsonl /path/to/tasks.jsonl \
        --output_dir /path/to/output/ \
        [--language_path /path/to/verus.so] \
        [--models claude-opus-4.6] \
        [--num_tests 3]
"""

import argparse
import json
import os
import re
import subprocess
import tempfile
import traceback
from concurrent.futures import ThreadPoolExecutor, as_completed

from .adapters import BitmapAdapter, VeruSAGEAdapter, CaseAdapter
from .adapters.base import Task

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

COPILOT_BIN = os.environ.get(
    "COPILOT_BIN",
    os.path.expanduser(
        "~/.vscode-server/data/User/globalStorage/"
        "github.copilot-chat/copilotCli/copilot"
    ),
)

DEFAULT_MODELS = ["claude-opus-4.6"]

PROMPT_TEMPLATE_PATH = os.path.join(
    os.path.dirname(__file__), "prompts", "test_gen_template.md",
)

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _load_prompt_template() -> str:
    with open(PROMPT_TEMPLATE_PATH) as f:
        return f.read()


def _call_copilot(model: str, prompt: str, timeout: int = 600) -> str:
    cmd = [
        COPILOT_BIN,
        "--model", model,
        "-s",
        "--no-auto-update",
        "--allow-all-tools",
        "--allow-all-paths",
        "--allow-all-urls",
        "-p", prompt,
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    if proc.returncode != 0:
        raise RuntimeError(
            f"copilot exited {proc.returncode}:\n{proc.stderr.strip()}"
        )
    return proc.stdout.strip()


def _extract_code_block(text: str) -> str:
    """Extract the last ```rust code block from the LLM response."""
    pattern = r"```(?:rust|verus)?\s*\n(.*?)```"
    blocks = re.findall(pattern, text, flags=re.DOTALL)
    if blocks:
        return blocks[-1].strip()
    lines = text.split("\n")
    if lines and lines[0].strip().startswith("```"):
        lines = lines[1:]
    if lines and lines[-1].strip() == "```":
        lines = lines[:-1]
    return "\n".join(lines).strip()


# ---------------------------------------------------------------------------
# Core generation
# ---------------------------------------------------------------------------


def generate_tests_for_task(
    task: Task,
    adapter: CaseAdapter,
    model: str,
    num_tests: int,
    tmp_dir: str,
    timeout: int = 600,
) -> dict:
    """
    Generate verified test functions for one task.

    Returns a dict with:
      - task_id, target_function, model
      - generated_tests : raw LLM response
      - test_code       : extracted test function code
      - error           : error message (empty on success)
    """
    result = {
        "task_id": task.task_id,
        "target_function": task.target_function,
        "model": model,
        "generated_tests": "",
        "test_code": "",
        "error": "",
    }

    # Write the source to a temp file for the LLM to read.
    task_file = os.path.join(tmp_dir, f"{task.task_id}.rs")
    with open(task_file, "w") as f:
        f.write(task.source_code)

    # Build the prompt.
    template = _load_prompt_template()
    prompt = template.replace("{{file_path}}", task_file)
    prompt = prompt.replace("{{target_function}}", task.target_function)
    prompt = prompt.replace("{{num_tests}}", str(num_tests))
    prompt = prompt.replace("{{extra_context}}", "")

    try:
        raw_response = _call_copilot(model, prompt, timeout=timeout)
        result["generated_tests"] = raw_response
        result["test_code"] = _extract_code_block(raw_response)
    except subprocess.TimeoutExpired:
        result["error"] = "timeout"
    except Exception as e:
        result["error"] = str(e)

    return result


# ---------------------------------------------------------------------------
# Pipeline
# ---------------------------------------------------------------------------


def run_pipeline(args: argparse.Namespace) -> None:
    # Build the adapter.
    adapter: CaseAdapter
    if args.case == "bitmap":
        adapter = BitmapAdapter(
            project_dir=args.project_dir,
            variant=args.variant,
        )
    elif args.case == "verusage":
        adapter = VeruSAGEAdapter(
            tasks_jsonl=args.tasks_jsonl,
            language_path=args.language_path,
            start=args.start,
            end=args.end,
        )
    else:
        raise ValueError(f"Unknown case: {args.case}")

    tasks = list(adapter.iter_tasks())
    print(f"Loaded {len(tasks)} tasks for case '{args.case}'")

    tmp_dir = os.path.join(args.output_dir, "_tmp")
    os.makedirs(tmp_dir, exist_ok=True)

    for model in args.models:
        model_dir = os.path.join(args.output_dir, model.replace("/", "_"))
        os.makedirs(model_dir, exist_ok=True)
        results_path = os.path.join(model_dir, "test_gen_results.jsonl")

        # Resume support.
        done_ids: set = set()
        if os.path.exists(results_path):
            with open(results_path) as f:
                for line in f:
                    line = line.strip()
                    if line:
                        done_ids.add(json.loads(line)["task_id"])
            print(f"  Resuming {model}: {len(done_ids)} already done")

        remaining = [t for t in tasks if t.task_id not in done_ids]
        if not remaining:
            print(f"  [{model}] All tasks done — skipping.")
            continue

        print(f"  [{model}] Generating tests for {len(remaining)} tasks "
              f"(workers={args.max_workers}, timeout={args.timeout}s) ...")

        succeeded = 0
        failed = 0

        def _process(task: Task) -> dict:
            return generate_tests_for_task(
                task, adapter, model, args.num_tests, tmp_dir, args.timeout,
            )

        with open(results_path, "a") as out_f:
            if args.max_workers <= 1:
                for idx, task in enumerate(remaining):
                    res = _process(task)
                    out_f.write(json.dumps(res, ensure_ascii=False) + "\n")
                    out_f.flush()
                    if res["error"]:
                        failed += 1
                    else:
                        succeeded += 1
                    if (idx + 1) % 10 == 0 or idx + 1 == len(remaining):
                        print(f"    [{model}] {idx+1}/{len(remaining)}  "
                              f"ok={succeeded} err={failed}")
            else:
                futures = {}
                with ThreadPoolExecutor(max_workers=args.max_workers) as pool:
                    for task in remaining:
                        fut = pool.submit(_process, task)
                        futures[fut] = task.task_id

                    for idx, fut in enumerate(as_completed(futures)):
                        try:
                            res = fut.result()
                        except Exception:
                            res = {
                                "task_id": futures[fut],
                                "error": traceback.format_exc(),
                                "model": model,
                                "generated_tests": "",
                                "test_code": "",
                            }
                        out_f.write(json.dumps(res, ensure_ascii=False) + "\n")
                        out_f.flush()
                        if res.get("error"):
                            failed += 1
                        else:
                            succeeded += 1
                        if (idx + 1) % 10 == 0 or idx + 1 == len(remaining):
                            print(f"    [{model}] {idx+1}/{len(remaining)}  "
                                  f"ok={succeeded} err={failed}")

        print(f"  [{model}] Done.  succeeded={succeeded}  failed={failed}")
        print(f"  Results → {results_path}")

    print("\nAll models finished.")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main():
    p = argparse.ArgumentParser(
        description="Generate Verus verified tests via Copilot CLI.")

    p.add_argument("--case", type=str, required=True,
                   choices=["bitmap", "verusage"],
                   help="Case study to process")

    # Bitmap-specific.
    p.add_argument("--project_dir", type=str, default="./bitmap",
                   help="Root of the bitmap project (contains bitmap_new/, etc.)")
    p.add_argument("--variant", type=str, default="bitmap_new",
                   help="Bitmap variant: bitmap_new or bitmap_raw")

    # VeruSAGE-specific.
    p.add_argument("--tasks_jsonl", type=str, default=None,
                   help="Path to VeruSAGE-Bench tasks.jsonl")
    p.add_argument("--language_path", type=str, default=None,
                   help="Path to tree-sitter verus .so file")

    # Common.
    p.add_argument("--output_dir", type=str, required=True,
                   help="Output directory for results")
    p.add_argument("--models", nargs="+", type=str,
                   default=DEFAULT_MODELS,
                   help="LLM model names for Copilot CLI")
    p.add_argument("--num_tests", type=int, default=5,
                   help="Number of test functions to generate per task")
    p.add_argument("--max_workers", type=int, default=4,
                   help="Max parallel Copilot CLI invocations")
    p.add_argument("--timeout", type=int, default=600,
                   help="Per-task timeout in seconds")
    p.add_argument("--start", type=int, default=0,
                   help="Start index for VeruSAGE tasks (inclusive)")
    p.add_argument("--end", type=int, default=-1,
                   help="End index for VeruSAGE tasks (exclusive, -1=all)")

    args = p.parse_args()
    run_pipeline(args)


if __name__ == "__main__":
    main()
