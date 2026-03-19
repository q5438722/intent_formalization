"""
Adapter for the Bitmap allocator case study.

The Bitmap project uses a split-file layout:
  - lib.rs           (implementation + include! for other files)
  - lib.spec.rs      (BitmapView + specification functions)
  - lib.proof.rs     (proof lemmas)
  - lib.test.rs      (verified test functions)

Two variants exist:
  - bitmap_raw/  — original Verus-native implementation
  - bitmap_new/  — refactored with attribute macros (#[verus_spec(...)])
"""

import os
import re
from typing import Iterator

from .base import CaseAdapter, FunctionInfo, Task

# Regex for matching Verus function signatures (simplified).
_FN_RE = re.compile(
    r"(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\(([^)]*)\)",
    re.DOTALL,
)


class BitmapAdapter(CaseAdapter):
    """Adapter for the Bitmap allocator multi-file project."""

    def __init__(
        self,
        project_dir: str,
        variant: str = "bitmap_new",
    ):
        self.project_dir = os.path.join(project_dir, variant)
        self.variant = variant
        self._validate()

    def _validate(self):
        required = ["lib.rs", "lib.spec.rs", "lib.proof.rs", "lib.test.rs"]
        for fname in required:
            path = os.path.join(self.project_dir, fname)
            if not os.path.isfile(path):
                raise FileNotFoundError(
                    f"Expected {fname} in {self.project_dir}"
                )

    def _read(self, filename: str) -> str:
        with open(os.path.join(self.project_dir, filename)) as f:
            return f.read()

    def _extract_functions(self, source: str, file_path: str) -> list[FunctionInfo]:
        """Extract function metadata from Verus source (best-effort regex)."""
        functions = []
        for m in _FN_RE.finditer(source):
            name = m.group(1)
            sig = m.group(0)
            functions.append(FunctionInfo(
                name=name,
                signature=sig,
                specs="",       # populated by tree-sitter if available
                body="",
                full_text="",
                file_path=file_path,
            ))
        return functions

    # ------------------------------------------------------------------
    # CaseAdapter interface
    # ------------------------------------------------------------------

    def iter_tasks(self) -> Iterator[Task]:
        """
        For Bitmap, each public API function is one task.
        We yield a single task whose source is the concatenated project.
        """
        lib_rs = self._read("lib.rs")
        spec_rs = self._read("lib.spec.rs")
        proof_rs = self._read("lib.proof.rs")
        test_rs = self._read("lib.test.rs")

        # The full compilable source is lib.rs (which include!s the rest).
        full_source = lib_rs

        # Extract public functions from lib.rs as potential targets.
        functions = self._extract_functions(lib_rs, "lib.rs")
        functions += self._extract_functions(spec_rs, "lib.spec.rs")

        # Yield one task per public fn in the implementation.
        for fn_info in functions:
            yield Task(
                task_id=f"bitmap_{self.variant}_{fn_info.name}",
                target_function=fn_info.name,
                source_code=full_source,
                functions=[fn_info],
                extra={
                    "variant": self.variant,
                    "spec_file": spec_rs,
                    "proof_file": proof_rs,
                    "test_file": test_rs,
                },
            )

    def build_verifiable_source(self, task: Task, test_code: str) -> str:
        """
        Inject test_code into the Bitmap project by appending it inside
        the verus! { } block of lib.test.rs, then returning the combined
        source that lib.rs would include.

        For verification, we write all split files to a temp directory and
        return lib.rs as the entry point.
        """
        test_rs = task.extra.get("test_file", "")
        # Insert new tests before the closing `} // verus!`
        closing = "} // verus!"
        if closing in test_rs:
            combined_test = test_rs.replace(
                closing,
                f"\n// === Generated Tests ===\n{test_code}\n\n{closing}",
            )
        else:
            combined_test = test_rs + f"\n\n// === Generated Tests ===\n{test_code}\n"
        return combined_test

    def build_verifiable_source_with_spec(
        self, task: Task, spec: str, test_code: str,
    ) -> str:
        # For spec replacement, we'd need tree-sitter.
        # For now, return the test-injected source as-is (spec replacement
        # is handled externally by the spec_checker pipeline).
        return self.build_verifiable_source(task, test_code)

    def write_test_file(self, task: Task, test_code: str, output_dir: str) -> str:
        """
        Write the full Bitmap project (with injected tests) to output_dir.
        Returns the path to lib.rs (the Verus entry point).
        """
        os.makedirs(output_dir, exist_ok=True)

        # Copy lib.rs, lib.spec.rs, lib.proof.rs as-is.
        for fname in ("lib.rs", "lib.spec.rs", "lib.proof.rs"):
            src = os.path.join(self.project_dir, fname)
            dst = os.path.join(output_dir, fname)
            with open(src) as f_in, open(dst, "w") as f_out:
                f_out.write(f_in.read())

        # Write lib.test.rs with injected tests.
        combined_test = self.build_verifiable_source(task, test_code)
        test_path = os.path.join(output_dir, "lib.test.rs")
        with open(test_path, "w") as f:
            f.write(combined_test)

        return os.path.join(output_dir, "lib.rs")
