#!/home/chentianyu/miniconda3/bin/python3
"""
Run the full pipeline: step1 → step2 → step3 → step4.

Usage:
  python3 run_pipeline_v3.py [--limit N] [--offset N] [--model MODEL]
  python3 run_pipeline_v3.py --step 2  # Run only step 2 onwards
"""

import argparse
import subprocess
import sys
from pathlib import Path

PIPELINE = Path(__file__).parent.parent / "src" / "pipeline"
PYTHON = sys.executable


def run_step(name: str, script: str, extra_args: list = []):
    print(f"\n{'='*60}")
    print(f"  {name}")
    print(f"{'='*60}\n")
    cmd = [PYTHON, str(PIPELINE / script)] + extra_args
    result = subprocess.run(cmd)
    if result.returncode != 0:
        print(f"\n⚠️  {name} exited with code {result.returncode}")
        return False
    return True


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--step", type=int, default=1, help="Start from step N")
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--model", type=str, default="claude-opus-4.6")
    parser.add_argument("--verus-timeout", type=int, default=120)
    args = parser.parse_args()

    extra = []
    if args.limit:
        extra += ["--limit", str(args.limit)]
    if args.offset:
        extra += ["--offset", str(args.offset)]

    steps = [
        (1, "Step 1: Extract exec functions", "step1_extract.py", []),
        (2, "Step 2: Generate candidates", "step2_generate.py", extra + ["--model", args.model]),
        (3, "Step 3: Entailment check", "step3_entailment.py", extra + ["--timeout", str(args.verus_timeout)]),
        (4, "Step 4: Critic", "step4_critic.py", extra + ["--model", args.model]),
    ]

    for step_num, name, script, step_args in steps:
        if step_num < args.step:
            continue
        if not run_step(name, script, step_args):
            print(f"\nPipeline stopped at {name}")
            sys.exit(1)

    print(f"\n{'='*60}")
    print("  Pipeline complete! ✅")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
