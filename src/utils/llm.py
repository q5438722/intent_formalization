"""
Unified LLM client using GitHub Copilot CLI.

Invokes `copilot -p <prompt> -s --model <model>` as a subprocess.
For structured output, uses `--output-format json` and parses JSONL.
"""

import json
import logging
import os
import subprocess
import tempfile
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


@dataclass
class LLMResponse:
    content: str
    model: str
    usage: dict = field(default_factory=dict)


def _run_copilot(
    prompt: str,
    model: str = "claude-opus-4.6",
    timeout: int = 600,
    silent: bool = True,
) -> LLMResponse:
    """
    Run copilot CLI with a prompt and return the response.
    
    For large prompts (>4K chars), pipes via stdin to avoid shell arg limits.
    Uses -s (silent) for clean text output.
    Runs from /tmp to avoid workspace context pollution.
    """
    resolved_model = MODEL_MAP.get(model, model)

    use_stdin = len(prompt) > 4000

    if use_stdin:
        cmd = ["copilot", "--model", resolved_model]
    else:
        cmd = ["copilot", "-p", prompt, "--model", resolved_model]
    if silent:
        cmd.append("-s")

    logger.info(f"Copilot request: model={resolved_model}, prompt_len={len(prompt)}, stdin={use_stdin}")

    try:
        result = subprocess.run(
            cmd,
            input=prompt if use_stdin else None,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=COPILOT_CWD,
        )

        if result.returncode != 0:
            logger.error(f"Copilot failed (exit {result.returncode}): {result.stderr[:500]}")
            raise RuntimeError(f"Copilot exited with code {result.returncode}: {result.stderr[:500]}")

        content = result.stdout.strip()
        return LLMResponse(content=content, model=resolved_model)

    except subprocess.TimeoutExpired:
        raise TimeoutError(f"Copilot timed out after {timeout}s")


def _run_copilot_json(
    prompt: str,
    model: str = "claude-opus-4.6",
    timeout: int = 300,
) -> LLMResponse:
    """
    Run copilot CLI with JSON output format and parse the response.
    
    Extracts the final assistant.message content from JSONL stream.
    """
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

    def __init__(self, timeout: int = 600):
        self.timeout = timeout

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
