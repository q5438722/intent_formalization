#!/usr/bin/env python3
"""
Spec Consistency Pipeline — Batch runner over VeruSage files.

Usage:
  python3 run_pipeline.py [--concurrency N] [--limit N] [--offset N] [--file PATH]

Runs generator → entailment check → critic for each Verus source file.
Results written to ~/intent_formalization/results/<project>/<filename>.json
"""

import argparse
import asyncio
import json
import os
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path

BASE = Path.home() / "intent_formalization"
VERUS = BASE / "verus" / "verus"
VERUSAGE = BASE / "verusage" / "source-projects"
RESULTS = BASE / "results"

# ── Prompts ──────────────────────────────────────────────────────────

GENERATOR_SYSTEM = """You are a spec consistency query generator for Verus (Rust verification).

Given a Verus source file, you:
1. Read and understand its specifications (requires/ensures/invariants/recommends)
2. Generate exactly 5 candidate "undesirable properties" — things the spec probably should NOT entail

For EACH candidate, output in this EXACT format (parseable):

===PHI_START===
NAME: <short name>
TYPE: behavioral | boundary | logical
CODE:
```verus
proof fn phi_<n>_<snake_name>(<params>)
    requires
        <preconditions from the spec>,
    ensures
        <the undesirable property>,
{
    // proof body (can be empty if you expect Verus to figure it out)
}
```
REASON: <why this would be undesirable if entailed>
===PHI_END===

IMPORTANT:
- Each proof fn must be SELF-CONTAINED — it will be appended after the original file's definitions
- Use types/functions/traits defined in the source file
- Do NOT add new `use` statements or `mod` declarations
- Do NOT wrap in verus!{} — the proof fn will be inserted inside an existing verus!{} block
"""

CRITIC_SYSTEM = """You are a spec consistency critic for Verus.

You receive a Verus source file and a list of candidate properties (φ) that the spec has been
verified to entail. For each φ, decide:
- TRUE_POSITIVE: real spec issue (too weak, too strong, missing axiom, unverified assumption)
- FALSE_POSITIVE: actually a desirable property, not a spec gap

Output EXACTLY this format for each:

===VERDICT_START===
PHI: <name>
VERDICT: TRUE_POSITIVE | FALSE_POSITIVE
CONFIDENCE: high | medium | low
REASONING: <2-3 sentences>
===VERDICT_END===
"""


# ── Helpers ──────────────────────────────────────────────────────────

def find_verusage_files(limit=None, offset=0):
    """Find all verified .rs files in VeruSage."""
    files = sorted(VERUSAGE.rglob("verified/**/*.rs"))
    files = files[offset:]
    if limit:
        files = files[:limit]
    return files


def run_verus(filepath: Path, timeout: int = 120) -> dict:
    """Run verus on a file, return {success, output, timed_out}."""
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


def call_llm(system_prompt: str, user_prompt: str) -> str:
    """Call LLM via openclaw's subagent. Falls back to direct API if available."""
    # Use a temp file approach: write prompt, call openclaw CLI
    # For now, use subprocess to call the LLM via a simple script
    with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
        f.write(user_prompt)
        prompt_file = f.name

    try:
        result = subprocess.run(
            ["node", "-e", f"""
const {{ execSync }} = require('child_process');
const fs = require('fs');
const prompt = fs.readFileSync('{prompt_file}', 'utf8');
const system = {json.dumps(system_prompt)};
// Use OpenClaw's internal LLM call
const payload = JSON.stringify({{
    model: "github-copilot/claude-sonnet-4",
    messages: [
        {{ role: "system", content: system }},
        {{ role: "user", content: prompt }}
    ],
    max_tokens: 4096
}});
// Write to stdout for the python script to capture
process.stdout.write(payload);
"""],
            capture_output=True, text=True, timeout=10
        )
        # If node approach doesn't work, fall back to file-based approach
        return result.stdout
    except Exception:
        pass
    finally:
        os.unlink(prompt_file)

    return ""


def parse_phi_blocks(text: str) -> list:
    """Parse generator output into structured phi blocks."""
    blocks = []
    pattern = r'===PHI_START===(.*?)===PHI_END==='
    for match in re.finditer(pattern, text, re.DOTALL):
        block = match.group(1).strip()
        name_m = re.search(r'NAME:\s*(.+)', block)
        type_m = re.search(r'TYPE:\s*(.+)', block)
        code_m = re.search(r'```(?:verus|rust)?\s*\n(.*?)```', block, re.DOTALL)
        reason_m = re.search(r'REASON:\s*(.+)', block, re.DOTALL)

        if name_m and code_m:
            # Clean up reason - only take text before any ===
            reason = reason_m.group(1).strip() if reason_m else ""
            reason = reason.split("===")[0].strip()
            blocks.append({
                "name": name_m.group(1).strip(),
                "type": type_m.group(1).strip() if type_m else "unknown",
                "code": code_m.group(1).strip(),
                "reason": reason,
            })
    return blocks


