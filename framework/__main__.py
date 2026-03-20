"""
Entry point: python -m framework <subcommand>

Subcommands:
  test_gen        Generate verified test functions
  spec_checker    Evaluate spec quality using generated tests
  workspace_eval  Evaluate pre-generated workspace results
"""

import argparse
import sys


def main():
    if len(sys.argv) < 2:
        print("Usage: python -m framework <subcommand> [args...]")
        print("Subcommands: test_gen, spec_checker, workspace_eval")
        sys.exit(1)

    subcmd = sys.argv[1]
    sys.argv = sys.argv[1:]  # shift argv so argparse sees correct args

    if subcmd == "test_gen":
        from .test_gen import main as test_gen_main
        test_gen_main()
    elif subcmd == "spec_checker":
        from .spec_checker import main as spec_checker_main
        spec_checker_main()
    elif subcmd == "workspace_eval":
        _workspace_eval_main()
    else:
        print(f"Unknown subcommand: {subcmd}")
        print("Available: test_gen, spec_checker, workspace_eval")
        sys.exit(1)


def _workspace_eval_main():
    from .workspace_evaluator import evaluate_workspace

    p = argparse.ArgumentParser(
        description="Evaluate pre-generated workspace results (correctness + completeness).")
    p.add_argument("--workspace_dir", type=str, required=True,
                   help="Path to workspace directory with per-task folders")
    p.add_argument("--verus_bin", type=str, default=None,
                   help="Path to Verus binary (defaults to verusage/verus/verus)")
    p.add_argument("--output_dir", type=str, required=True,
                   help="Output directory for evaluation results")
    p.add_argument("--timeout", type=int, default=300,
                   help="Per-file verification timeout in seconds")
    args = p.parse_args()

    verus_bin = args.verus_bin
    if verus_bin is None:
        import os
        verus_bin = os.path.join(
            os.path.dirname(__file__), "..", "verusage", "verus", "verus",
        )

    evaluate_workspace(
        workspace_dir=args.workspace_dir,
        verus_bin=verus_bin,
        output_dir=args.output_dir,
        timeout=args.timeout,
    )


if __name__ == "__main__":
    main()
