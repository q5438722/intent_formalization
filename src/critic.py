"""
critic.py — Multi-model critic ensemble for adversarial query filtering.

Each candidate query φ is evaluated by multiple critic LLMs via Copilot CLI.
The critic determines whether φ is a valid "undesirable property" or a false positive.

Grounded in test cases as evidence of developer intent.

Usage:
    python -m src.critic --queries queries.json --spec path/to/spec.rs [--tests path/to/tests.rs] [--output filtered.json]
"""

import argparse
import json
import logging
import re
import subprocess
import yaml
from pathlib import Path
from collections import Counter

logger = logging.getLogger(__name__)

BASE_DIR = Path(__file__).resolve().parent.parent

CRITIC_SYSTEM_PROMPT = """\
You are a formal verification expert acting as a specification critic.

Your role: given a Verus specification and a candidate adversarial query φ, determine whether φ is:

1. VALID — φ is genuinely undesirable. The spec should NOT entail it.
2. INVALID — φ is actually an intended property. The spec SHOULD entail it (or it's trivially true).
3. AMBIGUOUS — Cannot determine without more context.

Be rigorous. A query is only VALID if you can articulate WHY the spec should not entail it.

Respond with your analysis, then end with exactly one line:
VERDICT: VALID
or
VERDICT: INVALID
or
VERDICT: AMBIGUOUS
"""

CRITIC_QUERY_TEMPLATE = """\
## Specification

Read the specification from: {spec_path}

{tests_section}

## Candidate Query φ

Category: {category}

```rust
{query_code}
```

## Task

1. What property does this query assert?
2. Is this property intended or unintended given the spec and tests?
3. Your verdict.
"""

DEBATE_TEMPLATE = """\
You previously evaluated a query and gave verdict: {prev_verdict}

Your reasoning:
{prev_reasoning}

Another critic disagreed. Their reasoning:
{other_reasoning}

Reconsider your verdict. You may keep or change it.
End with: VERDICT: VALID / INVALID / AMBIGUOUS
"""


def parse_verdict(text: str) -> str:
    """Extract VERDICT from critic response."""
    for line in reversed(text.strip().split("\n")):
        upper = line.strip().upper()
        if "VERDICT:" in upper:
            for v in ["VALID", "INVALID", "AMBIGUOUS"]:
                if v in upper:
                    return v
    return "AMBIGUOUS"


def _run_copilot(prompt: str, model: str, timeout: int = 600) -> str:
    """Run copilot -p with a prompt, return stdout."""
    cmd = ["copilot", "-p", prompt, "--model", model, "-s", "--allow-all"]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, cwd="/tmp")
    if result.returncode != 0:
        raise RuntimeError(f"Copilot exited {result.returncode}: {result.stderr[:300]}")
    return result.stdout.strip()


def critique_single(
    spec_path: str,
    query: dict,
    model: str,
    tests_path: str = "",
) -> dict:
    """Get a single critic's judgment."""
    tests_section = ""
    if tests_path:
        tests_section = f"## Developer Test Cases\n\nRead test cases from: {tests_path}\n"

    user_prompt = CRITIC_QUERY_TEMPLATE.format(
        spec_path=spec_path,
        tests_section=tests_section,
        category=query.get("category", "unknown"),
        query_code=query["code"],
    )

    full_prompt = f"{CRITIC_SYSTEM_PROMPT}\n\n---\n\n{user_prompt}"
    content = _run_copilot(full_prompt, model)
    verdict = parse_verdict(content)

    return {
        "model": model,
        "verdict": verdict,
        "reasoning": content,
        "turn": 1,
    }


