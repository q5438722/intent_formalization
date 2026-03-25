"""
generate.py — Adversarial test generation using Copilot CLI.

Passes the spec file path to copilot (which reads it via its own file tools)
rather than embedding the full spec in the prompt.

Usage:
    python -m src.generate --spec path/to/spec.rs [--config src/config.yaml] [--output queries.json]
"""

import argparse
import json
import logging
import re
import subprocess
import yaml
from pathlib import Path

from src.utils.llm import LLMClient

logger = logging.getLogger(__name__)

BASE_DIR = Path(__file__).resolve().parent.parent


def load_config(path: str = "src/config.yaml") -> dict:
    with open(BASE_DIR / path) as f:
        return yaml.safe_load(f)


def load_prompt_template(config: dict) -> str:
    prompt_path = BASE_DIR / config["prompts"]["test_gen"]
    return prompt_path.read_text()


def load_system_prompt(config: dict) -> str:
    parts = []
    for key in ["system_role", "reasoning_style"]:
        path = BASE_DIR / config["prompts"][key]
        if path.exists():
            parts.append(path.read_text().strip())
    return "\n\n".join(parts)


def parse_generated_tests(raw_output: str) -> list[dict]:
    """Parse LLM output into structured test cases."""
    tests = []

    category_patterns = {
        "boundary": re.compile(r"BOUNDARY\s+CONSISTENCY", re.IGNORECASE),
        "behavioral": re.compile(r"BEHAVIORAL\s+CONSISTENCY", re.IGNORECASE),
        "logical": re.compile(r"LOGICAL\s+CONSISTENCY", re.IGNORECASE),
    }

    lines = raw_output.split("\n")

    # Find category boundaries across all lines
    category_ranges = []
    for i, line in enumerate(lines):
        for cat, pat in category_patterns.items():
            if pat.search(line):
                category_ranges.append((i, cat))

    # Extract code blocks (```rust ... ```)
    code_blocks = []
    in_block = False
    block_start = 0
    block_lines = []
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("```rust") or (stripped == "```" and in_block):
            if in_block:
                code_blocks.append((block_start, i, "\n".join(block_lines)))
                block_lines = []
                in_block = False
            else:
                in_block = True
                block_start = i
                block_lines = []
        elif in_block:
            block_lines.append(line)

    if not code_blocks:
        code_blocks = [(0, len(lines), raw_output)]

    for block_start_line, _, code in code_blocks:
        # Build category ranges within this code block
        block_cat_ranges = []
        for ci, cline in enumerate(code.split("\n")):
            for cat_name, cat_pat in category_patterns.items():
                if cat_pat.search(cline):
                    block_cat_ranges.append((ci, cat_name))

        # Default category from global ranges
        default_cat = "unknown"
        for range_line, range_cat in reversed(category_ranges):
            if range_line <= block_start_line:
                default_cat = range_cat
                break

        # Extract individual proof fns
        fns = list(re.finditer(
            r"((?:\s*//[^\n]*\n)*)\s*proof\s+fn\s+(\w+)\s*\(([^)]*)\)\s*\{",
            code,
        ))

        if not fns:
            tests.append({
                "name": f"test_{default_cat}_{len(tests)}",
                "category": default_cat,
                "code": code.strip(),
                "should_fail": "SHOULD FAIL" in code,
            })
            continue

        for idx, match in enumerate(fns):
            fn_name = match.group(2)
            start = match.start()
            end = fns[idx + 1].start() if idx + 1 < len(fns) else len(code)
            fn_code = code[start:end].strip()

            # Determine category by position within the code block
            fn_line = code[:start].count("\n")
            cat = default_cat
            for range_line, range_cat in reversed(block_cat_ranges):
                if range_line <= fn_line:
                    cat = range_cat
                    break

            tests.append({
                "name": fn_name,
                "category": cat,
                "code": fn_code,
                "should_fail": "SHOULD FAIL" in fn_code,
            })

    return tests


def run_copilot_with_file(
    prompt: str,
    spec_path: str,
    model: str = "claude-opus-4.6",
    timeout: int = 600,
    output_dir: str | None = None,
) -> str:
    """
    Run copilot with a prompt that references a file path.
    Copilot reads the file itself via its built-in file tools.
    """
    full_prompt = f"{prompt}\n\nThe target file is {spec_path}"

    cmd = ["copilot", "-p", full_prompt, "--model", model, "-s", "--allow-all"]
    if output_dir:
        cmd.extend(["--add-dir", output_dir])

    logger.info(f"Copilot request: model={model}, spec={spec_path}")

    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=timeout,
    )

    if result.returncode != 0:
        raise RuntimeError(f"Copilot exited with code {result.returncode}: {result.stderr[:500]}")

    return result.stdout.strip()


def generate_queries(
    spec_path: str,
    config: dict | None = None,
    output_path: str | None = None,
) -> list[dict]:
    """Generate adversarial queries for a given spec file."""
    if config is None:
        config = load_config()

    system_prompt = load_system_prompt(config)
    test_gen_prompt = load_prompt_template(config)
    gen_config = config["generator"]
    num_queries = config["generation"]["num_queries"]

    prompt = (
        f"{system_prompt}\n\n"
        f"{test_gen_prompt}\n\n"
        f"Generate at least {num_queries} adversarial queries across all three categories."
    )

    # Resolve spec path to absolute
    spec_abs = str(Path(spec_path).resolve())

    logger.info(f"Generating queries for {spec_abs} using {gen_config['model']}")
    raw_output = run_copilot_with_file(
        prompt, spec_abs,
        model=gen_config["model"],
        output_dir=str(Path(output_path).parent) if output_path else None,
    )

    tests = parse_generated_tests(raw_output)
    logger.info(f"Generated {len(tests)} queries: "
                f"{sum(1 for t in tests if t['category'] == 'boundary')} boundary, "
                f"{sum(1 for t in tests if t['category'] == 'behavioral')} behavioral, "
                f"{sum(1 for t in tests if t['category'] == 'logical')} logical")

    result = {
        "spec_path": spec_path,
        "generator_model": gen_config["model"],
        "queries": tests,
        "raw_output": raw_output,
    }

    if output_path:
        Path(output_path).parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            json.dump(result, f, indent=2)
        logger.info(f"Saved queries to {output_path}")

    return tests


def main():
    parser = argparse.ArgumentParser(description="Generate adversarial queries for spec consistency")
    parser.add_argument("--spec", required=True, help="Path to Verus spec file")
    parser.add_argument("--config", default="src/config.yaml", help="Config file path")
    parser.add_argument("--output", default=None, help="Output JSON path")
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)
    config = load_config(args.config)
    generate_queries(args.spec, config=config, output_path=args.output)


if __name__ == "__main__":
    main()
