"""
Verus verifier wrapper.

Runs the Verus binary on a .rs file and parses the output to determine
verification success/failure.
"""

import os
import re
import subprocess
from dataclasses import dataclass


VERUS_BIN = os.environ.get(
    "VERUS_BIN",
    os.path.join(
        os.path.dirname(__file__), "..", "bitmap",
        "verus", "source", "target-verus", "release", "verus",
    ),
)

_RESULT_RE = re.compile(
    r"verification results::\s*(\d+)\s+verified,\s*(\d+)\s+errors?"
)


@dataclass
class VerificationResult:
    """Parsed output from a single Verus invocation."""
    verified: int = 0
    errors: int = 0
    raw_output: str = ""
    raw_stderr: str = ""
    returncode: int = -1
    timed_out: bool = False
    parse_error: bool = False       # True if Verus produced no results line

    @property
    def success(self) -> bool:
        """All obligations verified, zero errors, no parse failure."""
        return self.verified > 0 and self.errors == 0 and not self.parse_error

    @property
    def summary(self) -> str:
        if self.timed_out:
            return "TIMEOUT"
        if self.parse_error:
            return f"PARSE_ERROR (rc={self.returncode})"
        return f"{self.verified} verified, {self.errors} errors"


def run_verus(
    file_path: str,
    verus_bin: str = VERUS_BIN,
    extra_args: list[str] | None = None,
    timeout: int = 300,
    cwd: str | None = None,
) -> VerificationResult:
    """
    Run the Verus verifier on *file_path* and return parsed results.

    Parameters
    ----------
    file_path : absolute path to the .rs file to verify.
    verus_bin : path to the verus binary.
    extra_args : additional CLI flags forwarded to Verus.
    timeout : seconds before killing the process.
    cwd : working directory for the subprocess (defaults to file's parent).
    """
    cmd = [os.path.abspath(verus_bin), os.path.abspath(file_path)]
    if extra_args:
        cmd.extend(extra_args)

    if cwd is None:
        cwd = os.path.dirname(os.path.abspath(file_path))

    result = VerificationResult()

    try:
        proc = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=cwd,
        )
        result.returncode = proc.returncode
        result.raw_output = proc.stdout
        result.raw_stderr = proc.stderr
    except subprocess.TimeoutExpired:
        result.timed_out = True
        return result
    except Exception as exc:
        result.raw_stderr = str(exc)
        return result

    # Parse the verification results line.
    combined = result.raw_output + "\n" + result.raw_stderr
    m = _RESULT_RE.search(combined)
    if m:
        result.verified = int(m.group(1))
        result.errors = int(m.group(2))
    else:
        result.parse_error = True

    return result


def run_verus_on_source(
    source: str,
    tmp_dir: str,
    filename: str = "task.rs",
    **kwargs,
) -> VerificationResult:
    """
    Write *source* to a temp file inside *tmp_dir* and verify it.
    Convenience wrapper around ``run_verus``.
    """
    os.makedirs(tmp_dir, exist_ok=True)
    path = os.path.join(tmp_dir, filename)
    with open(path, "w") as f:
        f.write(source)
    return run_verus(path, **kwargs)