def critique_with_debate(
    spec_path: str,
    query: dict,
    models: list[str],
    tests_path: str = "",
    max_turns: int = 3,
) -> dict:
    """
    Multi-model critique with optional debate for disagreements.
    
    Phase 1: Independent judgments from all critics
    Phase 2: If disagreement, share opposing reasoning and re-evaluate
    """
    # Phase 1: Independent judgments
    judgments = []
    for model in models:
        try:
            j = critique_single(spec_path, query, model, tests_path)
            judgments.append(j)
            logger.info(f"    {model}: {j['verdict']}")
        except Exception as e:
            logger.warning(f"    {model} failed: {e}")
            judgments.append({"model": model, "verdict": "AMBIGUOUS", "reasoning": str(e), "turn": 1})

    # Check consensus
    verdicts = [j["verdict"] for j in judgments]
    if len(set(verdicts)) == 1:
        return {
            "query": query,
            "judgments": judgments,
            "final_verdict": verdicts[0],
            "consensus": True,
        }

    # Phase 2: Debate rounds
    if max_turns > 1:
        for turn in range(2, max_turns + 1):
            new_judgments = []
            for i, j in enumerate(judgments):
                # Find dissenting opinion
                dissenter = next((o for o in judgments if o["verdict"] != j["verdict"]), None)
                if dissenter is None:
                    new_judgments.append(j)
                    continue

                # Ask to reconsider
                debate_prompt = DEBATE_TEMPLATE.format(
                    prev_verdict=j["verdict"],
                    prev_reasoning=j["reasoning"][:2000],
                    other_reasoning=dissenter["reasoning"][:2000],
                )

                try:
                    full_prompt = f"{CRITIC_SYSTEM_PROMPT}\n\n---\n\n{debate_prompt}"
                    response_text = _run_copilot(full_prompt, model=j["model"])
                    new_verdict = parse_verdict(response_text)
                    new_judgments.append({
                        "model": j["model"],
                        "verdict": new_verdict,
                        "reasoning": response_text,
                        "turn": turn,
                    })
                    if new_verdict != j["verdict"]:
                        logger.info(f"    {j['model']} (turn {turn}): {j['verdict']} -> {new_verdict}")
                except Exception:
                    new_judgments.append(j)

            judgments = new_judgments
            if len(set(j["verdict"] for j in judgments)) == 1:
                break

    # Majority vote
    final = Counter(j["verdict"] for j in judgments).most_common(1)[0][0]

    return {
        "query": query,
        "judgments": judgments,
        "final_verdict": final,
        "consensus": len(set(j["verdict"] for j in judgments)) == 1,
    }


def run_critic(
    queries_path: str,
    spec_path: str,
    tests_path: str | None = None,
    config_path: str = "src/config.yaml",
    output_path: str | None = None,
) -> list[dict]:
    """Run critic ensemble on all queries."""
    with open(BASE_DIR / config_path) as f:
        config = yaml.safe_load(f)

    with open(queries_path) as f:
        queries_data = json.load(f)

    critic_config = config["critic"]
    models = [m["id"] for m in critic_config["models"]]

    results = []

    queries = queries_data.get("queries", queries_data)
    logger.info(f"Running critic ensemble ({', '.join(models)}) on {len(queries)} queries")

    spec_abs = str(Path(spec_path).resolve())
    tests_abs = str(Path(tests_path).resolve()) if tests_path else ""

    for i, query in enumerate(queries):
        logger.info(f"  [{i+1}/{len(queries)}] {query.get('name', 'unnamed')}")
        result = critique_with_debate(
            spec_abs, query, models,
            tests_path=tests_abs,
            max_turns=critic_config["max_turns"],
        )
        results.append(result)

    valid = [r for r in results if r["final_verdict"] == "VALID"]
    rejected = [r for r in results if r["final_verdict"] != "VALID"]

    logger.info(f"Results: {len(valid)} VALID, "
                f"{sum(1 for r in results if r['final_verdict'] == 'INVALID')} INVALID, "
                f"{sum(1 for r in results if r['final_verdict'] == 'AMBIGUOUS')} AMBIGUOUS")

    output = {
        "spec_path": spec_path,
        "critic_models": models,
        "total_queries": len(queries),
        "valid_queries": [r["query"] for r in valid],
        "rejected_queries": [{"query": r["query"], "verdict": r["final_verdict"]} for r in rejected],
        "full_results": results,
    }

    if output_path:
        Path(output_path).parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            json.dump(output, f, indent=2)
        logger.info(f"Saved filtered queries to {output_path}")

    return [r["query"] for r in valid]


def main():
    parser = argparse.ArgumentParser(description="Run critic ensemble on adversarial queries")
    parser.add_argument("--queries", required=True, help="Path to generated queries JSON")
    parser.add_argument("--spec", required=True, help="Path to Verus spec file")
    parser.add_argument("--tests", default=None, help="Path to developer test file")
    parser.add_argument("--config", default="src/config.yaml")
    parser.add_argument("--output", default=None, help="Output JSON path")
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)
    run_critic(args.queries, args.spec, args.tests, args.config, args.output)


if __name__ == "__main__":
    main()
