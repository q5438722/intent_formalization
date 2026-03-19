"""
Generate Verus specifications AND proofs for VeruSAGE-Bench tasks using
GitHub Copilot CLI.

For each task the target function's specs are stripped (same as spec_gen.py),
then Copilot is asked to write **both** specification clauses **and** proof
bodies / assertions / loop invariants that make Verus fully verify the file
with 0 errors.

Usage:
    python spec_proof_gen.py \
        --input  /path/to/tasks.jsonl \
        --output_dir /path/to/output/ \
        [--models claude-opus-4.5 gpt-5.2] \
        [--language_path /path/to/verus.so] \
        [--max_workers 4] \
        [--start 0] [--end -1]

Outputs (one per model):
    <output_dir>/<model>/results.jsonl   — each line is the original task dict
                                           plus generated fields
"""

import os
import sys
import json
import re
import argparse
import subprocess
import traceback
from concurrent.futures import ThreadPoolExecutor, as_completed

# ---------------------------------------------------------------------------
# verus_parser imports (for spec-less task preparation)
# ---------------------------------------------------------------------------
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'static'))
from verus_parser import verus_editor, node_to_text          # noqa: E402
from verusage import remove_target_fn_spec                    # noqa: E402

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

DEFAULT_MODELS = ["claude-opus-4.5", "gpt-5.2"]

VERUS_BIN = os.environ.get(
    "VERUS_BIN",
    os.path.join(
        os.path.dirname(__file__),
        "verus", "source", "target-verus", "release", "verus",
    ),
)

PROMPT_TEMPLATE = """\
Read the file {file_path}.

This is a Verus (verified Rust) program. The target function `{target_function}` \
currently has NO specification clauses (no requires / ensures / recommends / \
decreases) and its proof body may be incomplete or empty.

Your task is to make this file **fully verify** with Verus — that means the \
verification output must show **0 errors** (e.g. "verification results:: \
N verified, 0 errors").

Concretely you must:
1. Analyse the full file to understand the purpose of `{target_function}`, its \
parameters, return type, helper lemmas, and how it is used elsewhere in the file.
2. Write appropriate Verus **specification** clauses (requires, ensures, and if \
needed recommends or decreases) for `{target_function}`.
3. Write or complete the **proof body** of `{target_function}` — including any \
proof blocks (proof {{ ... }}), assert-by blocks, loop invariants \
(invariant clauses inside while/for loops), decreases clauses for recursive \
functions, and any helper lemma invocations needed to discharge proof \
obligations.
4. Edit the file {file_path} in-place with your changes, then save.
5. Run the Verus verifier:
       {verus_bin} {file_path}
6. Examine the output.
   - Your target is: **verification results:: N verified, 0 errors** \
(N >= 1, errors MUST be 0).
   - If there are verification errors (e.g. "1 verified, 2 errors"), read the \
error messages carefully, fix the specs and/or proof in the file, save, and \
re-run Verus. Repeat until you reach 0 errors or you are confident no further \
progress can be made.
   - If Verus reports a syntax or parse error (no "verification results" line), \
fix the syntax first and re-run.
7. Once you have your best result, output your FINAL answer in EXACTLY this \
format:

   VERIFICATION: <paste the "verification results:: ..." line here>

   ```rust
   <the COMPLETE content of the file {file_path} as it is now>
   ```

IMPORTANT:
- The ```rust code block must contain the ENTIRE file, not just the target \
function.
- Your final message MUST contain both the VERIFICATION line and the full-file \
```rust code block.
- Aim for 0 errors. If you cannot reach 0 errors after several iterations, \
still output your best attempt.\
"""

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _call_copilot(
    model: str,
    prompt: str,
    timeout: int = 600,
) -> str:
    """
    Invoke the Copilot CLI in non-interactive mode and return stdout.
    """
    cmd = [
        COPILOT_BIN,
        "--model", model,
        "-s",                       # silent — agent response only
        "--no-auto-update",
        "--allow-all-tools",        # allow file reads & shell
        "--allow-all-paths",        # no path restrictions
        "--allow-all-urls",         # no URL restrictions
        "-p", prompt,
    ]
    proc = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"copilot exited {proc.returncode}:\n{proc.stderr.strip()}"
        )
    return proc.stdout.strip()


