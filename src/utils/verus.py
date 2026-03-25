"""
Verus runner — execute Verus verification and parse results.
"""

import os
import re
import json
import logging
import subprocess
import tempfile
import glob
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger(__name__)

# Default Verus cache location
VERUS_CACHE_DIR = os.path.expanduser("~/intent_formalization/nanvix/.verus-cache")


@dataclass
class VerusResult:
    success: bool
    verified: int
    errors: int
    output: str
    error_details: list[dict]


def find_verus_binary(config_path: str = "") -> str:
    """Find the Verus binary, checking config path, .verus-cache, and PATH."""
    if config_path and os.path.isfile(config_path):
        return config_path

    # Check .verus-cache for latest version
    if os.path.isdir(VERUS_CACHE_DIR):
        zips = sorted(glob.glob(os.path.join(VERUS_CACHE_DIR, "verus-*.zip")))
        if zips:
            # Latest zip — extract dir name
            latest = zips[-1]
            extract_dir = latest.replace(".zip", "")
            binary = os.path.join(extract_dir, "verus")
            if os.path.isfile(binary):
                return binary

    # Fall back to PATH
    result = subprocess.run(["which", "verus"], capture_output=True, text=True)
    if result.returncode == 0:
        return result.stdout.strip()

    raise FileNotFoundError("Verus binary not found. Set verus.binary in config.yaml or add to PATH.")


def parse_verus_output(output: str) -> VerusResult:
    """Parse Verus stdout/stderr into structured result."""
    verified = 0
    errors = 0
    error_details = []

    # Match "verification results:: N verified, M errors"
    match = re.search(r"(\d+)\s+verified,\s+(\d+)\s+errors?", output)
    if match:
        verified = int(match.group(1))
        errors = int(match.group(2))

    # Extract individual error messages
    for m in re.finditer(r"error\[.*?\]:\s*(.*?)(?:\n\s*-->.*?)?(?:\n|$)", output):
        error_details.append({"message": m.group(1).strip()})

    # Must have verified > 0 AND errors == 0 to count as success.
    # If Verus never printed "N verified, M errors" (e.g. compile error),
    # verified=0 and errors=0 — that's a failure, not success.
    success = verified > 0 and errors == 0

    return VerusResult(
        success=success,
        verified=verified,
        errors=errors,
        output=output,
        error_details=error_details,
    )


def run_verus(
    file_path: str,
    verus_binary: str = "",
    timeout: int = 60,
    extra_args: list[str] | None = None,
) -> VerusResult:
    """Run Verus on a file and return parsed results."""
    binary = find_verus_binary(verus_binary)
    cmd = [binary, file_path] + (extra_args or [])

    logger.info(f"Running Verus: {' '.join(cmd)}")

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        combined = result.stdout + "\n" + result.stderr
        return parse_verus_output(combined)
    except subprocess.TimeoutExpired:
        return VerusResult(
            success=False,
            verified=0,
            errors=1,
            output=f"TIMEOUT after {timeout}s",
            error_details=[{"message": f"Verus timed out after {timeout}s"}],
        )
    except Exception as e:
        return VerusResult(
            success=False,
            verified=0,
            errors=1,
            output=str(e),
            error_details=[{"message": str(e)}],
        )


def check_entailment(
    spec_code: str,
    query_code: str,
    verus_binary: str = "",
    timeout: int = 60,
) -> dict:
    """
    Check if spec entails query by combining them into a single file and running Verus.
    
    Returns:
        {"entailed": bool, "result": VerusResult}
        
    If Verus verifies successfully, the spec entails the query (potential inconsistency).
    If Verus fails, the spec does NOT entail the query (expected for adversarial queries).
    """
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        # Combine spec and query into one file
        f.write(spec_code)
        f.write("\n\n// === ENTAILMENT QUERY ===\n\n")
        f.write(query_code)
        f.flush()
        tmp_path = f.name

    try:
        result = run_verus(tmp_path, verus_binary=verus_binary, timeout=timeout)
        return {
            "entailed": result.success,  # If it verifies, spec ⊢ φ
            "result": result,
        }
    finally:
        os.unlink(tmp_path)
