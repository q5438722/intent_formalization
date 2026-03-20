"""
Abstract base adapter for different Verus project structures.

Each case study (Bitmap, VeruSAGE) has a different file layout and
verification invocation pattern.  Adapters normalise these differences
behind a common interface so the test-generation pipeline and the
spec-quality checker can operate uniformly.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Iterator


@dataclass
class FunctionInfo:
    """Metadata about a single Verus function extracted from source."""
    name: str
    signature: str          # full signature line(s) including generics
    specs: str              # requires/ensures/decreases/recommends block (may be empty)
    body: str               # function body (excluding specs)
    full_text: str          # entire function text as it appears in source
    file_path: str          # originating file


@dataclass
class Task:
    """A single unit of work: one function whose spec quality we evaluate."""
    task_id: str
    target_function: str
    source_code: str                        # full compilable source with specs
    source_no_spec: str = ""                # source with target fn's specs stripped
    generated_spec: str = ""                # LLM-generated spec (if available)
    ground_truth_spec: str = ""             # original spec (if available)
    functions: list[FunctionInfo] = field(default_factory=list)
    extra: dict = field(default_factory=dict)   # adapter-specific metadata


@dataclass
class StructuredTestResult:
    """Result of structured (correctness + completeness) test generation."""
    task_id: str
    target_function: str
    model: str
    correctness_tests: str = ""
    completeness_rounds: dict[int, str] = field(default_factory=dict)  # {1: code, ..., 5: code}
    error: str = ""


class CaseAdapter(ABC):
    """Interface that every case-study adapter must implement."""

    @abstractmethod
    def iter_tasks(self) -> Iterator[Task]:
        """Yield tasks one at a time (lazy, to handle large benchmarks)."""
        ...

    @abstractmethod
    def build_verifiable_source(self, task: Task, test_code: str) -> str:
        """
        Return a single compilable Verus source string that includes:
          - the original source (with specs)
          - the injected *test_code*

        The returned string must be ready to write to a .rs file and
        pass to the Verus binary.
        """
        ...

    @abstractmethod
    def build_verifiable_source_with_spec(
        self, task: Task, spec: str, test_code: str,
    ) -> str:
        """
        Same as ``build_verifiable_source`` but replaces the target
        function's spec with *spec* before injecting tests.  Used to
        evaluate LLM-generated specs against ground-truth tests.
        """
        ...

    @abstractmethod
    def write_test_file(self, task: Task, test_code: str, output_dir: str) -> str:
        """
        Persist the verifiable source (with tests injected) to disk.
        Returns the path of the written file.
        """
        ...