def parse_verdicts(text: str) -> list:
    """Parse critic output into structured verdicts."""
    verdicts = []
    pattern = r'===VERDICT_START===(.*?)===VERDICT_END==='
    for match in re.finditer(pattern, text, re.DOTALL):
        block = match.group(1).strip()
        phi_m = re.search(r'PHI:\s*(.+)', block)
        verdict_m = re.search(r'VERDICT:\s*(.+)', block)
        conf_m = re.search(r'CONFIDENCE:\s*(.+)', block)
        reason_m = re.search(r'REASONING:\s*(.+)', block, re.DOTALL)

        if phi_m and verdict_m:
            reason = reason_m.group(1).strip() if reason_m else ""
            reason = reason.split("===")[0].strip()
            verdicts.append({
                "phi": phi_m.group(1).strip(),
                "verdict": verdict_m.group(1).strip(),
                "confidence": conf_m.group(1).strip() if conf_m else "unknown",
                "reasoning": reason,
            })
    return verdicts


def build_entailment_file(source_path: Path, phi_code: str, tmp_dir: Path) -> Path:
    """Create a standalone .rs file with the original source + one phi appended."""
    source = source_path.read_text()

    # Find the last closing `}` of the verus!{} block and insert phi before it
    # Strategy: remove the final `}` (closing verus! block), append phi, re-add `}`
    last_brace = source.rstrip().rfind('}')
    if last_brace == -1:
        raise ValueError(f"Cannot find closing brace in {source_path}")

    modified = source[:last_brace] + "\n\n// === Entailment query ===\n" + phi_code + "\n\n}\n"

    out = tmp_dir / f"entailment_{hash(phi_code) & 0xFFFFFFFF:08x}.rs"
    out.write_text(modified)
    return out


async def run_pipeline_for_file(source_path: Path, semaphore: asyncio.Semaphore) -> dict:
    """Run full pipeline for one Verus source file."""
    async with semaphore:
        rel = source_path.relative_to(VERUSAGE)
        project = rel.parts[0]
        filename = source_path.stem
        print(f"[START] {rel}")

        result = {
            "file": str(rel),
            "source_path": str(source_path),
            "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "stages": {},
        }

        # Step 0: Verify original file compiles
        check = run_verus(source_path, timeout=60)
        if not check["success"]:
            result["stages"]["baseline"] = {"status": "FAIL", "output": check["output"][:500]}
            result["status"] = "SKIP_BASELINE_FAIL"
            print(f"[SKIP] {rel} — baseline verification failed")
            return result
        result["stages"]["baseline"] = {"status": "OK"}

        # Step 1: Generator (via subagent — called from main Lem session)
        # For batch mode, we write the prompt and let the caller handle LLM calls
        source_text = source_path.read_text()
        generator_prompt = f"Read and analyze this Verus file, then generate 5 candidate undesirable properties:\n\nFile: {source_path}\n\n```rust\n{source_text}\n```"

        result["stages"]["generator"] = {
            "prompt": generator_prompt[:200] + "...",
            "status": "NEEDS_LLM_CALL",
        }

        # For now, return the result with prompts — the orchestrator (Lem) will
        # handle actual LLM calls via subagents with concurrency control
        result["status"] = "READY_FOR_LLM"
        print(f"[READY] {rel}")
        return result


async def batch_baseline_check(files: list, concurrency: int = 3) -> list:
    """Check which files pass baseline Verus verification."""
    sem = asyncio.Semaphore(concurrency)
    valid = []

    async def check_one(f):
        async with sem:
            loop = asyncio.get_event_loop()
            check = await loop.run_in_executor(None, run_verus, f, 60)
            if check["success"]:
                valid.append(f)
                print(f"  ✓ {f.relative_to(VERUSAGE)}")
            elif check["timed_out"]:
                print(f"  ⏱ {f.relative_to(VERUSAGE)} (timeout)")
            else:
                print(f"  ✗ {f.relative_to(VERUSAGE)}")

    print(f"Baseline check: {len(files)} files, concurrency={concurrency}")
    tasks = [check_one(f) for f in files]
    await asyncio.gather(*tasks)
    print(f"\n{len(valid)}/{len(files)} pass baseline verification\n")
    return valid


def main():
    parser = argparse.ArgumentParser(description="Spec Consistency Pipeline")
    parser.add_argument("--concurrency", type=int, default=3,
                        help="Max concurrent Verus processes (default: 3)")
    parser.add_argument("--limit", type=int, default=None,
                        help="Max files to process")
    parser.add_argument("--offset", type=int, default=0,
                        help="Skip first N files")
    parser.add_argument("--file", type=str, default=None,
                        help="Process a single file")
    parser.add_argument("--baseline-only", action="store_true",
                        help="Only run baseline verification check")
    args = parser.parse_args()

    if args.file:
        files = [Path(args.file)]
    else:
        files = find_verusage_files(limit=args.limit, offset=args.offset)

    print(f"Found {len(files)} VeruSage files")

    if args.baseline_only:
        valid = asyncio.run(batch_baseline_check(files, args.concurrency))
        # Save valid file list
        out = RESULTS / "baseline_valid.json"
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps([str(f) for f in valid], indent=2))
        print(f"Saved {len(valid)} valid files to {out}")
        return

    # Full pipeline needs LLM orchestration — output file list for Lem to process
    valid = asyncio.run(batch_baseline_check(files, args.concurrency))

    out = RESULTS / "pipeline_queue.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps([str(f) for f in valid], indent=2))
    print(f"\nQueued {len(valid)} files for pipeline processing → {out}")
    print("Run the pipeline from Lem (OpenClaw) to handle LLM calls with concurrency control.")


if __name__ == "__main__":
    main()
