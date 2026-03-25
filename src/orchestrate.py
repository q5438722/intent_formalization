"""
orchestrate.py — End-to-end pipeline runner.

Runs: generate → critic → verify → report

Usage:
    python -m src.orchestrate --spec path/to/spec.rs [--tests path/to/tests.rs] [--output results/]
"""

import argparse
import json
import logging
import os
from datetime import datetime
from pathlib import Path

from src.generate import generate_queries, load_config
from src.critic import run_critic
from src.verify import run_entailment_checks

logger = logging.getLogger(__name__)

BASE_DIR = Path(__file__).resolve().parent.parent


def run_pipeline(
    spec_path: str,
    tests_path: str | None = None,
    config_path: str = "src/config.yaml",
    output_dir: str = "results",
) -> dict:
    """
    Run the full spec consistency pipeline.
    
    Steps:
    1. Generate adversarial queries (single generator LLM)
    2. Filter via critic ensemble (multi-model debate)
    3. Check entailment via Verus
    4. Produce inconsistency report
    """
    config = load_config(config_path)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    spec_name = Path(spec_path).stem

    # Create output directory
    run_dir = Path(BASE_DIR) / output_dir / f"{spec_name}_{timestamp}"
    run_dir.mkdir(parents=True, exist_ok=True)

    logger.info(f"{'='*60}")
    logger.info(f"SPEC CONSISTENCY PIPELINE")
    logger.info(f"Spec: {spec_path}")
    logger.info(f"Tests: {tests_path or 'none'}")
    logger.info(f"Output: {run_dir}")
    logger.info(f"{'='*60}")

    # Step 1: Generate
    logger.info("\n[Step 1/3] Generating adversarial queries...")
    queries_path = str(run_dir / "01_queries.json")
    queries = generate_queries(spec_path, config=config, output_path=queries_path)
    logger.info(f"Generated {len(queries)} candidate queries")

    # Step 2: Critic
    logger.info("\n[Step 2/3] Running critic ensemble...")
    filtered_path = str(run_dir / "02_filtered.json")
    valid_queries = run_critic(
        queries_path, spec_path,
        tests_path=tests_path,
        config_path=config_path,
        output_path=filtered_path,
    )
    logger.info(f"Critic retained {len(valid_queries)} / {len(queries)} queries")

    # Step 3: Verify
    logger.info("\n[Step 3/3] Running Verus entailment checks...")
    report_path = str(run_dir / "03_report.json")
    report = run_entailment_checks(
        filtered_path, spec_path,
        config_path=config_path,
        output_path=report_path,
    )

    # Summary
    summary = {
        "spec_path": spec_path,
        "tests_path": tests_path,
        "timestamp": timestamp,
        "pipeline": {
            "generated": len(queries),
            "after_critic": len(valid_queries),
            "inconsistencies": report["inconsistencies_found"],
            "correct_rejections": report["correctly_rejected"],
            "errors": report["errors"],
        },
        "outputs": {
            "queries": queries_path,
            "filtered": filtered_path,
            "report": report_path,
        },
    }

    summary_path = str(run_dir / "summary.json")
    with open(summary_path, "w") as f:
        json.dump(summary, f, indent=2)

    logger.info(f"\n{'='*60}")
    logger.info(f"PIPELINE COMPLETE")
    logger.info(f"{'='*60}")
    logger.info(f"Generated:       {len(queries)} queries")
    logger.info(f"After critic:    {len(valid_queries)} queries")
    logger.info(f"Inconsistencies: {report['inconsistencies_found']}")
    logger.info(f"Results:         {run_dir}")
    logger.info(f"{'='*60}")

    return summary


def main():
    parser = argparse.ArgumentParser(description="Run full spec consistency pipeline")
    parser.add_argument("--spec", required=True, help="Path to Verus spec file")
    parser.add_argument("--tests", default=None, help="Path to developer test file")
    parser.add_argument("--config", default="src/config.yaml")
    parser.add_argument("--output", default="results", help="Output directory")
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
    run_pipeline(args.spec, args.tests, args.config, args.output)


if __name__ == "__main__":
    main()
