"""
Spec quality assessment via generated tests.

Given a set of generated test functions and specs (ground-truth and/or
LLM-generated), this module evaluates spec quality by running the Verus
verifier and comparing results:

  1. **Soundness check** — Do tests verify against ground-truth specs?
     Tests that don't verify against correct specs are invalid.
  2. **Discriminative power** — Do tests catch bad specs?
     Good tests should FAIL when specs are wrong/incomplete.
  3. **Quality score** — For each LLM-generated spec:
     - Run valid tests against it
     - verification rate = (tests verified) / (total valid tests)
     - A high rate suggests the generated spec is correct; low suggests flaws.

Usage:
    python -m framework.spec_checker \
        --case verusage \
        --tasks_jsonl /path/to/tasks.jsonl \
        --test_results /path/to/test_gen_results.jsonl \
        --spec_results /path/to/spec_gen_results.jsonl \
        --output_dir /path/to/eval/ \
        [--language_path /path/to/verus.so] \
        [--max_workers 4]

    python -m framework.spec_checker \
        --case bitmap \
        --project_dir ./bitmap \
        --test_results /path/to/test_gen_results.jsonl \
        --output_dir /path/to/eval/
"""

import argparse
import json
import os
import traceback
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass, field

from .adapters import BitmapAdapter, VeruSAGEAdapter, CaseAdapter
from .adapters.base import Task
from .verus_runner import run_verus_on_source, VerificationResult


@dataclass
class TestEvaluation:
    """Evaluation of one test against one spec variant."""
    task_id: str
    target_function: str
    test_code: str
    spec_variant: str           # "ground_truth" or model name
    result: VerificationResult | None = None

    @property
    def passed(self) -> bool:
        return self.result is not None and self.result.success


@dataclass
class SpecQualityReport:
    """Quality report for one task's generated spec."""
    task_id: str
    target_function: str
    spec_variant: str
    total_tests: int = 0
    valid_tests: int = 0        # tests that verify with ground-truth spec
    passed_tests: int = 0       # valid tests that also verify with this spec
    failed_tests: int = 0       # valid tests that fail with this spec
    quality_score: float = 0.0  # passed / valid (1.0 = perfect match)
    details: list[dict] = field(default_factory=list)

    def compute_score(self):
        if self.valid_tests > 0:
            self.quality_score = self.passed_tests / self.valid_tests
        else:
            self.quality_score = 0.0


# ---------------------------------------------------------------------------
# Core evaluation
# ---------------------------------------------------------------------------


def validate_tests_against_ground_truth(
    task: Task,
    adapter: CaseAdapter,
    test_code: str,
    tmp_dir: str,
    timeout: int = 300,
) -> VerificationResult:
    """
    Check that test_code verifies when combined with the ground-truth spec.
    """
    source = adapter.build_verifiable_source(task, test_code)
    return run_verus_on_source(
        source, tmp_dir,
        filename=f"{task.task_id}_gt.rs",
        timeout=timeout,
    )


def evaluate_spec_with_tests(
    task: Task,
    adapter: CaseAdapter,
    spec: str,
    test_code: str,
    tmp_dir: str,
    timeout: int = 300,
) -> VerificationResult:
    """
    Check that test_code verifies when the target function's spec is
    replaced with *spec*.
    """
    source = adapter.build_verifiable_source_with_spec(task, spec, test_code)
    return run_verus_on_source(
        source, tmp_dir,
        filename=f"{task.task_id}_eval.rs",
        timeout=timeout,
    )


def evaluate_task(
    task: Task,
    adapter: CaseAdapter,
    test_codes: list[str],
    generated_specs: dict[str, str],   # model_name -> spec
    tmp_dir: str,
    timeout: int = 300,
) -> list[SpecQualityReport]:
    """
    Full evaluation for one task:
      1. Validate each test against ground-truth spec.
      2. Run valid tests against each generated spec.
      3. Produce a SpecQualityReport per spec variant.
    """
    task_tmp = os.path.join(tmp_dir, task.task_id)
    os.makedirs(task_tmp, exist_ok=True)

    # Step 1: validate tests against ground truth.
    valid_tests: list[str] = []
    for i, tc in enumerate(test_codes):
        gt_result = validate_tests_against_ground_truth(
            task, adapter, tc, task_tmp, timeout,
        )
        if gt_result.success:
            valid_tests.append(tc)

    # Step 2: evaluate each generated spec.
    reports = []
    for model_name, spec in generated_specs.items():
        report = SpecQualityReport(
            task_id=task.task_id,
            target_function=task.target_function,
            spec_variant=model_name,
            total_tests=len(test_codes),
            valid_tests=len(valid_tests),
        )

        for tc in valid_tests:
            eval_result = evaluate_spec_with_tests(
                task, adapter, spec, tc, task_tmp, timeout,
            )
            detail = {
                "test_snippet": tc[:200],
                "passed": eval_result.success,
                "summary": eval_result.summary,
            }
            report.details.append(detail)
            if eval_result.success:
                report.passed_tests += 1
            else:
                report.failed_tests += 1

        report.compute_score()
        reports.append(report)

    return reports


