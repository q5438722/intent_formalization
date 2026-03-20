"""
Evaluate pre-generated workspace results (correctness + completeness rounds).

Each workspace task directory contains:
  - correctness_tests.rs   (should verify successfully)
  - completeness_round1.rs (should produce verification errors)
  - completeness_round2.rs
  - completeness_round3.rs
  - completeness_round4.rs
  - completeness_round5.rs
  - summary.md             (human-readable, ignored by evaluator)
"""

import json
import os
from dataclasses import dataclass, field

from .verus_runner import run_verus, VerificationResult


@dataclass
class WorkspaceTaskReport:
    """Evaluation report for one workspace task."""
    task_id: str
    correctness: VerificationResult | None = None
    completeness: dict[int, VerificationResult] = field(default_factory=dict)

    @property
    def correctness_ok(self) -> bool:
        """Correctness tests should PASS (verify successfully)."""
        return self.correctness is not None and self.correctness.success

    def completeness_ok(self, round_num: int) -> bool:
        """Completeness tests should FAIL (have verification errors)."""
        r = self.completeness.get(round_num)
        if r is None:
            return False
        # "ok" means the test correctly failed: errors > 0 or parse error
        return r.errors > 0 or r.parse_error

    @property
    def all_completeness_ok(self) -> bool:
        return all(self.completeness_ok(i) for i in range(1, 6)
                   if i in self.completeness)

    def to_dict(self) -> dict:
        def _vr(v: VerificationResult | None) -> dict | None:
            if v is None:
                return None
            return {
                "verified": v.verified,
                "errors": v.errors,
                "success": v.success,
                "timed_out": v.timed_out,
                "parse_error": v.parse_error,
                "summary": v.summary,
            }

        return {
            "task_id": self.task_id,
            "correctness": _vr(self.correctness),
            "correctness_ok": self.correctness_ok,
            "completeness": {
                str(k): _vr(v) for k, v in sorted(self.completeness.items())
            },
            "completeness_ok": {
                str(i): self.completeness_ok(i)
                for i in range(1, 6) if i in self.completeness
            },
            "all_completeness_ok": self.all_completeness_ok,
        }


def evaluate_workspace(
    workspace_dir: str,
    verus_bin: str,
    output_dir: str,
    timeout: int = 300,
) -> list[WorkspaceTaskReport]:
    """
    Evaluate all task directories under workspace_dir.

    Returns a list of WorkspaceTaskReport and writes JSONL output.
    """
    os.makedirs(output_dir, exist_ok=True)
    results_path = os.path.join(output_dir, "workspace_eval_results.jsonl")

    task_dirs = sorted(
        d for d in os.listdir(workspace_dir)
        if os.path.isdir(os.path.join(workspace_dir, d))
    )

    print(f"Found {len(task_dirs)} workspace tasks")
    reports: list[WorkspaceTaskReport] = []

    with open(results_path, "w") as out_f:
        for idx, task_id in enumerate(task_dirs):
            task_path = os.path.join(workspace_dir, task_id)
            report = WorkspaceTaskReport(task_id=task_id)

            # Correctness: expect success
            correctness_file = os.path.join(task_path, "correctness_tests.rs")
            if os.path.exists(correctness_file):
                report.correctness = run_verus(
                    correctness_file, verus_bin=verus_bin, timeout=timeout,
                )

            # Completeness rounds: expect errors
            for round_num in range(1, 6):
                round_file = os.path.join(
                    task_path, f"completeness_round{round_num}.rs",
                )
                if os.path.exists(round_file):
                    report.completeness[round_num] = run_verus(
                        round_file, verus_bin=verus_bin, timeout=timeout,
                    )

            reports.append(report)
            out_f.write(json.dumps(report.to_dict(), ensure_ascii=False) + "\n")
            out_f.flush()

            status = "OK" if report.correctness_ok and report.all_completeness_ok else "ISSUES"
            if (idx + 1) % 10 == 0 or idx + 1 == len(task_dirs):
                print(f"  [{idx+1}/{len(task_dirs)}] {task_id}: {status}")

    # Print summary
    correct_ok = sum(1 for r in reports if r.correctness_ok)
    all_ok = sum(1 for r in reports if r.correctness_ok and r.all_completeness_ok)
    print(f"\nSummary: {correct_ok}/{len(reports)} correctness pass, "
          f"{all_ok}/{len(reports)} fully pass")
    print(f"Results -> {results_path}")

    return reports
