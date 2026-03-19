"""
Adapter for the VeruSAGE benchmark.

VeruSAGE-Bench is a JSONL file where each line is a task dict with:
  - task_id           : unique identifier
  - target_function   : name of the function under evaluation
  - task              : full Verus source code (single .rs file)
  - (other fields vary by benchmark version)

This adapter handles single-file tasks: tests are injected directly
into the source's verus! { } block.
"""

import json
import os
import re
import sys
from typing import Iterator

from .base import CaseAdapter, FunctionInfo, Task


class VeruSAGEAdapter(CaseAdapter):
    """Adapter for VeruSAGE benchmark tasks (single-file .rs)."""

    def __init__(
        self,
        tasks_jsonl: str,
        language_path: str | None = None,
        start: int = 0,
        end: int = -1,
    ):
        """
        Parameters
        ----------
        tasks_jsonl : path to the VeruSAGE-Bench tasks.jsonl file.
        language_path : path to the tree-sitter verus .so for spec stripping.
        start, end : slice of tasks to process (end=-1 means all).
        """
        self.tasks_jsonl = tasks_jsonl
        self.language_path = language_path
        self.start = start
        self.end = end

    def _load_tasks(self) -> list[dict]:
        tasks = []
        with open(self.tasks_jsonl) as f:
            for line in f:
                line = line.strip()
                if line:
                    tasks.append(json.loads(line))
        end = self.end if self.end >= 0 else len(tasks)
        return tasks[self.start:end]

    @staticmethod
    def _find_verus_block_end(source: str) -> int | None:
        """
        Find the position of the closing `} // verus!` (or just the last
        top-level `}` of a `verus! {` block).  Returns the byte offset
        of the closing brace, or None.
        """
        # Simple heuristic: find the last `} // verus!` pattern.
        m = None
        for m in re.finditer(r"}\s*//\s*verus!", source):
            pass
        if m:
            return m.start()
        # Fallback: find the last `}` in the file.
        idx = source.rfind("}")
        return idx if idx >= 0 else None

    @staticmethod
    def _inject_tests(source: str, test_code: str) -> str:
        """Insert test_code inside the verus! { } block, before closing."""
        insert_pos = VeruSAGEAdapter._find_verus_block_end(source)
        if insert_pos is not None:
            return (
                source[:insert_pos]
                + f"\n// === Generated Tests ===\n{test_code}\n\n"
                + source[insert_pos:]
            )
        # If we can't find the block, append at end (best effort).
        return source + f"\n\nverus! {{\n// === Generated Tests ===\n{test_code}\n}} // verus!\n"

    # ------------------------------------------------------------------
    # CaseAdapter interface
    # ------------------------------------------------------------------

    def iter_tasks(self) -> Iterator[Task]:
        for raw in self._load_tasks():
            task_code = raw["task"]
            target_fn = raw["target_function"]

            # Optionally strip specs using tree-sitter (if available).
            source_no_spec = ""
            if self.language_path:
                try:
                    # Import lazily — the verus_parser may not be on sys.path
                    # in all environments.
                    bitmap_dir = os.path.join(
                        os.path.dirname(__file__), "..", "..", "bitmap",
                    )
                    sys.path.insert(0, os.path.join(bitmap_dir, "..", "static"))
                    sys.path.insert(0, bitmap_dir)
                    from verusage import remove_target_fn_spec
                    source_no_spec = remove_target_fn_spec(
                        task_code, target_fn, self.language_path,
                    )
                except Exception:
                    source_no_spec = ""

            yield Task(
                task_id=raw["task_id"],
                target_function=target_fn,
                source_code=task_code,
                source_no_spec=source_no_spec,
                extra={k: v for k, v in raw.items()
                       if k not in ("task_id", "target_function", "task")},
            )

    def build_verifiable_source(self, task: Task, test_code: str) -> str:
        return self._inject_tests(task.source_code, test_code)

    def build_verifiable_source_with_spec(
        self, task: Task, spec: str, test_code: str,
    ) -> str:
        """
        Replace the target function's spec with *spec*, then inject tests.

        This requires the tree-sitter verus parser.  If unavailable, falls
        back to injecting tests into the original source.
        """
        base_source = task.source_code
        if self.language_path and spec:
            try:
                bitmap_dir = os.path.join(
                    os.path.dirname(__file__), "..", "..", "bitmap",
                )
                sys.path.insert(0, os.path.join(bitmap_dir, "..", "static"))
                sys.path.insert(0, bitmap_dir)
                from verusage import remove_target_fn_spec
                # Strip old spec, then we'd need to insert the new one.
                # For now, spec replacement is a TODO that requires
                # tree-sitter insertion (not just removal).
                stripped = remove_target_fn_spec(
                    base_source, task.target_function, self.language_path,
                )
                # Insert new spec — placeholder for tree-sitter-based insertion.
                # A simple regex approach: find the fn signature and insert spec before `{`.
                fn_pattern = re.compile(
                    rf"((?:pub\s+)?fn\s+{re.escape(task.target_function)}\s*"
                    r"(?:<[^>]*>)?\s*\([^)]*\)(?:\s*->\s*[^\n{{]+)?)\s*\{{",
                    re.DOTALL,
                )
                m = fn_pattern.search(stripped)
                if m:
                    base_source = (
                        stripped[:m.end(1)]
                        + f"\n    {spec}\n"
                        + stripped[m.end(1):]
                    )
                else:
                    base_source = stripped
            except Exception:
                pass

        return self._inject_tests(base_source, test_code)

    def write_test_file(self, task: Task, test_code: str, output_dir: str) -> str:
        os.makedirs(output_dir, exist_ok=True)
        out_path = os.path.join(output_dir, f"{task.task_id}.rs")
        source = self.build_verifiable_source(task, test_code)
        with open(out_path, "w") as f:
            f.write(source)
        return out_path
