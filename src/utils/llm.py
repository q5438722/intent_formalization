"""
Unified LLM client using GitHub Copilot CLI.

Invokes `copilot -p <prompt> -s --model <model>` as a subprocess.
For structured output, uses `--output-format json` and parses JSONL.
"""

import json
import logging
import os
import select
import subprocess
import tempfile
import time
from dataclasses import dataclass, field
from typing import Optional

logger = logging.getLogger(__name__)

# Model ID mapping: config name -> copilot model string
MODEL_MAP = {
    "claude-opus-4.6": "claude-opus-4.6",
    "gpt-5.4": "gpt-5.4",
    "gemini-3.1-pro": "gemini-3.1-pro",
}

# Use /tmp as working dir to avoid copilot reading workspace files
COPILOT_CWD = "/tmp"

# Global default timeout (seconds). Override via LLM_TIMEOUT env var.
DEFAULT_TIMEOUT = int(os.environ.get("LLM_TIMEOUT", "150"))


@dataclass
class LLMResponse:
    content: str
    model: str
    usage: dict = field(default_factory=dict)


# Stall timeout: kill if no output for this many seconds
STALL_TIMEOUT = int(os.environ.get("LLM_STALL_TIMEOUT", "360"))


def _run_copilot(
    prompt: str,
    model: str = "claude-opus-4.6",
    timeout: int | None = None,
    silent: bool = True,
) -> LLMResponse:
    """
    Run copilot CLI with a prompt and return the response.
    
    Uses Popen to monitor for stalls — if no stdout data arrives for
    STALL_TIMEOUT seconds, the process is killed.
    Total wall-clock timeout is also enforced.
    """
    timeout = timeout or DEFAULT_TIMEOUT
    resolved_model = MODEL_MAP.get(model, model)

    use_stdin = len(prompt) > 4000

    if use_stdin:
        cmd = ["copilot", "--model", resolved_model]
    else:
        cmd = ["copilot", "-p", prompt, "--model", resolved_model]
    if silent:
        cmd.append("-s")

    logger.info(f"Copilot request: model={resolved_model}, prompt_len={len(prompt)}, stdin={use_stdin}")

    proc = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE if use_stdin else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd=COPILOT_CWD,
    )

    try:
        # Send input if piping via stdin
        if use_stdin:
            proc.stdin.write(prompt)
            proc.stdin.close()

        chunks = []
        start_time = time.monotonic()
        last_data_time = time.monotonic()

        while True:
            elapsed = time.monotonic() - start_time
            if elapsed > timeout:
                proc.kill()
                proc.wait()
                raise TimeoutError(f"Copilot timed out after {timeout}s (total)")

            stall = time.monotonic() - last_data_time
            if stall > STALL_TIMEOUT:
                proc.kill()
                proc.wait()
                partial = "".join(chunks)
                raise TimeoutError(
                    f"Copilot stalled for {STALL_TIMEOUT}s (no output). "
                    f"Got {len(partial)} chars so far. Total elapsed: {elapsed:.0f}s"
                )

            # Poll stdout with 1s timeout
            ready, _, _ = select.select([proc.stdout], [], [], 1.0)
            if ready:
                chunk = proc.stdout.read(4096)
                if chunk:
                    chunks.append(chunk)
                    last_data_time = time.monotonic()
                else:
                    # EOF — process done writing
                    break
            
            # Check if process exited
            if proc.poll() is not None:
                # Read remaining
                remaining = proc.stdout.read()
                if remaining:
                    chunks.append(remaining)
                break

        proc.wait()
        content = "".join(chunks).strip()
        stderr = proc.stderr.read()

        if proc.returncode != 0:
            logger.error(f"Copilot failed (exit {proc.returncode}): {stderr[:500]}")
            raise RuntimeError(f"Copilot exited with code {proc.returncode}: {stderr[:500]}")

        elapsed = time.monotonic() - start_time
        logger.info(f"Copilot completed in {elapsed:.0f}s, {len(content)} chars")
        return LLMResponse(content=content, model=resolved_model)

    except Exception:
        # Ensure process is dead on any error
        if proc.poll() is None:
            proc.kill()
            proc.wait()
        raise


def _run_copilot_json(
    prompt: str,
    model: str = "claude-opus-4.6",
    timeout: int | None = None,
) -> LLMResponse:
    """
    Run copilot CLI with JSON output format and parse the response.
    
    Extracts the final assistant.message content from JSONL stream.
    """
    timeout = timeout or DEFAULT_TIMEOUT
    resolved_model = MODEL_MAP.get(model, model)

    cmd = ["copilot", "-p", prompt, "--model", resolved_model, "--output-format", "json"]

    logger.info(f"Copilot JSON request: model={resolved_model}, prompt_len={len(prompt)}")

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=COPILOT_CWD,
        )

        content = ""
        usage = {}

        for line in result.stdout.strip().split("\n"):
            line = line.strip()
            if not line:
                continue
            try:
                event = json.loads(line)
                etype = event.get("type", "")

                if etype == "assistant.message":
                    content = event.get("data", {}).get("content", "")

                if etype == "result":
                    usage = event.get("usage", {})

            except json.JSONDecodeError:
                continue

        if not content:
            # Fallback: concatenate message deltas
            for line in result.stdout.strip().split("\n"):
                try:
                    event = json.loads(line.strip())
                    if event.get("type") == "assistant.message_delta":
                        content += event.get("data", {}).get("deltaContent", "")
                except (json.JSONDecodeError, KeyError):
                    continue

        return LLMResponse(content=content, model=resolved_model, usage=usage)

    except subprocess.TimeoutExpired:
        raise TimeoutError(f"Copilot timed out after {timeout}s")


class LLMClient:
    """LLM client using GitHub Copilot CLI."""

    def __init__(self, timeout: int | None = None):
        self.timeout = timeout or DEFAULT_TIMEOUT

    def chat(
        self,
        system_prompt: str,
        user_prompt: str,
        model: str = "claude-opus-4.6",
        timeout: int | None = None,
    ) -> LLMResponse:
        """
        Send a prompt to copilot and get response.
        
        Since copilot -p is single-turn, we combine system + user prompts.
        """
        combined = f"{system_prompt}\n\n---\n\n{user_prompt}"
        return _run_copilot(combined, model=model, timeout=timeout or self.timeout)

    def chat_json(
        self,
        system_prompt: str,
        user_prompt: str,
        model: str = "claude-opus-4.6",
        timeout: int | None = None,
    ) -> LLMResponse:
        """Same as chat() but with structured JSON output parsing."""
        combined = f"{system_prompt}\n\n---\n\n{user_prompt}"
        return _run_copilot_json(combined, model=model, timeout=timeout or self.timeout)

    def chat_multi(
        self,
        system_prompt: str,
        user_prompt: str,
        models: list[str],
        timeout: int | None = None,
    ) -> list[LLMResponse]:
        """Send the same prompt to multiple models. Returns list of responses."""
        responses = []
        for model in models:
            try:
                resp = self.chat(system_prompt, user_prompt, model=model, timeout=timeout)
                responses.append(resp)
            except Exception as e:
                logger.warning(f"Model {model} failed: {e}")
                responses.append(LLMResponse(content="", model=model, usage={"error": str(e)}))
        return responses