def _extract_code_block(text: str) -> str:
    """
    Extract the last ```rust (or ```verus / ```) code block from the
    Copilot response.
    """
    pattern = r"```(?:rust|verus)?\s*\n(.*?)```"
    blocks = re.findall(pattern, text, flags=re.DOTALL)
    if blocks:
        return blocks[-1].strip()
    # Fallback: strip leading/trailing fences if present
    lines = text.split("\n")
    if lines and lines[0].strip().startswith("```"):
        lines = lines[1:]
    if lines and lines[-1].strip() == "```":
        lines = lines[:-1]
    return "\n".join(lines).strip()


def _extract_verification_result(text: str) -> str:
    """
    Extract the ``verification results:: ...`` line from the Copilot
    response (or from the VERIFICATION: tag we asked for).
    Returns the matched line, or empty string if not found.
    """
    # Look for the VERIFICATION: tag first
    m = re.search(r"VERIFICATION:\s*(.*)", text)
    if m:
        return m.group(1).strip()
    # Fallback: look for raw Verus output
    m = re.search(r"verification results::.*", text, re.IGNORECASE)
    if m:
        return m.group(0).strip()
    return ""


def _verification_has_zero_errors(verification_line: str) -> bool:
    """Return True if the verification line shows 0 errors."""
    m = re.search(r"(\d+)\s+errors?", verification_line, re.IGNORECASE)
    if m:
        return int(m.group(1)) == 0
    return False


def generate_spec_proof_for_task(
    task: dict,
    model: str,
    language_path: str,
    tmp_dir: str,
    timeout: int = 600,
) -> dict:
    """
    Generate specifications AND proofs for one task using one model.

    Returns a copy of *task* augmented with:
        - ``generated_raw``   : raw Copilot response
        - ``generated_file``  : extracted full file content (fences stripped)
        - ``verification``    : the "verification results:: ..." line (or "")
        - ``verified_zero_errors`` : bool — True if 0 errors achieved
        - ``model``           : model name used
        - ``error``           : error message, if any (empty string on success)
    """
    result = dict(task)
    result["model"] = model
    result["generated_raw"] = ""
    result["generated_file"] = ""
    result["verification"] = ""
    result["verified_zero_errors"] = False
    result["error"] = ""

    task_id = task["task_id"]
    target_fn = task["target_function"]
    task_code = task["task"]

    # ---- Strip specs from the target function (idempotent) ----
    no_spec_code = remove_target_fn_spec(task_code, target_fn, language_path)

    # ---- Write code to a temp file so Copilot can read & edit it ----
    tmp_file = os.path.join(tmp_dir, f"{task_id}.rs")
    with open(tmp_file, "w") as f:
        f.write(no_spec_code)

    prompt = PROMPT_TEMPLATE.format(
        file_path=tmp_file,
        target_function=target_fn,
        verus_bin=os.path.abspath(VERUS_BIN),
    )

    try:
        raw_response = _call_copilot(model, prompt, timeout=timeout)
        result["generated_raw"] = raw_response
        result["generated_file"] = _extract_code_block(raw_response)
        result["verification"] = _extract_verification_result(raw_response)
        result["verified_zero_errors"] = _verification_has_zero_errors(
            result["verification"]
        )
    except subprocess.TimeoutExpired:
        result["error"] = "timeout"
    except Exception as e:
        result["error"] = str(e)

    return result


# ---------------------------------------------------------------------------
# Main pipeline
# ---------------------------------------------------------------------------


