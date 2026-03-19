"""
Entry point: python -m framework <subcommand>

Subcommands:
  test_gen      Generate verified test functions
  spec_checker  Evaluate spec quality using generated tests
"""

import sys


def main():
    if len(sys.argv) < 2:
        print("Usage: python -m framework <subcommand> [args...]")
        print("Subcommands: test_gen, spec_checker")
        sys.exit(1)

    subcmd = sys.argv[1]
    sys.argv = sys.argv[1:]  # shift argv so argparse sees correct args

    if subcmd == "test_gen":
        from .test_gen import main as test_gen_main
        test_gen_main()
    elif subcmd == "spec_checker":
        from .spec_checker import main as spec_checker_main
        spec_checker_main()
    else:
        print(f"Unknown subcommand: {subcmd}")
        print("Available: test_gen, spec_checker")
        sys.exit(1)


if __name__ == "__main__":
    main()
