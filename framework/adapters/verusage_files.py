"""
Adapter for file-based VeruSAGE tasks.

Walks source-projects/<project>/verified/<category>/<file>.rs to discover
tasks.  Unlike VeruSAGEAdapter (JSONL-based), this reads .rs files directly
from the directory tree.
"""

import os
import re
from typing import Iterator

from .base import CaseAdapter, Task
from .verusage import VeruSAGEAdapter


# Segments that indicate an impl block index rather than the function name.
_IMPL_PATTERN = re.compile(r"^impl\d+$")


def _extract_target_function(file_stem: str) -> str:
    """
    Derive the target function name from the file stem.

    Convention: segments are separated by `__`.  The target function is the
    last segment, *unless* it matches `impl0`/`impl1`/... in which case we
    take the segment before it as the function name and the impl segment is
    dropped.

    Examples:
        commit_mask__lemma_obtain_bit_index_1  -> lemma_obtain_bit_index_1
        kernel__create_and_map_pages__impl0__alloc_and_map  -> alloc_and_map
    """
    parts = file_stem.split("__")
    # Walk from the end and skip impl-index segments.
    for i in range(len(parts) - 1, -1, -1):
        if not _IMPL_PATTERN.match(parts[i]):
            return parts[i]
    # Fallback: return the full stem.
    return file_stem


class VeruSAGEFileAdapter(CaseAdapter):
    """Adapter for file-based VeruSAGE tasks (individual .rs files)."""

    def __init__(self, source_dir: str, start: int = 0, end: int = -1):
        """
        Parameters
        ----------
        source_dir : root of the source-projects tree
                     (e.g. verusage/source-projects).
        start, end : slice of discovered tasks to process.
        """
        self.source_dir = source_dir
        self.start = start
        self.end = end

    def _discover_files(self) -> list[dict]:
        """Walk the verified/ subtrees and return metadata dicts."""
        entries: list[dict] = []
        for project in sorted(os.listdir(self.source_dir)):
            verified_dir = os.path.join(self.source_dir, project, "verified")
            if not os.path.isdir(verified_dir):
                continue
            for root, _dirs, files in os.walk(verified_dir):
                for fname in sorted(files):
                    if not fname.endswith(".rs"):
                        continue
                    rel = os.path.relpath(root, verified_dir)
                    category = rel if rel != "." else ""
                    stem = fname[:-3]  # strip .rs
                    entries.append({
                        "project": project,
                        "category": category,
                        "file_stem": stem,
                        "source_path": os.path.join(root, fname),
                    })
        end = self.end if self.end >= 0 else len(entries)
        return entries[self.start:end]

    # ------------------------------------------------------------------
    # CaseAdapter interface
    # ------------------------------------------------------------------

    def iter_tasks(self) -> Iterator[Task]:
        for entry in self._discover_files():
            source_path = entry["source_path"]
            with open(source_path) as f:
                source_code = f.read()

            task_id = entry["file_stem"]
            target_fn = _extract_target_function(task_id)

            yield Task(
                task_id=task_id,
                target_function=target_fn,
                source_code=source_code,
                extra={
                    "project": entry["project"],
                    "category": entry["category"],
                    "source_path": source_path,
                },
            )

    def build_verifiable_source(self, task: Task, test_code: str) -> str:
        return VeruSAGEAdapter._inject_tests(task.source_code, test_code)

    def build_verifiable_source_with_spec(
        self, task: Task, spec: str, test_code: str,
    ) -> str:
        # For file-based tasks we don't yet support spec replacement;
        # fall back to injecting tests into the original source.
        return VeruSAGEAdapter._inject_tests(task.source_code, test_code)

    def write_test_file(self, task: Task, test_code: str, output_dir: str) -> str:
        os.makedirs(output_dir, exist_ok=True)
        out_path = os.path.join(output_dir, f"{task.task_id}.rs")
        source = self.build_verifiable_source(task, test_code)
        with open(out_path, "w") as f:
            f.write(source)
        return out_path