def run_pipeline(args: argparse.Namespace) -> None:
    # ---- Load tasks ----
    tasks = []
    with open(args.input, "r") as f:
        for line in f:
            line = line.strip()
            if line:
                tasks.append(json.loads(line))

    end = args.end if args.end >= 0 else len(tasks)
    tasks = tasks[args.start : end]
    print(f"Loaded {len(tasks)} tasks  (range [{args.start}, {end}))")

    # ---- Prepare temp dir ----
    tmp_dir = os.path.join(args.output_dir, "_tmp")
    os.makedirs(tmp_dir, exist_ok=True)

    for model in args.models:
        model_dir = os.path.join(args.output_dir, model.replace("/", "_"))
        os.makedirs(model_dir, exist_ok=True)
        results_path = os.path.join(model_dir, "results.jsonl")

        # ---- Resume support: skip already-completed task_ids ----
        done_ids: set = set()
        if os.path.exists(results_path):
            with open(results_path, "r") as f:
                for line in f:
                    line = line.strip()
                    if line:
                        done_ids.add(json.loads(line)["task_id"])
            print(f"  Resuming {model}: {len(done_ids)} already done")

        remaining = [t for t in tasks if t["task_id"] not in done_ids]
        if not remaining:
            print(f"  [{model}] All tasks already completed — skipping.")
            continue

        print(f"  [{model}] Processing {len(remaining)} tasks "
              f"(workers={args.max_workers}, timeout={args.timeout}s) ...")

        succeeded = 0
        failed = 0
        zero_errors = 0

        # ---- Sequential or parallel ---------------------------------
        def _process(task):
            return generate_spec_proof_for_task(
                task, model, args.language_path, tmp_dir, args.timeout,
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
                    if res.get("verified_zero_errors"):
                        zero_errors += 1
                    if (idx + 1) % 10 == 0 or idx + 1 == len(remaining):
                        print(f"    [{model}] {idx+1}/{len(remaining)}  "
                              f"ok={succeeded} err={failed} "
                              f"0-errors={zero_errors}")
            else:
                futures = {}
                with ThreadPoolExecutor(max_workers=args.max_workers) as pool:
                    for task in remaining:
                        fut = pool.submit(_process, task)
                        futures[fut] = task["task_id"]

                    for idx, fut in enumerate(as_completed(futures)):
                        try:
                            res = fut.result()
                        except Exception:
                            res = {
                                "task_id": futures[fut],
                                "error": traceback.format_exc(),
                                "model": model,
                                "generated_raw": "",
                                "generated_file": "",
                                "verification": "",
                                "verified_zero_errors": False,
                            }
                        out_f.write(json.dumps(res, ensure_ascii=False) + "\n")
                        out_f.flush()
                        if res.get("error"):
                            failed += 1
                        else:
                            succeeded += 1
                        if res.get("verified_zero_errors"):
                            zero_errors += 1
                        if (idx + 1) % 10 == 0 or idx + 1 == len(remaining):
                            print(f"    [{model}] {idx+1}/{len(remaining)}  "
                                  f"ok={succeeded} err={failed} "
                                  f"0-errors={zero_errors}")

        print(f"  [{model}] Done.  succeeded={succeeded}  failed={failed}  "
              f"zero_errors={zero_errors}")
        print(f"  Results → {results_path}")

    print("\nAll models finished.")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main():
    p = argparse.ArgumentParser(
        description="Generate Verus specs + proofs for VeruSAGE tasks "
                    "via Copilot CLI.")

    p.add_argument(
        "--input", type=str,
        default="/home/chentianyu/verus-proof-synthesis/"
                "benchmarks/VeruSAGE-Bench/tasks.jsonl",
        help="Path to the input tasks.jsonl",
    )
    p.add_argument(
        "--output_dir", type=str,
        default="/home/chentianyu/data/spec_proof_gen_verusage",
        help="Directory for per-model output JSONL files",
    )
    p.add_argument(
        "--models", nargs="+", type=str,
        default=DEFAULT_MODELS,
        help="Copilot CLI model names to use "
             f"(default: {' '.join(DEFAULT_MODELS)})",
    )
    p.add_argument(
        "--language_path", type=str,
        default="/home/chentianyu/verus.so",
        help="Path to the tree-sitter verus .so file",
    )
    p.add_argument(
        "--max_workers", type=int, default=4,
        help="Max parallel Copilot CLI invocations per model",
    )
    p.add_argument(
        "--timeout", type=int, default=600,
        help="Per-task timeout in seconds",
    )
    p.add_argument("--start", type=int, default=0,
                   help="Start index (inclusive)")
    p.add_argument("--end", type=int, default=-1,
                   help="End index (exclusive, -1 = all)")

    args = p.parse_args()
    run_pipeline(args)


if __name__ == "__main__":
    main()
