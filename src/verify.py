"""
verify.py — Run Verus entailment checks on filtered adversarial queries.

For each query φ that passed the critic, check: S ⊢ φ ?
If Verus verifies (S ⊢ φ), the spec has an unintended entailment → inconsistency.

Usage:
    python -m src.verify --queries filtered.json --spec path/to/spec.rs [--output report.json]
"""

import argparse
import json
import logging
import yaml
import tempfile
import os
from pathlib import Path

from src.utils.verus import run_verus, find_verus_binary, check_entailment

logger = logging.getLogger(__name__)

BASE_DIR = Path(__file__).resolve().parent.parent


def load_config(path: str = "src/config.yaml") -> dict:
    with open(BASE_DIR / path) as f:
        return yaml.safe_load(f)


def run_entailment_checks(
    queries_path: str,
    spec_path: str,
    config_path: str = "src/config.yaml",
    output_path: str | None = None,
) -> dict:
    """
    Run Verus entailment checks on all filtered queries.
    
    Returns a report with:
    - inconsistencies: queries where S ⊢ φ (bad — spec entails undesirable property)
    - rejected: queries where S ⊬ φ (good — spec correctly rejects)
    - errors: queries that caused Verus errors/timeouts
    """
    config = load_config(config_path)
    verus_config = config["verus"]
    verus_binary = find_verus_binary(verus_config.get("binary", ""))
    timeout = verus_config.get("timeout_seconds", 60)

    with open(queries_path) as f:
        data = json.load(f)

    spec_code = Path(spec_path).read_text()
    queries = data.get("valid_queries", data.get("queries", data))

    logger.info(f"Running Verus entailment checks on {len(queries)} queries")

    inconsistencies = []
    correct_rejections = []
    errors = []

    for i, query in enumerate(queries):
        name = query.get("name", f"query_{i}")
        logger.info(f"[{i+1}/{len(queries)}] Checking: {name}")

        try:
            result = check_entailment(
                spec_code, query["code"],
                verus_binary=verus_binary,
                timeout=timeout,
            )

            entry = {
                "query": query,
                "entailed": result["entailed"],
                "verified_count": result["result"].verified,
                "error_count": result["result"].errors,
                "output_snippet": result["result"].output[:500],
            }

            if result["entailed"]:
                # BAD: spec entails undesirable property
                logger.warning(f"  ⚠️  INCONSISTENCY: {name} — spec entails this query!")
                inconsistencies.append(entry)
            else:
                # GOOD: spec correctly rejects
                logger.info(f"  ✓ Correctly rejected: {name}")
                correct_rejections.append(entry)

        except Exception as e:
            logger.error(f"  ✗ Error checking {name}: {e}")
            errors.append({"query": query, "error": str(e)})

    report = {
        "spec_path": spec_path,
        "total_checked": len(queries),
        "inconsistencies_found": len(inconsistencies),
        "correctly_rejected": len(correct_rejections),
        "errors": len(errors),
        "inconsistencies": inconsistencies,
        "correct_rejections": correct_rejections,
        "error_details": errors,
    }

    logger.info(
        f"\n{'='*60}\n"
        f"ENTAILMENT CHECK REPORT\n"
        f"{'='*60}\n"
        f"Total queries checked: {len(queries)}\n"
        f"Inconsistencies found: {len(inconsistencies)}\n"
        f"Correctly rejected:    {len(correct_rejections)}\n"
        f"Errors/timeouts:       {len(errors)}\n"
        f"{'='*60}"
    )

    if output_path:
        Path(output_path).parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            json.dump(report, f, indent=2)
        logger.info(f"Report saved to {output_path}")

    return report


def main():
    parser = argparse.ArgumentParser(description="Run Verus entailment checks")
    parser.add_argument("--queries", required=True, help="Path to filtered queries JSON")
    parser.add_argument("--spec", required=True, help="Path to Verus spec file")
    parser.add_argument("--config", default="src/config.yaml")
    parser.add_argument("--output", default=None, help="Output report JSON path")
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)
    run_entailment_checks(args.queries, args.spec, args.config, args.output)


if __name__ == "__main__":
    main()