# ---------------------------------------------------------------------------
# Pipeline
# ---------------------------------------------------------------------------


def _load_test_results(path: str) -> dict[str, list[str]]:
    """Load test generation results. Returns {task_id: [test_code, ...]}."""
    results: dict[str, list[str]] = {}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            tid = rec["task_id"]
            tc = rec.get("test_code", "")
            if tc:
                results.setdefault(tid, []).append(tc)
    return results


def _load_spec_results(path: str) -> dict[str, dict[str, str]]:
    """Load spec generation results. Returns {task_id: {model: spec_code}}."""
    results: dict[str, dict[str, str]] = {}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            tid = rec["task_id"]
            model = rec.get("model", "unknown")
            code = rec.get("generated_file", "")
            if code:
                results.setdefault(tid, {})[model] = code
    return results


def run_pipeline(args: argparse.Namespace) -> None:
    # Build adapter.
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

    # Load generated tests and specs.
    test_results = _load_test_results(args.test_results)
    spec_results = {}
    if args.spec_results:
        spec_results = _load_spec_results(args.spec_results)

    tasks = list(adapter.iter_tasks())
    print(f"Loaded {len(tasks)} tasks, "
          f"{len(test_results)} with tests, "
          f"{len(spec_results)} with generated specs")

    tmp_dir = os.path.join(args.output_dir, "_tmp")
    os.makedirs(tmp_dir, exist_ok=True)
    report_path = os.path.join(args.output_dir, "spec_quality_report.jsonl")

    total_evaluated = 0
    with open(report_path, "w") as out_f:
        for task in tasks:
            tests = test_results.get(task.task_id, [])
            specs = spec_results.get(task.task_id, {})
            if not tests:
                continue

            # If no generated specs, just validate tests against ground truth.
            if not specs:
                specs = {"ground_truth_only": ""}

            try:
                reports = evaluate_task(
                    task, adapter, tests, specs, tmp_dir, args.timeout,
                )
                for report in reports:
                    out_f.write(json.dumps({
                        "task_id": report.task_id,
                        "target_function": report.target_function,
                        "spec_variant": report.spec_variant,
                        "total_tests": report.total_tests,
                        "valid_tests": report.valid_tests,
                        "passed_tests": report.passed_tests,
                        "failed_tests": report.failed_tests,
                        "quality_score": report.quality_score,
                        "details": report.details,
                    }, ensure_ascii=False) + "\n")
                    out_f.flush()
                total_evaluated += 1
            except Exception:
                print(f"  ERROR on {task.task_id}: {traceback.format_exc()}")

            if total_evaluated % 10 == 0:
                print(f"  Evaluated {total_evaluated}/{len(tasks)} tasks")

    print(f"\nDone. Evaluated {total_evaluated} tasks.")
    print(f"Report → {report_path}")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main():
    p = argparse.ArgumentParser(
        description="Evaluate spec quality using generated Verus tests.")

    p.add_argument("--case", type=str, required=True,
                   choices=["bitmap", "verusage"])

    # Bitmap.
    p.add_argument("--project_dir", type=str, default="./bitmap")
    p.add_argument("--variant", type=str, default="bitmap_new")

    # VeruSAGE.
    p.add_argument("--tasks_jsonl", type=str, default=None)
    p.add_argument("--language_path", type=str, default=None)

    # Inputs.
    p.add_argument("--test_results", type=str, required=True,
                   help="Path to test_gen_results.jsonl")
    p.add_argument("--spec_results", type=str, default=None,
                   help="Path to spec_gen results.jsonl (optional)")

    # Common.
    p.add_argument("--output_dir", type=str, required=True)
    p.add_argument("--timeout", type=int, default=300)
    p.add_argument("--start", type=int, default=0)
    p.add_argument("--end", type=int, default=-1)

    args = p.parse_args()
    run_pipeline(args)


if __name__ == "__main__":
    main()
